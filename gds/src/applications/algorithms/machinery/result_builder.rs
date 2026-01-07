//! Result builders (Java parity).

use crate::core::loading::GraphResources;
use std::marker::PhantomData;

use super::AlgorithmProcessingTimings;

pub trait StatsResultBuilder<ResultFromAlgorithm, ResultToCaller> {
    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
    ) -> ResultToCaller;
}

/// Named adapter so “builder objects” read like Java.
pub struct FnStatsResultBuilder<F>(pub F);

impl<ResultFromAlgorithm, ResultToCaller, F> StatsResultBuilder<ResultFromAlgorithm, ResultToCaller>
    for FnStatsResultBuilder<F>
where
    F: Fn(
            &GraphResources,
            Option<ResultFromAlgorithm>,
            AlgorithmProcessingTimings,
        ) -> ResultToCaller
        + Send
        + Sync,
{
    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
    ) -> ResultToCaller {
        (self.0)(graph_resources, result, timings)
    }
}

pub trait StreamResultBuilder<ResultFromAlgorithm, Row> {
    type Stream: Iterator<Item = Row>;

    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
    ) -> Self::Stream;
}

/// Named adapter so “builder objects” read like Java.
pub struct FnStreamResultBuilder<F, Stream>(pub F, PhantomData<Stream>);

impl<F, Stream> FnStreamResultBuilder<F, Stream> {
    pub fn new(func: F) -> Self {
        Self(func, PhantomData)
    }
}

impl<ResultFromAlgorithm, Row, Stream, F> StreamResultBuilder<ResultFromAlgorithm, Row>
    for FnStreamResultBuilder<F, Stream>
where
    Stream: Iterator<Item = Row>,
    F: Fn(&GraphResources, Option<ResultFromAlgorithm>) -> Stream + Send + Sync,
{
    type Stream = Stream;

    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<ResultFromAlgorithm>,
    ) -> Self::Stream {
        (self.0)(graph_resources, result)
    }
}

/// Java-parity builder for MUTATE mode.
///
/// Mutate/Write modes typically combine:
/// - configuration (the request)
/// - algorithm result
/// - timings
/// - metadata from side effects (e.g. counts)
pub trait MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata> {
    fn build(
        &self,
        graph_resources: &GraphResources,
        configuration: &Configuration,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller;
}

/// Named adapter so “builder objects” read like Java.
pub struct FnMutateResultBuilder<F>(pub F);

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, F>
    MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>
    for FnMutateResultBuilder<F>
where
    F: Fn(
            &GraphResources,
            &Configuration,
            Option<ResultFromAlgorithm>,
            AlgorithmProcessingTimings,
            Option<Metadata>,
        ) -> ResultToCaller
        + Send
        + Sync,
{
    fn build(
        &self,
        graph_resources: &GraphResources,
        configuration: &Configuration,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller {
        (self.0)(graph_resources, configuration, result, timings, metadata)
    }
}

/// Java-parity builder for WRITE mode.
///
/// We keep it separate from mutate even if the signature is the same, so the
/// control-flow reads like Java and can diverge later.
pub trait WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata> {
    fn build(
        &self,
        graph_resources: &GraphResources,
        configuration: &Configuration,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller;
}

/// Named adapter so “builder objects” read like Java.
pub struct FnWriteResultBuilder<F>(pub F);

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, F>
    WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>
    for FnWriteResultBuilder<F>
where
    F: Fn(
            &GraphResources,
            &Configuration,
            Option<ResultFromAlgorithm>,
            AlgorithmProcessingTimings,
            Option<Metadata>,
        ) -> ResultToCaller
        + Send
        + Sync,
{
    fn build(
        &self,
        graph_resources: &GraphResources,
        configuration: &Configuration,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller {
        (self.0)(graph_resources, configuration, result, timings, metadata)
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, F>
    MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata> for F
where
    F: Fn(
            &GraphResources,
            &Configuration,
            Option<ResultFromAlgorithm>,
            AlgorithmProcessingTimings,
            Option<Metadata>,
        ) -> ResultToCaller
        + Send
        + Sync,
{
    fn build(
        &self,
        graph_resources: &GraphResources,
        configuration: &Configuration,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller {
        (self)(graph_resources, configuration, result, timings, metadata)
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, F>
    WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata> for F
where
    F: Fn(
            &GraphResources,
            &Configuration,
            Option<ResultFromAlgorithm>,
            AlgorithmProcessingTimings,
            Option<Metadata>,
        ) -> ResultToCaller
        + Send
        + Sync,
{
    fn build(
        &self,
        graph_resources: &GraphResources,
        configuration: &Configuration,
        result: Option<ResultFromAlgorithm>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Metadata>,
    ) -> ResultToCaller {
        (self)(graph_resources, configuration, result, timings, metadata)
    }
}

// Placeholder to make it easy to store builders generically.
#[allow(dead_code)]
pub struct BuilderPhantom<CONFIGURATION>(PhantomData<CONFIGURATION>);
