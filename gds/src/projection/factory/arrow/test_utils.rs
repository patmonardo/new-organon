use std::sync::Arc;

use arrow2::array::{Array, PrimitiveArray, Utf8Array};
use arrow2::chunk::Chunk;
use arrow2::datatypes::{DataType, Field, Schema};

use super::reference::{EdgeTableReference, NodeTableReference};

/// Build a simple NodeTableReference with id + label columns.
pub fn sample_node_table() -> NodeTableReference {
    let ids = PrimitiveArray::<i64>::from_vec(vec![0, 1, 2]);
    let labels = Utf8Array::<i32>::from_slice(["Person", "Person", "Company"]);

    let fields = vec![
        Field::new("id", DataType::Int64, false),
        Field::new("label", DataType::Utf8, true),
    ];
    let schema = Arc::new(Schema::from(fields));

    let chunk: Chunk<Box<dyn Array>> = Chunk::new(vec![Box::new(ids), Box::new(labels)]);

    NodeTableReference::new("nodes", chunk, Arc::clone(&schema))
        .expect("sample node table schema should be valid")
}

/// Build a simple EdgeTableReference with source/target/type columns.
pub fn sample_edge_table() -> EdgeTableReference {
    let sources = PrimitiveArray::<i64>::from_vec(vec![0, 1]);
    let targets = PrimitiveArray::<i64>::from_vec(vec![1, 2]);
    let types = Utf8Array::<i32>::from_slice(["KNOWS", "WORKS_FOR"]);

    let fields = vec![
        Field::new("source", DataType::Int64, false),
        Field::new("target", DataType::Int64, false),
        Field::new("type", DataType::Utf8, true),
    ];
    let schema = Arc::new(Schema::from(fields));

    let chunk: Chunk<Box<dyn Array>> =
        Chunk::new(vec![Box::new(sources), Box::new(targets), Box::new(types)]);

    EdgeTableReference::new("edges", chunk, Arc::clone(&schema))
        .expect("sample edge table schema should be valid")
}
