//! Bridges Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.bridges.Bridges`
//!
//! This module implements Tarjan's algorithm for finding bridges using iterative DFS.

use crate::collections::{BitSet, HugeLongArray};

/// Stack event for iterative DFS
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

/// Bridge edge
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bridge {
    pub from: u64,
    pub to: u64,
}

/// Bridges computation result
#[derive(Clone)]
pub struct BridgesComputationResult {
    pub bridges: Vec<Bridge>,
}

/// Bridges computation runtime
pub struct BridgesComputationRuntime {
    visited: BitSet,
    tin: HugeLongArray,
    low: HugeLongArray,
    timer: i64,
    bridges: Vec<Bridge>,
}

impl BridgesComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            visited: BitSet::new(node_count),
            tin: HugeLongArray::new(node_count),
            low: HugeLongArray::new(node_count),
            timer: 0,
            bridges: Vec::new(),
        }
    }

    /// Compute bridges for a graph
    pub fn compute<F>(&mut self, node_count: usize, get_neighbors: F) -> BridgesComputationResult
    where
        F: Fn(usize) -> Vec<usize>,
    {
        // Clear state
        self.timer = 0;
        self.bridges.clear();
        self.visited.clear_all();

        // Initialize tin and low to -1
        for i in 0..node_count {
            self.tin.set(i, -1);
            self.low.set(i, -1);
        }

        // Process each unvisited node
        for i in 0..node_count {
            if !self.visited.get(i) {
                self.dfs(i, &get_neighbors);
            }
        }

        BridgesComputationResult {
            bridges: self.bridges.clone(),
        }
    }

    fn dfs<F>(&mut self, start_node: usize, get_neighbors: &F)
    where
        F: Fn(usize) -> Vec<usize>,
    {
        let mut stack: Vec<StackEvent> = Vec::new();

        // Push initial event
        stack.push(StackEvent::upcoming_visit(start_node, None));

        while let Some(event) = stack.pop() {
            if event.last_visit {
                // Last visit - process backtracking
                let v = event.trigger_node.unwrap();
                let to = event.event_node;

                let low_v = self.low.get(v);
                let low_to = self.low.get(to);
                self.low.set(v, std::cmp::min(low_v, low_to));

                let tin_v = self.tin.get(v);
                if low_to > tin_v {
                    // This is a bridge
                    self.bridges.push(Bridge {
                        from: std::cmp::min(v as u64, to as u64),
                        to: std::cmp::max(v as u64, to as u64),
                    });
                }
            } else {
                // First visit - process node
                let v = event.event_node;
                let p = event.trigger_node;

                if !self.visited.get(v) {
                    self.visited.set(v);
                    self.tin.set(v, self.timer);
                    self.low.set(v, self.timer);
                    self.timer += 1;

                    // Push post-visit event if not root
                    if let Some(parent) = p {
                        stack.push(StackEvent::last_visit(v, parent));
                    }

                    // Process neighbors (skip parent edge once)
                    let neighbors = get_neighbors(v);
                    let mut parent_skipped = false;

                    for to in neighbors {
                        if Some(to) == p && !parent_skipped {
                            parent_skipped = true;
                            continue;
                        }
                        stack.push(StackEvent::upcoming_visit(to, Some(v)));
                    }
                } else if let Some(parent) = p {
                    // Back edge - update low value
                    let low_p = self.low.get(parent);
                    let tin_v = self.tin.get(v);
                    self.low.set(parent, std::cmp::min(low_p, tin_v));
                }
            }
        }
    }
}
