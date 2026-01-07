//! Java: `GraphSageMemoryEstimateDefinition`.

use crate::algo::embeddings::graphsage::train_config_transformer::TrainConfigTransformer;
use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;
use crate::core::graph_dimensions::GraphDimensions;
use crate::mem::{Estimate, MemoryEstimation, MemoryRange, MemoryTree};

/// A memory estimation for GraphSage inference (embeddings generation).
#[derive(Debug, Clone)]
pub struct GraphSageMemoryEstimateDefinition {
    train_config: GraphSageTrainConfig,
    mutating: bool,
}

impl GraphSageMemoryEstimateDefinition {
    pub fn new(train_config: GraphSageTrainConfig, mutating: bool) -> Self {
        Self {
            train_config,
            mutating,
        }
    }
}

impl MemoryEstimation for GraphSageMemoryEstimateDefinition {
    fn description(&self) -> String {
        "GraphSage".to_string()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let node_count = dimensions.node_count();
        let train_params =
            TrainConfigTransformer::to_memory_estimate_parameters(&self.train_config);

        // initialFeatures: per node double[] of estimationFeatureDimension
        let initial_features =
            node_count * Estimate::size_of_double_array(train_params.estimation_feature_dimension);

        // resultFeatures: per node double[] of embeddingDimension (resident if mutating, otherwise temporary in Java)
        let result_features =
            node_count * Estimate::size_of_double_array(train_params.embedding_dimension);

        // A coarse per-thread estimate: embeddings computation graph allocs roughly scale with batchSize.
        let per_thread_batch = (3 * train_params.batch_size).max(1);
        let per_thread =
            per_thread_batch * Estimate::size_of_double_array(train_params.embedding_dimension);

        let mut temporary_components = vec![
            MemoryTree::leaf(
                "this.instance".into(),
                MemoryRange::of(Estimate::BYTES_OBJECT_HEADER),
            ),
            MemoryTree::leaf("initialFeatures".into(), MemoryRange::of(initial_features)),
            MemoryTree::leaf(
                "concurrentBatches".into(),
                MemoryRange::of(per_thread).times(concurrency.max(1)),
            ),
        ];

        let mut resident_components = Vec::new();

        if self.mutating {
            resident_components.push(MemoryTree::leaf(
                "resultFeatures".into(),
                MemoryRange::of(result_features),
            ));
        } else {
            temporary_components.push(MemoryTree::leaf(
                "resultFeatures".into(),
                MemoryRange::of(result_features),
            ));
        }

        let resident_mem = MemoryTree::new(
            "residentMemory".into(),
            sum_ranges(&resident_components),
            resident_components,
        );
        let temp_mem = MemoryTree::new(
            "temporaryMemory".into(),
            sum_ranges(&temporary_components),
            temporary_components,
        );

        MemoryTree::new(
            "GraphSage".into(),
            sum_ranges(&[resident_mem.clone(), temp_mem.clone()]),
            vec![resident_mem, temp_mem],
        )
    }
}

fn sum_ranges(children: &[MemoryTree]) -> MemoryRange {
    children
        .iter()
        .fold(MemoryRange::empty(), |acc, t| acc.add(t.memory_usage()))
}
