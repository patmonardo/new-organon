//! Eval Runner
//!
//! A thin orchestration surface that composes the evaluators under `projection/eval/`.
//!
//! Today this is intentionally minimal: it delegates to the Form ISA (`FormProcessor`).
//! The purpose of the runner is to provide a stable seam where higher-level concerns
//! (policy/audit/tenancy/quotas/observability) can attach later, without leaking into
//! operators or substrate.

use crate::projection::eval::form::{
    FormArtifacts, FormCatalog, FormError, FormProcessor, FormRequest, FormResult,
};
use crate::projection::eval::procedure::ExecutionContext;

#[derive(Debug, thiserror::Error)]
pub enum RunnerError {
    #[error(transparent)]
    Form(#[from] FormError),
}

/// Primary composition surface for Projection Eval.
///
/// The runner owns the `ExecutionContext` via the underlying `FormProcessor`.
pub struct EvalRunner {
    form: FormProcessor,
}

impl EvalRunner {
    pub fn new(context: ExecutionContext) -> Self {
        Self {
            form: FormProcessor::new(context),
        }
    }

    pub fn context(&self) -> &ExecutionContext {
        self.form.context()
    }

    pub fn context_mut(&mut self) -> &mut ExecutionContext {
        self.form.context_mut()
    }

    pub fn form_catalog_mut(&mut self) -> &mut FormCatalog {
        self.form.catalog_mut()
    }

    /// Execute a fully-formed Form request.
    pub fn run_form(&mut self, request: &FormRequest) -> Result<FormResult, RunnerError> {
        Ok(self.form.evaluate(request)?)
    }

    /// Convenience API: run a program against a base graph using explicit artifacts.
    pub fn run_form_program(
        &mut self,
        graph_name: impl Into<String>,
        program: crate::form::FormShape,
        artifacts: FormArtifacts,
    ) -> Result<FormResult, RunnerError> {
        let mut request = FormRequest::new(graph_name, program);
        request.artifacts = artifacts;
        self.run_form(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::form::{Context, FormShape, Morph, Shape};
    use crate::types::graph_store::GraphStore;
    use crate::types::prelude::{DefaultGraphStore, RandomGraphConfig};

    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn runner_executes_form_and_can_write_output_graph() {
        let graph = Arc::new(DefaultGraphStore::random(&RandomGraphConfig::seeded(7)).unwrap());

        let mut ctx = ExecutionContext::new("user");
        ctx.add_graph("g", graph.clone());

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let fctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec!["passThrough".to_string()]);
        let program = FormShape::new(shape, fctx, morph);

        let mut request = FormRequest::new("g", program);
        request.output_graph_name = Some("out".to_string());

        let mut runner = EvalRunner::new(ctx);
        let result = runner.run_form(&request).unwrap();

        assert_eq!(result.operator, "passThrough");
        assert_eq!(result.graph.node_count(), graph.node_count());
        assert!(runner.context().has_graph("out"));
    }
}
