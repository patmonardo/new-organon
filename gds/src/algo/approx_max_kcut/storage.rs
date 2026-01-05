//! ApproxMaxKCut storage adapter.
//!
//! The current Rust facade wires graph access directly and feeds a neighbor
//! closure into `ApproxMaxKCutComputationRuntime`. This file exists to keep the
//! algorithm module aligned with the repo's spec/storage/computation pattern.

#[derive(Debug, Default, Clone)]
pub struct ApproxMaxKCutStorageRuntime;

impl ApproxMaxKCutStorageRuntime {
	pub fn new() -> Self {
		Self
	}
}
