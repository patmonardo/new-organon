//! Expository example: gradient descent without pipeline APIs.
//!
//! This example trains a linear regression model by directly wiring:
//! - `GradientDescentConfig`
//! - `Training`
//! - `LinearRegressionObjective`
//!
//! Run:
//!   cargo run -p gds --example ml_gradient_descent_pure_api --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\n\
Run: cargo run -p gds --example ml_gradient_descent_pure_api --features ml"
    );
}

#[cfg(feature = "ml")]
mod enabled {
    use std::sync::Arc;

    use parking_lot::RwLock;

    use gds::collections::HugeDoubleArray;
    use gds::ml::core::batch::from_array;
    use gds::ml::gradient_descent::{GradientDescentConfig, Objective, Training};
    use gds::ml::models::linear_regression::{LinearRegressionObjective, LinearRegressor};
    use gds::ml::models::{Features, Regressor};

    struct VecFeatures {
        data: Vec<Vec<f64>>,
    }

    impl Features for VecFeatures {
        fn get(&self, node_id: usize) -> &[f64] {
            &self.data[node_id]
        }

        fn feature_dimension(&self) -> usize {
            if self.data.is_empty() {
                0
            } else {
                self.data[0].len()
            }
        }

        fn size(&self) -> usize {
            self.data.len()
        }
    }

    fn build_linear_data() -> (VecFeatures, HugeDoubleArray) {
        // y = 2x + 1 (noisy) for x in 0..8
        let mut features = Vec::new();
        let mut targets = Vec::new();

        for x in 0..8 {
            let xf = x as f64;
            features.push(vec![xf]);
            targets.push(2.0 * xf + 1.0 + (xf * 0.01));
        }

        (
            VecFeatures { data: features },
            HugeDoubleArray::from_vec(targets),
        )
    }

    pub fn main() {
        println!("=== ML: Gradient Descent (Pure API) ===");

        let (features, targets) = build_linear_data();
        let train_ids = Arc::new((0u64..(features.size() as u64)).collect::<Vec<u64>>());

        let objective = LinearRegressionObjective::new(&features, &targets, 0.0);

        let config = GradientDescentConfig::builder()
            .batch_size(4)
            .min_epochs(5)
            .max_epochs(50)
            .patience(3)
            .learning_rate(0.05)
            .tolerance(1e-6)
            .build()
            .expect("valid gradient descent config");

        let termination_flag = Arc::new(RwLock::new(false));
        let training = Training::new(
            config.clone(),
            train_ids.len(),
            Arc::clone(&termination_flag),
        );

        let batch_size = config.batch_size();
        let queue_supplier = move || from_array(Arc::clone(&train_ids), batch_size);

        training.train(&objective, queue_supplier, 1);

        let regressor = LinearRegressor::new(objective.model_data().clone());

        println!("Predictions after training:");
        for x in 0..5 {
            let xf = x as f64;
            let y_hat = regressor.predict(&[xf]);
            println!("  x={:<2} -> ŷ={:.4}", x, y_hat);
        }

        println!("Summary:");
        println!(" - Directly used `Objective` + `Training` (no pipeline executor).");
        println!(" - Handy for isolated debugging of gradient descent behavior.");
    }
}

#[cfg(feature = "ml")]
fn main() {
    enabled::main();
}
