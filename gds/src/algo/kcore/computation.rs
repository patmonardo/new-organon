//! K-Core decomposition computation runtime
//!
//! Java parity references:
//! - `org.neo4j.gds.kcore.KCoreDecomposition`
//! - `org.neo4j.gds.kcore.KCoreDecompositionTask`
//! - `org.neo4j.gds.kcore.RebuildTask`
//! - `org.neo4j.gds.kcore.NodeProvider`

use crate::collections::HugeAtomicLongArray;
use crate::concurrency::{install_with_concurrency, Concurrency};
use crate::core::utils::paged::{HugeLongArrayQueue, HugeLongArrayStack};
use crate::core::utils::partition::{Partition, PartitionUtils};
use rayon::prelude::*;
use std::cmp;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::Arc;

pub const CHUNK_SIZE: usize = 64;
pub const UNASSIGNED: i32 = -1;
pub const REBUILD_CONSTANT: f64 = 0.02;
const UNASSIGNED_LONG: i64 = UNASSIGNED as i64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KCoreComputationResult {
    pub core_values: Vec<i32>,
    pub degeneracy: i32,
}

#[derive(Debug, Clone)]
pub struct KCoreComputationRuntime {
    chunk_size: usize,
    concurrency: usize,
    #[cfg(test)]
    rebuild_count: usize,
}

impl Default for KCoreComputationRuntime {
    fn default() -> Self {
        Self {
            chunk_size: CHUNK_SIZE,
            concurrency: 1,
            #[cfg(test)]
            rebuild_count: 0,
        }
    }
}

impl KCoreComputationRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    #[cfg(test)]
    pub fn rebuild_count(&self) -> usize {
        self.rebuild_count
    }

    pub fn compute<F>(&mut self, node_count: usize, neighbors: F) -> KCoreComputationResult
    where
        F: Fn(usize) -> Vec<usize> + Sync,
    {
        #[cfg(test)]
        {
            self.rebuild_count = 0;
        }

        if node_count == 0 {
            return KCoreComputationResult {
                core_values: Vec::new(),
                degeneracy: 0,
            };
        }

        let concurrency = Concurrency::from_usize(self.concurrency);

        let current_degrees = HugeAtomicLongArray::new(node_count);
        let core = HugeAtomicLongArray::new(node_count);

        let degree_zero = AtomicUsize::new(0);

        install_with_concurrency(concurrency, || {
            (0..node_count).into_par_iter().for_each(|v| {
                core.set(v, UNASSIGNED_LONG);
                let degree = neighbors(v).len();
                current_degrees.set(v, degree as i64);

                if degree == 0 {
                    core.set(v, 0);
                    degree_zero.fetch_add(1, Ordering::Relaxed);
                }
            });
        });

        let rebuild_limit = ((REBUILD_CONSTANT * node_count as f64).ceil() as usize).max(1);
        let remaining_nodes =
            AtomicIsize::new((node_count - degree_zero.load(Ordering::Relaxed)) as isize);

        let node_index = Arc::new(AtomicUsize::new(0));
        let remaining_nodes_arc = Arc::new(remaining_nodes);
        let current_degrees = Arc::new(current_degrees);
        let core = Arc::new(core);

        let mut scanning_degree: i32 = 1;
        let mut degeneracy: i32 = 0;

        let mut node_provider = Arc::new(NodeProvider::full(node_count));

        let mut tasks: Vec<KCoreTask> = (0..self.concurrency.max(1))
            .map(|_| {
                KCoreTask::new(
                    Arc::clone(&current_degrees),
                    Arc::clone(&core),
                    Arc::clone(&node_index),
                    Arc::clone(&remaining_nodes_arc),
                    self.chunk_size,
                    Arc::clone(&node_provider),
                )
            })
            .collect();

        let mut has_rebuild = false;

        while remaining_nodes_arc.load(Ordering::Relaxed) > 0 {
            let remaining_now = remaining_nodes_arc.load(Ordering::Relaxed) as usize;
            if !has_rebuild && remaining_now < rebuild_limit {
                node_provider = Arc::new(rebuild(
                    node_count,
                    remaining_now,
                    self.concurrency.max(1),
                    Arc::clone(&core),
                ));
                for t in tasks.iter_mut() {
                    t.update_node_provider(Arc::clone(&node_provider));
                }
                has_rebuild = true;
                #[cfg(test)]
                {
                    self.rebuild_count += 1;
                }
            }

            node_index.store(0, Ordering::Relaxed);

            for t in tasks.iter_mut() {
                t.set_scanning_degree(scanning_degree);
                t.set_phase(KCorePhase::Scan);
            }

            run_tasks(&mut tasks, concurrency, &neighbors);

            let next_scanning_degree = tasks
                .iter()
                .map(|t| t.smallest_active_degree())
                .filter(|v| *v > -1)
                .min()
                .unwrap_or(-1);

            if next_scanning_degree == scanning_degree {
                degeneracy = scanning_degree;

                for t in tasks.iter_mut() {
                    t.set_phase(KCorePhase::Act);
                }

                run_tasks(&mut tasks, concurrency, &neighbors);
                scanning_degree += 1;
            } else if next_scanning_degree > -1 {
                scanning_degree = next_scanning_degree;
            } else {
                // No active nodes found but remaining > 0 would be inconsistent.
                break;
            }
        }

        let mut out = Vec::with_capacity(node_count);
        for i in 0..node_count {
            out.push(core.get(i) as i32);
        }

        KCoreComputationResult {
            core_values: out,
            degeneracy,
        }
    }
}

fn run_tasks<F>(tasks: &mut [KCoreTask], concurrency: Concurrency, neighbors: &F)
where
    F: Fn(usize) -> Vec<usize> + Sync,
{
    install_with_concurrency(concurrency, || {
        tasks.par_iter_mut().for_each(|t| t.run(neighbors));
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KCorePhase {
    Scan,
    Act,
}

#[derive(Clone)]
enum NodeProvider {
    Full {
        size: usize,
    },
    Reduced {
        size: usize,
        node_order: Arc<HugeAtomicLongArray>,
    },
}

impl NodeProvider {
    fn full(size: usize) -> Self {
        Self::Full { size }
    }

    fn reduced(size: usize, node_order: Arc<HugeAtomicLongArray>) -> Self {
        Self::Reduced { size, node_order }
    }

    fn size(&self) -> usize {
        match self {
            NodeProvider::Full { size } => *size,
            NodeProvider::Reduced { size, .. } => *size,
        }
    }

    fn node(&self, index: usize) -> usize {
        match self {
            NodeProvider::Full { .. } => index,
            NodeProvider::Reduced { node_order, .. } => node_order.get(index) as usize,
        }
    }
}

struct KCoreTask {
    current_degrees: Arc<HugeAtomicLongArray>,
    core: Arc<HugeAtomicLongArray>,
    examination_stack: HugeLongArrayStack,
    node_index: Arc<AtomicUsize>,
    remaining_nodes: Arc<AtomicIsize>,
    smallest_active_degree: i32,
    scanning_degree: i32,
    phase: KCorePhase,
    node_provider: Arc<NodeProvider>,
    chunk_size: usize,
}

impl KCoreTask {
    fn new(
        current_degrees: Arc<HugeAtomicLongArray>,
        core: Arc<HugeAtomicLongArray>,
        node_index: Arc<AtomicUsize>,
        remaining_nodes: Arc<AtomicIsize>,
        chunk_size: usize,
        node_provider: Arc<NodeProvider>,
    ) -> Self {
        let capacity = node_provider.size();
        Self {
            current_degrees,
            core,
            examination_stack: HugeLongArrayStack::new(capacity),
            node_index,
            remaining_nodes,
            smallest_active_degree: -1,
            scanning_degree: 1,
            phase: KCorePhase::Scan,
            node_provider,
            chunk_size,
        }
    }

    fn update_node_provider(&mut self, node_provider: Arc<NodeProvider>) {
        self.node_provider = node_provider;
        // keep existing stack allocation
    }

    fn set_scanning_degree(&mut self, scanning_degree: i32) {
        self.scanning_degree = scanning_degree;
    }

    fn set_phase(&mut self, phase: KCorePhase) {
        self.phase = phase;
    }

    fn smallest_active_degree(&self) -> i32 {
        self.smallest_active_degree
    }

    fn run<F>(&mut self, neighbors: &F)
    where
        F: Fn(usize) -> Vec<usize> + Sync,
    {
        match self.phase {
            KCorePhase::Scan => self.scan(),
            KCorePhase::Act => self.act(neighbors),
        }
    }

    fn scan(&mut self) {
        let upper_bound = self.node_provider.size();
        self.smallest_active_degree = -1;
        self.examination_stack.clear();

        loop {
            let offset = self
                .node_index
                .fetch_add(self.chunk_size, Ordering::Relaxed);
            if offset >= upper_bound {
                break;
            }

            let current_chunk = cmp::min(offset + self.chunk_size, upper_bound);
            for index_id in offset..current_chunk {
                let node_id = self.node_provider.node(index_id);
                let node_degree = self.current_degrees.get(node_id) as i32;

                if node_degree >= self.scanning_degree {
                    if node_degree == self.scanning_degree {
                        self.smallest_active_degree = node_degree;
                        self.examination_stack.push(node_id as i64);
                    } else if self.smallest_active_degree == -1
                        || self.smallest_active_degree > node_degree
                    {
                        self.smallest_active_degree = node_degree;
                    }
                }
            }
        }
    }

    fn act<F>(&mut self, neighbors: &F)
    where
        F: Fn(usize) -> Vec<usize> + Sync,
    {
        let mut nodes_examined: isize = 0;

        while !self.examination_stack.is_empty() {
            let node_id = self.examination_stack.pop() as usize;
            self.core.set(node_id, self.scanning_degree as i64);
            nodes_examined += 1;

            self.relax(node_id, neighbors);
        }

        if nodes_examined > 0 {
            self.remaining_nodes
                .fetch_sub(nodes_examined, Ordering::Relaxed);
        }
    }

    fn relax<F>(&mut self, node_id: usize, neighbors: &F)
    where
        F: Fn(usize) -> Vec<usize> + Sync,
    {
        for target in neighbors(node_id) {
            if self.core.get(target) != UNASSIGNED_LONG {
                continue;
            }

            let old = self.current_degrees.get_and_add(target, -1);
            if old == (self.scanning_degree as i64 + 1) {
                self.examination_stack.push(target as i64);
            }
        }
    }
}

fn rebuild(
    node_count: usize,
    number_of_remaining_nodes: usize,
    concurrency: usize,
    core: Arc<HugeAtomicLongArray>,
) -> NodeProvider {
    let node_order = Arc::new(HugeAtomicLongArray::new(number_of_remaining_nodes));
    let atomic_index = Arc::new(AtomicUsize::new(0));

    let mut tasks = PartitionUtils::range_partition(
        concurrency,
        node_count,
        |partition| {
            RebuildTask::new(
                partition,
                Arc::clone(&atomic_index),
                Arc::clone(&core),
                Arc::clone(&node_order),
            )
        },
        None,
    );

    install_with_concurrency(Concurrency::from_usize(concurrency), || {
        tasks.par_iter_mut().for_each(|t| t.run());
    });

    NodeProvider::reduced(number_of_remaining_nodes, node_order)
}

struct RebuildTask {
    partition: Partition,
    atomic_index: Arc<AtomicUsize>,
    core: Arc<HugeAtomicLongArray>,
    node_order: Arc<HugeAtomicLongArray>,
    local_queue: HugeLongArrayQueue,
}

impl RebuildTask {
    fn new(
        partition: Partition,
        atomic_index: Arc<AtomicUsize>,
        core: Arc<HugeAtomicLongArray>,
        node_order: Arc<HugeAtomicLongArray>,
    ) -> Self {
        let capacity = node_order.size();
        Self {
            partition,
            atomic_index,
            core,
            node_order,
            local_queue: HugeLongArrayQueue::new(capacity),
        }
    }

    fn run(&mut self) {
        let start_node = self.partition.start_node();
        let end_node = start_node + self.partition.node_count();

        for node_id in start_node..end_node {
            if self.core.get(node_id) == UNASSIGNED_LONG {
                self.local_queue.add(node_id as i64);
            }
        }

        let local_queue_size = self.local_queue.size();
        let mut current_index = self
            .atomic_index
            .fetch_add(local_queue_size, Ordering::Relaxed);

        while !self.local_queue.is_empty() {
            let node_id = self.local_queue.remove();
            self.node_order.set(current_index, node_id);
            current_index += 1;
        }
    }
}
