//! Node classification MLP training demo with explanatory printing.
//!
//! Run:
//!   cargo run -p gds --example node_classification_mlp_explanatory --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\nRun: cargo run -p gds --example node_classification_mlp_explanatory --features ml"
    );
}

#[cfg(feature = "ml")]
mod enabled {
    use std::collections::HashMap;
    use std::sync::Arc;

    use gds::collections::backends::vec::{VecDouble, VecLong};
    use gds::ml::models::base::TrainerConfigTrait;
    use gds::ml::models::mlp::MLPClassifierTrainConfig;
    use gds::procedures::pipelines::{
        NodeClassificationTrainComputation, PipelineName, PipelineRepository,
    };
    use gds::projection::eval::pipeline::{
        NodeClassificationPipelineTrainConfig, NodeFeatureStep, PipelineCatalog, TrainingMethod,
        TrainingPipeline, TunableTrainerConfig,
    };
    use gds::types::graph_store::{DefaultGraphStore, GraphStore};
    use gds::types::properties::node::DefaultDoubleNodePropertyValues;
    use gds::types::properties::node::DefaultLongNodePropertyValues;
    use gds::types::properties::node::NodePropertyValues;
    use gds::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use gds::types::user::User;
    use serde_json::Value;

    #[derive(Clone)]
    struct WrappedMLPConfig {
        cfg: MLPClassifierTrainConfig,
    }

    impl WrappedMLPConfig {
        fn new(cfg: MLPClassifierTrainConfig) -> Self {
            Self { cfg }
        }
    }

    impl TunableTrainerConfig for WrappedMLPConfig {
        fn training_method(&self) -> TrainingMethod {
            TrainingMethod::MLPClassification
        }

        fn is_concrete(&self) -> bool {
            true
        }

        fn to_map(&self) -> HashMap<String, Value> {
            self.cfg.to_map()
        }
    }

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("node_classification_mlp_explanatory failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 1) Build small random graph
        let config = RandomGraphConfig {
            graph_name: "mlp-demo-graph".into(),
            database_name: "in-memory".into(),
            node_count: 16,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("REL", 0.15)],
            directed: false,
            inverse_indexed: false,
            seed: Some(13),
        };

        let mut store = DefaultGraphStore::random(&config)?;

        // Add a simple numeric feature and binary target
        let feature_values: Vec<f64> = (0..config.node_count)
            .map(|i| ((i % 4) as f64) / 4.0)
            .collect();
        let target_values: Vec<i64> = (0..config.node_count)
            .map(|i| if i % 2 == 0 { 0 } else { 1 })
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
        store.add_node_property(labels_set.clone(), "target", target_values)?;

        // 2) Create pipeline and add a feature step
        let user = User::from("mlp-demo-user");
        let pipeline_catalog = Arc::new(PipelineCatalog::new());
        let pipeline_repository = PipelineRepository::new(Arc::clone(&pipeline_catalog));
        let pipeline_name = PipelineName::parse("nc-mlp-explain")?;

        let pipeline =
            pipeline_repository.create_node_classification_training_pipeline(&user, &pipeline_name);

        let mut updated = (*pipeline).clone();
        updated.add_feature_step(NodeFeatureStep::of("feat"));

        // Build a concrete MLP trainer config and wrap it for pipeline
        let mlp_cfg = MLPClassifierTrainConfig::builder()
            .batch_size(8)
            .max_epochs(50)
            .learning_rate(0.01)
            .hidden_layer_sizes(vec![8])
            .build()
            .unwrap();

        updated.add_trainer_config(Box::new(WrappedMLPConfig::new(mlp_cfg)));
        pipeline_repository.replace(&user, &pipeline_name, Arc::new(updated));

        // Print pipeline metadata and parameter space
        let typed =
            pipeline_repository.get_node_classification_training_pipeline(&user, &pipeline_name);
        println!("Pipeline type: {}", typed.pipeline_type());
        let param_space = typed.parameter_space_to_map();
        println!(
            "Parameter space (JSON): {}",
            serde_json::to_string_pretty(&param_space)?
        );
        println!(
            "Model selection trials: {}",
            typed.number_of_model_selection_trials()
        );

        // 3) Configure and run training (train/test split omitted for brevity)
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

        println!("OK: trained MLP node classification model");
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
