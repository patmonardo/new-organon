//! Features module - 1:1 translation of Features.java and FeaturesFactory.java
//!
//! The Features trait is the core interface for accessing feature data.
//! Moved from NodeId-based API to index-based API to match Java.

use crate::ml::core::tensor::Vector;
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

/// Lazy feature extraction from graph properties
/// TODO: Implement lazy extraction following FeaturesFactory.extractLazyFeatures()
pub struct LazyFeatures {
    size: usize,
    feature_dimension: usize,
    producer: Arc<dyn Fn(usize) -> Vec<f64> + Send + Sync>,
    cache: Vec<OnceCell<Vec<f64>>>,
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
    pub fn wrap_array(features: Vec<Vec<f64>>) -> Box<dyn Features> {
        Box::new(DenseFeatures::new(features))
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

    // TODO: Add extractLazyFeatures and extractEagerFeatures methods
    // These require graph property extraction infrastructure
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
