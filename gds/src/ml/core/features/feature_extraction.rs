//! Feature extraction utilities for ML in GDS.
//!
//! Translated from Java GDS ml-core FeatureExtraction.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! Responsible for extracting features into abstract consumers (FeatureConsumer).
//! Also contains logic for looping on graphs and batches and writing into
//! Matrices and HugeObjectArrays.

use super::{
    ArrayFeatureExtractor, ArrayPropertyExtractor, BiasFeature, FeatureConsumer, FeatureExtractor,
    HugeObjectArrayFeatureConsumer, LongArrayPropertyExtractor, ScalarFeatureExtractor,
    ScalarPropertyExtractor,
};
use crate::collections::HugeObjectArray;
use crate::mem::Estimate;
use crate::ml::core::batch::Batch;
use crate::ml::core::functions::Constant;
use crate::ml::core::Matrix;
use crate::types::graph::Graph;
use crate::types::ValueType;
// use crate::collections::HugeObjectArray;

/// Type-erased feature extractor wrapper.
///
/// **Rust Pattern**: Enum dispatch instead of Java's instanceof.
/// This is more idiomatic Rust and avoids dynamic casting.
pub enum AnyFeatureExtractor {
    Scalar(Box<dyn ScalarFeatureExtractor>),
    Array(Box<dyn ArrayFeatureExtractor>),
}

impl FeatureExtractor for AnyFeatureExtractor {
    fn dimension(&self) -> usize {
        match self {
            AnyFeatureExtractor::Scalar(_) => 1,
            AnyFeatureExtractor::Array(extractor) => extractor.dimension(),
        }
    }
}

impl AnyFeatureExtractor {
    /// Extract features into a consumer (replaces Java's instanceof dispatch).
    pub fn extract_into(
        &self,
        node_id: u64,
        node_offset: u64,
        offset: usize,
        consumer: &mut dyn FeatureConsumer,
    ) {
        match self {
            AnyFeatureExtractor::Scalar(extractor) => {
                let value = extractor.extract(node_id);
                consumer.accept_scalar(node_offset, offset, value);
            }
            AnyFeatureExtractor::Array(extractor) => {
                let values = extractor.extract(node_id);
                consumer.accept_array(node_offset, offset, &values);
            }
        }
    }
}

/// Extract features for a single node using a list of extractors.
///
/// Corresponds to: `FeatureExtraction.extract(long nodeId, long nodeOffset,
///                  List<FeatureExtractor> extractors, FeatureConsumer consumer)`
pub fn extract(
    node_id: u64,
    node_offset: u64,
    extractors: &[AnyFeatureExtractor],
    consumer: &mut dyn FeatureConsumer,
) {
    let mut offset = 0;
    for extractor in extractors {
        extractor.extract_into(node_id, node_offset, offset, consumer);
        offset += extractor.dimension();
    }
}

/// Extract features for a batch into a constant matrix.
///
/// Java: `FeatureExtraction.extract(Batch batch, List<FeatureExtractor> extractors)`
pub fn extract_batch<B: Batch>(batch: &B, extractors: &[AnyFeatureExtractor]) -> Constant {
    let feature_dim = feature_count(extractors);
    let mut matrix = Matrix::with_dimensions(batch.size(), feature_dim);

    struct MatrixFeatureConsumer<'a> {
        matrix: &'a mut Matrix,
    }

    impl FeatureConsumer for MatrixFeatureConsumer<'_> {
        fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
            self.matrix.set_data_at(node_offset as usize, offset, value);
        }

        fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
            let row = node_offset as usize;
            for (i, value) in values.iter().enumerate() {
                self.matrix.set_data_at(row, offset + i, *value);
            }
        }
    }

    let mut consumer = MatrixFeatureConsumer {
        matrix: &mut matrix,
    };

    for (row_idx, node_id) in batch.element_ids().enumerate() {
        extract(node_id, row_idx as u64, extractors, &mut consumer);
    }

    Constant::new(Box::new(matrix))
}

/// Extract features for all nodes in a graph into a HugeObjectArray of `Vec<f64>`.
///
/// Java: `FeatureExtraction.extract(Graph graph, List<FeatureExtractor> extractors, HugeObjectArray<double[]> features)`
pub fn extract_graph(
    graph: &dyn Graph,
    extractors: &[AnyFeatureExtractor],
    features: HugeObjectArray<Vec<f64>>,
) -> HugeObjectArray<Vec<f64>> {
    let feature_dim = feature_count(extractors);

    // Ensure every row is allocated to the correct length.
    // (HugeObjectArray default is empty Vec)
    let mut consumer = HugeObjectArrayFeatureConsumer::new(features);
    consumer
        .features_mut()
        .set_all(|_| vec![0.0f64; feature_dim]);

    for node_id in 0..graph.node_count() {
        extract(node_id as u64, node_id as u64, extractors, &mut consumer);
    }

    consumer.into_inner()
}

/// Calculate total feature count from a collection of extractors.
///
/// Corresponds to: `FeatureExtraction.featureCount(Collection<FeatureExtractor> extractors)`
pub fn feature_count(extractors: &[AnyFeatureExtractor]) -> usize {
    extractors.iter().map(|e| e.dimension()).sum()
}

/// Create feature extractors backed by node properties (Java: FeatureExtraction.propertyExtractors()).
///
/// Panics if a requested property is missing.
pub fn property_extractors(
    graph: &dyn Graph,
    feature_properties: &[String],
) -> Vec<AnyFeatureExtractor> {
    property_extractors_with_init(graph, feature_properties, 0)
}

/// Create feature extractors backed by node properties with an explicit init node id.
///
/// This is used to infer array dimensions when not present in schema/values.
pub fn property_extractors_with_init(
    graph: &dyn Graph,
    feature_properties: &[String],
    init_node_id: u64,
) -> Vec<AnyFeatureExtractor> {
    feature_properties
        .iter()
        .map(|key| {
            let values = graph.node_properties(key).unwrap_or_else(|| {
                panic!(
                    "Missing node property `{}`. Consider using a default value in the property projection.",
                    key
                )
            });

            match values.value_type() {
                ValueType::Double | ValueType::Long => AnyFeatureExtractor::Scalar(Box::new(
                    ScalarPropertyExtractor::new(key.clone(), values),
                )),
                ValueType::DoubleArray | ValueType::FloatArray => {
                    let dim = values.dimension().unwrap_or_else(|| {
                        // Infer from init node.
                        values
                            .double_array_value(init_node_id)
                            .map(|v| v.len())
                            .or_else(|_| values.float_array_value(init_node_id).map(|v| v.len()))
                            .unwrap_or_else(|e| panic!("Failed inferring dimension for `{key}`: {e}"))
                    });
                    AnyFeatureExtractor::Array(Box::new(ArrayPropertyExtractor::new(
                        dim,
                        key.clone(),
                        values,
                    )))
                }
                ValueType::LongArray => {
                    let dim = values.dimension().unwrap_or_else(|| {
                        values
                            .long_array_value(init_node_id)
                            .map(|v| v.len())
                            .unwrap_or_else(|e| panic!("Failed inferring dimension for `{key}`: {e}"))
                    });
                    AnyFeatureExtractor::Array(Box::new(LongArrayPropertyExtractor::new(
                        dim,
                        key.clone(),
                        values,
                    )))
                }
                other => panic!(
                    "Unsupported feature property type for `{}`: {:?}",
                    key, other
                ),
            }
        })
        .collect()
}

/// Convenience: compute feature count directly from graph properties.
pub fn feature_count_from_graph(graph: &dyn Graph, feature_properties: &[String]) -> usize {
    feature_count(&property_extractors(graph, feature_properties))
}

/// Compute feature count including a bias feature.
///
/// Java: `FeatureExtraction.featureCountWithBias(graph, featureProperties)`
pub fn feature_count_with_bias(graph: &dyn Graph, feature_properties: &[String]) -> usize {
    let mut feature_extractors = property_extractors(graph, feature_properties);
    feature_extractors.push(AnyFeatureExtractor::Scalar(Box::new(BiasFeature)));
    feature_count(&feature_extractors)
}

/// Estimate memory usage for feature extractors.
///
/// Java: `FeatureExtraction.memoryUsageInBytes(int numberOfFeatures)`
pub fn memory_usage_in_bytes(number_of_features: usize) -> usize {
    let size_if_all_scalars =
        number_of_features * Estimate::size_of_instance("ScalarPropertyExtractor");
    let size_if_all_arrays =
        number_of_features * Estimate::size_of_instance("ArrayPropertyExtractor");

    size_if_all_scalars.max(size_if_all_arrays)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockScalarExtractor;
    impl FeatureExtractor for MockScalarExtractor {
        fn dimension(&self) -> usize {
            1
        }
    }
    impl ScalarFeatureExtractor for MockScalarExtractor {
        fn extract(&self, _node_id: u64) -> f64 {
            42.0
        }
    }

    struct MockArrayExtractor {
        dim: usize,
    }
    impl FeatureExtractor for MockArrayExtractor {
        fn dimension(&self) -> usize {
            self.dim
        }
    }
    impl ArrayFeatureExtractor for MockArrayExtractor {
        fn extract(&self, _node_id: u64) -> Vec<f64> {
            vec![1.0; self.dim]
        }
    }

    #[test]
    fn test_feature_count() {
        let extractors = vec![
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
        ];
        assert_eq!(feature_count(&extractors), 2);
    }

    #[test]
    fn test_feature_count_mixed() {
        let extractors = vec![
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
            AnyFeatureExtractor::Array(Box::new(MockArrayExtractor { dim: 3 })),
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
        ];
        assert_eq!(feature_count(&extractors), 5); // 1 + 3 + 1
    }

    #[test]
    fn test_extract() {
        struct TestConsumer {
            scalars: Vec<(u64, usize, f64)>,
            arrays: Vec<(u64, usize, Vec<f64>)>,
        }
        impl FeatureConsumer for TestConsumer {
            fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
                self.scalars.push((node_offset, offset, value));
            }
            fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
                self.arrays.push((node_offset, offset, values.to_vec()));
            }
        }

        let extractors = vec![
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
            AnyFeatureExtractor::Array(Box::new(MockArrayExtractor { dim: 2 })),
        ];

        let mut consumer = TestConsumer {
            scalars: Vec::new(),
            arrays: Vec::new(),
        };

        extract(100, 0, &extractors, &mut consumer);

        assert_eq!(consumer.scalars.len(), 1);
        assert_eq!(consumer.scalars[0], (0, 0, 42.0));

        assert_eq!(consumer.arrays.len(), 1);
        assert_eq!(consumer.arrays[0].0, 0); // node_offset
        assert_eq!(consumer.arrays[0].1, 1); // offset (after scalar)
        assert_eq!(consumer.arrays[0].2, vec![1.0, 1.0]);
    }
}
