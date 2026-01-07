use super::modes;
use super::request::SteinerTreeRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::CatalogLoader;
use crate::types::catalog::GraphCatalog;
use serde_json::Value;
use std::sync::Arc;

pub fn handle_steiner_tree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "steiner_tree";

    let parsed = match SteinerTreeRequest::parse(request) {
        Ok(r) => r,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let graph_resources =
        match CatalogLoader::load_or_err(catalog.as_ref(), &parsed.common.graph_name) {
            Ok(r) => r,
            Err(e) => return err(op, "GRAPH_NOT_FOUND", &e.to_string()),
        };

    match parsed.common.mode {
        Mode::Stream => modes::stream::run(op, &parsed, &graph_resources),
        Mode::Stats => modes::stats::run(op, &parsed, &graph_resources),
        Mode::Estimate => modes::estimate::run(op, &parsed, &graph_resources),
        Mode::Mutate => err(
            op,
            "NOT_IMPLEMENTED",
            "SteinerTree mutate is not implemented yet",
        ),
        Mode::Write => err(
            op,
            "NOT_IMPLEMENTED",
            "SteinerTree write is not implemented yet",
        ),
    }
}
