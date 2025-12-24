//! Form program types.
//!
//! This module intentionally stays small: it exists to carry a structured program
//! into the Form evaluator (`projection/eval/form`).

pub mod program;

pub use program::*;

// Inference APIs live under form now
pub mod inference;
pub use inference::*;
