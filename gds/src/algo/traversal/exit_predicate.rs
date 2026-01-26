use serde::{Deserialize, Serialize};

use crate::types::graph::NodeId;

/// Exit predicate result for traversal control
///
/// Translation of: `ExitPredicate.Result` (lines 26-40)
/// Controls the behavior of graph traversal algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitPredicateResult {
    /// Add current node to result set and visit all neighbors
    Follow,
    /// Add current node to result set and terminate traversal
    Break,
    /// Don't add node to result set, don't follow neighbors, continue with next element
    Continue,
}

/// Exit predicate for controlling traversal behavior
///
/// Translation of: `ExitPredicate.java` (lines 22-52)
/// Called once for each accepted node during traversal
pub trait ExitPredicate {
    /// Test whether to continue traversal from current node
    ///
    /// # Arguments
    /// * `source_node` - The source node
    /// * `current_node` - The current node being processed
    /// * `weight_at_source` - Total weight collected by Aggregator during traversal
    fn test(
        &self,
        source_node: NodeId,
        current_node: NodeId,
        weight_at_source: f64,
    ) -> ExitPredicateResult;
}

/// Default exit predicate that follows all nodes
///
/// Translation of: `ExitPredicate.FOLLOW` (line 24)
pub struct FollowExitPredicate;

impl ExitPredicate for FollowExitPredicate {
    fn test(
        &self,
        _source_node: NodeId,
        _current_node: NodeId,
        _weight_at_source: f64,
    ) -> ExitPredicateResult {
        ExitPredicateResult::Follow
    }
}

/// Target-based exit predicate
///
/// Translation of: `TargetExitPredicate.java` (lines 24-33)
/// Terminates traversal when target nodes are reached
pub struct TargetExitPredicate {
    targets: Vec<NodeId>,
}

impl TargetExitPredicate {
    /// Create new target exit predicate
    pub fn new(targets: Vec<NodeId>) -> Self {
        Self { targets }
    }
}

impl ExitPredicate for TargetExitPredicate {
    fn test(
        &self,
        _source_node: NodeId,
        current_node: NodeId,
        _weight_at_source: f64,
    ) -> ExitPredicateResult {
        if self.targets.contains(&current_node) {
            ExitPredicateResult::Break
        } else {
            ExitPredicateResult::Follow
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_exit_predicate() {
        let predicate = FollowExitPredicate;
        assert_eq!(predicate.test(0, 1, 1.0), ExitPredicateResult::Follow);
        assert_eq!(predicate.test(1, 2, 2.0), ExitPredicateResult::Follow);
    }

    #[test]
    fn test_target_exit_predicate() {
        let predicate = TargetExitPredicate::new(vec![3, 5]);

        assert_eq!(predicate.test(0, 1, 1.0), ExitPredicateResult::Follow);
        assert_eq!(predicate.test(0, 3, 2.0), ExitPredicateResult::Break);
        assert_eq!(predicate.test(0, 5, 3.0), ExitPredicateResult::Break);
    }
}
