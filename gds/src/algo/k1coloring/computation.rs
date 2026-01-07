//! K1-Coloring computation runtime
//!
//! Java parity references:
//! - `org.neo4j.gds.k1coloring.K1Coloring`
//! - `org.neo4j.gds.k1coloring.ColoringStep`
//! - `org.neo4j.gds.k1coloring.ValidationStep`
//!
//! The algorithm alternates between:
//! 1) greedy coloring of a working set of nodes
//! 2) validation which schedules conflicting nodes for recoloring

use crate::collections::{BitSet, HugeAtomicLongArray};
use crate::concurrency::{install_with_concurrency, Concurrency, TerminationFlag};
use crate::core::graph_dimensions::GraphDimensions;
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::mem::{Estimate, MemoryEstimation, MemoryRange, MemoryTree};
use rayon::prelude::*;

pub const INITIAL_FORBIDDEN_COLORS: usize = 1000;

/// Per-iteration progress surface for controller-driven tracking.
#[derive(Debug, Clone, Copy)]
pub struct K1IterationProgress {
    pub iteration: u64,
    pub colored: usize,
    pub validated: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct K1ColoringRunResult {
    pub colors: Vec<u64>,
    pub ran_iterations: u64,
    pub did_converge: bool,
}

#[derive(Debug, Clone)]
pub struct K1ColoringComputationRuntime {
    _node_count: usize,
    max_iterations: u64,
    concurrency: usize,
}

impl K1ColoringComputationRuntime {
    pub fn new(node_count: usize, max_iterations: u64) -> Self {
        Self {
            _node_count: node_count,
            max_iterations,
            concurrency: 1,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    pub fn compute<F, P>(
        &mut self,
        node_count: usize,
        neighbors: F,
        termination_flag: &TerminationFlag,
        mut progress: P,
    ) -> K1ColoringRunResult
    where
        F: Fn(usize) -> Vec<usize> + Sync,
        P: FnMut(K1IterationProgress),
    {
        if node_count == 0 {
            return K1ColoringRunResult {
                colors: Vec::new(),
                ran_iterations: 0,
                did_converge: true,
            };
        }

        if self.max_iterations == 0 {
            return K1ColoringRunResult {
                colors: vec![0; node_count],
                ran_iterations: 0,
                did_converge: false,
            };
        }

        let colors = HugeAtomicLongArray::new(node_count);
        for i in 0..node_count {
            colors.set(i, INITIAL_FORBIDDEN_COLORS as i64);
        }

        let mut current_nodes = HugeAtomicBitSet::new(node_count);
        let mut next_nodes = HugeAtomicBitSet::new(node_count);
        for i in 0..node_count {
            current_nodes.set(i);
        }

        let mut ran_iterations = 0u64;

        while ran_iterations < self.max_iterations && !current_nodes.is_empty() {
            if !termination_flag.running() {
                break;
            }
            // Materialize the current working set for stable parallel iteration.
            let mut nodes_to_color = Vec::with_capacity(current_nodes.cardinality());
            current_nodes.for_each_set_bit(|n| {
                if n < node_count {
                    nodes_to_color.push(n);
                }
            });

            self.run_coloring(&colors, &nodes_to_color, &neighbors, termination_flag);

            progress(K1IterationProgress {
                iteration: ran_iterations,
                colored: nodes_to_color.len(),
                validated: 0,
            });

            next_nodes.clear();
            self.run_validation(
                &colors,
                &nodes_to_color,
                &neighbors,
                &next_nodes,
                termination_flag,
            );

            progress(K1IterationProgress {
                iteration: ran_iterations,
                colored: 0,
                validated: nodes_to_color.len(),
            });

            ran_iterations += 1;

            // Swap working sets.
            std::mem::swap(&mut current_nodes, &mut next_nodes);
        }

        let did_converge = termination_flag.running() && current_nodes.is_empty();

        let mut out = vec![0u64; node_count];
        for i in 0..node_count {
            out[i] = colors.get(i) as u64;
        }

        K1ColoringRunResult {
            colors: out,
            ran_iterations,
            did_converge,
        }
    }

    fn run_coloring<F>(
        &self,
        colors: &HugeAtomicLongArray,
        nodes: &[usize],
        neighbors: &F,
        termination_flag: &TerminationFlag,
    ) where
        F: Fn(usize) -> Vec<usize> + Sync,
    {
        let concurrency = Concurrency::from_usize(self.concurrency);
        install_with_concurrency(concurrency, || {
            nodes.par_iter().for_each_init(
                || BitSet::new(INITIAL_FORBIDDEN_COLORS),
                |forbidden, &node_id| {
                    if !termination_flag.running() {
                        return;
                    }

                    forbidden.clear_all();

                    for target in neighbors(node_id) {
                        if target == node_id {
                            continue;
                        }
                        let c = colors.get(target);
                        if c >= 0 {
                            forbidden.set(c as usize);
                        }
                    }

                    let mut next_color = 0usize;
                    while forbidden.get(next_color) {
                        next_color += 1;
                    }
                    colors.set(node_id, next_color as i64);
                },
            );
        });
    }

    fn run_validation<F>(
        &self,
        colors: &HugeAtomicLongArray,
        nodes: &[usize],
        neighbors: &F,
        next_nodes: &HugeAtomicBitSet,
        termination_flag: &TerminationFlag,
    ) where
        F: Fn(usize) -> Vec<usize> + Sync,
    {
        let concurrency = Concurrency::from_usize(self.concurrency);
        install_with_concurrency(concurrency, || {
            nodes.par_iter().for_each(|&source| {
                if !termination_flag.running() {
                    return;
                }

                let source_color = colors.get(source);
                for target in neighbors(source) {
                    if target == source {
                        continue;
                    }

                    if source_color == colors.get(target) && !next_nodes.get(target) {
                        next_nodes.set(source);
                        break;
                    }
                }
            });
        });
    }
}

// -------------------------------------------------------------------------------------------------
// Memory estimation (Java parity-ish)
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct K1ColoringMemoryEstimation;

impl MemoryEstimation for K1ColoringMemoryEstimation {
    fn description(&self) -> String {
        "K1Coloring".to_string()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let node_count = dimensions.node_count();

        let colors = Estimate::size_of_long_array(node_count);
        // Two bitsets (current + next) over nodes.
        let nodes_to_color = 2 * Estimate::size_of_bitset(node_count);
        // Per-thread forbidden colors (starts at 1000 bits; may grow at runtime).
        let forbidden_colors = Estimate::size_of_bitset(INITIAL_FORBIDDEN_COLORS);

        let resident = vec![
            MemoryTree::leaf(
                "this.instance".into(),
                MemoryRange::of(Estimate::BYTES_OBJECT_HEADER),
            ),
            MemoryTree::leaf("colors".into(), MemoryRange::of(colors)),
            MemoryTree::leaf("nodesToColor".into(), MemoryRange::of(nodes_to_color)),
        ];

        let temporary = vec![MemoryTree::leaf(
            "forbiddenColors(perThread)".into(),
            MemoryRange::of(forbidden_colors).times(concurrency.max(1)),
        )];

        let resident_tree =
            MemoryTree::new("residentMemory".into(), sum_ranges(&resident), resident);
        let temporary_tree =
            MemoryTree::new("temporaryMemory".into(), sum_ranges(&temporary), temporary);

        MemoryTree::new(
            "K1Coloring".into(),
            sum_ranges(&[resident_tree.clone(), temporary_tree.clone()]),
            vec![resident_tree, temporary_tree],
        )
    }
}

fn sum_ranges(children: &[MemoryTree]) -> MemoryRange {
    children
        .iter()
        .fold(MemoryRange::empty(), |acc, t| acc.add(t.memory_usage()))
}
