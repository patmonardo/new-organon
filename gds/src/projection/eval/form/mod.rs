//! Form Evaluator (Form ISA)
//!
//! The Form ISA is the third evaluator under `projection/eval/`.
//!
//! In the repo’s two-stage meaning of “Projection”, Eval is the **Revealing power**:
//! it takes a **base** `GraphStore` (the Image laid down by Factory) and produces a
//! **derived** `GraphStore` (the ResultStore).
//!
//! The Form ISA is where the system starts to encode the “origin of certainty”:
//! Procedure and ML can emit artifacts (assertions / problematics), but **Form projects
//! the returned graph** (apodictic/singular) by selecting a `FormOperator` from a `FormShape`
//! program and executing it against a base graph.
//!
//! ## The Three ISA
//!
//! ```
//! eval/procedure (Computation ISA)  ← AlgorithmSpec implementations
//! eval/ml (Pipeline ISA)           ← Pipeline implementations
//! eval/form (Form ISA)             ← FormProcessor + FormOperator
//! ```

pub mod form_spec;
pub mod executor;
pub mod triadic_cycle;

pub use executor::*;
pub use form_spec::*;
pub use triadic_cycle::*;
