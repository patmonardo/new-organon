//! Triangle-related utilities.

pub mod intersect;
pub mod count;

pub use count::{
	TriangleCountAlgorithmSpec, TriangleCountComputationRuntime, TriangleCountConfig,
	TriangleCountResult, TriangleCountStorageRuntime,
};
