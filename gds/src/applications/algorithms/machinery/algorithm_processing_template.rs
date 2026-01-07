//! AlgorithmProcessingTemplate (Java parity, simplified).
//!
//! This is the orchestration layer that composes:
//! - progress tracker creation
//! - algorithm execution (via AlgorithmMachinery)
//! - optional side effects
//! - result rendering

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{ProgressTracker, Task};
use crate::errors::MemoryEstimationError;
use crate::mem::MemoryTreeWithDimensions;

use super::{
    AlgoBaseConfigLike, AlgorithmMachinery, AlgorithmProcessingTimings,
    AlgorithmProcessingTimingsBuilder, DimensionTransformer, Label, MemoryGuard,
    ProgressTrackerCreator, ResultRenderer, SideEffect,
};

pub trait AlgorithmProcessingTemplate {
    /// Lowest common denominator processing pipeline.
    ///
    /// This is intentionally generic; concrete algorithm facades provide the
    /// `compute` closure and renderer/side-effect implementations.
    fn process<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata, E>(
        &self,
        graph_resources: &GraphResources,
        concurrency: Concurrency,
        task: Task,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<ResultFromAlgorithm>, E>,
        renderer: &dyn ResultRenderer<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata>,
        side_effect: Option<&dyn SideEffect<ResultFromAlgorithm, SideEffectMetadata>>,
    ) -> Result<ResultToCaller, E>;

    /// Java-parity: run a `MemoryGuard` assertion before processing.
    ///
    /// This mirrors the Java `ComputationService.computeAlgorithm(...)` flow:
    /// guard first, then compute/render pipeline.
    fn process_with_memory_guard<
        Configuration,
        ResultFromAlgorithm,
        ResultToCaller,
        SideEffectMetadata,
        G,
        E,
    >(
        &self,
        username: &str,
        memory_guard: &G,
        graph_resources: &GraphResources,
        configuration: &Configuration,
        label: &dyn Label,
        estimation_factory: impl FnOnce(
            &GraphResources,
            &Configuration,
        )
            -> Result<MemoryTreeWithDimensions, MemoryEstimationError>,
        dimension_transformer: &dyn DimensionTransformer,
        concurrency: Concurrency,
        task: Task,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<ResultFromAlgorithm>, E>,
        renderer: &dyn ResultRenderer<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata>,
        side_effect: Option<&dyn SideEffect<ResultFromAlgorithm, SideEffectMetadata>>,
    ) -> Result<ResultToCaller, ProcessWithMemoryGuardError<E>>
    where
        Configuration: AlgoBaseConfigLike,
        G: MemoryGuard,
    {
        memory_guard
            .assert_algorithm_can_run(
                username,
                estimation_factory,
                graph_resources,
                configuration,
                label,
                dimension_transformer,
            )
            .map_err(ProcessWithMemoryGuardError::Guard)?;

        self.process(
            graph_resources,
            concurrency,
            task,
            compute,
            renderer,
            side_effect,
        )
        .map_err(ProcessWithMemoryGuardError::Compute)
    }
}

#[derive(Debug)]
pub enum ProcessWithMemoryGuardError<E> {
    Guard(super::MemoryGuardError),
    Compute(E),
}

pub struct DefaultAlgorithmProcessingTemplate {
    pub progress_tracker_creator: ProgressTrackerCreator,
}

impl DefaultAlgorithmProcessingTemplate {
    pub fn new(progress_tracker_creator: ProgressTrackerCreator) -> Self {
        Self {
            progress_tracker_creator,
        }
    }
}

impl AlgorithmProcessingTemplate for DefaultAlgorithmProcessingTemplate {
    fn process<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata, E>(
        &self,
        graph_resources: &GraphResources,
        concurrency: Concurrency,
        task: Task,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<ResultFromAlgorithm>, E>,
        renderer: &dyn ResultRenderer<ResultFromAlgorithm, ResultToCaller, SideEffectMetadata>,
        side_effect: Option<&dyn SideEffect<ResultFromAlgorithm, SideEffectMetadata>>,
    ) -> Result<ResultToCaller, E> {
        let start_pre_processing = std::time::Instant::now();
        // Java parity: tracker is request-scoped via JobId + TaskRegistryFactory.
        let mut tracker = self
            .progress_tracker_creator
            .create_progress_tracker(concurrency, task);

        let termination_flag = self.progress_tracker_creator.termination_flag().clone();

        let pre_processing_millis = start_pre_processing.elapsed().as_millis() as i64;

        // Java parity: the machinery owns the progress lifecycle (begin/end/failure/release).
        AlgorithmMachinery::run_algorithms_and_manage_progress_tracker(
            &mut tracker,
            true,
            concurrency,
            |tracker| {
                let mut timings_builder = AlgorithmProcessingTimingsBuilder::new();
                timings_builder.with_pre_processing_millis(pre_processing_millis);

                let start_compute = std::time::Instant::now();
                let computed = compute(graph_resources, tracker, &termination_flag)?;
                timings_builder.with_compute_millis(start_compute.elapsed().as_millis() as i64);

                let start_side_effect = std::time::Instant::now();
                let metadata =
                    side_effect.and_then(|se| se.process(graph_resources, computed.as_ref()));
                timings_builder
                    .with_side_effect_millis(start_side_effect.elapsed().as_millis() as i64);

                let timings: AlgorithmProcessingTimings = timings_builder.build();
                Ok(renderer.render(graph_resources, computed, timings, metadata))
            },
        )
    }
}
