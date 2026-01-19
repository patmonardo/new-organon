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
//! eval/form (Form ISA)             ← FormSpec implementations (currently inactive)
//! ```

// Procedure - Raising src/procedure infrastructure into consciousness
pub mod pipeline;
pub mod procedure;
