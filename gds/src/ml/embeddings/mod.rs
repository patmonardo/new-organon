//! Embeddings for ML.
//!
//! This module provides small, dependency-free embedding utilities.
//! The intent is to offer a minimal “embed → similarity” substrate that higher
//! levels (pipeline steps, form evaluation, etc.) can build upon.

pub mod hashing;

pub use hashing::*;



