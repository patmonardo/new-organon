use std::marker::PhantomData;

use crate::api::Graph;

use super::AlgorithmProcessingTimings;

/// Interface for building results from algorithm execution.
/// This is a core pattern in the Applications system for transforming
/// algorithm results into different output formats.
pub trait ResultBuilder<CONFIG, RESULT, OUTPUT, META> {
    /// Builds the final result from the algorithm execution.
    ///
    /// # Arguments
    /// * `graph` - The loaded graph
    /// * `config` - The algorithm configuration
    /// * `result` - The algorithm result (if computation ran)
    /// * `timings` - Wall-clock timings for processing
    /// * `meta` - Metadata about the execution (e.g., properties written)
    ///
    /// # Returns
    /// The final output result
    fn build(
        &self,
        graph: Graph,
        config: &CONFIG,
        result: Option<RESULT>,
        timings: AlgorithmProcessingTimings,
        meta: Option<META>,
    ) -> OUTPUT;
}

/// Generic result builder that can be used for simple cases.
pub struct GenericResultBuilder<F, CONFIG, RESULT, OUTPUT, META>
where
    F: Fn(Graph, &CONFIG, Option<RESULT>, AlgorithmProcessingTimings, Option<META>) -> OUTPUT,
{
    build_fn: F,
    _phantom: PhantomData<(CONFIG, RESULT, OUTPUT, META)>,
}

impl<F, CONFIG, RESULT, OUTPUT, META> GenericResultBuilder<F, CONFIG, RESULT, OUTPUT, META>
where
    F: Fn(Graph, &CONFIG, Option<RESULT>, AlgorithmProcessingTimings, Option<META>) -> OUTPUT,
{
    pub fn new(build_fn: F) -> Self {
        Self {
            build_fn,
            _phantom: PhantomData,
        }
    }
}

impl<F, CONFIG, RESULT, OUTPUT, META> ResultBuilder<CONFIG, RESULT, OUTPUT, META>
    for GenericResultBuilder<F, CONFIG, RESULT, OUTPUT, META>
where
    F: Fn(Graph, &CONFIG, Option<RESULT>, AlgorithmProcessingTimings, Option<META>) -> OUTPUT,
{
    fn build(
        &self,
        graph: Graph,
        config: &CONFIG,
        result: Option<RESULT>,
        timings: AlgorithmProcessingTimings,
        meta: Option<META>,
    ) -> OUTPUT {
        (self.build_fn)(graph, config, result, timings, meta)
    }
}

/// Specialized result builders for different execution modes.

/// Result builder for streaming results.
pub trait StreamResultBuilder<RESULT, OUTPUT> {
    fn build(&self, graph: Graph, graph_store: crate::api::GraphStore, result: Option<RESULT>) -> Vec<OUTPUT>;
}

/// Result builder for statistics results.
pub trait StatsResultBuilder<RESULT, OUTPUT> {
    fn build(&self, graph: Graph, result: Option<RESULT>, timings: AlgorithmProcessingTimings) -> OUTPUT;
}

/// Result builder for mutation results.
pub trait MutateResultBuilder<CONFIG, RESULT, OUTPUT, META> {
    fn build_mutate(
        &self,
        graph: Graph,
        config: &CONFIG,
        result: Option<RESULT>,
        timings: AlgorithmProcessingTimings,
        meta: Option<META>,
    ) -> OUTPUT;
}

/// Result builder for write results.
pub trait WriteResultBuilder<CONFIG, RESULT, OUTPUT, META> {
    fn build_write(
        &self,
        graph: Graph,
        config: &CONFIG,
        result: Option<RESULT>,
        timings: AlgorithmProcessingTimings,
        meta: Option<META>,
    ) -> OUTPUT;
}

impl<T, CONFIG, RESULT, OUTPUT, META> MutateResultBuilder<CONFIG, RESULT, OUTPUT, META> for T
where
    T: ResultBuilder<CONFIG, RESULT, OUTPUT, META>,
{
    fn build_mutate(
        &self,
        graph: Graph,
        config: &CONFIG,
        result: Option<RESULT>,
        timings: AlgorithmProcessingTimings,
        meta: Option<META>,
    ) -> OUTPUT {
        self.build(graph, config, result, timings, meta)
    }
}

impl<T, CONFIG, RESULT, OUTPUT, META> WriteResultBuilder<CONFIG, RESULT, OUTPUT, META> for T
where
    T: ResultBuilder<CONFIG, RESULT, OUTPUT, META>,
{
    fn build_write(
        &self,
        graph: Graph,
        config: &CONFIG,
        result: Option<RESULT>,
        timings: AlgorithmProcessingTimings,
        meta: Option<META>,
    ) -> OUTPUT {
        self.build(graph, config, result, timings, meta)
    }
}
