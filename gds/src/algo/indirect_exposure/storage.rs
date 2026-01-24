//! Indirect Exposure storage runtime.
//! Translation source: `org.neo4j.gds.indirectExposure.IndirectExposure`.
//!
//! Responsibilities:
//! - Project graph (optionally weighted) with natural orientation.
//! - Precompute sanctioned mask and total transfers (degree or weighted degree).
//! - Wire Pregel executor with reducer/sender tracking.

use super::{IndirectExposureComputationRuntime, IndirectExposurePregelRuntimeConfig};
use super::spec::{IndirectExposureConfig, IndirectExposureResult};
use crate::pregel::{Pregel, PregelResult, ReducingMessenger};
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use crate::ValueType;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

pub struct IndirectExposureStorageRuntime {
    graph: Arc<dyn Graph>,
    weight_property: Option<String>,
}

impl IndirectExposureStorageRuntime {
    pub fn with_default_projection<G: GraphStore>(
        graph_store: &G,
        weight_property: Option<String>,
    ) -> Result<Self, String> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();

        let graph = if let Some(weight) = &weight_property {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), weight.clone()))
                .collect();

            graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Natural,
                )
                .map_err(|e| e.to_string())?
        } else {
            graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
                .map_err(|e| e.to_string())?
        };

        Ok(Self {
            graph,
            weight_property,
        })
    }

    pub fn run(&self, config: &IndirectExposureConfig) -> Result<IndirectExposureResult, String> {
        let node_count = self.graph.node_count();
        let sanctioned_mask = self.build_sanctioned_mask(config)?;
        let total_transfers = self.build_total_transfers();

        let computation = IndirectExposureComputationRuntime::new(sanctioned_mask, total_transfers);
        let schema = computation.schema();
        let init_fn = computation.init_fn();
        let compute_fn = computation.compute_fn();
        let reducer = computation.reducer();

        let messenger = Arc::new(ReducingMessenger::new(
            node_count,
            reducer.as_trait_object(),
            true,
        ));

        let runtime_cfg = IndirectExposurePregelRuntimeConfig {
            concurrency: config.concurrency.max(1),
            max_iterations: config.max_iterations,
        };

        let result = Pregel::new(
            Arc::clone(&self.graph),
            runtime_cfg,
            schema,
            init_fn,
            compute_fn,
            messenger,
            None,
        )
        .run();

        Ok(materialize_result(&result, node_count))
    }

    fn build_sanctioned_mask(&self, config: &IndirectExposureConfig) -> Result<Vec<bool>, String> {
        let props = self
            .graph
            .node_properties(&config.sanctioned_property)
            .ok_or_else(|| {
                format!(
                    "missing sanctionedProperty '{}'",
                    config.sanctioned_property
                )
            })?;

        let node_count = self.graph.node_count();
        let mut mask = vec![false; node_count];

        match props.value_type() {
            ValueType::Long => {
                for node in 0..node_count {
                    mask[node] = props.long_value(node as u64).unwrap_or(0) == 1;
                }
            }
            ValueType::Double => {
                for node in 0..node_count {
                    mask[node] =
                        (props.double_value(node as u64).unwrap_or(0.0)).round() as i64 == 1;
                }
            }
            _ => {
                return Err(format!(
                    "sanctionedProperty '{}' must be numeric (long/double)",
                    config.sanctioned_property
                ))
            }
        }

        Ok(mask)
    }

    fn build_total_transfers(&self) -> Vec<f64> {
        let node_count = self.graph.node_count();
        let fallback = self.graph.default_property_value();

        let mut totals = vec![0.0f64; node_count];
        for node in 0..node_count {
            let sum = self
                .graph
                .stream_relationships(node as i64, fallback)
                .map(|c| c.property())
                .sum::<f64>();
            let degree = self.graph.degree(node as i64) as f64;
            let total = if self.weight_property.is_some() {
                sum
            } else {
                degree
            };
            totals[node] = if total <= 0.0 { 1.0 } else { total };
        }
        totals
    }
}

fn materialize_result(result: &PregelResult, node_count: usize) -> IndirectExposureResult {
    let mut exposures = Vec::with_capacity(node_count);
    let mut roots = Vec::with_capacity(node_count);
    let mut parents = Vec::with_capacity(node_count);
    let mut hops = Vec::with_capacity(node_count);

    for node in 0..node_count {
        exposures.push(result.node_values.double_value(EXPOSURE_KEY, node));
        roots.push(result.node_values.long_value(ROOT_KEY, node));
        parents.push(result.node_values.long_value(PARENT_KEY, node));
        hops.push(result.node_values.long_value(HOP_KEY, node));
    }

    IndirectExposureResult {
        exposures,
        roots,
        parents,
        hops,
        iterations_ran: result.ran_iterations,
        did_converge: result.did_converge,
    }
}

// Constants reused from computation for materialization
const EXPOSURE_KEY: &str = "exposure";
const HOP_KEY: &str = "hop";
const PARENT_KEY: &str = "parent";
const ROOT_KEY: &str = "root";
