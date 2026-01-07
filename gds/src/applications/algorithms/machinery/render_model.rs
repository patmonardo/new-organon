//! RenderModel(s) (Java parity).
//!
//! Java GDS exposes explicit "render models" per mode (stats/stream/mutate/write)
//! that bundle the renderer and any side-effect metadata wiring.

use crate::core::loading::GraphResources;

use super::{
    AlgorithmProcessingTimings, MutateResultBuilder, MutateResultRenderer, ResultRenderer,
    SideEffect, StatsResultBuilder, StatsResultRenderer, StreamResultBuilder, StreamResultRenderer,
    WriteResultBuilder, WriteResultRenderer,
};

pub trait RenderModel {
    type ResultFromAlgorithm;
    type ResultToCaller;
    type Metadata;

    fn renderer(
        &self,
    ) -> &dyn ResultRenderer<Self::ResultFromAlgorithm, Self::ResultToCaller, Self::Metadata>;

    fn side_effect(&self) -> Option<&dyn SideEffect<Self::ResultFromAlgorithm, Self::Metadata>>;
}

pub struct StatsRenderModel<ResultFromAlgorithm, ResultToCaller, B>(
    pub StatsResultRenderer<B>,
    pub std::marker::PhantomData<(ResultFromAlgorithm, ResultToCaller)>,
);

impl<ResultFromAlgorithm, ResultToCaller, B>
    StatsRenderModel<ResultFromAlgorithm, ResultToCaller, B>
{
    pub fn new(builder: B) -> Self {
        Self(StatsResultRenderer(builder), std::marker::PhantomData)
    }
}

impl<ResultFromAlgorithm, ResultToCaller, B> RenderModel
    for StatsRenderModel<ResultFromAlgorithm, ResultToCaller, B>
where
    B: StatsResultBuilder<ResultFromAlgorithm, ResultToCaller>,
{
    type ResultFromAlgorithm = ResultFromAlgorithm;
    type ResultToCaller = ResultToCaller;
    type Metadata = ();

    fn renderer(
        &self,
    ) -> &dyn ResultRenderer<Self::ResultFromAlgorithm, Self::ResultToCaller, Self::Metadata> {
        &self.0
    }

    fn side_effect(&self) -> Option<&dyn SideEffect<Self::ResultFromAlgorithm, Self::Metadata>> {
        None
    }
}

pub struct StreamRenderModel<ResultFromAlgorithm, Row, B>(
    pub StreamResultRenderer<Row, B>,
    pub std::marker::PhantomData<(ResultFromAlgorithm, Row)>,
);

impl<ResultFromAlgorithm, Row, B> StreamRenderModel<ResultFromAlgorithm, Row, B> {
    pub fn new(builder: B) -> Self {
        Self(StreamResultRenderer::new(builder), std::marker::PhantomData)
    }
}

impl<ResultFromAlgorithm, Row, B> RenderModel for StreamRenderModel<ResultFromAlgorithm, Row, B>
where
    B: StreamResultBuilder<ResultFromAlgorithm, Row>,
{
    type ResultFromAlgorithm = ResultFromAlgorithm;
    type ResultToCaller = B::Stream;
    type Metadata = ();

    fn renderer(
        &self,
    ) -> &dyn ResultRenderer<Self::ResultFromAlgorithm, Self::ResultToCaller, Self::Metadata> {
        &self.0
    }

    fn side_effect(&self) -> Option<&dyn SideEffect<Self::ResultFromAlgorithm, Self::Metadata>> {
        None
    }
}

pub struct MutateRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S> {
    pub renderer:
        MutateResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>,
    pub side_effect: Option<S>,
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
    MutateRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, ()>
where
    B: MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
{
    pub fn new(configuration: Configuration, builder: B) -> Self {
        Self {
            renderer: MutateResultRenderer::new(configuration, builder),
            side_effect: None,
        }
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S>
    MutateRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S>
where
    B: MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
{
    pub fn with_side_effect(configuration: Configuration, builder: B, side_effect: S) -> Self {
        Self {
            renderer: MutateResultRenderer::new(configuration, builder),
            side_effect: Some(side_effect),
        }
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S> RenderModel
    for MutateRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S>
where
    B: MutateResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
    S: SideEffect<ResultFromAlgorithm, Metadata>,
{
    type ResultFromAlgorithm = ResultFromAlgorithm;
    type ResultToCaller = ResultToCaller;
    type Metadata = Metadata;

    fn renderer(
        &self,
    ) -> &dyn ResultRenderer<Self::ResultFromAlgorithm, Self::ResultToCaller, Self::Metadata> {
        &self.renderer
    }

    fn side_effect(&self) -> Option<&dyn SideEffect<Self::ResultFromAlgorithm, Self::Metadata>> {
        self.side_effect.as_ref().map(|se| se as _)
    }
}

pub struct WriteRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S> {
    pub renderer:
        WriteResultRenderer<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>,
    pub side_effect: Option<S>,
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B>
    WriteRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, ()>
where
    B: WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
{
    pub fn new(configuration: Configuration, builder: B) -> Self {
        Self {
            renderer: WriteResultRenderer::new(configuration, builder),
            side_effect: None,
        }
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S>
    WriteRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S>
where
    B: WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
{
    pub fn with_side_effect(configuration: Configuration, builder: B, side_effect: S) -> Self {
        Self {
            renderer: WriteResultRenderer::new(configuration, builder),
            side_effect: Some(side_effect),
        }
    }
}

impl<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S> RenderModel
    for WriteRenderModel<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata, B, S>
where
    B: WriteResultBuilder<Configuration, ResultFromAlgorithm, ResultToCaller, Metadata>,
    S: SideEffect<ResultFromAlgorithm, Metadata>,
{
    type ResultFromAlgorithm = ResultFromAlgorithm;
    type ResultToCaller = ResultToCaller;
    type Metadata = Metadata;

    fn renderer(
        &self,
    ) -> &dyn ResultRenderer<Self::ResultFromAlgorithm, Self::ResultToCaller, Self::Metadata> {
        &self.renderer
    }

    fn side_effect(&self) -> Option<&dyn SideEffect<Self::ResultFromAlgorithm, Self::Metadata>> {
        self.side_effect.as_ref().map(|se| se as _)
    }
}

// Keep the import of GraphResources/Timings referenced in this module (also makes the signatures obvious).
#[allow(dead_code)]
fn _type_anchor(_gr: &GraphResources, _timings: AlgorithmProcessingTimings) {}
