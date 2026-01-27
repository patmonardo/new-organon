//! Expository example: Logistic Regression training
//!
//! This example builds a tiny synthetic dataset, trains a Logistic Regression
//! classifier, and prints a clear step-by-step narration.
//!
//! Run:
//!   cargo run -p gds --example ml_logistic_regression_expository --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\n\
Run: cargo run -p gds --example ml_logistic_regression_expository --features ml"
    );
}

#[cfg(feature = "ml")]
mod enabled {
    use std::sync::Arc;

    use parking_lot::RwLock;

    use gds::collections::HugeIntArray;
    use gds::ml::models::logistic_regression::LogisticRegressionTrainConfig;
    use gds::ml::models::logistic_regression::LogisticRegressionTrainer;
    use gds::ml::models::{ClassifierTrainer, Features};

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

    fn build_synthetic_binary() -> (VecFeatures, Vec<i32>) {
        // Two clusters: class 0 near (0,0), class 1 near (5,5)
        let mut features = Vec::new();
        let mut labels = Vec::new();

        for i in 0..6 {
            features.push(vec![i as f64 * 0.1, i as f64 * 0.2]);
            labels.push(0);
        }
        for i in 0..6 {
            features.push(vec![5.0 + i as f64 * 0.1, 5.0 + i as f64 * 0.1]);
            labels.push(1);
        }

        (VecFeatures { data: features }, labels)
    }

    pub fn main() {
        println!("=== ML: Logistic Regression (Expository) ===");

        println!("Step 1: Build synthetic dataset...");
        let (features, labels_vec) = build_synthetic_binary();
        println!("  - examples: {}", labels_vec.len());
        println!("  - feature dim: {}", features.feature_dimension());

        let labels = HugeIntArray::from_vec(labels_vec.clone());
        let train_ids = Arc::new((0u64..(features.size() as u64)).collect::<Vec<u64>>());

        println!("Step 2: Configure Logistic Regression trainer...");
        let config = LogisticRegressionTrainConfig {
            batch_size: 4,
            max_epochs: 50,
            learning_rate: 0.05,
            penalty: 0.0,
            tolerance: 1e-4,
            focus_weight: 0.0,
            class_weights: None,
        };
        println!("  - batch_size: {}", config.batch_size);
        println!("  - max_epochs: {}", config.max_epochs);
        println!("  - learning_rate: {}", config.learning_rate);
        println!("  - loss: reduced cross-entropy (focus_weight=0.0)");

        let termination_flag = Arc::new(RwLock::new(false));
        let trainer =
            LogisticRegressionTrainer::new(config, 2, false, Arc::clone(&termination_flag), 1);

        println!("Step 3: Train classifier...");
        let classifier = trainer.train(&features, &labels, &train_ids);
        println!("  - Training complete. Inspecting predictions...");
        println!("  - probs[0]=P(class=0), probs[1]=P(class=1) for binary case");

        for (i, label) in labels_vec.iter().enumerate() {
            let f = features.get(i).to_vec();
            let probs = classifier.predict_probabilities(&f);
            println!("    example {:>2}: true={} probs={:?}", i, label, probs);
        }

        println!("Summary:");
        println!(" - Uses gradient descent under the hood.");
        println!(" - loss is cross-entropy unless focus_weight > 0.0 (focal loss).");
        println!(" - No pipeline APIs involved; this is the pure model path.");
    }
}

#[cfg(feature = "ml")]
fn main() {
    enabled::main();
}
