//! Form Evaluator (Form ISA)
//!
//! The Form ISA is the third evaluator under `projection/eval/`.
//!
//! It consumes a base graph plus artifacts produced by earlier stages and returns
//! a derived graph (a ResultStore) by running a chain of `FormOperator`s.
//!
//! ```text
//! eval/procedure  ← algorithms
//! eval/ml         ← optional pipeline
//! eval/form       ← FormProcessor + FormOperator
//! ```

pub mod executor;
pub mod form_spec;
pub mod pure_executor;

pub use executor::*;
pub use form_spec::*;
pub use pure_executor::*;
