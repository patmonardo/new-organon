use crate::applications::algorithms::machinery::SideEffect;
use crate::core::loading::GraphResources;
use crate::projection::eval::pipeline::node_pipeline::regression::node_regression_pipeline_train_config::NodeRegressionPipelineTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::regression::node_regression_train_result::NodeRegressionTrainPipelineResult;

pub struct NodeRegressionTrainSideEffects {
    #[allow(dead_code)]
    configuration: NodeRegressionPipelineTrainConfig,
}

impl NodeRegressionTrainSideEffects {
    pub fn new(configuration: NodeRegressionPipelineTrainConfig) -> Self {
        Self { configuration }
    }
}

impl SideEffect<NodeRegressionTrainPipelineResult, ()> for NodeRegressionTrainSideEffects {
    fn process(
        &self,
        _graph_resources: &GraphResources,
        _result: Option<&NodeRegressionTrainPipelineResult>,
    ) -> Option<()> {
        None
    }
}
