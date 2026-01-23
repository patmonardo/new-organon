use crate::applications::services::tsjson_support::{err, ok, FacadeContext};
use crate::collections::backends::vec::{VecDouble, VecDoubleArray, VecLong};
use crate::config::GraphStoreConfig;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::id_map::SimpleIdMap;
use crate::types::graph::RelationshipTopology;
use crate::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::node::{
    DefaultDoubleArrayNodePropertyValues, DefaultDoubleNodePropertyValues,
    DefaultLongNodePropertyValues,
};
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::properties::relationship::{
    DefaultDoubleRelationshipPropertyValues, DefaultLongRelationshipPropertyValues,
};
use crate::types::schema::GraphSchema;

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) fn handle_graph_store(request: &Value, ctx: &FacadeContext) -> Value {
    let op = ctx.op.as_str();

    match op {
        "put" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            let Some(snapshot) = request.get("snapshot") else {
                return err(op, "INVALID_REQUEST", "Missing required field: snapshot");
            };

            let nodes_value = snapshot.get("nodes").and_then(|v| v.as_array());
            let Some(nodes) = nodes_value else {
                return err(
                    op,
                    "INVALID_REQUEST",
                    "snapshot.nodes must be a non-empty integer array",
                );
            };
            if nodes.is_empty() {
                return err(
                    op,
                    "INVALID_REQUEST",
                    "snapshot.nodes must be a non-empty integer array",
                );
            }

            let mut original_node_ids: Vec<i64> = Vec::with_capacity(nodes.len());
            for v in nodes.iter() {
                let Some(n) = v.as_i64() else {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "snapshot.nodes must be a non-empty integer array",
                    );
                };
                original_node_ids.push(n);
            }

            let mut index_by_original: HashMap<i64, i64> =
                HashMap::with_capacity(original_node_ids.len());
            for (idx, original) in original_node_ids.iter().copied().enumerate() {
                index_by_original.insert(original, idx as i64);
            }

            #[derive(Clone, Debug)]
            struct RelEdge {
                source: i64,
                target: i64,
                props: HashMap<String, serde_json::Value>,
            }
            let mut rels_by_type: HashMap<String, Vec<RelEdge>> = HashMap::new();
            if let Some(rels) = snapshot.get("relationships").and_then(|v| v.as_array()) {
                for rel in rels.iter() {
                    let Some(rel_type) = rel.get("type").and_then(|v| v.as_str()) else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].type must be a non-empty string",
                        );
                    };
                    if rel_type.trim().is_empty() {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].type must be a non-empty string",
                        );
                    }
                    let Some(source_original) = rel.get("source").and_then(|v| v.as_i64()) else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].source must be an integer",
                        );
                    };
                    let Some(target_original) = rel.get("target").and_then(|v| v.as_i64()) else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].target must be an integer",
                        );
                    };

                    let Some(source_mapped) = index_by_original.get(&source_original).copied()
                    else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].source not found in snapshot.nodes",
                        );
                    };
                    let Some(target_mapped) = index_by_original.get(&target_original).copied()
                    else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].target not found in snapshot.nodes",
                        );
                    };

                    rels_by_type
                        .entry(rel_type.to_string())
                        .or_default()
                        .push(RelEdge {
                            source: source_mapped,
                            target: target_mapped,
                            props: rel
                                .get("properties")
                                .and_then(|v| v.as_object())
                                .map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                                .unwrap_or_default(),
                        });
                }
            }

            let mut relationship_topologies = HashMap::new();
            let mut rel_props_by_type: HashMap<
                String,
                HashMap<String, Vec<Vec<serde_json::Value>>>,
            > = HashMap::new();

            for (rel_type, edges) in rels_by_type.into_iter() {
                let mut adjacency: Vec<Vec<i64>> = vec![Vec::new(); original_node_ids.len()];

                let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
                for e in edges.iter() {
                    for k in e.props.keys() {
                        keys.insert(k.clone());
                    }
                }

                if !keys.is_empty() {
                    let mut by_key: HashMap<String, Vec<Vec<serde_json::Value>>> = HashMap::new();
                    for k in keys.iter() {
                        by_key.insert(k.clone(), vec![Vec::new(); original_node_ids.len()]);
                    }
                    rel_props_by_type.insert(rel_type.clone(), by_key);
                }

                for e in edges.into_iter() {
                    adjacency[e.source as usize].push(e.target);
                    if let Some(by_key) = rel_props_by_type.get_mut(&rel_type) {
                        for (_k, per_source) in by_key.iter_mut() {
                            let v = e.props.get(_k).cloned().unwrap_or(serde_json::Value::Null);
                            per_source[e.source as usize].push(v);
                        }
                    }
                }

                let topology = RelationshipTopology::new(adjacency, None);
                relationship_topologies.insert(RelationshipType::of(&rel_type), topology);
            }

            let database_info = DatabaseInfo::new(
                DatabaseId::new(&ctx.db),
                DatabaseLocation::remote("tsjson", 0, None, None),
            );

            let mut store = DefaultGraphStore::new(
                GraphStoreConfig::default(),
                GraphName::new(graph_name),
                database_info,
                GraphSchema::empty(),
                Capabilities::default(),
                SimpleIdMap::from_original_ids(original_node_ids),
                relationship_topologies,
            );

            // persist rel props
            for (rel_type, by_key) in rel_props_by_type.into_iter() {
                let rel_type_id = RelationshipType::of(&rel_type);
                for (key, per_source) in by_key.into_iter() {
                    let mut flat: Vec<serde_json::Value> = Vec::new();
                    for src in per_source.into_iter() {
                        flat.extend(src);
                    }
                    if flat.is_empty() {
                        continue;
                    }

                    let mut all_longs: Vec<i64> = Vec::with_capacity(flat.len());
                    let mut all_doubles: Vec<f64> = Vec::with_capacity(flat.len());
                    let mut is_all_longs = true;
                    for v in flat.iter() {
                        if let Some(i) = v.as_i64() {
                            all_longs.push(i);
                            all_doubles.push(i as f64);
                        } else if let Some(f) = v.as_f64() {
                            is_all_longs = false;
                            all_doubles.push(f);
                        } else {
                            is_all_longs = false;
                            all_doubles.push(f64::NAN);
                        }
                    }

                    let element_count = all_doubles.len();
                    let pv: Arc<dyn RelationshipPropertyValues> = if is_all_longs {
                        let backend = VecLong::from(all_longs);
                        Arc::new(
                            DefaultLongRelationshipPropertyValues::<VecLong>::from_collection(
                                backend,
                                element_count,
                            ),
                        )
                    } else {
                        let backend = VecDouble::from(all_doubles);
                        Arc::new(
                            DefaultDoubleRelationshipPropertyValues::<VecDouble>::from_collection(
                                backend,
                                element_count,
                            ),
                        )
                    };

                    if let Err(e) =
                        store.add_relationship_property(rel_type_id.clone(), key.clone(), pv)
                    {
                        return err(op, "ERROR", format!("Failed to add relationship property '{key}' for type '{rel_type}': {e}"));
                    }
                }
            }

            // node properties
            if let Some(props_obj) = snapshot.get("nodeProperties").and_then(|v| v.as_object()) {
                let node_count = GraphStore::node_count(&store);
                for (key, val) in props_obj.iter() {
                    let Some(arr) = val.as_array() else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.nodeProperties values must be arrays",
                        );
                    };
                    if arr.len() != node_count {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.nodeProperties[*] arrays must match snapshot.nodes length",
                        );
                    }

                    let mut saw_array = false;
                    let mut saw_scalar = false;
                    for v in arr.iter() {
                        if v.is_array() {
                            saw_array = true;
                        } else if v.is_null() || v.is_number() {
                            saw_scalar = true;
                        } else {
                            saw_scalar = true;
                        }
                    }

                    if saw_array && saw_scalar {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.nodeProperties arrays must contain either only numbers or only number arrays",
                        );
                    }

                    let pv: Arc<dyn NodePropertyValues> = if saw_array {
                        let mut data: Vec<Option<Vec<f64>>> = Vec::with_capacity(arr.len());
                        let mut dimension: Option<usize> = None;
                        for v in arr.iter() {
                            if v.is_null() {
                                data.push(None);
                                continue;
                            }

                            let Some(inner) = v.as_array() else {
                                return err(
                                    op,
                                    "INVALID_REQUEST",
                                    "snapshot.nodeProperties array entries must be arrays of numbers",
                                );
                            };

                            let mut vec = Vec::with_capacity(inner.len());
                            for elem in inner.iter() {
                                if let Some(i) = elem.as_i64() {
                                    vec.push(i as f64);
                                } else if let Some(f) = elem.as_f64() {
                                    vec.push(f);
                                } else {
                                    return err(
                                        op,
                                        "INVALID_REQUEST",
                                        "snapshot.nodeProperties array entries must contain only numbers",
                                    );
                                }
                            }

                            if let Some(dim) = dimension {
                                if vec.len() != dim {
                                    return err(
                                        op,
                                        "INVALID_REQUEST",
                                        "snapshot.nodeProperties array entries must have a consistent length",
                                    );
                                }
                            } else {
                                dimension = Some(vec.len());
                            }

                            data.push(Some(vec));
                        }

                        Arc::new(
                            DefaultDoubleArrayNodePropertyValues::<VecDoubleArray>::from_collection(
                                VecDoubleArray::from(data),
                                node_count,
                            ),
                        )
                    } else {
                        let mut all_longs = Vec::with_capacity(arr.len());
                        let mut all_doubles = Vec::with_capacity(arr.len());
                        let mut is_all_longs = true;
                        let mut is_all_numbers = true;
                        for v in arr.iter() {
                            if let Some(i) = v.as_i64() {
                                all_longs.push(i);
                                all_doubles.push(i as f64);
                            } else if let Some(f) = v.as_f64() {
                                is_all_longs = false;
                                all_doubles.push(f);
                            } else {
                                is_all_numbers = false;
                                break;
                            }
                        }
                        if !is_all_numbers {
                            return err(
                                op,
                                "INVALID_REQUEST",
                                "snapshot.nodeProperties arrays must contain only numbers",
                            );
                        }

                        if is_all_longs {
                            Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(
                                VecLong::from(all_longs),
                                node_count,
                            ))
                        } else {
                            Arc::new(
                                DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(
                                    VecDouble::from(all_doubles),
                                    node_count,
                                ),
                            )
                        }
                    };

                    let labels = std::collections::HashSet::from([NodeLabel::all_nodes()]);
                    if let Err(e) = store.add_node_property(labels, key.to_string(), pv) {
                        return err(
                            op,
                            "ERROR",
                            format!("Failed to add node property '{key}': {e}"),
                        );
                    }
                }
            }

            let node_count = GraphStore::node_count(&store) as u64;
            let relationship_count_actual = GraphStore::relationship_count(&store) as u64;

            ctx.catalog.set(graph_name, Arc::new(store));

            ok(
                op,
                serde_json::json!({ "graphName": graph_name, "nodeCount": node_count, "relationshipCount": relationship_count_actual }),
            )
        }
        _ => err(op, "UNSUPPORTED_OP", "Unsupported graph_store operation."),
    }
}
