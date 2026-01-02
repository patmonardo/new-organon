use crate::api::{Graph, GraphStore, GraphName};
use crate::applications::algorithms::machinery::{
    AlgorithmLabel, MutateStep, WriteStep, ResultBuilder, StreamResultBuilder, StatsResultBuilder,
    DimensionTransformer,
};
use crate::config::base_types::Config;
use crate::mem::MemoryEstimation;
use crate::core::loading::{PostLoadValidationHook, PostLoadETLHook};

use super::{ResultRenderer, SideEffect};

/// The core Algorithm Processing Template interface.
/// This defines the ISA (Instruction Set Architecture) for our Platonic Form Processor.
/// All algorithms follow the same execution pattern:
/// 1. Load data
/// 2. Compute algorithm
/// 3. Process any side effects (mutate/write)
/// 4. Render a result
pub trait AlgorithmProcessingTemplate {
    /// Process algorithm for write mode with full configuration
    fn process_algorithm_for_write<
        ConfigT: Config,
        ResultToCaller,
        ResultFromAlgorithm,
        WriteMetadata,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: ConfigT,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> ResultFromAlgorithm,
        write_step: Box<dyn WriteStep<ResultFromAlgorithm, WriteMetadata>>,
        result_builder: Box<dyn ResultBuilder<ConfigT, ResultFromAlgorithm, ResultToCaller, WriteMetadata>>,
    ) -> ResultToCaller;

    /// Process algorithm for mutate mode with full configuration
    fn process_algorithm_for_mutate<
        ConfigT: Config,
        ResultToCaller,
        ResultFromAlgorithm,
        MutateMetadata,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: ConfigT,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> ResultFromAlgorithm,
        mutate_step: Box<dyn MutateStep<ResultFromAlgorithm, MutateMetadata>>,
        result_builder: Box<dyn ResultBuilder<ConfigT, ResultFromAlgorithm, ResultToCaller, MutateMetadata>>,
    ) -> ResultToCaller;

    /// Process algorithm for stream mode with full configuration
    fn process_algorithm_for_stream<
        ConfigT: Config,
        ResultToCaller,
        ResultFromAlgorithm,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: ConfigT,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> ResultFromAlgorithm,
        result_builder: Box<dyn StreamResultBuilder<ResultFromAlgorithm, ResultToCaller>>,
    ) -> Vec<ResultToCaller>;

    /// Process algorithm for stats mode with full configuration
    fn process_algorithm_for_stats<
        ConfigT: Config,
        ResultToCaller,
        ResultFromAlgorithm,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: ConfigT,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> ResultFromAlgorithm,
        result_builder: Box<dyn StatsResultBuilder<ResultFromAlgorithm, ResultToCaller>>,
    ) -> ResultToCaller;

    /// Process algorithm with any side effects - the core template method
    fn process_algorithm_and_any_side_effects<
        ConfigT: Config,
        ResultToCaller,
        ResultFromAlgorithm,
        SideEffectMetadata,
    >(
        &self,
        relationship_weight_override: Option<String>,
        graph_name: GraphName,
        configuration: ConfigT,
        post_graph_store_load_validation_hooks: Option<Vec<Box<dyn PostLoadValidationHook>>>,
        post_graph_store_load_etl_hooks: Option<Vec<Box<dyn PostLoadETLHook>>>,
        label: AlgorithmLabel,
        dimension_transformer: Box<dyn DimensionTransformer>,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        computation: impl Fn(Graph, GraphStore) -> ResultFromAlgorithm,
        side_effect: Option<Box<dyn SideEffect<ResultFromAlgorithm, SideEffectMetadata>>>,
        result_renderer: Box<dyn ResultRenderer<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata>>,
    ) -> ResultToCaller;
}
