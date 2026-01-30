//! Dataset-level expression facade.
//!
//! This module re-exports the DataFrame expression helpers to keep a
//! Python-like "dataset -> expr" workflow scoped under `datasets`.

pub use crate::collections::dataframe::expr::*;
