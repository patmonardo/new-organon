//! ML ISA (Machine Learning Evaluation).
//!
//! This module is the ML side of Projection Eval.
//!
//! **Current status (2025-12):** the actively-maintained and compiling ML surface is the
//! Java GDS pipeline translation under `ml/pipeline/**`.
//!
//! The older “descriptor-driven” runtime stubs (PipelineDescriptor/StepDescriptor, etc.) are
//! intentionally not wired into the module tree right now, because the descriptor tables are
//! being migrated toward a proc-macro injected Reality layer (the “Island of Truth”).

/// Java GDS pipeline translation (core ML runtime surface today).
pub mod pipeline;

// Re-export the pipeline surface as the ML ISA entrypoint.
pub use pipeline::*;
