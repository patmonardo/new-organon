use super::*;
use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::core::model::{InMemoryModelCatalog, ModelCatalog};
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::mem::MemoryEstimation;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

#[test]
fn graphsage_train_then_infer_via_catalog() {
    let store = DefaultGraphStore::random(&RandomGraphConfig {
        graph_name: "graphsage-algo".into(),
        database_name: "in-memory".into(),
        node_count: 50,
        node_labels: vec!["A".into(), "B".into()],
        relationships: vec![RandomRelationshipConfig::new("R", 0.2)],
        directed: true,
        inverse_indexed: false,
        seed: Some(1),
    })
    .unwrap();
    let graph = store.graph();

    let train_cfg = crate::algo::embeddings::graphsage::types::GraphSageTrainConfig {
        model_user: "alice".to_string(),
        model_name: "m1".to_string(),
        concurrency: crate::concurrency::Concurrency::of(1),
        batch_size: 10,
        max_iterations: 2,
        search_depth: 2,
        epochs: 1,
        learning_rate: 0.01,
        tolerance: 0.0,
        negative_sample_weight: 1,
        penalty_l2: 0.001,
        embedding_dimension: 8,
        sample_sizes: vec![5],
        feature_properties: vec!["random_score".to_string()],
        maybe_batch_sampling_ratio: None,
        random_seed: Some(42),
        aggregator: crate::algo::embeddings::graphsage::types::AggregatorType::Mean,
        activation_function:
            crate::algo::embeddings::graphsage::types::ActivationFunctionType::Relu,
        is_multi_label: true,
        projected_feature_dimension: Some(4),
    };

    let train = GraphSageTrainAlgorithmFactory::new("1.0.0".to_string()).build(
        crate::types::graph::Graph::concurrent_copy(graph.as_ref()),
        train_cfg.clone(),
        TaskProgressTracker::new(Tasks::leaf_with_volume("GraphSageTrain".to_string(), 1)),
        crate::concurrency::TerminationFlag::default(),
    );
    let model = train.compute();

    let catalog = InMemoryModelCatalog::new();
    catalog.set(model).unwrap();

    let factory = GraphSageAlgorithmFactory::new(std::sync::Arc::new(catalog));
    let algo = factory.build(
        crate::types::graph::Graph::concurrent_copy(graph.as_ref()),
        "alice",
        "m1",
        crate::concurrency::Concurrency::of(1),
        10,
        TaskProgressTracker::new(Tasks::leaf_with_volume("GraphSage".to_string(), 1)),
    );

    let res = algo.compute();
    assert_eq!(res.embeddings.size(), 50);
    assert_eq!(res.embeddings.get(0).len(), 8);
}

#[test]
fn graphsage_memory_estimation_smoke() {
    let dims = ConcreteGraphDimensions::of(1000, 5000);
    let cfg = crate::algo::embeddings::graphsage::types::GraphSageTrainConfig {
        model_user: "alice".to_string(),
        model_name: "m1".to_string(),
        concurrency: crate::concurrency::Concurrency::of(4),
        batch_size: 100,
        max_iterations: 10,
        search_depth: 2,
        epochs: 1,
        learning_rate: 0.01,
        tolerance: 0.0,
        negative_sample_weight: 1,
        penalty_l2: 0.001,
        embedding_dimension: 128,
        sample_sizes: vec![25, 10],
        feature_properties: vec!["f".to_string()],
        maybe_batch_sampling_ratio: None,
        random_seed: Some(42),
        aggregator: crate::algo::embeddings::graphsage::types::AggregatorType::Mean,
        activation_function:
            crate::algo::embeddings::graphsage::types::ActivationFunctionType::Relu,
        is_multi_label: false,
        projected_feature_dimension: None,
    };
    let est = GraphSageMemoryEstimateDefinition::new(cfg, false);
    let tree = est.estimate(&dims, 4);
    assert!(!tree.description().is_empty());
    assert!(tree.memory_usage().max() > 0);
}
