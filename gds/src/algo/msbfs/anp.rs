//! Aggregated Neighbor Processing (ANP) MSBFS
//!
//! Translation target: Neo4j GDS `org.neo4j.gds.msbfs.*` (ANPStrategy + MultiSourceBFSAccessMethods).
//!
//! This is a sequential implementation of ANP over a `get_neighbors(node_id)` callback.
//! It maintains three per-node bitsets (`visit`, `visit_next`, `seen`) packed into `u64`.
//! Each bit represents a source within a batch of up to `OMEGA = 64` sources.
//!
//! Notes:
//! - The implementation supports source batching via `(source_offset, source_len)`.
//! - `allow_start_node_traversal` defaults to `false` for centrality algorithms
//!   (avoids depth=0 callbacks and division-by-zero in harmonic).

pub const OMEGA: usize = 64;

use crate::concurrency::TerminationFlag;

#[derive(Debug, Clone)]
pub struct AggregatedNeighborProcessingMsBfs {
    node_count: usize,
    visit: Vec<u64>,
    visit_next: Vec<u64>,
    seen: Vec<u64>,
}

impl AggregatedNeighborProcessingMsBfs {
    pub fn new(node_count: usize) -> Self {
        Self {
            node_count,
            visit: vec![0u64; node_count],
            visit_next: vec![0u64; node_count],
            seen: vec![0u64; node_count],
        }
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn clear(&mut self) {
        self.visit.fill(0);
        self.visit_next.fill(0);
        self.seen.fill(0);
    }

    pub fn run<G, F>(
        &mut self,
        source_offset: usize,
        source_len: usize,
        allow_start_node_traversal: bool,
        get_neighbors: G,
        per_node_action: F,
    ) where
        G: Fn(usize) -> Vec<usize>,
        F: FnMut(usize, u32, u64),
    {
        self.run_with_termination(
            source_offset,
            source_len,
            allow_start_node_traversal,
            None,
            get_neighbors,
            per_node_action,
        )
    }

    pub fn run_with_termination<G, F>(
        &mut self,
        source_offset: usize,
        source_len: usize,
        allow_start_node_traversal: bool,
        termination: Option<&TerminationFlag>,
        get_neighbors: G,
        mut per_node_action: F,
    ) where
        G: Fn(usize) -> Vec<usize>,
        F: FnMut(usize, u32, u64),
    {
        assert!(source_len <= OMEGA, "source_len must be <= {OMEGA}");
        assert!(source_offset + source_len <= self.node_count);

        self.clear();

        // Initialization: each source starts at its own node.
        for bit_idx in 0..source_len {
            let source_node = source_offset + bit_idx;
            let mask = 1u64 << bit_idx;
            self.visit[source_node] |= mask;
            self.seen[source_node] |= mask;
        }

        let mut depth: u32 = 0;

        loop {
            if let Some(t) = termination {
                if !t.running() {
                    return;
                }
            }

            // Phase 1: traverse nodes currently in visit set.
            for node_id in 0..self.node_count {
                if let Some(t) = termination {
                    if !t.running() {
                        return;
                    }
                }

                let node_visit = self.visit[node_id];
                if node_visit == 0 {
                    continue;
                }

                if allow_start_node_traversal || depth > 0 {
                    per_node_action(node_id, depth, node_visit);
                }

                // prepareNextVisit: OR node_visit bits into neighbors' next set.
                for neighbor in get_neighbors(node_id) {
                    if let Some(t) = termination {
                        if !t.running() {
                            return;
                        }
                    }

                    if neighbor < self.node_count {
                        self.visit_next[neighbor] |= node_visit;
                    }
                }
            }

            depth += 1;

            // Phase 2: compute next frontier and update seen.
            let mut has_next = false;
            for node_id in 0..self.node_count {
                if let Some(t) = termination {
                    if !t.running() {
                        return;
                    }
                }

                let next = self.visit_next[node_id] & !self.seen[node_id];
                if next != 0 {
                    self.seen[node_id] |= next;
                    self.visit[node_id] = next;
                    has_next = true;
                } else {
                    self.visit[node_id] = 0;
                }
                self.visit_next[node_id] = 0;
            }

            if !has_next {
                return;
            }
        }
    }

    pub fn run_all_sources_batched<G, F>(
        &mut self,
        allow_start_node_traversal: bool,
        get_neighbors: G,
        per_node_action: F,
    ) where
        G: Fn(usize) -> Vec<usize> + Copy,
        F: FnMut(usize, u32, u64) + Copy,
    {
        self.run_all_sources_batched_with_termination(
            allow_start_node_traversal,
            None,
            get_neighbors,
            per_node_action,
        )
    }

    pub fn run_all_sources_batched_with_termination<G, F>(
        &mut self,
        allow_start_node_traversal: bool,
        termination: Option<&TerminationFlag>,
        get_neighbors: G,
        mut per_node_action: F,
    ) where
        G: Fn(usize) -> Vec<usize> + Copy,
        F: FnMut(usize, u32, u64) + Copy,
    {
        for source_offset in (0..self.node_count).step_by(OMEGA) {
            if let Some(t) = termination {
                if !t.running() {
                    return;
                }
            }

            let source_len = (source_offset + OMEGA).min(self.node_count) - source_offset;
            self.run_with_termination(
                source_offset,
                source_len,
                allow_start_node_traversal,
                termination,
                get_neighbors,
                &mut per_node_action,
            );
        }
    }

    #[inline]
    pub fn source_count(mask: u64) -> u32 {
        mask.count_ones()
    }

    pub fn iter_sources(mask: u64, source_offset: usize) -> impl Iterator<Item = usize> {
        SourceMaskIter {
            remaining: mask,
            source_offset,
        }
    }
}

struct SourceMaskIter {
    remaining: u64,
    source_offset: usize,
}

impl Iterator for SourceMaskIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let tz = self.remaining.trailing_zeros() as usize;
        self.remaining &= self.remaining - 1;
        Some(self.source_offset + tz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_graph(edges: Vec<(usize, usize)>, node_count: usize) -> HashMap<usize, Vec<usize>> {
        let mut graph = HashMap::new();
        for i in 0..node_count {
            graph.insert(i, Vec::new());
        }
        for (from, to) in edges {
            graph.entry(from).or_insert_with(Vec::new).push(to);
            if from != to {
                graph.entry(to).or_insert_with(Vec::new).push(from);
            }
        }
        for neighbors in graph.values_mut() {
            neighbors.sort_unstable();
            neighbors.dedup();
        }
        graph
    }

    #[test]
    fn skips_depth_zero_when_disallowed() {
        let graph = create_graph(vec![(0, 1)], 2);
        let mut msbfs = AggregatedNeighborProcessingMsBfs::new(2);

        let mut depths = Vec::new();
        msbfs.run(
            0,
            1,
            false,
            |n| graph.get(&n).cloned().unwrap_or_default(),
            |_node_id, depth, _mask| {
                depths.push(depth);
            },
        );

        // Only node 1 at depth 1 should be reported.
        assert_eq!(depths, vec![1]);
    }

    #[test]
    fn iter_sources_decodes_with_offset() {
        let mask = 0b1010u64; // bits 1 and 3
        let sources: Vec<_> = AggregatedNeighborProcessingMsBfs::iter_sources(mask, 10).collect();
        assert_eq!(sources, vec![11, 13]);
    }
}
