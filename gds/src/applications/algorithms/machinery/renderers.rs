//! Concrete ResultRenderer implementations (Java parity).

use crate::core::loading::GraphResources;

use super::{
    AlgorithmProcessingTimings, MutateResultBuilder, ResultRenderer, StatsResultBuilder,
    StreamResultBuilder, WriteResultBuilder,
};

pub struct StatsResultRenderer<B>(pub B);

impl<ResultFromAlgorithm, ResultToCaller, B> ResultRenderer<ResultFromAlgorithm, ResultToCaller, ()>
    for StatsResultRenderer<B>
where
    B: StatsResultBuilder<ResultFromAlgorithm, ResultToCaller>,
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> ResultToCaller {
        self.0.build(graph_resources, result, timings)
    }
}

// Note: ROW is a type-level marker only (needed to satisfy Rust's generic constraints).
pub struct StreamResultRenderer<Row, B>(pub B, pub std::marker::PhantomData<Row>);

impl<Row, B> StreamResultRenderer<Row, B> {
    pub fn new(builder: B) -> Self {
        Self(builder, std::marker::PhantomData)
    }
}

impl<ResultFromAlgorithm, Row, B> ResultRenderer<ResultFromAlgorithm, B::Stream, ()>
    for StreamResultRenderer<Row, B>
where
    B: StreamResultBuilder<ResultFromAlgorithm, Row>,
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        _timings: AlgorithmProcessingTimings,
        _metadata: Option<()>,
    ) -> B::Stream {
        self.0.build(graph_resources, result)
    }
}

/// Java-parity renderer model for MUTATE mode.
pub struct MutateResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B> {
    pub configuration: Configuration,
    pub result_builder: B,
    pub _phantom: std::marker::PhantomData<(ResultFromAlgorithm, ResultToCaller, Metadata)>,
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
    MutateResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
{
    pub fn new(configuration: Configuration, result_builder: B) -> Self {
        Self {
            configuration,
            result_builder,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
    ResultRenderer<ResultFromAlgorithm, ResultToCaller, Metadata>
    for MutateResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
where
    B: MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller {
        self.result_builder.build(
            graph_resources,
            &self.configuration,
            result,
            timings,
            metadata,
        )
    }
}

/// Java-parity renderer model for WRITE mode.
///
/// For now this is intentionally a thin shell; we will flesh out database-specific
/// semantics later, but the control-flow shape is correct.
pub struct WriteResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B> {
    pub configuration: Configuration,
    pub result_builder: B,
    pub _phantom: std::marker::PhantomData<(ResultFromAlgorithm, ResultToCaller, Metadata)>,
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
    WriteResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
{
    pub fn new(configuration: Configuration, result_builder: B) -> Self {
        Self {
            configuration,
            result_builder,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
    ResultRenderer<ResultFromAlgorithm, ResultToCaller, Metadata>
    for WriteResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
where
    B: WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
{
    fn render(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller {
        self.result_builder.build(
            graph_resources,
            &self.configuration,
            result,
            timings,
            metadata,
        )
    }
}
