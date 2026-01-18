use super::Updater;
use crate::mem::Estimate;
use crate::ml::core::functions::Weights;
use crate::ml::core::Tensor;
use std::sync::Arc;

const CLIP_MAX: f64 = 5.0;
const CLIP_MIN: f64 = -5.0;

/// Adam optimizer implementation based on https://arxiv.org/pdf/1412.6980.pdf
///
/// Division, squaring and square-rooting is done element-wise.
pub struct AdamOptimizer {
    learning_rate: f64,
    beta_1: f64,
    beta_2: f64,
    epsilon: f64,
    weights: Vec<Arc<Weights>>,
    momentum_terms: Vec<Box<dyn Tensor>>,
    velocity_terms: Vec<Box<dyn Tensor>>,
    iteration: usize,
}

impl AdamOptimizer {
    /// Calculate memory size in bytes for given dimensions
    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        let term_size = Weights::size_in_bytes(rows, cols);
        Estimate::size_of_instance("AdamOptimizer")
            + 2 * term_size // fields
            + 2 * term_size // working memory: mCap, vCap
    }

    /// Create a new Adam optimizer
    pub fn new(weights: Vec<Arc<Weights>>, learning_rate: f64) -> Self {
        let momentum_terms: Vec<Box<dyn Tensor>> = weights
            .iter()
            .map(|w| w.borrow().create_with_same_dimensions())
            .collect();

        let velocity_terms: Vec<Box<dyn Tensor>> = weights
            .iter()
            .map(|w| w.borrow().create_with_same_dimensions())
            .collect();

        Self {
            learning_rate,
            beta_1: 0.9,
            beta_2: 0.999,
            epsilon: 1e-8,
            weights,
            momentum_terms,
            velocity_terms,
            iteration: 0,
        }
    }

    /// Clip gradient values to avoid exploding gradients
    fn clip(value: f64) -> f64 {
        if value > CLIP_MAX {
            CLIP_MAX
        } else {
            value.max(CLIP_MIN)
        }
    }
}

impl Updater for AdamOptimizer {
    fn update(&mut self, context_local_weight_gradients: &[Box<dyn Tensor>]) {
        self.iteration += 1;

        for i in 0..self.weights.len() {
            let gradient = context_local_weight_gradients[i].as_ref();

            // Clip gradient to avoid exploding gradients
            let clipped_gradient = gradient.map(Self::clip);

            // m_t = beta_1 * m_t + (1 - beta_1) * g_t
            let mut new_momentum = self.momentum_terms[i].scalar_multiply(self.beta_1);
            let scaled_gradient = clipped_gradient.scalar_multiply(1.0 - self.beta_1);
            new_momentum.add_inplace(scaled_gradient.as_ref());
            self.momentum_terms[i] = new_momentum;

            // v_t = beta_2 * v_t + (1 - beta_2) * (g_t^2)
            let mut new_velocity = self.velocity_terms[i].scalar_multiply(self.beta_2);
            let squared_gradient = clipped_gradient.map(|v| v * v);
            let scaled_squared = squared_gradient.scalar_multiply(1.0 - self.beta_2);
            new_velocity.add_inplace(scaled_squared.as_ref());
            self.velocity_terms[i] = new_velocity;

            // m_cap = m_t / (1 - beta_1^t)
            let m_cap = self.momentum_terms[i]
                .scalar_multiply(1.0 / (1.0 - self.beta_1.powi(self.iteration as i32)));

            // v_cap = v_t / (1 - beta_2^t)
            let v_cap = self.velocity_terms[i]
                .scalar_multiply(1.0 / (1.0 - self.beta_2.powi(self.iteration as i32)));

            // theta_0 = theta_0 - (alpha * m_cap) / (sqrt(v_cap) + epsilon)
            let update = m_cap.scalar_multiply(-self.learning_rate);
            let v_cap_sqrt = v_cap.map(f64::sqrt);
            let epsilon_tensor = v_cap_sqrt.ones_like().scalar_multiply(self.epsilon);
            let v_cap_sqrt_with_epsilon = v_cap_sqrt.add(epsilon_tensor.as_ref());
            let v_cap_inv = v_cap_sqrt_with_epsilon.map(|v| 1.0 / v);
            let final_update = update.elementwise_product(v_cap_inv.as_ref());

            let mut weight = self.weights[i].borrow_mut();
            weight.add_inplace(final_update.as_ref());
        }
    }
}
