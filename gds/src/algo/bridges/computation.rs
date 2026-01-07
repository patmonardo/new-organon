//! Bridges Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.bridges.Bridges`
//!
//! Finds all bridge edges in an undirected graph using Tarjan-style DFS.
//! Uses an explicit stack of events to avoid recursion.

use crate::collections::{BitSet, HugeLongArray};

/// A bridge edge as `(min(from,to), max(from,to))`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Bridge {
    pub from: u64,
    pub to: u64,
}

impl Bridge {
    pub fn create(from: usize, to: usize) -> Self {
        let a = from.min(to) as u64;
        let b = from.max(to) as u64;
        Self { from: a, to: b }
    }
}

/// Stack event for iterative DFS.
#[derive(Debug, Clone, Copy)]
pub struct StackEvent {
    pub event_node: usize,
    pub trigger_node: Option<usize>,
    pub last_visit: bool,
}

impl StackEvent {
    pub fn upcoming_visit(node: usize, trigger_node: Option<usize>) -> Self {
        Self {
            event_node: node,
            trigger_node,
            last_visit: false,
        }
    }

    pub fn last_visit(node: usize, trigger_node: usize) -> Self {
        Self {
            event_node: node,
            trigger_node: Some(trigger_node),
            last_visit: true,
        }
    }
}

#[derive(Clone)]
pub struct BridgesComputationResult {
    pub bridges: Vec<Bridge>,
}

pub struct BridgesComputationRuntime {
    visited: BitSet,
    tin: HugeLongArray,
    low: HugeLongArray,
    timer: i64,
    stack: Vec<StackEvent>,
}

impl BridgesComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self::new_with_stack_capacity(node_count, 0)
    }

    /// Create a runtime with a reusable DFS stack.
    ///
    /// For Java parity, pass `relationship_count` as `stack_capacity`.
    pub fn new_with_stack_capacity(node_count: usize, stack_capacity: usize) -> Self {
        Self {
            visited: BitSet::new(node_count),
            tin: HugeLongArray::new(node_count),
            low: HugeLongArray::new(node_count),
            timer: 0,
            stack: Vec::with_capacity(stack_capacity),
        }
    }

    /// Compute all bridge edges.
    ///
    /// This is a convenience wrapper; in the preferred architecture, the storage runtime
    /// owns the outer loop and calls `dfs_component()`.
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> BridgesComputationResult {
        self.reset(node_count);

        let mut bridges: Vec<Bridge> = Vec::new();

        for i in 0..node_count {
            if !self.is_visited(i) {
                self.dfs_component(i, &get_neighbors, &mut bridges);
            }
        }

        BridgesComputationResult { bridges }
    }

    /// Reset the computation state for a new run.
    pub fn reset(&mut self, node_count: usize) {
        self.timer = 0;
        self.visited.clear_all();

        for i in 0..node_count {
            self.tin.set(i, -1);
            self.low.set(i, -1);
        }
        self.stack.clear();
    }

    /// Check if a node has been visited.
    pub fn is_visited(&self, node: usize) -> bool {
        self.visited.get(node)
    }

    /// Explore a connected component starting at `start_node`, adding any bridges found.
    ///
    /// `get_neighbors(node)` must provide an undirected neighbor list.
    pub fn dfs_component(
        &mut self,
        start_node: usize,
        get_neighbors: &impl Fn(usize) -> Vec<usize>,
        bridges: &mut Vec<Bridge>,
    ) {
        self.stack.clear();
        self.stack
            .push(StackEvent::upcoming_visit(start_node, None));

        while let Some(event) = self.stack.pop() {
            if event.last_visit {
                // Backtracking phase (Java: lastVisit())
                let v = match event.trigger_node {
                    Some(v) => v,
                    None => continue,
                };
                let to = event.event_node;

                let low_v = self.low.get(v);
                let low_to = self.low.get(to);
                self.low.set(v, std::cmp::min(low_v, low_to));

                let tin_v = self.tin.get(v);
                if low_to > tin_v {
                    bridges.push(Bridge::create(v, to));
                }

                continue;
            }

            // Forward phase (Java: upcomingVisit())
            let node = event.event_node;
            let trigger = event.trigger_node;

            if !self.visited.get(node) {
                self.visited.set(node);
                self.tin.set(node, self.timer);
                self.low.set(node, self.timer);
                self.timer += 1;

                if let Some(p) = trigger {
                    // Post event must be before exploring neighbors
                    self.stack.push(StackEvent::last_visit(node, p));
                }

                let mut parent_skipped = false;
                for to in get_neighbors(node) {
                    if Some(to) == trigger && !parent_skipped {
                        // Skip exactly one parent edge (handles multi-edges)
                        parent_skipped = true;
                        continue;
                    }
                    self.stack.push(StackEvent::upcoming_visit(to, Some(node)));
                }
            } else if let Some(v) = trigger {
                // Back edge: update low(trigger) with tin(to)
                let low_v = self.low.get(v);
                let tin_to = self.tin.get(node);
                self.low.set(v, std::cmp::min(low_v, tin_to));
            }
        }
    }
}

impl Default for BridgesComputationRuntime {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn undirected_adj(edges: &[(usize, usize)], node_count: usize) -> Vec<Vec<usize>> {
        let mut adj = vec![Vec::<usize>::new(); node_count];
        for &(a, b) in edges {
            adj[a].push(b);
            if a != b {
                adj[b].push(a);
            }
        }
        adj
    }

    #[test]
    fn finds_bridges_in_line() {
        // 0-1-2
        let node_count = 3;
        let adj = undirected_adj(&[(0, 1), (1, 2)], node_count);
        let neighbors = |n: usize| adj[n].clone();

        let mut rt = BridgesComputationRuntime::new(node_count);
        let result = rt.compute(node_count, neighbors);

        assert_eq!(result.bridges.len(), 2);
        assert!(result.bridges.contains(&Bridge::create(0, 1)));
        assert!(result.bridges.contains(&Bridge::create(1, 2)));
    }

    #[test]
    fn no_bridges_in_triangle() {
        // 0-1-2-0
        let node_count = 3;
        let adj = undirected_adj(&[(0, 1), (1, 2), (2, 0)], node_count);
        let neighbors = |n: usize| adj[n].clone();

        let mut rt = BridgesComputationRuntime::new(node_count);
        let result = rt.compute(node_count, neighbors);

        assert!(result.bridges.is_empty());
    }
}
