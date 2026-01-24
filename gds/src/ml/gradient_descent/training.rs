use super::config::GradientDescentConfig;
use super::objective::Objective;
use super::stopper::factory;
use super::stopper::TrainingStopper;
use crate::ml::core::batch::AnyBatch;
use crate::ml::core::batch::Batch;
use crate::ml::core::batch::BatchQueue;
use crate::ml::core::computation_context::ComputationContext;
use crate::ml::core::optimizer::AdamOptimizer;
use crate::ml::core::optimizer::Updater;
use crate::ml::core::tensor::Tensor;
use parking_lot::RwLock;
use std::sync::Arc;

/// Training implementation for gradient descent optimization
pub struct Training {
    config: GradientDescentConfig,
    train_size: usize,
    termination_flag: Arc<RwLock<bool>>,
}

impl Training {
    pub fn new(
        config: GradientDescentConfig,
        train_size: usize,
        termination_flag: Arc<RwLock<bool>>,
    ) -> Self {
        Self {
            config,
            train_size,
            termination_flag,
        }
    }

    /// Train the objective using gradient descent
    ///
    /// Translated from Java Training.train() method.
    pub fn train<O: Objective>(
        &self,
        objective: &O,
        mut queue_supplier: impl FnMut() -> Box<dyn BatchQueue>,
        concurrency: usize,
    ) {
        if *self.termination_flag.read() {
            log::info!("terminated before training start");
            return;
        }

        // Create updater with weights
        let weights = objective.weights();
        let mut updater = AdamOptimizer::new(weights, self.config.learning_rate());
        let mut stopper = factory::default_stopper(&self.config);

        let mut losses = Vec::new();

        // Initial loss computation (Java: var initialLoss = avgLoss(consumers))
        let consumers = self.execute_batches(concurrency, objective, queue_supplier());
        let mut prev_weight_gradients = Self::avg_weight_gradients(&consumers);
        let initial_loss = Self::avg_loss(&consumers);

        log::info!("Initial loss {}", initial_loss);

        let mut terminated_by_flag = false;

        // Main training loop (Java: while (!stopper.terminated()))
        while !stopper.terminated() {
            if *self.termination_flag.read() {
                terminated_by_flag = true;
                break;
            }

            // Update weights with previous gradients BEFORE computing new ones
            // Java: updater.update(prevWeightGradients);
            updater.update(&prev_weight_gradients);

            // Execute batches for this epoch
            // Java: consumers = executeBatches(concurrency, objective, queueSupplier.get());
            let consumers = self.execute_batches(concurrency, objective, queue_supplier());

            if *self.termination_flag.read() {
                terminated_by_flag = true;
                break;
            }

            prev_weight_gradients = Self::avg_weight_gradients(&consumers);

            // Compute average loss for this epoch
            // Java: double loss = avgLoss(consumers);
            let loss = Self::avg_loss(&consumers);
            stopper.register_loss(loss);
            losses.push(loss);

            log::info!("Epoch {} with loss {}", losses.len(), loss);
        }

        // Final logging (Java: progressTracker.logMessage(...))
        let last_loss = losses.last().copied().unwrap_or(initial_loss);
        let epochs_ran = losses.len();

        log::info!(
            "{} after {} out of {} epochs. Initial loss: {}, Last loss: {}.{}",
            if stopper.converged() {
                "converged"
            } else {
                "terminated"
            },
            epochs_ran,
            self.config.max_epochs(),
            initial_loss,
            last_loss,
            if stopper.converged() {
                ""
            } else if terminated_by_flag {
                " Terminated early"
            } else {
                " Did not converge"
            }
        );
    }

    fn execute_batches<'a, O: Objective>(
        &self,
        concurrency: usize,
        objective: &'a O,
        mut batches: Box<dyn BatchQueue>,
    ) -> Vec<ObjectiveUpdateConsumer<'a, O>> {
        assert!(concurrency > 0, "Concurrency must be at least 1");
        let mut consumers: Vec<ObjectiveUpdateConsumer<O>> = (0..concurrency)
            .map(|_| ObjectiveUpdateConsumer::new(objective, self.train_size))
            .collect();

        // Process batches across consumers
        let mut consumer_idx = 0;
        while let Some(batch) = batches.pop() {
            if *self.termination_flag.read() {
                break;
            }
            let vec_batch = VecBatch::from_any_batch(batch.as_ref());
            consumers[consumer_idx].accept(&vec_batch);
            consumer_idx = (consumer_idx + 1) % consumers.len();
        }

        consumers
    }

    fn avg_weight_gradients(
        consumers: &[ObjectiveUpdateConsumer<impl Objective>],
    ) -> Vec<Box<dyn Tensor>> {
        let local_gradient_sums: Vec<&Vec<Box<dyn Tensor>>> = consumers
            .iter()
            .map(|c| c.summed_weight_gradients())
            .collect();

        let number_of_batches: usize = consumers.iter().map(|c| c.consumed_batches()).sum();

        // Average tensors
        Self::average_tensors(&local_gradient_sums, number_of_batches)
    }

    fn average_tensors(
        tensor_lists: &[&Vec<Box<dyn Tensor>>],
        divisor: usize,
    ) -> Vec<Box<dyn Tensor>> {
        if tensor_lists.is_empty() || divisor == 0 {
            return Vec::new();
        }

        let num_tensors = tensor_lists[0].len();
        let mut result = Vec::with_capacity(num_tensors);

        for i in 0..num_tensors {
            let mut sum = tensor_lists[0][i].clone_box();
            for tensor_list in &tensor_lists[1..] {
                sum.add_inplace(tensor_list[i].as_ref());
            }
            let scaled_sum = sum.scalar_multiply(1.0 / divisor as f64);
            result.push(scaled_sum);
        }

        result
    }

    fn avg_loss(consumers: &[ObjectiveUpdateConsumer<impl Objective>]) -> f64 {
        let total_loss: f64 = consumers.iter().map(|c| c.loss_sum()).sum();
        let total_batches: usize = consumers.iter().map(|c| c.consumed_batches()).sum();

        if total_batches == 0 {
            0.0
        } else {
            total_loss / total_batches as f64
        }
    }
}

/// Consumer that processes batches and accumulates gradients
struct ObjectiveUpdateConsumer<'a, O: Objective> {
    objective: &'a O,
    train_size: usize,
    summed_weight_gradients: Vec<Box<dyn Tensor>>,
    loss_sum: f64,
    consumed_batches: usize,
}

impl<'a, O: Objective> ObjectiveUpdateConsumer<'a, O> {
    fn new(objective: &'a O, train_size: usize) -> Self {
        // Java: this.summedWeightGradients = objective
        //     .weights()
        //     .stream()
        //     .map(weight -> weight.data().createWithSameDimensions())
        //     .collect(Collectors.toList());
        let summed_weight_gradients = objective
            .weights()
            .into_iter()
            .map(|weights| weights.snapshot().create_with_same_dimensions())
            .collect();

        Self {
            objective,
            train_size,
            summed_weight_gradients,
            loss_sum: 0.0,
            consumed_batches: 0,
        }
    }

    fn accept<B: Batch>(&mut self, batch: &B) {
        // Java: Variable<Scalar> loss = objective.loss(batch, trainSize);
        let loss_variable = self.objective.loss(batch, self.train_size);

        // Java: var ctx = new ComputationContext();
        let ctx = ComputationContext::new();

        // Java: lossSum += ctx.forward(loss).value();
        let loss_value = ctx.forward(loss_variable.as_ref());
        self.loss_sum += loss_value.aggregate_sum();

        // Java: ctx.backward(loss);
        ctx.backward(loss_variable.as_ref());

        let local_weight_gradients: Vec<Box<dyn Tensor>> = self
            .objective
            .weights()
            .iter()
            .map(|weights| {
                ctx.gradient(weights.as_ref())
                    .unwrap_or_else(|| weights.snapshot().create_with_same_dimensions())
            })
            .collect();

        // Java: for (int i = 0; i < summedWeightGradients.size(); i++) {
        //     summedWeightGradients.get(i).addInPlace(localWeightGradient.get(i));
        // }
        for (i, gradient) in local_weight_gradients.iter().enumerate() {
            self.summed_weight_gradients[i].add_inplace(gradient.as_ref());
        }

        self.consumed_batches += 1;
    }

    fn summed_weight_gradients(&self) -> &Vec<Box<dyn Tensor>> {
        &self.summed_weight_gradients
    }

    fn consumed_batches(&self) -> usize {
        self.consumed_batches
    }

    fn loss_sum(&self) -> f64 {
        self.loss_sum
    }
}

/// Simple batch backed by an owned Vec<u64> to cooperate with AnyBatch values.
struct VecBatch {
    element_ids: Vec<u64>,
}

impl VecBatch {
    fn from_any_batch(batch: &dyn AnyBatch) -> Self {
        let element_ids = batch.element_ids_boxed().collect();
        Self { element_ids }
    }
}

impl Batch for VecBatch {
    type ElementIdsIter = std::vec::IntoIter<u64>;

    fn element_ids(&self) -> Self::ElementIdsIter {
        self.element_ids.clone().into_iter()
    }

    fn size(&self) -> usize {
        self.element_ids.len()
    }
}
