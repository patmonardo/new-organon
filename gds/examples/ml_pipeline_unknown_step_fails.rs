//! Demonstrates that unknown node-property steps fail loudly.
//!
//! Run:
//!   cargo run -p gds --example ml_pipeline_unknown_step_fails --features ml

#[cfg(not(feature = "ml"))]
fn main() {
    eprintln!(
        "This example requires the `ml` feature.\n\
Run: cargo run -p gds --example ml_pipeline_unknown_step_fails --features ml"
    );
}

#[cfg(feature = "ml")]
mod enabled {
    use std::collections::HashMap;
    use std::sync::Arc;

    use gds::projection::eval::pipeline::node_pipeline::NodeFeatureStep;
    use gds::projection::eval::pipeline::{
        Pipeline, PipelineGraphFilter, PipelineValidationError, PredictPipelineExecutor,
        PredictPipelineExecutorError,
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
            GraphName::new("g"),
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

    struct SmokePipeline {
        node_property_steps:
            Vec<Box<dyn gds::projection::eval::pipeline::ExecutableNodePropertyStep>>,
        feature_steps: Vec<NodeFeatureStep>,
    }

    impl Pipeline for SmokePipeline {
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

    struct SmokeExecutor {
        pipeline: SmokePipeline,
        graph_store: Arc<DefaultGraphStore>,
        node_labels: Vec<String>,
        relationship_types: Vec<String>,
    }

    impl PredictPipelineExecutor<SmokePipeline, ()> for SmokeExecutor {
        fn pipeline(&self) -> &SmokePipeline {
            &self.pipeline
        }

        fn pipeline_and_graph_store_mut(
            &mut self,
        ) -> (&SmokePipeline, &mut Arc<DefaultGraphStore>) {
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
        let mut store = store_from_outgoing(vec![vec![1], vec![0]]);
        store
            .add_node_label(NodeLabel::of("N"))
            .expect("add node label");

        let mut config = HashMap::new();
        config.insert(
            gds::projection::eval::pipeline::MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("feat".to_string()),
        );

        let step = gds::projection::eval::pipeline::NodePropertyStep::new(
            "gds.fastRP.mutate".to_string(),
            config,
        );

        let pipeline = SmokePipeline {
            node_property_steps: vec![Box::new(step)],
            feature_steps: vec![NodeFeatureStep::of("feat")],
        };

        let mut executor = SmokeExecutor {
            pipeline,
            graph_store: Arc::new(store),
            node_labels: vec!["N".to_string()],
            relationship_types: vec!["REL".to_string()],
        };

        match executor.compute() {
            Ok(()) => {
                eprintln!("unexpected success: algorithm should be unimplemented");
                std::process::exit(1);
            }
            Err(e) => {
                println!("OK: pipeline failed loudly as expected: {e}");
            }
        }
    }
}

#[cfg(feature = "ml")]
fn main() {
    enabled::main();
}
