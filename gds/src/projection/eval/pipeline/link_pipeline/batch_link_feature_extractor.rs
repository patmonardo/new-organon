// Phase 3.3: BatchLinkFeatureExtractor - Parallel worker for link feature extraction

use super::LinkFeatureExtractor;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Parallel worker for link feature extraction.
///
/// Processes a partition of relationships, extracts features per edge,
/// writes into a shared output array, and updates progress.
pub struct BatchLinkFeatureExtractor {
    /// The feature extractor (orchestrator)
    _extractor: Arc<LinkFeatureExtractor>,

    /// The partition of nodes to process
    /// Note: Replace with actual DegreePartition when ported.
    partition: PhantomData<()>,

    /// The graph to extract from (concurrent copy)
    graph: PhantomData<()>, // Note: replace with Arc<dyn Graph>

    /// Offset into linkFeatures array for this batch
    relationship_offset: Arc<AtomicU64>,

    /// Shared output array for all batches
    /// Note: Replace with HugeObjectArray<Vec<f64>>
    link_features: PhantomData<()>,

    /// Progress tracker
    /// Note: Replace with ProgressTracker
    progress_tracker: PhantomData<()>,
}

impl BatchLinkFeatureExtractor {
    /// Creates a new batch extractor.
    /// # Arguments
    ///
    /// * `extractor` - The LinkFeatureExtractor orchestrator
    /// * `partition` - DegreePartition to process
    /// * `graph` - Concurrent graph copy
    /// * `relationship_offset` - Starting offset in linkFeatures
    /// * `link_features` - Shared output array
    /// * `progress_tracker` - Progress reporting
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        extractor: Arc<LinkFeatureExtractor>,
        _partition: PhantomData<()>,
        _graph: PhantomData<()>,
        relationship_offset: u64,
        _link_features: PhantomData<()>,
        _progress_tracker: PhantomData<()>,
    ) -> Self {
        Self {
            _extractor: extractor,
            partition: PhantomData,
            graph: PhantomData,
            relationship_offset: Arc::new(AtomicU64::new(relationship_offset)),
            link_features: PhantomData,
            progress_tracker: PhantomData,
        }
    }

    /// Run the extraction for this batch.
    pub fn run(&self) {
        // Note: Implement the extraction loop once DegreePartition/Graph/HugeObjectArray are wired.
        // let mut current_offset = self.relationship_offset.load(Ordering::Relaxed);
        //
        // self.partition.consume(|node_id| {
        //     self.graph.for_each_relationship(node_id, |source, target| {
        //         // Extract features (ENTITY EXTRACTION!)
        //         let features = self.extractor.extract_features_for_pair(source, target);
        //
        //         // Write to shared array
        //         self.link_features.set(current_offset, features);
        //         current_offset += 1;
        //
        //         true // Continue iteration
        //     });
        // });
        //
        // // Report progress
        // self.progress_tracker.log_steps(self.partition.relationship_count());

        let _ = self.relationship_offset.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::link_pipeline::link_functions::HadamardFeatureStep;
    use crate::projection::eval::pipeline::link_pipeline::LinkFeatureStep;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    fn random_graph_store(config: &RandomGraphConfig) -> DefaultGraphStore {
        DefaultGraphStore::random(config).expect("random graph")
    }

    #[test]
    #[ignore] // FIXME: Random graph doesn't have 'prop' property
    fn test_batch_extractor_creation() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> =
            vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))];

        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        let batch_extractor = BatchLinkFeatureExtractor::new(
            extractor,
            PhantomData,
            PhantomData,
            0,
            PhantomData,
            PhantomData,
        );

        // Verify offset starts at 0
        assert_eq!(
            batch_extractor.relationship_offset.load(Ordering::Relaxed),
            0
        );
    }

    #[test]
    #[ignore] // FIXME: Random graph doesn't have 'prop' property
    fn test_batch_extractor_with_offset() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> =
            vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))];

        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        let batch_extractor = BatchLinkFeatureExtractor::new(
            extractor,
            PhantomData,
            PhantomData,
            100, // Start at offset 100
            PhantomData,
            PhantomData,
        );

        assert_eq!(
            batch_extractor.relationship_offset.load(Ordering::Relaxed),
            100
        );
    }

    #[test]
    #[ignore] // FIXME: Random graph doesn't have 'embedding' property
    fn test_features_are_entities() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> = vec![Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string(),
        ]))];

        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        let _batch_extractor = BatchLinkFeatureExtractor::new(
            extractor,
            PhantomData,
            PhantomData,
            0,
            PhantomData,
            PhantomData,
        );
    }

    #[test]
    #[ignore] // FIXME: Random graph doesn't have 'features' property
    fn test_car_cdr_entity_extraction() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> = vec![Box::new(HadamardFeatureStep::new(vec![
            "features".to_string(),
        ]))];

        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        let batch_extractor = BatchLinkFeatureExtractor::new(
            Arc::clone(&extractor),
            PhantomData,
            PhantomData,
            0,
            PhantomData,
            PhantomData,
        );

        assert_eq!(
            batch_extractor.relationship_offset.load(Ordering::Relaxed),
            0
        );
    }
}
