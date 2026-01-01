#![allow(clippy::all)]

use std::collections::HashMap;
use std::sync::Arc;

use gds::collections::backends::vec::{VecFloat, VecInt};
use gds::config::GraphStoreConfig;
use gds::projection::{NodeLabel, RelationshipType};
use gds::types::graph::id_map::{IdMap, SimpleIdMap};
use gds::types::graph::RelationshipTopology;
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
};
use gds::types::prelude::GraphStore;
use gds::types::properties::node::impls::default_node_property_values::DefaultFloatNodePropertyValues;
use gds::types::properties::relationship::impls::default_relationship_property_values::DefaultIntRelationshipPropertyValues;
use gds::types::properties::relationship::impls::default_relationship_property_values::DefaultRelationshipPropertyValues;
use gds::types::schema::{Direction, MutableGraphSchema};
use gds::types::ValueType;

#[test]
fn projection_factory_induces_subgraph_and_preserves_labels() {
    let rel_type = RelationshipType::of("CONNECTS");
    let labels = vec![NodeLabel::of("User"), NodeLabel::of("Item")];

    // Original graph: 4 nodes, directed edges with inverse index.
    // 0 -> 1,2
    // 1 -> 2,3
    // 2 -> 3
    // 3 -> 0
    let outgoing = vec![vec![1, 2], vec![2, 3], vec![3], vec![0]];
    let incoming = vec![vec![3], vec![0], vec![0, 1], vec![1, 2]];

    let store = build_store(&labels, rel_type.clone(), outgoing, incoming);

    let selection = vec![0, 2, 3];
    let result = store
        .commit_induced_subgraph_by_original_node_ids(GraphName::new("projected"), &selection)
        .expect("projection succeeds");

    let projected = result.store;
    let mapping = result.old_to_new_mapping;
    let kept = result.relationships_kept_by_type;

    assert_eq!(projected.node_count(), 3);
    assert_eq!(projected.relationship_count(), 3);
    assert_eq!(mapping.get(&0), Some(&0));
    assert_eq!(mapping.get(&2), Some(&1));
    assert_eq!(mapping.get(&3), Some(&2));
    assert_eq!(kept.get(&rel_type), Some(&3));

    // Labels should survive the projection.
    let projected_labels = projected.node_labels();
    for label in labels {
        assert!(projected_labels.contains(&label));
    }
}

#[test]
fn projection_factory_projects_node_properties() {
    let labels = vec![NodeLabel::of("User"), NodeLabel::of("Item")];
    let rel_type = RelationshipType::of("CONNECTS");

    let mut store = build_store(
        &labels,
        rel_type,
        vec![vec![1, 2], vec![2, 3], vec![3], vec![0]],
        vec![vec![3], vec![0], vec![0, 1], vec![1, 2]],
    );

    store
        .add_node_property_f64("score".to_string(), vec![0.5, 1.5, 2.5, 3.5])
        .expect("property added");

    let selection = vec![0, 2, 3];
    let result = store
        .commit_induced_subgraph_by_original_node_ids(GraphName::new("projected"), &selection)
        .expect("projection succeeds");

    let projected = result.store;

    let keys = projected.node_property_keys();
    assert!(keys.contains("score"));

    let values = projected
        .node_property_values("score")
        .expect("score projected");
    assert_eq!(values.double_value(0).unwrap(), 0.5);
    assert_eq!(values.double_value(1).unwrap(), 2.5);
    assert_eq!(values.double_value(2).unwrap(), 3.5);
}

#[test]
fn projection_factory_projects_float_node_properties() {
    let labels = vec![NodeLabel::of("User"), NodeLabel::of("Item")];
    let rel_type = RelationshipType::of("CONNECTS");

    let mut store = build_store(
        &labels,
        rel_type,
        vec![vec![1, 2], vec![2, 3], vec![3], vec![0]],
        vec![vec![3], vec![0], vec![0, 1], vec![1, 2]],
    );

    let backend = VecFloat::from(vec![1.0f32, 2.0, 3.0, 4.0]);
    let pv = DefaultFloatNodePropertyValues::<VecFloat>::from_collection(backend, 4);
    store
        .add_node_property(store.node_labels(), "float_score", Arc::new(pv))
        .expect("add float property");

    let selection = vec![0, 2, 3];
    let result = store
        .commit_induced_subgraph_by_original_node_ids(GraphName::new("projected"), &selection)
        .expect("projection succeeds");

    let projected = result.store;

    let values = projected
        .node_property_values("float_score")
        .expect("float_score projected");
    assert_eq!(values.double_value(0).unwrap(), 1.0);
    assert_eq!(values.double_value(1).unwrap(), 3.0);
    assert_eq!(values.double_value(2).unwrap(), 4.0);
}

#[test]
fn projection_factory_projects_relationship_properties() {
    let labels = vec![NodeLabel::of("User"), NodeLabel::of("Item")];
    let rel_type = RelationshipType::of("CONNECTS");

    // Outgoing flatten order:
    // 0->1, 0->2, 1->2, 1->3, 2->3, 3->0
    let outgoing = vec![vec![1, 2], vec![2, 3], vec![3], vec![0]];
    let incoming = vec![vec![3], vec![0], vec![0, 1], vec![1, 2]];
    let mut store = build_store(&labels, rel_type.clone(), outgoing, incoming);

    let values: Vec<f64> = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0];
    let rel_values = DefaultRelationshipPropertyValues::with_values(values, 0.0, 6);
    store
        .add_relationship_property(rel_type.clone(), "weight", Arc::new(rel_values))
        .expect("add relationship property");

    let selection = vec![0, 2, 3];
    let result = store
        .commit_induced_subgraph_by_original_node_ids(GraphName::new("projected"), &selection)
        .expect("projection succeeds");

    let projected = result.store;

    assert_eq!(projected.relationship_count(), 3);
    let projected_values = projected
        .relationship_property_values(&rel_type, "weight")
        .expect("weight projected");

    // Kept edges (flattened): 0->2 (20.0), 2->3 (50.0), 3->0 (60.0)
    assert_eq!(projected_values.double_value(0).unwrap(), 20.0);
    assert_eq!(projected_values.double_value(1).unwrap(), 50.0);
    assert_eq!(projected_values.double_value(2).unwrap(), 60.0);
}

#[test]
fn projection_factory_projects_int_relationship_properties() {
    let labels = vec![NodeLabel::of("User"), NodeLabel::of("Item")];
    let rel_type = RelationshipType::of("CONNECTS");

    let outgoing = vec![vec![1, 2], vec![2, 3], vec![3], vec![0]];
    let incoming = vec![vec![3], vec![0], vec![0, 1], vec![1, 2]];
    let mut store = build_store(&labels, rel_type.clone(), outgoing, incoming);

    let values: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let pv =
        DefaultIntRelationshipPropertyValues::<VecInt>::from_collection(VecInt::from(values), 6);
    store
        .add_relationship_property(rel_type.clone(), "rank", Arc::new(pv))
        .expect("add relationship property");

    let selection = vec![0, 2, 3];
    let result = store
        .commit_induced_subgraph_by_original_node_ids(GraphName::new("projected"), &selection)
        .expect("projection succeeds");

    let projected = result.store;

    let projected_values = projected
        .relationship_property_values(&rel_type, "rank")
        .expect("rank projected");

    assert_eq!(projected_values.long_value(0).unwrap(), 2);
    assert_eq!(projected_values.long_value(1).unwrap(), 5);
    assert_eq!(projected_values.long_value(2).unwrap(), 6);
}

#[test]
fn projection_factory_handles_multiple_relationship_types() {
    let labels = vec![NodeLabel::of("User"), NodeLabel::of("Item")];
    let connects = RelationshipType::of("CONNECTS");
    let follows = RelationshipType::of("FOLLOWS");

    // 4-node graph with two relationship types.
    // CONNECTS: 0->1, 1->2, 2->3, 3->0
    // FOLLOWS: 1->0, 2->1, 3->2
    let store = build_store_multi(
        &labels,
        vec![
            (
                connects.clone(),
                vec![vec![1], vec![2], vec![3], vec![0]],
                vec![vec![3], vec![0], vec![1], vec![2]],
            ),
            (
                follows.clone(),
                vec![Vec::new(), vec![0], vec![1], vec![2]],
                vec![vec![1], vec![2], vec![3], Vec::new()],
            ),
        ],
    );

    let selection = vec![0, 1, 2];
    let result = store
        .commit_induced_subgraph_by_original_node_ids(GraphName::new("projected"), &selection)
        .expect("projection succeeds");

    let projected = result.store;
    let kept = result.relationships_kept_by_type;

    assert_eq!(projected.node_count(), 3);
    // CONNECTS keeps 0->1 and 1->2, FOLLOWS keeps 1->0 and 2->0.
    assert_eq!(projected.relationship_count(), 4);
    assert_eq!(kept.get(&connects), Some(&2));
    assert_eq!(kept.get(&follows), Some(&2));
}

fn build_store(
    labels: &[NodeLabel],
    rel_type: RelationshipType,
    outgoing: Vec<Vec<i64>>,
    incoming: Vec<Vec<i64>>,
) -> DefaultGraphStore {
    let node_count = outgoing.len();
    let mut schema_builder = MutableGraphSchema::empty();
    for label in labels {
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
    let schema = schema_builder.build();

    let mut id_map = SimpleIdMap::from_original_ids((0..node_count as i64).collect::<Vec<i64>>());
    for (mapped, label) in (0..node_count).zip(labels.iter().cycle()) {
        let mapped = mapped as i64;
        IdMap::add_node_label(&mut id_map, label.clone());
        IdMap::add_node_id_to_label(&mut id_map, mapped, label.clone());
    }

    let mut relationship_topologies = HashMap::new();
    relationship_topologies.insert(
        rel_type,
        RelationshipTopology::new(outgoing, Some(incoming)),
    );

    let mut capabilities = Capabilities::new();
    capabilities.add_feature("projection");

    DefaultGraphStore::new(
        GraphStoreConfig::default(),
        GraphName::new("factory-test"),
        DatabaseInfo::new(
            DatabaseId::new("graph-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        ),
        schema,
        capabilities,
        id_map,
        relationship_topologies,
    )
}

fn build_store_multi(
    labels: &[NodeLabel],
    relationships: Vec<(RelationshipType, Vec<Vec<i64>>, Vec<Vec<i64>>)>,
) -> DefaultGraphStore {
    let node_count = relationships
        .first()
        .map(|(_, outgoing, _)| outgoing.len())
        .unwrap_or(0);

    let mut schema_builder = MutableGraphSchema::empty();
    for label in labels {
        schema_builder.node_schema_mut().add_label(label.clone());
    }
    for (rel_type, _, _) in &relationships {
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
    }
    let schema = schema_builder.build();

    let mut id_map = SimpleIdMap::from_original_ids((0..node_count as i64).collect::<Vec<i64>>());
    for (mapped, label) in (0..node_count).zip(labels.iter().cycle()) {
        let mapped = mapped as i64;
        IdMap::add_node_label(&mut id_map, label.clone());
        IdMap::add_node_id_to_label(&mut id_map, mapped, label.clone());
    }

    let mut relationship_topologies = HashMap::new();
    for (rel_type, outgoing, incoming) in relationships {
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );
    }

    let mut capabilities = Capabilities::new();
    capabilities.add_feature("projection");

    DefaultGraphStore::new(
        GraphStoreConfig::default(),
        GraphName::new("factory-test"),
        DatabaseInfo::new(
            DatabaseId::new("graph-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        ),
        schema,
        capabilities,
        id_map,
        relationship_topologies,
    )
}
