//! Expository example: ml::metrics
//!
//! This walkthrough shows classification + regression metrics and how
//! aggregate stats are built across splits.

use gds::collections::{HugeDoubleArray, HugeLongArray};
use gds::ml::metrics::{
    Accuracy, F1Score, GlobalAccuracy, Metric, MetricComparator, ModelStatsBuilder,
    RegressionMetric,
};

fn print_section(title: &str) {
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
}

fn format_score(name: &str, value: f64) -> String {
    format!("{name:<28} {value:>10.6}")
}

fn main() {
    println!("=== ML: Metrics Walkthrough ===");

    print_section("Classification metrics");
    let targets = HugeLongArray::from_vec(vec![0, 1, 1, 0, 1, 0]);
    let predictions = HugeLongArray::from_vec(vec![0, 1, 0, 0, 1, 1]);

    let global_acc = GlobalAccuracy::new();
    let acc_pos = Accuracy::new(1, 1);
    let f1_pos = F1Score::new(1, 1);

    let global_value = global_acc
        .as_classification_metric()
        .unwrap()
        .compute(&targets, &predictions);
    let acc_value = acc_pos
        .as_classification_metric()
        .unwrap()
        .compute(&targets, &predictions);
    let f1_value = f1_pos
        .as_classification_metric()
        .unwrap()
        .compute(&targets, &predictions);

    println!("{}", format_score(global_acc.name(), global_value));
    println!("{}", format_score(acc_pos.name(), acc_value));
    println!("{}", format_score(f1_pos.name(), f1_value));

    print_section("Regression metrics");
    let reg_targets = HugeDoubleArray::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    let reg_predictions = HugeDoubleArray::from_vec(vec![0.5, 2.5, 2.0, 5.0]);

    let mse = RegressionMetric::MeanSquaredError;
    let rmse = RegressionMetric::RootMeanSquaredError;
    let mae = RegressionMetric::MeanAbsoluteError;

    println!(
        "{}",
        format_score(mse.name(), mse.compute(&reg_targets, &reg_predictions))
    );
    println!(
        "{}",
        format_score(rmse.name(), rmse.compute(&reg_targets, &reg_predictions))
    );
    println!(
        "{}",
        format_score(mae.name(), mae.compute(&reg_targets, &reg_predictions))
    );

    print_section("ModelStatsBuilder (2 splits)");
    let mut stats = ModelStatsBuilder::new(2);

    // Split 1
    stats.update(&global_acc, global_value);
    stats.update(&mse, mse.compute(&reg_targets, &reg_predictions));

    // Split 2 (pretend we measured a different fold)
    stats.update(&global_acc, 0.90);
    stats.update(&mse, 0.25);

    let summary = stats.build();
    for (name, scores) in summary.iter() {
        println!(
            "{name:<28} avg={:>7.4} min={:>7.4} max={:>7.4}",
            scores.avg, scores.min, scores.max
        );
    }

    print_section("Comparator intuition");
    let comparator = MetricComparator::Natural;
    println!("{:?}", comparator.compare(0.9, 0.8));
    let comparator = MetricComparator::Inverse;
    println!("{:?}", comparator.compare(0.9, 0.8));

    print_section("Summary");
    println!(" - Classification metrics operate on HugeLongArray targets/predictions.");
    println!(" - Regression metrics operate on HugeDoubleArray targets/predictions.");
    println!(" - ModelStatsBuilder aggregates metrics across splits.");
}
