//! Types for ApproxMaxKCut.

/// Configuration for approx max k-cut computation.
#[derive(Debug, Clone)]
pub struct ApproxMaxKCutConfig {
	pub k: u8,
	pub iterations: usize,
	pub random_seed: u64,
	pub minimize: bool,
	pub has_relationship_weight_property: bool,
	pub min_community_sizes: Vec<usize>,
}

/// Result for approx max k-cut computation.
#[derive(Debug, Clone)]
pub struct ApproxMaxKCutResult {
	pub communities: Vec<u8>,
	pub cut_cost: f64,
}
