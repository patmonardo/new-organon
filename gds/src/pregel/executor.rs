//! Pregel - Main executor for BSP graph computation
//!
//! The Pregel struct ties together all components and runs the
//! Bulk Synchronous Parallel (BSP) loop.

use crate::collections::HugeAtomicBitSet;
use crate::concurrency::install_with_concurrency;
use crate::concurrency::Concurrency;
use crate::core::utils::progress::tasks::LeafTask;
use crate::pregel::{
    projection::PropertyProjection, ComputeFn, DefaultValue, ForkJoinComputer, InitFn,
    MasterComputeContext, MessageIterator, Messenger, NodeValue, PregelComputer, PregelResult,
    PregelRuntimeConfig, PregelSchema,
};
use crate::types::graph::Graph;
use std::sync::Arc;

type MasterComputeFn<C> = Arc<dyn Fn(&mut MasterComputeContext<C>) -> bool + Send + Sync>;

/// Main executor for Pregel computations.
///
/// Coordinates the Bulk Synchronous Parallel (BSP) loop:
/// 1. Initialize computation
/// 2. For each iteration:
///    - Initialize iteration
///    - Run compute step (parallel)
///    - Run master compute (convergence check)
///    - Check if converged
/// 3. Return results
///
/// # Example
///
/// ```ignore
/// use gds::pregel::Pregel;
///
/// let pregel = Pregel::new(graph, config, schema, init_fn, compute_fn, messenger, None);
/// let result = pregel.run();
///
/// if result.did_converge {
///     println!("Converged after {} iterations", result.ran_iterations);
/// }
/// ```
pub struct Pregel<C: PregelRuntimeConfig + Clone, I: MessageIterator> {
    /// Configuration for this computation
    config: C,

    /// The graph to compute on
    graph: Arc<dyn Graph>,

    /// Node property values (results)
    node_values: Arc<parking_lot::RwLock<NodeValue>>,

    /// Message passing system (held for lifecycle management)
    messenger: Arc<dyn Messenger<I>>,

    /// The computer that executes iterations
    computer: ForkJoinComputer<C, I>,

    /// Optional master-compute hook executed once after each superstep.
    ///
    /// This runs on a single thread and may request early termination.
    master_compute_fn: Option<MasterComputeFn<C>>,

    /// Progress tracking task (optional)
    progress_task: Option<Arc<LeafTask>>,
}

impl<C: PregelRuntimeConfig + Clone, I: MessageIterator> Pregel<C, I> {
    /// Create a new Pregel executor.
    ///
    /// This is the main entry point for running Pregel computations.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to compute on
    /// * `config` - Algorithm configuration
    /// * `schema` - Node property schema
    /// * `init_fn` - Initialization function (called once per node)
    /// * `compute_fn` - Compute function (called each iteration)
    /// * `messenger` - Message passing system
    /// * `progress_tracker` - Progress tracking
    ///
    /// # Returns
    ///
    /// A configured Pregel executor ready to run.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: Arc<dyn Graph>,
        config: C,
        schema: PregelSchema,
        init_fn: InitFn<C>,
        compute_fn: ComputeFn<C, I>,
        messenger: Arc<dyn Messenger<I>>,
        progress_task: Option<Arc<LeafTask>>,
    ) -> Self {
        // Create node value storage based on schema
        let node_values = Arc::new(parking_lot::RwLock::new(NodeValue::of(
            &schema,
            graph.node_count() as u64,
            Concurrency::from_usize(config.concurrency()),
        )));

        // Initialize node values from PropertyStore (if property_source is set)
        Self::initialize_from_property_store(&graph, &schema, &node_values);

        // Create vote bits for convergence tracking
        let vote_bits = Arc::new(HugeAtomicBitSet::new(graph.node_count()));

        // Create the computer
        let computer = ForkJoinComputer::new(
            Arc::clone(&graph),
            init_fn,
            compute_fn,
            config.clone(),
            Arc::clone(&node_values),
            Arc::clone(&messenger),
            Arc::clone(&vote_bits),
            progress_task.clone(),
        );

        Self {
            config,
            graph,
            node_values,
            messenger,
            computer,
            master_compute_fn: None,
            progress_task,
        }
    }

    /// Provide a master-compute hook executed once after each superstep.
    pub fn with_master_compute_fn(
        mut self,
        master_compute_fn: impl Fn(&mut MasterComputeContext<C>) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.master_compute_fn = Some(Arc::new(master_compute_fn));
        self
    }

    /// Initialize node values from PropertyStore based on schema mappings.
    ///
    /// For each property in the schema that has a `property_source` set, this method
    /// attempts to load values from the PropertyStore and convert them to Pregel's
    /// DefaultValue format using the PropertyProjection trait.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph (must implement NodePropertyContainer)
    /// * `schema` - The Pregel schema with property_source mappings
    /// * `node_values` - The NodeValue storage to populate
    fn initialize_from_property_store(
        graph: &Arc<dyn Graph>,
        schema: &PregelSchema,
        node_values: &Arc<parking_lot::RwLock<NodeValue>>,
    ) {
        for element in schema.elements() {
            // Skip if no property_source is set
            let Some(source_key) = &element.property_source else {
                continue;
            };

            // Try to get property values from PropertyStore
            let Some(property_values) = graph.node_properties(source_key) else {
                // Property not found in store - silently continue (will use defaults)
                continue;
            };

            // Convert and populate values for all nodes
            let mut node_values_guard = node_values.write();
            for node_id in 0..graph.node_count() {
                // Use PropertyProjection to convert PropertyStore → Pregel DefaultValue
                if let Some(value) =
                    DefaultValue::from_property(property_values.as_ref(), node_id as u64)
                {
                    // Store the value in NodeValue based on type
                    match value {
                        DefaultValue::Long(v) => {
                            node_values_guard.set_long(&element.property_key, node_id, v);
                        }
                        DefaultValue::Double(v) => {
                            node_values_guard.set(&element.property_key, node_id, v);
                        }
                        DefaultValue::LongArray(v) => {
                            node_values_guard.set_long_array(&element.property_key, node_id, v);
                        }
                        DefaultValue::DoubleArray(v) => {
                            node_values_guard.set_double_array(&element.property_key, node_id, v);
                        }
                    }
                }
            }
        }
    }

    /// Run the Pregel computation.
    ///
    /// Executes the BSP loop until convergence or max iterations reached.
    ///
    /// # Returns
    ///
    /// `PregelResult` containing computed node values and execution metadata.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = pregel.run();
    /// println!("Ran {} iterations", result.ran_iterations);
    /// println!("Converged: {}", result.did_converge);
    /// ```
    pub fn run(mut self) -> PregelResult {
        let mut did_converge = false;

        // Initialize computation
        self.computer.init_computation();

        // Track progress - start task if present
        if let Some(task) = &self.progress_task {
            task.base().start();
        }

        let mut ran_iterations = 0;
        for iter in 0..self.config.max_iterations() {
            ran_iterations = iter + 1;

            // Run this superstep inside the configured Rayon pool.
            //
            // Pregel is compute-parallel and barriered: each superstep is a synchronization
            // boundary, and we want all Rayon work inside it to respect the configured
            // concurrency (and pooled thread creation).
            let concurrency = Concurrency::from_usize(self.config.concurrency());
            install_with_concurrency(concurrency, || {
                // Advance/swap message queues for this superstep.
                self.messenger.init_iteration(iter);

                // Initialize iteration in computer
                self.computer.init_iteration(iter);

                // Run the compute step (parallel execution)
                self.computer.run_iteration();
            });

            // Run master compute step (convergence check)
            let master_converged = self.run_master_compute(iter);

            // Check convergence
            did_converge = master_converged || self.computer.has_converged();

            if did_converge {
                break;
            }
        }

        // Finish task if present
        if let Some(task) = &self.progress_task {
            task.finish();
        }

        // Release resources
        self.computer.release();

        // Give the messenger a chance to free internal buffers.
        self.messenger.release();

        // Return results - unwrap Arc<RwLock<NodeValue>> to get NodeValue
        let node_values = Arc::try_unwrap(self.node_values)
            .map(|lock| lock.into_inner())
            .unwrap_or_else(|_arc| NodeValue::stub()); // Fallback if still shared

        PregelResult::new(node_values, ran_iterations, did_converge)
    }

    /// Run the master compute step for convergence checking.
    ///
    /// The master compute runs in a single thread after each superstep
    /// and can signal early termination.
    fn run_master_compute(&self, iteration: usize) -> bool {
        // Create master compute context
        let mut context = MasterComputeContext::new(
            self.config.clone(),
            Arc::clone(&self.graph),
            iteration,
            Arc::clone(&self.node_values),
            self.progress_task.clone(),
        );

        match &self.master_compute_fn {
            Some(f) => f(&mut context),
            None => false,
        }
    }
}

/// Builder for creating Pregel instances with a fluent API.
pub struct PregelBuilder<C: PregelRuntimeConfig, I: MessageIterator> {
    graph: Option<Arc<dyn Graph>>,
    config: Option<C>,
    schema: Option<PregelSchema>,
    init_fn: Option<InitFn<C>>,
    compute_fn: Option<ComputeFn<C, I>>,
    messenger: Option<Arc<dyn Messenger<I>>>,
    master_compute_fn: Option<MasterComputeFn<C>>,
    progress_task: Option<Arc<LeafTask>>,
}

impl<C: PregelRuntimeConfig + Clone, I: MessageIterator> PregelBuilder<C, I> {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            graph: None,
            config: None,
            schema: None,
            init_fn: None,
            compute_fn: None,
            messenger: None,
            master_compute_fn: None,
            progress_task: None,
        }
    }

    /// Set the graph.
    pub fn graph(mut self, graph: Arc<dyn Graph>) -> Self {
        self.graph = Some(graph);
        self
    }

    /// Set the configuration.
    pub fn config(mut self, config: C) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the schema.
    pub fn schema(mut self, schema: PregelSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Set the init function.
    pub fn init_fn(mut self, init_fn: InitFn<C>) -> Self {
        self.init_fn = Some(init_fn);
        self
    }

    /// Set the compute function.
    pub fn compute_fn(mut self, compute_fn: ComputeFn<C, I>) -> Self {
        self.compute_fn = Some(compute_fn);
        self
    }

    /// Set the messenger.
    pub fn messenger(mut self, messenger: Arc<dyn Messenger<I>>) -> Self {
        self.messenger = Some(messenger);
        self
    }

    /// Set the master-compute hook.
    pub fn master_compute_fn(
        mut self,
        master_compute_fn: impl Fn(&mut MasterComputeContext<C>) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.master_compute_fn = Some(Arc::new(master_compute_fn));
        self
    }

    /// Set the progress task (optional).
    pub fn progress_task(mut self, progress_task: Arc<LeafTask>) -> Self {
        self.progress_task = Some(progress_task);
        self
    }

    /// Build the Pregel executor.
    ///
    /// # Panics
    ///
    /// Panics if any required field is missing.
    pub fn build(self) -> Pregel<C, I> {
        let mut pregel = Pregel::new(
            self.graph.expect("graph is required"),
            self.config.expect("config is required"),
            self.schema.expect("schema is required"),
            self.init_fn.expect("init_fn is required"),
            self.compute_fn.expect("compute_fn is required"),
            self.messenger.expect("messenger is required"),
            self.progress_task, // Optional
        );

        if let Some(master_compute_fn) = self.master_compute_fn {
            pregel.master_compute_fn = Some(master_compute_fn);
        }

        pregel
    }
}

impl<C: PregelRuntimeConfig + Clone, I: MessageIterator> Default for PregelBuilder<C, I> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // Note: Add integration tests once we have:
    // - Mock graph implementation
    // - Mock messenger implementation
    // - Simple test algorithm (e.g., node count)
}
