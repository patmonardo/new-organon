//! Minimal Form program types.
//!
//! This module exists to carry a small, stable “program shape” into the Form evaluator.
//! At runtime, the only required field today is `Morph.patterns`, which drives the
//! `FormOperator` chain.
//!
//! Dialectical convention (semantic mapping; not enforced by the kernel):
//! - `Shape`   → Essence (the program's essential envelope / what is “there” as form)
//! - `Context` → Determination of Essence / Reflection (how the form is determined)
//! - `Morph`   → Ground as **Active Ground** (the operator chain that actualizes the determination)

use std::collections::HashMap;

/// Form program payload passed into the Form evaluator.
#[derive(Debug, Clone)]
pub struct FormShape {
    pub shape: Shape,
    pub context: Context,
    pub morph: Morph,
}

impl FormShape {
    pub fn new(shape: Shape, context: Context, morph: Morph) -> Self {
        Self {
            shape,
            context,
            morph,
        }
    }
}

/// Optional structural metadata about the program.
///
/// Semantic convention: this is the program's *Essence* envelope.
///
/// Currently unused by the evaluator, but retained as a small, non-opinionated envelope
/// for schema-first expansion.
#[derive(Debug, Clone, Default)]
pub struct Shape {
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub type_constraints: HashMap<String, String>,
    pub validation_rules: HashMap<String, String>,
}

impl Shape {
    pub fn new(
        required_fields: Vec<String>,
        optional_fields: Vec<String>,
        type_constraints: HashMap<String, String>,
        validation_rules: HashMap<String, String>,
    ) -> Self {
        Self {
            required_fields,
            optional_fields,
            type_constraints,
            validation_rules,
        }
    }
}

/// Execution context metadata.
///
/// Semantic convention: this carries *Determination of Essence / Reflection* inputs.
///
/// Currently unused by the evaluator; kept lightweight.
#[derive(Debug, Clone)]
pub struct Context {
    pub dependencies: Vec<String>,
    pub execution_order: Vec<String>,
    pub runtime_strategy: String,
    pub conditions: Vec<String>,
}

impl Context {
    pub fn new(
        dependencies: Vec<String>,
        execution_order: Vec<String>,
        runtime_strategy: String,
        conditions: Vec<String>,
    ) -> Self {
        Self {
            dependencies,
            execution_order,
            runtime_strategy,
            conditions,
        }
    }
}

/// The only part of the program required by the current Form ISA.
///
/// Semantic convention: `Morph` is **Active Ground** — the effective ground that
/// realizes the program through the selected operator chain.
#[derive(Debug, Clone, Default)]
pub struct Morph {
    /// Names of Form operators to run, in order.
    pub patterns: Vec<String>,
}

impl Morph {
    pub fn new(patterns: Vec<String>) -> Self {
        Self { patterns }
    }
}
