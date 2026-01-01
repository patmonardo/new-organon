//! Eval - Evaluation System
//!
//! This module provides the evaluation system for the GDS platform.
//! It contains the core evaluation logic and interfaces.
//!
//! ## The Three ISA Architecture
//!
//! ```
//! eval/procedure (Computation ISA)  ← AlgorithmSpec implementations
//! eval/pipeline (Pipeline ISA)                 ← Pipeline implementations
//! eval/form (Form ISA)             ← FormSpec implementations
//! ```

// Form (Form ISA) - Apodictic/Singular (ResultStore synthesis)
pub mod form;

// Runner - thin orchestration seam above the Three ISA
pub mod runner;

// Storage Processor - enterprise/storage boundary seam
pub mod storage_processor;

// Compute Processor - enterprise/runtime resource boundary seam
pub mod compute_processor;

// Procedure - Raising src/procedure infrastructure into consciousness
pub mod pipeline;
pub mod procedure;

pub use compute_processor::*;
pub use form::*;
pub use runner::*;
pub use storage_processor::*;

#[allow(ambiguous_glob_reexports)]
pub use pipeline::*;
#[allow(ambiguous_glob_reexports)]
pub use procedure::*;
