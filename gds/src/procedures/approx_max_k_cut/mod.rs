//! ApproxMaxKCut: approximate maximum k-cut using GRASP
//!
//! Divides nodes into k communities to maximize (or minimize) the weight
//! of edges crossing between communities using a Greedy Randomized Adaptive
//! Search Procedure (GRASP) with local search.

pub mod spec;
pub mod storage;
pub mod computation;

#[cfg(test)]
mod integration_tests;
