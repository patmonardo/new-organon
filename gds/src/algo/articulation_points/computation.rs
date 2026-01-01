//! Articulation Points Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.articulationpoints.ArticulationPoints`
//!
//! This module implements Tarjan's algorithm for finding articulation points using iterative DFS.

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

/// Articulation Points computation result
#[derive(Clone)]
pub struct ArticulationPointsComputationResult {
    pub articulation_points: BitSet,
}

/// Articulation Points computation runtime
pub struct ArticulationPointsComputationRuntime {
    visited: BitSet,
    tin: HugeLongArray,
    low: HugeLongArray,
    children: HugeLongArray,
    timer: i64,
    articulation_points: BitSet,
}

impl ArticulationPointsComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            visited: BitSet::new(node_count),
            tin: HugeLongArray::new(node_count),
            low: HugeLongArray::new(node_count),
            children: HugeLongArray::new(node_count),
            timer: 0,
            articulation_points: BitSet::new(node_count),
        }
    }

    /// Compute articulation points for a graph
    /// get_neighbors returns the neighbor nodes for a given node
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> ArticulationPointsComputationResult {
        self.timer = 0;
        self.visited.clear_all();
        self.articulation_points.clear_all();

        // Initialize tin/low to -1 and children to 0.
        for i in 0..node_count {
            self.tin.set(i, -1);
            self.low.set(i, -1);
            self.children.set(i, 0);
        }

        // Process each unvisited node
        for i in 0..node_count {
            if !self.visited.get(i) {
                self.dfs(i, &get_neighbors);
            }
        }

        ArticulationPointsComputationResult {
            articulation_points: self.articulation_points.clone(),
        }
    }

    fn dfs(&mut self, start_node: usize, get_neighbors: &impl Fn(usize) -> Vec<usize>) {
        // Use a concrete stack of events. This avoids lossy bit-packing and works
        // for graphs well beyond 65k nodes.
        let mut stack: Vec<StackEvent> = Vec::new();

        // Push initial event.
        stack.push(StackEvent::upcoming_visit(start_node, None));

        while let Some(event) = stack.pop() {
            if event.last_visit {
                // Last visit - process backtracking (Java: lastVisit())
                let v = match event.trigger_node {
                    Some(v) => v,
                    None => continue,
                };
                let to = event.event_node;

                let low_v = self.low.get(v);
                let low_to = self.low.get(to);
                self.low.set(v, std::cmp::min(low_v, low_to));

                let tin_v = self.tin.get(v);
                if low_to >= tin_v {
                    self.articulation_points.set(v);
                }

                self.children.add_to(v, 1);
                continue;
            }

            // First visit - process node (Java: upcomingVisit())
            let node = event.event_node;
            let trigger = event.trigger_node;

            if !self.visited.get(node) {
                self.visited.set(node);
                self.children.set(node, 0);
                self.tin.set(node, self.timer);
                self.low.set(node, self.timer);
                self.timer += 1;

                // Add post event (should be before exploring neighbors).
                if let Some(p) = trigger {
                    stack.push(StackEvent::last_visit(node, p));
                }

                let neighbors = get_neighbors(node);
                for to in neighbors {
                    if Some(to) == trigger {
                        continue;
                    }
                    stack.push(StackEvent::upcoming_visit(to, Some(node)));
                }
            } else {
                // Back edge: update low(trigger) with tin(to)
                // Java:
                //   long v = event.triggerNode();
                //   long to = event.eventNode();
                //   low[v] = min(low[v], tin[to]);
                if let Some(v) = trigger {
                    let low_v = self.low.get(v);
                    let tin_to = self.tin.get(node);
                    self.low.set(v, std::cmp::min(low_v, tin_to));
                }
            }
        }

        // Mark root as articulation point if it has more than 1 child
        if self.children.get(start_node) > 1 {
            self.articulation_points.set(start_node);
        } else {
            self.articulation_points.clear(start_node);
        }
    }
}
