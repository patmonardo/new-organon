//! Expository example: ml::core functions + computation context
//!
//! This example exercises a tiny computation graph without the pipeline engine.
//! We build a graph of Variables directly and run forward/backward passes.

use std::sync::Arc;

use gds::ml::core::{
    ComputationContext, Constant, MeanSquareError, Relu, Scalar, Variable, VariableRef, Vector,
    Weights,
};

fn print_section(title: &str) {
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
}

fn format_vector(v: &Vector) -> String {
    let items: Vec<String> = v.as_slice().iter().map(|x| format!("{x:>7.3}")).collect();
    format!("[{}]", items.join(", "))
}

fn main() {
    println!("=== ML Core: Functions Walkthrough ===");

    // Step 1: Create trainable weights (requires gradients).
    print_section("Step 1: Build the graph");
    let weights = Arc::new(Weights::of_vector(vec![-1.0, 0.5, 2.0]));
    let weights_ref: VariableRef = weights.clone();

    // Step 2: Wrap weights with a function (ReLU).
    let relu = Arc::new(Relu::with_default_alpha_ref(weights_ref));

    // Step 3: Define constant targets (no gradients required).
    let targets = Arc::new(Constant::vector(vec![0.0, 1.0, 1.0]));

    // Step 4: Define loss function (MSE) over predictions vs targets.
    let mse = MeanSquareError::new_ref(relu.clone(), targets.clone());

    println!("Graph: ");
    println!("  Weights (trainable)");
    println!("        │");
    println!("       Relu");
    println!("        │");
    println!("   Predictions");
    println!("        │\\");
    println!("        │ \\");
    println!("        │  \\");
    println!("        │   Targets (constant)");
    println!("        │     /");
    println!("        │    /");
    println!("        │   /");
    println!("        MSE (loss)");
    println!("\nParents:");
    println!(" - Relu.parent = Weights");
    println!(" - MSE.parents = [Relu, Targets]");

    let ctx = ComputationContext::new();

    print_section("Step 2: Forward pass");
    let loss = ctx.forward(&mse);
    let loss_value = loss
        .as_any()
        .downcast_ref::<Scalar>()
        .expect("Loss should be Scalar")
        .value();
    println!("loss         : {loss_value:.6}");

    let weights_snapshot_data = weights.snapshot();
    let weights_snapshot = weights_snapshot_data
        .as_any()
        .downcast_ref::<Vector>()
        .expect("Weights should be Vector");
    println!("weights      : {}", format_vector(weights_snapshot));

    let targets_data_box = targets.apply(&ctx);
    let targets_data = targets_data_box
        .as_any()
        .downcast_ref::<Vector>()
        .expect("Targets should be Vector");
    println!("targets      : {}", format_vector(targets_data));

    let predictions_data = ctx.data(relu.as_ref()).expect("Predictions not computed");
    let predictions = predictions_data
        .as_any()
        .downcast_ref::<Vector>()
        .expect("Predictions should be Vector");
    println!("predictions  : {}", format_vector(predictions));

    print_section("Step 3: Backward pass");
    ctx.backward(&mse);

    let weight_grad_data = ctx
        .gradient(weights.as_ref())
        .expect("Weights gradient not computed");
    let weight_grad = weight_grad_data
        .as_any()
        .downcast_ref::<Vector>()
        .expect("Weights gradient should be Vector");
    println!("dLoss/dWeights: {}", format_vector(weight_grad));

    print_section("Summary");
    println!(" - We used core Variables + ComputationContext directly (no pipeline engine).");
    println!(" - Relu and MeanSquareError are regular functions in the DAG.");
    println!(" - Gradients flow back to Weights as expected.");
    println!(
        " - Parents define data dependencies; ComputationContext walks them for forward/backward."
    );
}
