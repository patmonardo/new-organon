//! Articulation Points Computation Runtime
//!
//! Finds articulation points (cut vertices) in an undirected graph.
//!
//! Implementation notes:
//! - Uses an explicit stack of DFS events to avoid recursion.
//! - Mirrors the Java GDS structure (upcomingVisit / lastVisit events).

use crate::collections::{BitSet, HugeLongArray};

pub const STACK_EVENT_SIZE_BYTES: usize = std::mem::size_of::<StackEvent>();

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

    pub fn is_last_visit(&self) -> bool {
        self.last_visit
    }
}

/// Articulation Points computation result.
#[derive(Clone)]
pub struct ArticulationPointsComputationResult {
    pub articulation_points: BitSet,
}

/// Articulation Points computation runtime.
///
/// This struct owns reusable per-run buffers.
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

    pub fn initialize(&mut self, node_count: usize) {
        self.timer = 0;
        self.visited.clear_all();
        self.articulation_points.clear_all();

        for i in 0..node_count {
            self.tin.set(i, -1);
            self.low.set(i, -1);
            self.children.set(i, 0);
        }
    }

    pub fn is_visited(&self, node: usize) -> bool {
        self.visited.get(node)
    }

    pub fn set_visited(&mut self, node: usize) {
        self.visited.set(node);
    }

    pub fn get_tin(&self, node: usize) -> i64 {
        self.tin.get(node)
    }

    pub fn set_tin(&mut self, node: usize, value: i64) {
        self.tin.set(node, value);
    }

    pub fn get_low(&self, node: usize) -> i64 {
        self.low.get(node)
    }

    pub fn set_low(&mut self, node: usize, value: i64) {
        self.low.set(node, value);
    }

    pub fn get_children(&self, node: usize) -> i64 {
        self.children.get(node)
    }

    pub fn set_children(&mut self, node: usize, value: i64) {
        self.children.set(node, value);
    }

    pub fn add_to_children(&mut self, node: usize, delta: i64) {
        self.children.add_to(node, delta);
    }

    pub fn increment_timer(&mut self) -> i64 {
        let current = self.timer;
        self.timer += 1;
        current
    }

    pub fn set_articulation_point(&mut self, node: usize) {
        self.articulation_points.set(node);
    }

    pub fn clear_articulation_point(&mut self, node: usize) {
        self.articulation_points.clear(node);
    }

    pub fn finalize_result(&self) -> ArticulationPointsComputationResult {
        ArticulationPointsComputationResult {
            articulation_points: self.articulation_points.clone(),
        }
    }

    /// Compute articulation points.
    ///
    /// `get_neighbors(node)` must return an undirected neighbor list.
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> ArticulationPointsComputationResult {
        self.compute_with_relationship_count(node_count, 0, get_neighbors)
    }

    /// Java parity: size the internal DFS stack based on `relationship_count`.
    pub fn compute_with_relationship_count(
        &mut self,
        node_count: usize,
        relationship_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> ArticulationPointsComputationResult {
        self.timer = 0;
        self.visited.clear_all();
        self.articulation_points.clear_all();

        for i in 0..node_count {
            self.tin.set(i, -1);
            self.low.set(i, -1);
            self.children.set(i, 0);
        }

        for node in 0..node_count {
            if !self.visited.get(node) {
                self.dfs_component_root(node, relationship_count, &get_neighbors);
            }
        }

        ArticulationPointsComputationResult {
            articulation_points: self.articulation_points.clone(),
        }
    }

    fn dfs_component_root(
        &mut self,
        root: usize,
        relationship_count: usize,
        get_neighbors: &impl Fn(usize) -> Vec<usize>,
    ) {
        let mut stack: Vec<StackEvent> = Vec::new();
        if relationship_count > 0 {
            // Java parity: stack sized by relationship count.
            let _ = stack.try_reserve(relationship_count);
        }
        stack.push(StackEvent::upcoming_visit(root, None));

        while let Some(event) = stack.pop() {
            self.visit_event(event, get_neighbors, &mut stack);
        }

        // Root rule: root is articulation point iff it has > 1 DFS-tree children.
        if self.children.get(root) > 1 {
            self.articulation_points.set(root);
        } else {
            self.articulation_points.clear(root);
        }
    }

    fn visit_event(
        &mut self,
        event: StackEvent,
        get_neighbors: &impl Fn(usize) -> Vec<usize>,
        stack: &mut Vec<StackEvent>,
    ) {
        if event.is_last_visit() {
            let to = event.event_node;
            let v = match event.trigger_node {
                Some(v) => v,
                None => return,
            };

            let low_v = self.low.get(v);
            let low_to = self.low.get(to);
            self.low.set(v, std::cmp::min(low_v, low_to));

            let tin_v = self.tin.get(v);
            if low_to >= tin_v {
                self.articulation_points.set(v);
            }

            self.children.add_to(v, 1);
            return;
        }

        let v = event.event_node;
        let parent = event.trigger_node;

        if !self.visited.get(v) {
            self.visited.set(v);
            self.children.set(v, 0);

            self.tin.set(v, self.timer);
            self.low.set(v, self.timer);
            self.timer += 1;

            // Post event must be pushed before exploring neighbors.
            if let Some(p) = parent {
                stack.push(StackEvent::last_visit(v, p));
            }

            for to in get_neighbors(v) {
                if Some(to) == parent {
                    continue;
                }
                stack.push(StackEvent::upcoming_visit(to, Some(v)));
            }
        } else if let Some(p) = parent {
            // Back edge: update low(parent) with tin(v)
            let low_p = self.low.get(p);
            let tin_v = self.tin.get(v);
            self.low.set(p, std::cmp::min(low_p, tin_v));
        }
    }
}

impl Default for ArticulationPointsComputationRuntime {
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

    fn articulation_points(
        rt: &mut ArticulationPointsComputationRuntime,
        adj: &[Vec<usize>],
    ) -> Vec<usize> {
        let neighbors = |n: usize| adj[n].clone();
        let result = rt.compute(adj.len(), neighbors);

        let mut out = Vec::new();
        let mut idx = result.articulation_points.next_set_bit(0);
        while let Some(i) = idx {
            out.push(i);
            idx = result.articulation_points.next_set_bit(i + 1);
        }
        out
    }

    #[test]
    fn line_graph_has_internal_articulation_points() {
        // 0-1-2-3-4 => 1,2,3 are articulation points.
        let node_count = 5;
        let adj = undirected_adj(&[(0, 1), (1, 2), (2, 3), (3, 4)], node_count);
        let mut rt = ArticulationPointsComputationRuntime::new(node_count);
        let points = articulation_points(&mut rt, &adj);
        assert!(points.contains(&1));
        assert!(points.contains(&2));
        assert!(points.contains(&3));
    }

    #[test]
    fn cycle_has_no_articulation_points() {
        // 0-1-2-3-0
        let node_count = 4;
        let adj = undirected_adj(&[(0, 1), (1, 2), (2, 3), (3, 0)], node_count);
        let mut rt = ArticulationPointsComputationRuntime::new(node_count);
        let points = articulation_points(&mut rt, &adj);
        assert!(points.is_empty());
    }

    #[test]
    fn star_center_is_articulation_point() {
        // 0 connected to 1..4
        let node_count = 5;
        let adj = undirected_adj(&[(0, 1), (0, 2), (0, 3), (0, 4)], node_count);
        let mut rt = ArticulationPointsComputationRuntime::new(node_count);
        let points = articulation_points(&mut rt, &adj);
        assert!(points.contains(&0));
        assert!(!points.contains(&1));
        assert!(!points.contains(&2));
    }
}
