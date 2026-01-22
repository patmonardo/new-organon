use std::sync::Arc;

use crate::applications::graph_store_catalog::configs::NativeProjectionConfig;
use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::ProjectionResult;
use crate::core::User;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph_store::{DatabaseId, DefaultGraphStore, GraphStore as _};
use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig, Randomizable};
use rand::rngs::StdRng;
use rand::SeedableRng;

/// NativeProjectApplication
///
/// Pass-1 implementation:
/// - If `source_graph_name` is provided, clone the existing graph store from the catalog.
/// - Otherwise, if `fictitious_loading` is true, generate a small deterministic graph store.
///
/// This keeps the contract Java-shaped without requiring a real DB loader yet.
#[derive(Clone, Debug, Default)]
pub struct NativeProjectApplication;

impl NativeProjectApplication {
    pub fn project(
        &self,
        graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
        user: &dyn User,
        database_id: &DatabaseId,
        cfg: &NativeProjectionConfig,
    ) -> Result<ProjectionResult, String> {
        let started = std::time::Instant::now();

        let mut store: DefaultGraphStore = match cfg.source_graph_name.as_deref() {
            Some(source) => {
                let src = graph_store_catalog_service.get_graph_store(user, database_id, source)?;
                (*src).clone()
            }
            None => {
                if !cfg.fictitious_loading {
                    return Err(
                        "project_native requires projectionConfig.sourceGraphName (or set fictitiousLoading=true)"
                            .to_string(),
                    );
                }

                let config = RandomGraphConfig {
                    graph_name: cfg.graph_name.clone(),
                    database_name: database_id.to_string(),
                    node_count: 64,
                    relationships: vec![RandomRelationshipConfig::new("REL", 0.1)],
                    ..Default::default()
                };
                let mut rng = StdRng::seed_from_u64(0);
                DefaultGraphStore::random_with_rng(&config, &mut rng)
                    .map_err(|e| format!("Failed to generate fictitious graph store: {e}"))?
            }
        };

        // Minimal “projection vocabulary” (pass-1):
        // - nodeLabels filter: validate labels exist (no physical node filtering yet)
        // Java parity: "*" means PROJECT_ALL.
        let node_has_wildcard = cfg.node_labels.iter().any(|s| s.trim() == "*");
        if !cfg.node_labels.is_empty() && !node_has_wildcard {
            for l in cfg.node_labels.iter() {
                let t = l.trim();
                if t.is_empty() {
                    continue;
                }
                let label = NodeLabel::of(t);
                if !store.has_node_label(&label) {
                    return Err(format!(
                        "Unknown node label '{t}' in projectionConfig.nodeLabels"
                    ));
                }
            }
        }

        // Minimal “projection vocabulary” (pass-1):
        // - relationshipTypes filter: remove relationship types not included.
        // Java parity: "*" means PROJECT_ALL.
        let has_wildcard = cfg.relationship_types.iter().any(|s| s.trim() == "*");
        if !cfg.relationship_types.is_empty() && !has_wildcard {
            let keep: std::collections::HashSet<RelationshipType> = cfg
                .relationship_types
                .iter()
                .filter_map(|s| {
                    let t = s.trim();
                    if t.is_empty() {
                        None
                    } else {
                        Some(RelationshipType::of(t))
                    }
                })
                .collect();

            let all_types = store.relationship_types();
            for t in all_types.into_iter() {
                if !keep.contains(&t) {
                    store.delete_relationships(&t).map_err(|e| {
                        format!("Failed to filter relationship type '{}': {e}", t.name())
                    })?;
                }
            }
        }

        // Minimal “projection vocabulary” (pass-1): nodeProperties pruning.
        let node_prop_has_wildcard = cfg.node_properties.iter().any(|s| s.trim() == "*");
        if !cfg.node_properties.is_empty() && !node_prop_has_wildcard {
            let keep: std::collections::HashSet<String> = cfg
                .node_properties
                .iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let keys = store.node_property_keys();
            for k in keys {
                if !keep.contains(&k) {
                    store
                        .remove_node_property(&k)
                        .map_err(|e| format!("Failed to prune node property '{k}': {e}"))?;
                }
            }
        }

        // Minimal “projection vocabulary” (pass-1): relationshipProperties pruning.
        let rel_prop_has_wildcard = cfg.relationship_properties.iter().any(|s| s.trim() == "*");
        if !cfg.relationship_properties.is_empty() && !rel_prop_has_wildcard {
            let mut keep: std::collections::HashSet<String> = cfg
                .relationship_properties
                .iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            // Always keep selector properties so pruning doesn't invalidate selection.
            for (_, v) in cfg.relationship_property_selectors.iter() {
                let t = v.trim();
                if !t.is_empty() {
                    keep.insert(t.to_string());
                }
            }
            if let Some(w) = cfg.weight_property.as_deref() {
                let t = w.trim();
                if !t.is_empty() {
                    keep.insert(t.to_string());
                }
            }
            let types = store.relationship_types();
            for t in types {
                let keys = store.relationship_property_keys_for_type(&t);
                for k in keys {
                    if !keep.contains(&k) {
                        store.remove_relationship_property(&t, &k).map_err(|e| {
                            format!(
                                "Failed to prune relationship property '{}' for type '{}': {e}",
                                k,
                                t.name()
                            )
                        })?;
                    }
                }
            }
        }

        let node_count = store.node_count() as u64;
        let relationship_count = store.relationship_count() as u64;

        // Persist into the catalog under the requested name.
        let catalog = graph_store_catalog_service.graph_catalog(user, database_id);
        catalog.set(cfg.graph_name.as_str(), Arc::new(store));

        Ok(ProjectionResult::new(
            cfg.graph_name.clone(),
            node_count,
            relationship_count,
            started.elapsed().as_millis() as u64,
        ))
    }
}
