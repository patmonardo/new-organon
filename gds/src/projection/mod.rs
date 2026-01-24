//! Projection-related types for graph data science.
//!
//! Simple module containing lightweight projection types (labels, relationship types,
//! orientations) and delegating heavy codegen machinery to the `codegen` submodule.
//!
//! ## Architecture - GDSL Runtime
//!
//! The `projection` module is the **GDSL Runtime** (Graph Domain Specific Language).
//! It provides the execution environment for graph computations and data transformations.
//!
//! - **projection/** - Core projection types (labels, orientations, relationship types)
//! - **projection/traits/** - Core abstractions (ElementProjection, PropertyMapping)
//! - **projection/impls/** - Concrete implementations
//! - **projection/codegen/** - Code generation descriptors and utilities
//! - **projection/factory/** - Data ingestion (Arrow → GraphStore, future Neo4j/Polars)
//! - **projection/eval/** - Execution runtime (ML pipelines, Form evaluators, Procedures)
//!
//! ## Module Separation
//!
//! - **factory** = Concealing power (CAR / given data) - external/native data → GraphStore (Image)
//! - **eval** = Revealing power (CDR / derived computations) - computations on GraphStore → results
//! - **codegen** = Utilities for generating execution code

// ------------------------------------------------------------------------
// Core projection types (simple, widely used)
// ------------------------------------------------------------------------
pub mod impls;
pub mod node_label;
pub mod orientation;
pub mod power;
pub mod relationship_type;
pub mod trace;
pub mod traits;

// Re-export the stable projection surface
pub use impls::*;
pub use node_label::*;
pub use orientation::*;
pub use power::*;
pub use relationship_type::*;
pub use trace::*;
pub use traits::*;

// ------------------------------------------------------------------------
// Factory layer (data ingestion: external data → GraphStore)
// ------------------------------------------------------------------------
pub mod factory;

// ------------------------------------------------------------------------
// Eval layer (execution runtime: ML, Form, Procedures)
// ------------------------------------------------------------------------
pub mod eval;

// ------------------------------------------------------------------------
// Codegen layer (code generation utilities)
// ------------------------------------------------------------------------
pub mod codegen;

// Re-export commonly used codegen types
// pub use codegen::functors::{GrossSubtleFunctor, GrossToSubtle, SubtleToGross};  // Form processor dependency
// pub use codegen::descriptors::property;  // Moved to Reality

// That's it! Everything else stays under codegen::, factory::, or eval::.
// If you need ComputationDescriptor, use: crate::projection::codegen::descriptors::ComputationDescriptor
// If you need StorageDescriptor, use: crate::projection::codegen::descriptors::storage::StorageDescriptor
// If you need form_processor, use: crate::projection::eval::form_processor
// If you need ArrowNativeFactory, use: crate::projection::factory::arrow::ArrowNativeFactory
