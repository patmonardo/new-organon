//! Minimal “embedding → similarity” example.
//!
//! This intentionally uses a dependency-free hashing trick embedding so we can
//! iterate on the ML substrate without pulling in external model runtimes.
//!
//! Run:
//!   cargo run -p gds --example ml_embedding_similarity --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\n\
Run: cargo run -p gds --example ml_embedding_similarity --features ml"
    );
}

#[cfg(feature = "ml")]
fn main() {
    use gds::ml::core::tensor::operations::cosine_similarity;
    use gds::ml::embeddings::hashing_embed;

    let a = "pure reason speaks for itself";
    let b = "discursive syllogism is pure reason manifested";
    let c = "graph algorithms over adjacency lists";

    let dims = 256;
    let va = hashing_embed(a, dims);
    let vb = hashing_embed(b, dims);
    let vc = hashing_embed(c, dims);

    println!("dims={dims}");
    println!("cos(a,b) = {:.4}", cosine_similarity(va.as_slice(), vb.as_slice()));
    println!("cos(a,c) = {:.4}", cosine_similarity(va.as_slice(), vc.as_slice()));
    println!("cos(b,c) = {:.4}", cosine_similarity(vb.as_slice(), vc.as_slice()));
}



