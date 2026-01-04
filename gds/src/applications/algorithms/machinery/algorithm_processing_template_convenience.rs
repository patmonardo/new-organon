//! AlgorithmProcessingTemplate convenience wiring (Java parity).
//!
//! Java typically exposes a convenience layer that assembles:
//! - the processing template
//! - the mode-specific renderer model (stats/stream/mutate/write)
//! - optional mutate/write side effects
//!
//! This file keeps that "one obvious entry point per mode" surface.

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{ProgressTracker, Task};

use super::{
    AlgorithmProcessingTemplate, MutateResultBuilder, MutateResultRenderer, SideEffect,
    StatsResultBuilder, StatsResultRenderer, StreamResultBuilder, StreamResultRenderer,
    WriteResultBuilder, WriteResultRenderer,
};

use super::RenderModel;

pub struct AlgorithmProcessingTemplateConvenience<T> {
    pub template: T,
}

impl<T> AlgorithmProcessingTemplateConvenience<T>
where
    T: AlgorithmProcessingTemplate,
{
    pub fn new(template: T) -> Self {
        Self { template }
    }

    pub fn process_stats<ResultFromAlgorithm, ResultToCaller, E>(
        &self,
        graph_resources: &GraphResources,
        concurrency: Concurrency,
        task: Task,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<ResultFromAlgorithm>, E>,
        builder: impl StatsResultBuilder<ResultFromAlgorithm, ResultToCaller>,
    ) -> Result<ResultToCaller, E> {
        let renderer = StatsResultRenderer(builder);
        self.template
            .process(graph_resources, concurrency, task, compute, &renderer, None)
    }

    pub fn process_stream<ResultFromAlgorithm, Row, B, E>(
        &self,
        graph_resources: &GraphResources,
        concurrency: Concurrency,
        task: Task,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<ResultFromAlgorithm>, E>,
        builder: B,
    ) -> Result<B::Stream, E>
    where
        B: StreamResultBuilder<ResultFromAlgorithm, Row>,
    {
        let renderer = StreamResultRenderer::<Row, B>::new(builder);
        self.template
            .process(graph_resources, concurrency, task, compute, &renderer, None)
    }

    /// Java-parity: generic entry point that takes a mode-specific `RenderModel`.
    pub fn process_with_model<M, E>(
        &self,
        graph_resources: &GraphResources,
        concurrency: Concurrency,
        task: Task,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<M::ResultFromAlgorithm>, E>,
        model: &M,
    ) -> Result<M::ResultToCaller, E>
    where
        M: RenderModel,
    {
        self.template.process(
            graph_resources,
            concurrency,
            task,
            compute,
            model.renderer(),
            model.side_effect(),
        )
    }

    pub fn process_mutate<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, E>(
        &self,
        graph_resources: &GraphResources,
        concurrency: Concurrency,
        task: Task,
        configuration: Configuration,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<ResultFromAlgorithm>, E>,
        builder: B,
        side_effect: Option<&dyn SideEffect<ResultFromAlgorithm, Metadata>>,
    ) -> Result<ResultToCaller, E>
    where
        B: MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
    {
        let renderer = MutateResultRenderer::new(configuration, builder);
        self.template.process(
            graph_resources,
            concurrency,
            task,
            compute,
            &renderer,
            side_effect,
        )
    }

    pub fn process_write<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, E>(
        &self,
        graph_resources: &GraphResources,
        concurrency: Concurrency,
        task: Task,
        configuration: Configuration,
        compute: impl FnOnce(
            &GraphResources,
            &mut dyn ProgressTracker,
            &TerminationFlag,
        ) -> Result<Option<ResultFromAlgorithm>, E>,
        builder: B,
        side_effect: Option<&dyn SideEffect<ResultFromAlgorithm, Metadata>>,
    ) -> Result<ResultToCaller, E>
    where
        B: WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
    {
        // Java parity: write is a first-class mode even if its internal semantics are mocked.
        let renderer = WriteResultRenderer::new(configuration, builder);
        self.template.process(
            graph_resources,
            concurrency,
            task,
            compute,
            &renderer,
            side_effect,
        )
    }
}
