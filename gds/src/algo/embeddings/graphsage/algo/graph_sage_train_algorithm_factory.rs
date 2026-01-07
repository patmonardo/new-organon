//! Java: `GraphSageTrainAlgorithmFactory`.

use crate::algo::embeddings::graphsage::algo::graph_sage_train::GraphSageTrain;
use crate::algo::embeddings::graphsage::algo::multi_label_graph_sage_train::MultiLabelGraphSageTrain;
use crate::algo::embeddings::graphsage::algo::single_label_graph_sage_train::SingleLabelGraphSageTrain;
use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskProgressTracker;
use crate::types::graph::Graph;
use std::sync::Arc;

pub struct GraphSageTrainAlgorithmFactory {
    gds_version: String,
}

impl GraphSageTrainAlgorithmFactory {
    pub fn new(gds_version: String) -> Self {
        Self { gds_version }
    }

    pub fn build(
        &self,
        graph: Arc<dyn Graph>,
        config: GraphSageTrainConfig,
        _progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Box<dyn GraphSageTrain> {
        if config.is_multi_label {
            Box::new(MultiLabelGraphSageTrain::new(
                graph,
                config,
                termination_flag,
                self.gds_version.clone(),
            ))
        } else {
            Box::new(SingleLabelGraphSageTrain::new(
                graph,
                config,
                termination_flag,
                self.gds_version.clone(),
            ))
        }
    }
}
