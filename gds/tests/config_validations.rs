use gds::config::*;

#[test]
fn collections_config_default_validates() {
    let cfg: CollectionsConfig<i64> = CollectionsConfig::default();
    assert!(cfg.validate().is_ok());
}

#[test]
fn collections_config_invalid_cache_fails() {
    let mut cfg: CollectionsConfig<i64> = CollectionsConfig::default();
    cfg.performance.cache.size = 0;
    assert!(cfg.validate().is_err());
}

#[test]
fn graph_property_config_validate() {
    let p = PropertyConfig::builder("prop".to_string()).build().unwrap();
    assert!(p.validate().is_ok());
}

#[test]
fn file_exporter_config_validate() {
    let cfg = FileExporterConfig::default();
    assert!(cfg.validate().is_ok());
}

#[test]
fn database_exporter_invalid_name_fails() {
    let cfg = DatabaseExporterConfig::builder()
        .database_name("123bad".to_string())
        .build();
    assert!(cfg.is_err());
}

#[test]
fn arrow_projection_config_validate() {
    let cfg = gds::projection::factory::arrow::ArrowProjectionConfig::default();
    assert!(ValidatedConfig::validate(&cfg).is_ok());
}

#[test]
fn link_prediction_train_validate_and_invalid() {
    let cfg =
        gds::projection::eval::pipeline::link_pipeline::train::LinkPredictionTrainConfig::builder()
            .pipeline("p".to_string())
            .target_relationship_type("KNOWS".to_string())
            .graph_name("g".to_string())
            .username("u".to_string())
            .build()
            .unwrap();

    assert!(ValidatedConfig::validate(&cfg).is_ok());

    let bad =
        gds::projection::eval::pipeline::link_pipeline::train::LinkPredictionTrainConfig::builder()
            .pipeline("p".to_string())
            .target_relationship_type("*".to_string())
            .graph_name("g".to_string())
            .username("u".to_string())
            .build()
            .unwrap_err();
    assert!(bad.contains("'*' is not allowed") || bad.contains("'*' is not allowed"));
}

#[test]
fn link_prediction_split_validate() {
    let cfg = gds::projection::eval::pipeline::link_pipeline::LinkPredictionSplitConfig::default();
    assert!(ValidatedConfig::validate(&cfg).is_ok());

    let bad = gds::projection::eval::pipeline::link_pipeline::LinkPredictionSplitConfig::builder()
        .test_fraction(0.0)
        .build();
    assert!(bad.is_err());
}

#[test]
fn node_property_prediction_split_validates() {
    let cfg =
        gds::projection::eval::pipeline::node_pipeline::NodePropertyPredictionSplitConfig::default(
        );
    assert!(ValidatedConfig::validate(&cfg).is_ok());

    assert!(
        gds::projection::eval::pipeline::node_pipeline::NodePropertyPredictionSplitConfig::new(
            1.5, 3
        )
        .is_err()
    );
}

#[test]
fn node_classification_and_regression_validations() {
    let class_cfg = gds::projection::eval::pipeline::node_pipeline::classification::NodeClassificationPipelineTrainConfig::default();
    assert!(ValidatedConfig::validate(&class_cfg).is_ok());

    let reg_cfg = gds::projection::eval::pipeline::node_pipeline::regression::NodeRegressionPipelineTrainConfig::default();
    // regression default has empty metrics and should fail
    assert!(ValidatedConfig::validate(&reg_cfg).is_err());
}

#[test]
fn auto_tuning_and_context_property_validations() {
    let auto = gds::projection::eval::pipeline::auto_tuning_config::AutoTuningConfig::default();
    assert!(ValidatedConfig::validate(&auto).is_ok());

    let ctx = gds::projection::eval::pipeline::node_property_step_context_config::NodePropertyStepContextConfig::default();
    assert!(ValidatedConfig::validate(&ctx).is_ok());

    // PropertyConfig (importer) lives in a private module and was not given a public
    // ValidatedConfig impl in this batch; validate the graph-level PropertyConfig instead
    let prop = gds::config::PropertyConfig::builder("p".to_string())
        .build()
        .unwrap();
    assert!(prop.validate().is_ok());

    let bad_prop = gds::config::PropertyConfig::builder("".to_string()).build();
    assert!(bad_prop.is_err());
}

#[test]
fn application_config_validations() {
    // GraphStreamRelationshipsConfig has a validate closure in its define_config
    let rels = gds::applications::graph_store_catalog::GraphStreamRelationshipsConfig::default();
    assert!(ValidatedConfig::validate(&rels).is_ok());

    // SamplingConfig is permissive
    let samp = gds::applications::graph_store_catalog::SamplingConfig::default();
    assert!(ValidatedConfig::validate(&samp).is_ok());

    // GraphGenerationConfig default is ok; zero node_count is invalid
    let mut gen = gds::applications::graph_store_catalog::GraphGenerationConfig::default();
    assert!(ValidatedConfig::validate(&gen).is_ok());
    gen.node_count = Some(0);
    assert!(ValidatedConfig::validate(&gen).is_err());

    // GraphAccessGraphPropertiesConfig requires a non-empty graph_property; build a valid one
    let access = gds::applications::graph_store_catalog::GraphAccessGraphPropertiesConfig::of(
        None,
        "prop".to_string(),
    )
    .unwrap();
    assert!(ValidatedConfig::validate(&access).is_ok());

    // WriteRelationshipPropertiesConfig is a marker config (no-op)
    let write =
        gds::applications::graph_store_catalog::WriteRelationshipPropertiesConfig::of().unwrap();
    assert!(ValidatedConfig::validate(&write).is_ok());
}

#[test]
fn pagerank_louvain_node_similarity_betweenness_validations() {
    use gds::algo::pagerank::PageRankConfig;
    use gds::config::{BetweennessCentralityConfig, LouvainConfig, NodeSimilarityConfig};

    let pr = PageRankConfig::default();
    assert!(ValidatedConfig::validate(&pr).is_ok());

    let mut bad = pr.clone();
    bad.base.concurrency = 0;
    assert!(ValidatedConfig::validate(&bad).is_err());

    let l = LouvainConfig::default();
    assert!(ValidatedConfig::validate(&l).is_ok());

    let mut lb = l.clone();
    lb.gamma = 20.0;
    assert!(ValidatedConfig::validate(&lb).is_err());

    let ns = NodeSimilarityConfig::default();
    assert!(ValidatedConfig::validate(&ns).is_ok());

    let mut nsb = ns.clone();
    nsb.top_k = 0;
    assert!(ValidatedConfig::validate(&nsb).is_err());

    let b = BetweennessCentralityConfig::default();
    assert!(ValidatedConfig::validate(&b).is_ok());

    let mut bb = b.clone();
    bb.sampling_size = Some(0);
    assert!(ValidatedConfig::validate(&bb).is_err());
}

#[test]
fn kmeans_kcore_scc_wcc_validations() {
    use gds::algo::kcore::KCoreConfig;
    use gds::algo::kmeans::KMeansConfig;
    use gds::algo::scc::SccConfig;
    use gds::algo::wcc::WccConfig;

    let km = KMeansConfig::default();
    // default has node_property empty -> invalid per our validation
    assert!(ValidatedConfig::validate(&km).is_err());

    let mut k2 = km.clone();
    k2.node_property = "prop".to_string();
    assert!(ValidatedConfig::validate(&k2).is_ok());

    let mut badk = k2.clone();
    badk.k = 0;
    assert!(ValidatedConfig::validate(&badk).is_err());

    let kc = KCoreConfig::default();
    assert!(ValidatedConfig::validate(&kc).is_ok());

    let mut badkc = kc.clone();
    badkc.concurrency = 0;
    assert!(ValidatedConfig::validate(&badkc).is_err());

    let sc = SccConfig::default();
    assert!(ValidatedConfig::validate(&sc).is_ok());

    let mut badsc = sc.clone();
    badsc.concurrency = 0;
    assert!(ValidatedConfig::validate(&badsc).is_err());

    let wc = WccConfig::default();
    assert!(ValidatedConfig::validate(&wc).is_ok());

    let mut badwc = wc.clone();
    badwc.concurrency = 0;
    assert!(ValidatedConfig::validate(&badwc).is_err());
}

#[test]
fn gradient_and_trainer_config_validations() {
    use gds::ml::decision_tree::trainer_config::DecisionTreeTrainerConfig;
    use gds::ml::gradient_descent::GradientDescentConfig;
    use gds::ml::models::linear_regression::LinearRegressionTrainConfig;
    use gds::ml::models::logistic_regression::LogisticRegressionTrainConfig;
    use gds::ml::models::mlp::MLPClassifierTrainConfig;
    use gds::ml::models::random_forest::RandomForestConfig;

    let g = GradientDescentConfig::default();
    assert!(ValidatedConfig::validate(&g).is_ok());

    let mut gb = g.clone();
    // invalid learning rate
    gb = GradientDescentConfig::builder()
        .learning_rate(0.0)
        .build()
        .unwrap();
    assert!(ValidatedConfig::validate(&gb).is_err());

    let mlp = MLPClassifierTrainConfig::default();
    assert!(ValidatedConfig::validate(&mlp).is_ok());

    let mut bad_mlp = mlp.clone();
    bad_mlp.hidden_layer_sizes = vec![];
    assert!(ValidatedConfig::validate(&bad_mlp).is_err());

    let log = LogisticRegressionTrainConfig::default();
    assert!(ValidatedConfig::validate(&log).is_ok());

    let mut bad_log = log.clone();
    bad_log.learning_rate = 0.0;
    assert!(ValidatedConfig::validate(&bad_log).is_err());

    let lin = LinearRegressionTrainConfig::default();
    assert!(ValidatedConfig::validate(&lin).is_ok());

    let bad_lin = LinearRegressionTrainConfig::new(
        GradientDescentConfig::builder()
            .learning_rate(0.0)
            .build()
            .unwrap(),
        lin.penalty(),
    );
    assert!(ValidatedConfig::validate(&bad_lin).is_err());

    let rf = RandomForestConfig::default();
    assert!(ValidatedConfig::validate(&rf).is_ok());

    let mut bad_rf = rf.clone();
    bad_rf.num_decision_trees = 0;
    assert!(ValidatedConfig::validate(&bad_rf).is_err());

    // minSamplesLeaf must be strictly smaller than minSamplesSplit
    let mut bad_rf2 = rf.clone();
    bad_rf2.min_samples_leaf = 3;
    bad_rf2.min_samples_split = 3;
    assert!(ValidatedConfig::validate(&bad_rf2).is_err());

    // maxFeaturesRatio bounds
    let mut bad_rf3 = rf.clone();
    bad_rf3.max_features_ratio = Some(0.0);
    assert!(ValidatedConfig::validate(&bad_rf3).is_err());

    let mut bad_rf4 = rf.clone();
    bad_rf4.max_features_ratio = Some(1.5);
    assert!(ValidatedConfig::validate(&bad_rf4).is_err());

    // Decision tree builder enforces constraints; try a build that should fail
    let builder_err = DecisionTreeTrainerConfig::builder()
        .min_leaf_size(3)
        .min_split_size(3)
        .build();
    assert!(builder_err.is_err());

    // Decision tree validation upper bounds (builder accepts large values; ValidatedConfig should reject)
    let big_depth = DecisionTreeTrainerConfig::builder()
        .max_depth(100_000)
        .build()
        .unwrap();
    assert!(ValidatedConfig::validate(&big_depth).is_err());

    let big_leaf = DecisionTreeTrainerConfig::builder()
        .min_leaf_size(2_000_000)
        .min_split_size(2_000_001)
        .build()
        .unwrap();
    assert!(ValidatedConfig::validate(&big_leaf).is_err());

    // RandomForest numeric edge cases
    let mut rf_nan = rf.clone();
    rf_nan.num_samples_ratio = f64::NAN;
    assert!(ValidatedConfig::validate(&rf_nan).is_err());

    let mut rf_inf = rf.clone();
    rf_inf.max_features_ratio = Some(f64::INFINITY);
    assert!(ValidatedConfig::validate(&rf_inf).is_err());

    let mut rf_large = rf.clone();
    rf_large.num_decision_trees = 2_000_000; // exceeds our 1_000_000 cap
    assert!(ValidatedConfig::validate(&rf_large).is_err());

    let mut rf_bad_split = rf.clone();
    rf_bad_split.min_samples_split = 1;
    assert!(ValidatedConfig::validate(&rf_bad_split).is_err());

    // maxDepth unreasonable value should fail
    let mut rf_bad_depth = rf.clone();
    rf_bad_depth.max_depth = 100_000;
    assert!(ValidatedConfig::validate(&rf_bad_depth).is_err());

    // excessively large min_samples_leaf should fail
    let mut rf_bad_leaf = rf.clone();
    rf_bad_leaf.min_samples_leaf = 2_000_000;
    assert!(ValidatedConfig::validate(&rf_bad_leaf).is_err());

    // Trainer wrappers should propagate forest validation errors
    use gds::ml::decision_tree::ClassifierImpurityCriterionType;
    use gds::ml::models::random_forest::RandomForestClassifierTrainerConfig;
    let mut trainer = RandomForestClassifierTrainerConfig {
        forest: rf.clone(),
        criterion: ClassifierImpurityCriterionType::Gini,
    };
    trainer.forest.num_samples_ratio = f64::NAN;
    assert!(ValidatedConfig::validate(&trainer).is_err());
}

#[test]
fn graphsage_train_config_valid_and_invalid() {
    use gds::algo::embeddings::graphsage::types::{
        ActivationFunctionType, AggregatorType, GraphSageTrainConfig,
    };
    use gds::concurrency::Concurrency;

    let valid = GraphSageTrainConfig {
        model_user: "u".to_string(),
        model_name: "m".to_string(),
        concurrency: Concurrency::of(1),
        batch_size: 32,
        max_iterations: 10,
        search_depth: 1,
        epochs: 1,
        learning_rate: 0.01,
        tolerance: 0.0,
        negative_sample_weight: 1,
        penalty_l2: 0.0,
        embedding_dimension: 64,
        sample_sizes: vec![10, 5],
        feature_properties: vec![],
        maybe_batch_sampling_ratio: None,
        random_seed: None,
        aggregator: AggregatorType::Mean,
        activation_function: ActivationFunctionType::Relu,
        is_multi_label: false,
        projected_feature_dimension: None,
    };
    assert!(ValidatedConfig::validate(&valid).is_ok());

    let mut bad = valid.clone();
    bad.embedding_dimension = 0;
    assert!(ValidatedConfig::validate(&bad).is_err());

    let mut bad2 = valid.clone();
    bad2.sample_sizes = vec![10, 0];
    assert!(ValidatedConfig::validate(&bad2).is_err());
}

#[test]
fn fastrp_config_invalid_embedding_and_iteration_weights() {
    use gds::algo::embeddings::fastrp::FastRPConfig;

    let mut cfg = FastRPConfig::default();
    cfg.embedding_dimension = 0;
    assert!(ValidatedConfig::validate(&cfg).is_err());

    let mut cfg2 = FastRPConfig::default();
    cfg2.iteration_weights = vec![];
    assert!(ValidatedConfig::validate(&cfg2).is_err());
}

#[test]
fn hashgnn_config_valid_and_invalid() {
    use gds::algo::embeddings::hashgnn::algo::HashGNNConfig;

    let def = HashGNNConfig::default();
    // default has generate_features Some(..), so should be valid
    assert!(ValidatedConfig::validate(&def).is_ok());

    let mut bad = def.clone();
    bad.iterations = 0;
    assert!(ValidatedConfig::validate(&bad).is_err());

    let mut bad2 = def.clone();
    bad2.embedding_density = 0;
    assert!(ValidatedConfig::validate(&bad2).is_err());

    let mut bad3 = def.clone();
    bad3.generate_features = None;
    bad3.feature_properties = vec![]; // neither features nor generate set
    assert!(ValidatedConfig::validate(&bad3).is_err());
}

#[test]
fn gat_config_invalid_dropout_alpha() {
    use gds::algo::embeddings::gat::GATConfig;

    let mut cfg = GATConfig::default();
    cfg.dropout = -0.1;
    assert!(ValidatedConfig::validate(&cfg).is_err());

    let mut cfg2 = GATConfig::default();
    cfg2.alpha = 1.2; // >1.0
    assert!(ValidatedConfig::validate(&cfg2).is_err());
}

#[test]
fn node2vec_config_valid_and_invalid() {
    use gds::algo::embeddings::node2vec::algo::Node2VecConfig;

    let def = Node2VecConfig::default();
    assert!(ValidatedConfig::validate(&def).is_ok());

    let mut bad = def.clone();
    bad.embedding_dimension = 0;
    assert!(ValidatedConfig::validate(&bad).is_err());

    let mut bad2 = def.clone();
    bad2.iterations = 0;
    assert!(ValidatedConfig::validate(&bad2).is_err());
}
