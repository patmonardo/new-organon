use super::computation::ConductanceComputationRuntime;
use super::spec::ConductanceConfig;
use super::storage::ConductanceStorageRuntime;
use crate::algo::conductance::progress_task;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskProgressTracker;
use crate::projection::RelationshipType;
use crate::types::graph::RelationshipTopology;
use crate::types::graph_store::Capabilities;
use crate::types::graph_store::{DatabaseId, DatabaseInfo, DatabaseLocation, GraphName};
use crate::types::prelude::DefaultGraphStore;
use crate::types::prelude::GraphStore;
use crate::types::schema::GraphSchema;
use std::collections::HashMap;

fn make_store(outgoing: Vec<Vec<i64>>) -> DefaultGraphStore {
    let node_count = outgoing.len();

    let graph_name = GraphName::new("g");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("db"),
        DatabaseLocation::remote("localhost", 7687, None, None),
    );

    // Minimal schema/capabilities.
    let schema = GraphSchema::empty();
    let capabilities = Capabilities::default();

    // Simple id_map with nodes 0..node_count.
    let id_map = crate::types::graph::id_map::SimpleIdMap::from_original_ids(
        (0..node_count as i64).collect::<Vec<_>>(),
    );

    let topology = RelationshipTopology::new(outgoing, None);

    let mut relationship_topologies = HashMap::new();
    relationship_topologies.insert(RelationshipType::of("R"), topology);

    DefaultGraphStore::new(
        crate::config::GraphStoreConfig::default(),
        graph_name,
        database_info,
        schema,
        capabilities,
        id_map,
        relationship_topologies,
    )
}

#[test]
fn conductance_matches_expected_small_graph() {
    // Directed graph with 3 nodes:
    // 0 -> 1 (internal for community 0)
    // 1 -> 2 (external for community 0)
    // 2 -> 0 (external for community 1)
    let mut store = make_store(vec![vec![1], vec![2], vec![0]]);

    // community ids: [0,0,1]
    store
        .add_node_property_i64("community".to_string(), vec![0, 0, 1])
        .expect("add community property");

    let config = ConductanceConfig {
        concurrency: 2,
        min_batch_size: 1,
        has_relationship_weight_property: false,
        community_property: "community".to_string(),
    };

    let storage = ConductanceStorageRuntime::new();
    let mut runtime = ConductanceComputationRuntime::new();

    let base_task = progress_task(store.node_count());
    let registry_factory = crate::core::utils::progress::EmptyTaskRegistryFactory;
    let mut progress = TaskProgressTracker::with_registry(
        base_task,
        Concurrency::of(config.concurrency.max(1)),
        crate::core::utils::progress::JobId::new(),
        &registry_factory,
    );
    let result = storage
        .compute_conductance(
            &mut runtime,
            &store,
            &config,
            &mut progress,
            &TerminationFlag::default(),
        )
        .expect("compute conductance");

    // Community 0: internal=1, external=1 => 0.5
    assert!((result.community_conductances.get(&0).unwrap() - 0.5).abs() < 1e-12);
    // Community 1: internal=0, external=1 => 1.0
    assert!((result.community_conductances.get(&1).unwrap() - 1.0).abs() < 1e-12);
    // Global avg: (0.5 + 1.0) / 2 = 0.75
    assert!((result.global_average_conductance - 0.75).abs() < 1e-12);
}
