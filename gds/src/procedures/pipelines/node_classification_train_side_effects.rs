use crate::applications::algorithms::machinery::SideEffect;
use crate::core::loading::GraphResources;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_model_result::NodeClassificationModelResult;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;

use super::ModelPersister;

/// Side effects for node classification training.
///
/// Model persistence is not yet wired to the Rust model catalog; we currently
/// emit a warning when a model is produced.
pub struct NodeClassificationTrainSideEffects {
    model_persister: ModelPersister,
    #[allow(dead_code)]
    configuration: NodeClassificationPipelineTrainConfig,
}

impl NodeClassificationTrainSideEffects {
    pub fn new(
        model_persister: ModelPersister,
        configuration: NodeClassificationPipelineTrainConfig,
    ) -> Self {
        Self {
            model_persister,
            configuration,
        }
    }
}

impl SideEffect<NodeClassificationModelResult, ()> for NodeClassificationTrainSideEffects {
    fn process(
        &self,
        _graph_resources: &GraphResources,
        result: Option<&NodeClassificationModelResult>,
    ) -> Option<()> {
        if result.is_some() {
            self.model_persister
                .warn_unimplemented("NodeClassificationModelResult");
        }
        None
    }
}
