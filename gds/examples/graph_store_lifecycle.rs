//! GraphStore lifecycle and projection walk-through.
//!
//! This example is intentionally verbose: it prints each phase of building a
//! larger in-memory `DefaultGraphStore`, wiring it with the Collections
//! backends (Arrow vs Vec), projecting a subgraph, and running an algorithm
#![allow(clippy::all)]
//! over the projected view.
//!
//! Run with:
//!   cargo run -p gds --example graph_store_lifecycle

mod enabled {
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    use gds::collections::backends::arrow::ArrowDoubleArray;
    use gds::collections::backends::factory::{
        create_double_backend_from_config, DoubleCollection,
    };
    use gds::collections::backends::vec::VecDouble;
    use gds::config::{GraphStoreConfig, GraphStorePropertiesConfig};
    use gds::mem::graph_store_memory_container::GraphStoreMemoryContainer;
    use gds::prelude::GraphStore;
    use gds::procedures::centrality::PageRankFacade;
    use gds::procedures::community::TriangleFacade;
    use gds::projection::orientation::Orientation;
    use gds::projection::{NodeLabel, RelationshipType};
    use gds::types::graph::id_map::IdMap;
    use gds::types::graph::id_map::MappedNodeId;
    use gds::types::graph::{RelationshipTopology, SimpleIdMap};
    use gds::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use gds::types::properties::graph::DefaultDoubleGraphPropertyValues;
    use gds::types::properties::graph::GraphPropertyValues;
    use gds::types::properties::node::DefaultDoubleNodePropertyValues;
    use gds::types::properties::node::NodePropertyValues;
    use gds::types::schema::{Direction, MutableGraphSchema};
    use gds::types::ValueType;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    const GRAPH_NAME: &str = "graph-store-lifecycle";
    const PROJECTION_NAME: &str = "projected-slice";
    const USER: &str = "demo-user";
    const REL_TYPE: &str = "CONNECTS";
    const LABEL_A: &str = "User";
    const LABEL_B: &str = "Item";

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("GraphStore lifecycle example failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== GraphStore lifecycle + projection demo ===");

        let node_count = 5_000;
        let edges_per_node = 12;
        let properties = GraphStorePropertiesConfig::builder()
            .prefer_arrow(true)
            .enable_adaptive_backend(true)
            .huge_array_threshold(200_000) // keep the threshold visible in prints
            .build()?;
        let config = GraphStoreConfig::builder()
            .properties(properties.clone())
            .build()?;

        let node_backend = config.node_collections_config::<f64>(node_count);
        let vec_only_backend =
            GraphStoreConfig::vec_only().node_collections_config::<f64>(node_count);
        println!(
            "Collections backend selection -> prefer_arrow={} | adaptive={} | huge_threshold={}",
            properties.prefer_arrow,
            properties.enable_adaptive_backend,
            properties.huge_array_threshold
        );
        println!(
            "- node backend for {} elements: {:?} (fallbacks: {:?})",
            node_count, node_backend.backend.primary, node_backend.backend.fallbacks
        );
        println!(
            "- vec_only() would choose: {:?} (fallbacks: {:?})",
            vec_only_backend.backend.primary, vec_only_backend.backend.fallbacks
        );
        println!();

        println!(
            "Step 1: building {node_count} node graph with ~{edges_per_node} outgoing edges per node"
        );
        let mut store = build_graph_store(node_count, edges_per_node, &config)?;
        println!(
            "Graph ready: nodes={} relationships={} inverse_indexed={}",
            store.node_count(),
            store.relationship_count(),
            store
                .inverse_indexed_relationship_types()
                .contains(&RelationshipType::of(REL_TYPE))
        );
        println!("Node labels: {:?}\n", store.node_labels());

        println!("Step 2: hydrate properties using the Collections backend selection");
        add_node_score_property(&mut store, &config)?;
        add_graph_level_property(&mut store, &config)?;

        let node_props = store.node_property_values("activity_score")?;
        let graph_props = store.graph_property_values("graph_summary")?;
        println!("Stored node properties: {:?}", store.node_property_keys());
        println!(
            "Sample activity_score values: node 0 -> {:.3}, node 1 -> {:.3}, node 2 -> {:.3}",
            node_props.double_value(0)?,
            node_props.double_value(1)?,
            node_props.double_value(2)?,
        );
        println!(
            "Graph summary values (double iterator): {:?}",
            graph_props.double_values().take(3).collect::<Vec<_>>()
        );
        println!();

        println!("Step 3: register the store in an in-memory catalog");
        let mut memory = GraphStoreMemoryContainer::new();
        let reserved = memory.add_graph(USER, GRAPH_NAME, estimate_bytes(&store) as u64);
        println!(
            "- [{}] now tracks {} graphs consuming ~{} bytes",
            USER,
            memory.graph_count(USER),
            reserved
        );
        println!("  Graph list: {:?}\n", memory.list_graphs(USER));

        println!("Step 4: project a focused subgraph with induced nodes");
        let selection: Vec<i64> = (0..1_000.min(node_count)).map(|id| id as i64).collect();
        let result = store
            .commit_induced_subgraph_by_original_node_ids(
                GraphName::new(PROJECTION_NAME),
                &selection,
            )
            .expect("projection succeeds");
        let projected = result.store;
        let mapping = result.old_to_new_mapping;
        let kept = result.relationships_kept_by_type;
        println!(
            "- projected graph: nodes={} relationships={} (kept by type: {:?})",
            projected.node_count(),
            projected.relationship_count(),
            kept
        );
        println!(
            "- mapping preserved {} nodes (old -> new id for first 5): {:?}",
            mapping.len(),
            mapping.iter().take(5).collect::<Vec<_>>()
        );
        println!();

        println!("Step 5: inspect graph views and degrees");
        let graph_view = store.get_graph();
        let characteristics = graph_view.characteristics();
        println!(
            "- graph characteristics: directed={} inverse_indexed={} has_rel_props={}",
            characteristics.is_directed(),
            characteristics.is_inverse_indexed(),
            graph_view.has_relationship_property()
        );
        println!(
            "- degree snapshots: node 0 -> {}, node 1 -> {}, node 2 -> {}",
            graph_view.degree(0),
            graph_view.degree(1),
            graph_view.degree(2)
        );
        let undirected_view =
            store.get_graph_with_types_and_orientation(&HashSet::new(), Orientation::Undirected)?;
        println!(
            "- undirected view degree(node 0) -> {} (orientation override)",
            undirected_view.degree(0)
        );
        println!();

        println!("Step 6: run graph algorithms on the projected slice (PageRank + TriangleCount)");
        let projected_arc = Arc::new(projected);
        let stats = PageRankFacade::new(projected_arc.clone())
            .iterations(8)
            .tolerance(1e-6)
            .concurrency(2)
            .stats()?;
        println!(
            "- PageRank stats: converged={} iterations={} exec={}ms",
            stats.converged, stats.iterations_ran, stats.execution_time_ms
        );
        println!(
            "- score distribution: min={:.6} mean={:.6} max={:.6} p90={:.6}",
            stats.min, stats.mean, stats.max, stats.p90
        );
        let topk = PageRankFacade::new(projected_arc.clone())
            .iterations(8)
            .tolerance(1e-6)
            .concurrency(2)
            .stream()?
            .take(5)
            .collect::<Vec<_>>();
        println!(
            "- top PageRank nodes (first 5): {:?}",
            topk.iter()
                .map(|row| format!("{}:{:.4}", row.node_id, row.score))
                .collect::<Vec<_>>()
        );
        let triangle_stats = TriangleFacade::new(projected_arc.clone())
            .concurrency(2)
            .stats()?;
        println!(
            "- triangle count: global={} exec={}ms (undirected view)",
            triangle_stats.global_triangles, triangle_stats.execution_time_ms
        );
        println!();

        println!("Step 7: cleanup lifecycle bookkeeping");
        let after_drop = memory.remove_graph(USER, GRAPH_NAME);
        println!(
            "- removed '{}' from catalog; remaining reserved bytes: {}",
            GRAPH_NAME, after_drop
        );
        println!("=== Demo complete ===");

        Ok(())
    }

    fn build_graph_store(
        node_count: usize,
        edges_per_node: usize,
        config: &GraphStoreConfig,
    ) -> Result<DefaultGraphStore, Box<dyn std::error::Error + Send + Sync>> {
        let rel_type = RelationshipType::of(REL_TYPE);
        let labels = vec![NodeLabel::of(LABEL_A), NodeLabel::of(LABEL_B)];

        let mut schema_builder = MutableGraphSchema::empty();
        for label in &labels {
            schema_builder.node_schema_mut().add_label(label.clone());
            schema_builder.node_schema_mut().add_property(
                label.clone(),
                "activity_score",
                ValueType::Double,
            );
        }
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        schema_builder.relationship_schema_mut().add_property(
            rel_type.clone(),
            Direction::Directed,
            "weight",
            ValueType::Double,
        );
        let schema = schema_builder.build();

        let mut id_map = SimpleIdMap::from_original_ids((0..node_count as i64).collect::<Vec<_>>());
        for label in &labels {
            id_map.add_node_label(label.clone());
        }
        for mapped in 0..node_count as MappedNodeId {
            let label = if mapped % 2 == 0 {
                NodeLabel::of(LABEL_A)
            } else {
                NodeLabel::of(LABEL_B)
            };
            IdMap::add_node_id_to_label(&mut id_map, mapped as i64, label);
        }

        let (outgoing, incoming) = synthetic_topology(node_count, edges_per_node, true);
        let topology = RelationshipTopology::new(outgoing, Some(incoming));
        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(rel_type, topology);

        let mut capabilities = Capabilities::new();
        capabilities.add_feature("write");
        capabilities.add_feature("projection");
        capabilities.add_feature("collections-backend");

        Ok(DefaultGraphStore::new(
            config.clone(),
            GraphName::new(GRAPH_NAME),
            DatabaseInfo::new(
                DatabaseId::new("graph-db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            capabilities,
            id_map,
            relationship_topologies,
        ))
    }

    fn synthetic_topology(
        node_count: usize,
        edges_per_node: usize,
        inverse_indexed: bool,
    ) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
        let mut rng = StdRng::seed_from_u64(7);
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for source in 0..node_count {
            let mut targets = HashSet::new();
            while targets.len() < edges_per_node && targets.len() < node_count.saturating_sub(1) {
                let target = rng.gen_range(0..node_count);
                if target != source {
                    targets.insert(target as i64);
                }
            }

            for target in targets {
                outgoing[source].push(target);
                if inverse_indexed {
                    incoming[target as usize].push(source as i64);
                }
            }
        }

        (outgoing, incoming)
    }

    fn add_node_score_property(
        store: &mut DefaultGraphStore,
        config: &GraphStoreConfig,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let node_count = store.node_count();
        let values: Vec<f64> = (0..node_count)
            .map(|idx| ((idx % 100) as f64) / 10.0)
            .collect();

        let backend_config = config.node_collections_config::<f64>(node_count);
        let backend = create_double_backend_from_config(&backend_config, values);
        let property_values: Arc<dyn NodePropertyValues> = match backend {
            DoubleCollection::Vec(collection) => Arc::new(DefaultDoubleNodePropertyValues::<
                VecDouble,
            >::from_collection(
                collection, node_count
            )),
            DoubleCollection::Huge(collection) => Arc::new(DefaultDoubleNodePropertyValues::<
                VecDouble,
            >::from_collection(
                VecDouble::from(collection.to_vec()),
                node_count,
            )),
            DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleNodePropertyValues::<
                ArrowDoubleArray,
            >::from_collection(
                collection, node_count
            )),
        };

        let labels = store.node_labels();
        store.add_node_property(labels, "activity_score", property_values)?;
        Ok(())
    }

    fn add_graph_level_property(
        store: &mut DefaultGraphStore,
        config: &GraphStoreConfig,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let values = vec![store.node_count() as f64, store.relationship_count() as f64];
        let backend_config = config.graph_collections_config::<f64>(values.len());
        let backend = create_double_backend_from_config(&backend_config, values);
        let property_values: Arc<dyn GraphPropertyValues> = match backend {
            DoubleCollection::Vec(collection) => {
                Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(collection))
            }
            DoubleCollection::Huge(collection) => Arc::new(DefaultDoubleGraphPropertyValues::<
                VecDouble,
            >::from_collection(
                VecDouble::from(collection.to_vec())
            )),
            DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleGraphPropertyValues::<
                ArrowDoubleArray,
            >::from_collection(
                collection
            )),
        };

        store.add_graph_property("graph_summary", property_values)?;
        Ok(())
    }

    fn estimate_bytes(store: &DefaultGraphStore) -> usize {
        // Crude sizing: nodes + relationships + one double property per node.
        let topology_bytes = store.relationship_count() * std::mem::size_of::<i64>() * 2;
        let node_bytes = store.node_count() * std::mem::size_of::<i64>();
        let property_bytes = store.node_count() * std::mem::size_of::<f64>();
        topology_bytes + node_bytes + property_bytes
    }
}

fn main() {
    enabled::main();
}
