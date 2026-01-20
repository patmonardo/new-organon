//! Expository example: compare Logistic Regression and MLP training
//!
//! This example builds a small synthetic dataset, constructs trainers for
//! `LogisticRegression` and `MLP`, and prints a blow-by-blow narration of the
//! high-level steps so readers can follow the control flow without diving into
//! the pipeline/eval internals.
#![allow(dead_code)]

use std::sync::Arc;

use parking_lot::RwLock;

use gds::collections::HugeIntArray;
use gds::ml::models::logistic_regression::LogisticRegressionTrainConfig;
use gds::ml::models::logistic_regression::LogisticRegressionTrainer;
use gds::ml::models::mlp::MLPClassifierTrainConfig;
use gds::ml::models::mlp::MLPClassifierTrainer;
use gds::ml::models::{ClassifierTrainer, Features};

// Small, easy-to-follow synthetic dataset builder
fn build_synthetic() -> (Vec<Vec<f64>>, Vec<i32>) {
    // We'll create 12 examples: first 6 belong to class 0, next 6 to class 1.
    // Class 0 points cluster near (0,0), class 1 near (5,5).
    let mut features = Vec::with_capacity(12);
    let mut labels = Vec::with_capacity(12);

    for i in 0..6 {
        features.push(vec![i as f64 * 0.1, i as f64 * 0.2]);
        labels.push(0);
    }
    for i in 0..6 {
        features.push(vec![5.0 + i as f64 * 0.1, 5.0 + i as f64 * 0.1]);
        labels.push(1);
    }

    (features, labels)
}

// Minimal `Features` impl that the trainers expect
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

fn main() {
    println!("=== M L : Compare LogisticRegression vs MLP (Expository) ===");

    // Build data
    println!("Step 1: Building synthetic dataset...");
    let (features_vec, labels_vec) = build_synthetic();
    println!("  - examples: {}", features_vec.len());
    println!("  - feature dim: {}", features_vec[0].len());

    let features = VecFeatures { data: features_vec };

    // Wrap labels into HugeIntArray (crate utility)
    let labels_huge = HugeIntArray::from_vec(labels_vec.clone());
    let train_ids = Arc::new((0u64..(features.size() as u64)).collect::<Vec<u64>>());

    println!("Step 2: Configure Logistic Regression trainer...");
    let log_cfg = LogisticRegressionTrainConfig {
        batch_size: 4,
        max_epochs: 50,
        learning_rate: 0.05,
        ..Default::default()
    };
    println!("  - batch_size: {}", log_cfg.batch_size);
    println!("  - max_epochs: {}", log_cfg.max_epochs);
    println!("  - learning_rate: {}", log_cfg.learning_rate);

    let termination_flag = Arc::new(RwLock::new(false));
    let log_trainer =
        LogisticRegressionTrainer::new(log_cfg, 2, false, Arc::clone(&termination_flag), 1);

    println!("Step 3: Train Logistic Regression (high-level)...");
    println!("  - Creating internal model parameters (weights + bias)");
    let log_classifier = log_trainer.train(&features, &labels_huge, &train_ids);
    println!("  - Training complete. Inspecting classifier...");

    // Quick per-example prediction dump
    println!("  - Predictions (LogisticRegression):");
    for (i, label) in labels_vec.iter().enumerate().take(features.size()) {
        let f = features.get(i).to_vec();
        let probs = log_classifier.predict_probabilities(&f);
        println!("    example {:>2}: true={} probs={:?}", i, label, probs);
    }

    // MLP branch
    println!("\nStep 4: Configure MLP trainer...");
    let mlp_cfg = MLPClassifierTrainConfig::builder()
        .batch_size(4)
        .max_epochs(50)
        .learning_rate(0.05)
        .hidden_layer_sizes(vec![8])
        .build()
        .unwrap();

    println!("  - hidden_layer_sizes: {:?}", mlp_cfg.hidden_layer_sizes());

    println!("Step 5: Train MLP (high-level)...");
    let mlp_trainer = MLPClassifierTrainer::new(2, mlp_cfg, Some(42), 1);
    let mlp_classifier = mlp_trainer.train(&features, &labels_huge, &train_ids);
    println!("  - Training complete. Inspecting classifier...");

    println!("  - Predictions (MLP):");
    for (i, label) in labels_vec.iter().enumerate().take(features.size()) {
        let f = features.get(i).to_vec();
        let probs = mlp_classifier.predict_probabilities(&f);
        println!("    example {:>2}: true={} probs={:?}", i, label, probs);
    }

    println!("\nSummary:");
    println!(" - LogisticRegression: trained with simple linear decision boundary.");
    println!(" - MLP: introduces hidden layer(s) so it can learn non-linear features.");
    println!(" - Both trainers use the common gradient-descent infrastructure under the hood.");
    println!(" - This example intentionally prints high-level steps rather than internal per-batch math.");
}
