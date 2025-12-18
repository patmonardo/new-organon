//! Traversal infrastructure (module wiring only).
//!
//! Per your preference: keep `mod.rs` free of implementation code.

pub mod aggregator;
pub mod exit_predicate;

pub use aggregator::{Aggregator, NoAggregator, OneHopAggregator, WeightAggregator};
pub use exit_predicate::{
    ExitPredicate, ExitPredicateResult, FollowExitPredicate, TargetExitPredicate,
};
