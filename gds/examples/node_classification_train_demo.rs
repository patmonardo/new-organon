//! Node classification training demo (expository, minimal).
//!
//! This example builds a tiny in-memory graph store, adds a numeric feature
//! and a categorical target label, then trains a node classification pipeline.
//!
//! Run:
//!   cargo run -p gds --example node_classification_train_demo --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\n\
Run: cargo run -p gds --example node_classification_train_demo --features ml"
    );
}

#[cfg(feature = "ml")]
mod enabled {
    use std::collections::HashMap;
    use std::sync::Arc;

    use gds::collections::backends::vec::{VecDouble, VecLong};
    use gds::procedures::pipelines::{
        NodeClassificationTrainComputation, PipelineName, PipelineRepository,
    };
    use gds::projection::eval::pipeline::node_pipeline::classification::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
    use gds::projection::eval::pipeline::node_pipeline::node_feature_step::NodeFeatureStep;
    use gds::projection::eval::pipeline::{
        PipelineCatalog, TrainingMethod, TrainingPipeline, TunableTrainerConfig,
    };
    use gds::types::graph_store::{DefaultGraphStore, GraphStore};
    use gds::types::properties::node::impls::default_node_property_values::{
        DefaultDoubleNodePropertyValues, DefaultLongNodePropertyValues,
    };
    use gds::types::properties::node::NodePropertyValues;
    use gds::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use gds::types::user::User;
    use serde_json::{json, Value};

    #[derive(Clone)]
    struct DemoTrainerConfig {
        params: HashMap<String, Value>,
    }

    impl DemoTrainerConfig {
        fn new() -> Self {
            let mut params = HashMap::new();
            params.insert("methodName".to_string(), json!("LogisticRegression"));
            params.insert("penalty".to_string(), json!(0.1));
            params.insert("learningRate".to_string(), json!(0.01));
            params.insert("maxEpochs".to_string(), json!(50));
            params.insert("batchSize".to_string(), json!(4));
            Self { params }
        }
    }

    impl TunableTrainerConfig for DemoTrainerConfig {
        fn training_method(&self) -> TrainingMethod {
            TrainingMethod::LogisticRegression
        }

        fn is_concrete(&self) -> bool {
            true
        }

        fn to_map(&self) -> HashMap<String, Value> {
            self.params.clone()
        }
    }

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("node_classification_train_demo failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // --- 1) Build a tiny graph store ---
        let config = RandomGraphConfig {
            graph_name: "demo-graph".into(),
            database_name: "in-memory".into(),
            node_count: 12,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("REL", 0.2)],
            directed: false,
            inverse_indexed: false,
            seed: Some(7),
        };

        let mut store = DefaultGraphStore::random(&config)?;

        // Add a numeric feature and a categorical target label.
        let feature_values: Vec<f64> = (0..config.node_count)
            .map(|i| (i as f64) / (config.node_count as f64))
            .collect();
        let target_values: Vec<i64> = (0..config.node_count)
            .map(|i| if i < config.node_count / 2 { 0 } else { 1 })
            .collect();

        let node_count = config.node_count;
        let labels_set = store.node_labels();

        let feat_backend = VecDouble::from(feature_values);
        let feat_values =
            DefaultDoubleNodePropertyValues::from_collection(feat_backend, node_count);
        let feat_values: Arc<dyn NodePropertyValues> = Arc::new(feat_values);
        store.add_node_property(labels_set.clone(), "feat", feat_values)?;

        let target_backend = VecLong::from(target_values);
        let target_values =
            DefaultLongNodePropertyValues::from_collection(target_backend, node_count);
        let target_values: Arc<dyn NodePropertyValues> = Arc::new(target_values);
        store.add_node_property(labels_set, "target", target_values)?;

        // --- 2) Create a pipeline with a single feature step ---
        let user = User::from("demo-user");
        let pipeline_catalog = Arc::new(PipelineCatalog::new());
        let pipeline_repository = PipelineRepository::new(Arc::clone(&pipeline_catalog));
        let pipeline_name = PipelineName::parse("nc-demo")?;

        let pipeline =
            pipeline_repository.create_node_classification_training_pipeline(&user, &pipeline_name);

        let mut updated = (*pipeline).clone();
        updated.add_feature_step(NodeFeatureStep::of("feat"));
        updated.add_trainer_config(Box::new(DemoTrainerConfig::new()));
        pipeline_repository.replace(&user, &pipeline_name, Arc::new(updated));

        // --- 3) Configure and run training ---
        let train_config = NodeClassificationPipelineTrainConfig::new(
            pipeline_name.as_str().to_string(),
            vec!["N".to_string()],
            "target".to_string(),
            Some(42),
            vec![],
        );

        let computation = NodeClassificationTrainComputation::new(
            pipeline_repository.clone(),
            train_config,
            user.clone(),
        );

        let result = computation.compute(Arc::new(store))?;

        println!("OK: trained node classification model");
        let model_info_json = serde_json::to_string_pretty(&result.model_info().to_map())
            .unwrap_or_else(|_| "<failed to serialize model info>".to_string());
        let metrics_json = serde_json::to_string_pretty(&serde_json::json!(result
            .training_statistics()
            .winning_model_test_metrics()))
        .unwrap_or_else(|_| "<failed to serialize metrics>".to_string());
        println!("Model info:\n{}", model_info_json);
        println!("Train/test metrics:\n{}", metrics_json);

        Ok(())
    }
}

#[cfg(feature = "ml")]
fn main() {
    enabled::main();
}
