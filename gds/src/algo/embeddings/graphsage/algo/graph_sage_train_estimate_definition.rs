//! Java: `GraphSageTrainEstimateDefinition`.

use crate::algo::embeddings::graphsage::types::{
    AggregatorType, GraphSageTrainMemoryEstimateParameters,
};
use crate::core::graph_dimensions::GraphDimensions;
use crate::mem::{Estimate, MemoryEstimation, MemoryRange, MemoryTree};

#[derive(Debug, Clone)]
pub struct GraphSageTrainEstimateDefinition {
    parameters: GraphSageTrainMemoryEstimateParameters,
}

impl GraphSageTrainEstimateDefinition {
    pub fn new(parameters: GraphSageTrainMemoryEstimateParameters) -> Self {
        Self { parameters }
    }
}

impl MemoryEstimation for GraphSageTrainEstimateDefinition {
    fn description(&self) -> String {
        "GraphSageTrain".to_string()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let node_count = dimensions.node_count();

        // Resident: weights (all layers)
        let mut weights_components = Vec::new();
        let mut adam_init = 0usize;
        let mut adam_update = 0usize;

        for (i, layer) in self.parameters.layer_configs.iter().enumerate() {
            let base_dims = layer.rows * layer.cols;
            let mut weights_mem = Estimate::size_of_double_array(base_dims);
            if layer.aggregator_type == AggregatorType::Pool {
                // selfWeights (rows*cols), neighborsWeights (rows*rows), bias (rows)
                weights_mem += Estimate::size_of_double_array(layer.rows * layer.cols);
                weights_mem += Estimate::size_of_double_array(layer.rows * layer.rows);
                weights_mem += Estimate::size_of_double_array(layer.rows);
            }
            weights_components.push(MemoryTree::leaf(
                format!("layer {}", i + 1),
                MemoryRange::of(weights_mem),
            ));

            // Adam optimizer memory approximations mirroring Java formula.
            adam_init += 2 * Estimate::size_of_double_array(base_dims);
            adam_update += 5 * base_dims;
        }

        // Multi-label: weightsByLabel (range per label count) - we don't have labelCount in GraphDimensions.
        // We conservatively assume 1 label in estimation dimensions.
        let label_count = 1usize;
        let mut temp_components = vec![MemoryTree::leaf(
            "this.instance".into(),
            MemoryRange::of(Estimate::BYTES_OBJECT_HEADER),
        )];

        if self.parameters.is_multi_label {
            let min_num_properties = 1usize;
            let max_num_properties = self.parameters.feature_property_count + 1; // +1 bias

            let min_mem = Estimate::size_of_double_array(
                self.parameters.estimation_feature_dimension * min_num_properties,
            );
            let max_mem = Estimate::size_of_double_array(
                self.parameters.estimation_feature_dimension * max_num_properties,
            );
            temp_components.push(MemoryTree::leaf(
                "weightsByLabel".into(),
                MemoryRange::of_range(min_mem, max_mem).times(label_count),
            ));
        }

        // initialFeatures: range per node (multi-label min is 1 feature, max is estimationFeatureDimension)
        let min_init = Estimate::size_of_double_array(if self.parameters.is_multi_label {
            1
        } else {
            self.parameters.estimation_feature_dimension
        });
        let max_init = Estimate::size_of_double_array(self.parameters.estimation_feature_dimension);
        temp_components.push(MemoryTree::leaf(
            "initialFeatures".into(),
            MemoryRange::of_range(node_count * min_init, node_count * max_init),
        ));

        // trainOnEpoch -> per thread trainOnBatch estimate:
        let per_thread_batch = (3 * self.parameters.batch_size).max(1);
        let per_thread = per_thread_batch
            * Estimate::size_of_double_array(self.parameters.embedding_dimension)
            + adam_update;
        temp_components.push(MemoryTree::leaf(
            "initialAdamOptimizer".into(),
            MemoryRange::of(adam_init),
        ));
        temp_components.push(MemoryTree::leaf(
            "concurrentBatches".into(),
            MemoryRange::of(per_thread).times(concurrency.max(1)),
        ));

        let resident_weights = MemoryTree::new(
            "residentMemory".into(),
            sum_ranges(&weights_components),
            vec![MemoryTree::new(
                "weights".into(),
                sum_ranges(&weights_components),
                weights_components,
            )],
        );
        let temporary = MemoryTree::new(
            "temporaryMemory".into(),
            sum_ranges(&temp_components),
            temp_components,
        );

        MemoryTree::new(
            "GraphSageTrain".into(),
            sum_ranges(&[resident_weights.clone(), temporary.clone()]),
            vec![resident_weights, temporary],
        )
    }
}

fn sum_ranges(children: &[MemoryTree]) -> MemoryRange {
    children
        .iter()
        .fold(MemoryRange::empty(), |acc, t| acc.add(t.memory_usage()))
}
