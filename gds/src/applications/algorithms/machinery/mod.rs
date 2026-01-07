//! Java GDS Applications "machinery" (Rust parity scaffold).
//!
//! This folder is intentionally self-contained and mirrors the responsibilities in
//! `org.neo4j.gds.applications.algorithms.machinery`:
//! - request-scoped dependencies (termination + progress/task registry)
//! - progress tracker creation
//! - algorithm runner that manages tracker lifecycle
//! - processing template that composes compute + result rendering + side effects
//!
//! Note: this module is a parity scaffold; it does not attempt to preserve any
//! previous AI-generated structure.

pub mod algorithm_machinery;
pub mod algorithm_processing_template;
pub mod algorithm_processing_template_convenience;
pub mod algorithm_processing_timings;
pub mod computation;
pub mod computation_service;
pub mod graph_store_service;
pub mod label;
pub mod memory_guard;
pub mod progress_tracker_creator;
pub mod render_model;
pub mod renderers;
pub mod request_scoped_dependencies;
pub mod result_builder;
pub mod result_renderer;
pub mod side_effect;
pub mod side_effects;
pub mod steps;
pub mod termination_aware_progress_tracker;
pub mod write_context;
pub mod write_to_database;

pub use algorithm_machinery::AlgorithmMachinery;
pub use algorithm_processing_template::{
    AlgorithmProcessingTemplate, DefaultAlgorithmProcessingTemplate,
};
pub use algorithm_processing_template_convenience::AlgorithmProcessingTemplateConvenience;
pub use algorithm_processing_timings::{
    AlgorithmProcessingTimings, AlgorithmProcessingTimingsBuilder,
};
pub use computation::Computation;
pub use computation_service::ComputationService;
pub use graph_store_service::{
    GraphStoreService, NodePropertiesWritten as GraphStoreNodePropertiesWritten, NodeProperty,
};
pub use label::{AlgorithmLabel, Label, StandardLabel};
pub use memory_guard::{
    AlgoBaseConfigLike, DefaultMemoryGuard, DimensionTransformer, DisabledMemoryGuard,
    IdentityDimensionTransformer, MemoryGuard, MemoryGuardError, DISABLED,
};
pub use progress_tracker_creator::DefaultProgressTrackerCreator;
pub use progress_tracker_creator::ProgressTrackerCreator;
pub use render_model::{
    MutateRenderModel, RenderModel, StatsRenderModel, StreamRenderModel, WriteRenderModel,
};
pub use renderers::{
    MutateResultRenderer, StatsResultRenderer, StreamResultRenderer, WriteResultRenderer,
};
pub use request_scoped_dependencies::RequestScopedDependencies;
pub use result_builder::{
    FnMutateResultBuilder, FnStatsResultBuilder, FnStreamResultBuilder, FnWriteResultBuilder,
    MutateResultBuilder, StatsResultBuilder, StreamResultBuilder, WriteResultBuilder,
};
pub use result_renderer::ResultRenderer;
pub use side_effect::{SideEffect, SideEffectExecutor};
pub use side_effects::{
    MutateSideEffect, MutateStepSideEffect, WriteSideEffect, WriteStepSideEffect,
};
pub use steps::{MutateStep, WriteStep};

pub use termination_aware_progress_tracker::TerminationAwareProgressTracker;

pub use write_context::{
    ExportBuildersProvider, ExporterContext, NodeLabelExporterBuilder, NodePropertyExporterBuilder,
    RelationshipExporterBuilder, RelationshipPropertiesExporterBuilder,
    RelationshipStreamExporterBuilder, WriteContext,
};
pub use write_to_database::{WriteConfigLike, WritePropertyConfigLike, WriteToDatabase};
