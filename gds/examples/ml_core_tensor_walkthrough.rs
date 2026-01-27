//! Expository example: ml::core tensor primitives
//!
//! This walkthrough shows basic operations on Scalar, Vector, and Matrix.

use gds::ml::core::{dimensions, Matrix, Scalar, Tensor, Vector};

fn square(x: f64) -> f64 {
    x * x
}

fn print_section(title: &str) {
    println!("\n{title}");
    println!("{}", "-".repeat(title.len()));
}

fn format_vector(v: &Vector) -> String {
    let items: Vec<String> = v.as_slice().iter().map(|x| format!("{x:>7.3}")).collect();
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

fn main() {
    println!("=== ML Core: Tensor Walkthrough ===");

    // Scalars
    print_section("Scalars");
    let s = Scalar::new(3.0);
    println!("value : {:.3}", s.value());
    println!("dims  : {}", dimensions::render(s.dimensions()));

    // Vectors
    print_section("Vectors");
    let v = Vector::new(vec![1.0, 2.0, 3.0]);
    let w = Vector::create(0.5, 3);
    let v_plus_w = v.add(&w);
    let v_squared = v.map(square);
    let v_hadamard = v.elementwise_product(&w);

    let v_plus_w = v_plus_w
        .as_any()
        .downcast_ref::<Vector>()
        .expect("Vector expected");
    let v_squared = v_squared
        .as_any()
        .downcast_ref::<Vector>()
        .expect("Vector expected");
    let v_hadamard = v_hadamard
        .as_any()
        .downcast_ref::<Vector>()
        .expect("Vector expected");

    println!("v      : {}", format_vector(&v));
    println!("w      : {}", format_vector(&w));
    println!("v + w  : {}", format_vector(v_plus_w));
    println!("v^2    : {}", format_vector(v_squared));
    println!("v ⊙ w  : {}", format_vector(v_hadamard));

    // Matrices
    print_section("Matrices");
    let a = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
    let b = Matrix::new(vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0], 3, 2);
    let c = a.multiply(&b);

    println!("A dims : {}", dimensions::render(a.dimensions()));
    println!("A\n{}", format_matrix(&a));
    println!("B dims : {}", dimensions::render(b.dimensions()));
    println!("B\n{}", format_matrix(&b));
    println!("A × B  : {}", dimensions::render(c.dimensions()));
    println!("{}", format_matrix(&c));

    print_section("Summary");
    println!(" - Tensors are concrete types (Scalar, Vector, Matrix) implementing `Tensor`. ");
    println!(" - `Tensor` methods (map, add, elementwise_product) are available across types.");
    println!(
        " - Matrix provides specialized linear algebra helpers (multiply, transpose variants)."
    );
}
