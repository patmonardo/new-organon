//! Pipeline Executor flow walkthrough.
//!
//! This example is a compact, expository walkthrough of the *predict* pipeline
//! execution path. It shows how the Pipeline Executor orchestrates:
//!
//! ```text
//! ┌───────────────────────────────────────────────┐
//! │ PredictPipelineExecutor::compute()            │
//! │                                               │
//! │ 1) build node/rel filter                      │
//! │ 2) validate pipeline before execution         │
//! │ 3) create NodePropertyStepExecutor            │
//! │ 4) validate step context configs              │
//! │ 5) execute node property steps (mutate)       │
//! │ 6) validate feature properties                │
//! │ 7) execute prediction logic                   │
//! │ 8) cleanup intermediate properties            │
//! └───────────────────────────────────────────────┘
//! ```
//!
//! Run:
//!   cargo run -p gds --example pipeline_executor_flow --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\n\
Run: cargo run -p gds --example pipeline_executor_flow --features ml"
    );
}

#[cfg(feature = "ml")]
mod enabled {
    use std::collections::HashMap;
    use std::sync::Arc;

    use gds::projection::eval::pipeline::node_pipeline::NodeFeatureStep;
    use gds::projection::eval::pipeline::{
        node_property_step::PAGERANK_MUTATE, Pipeline, PipelineGraphFilter,
        PipelineValidationError, PredictPipelineExecutor, PredictPipelineExecutorError,
        MUTATE_PROPERTY_KEY,
    };
    use gds::projection::{NodeLabel, RelationshipType};
    use gds::types::graph::{RelationshipTopology, SimpleIdMap};
    use gds::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
        GraphStore,
    };
    use gds::types::schema::{Direction, MutableGraphSchema};

    fn store_from_outgoing(outgoing: Vec<Vec<i64>>) -> DefaultGraphStore {
        let node_count = outgoing.len();

        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (source, targets) in outgoing.iter().enumerate() {
            for &target in targets {
                if target >= 0 {
                    let t = target as usize;
                    if t < node_count {
                        incoming[t].push(source as i64);
                    }
                }
            }
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            gds::config::GraphStoreConfig::default(),
            GraphName::new("pipeline_graph"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

    struct FlowPipeline {
        node_property_steps:
            Vec<Box<dyn gds::projection::eval::pipeline::ExecutableNodePropertyStep>>,
        feature_steps: Vec<NodeFeatureStep>,
    }

    impl Pipeline for FlowPipeline {
        type FeatureStep = NodeFeatureStep;

        fn node_property_steps(
            &self,
        ) -> &[Box<dyn gds::projection::eval::pipeline::ExecutableNodePropertyStep>] {
            &self.node_property_steps
        }

        fn feature_steps(&self) -> &[Self::FeatureStep] {
            &self.feature_steps
        }

        fn specific_validate_before_execution(
            &self,
            _graph_store: &DefaultGraphStore,
        ) -> Result<(), PipelineValidationError> {
            Ok(())
        }

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            HashMap::new()
        }
    }

    struct FlowExecutor {
        pipeline: FlowPipeline,
        graph_store: Arc<DefaultGraphStore>,
        node_labels: Vec<String>,
        relationship_types: Vec<String>,
    }

    impl PredictPipelineExecutor<FlowPipeline, ()> for FlowExecutor {
        fn pipeline(&self) -> &FlowPipeline {
            &self.pipeline
        }

        fn pipeline_and_graph_store_mut(&mut self) -> (&FlowPipeline, &mut Arc<DefaultGraphStore>) {
            (&self.pipeline, &mut self.graph_store)
        }

        fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore> {
            &mut self.graph_store
        }

        fn graph_store(&self) -> &Arc<DefaultGraphStore> {
            &self.graph_store
        }

        fn node_labels(&self) -> &[String] {
            &self.node_labels
        }

        fn relationship_types(&self) -> &[String] {
            &self.relationship_types
        }

        fn concurrency(&self) -> usize {
            1
        }

        fn execute(&mut self) -> Result<(), PredictPipelineExecutorError> {
            if !self.graph_store().has_node_property("pagerank") {
                return Err(PredictPipelineExecutorError::ExecutionFailed(
                    "expected property 'pagerank' before cleanup".to_string(),
                ));
            }
            Ok(())
        }

        fn node_property_step_filter(&self) -> PipelineGraphFilter {
            PipelineGraphFilter::new(
                self.node_labels.clone(),
                Some(self.relationship_types.clone()),
            )
        }
    }

    pub fn main() {
        // Build a tiny 3-node graph.
        let mut store = store_from_outgoing(vec![vec![1, 2], vec![2], vec![]]);
        store
            .add_node_label(NodeLabel::of("N"))
            .expect("add node label");

        // Node-property step: PageRank.mutate -> writes 'pagerank' property.
        let mut config = HashMap::new();
        config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("pagerank".to_string()),
        );
        config.insert(
            "maxIterations".to_string(),
            serde_json::Value::Number(20.into()),
        );

        let step = gds::projection::eval::pipeline::NodePropertyStep::new(
            PAGERANK_MUTATE.to_string(),
            config,
        );

        let pipeline = FlowPipeline {
            node_property_steps: vec![Box::new(step)],
            feature_steps: vec![NodeFeatureStep::of("pagerank")],
        };

        let mut executor = FlowExecutor {
            pipeline,
            graph_store: Arc::new(store),
            node_labels: vec!["N".to_string()],
            relationship_types: vec!["REL".to_string()],
        };

        executor.compute().expect("pipeline should run");

        // After compute(), the intermediate property should be cleaned up.
        assert!(!executor.graph_store().has_node_property("pagerank"));

        println!("OK: pipeline executor flow ran end-to-end");
    }
}

#[cfg(feature = "ml")]
fn main() {
    enabled::main();
}
