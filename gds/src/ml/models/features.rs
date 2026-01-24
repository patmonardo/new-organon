use crate::collections::HugeObjectArray;
use crate::ml::core::features::{
    extract, extract_graph, feature_count, property_extractors, AnyFeatureExtractor,
    FeatureConsumer,
};
use crate::ml::core::tensor::Vector;
use crate::types::graph::Graph;
use once_cell::sync::OnceCell;
use std::sync::Arc;

/// Features trait - 1:1 with Features.java
/// This trait is implemented in base.rs as well, keeping this for re-export
pub use super::base::Features;

/// Dense in-memory feature storage
#[derive(Clone, Debug)]
pub struct DenseFeatures {
    data: Vec<Vec<f64>>,
}

impl DenseFeatures {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }

    pub fn from_vectors(vectors: Vec<Vector>) -> Self {
        let data = vectors.into_iter().map(|v| v.to_vec()).collect();
        Self { data }
    }
}

impl Features for DenseFeatures {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, id: usize) -> &[f64] {
        &self.data[id]
    }
}

/// Features backed by a HugeObjectArray of feature vectors.
/// 1:1 with the anonymous Features class in FeaturesFactory.wrap(HugeObjectArray<double[]>)
pub struct HugeArrayFeatures {
    features: HugeObjectArray<Vec<f64>>,
}

impl HugeArrayFeatures {
    fn new(features: HugeObjectArray<Vec<f64>>) -> Self {
        Self { features }
    }
}

impl Features for HugeArrayFeatures {
    fn size(&self) -> usize {
        self.features.size()
    }

    fn get(&self, id: usize) -> &[f64] {
        self.features.get(id)
    }

    fn feature_dimension(&self) -> usize {
        if self.size() > 0 {
            self.features.get(0).len()
        } else {
            0
        }
    }
}

/// Lazy feature extraction from graph properties
pub struct LazyFeatures {
    size: usize,
    feature_dimension: usize,
    producer: Arc<dyn Fn(usize) -> Vec<f64> + Send + Sync>,
    cache: Vec<OnceCell<Vec<f64>>>,
}

/// Lazy feature extraction from graph node properties.
/// 1:1 with the anonymous Features class in FeaturesFactory.extractLazyFeatures()
pub struct LazyExtractedFeatures {
    graph: Arc<dyn Graph>,
    extractors: Vec<AnyFeatureExtractor>,
    feature_dimension: usize,
    cache: Vec<OnceCell<Vec<f64>>>,
}

impl LazyExtractedFeatures {
    fn new(
        graph: Arc<dyn Graph>,
        extractors: Vec<AnyFeatureExtractor>,
        feature_dimension: usize,
    ) -> Self {
        let cache = (0..graph.node_count()).map(|_| OnceCell::new()).collect();
        Self {
            graph,
            extractors,
            feature_dimension,
            cache,
        }
    }
}

impl Features for LazyExtractedFeatures {
    fn size(&self) -> usize {
        self.graph.node_count()
    }

    fn get(&self, id: usize) -> &[f64] {
        let cell = &self.cache[id];
        let feature_dimension = self.feature_dimension;
        let extractors = &self.extractors;
        cell.get_or_init(|| {
            let mut features = vec![0.0f64; feature_dimension];
            let mut consumer = VecFeatureConsumer::new(&mut features);
            extract(id as u64, id as u64, extractors, &mut consumer);
            features
        })
        .as_slice()
    }

    fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }
}

/// Simple feature consumer that writes into a Vec<f64>
struct VecFeatureConsumer<'a> {
    features: &'a mut Vec<f64>,
}

impl<'a> VecFeatureConsumer<'a> {
    fn new(features: &'a mut Vec<f64>) -> Self {
        Self { features }
    }
}

impl<'a> FeatureConsumer for VecFeatureConsumer<'a> {
    fn accept_scalar(&mut self, _node_offset: u64, offset: usize, value: f64) {
        self.features[offset] = value;
    }

    fn accept_array(&mut self, _node_offset: u64, offset: usize, values: &[f64]) {
        self.features[offset..offset + values.len()].copy_from_slice(values);
    }
}

impl LazyFeatures {
    /// Create a lazily-computed feature store.
    ///
    /// This is intentionally graph-agnostic: callers provide a `producer` that
    /// maps an index to a feature vector. Values are cached on first access.
    pub fn new<F>(size: usize, feature_dimension: usize, producer: F) -> Self
    where
        F: Fn(usize) -> Vec<f64> + Send + Sync + 'static,
    {
        Self {
            size,
            feature_dimension,
            producer: Arc::new(producer),
            cache: (0..size).map(|_| OnceCell::new()).collect(),
        }
    }
}

impl Features for LazyFeatures {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, id: usize) -> &[f64] {
        let cell = &self.cache[id];
        let feature_dimension = self.feature_dimension;
        cell.get_or_init(|| {
            let v = (self.producer)(id);
            if feature_dimension != 0 && v.len() != feature_dimension {
                panic!(
                    "LazyFeatures producer returned dimension {}, expected {} (id={})",
                    v.len(),
                    feature_dimension,
                    id
                );
            }
            v
        })
        .as_slice()
    }

    fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }
}

/// Features factory - 1:1 with FeaturesFactory.java
pub struct FeaturesFactory;

impl FeaturesFactory {
    /// Wrap a HugeObjectArray of feature vectors
    /// 1:1 with wrap(HugeObjectArray<double[]>) in Java
    pub fn wrap_huge_array(features: HugeObjectArray<Vec<f64>>) -> Box<dyn Features> {
        Box::new(HugeArrayFeatures::new(features))
    }

    /// Wrap a single feature vector
    /// 1:1 with wrap(double[]) in Java
    pub fn wrap_single(features: Vec<f64>) -> Box<dyn Features> {
        Box::new(DenseFeatures::new(vec![features]))
    }

    /// Wrap a list of feature vectors
    /// 1:1 with wrap(List<double[]>) in Java
    pub fn wrap_list(features: Vec<Vec<f64>>) -> Box<dyn Features> {
        Box::new(DenseFeatures::new(features))
    }

    /// Construct a lazily-computed feature store with caching.
    pub fn wrap_lazy<F>(size: usize, feature_dimension: usize, producer: F) -> Box<dyn Features>
    where
        F: Fn(usize) -> Vec<f64> + Send + Sync + 'static,
    {
        Box::new(LazyFeatures::new(size, feature_dimension, producer))
    }

    /// Extract lazy features from graph properties.
    /// 1:1 with FeaturesFactory.extractLazyFeatures(Graph, List<String>) in Java
    pub fn extract_lazy_features(
        graph: Arc<dyn Graph>,
        feature_properties: &[String],
    ) -> Box<dyn Features> {
        let extractors = property_extractors(&*graph, feature_properties);
        let feature_dimension = feature_count(&extractors);
        Box::new(LazyExtractedFeatures::new(
            graph,
            extractors,
            feature_dimension,
        ))
    }

    /// Extract eager features from graph properties.
    /// 1:1 with FeaturesFactory.extractEagerFeatures(Graph, List<String>) in Java
    pub fn extract_eager_features(
        graph: Arc<dyn Graph>,
        feature_properties: &[String],
    ) -> Box<dyn Features> {
        let extractors = property_extractors(&*graph, feature_properties);
        let features_array = HugeObjectArray::new(graph.node_count());
        let features_array = extract_graph(&*graph, &extractors, features_array);
        Box::new(HugeArrayFeatures::new(features_array))
    }

    // extractLazyFeatures and extractEagerFeatures methods implemented above
}

#[cfg(test)]
mod tests {
    use super::{Features, LazyFeatures};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[test]
    fn lazy_features_caches_per_id() {
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let features = LazyFeatures::new(3, 2, move |id| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            vec![id as f64, (id as f64) + 1.0]
        });

        assert_eq!(features.size(), 3);

        let a1 = features.get(1);
        let a2 = features.get(1);
        assert_eq!(a1, &[1.0, 2.0]);
        assert_eq!(a2, &[1.0, 2.0]);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        let _ = features.get(2);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    #[should_panic(expected = "expected 3")]
    fn lazy_features_validates_dimension() {
        let features = LazyFeatures::new(1, 3, |_id| vec![1.0, 2.0]);
        let _ = features.get(0);
    }
}
