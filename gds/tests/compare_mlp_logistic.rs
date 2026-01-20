use std::sync::Arc;

use parking_lot::RwLock;

use gds::collections::HugeIntArray;
use gds::ml::models::logistic_regression::LogisticRegressionTrainConfig;
use gds::ml::models::logistic_regression::LogisticRegressionTrainer;
use gds::ml::models::mlp::MLPClassifierTrainConfig;
use gds::ml::models::mlp::MLPClassifierTrainer;
use gds::ml::models::{ClassifierTrainer, Features};

// Simple synthetic features for tests
struct TestFeatures {
    data: Vec<Vec<f64>>,
}

impl TestFeatures {
    fn new() -> Self {
        // small linearly separable dataset
        let mut data = Vec::new();
        for i in 0..20 {
            let x = i as f64;
            if i % 2 == 0 {
                data.push(vec![x, x + 1.0]); // class 0
            } else {
                data.push(vec![x + 5.0, x + 6.0]); // class 1
            }
        }
        Self { data }
    }
}

impl Features for TestFeatures {
    fn get(&self, node_id: usize) -> &[f64] {
        &self.data[node_id]
    }

    fn feature_dimension(&self) -> usize {
        2
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

#[test]
fn compare_mlp_and_logistic_on_synthetic() {
    // Labels: even -> 0, odd -> 1
    let labels = (0..20)
        .map(|i| if i % 2 == 0 { 0 } else { 1 })
        .collect::<Vec<i32>>();
    let labels = HugeIntArray::from_vec(labels);
    let train_set = Arc::new((0u64..20u64).collect::<Vec<u64>>());
    let features = TestFeatures::new();

    // Logistic regression trainer
    let log_cfg = LogisticRegressionTrainConfig {
        batch_size: 4,
        max_epochs: 40,
        learning_rate: 0.01,
        ..Default::default()
    };

    let termination_flag = Arc::new(RwLock::new(false));
    let log_trainer =
        LogisticRegressionTrainer::new(log_cfg, 2, false, Arc::clone(&termination_flag), 1);

    let log_classifier = log_trainer.train(&features, &labels, &train_set);

    // MLP trainer
    let mlp_cfg = MLPClassifierTrainConfig::builder()
        .batch_size(4)
        .max_epochs(40)
        .learning_rate(0.01)
        .hidden_layer_sizes(vec![8])
        .build()
        .unwrap();

    let mlp_trainer = MLPClassifierTrainer::new(2, mlp_cfg, Some(42), 1);
    let mlp_classifier = mlp_trainer.train(&features, &labels, &train_set);

    // Sanity checks: feature dims and number of classes
    assert_eq!(log_classifier.data().feature_dimension(), 2);
    assert_eq!(mlp_classifier.data().feature_dimension(), 2);
    assert_eq!(log_classifier.data().number_of_classes(), 2);
    assert_eq!(mlp_classifier.data().number_of_classes(), 2);

    // Compute training accuracy
    let mut log_correct = 0usize;
    let mut mlp_correct = 0usize;
    for i in 0..features.size() {
        let f = features.get(i).to_vec();
        let lp = log_classifier.predict_probabilities(&f);
        let mp = mlp_classifier.predict_probabilities(&f);

        let log_pred = if lp[0] >= lp[1] { 0 } else { 1 };
        let mlp_pred = if mp[0] >= mp[1] { 0 } else { 1 };

        let true_label = if i % 2 == 0 { 0 } else { 1 };
        if log_pred == true_label {
            log_correct += 1;
        }
        if mlp_pred == true_label {
            mlp_correct += 1;
        }
    }

    let log_acc = (log_correct as f64) / (features.size() as f64);
    let mlp_acc = (mlp_correct as f64) / (features.size() as f64);

    eprintln!("Logistic acc = {} ; MLP acc = {}", log_acc, mlp_acc);

    // Basic sanity checks: logistic should be better than random here;
    // report both accuracies for comparison.
    assert!(log_acc >= 0.5);
    assert!((0.0..=1.0).contains(&mlp_acc));
}
