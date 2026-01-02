use crate::core::loading::GraphResources;
use crate::applications::algorithms::machinery::AlgorithmProcessingTimings;
use crate::applications::algorithms::machinery::{ResultBuilder, StatsResultBuilder, StreamResultBuilder};

/// Result Renderer - renders results from algorithm execution
pub trait ResultRenderer<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata> {
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<SideEffectMetadata>,
    ) -> ResultToCaller;
}

/// Mutate Result Renderer - renders results for mutate mode
pub struct MutateResultRenderer<
    ResultFromAlgorithm,
    ResultToCaller,
    MutateMetadata,
    ConfigT: crate::config::base_types::Config,
> {
    configuration: ConfigT,
    result_builder: Box<dyn ResultBuilder<ConfigT, ResultFromAlgorithm, ResultToCaller, MutateMetadata>>,
}

impl<ResultFromAlgorithm, ResultToCaller, MutateMetadata, ConfigT: crate::config::base_types::Config>
    MutateResultRenderer<ResultFromAlgorithm, ResultToCaller, MutateMetadata, ConfigT>
{
    pub fn new(
        configuration: ConfigT,
        result_builder: Box<dyn ResultBuilder<ConfigT, ResultFromAlgorithm, ResultToCaller, MutateMetadata>>,
    ) -> Self {
        Self {
            configuration,
            result_builder,
        }
    }
}

impl<ResultFromAlgorithm, ResultToCaller, MutateMetadata, ConfigT: crate::config::base_types::Config>
    ResultRenderer<ResultFromAlgorithm, ResultToCaller, MutateMetadata>
    for MutateResultRenderer<ResultFromAlgorithm, ResultToCaller, MutateMetadata, ConfigT>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<MutateMetadata>,
    ) -> ResultToCaller {
        self.result_builder.build(
            graph_resources.graph.clone(),
            &self.configuration,
            result,
            timings,
            metadata,
        )
    }
}

/// Write Result Renderer - renders results for write mode
pub struct WriteResultRenderer<
    ResultFromAlgorithm,
    ResultToCaller,
    WriteMetadata,
    ConfigT: crate::config::base_types::Config,
> {
    configuration: ConfigT,
    result_builder: Box<dyn ResultBuilder<ConfigT, ResultFromAlgorithm, ResultToCaller, WriteMetadata>>,
}

impl<ResultFromAlgorithm, ResultToCaller, WriteMetadata, ConfigT: crate::config::base_types::Config>
    WriteResultRenderer<ResultFromAlgorithm, ResultToCaller, WriteMetadata, ConfigT>
{
    pub fn new(
        configuration: ConfigT,
        result_builder: Box<dyn ResultBuilder<ConfigT, ResultFromAlgorithm, ResultToCaller, WriteMetadata>>,
    ) -> Self {
        Self {
            configuration,
            result_builder,
        }
    }
}

impl<ResultFromAlgorithm, ResultToCaller, WriteMetadata, ConfigT: crate::config::base_types::Config>
    ResultRenderer<ResultFromAlgorithm, ResultToCaller, WriteMetadata>
    for WriteResultRenderer<ResultFromAlgorithm, ResultToCaller, WriteMetadata, ConfigT>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<WriteMetadata>,
    ) -> ResultToCaller {
        self.result_builder.build(
            graph_resources.graph.clone(),
            &self.configuration,
            result,
            timings,
            metadata,
        )
    }
}

/// Stats Result Renderer - renders results for stats mode
pub struct StatsResultRenderer<ResultFromAlgorithm, ResultToCaller> {
    stats_result_builder: Box<dyn StatsResultBuilder<ResultFromAlgorithm, ResultToCaller>>,
}

impl<ResultFromAlgorithm, ResultToCaller> StatsResultRenderer<ResultFromAlgorithm, ResultToCaller> {
    pub fn new(stats_result_builder: Box<dyn StatsResultBuilder<ResultFromAlgorithm, ResultToCaller>>) -> Self {
        Self { stats_result_builder }
    }
}

impl<ResultFromAlgorithm, ResultToCaller> ResultRenderer<ResultFromAlgorithm, ResultToCaller, ()>
    for StatsResultRenderer<ResultFromAlgorithm, ResultToCaller>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> ResultToCaller {
        self.stats_result_builder.build(
            graph_resources.graph.clone(),
            result,
            timings,
        )
    }
}

/// Stream Result Renderer - renders results for stream mode
pub struct StreamResultRenderer<ResultFromAlgorithm, ResultToCaller> {
    result_builder: Box<dyn StreamResultBuilder<ResultFromAlgorithm, ResultToCaller>>,
}

impl<ResultFromAlgorithm, ResultToCaller> StreamResultRenderer<ResultFromAlgorithm, ResultToCaller> {
    pub fn new(result_builder: Box<dyn StreamResultBuilder<ResultFromAlgorithm, ResultToCaller>>) -> Self {
        Self { result_builder }
    }
}

impl<ResultFromAlgorithm, ResultToCaller> ResultRenderer<ResultFromAlgorithm, Vec<ResultToCaller>, ()>
    for StreamResultRenderer<ResultFromAlgorithm, ResultToCaller>
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        _timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> Vec<ResultToCaller> {
        self.result_builder.build(
            graph_resources.graph.clone(),
            graph_resources.graph_store.clone(),
            result,
        )
    }
}
