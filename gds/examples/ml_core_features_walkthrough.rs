//! Expository example: ml::core features extraction
//!
//! This walkthrough builds custom feature extractors and shows how feature
//! vectors are assembled for individual nodes and batches.

use gds::ml::core::batch::Batch;
use gds::ml::core::features::{
    extract, extract_batch, feature_count, AnyFeatureExtractor, BiasFeature, FeatureConsumer,
    ScalarFeatureExtractor,
};
use gds::ml::core::{
    dimensions, ComputationContext, ElementSum, Matrix, Relu, Scalar, Tensor, Variable,
};

fn print_section(title: &str) {
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
}

fn format_vector(data: &[f64]) -> String {
    let items: Vec<String> = data.iter().map(|x| format!("{x:>7.3}")).collect();
    format!("[{}]", items.join(", "))
}

fn format_matrix(m: &Matrix) -> String {
    let mut lines = Vec::new();
    for row in 0..m.rows() {
        let row_items: Vec<String> = (0..m.cols())
            .map(|col| format!("{:.3}", m.data_at(row, col)))
            .collect();
        lines.push(format!("  [{}]", row_items.join(", ")));
    }
    lines.join("\n")
}

// -----------------------------------------------------------------------------
// Simple scalar extractor: returns node_id as f64.
// -----------------------------------------------------------------------------
struct NodeIdFeature;

impl gds::ml::core::features::FeatureExtractor for NodeIdFeature {
    fn dimension(&self) -> usize {
        1
    }
}

impl ScalarFeatureExtractor for NodeIdFeature {
    fn extract(&self, node_id: u64) -> f64 {
        node_id as f64
    }
}

// -----------------------------------------------------------------------------
// Toy batch type for extract_batch demonstration.
// -----------------------------------------------------------------------------
struct VecBatch {
    ids: Vec<u64>,
}

impl VecBatch {
    fn new(ids: Vec<u64>) -> Self {
        Self { ids }
    }
}

impl Batch for VecBatch {
    type ElementIdsIter = std::vec::IntoIter<u64>;

    fn size(&self) -> usize {
        self.ids.len()
    }

    fn element_ids(&self) -> Self::ElementIdsIter {
        self.ids.clone().into_iter()
    }
}

// -----------------------------------------------------------------------------
// Feature consumer that collects features into Vec<Vec<f64>> for display.
// -----------------------------------------------------------------------------
struct VecFeatureConsumer {
    rows: Vec<Vec<f64>>,
}

impl VecFeatureConsumer {
    fn new(rows: usize, row_len: usize) -> Self {
        Self {
            rows: vec![vec![0.0; row_len]; rows],
        }
    }
}

impl FeatureConsumer for VecFeatureConsumer {
    fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
        self.rows[node_offset as usize][offset] = value;
    }

    fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
        let row = &mut self.rows[node_offset as usize];
        for (i, value) in values.iter().enumerate() {
            row[offset + i] = *value;
        }
    }
}

fn main() {
    println!("=== ML Core: Features Walkthrough ===");

    print_section("Step 1: Build extractors");
    let extractors: Vec<AnyFeatureExtractor> = vec![
        AnyFeatureExtractor::Scalar(Box::new(BiasFeature)),
        AnyFeatureExtractor::Scalar(Box::new(NodeIdFeature)),
    ];

    println!("extractors  : bias + node_id");
    println!("feature_dim : {}", feature_count(&extractors));

    print_section("Step 2: Extract a single node");
    let mut consumer = VecFeatureConsumer::new(1, feature_count(&extractors));
    extract(42, 0, &extractors, &mut consumer);
    println!("node 42 => {}", format_vector(&consumer.rows[0]));

    print_section("Step 3: Extract a batch into a Matrix");
    let batch = VecBatch::new(vec![1, 2, 3, 7]);
    let batch_constant = extract_batch(&batch, &extractors);
    let ctx = ComputationContext::new();
    let batch_matrix_data = batch_constant.apply(&ctx);
    let batch_matrix = batch_matrix_data
        .as_any()
        .downcast_ref::<Matrix>()
        .expect("Expected Matrix from extract_batch");

    println!(
        "batch dims : {}",
        dimensions::render(batch_matrix.dimensions())
    );
    println!("{}", format_matrix(batch_matrix));

    print_section("Step 4: Use features in a computation graph (forward-only)");
    println!("Graph:");
    println!("  FeatureMatrix (Constant)");
    println!("            │");
    println!("           Relu");
    println!("            │");
    println!("        ElementSum");

    let feature_var: gds::ml::core::VariableRef = std::sync::Arc::new(batch_constant);
    let relu = std::sync::Arc::new(Relu::with_default_alpha_ref(feature_var.clone()));
    let sum = ElementSum::new_ref(vec![relu.clone()]);

    let graph_ctx = ComputationContext::new();
    let relu_data = graph_ctx.forward(relu.as_ref());
    let relu_matrix = relu_data
        .as_any()
        .downcast_ref::<Matrix>()
        .expect("Expected Matrix from Relu");

    let sum_data = graph_ctx.forward(&sum);
    let sum_value = sum_data
        .as_any()
        .downcast_ref::<Scalar>()
        .expect("Expected Scalar from ElementSum")
        .value();

    println!("relu(features) :");
    println!("{}", format_matrix(relu_matrix));
    println!("element_sum   : {sum_value:.6}");

    print_section("Summary");
    println!(" - Feature extractors declare their dimension via `FeatureExtractor`. ");
    println!(" - `AnyFeatureExtractor` erases scalar vs array extractors for uniform loops.");
    println!(" - `extract_batch` writes a dense feature matrix (rows = nodes, cols = features).");
}
