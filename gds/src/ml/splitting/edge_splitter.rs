use crate::projection::factory::RelationshipsBuilder;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::IdMap;
use crate::types::graph::Graph;
use crate::types::schema::Direction;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::sync::Arc;

pub const POSITIVE: f64 = 1.0;
pub const RELATIONSHIP_PROPERTY: &str = "label";

/// Factory for building relationship builders used by edge splitters.
pub trait RelationshipsBuilderFactory: Send + Sync {
    fn create(
        &self,
        relationship_type: RelationshipType,
        orientation: Orientation,
        property_key: Option<String>,
    ) -> Box<dyn RelationshipsBuilder>;
}

/// Minimal in-memory RelationshipsBuilder for splitters and tests.
#[derive(Debug, Clone)]
pub struct InMemoryRelationshipsBuilder {
    relationship_type: RelationshipType,
    orientation: Orientation,
    property_key: Option<String>,
    sources: Vec<u64>,
    targets: Vec<u64>,
    properties: Vec<Vec<f64>>,
}

impl InMemoryRelationshipsBuilder {
    pub fn new(
        relationship_type: RelationshipType,
        orientation: Orientation,
        property_key: Option<String>,
    ) -> Self {
        Self {
            relationship_type,
            orientation,
            property_key,
            sources: Vec::new(),
            targets: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    pub fn relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn property_key(&self) -> Option<&str> {
        self.property_key.as_deref()
    }

    pub fn sources(&self) -> &[u64] {
        &self.sources
    }

    pub fn targets(&self) -> &[u64] {
        &self.targets
    }

    pub fn properties(&self) -> &[Vec<f64>] {
        &self.properties
    }
}

impl RelationshipsBuilder for InMemoryRelationshipsBuilder {
    fn add_from_internal(&mut self, source: u64, target: u64, property_value: f64) {
        self.sources.push(source);
        self.targets.push(target);
        self.properties.push(vec![property_value]);
    }

    fn add_from_internal_with_properties(
        &mut self,
        source: u64,
        target: u64,
        property_values: &[f64],
    ) {
        self.sources.push(source);
        self.targets.push(target);
        self.properties.push(property_values.to_vec());
    }

    fn add_from_internal_no_property(&mut self, source: u64, target: u64) {
        self.sources.push(source);
        self.targets.push(target);
        self.properties.push(Vec::new());
    }
}

/// Factory that builds in-memory RelationshipsBuilder instances.
#[derive(Debug, Default, Clone)]
pub struct InMemoryRelationshipsBuilderFactory;

impl RelationshipsBuilderFactory for InMemoryRelationshipsBuilderFactory {
    fn create(
        &self,
        relationship_type: RelationshipType,
        orientation: Orientation,
        property_key: Option<String>,
    ) -> Box<dyn RelationshipsBuilder> {
        Box::new(InMemoryRelationshipsBuilder::new(
            relationship_type,
            orientation,
            property_key,
        ))
    }
}

/// Result of splitting edges in a graph
pub struct SplitResult {
    pub selected_relationships: Box<dyn RelationshipsBuilder>,
    pub remaining_relationships: Box<dyn RelationshipsBuilder>,
    pub selected_rel_count: usize,
    pub remaining_rel_count: usize,
}

/// Base trait for edge splitting strategies
pub trait EdgeSplitter {
    /// Splits positive examples in the graph
    fn split_positive_examples(
        &mut self,
        graph: Arc<dyn Graph>,
        holdout_fraction: f64,
        remaining_rel_property_key: Option<String>,
        builder_factory: &dyn RelationshipsBuilderFactory,
    ) -> SplitResult;

    /// Samples based on probability
    fn sample(&mut self, probability: f64) -> bool;

    /// Counts valid positive relationship candidates
    fn valid_positive_relationship_candidate_count(
        &self,
        graph: &dyn Graph,
        is_valid_node_pair: Arc<dyn Fn(i64, i64) -> bool + Send + Sync>,
    ) -> usize;

    /// Performs positive sampling
    fn positive_sampling(
        &mut self,
        graph: &dyn Graph,
        selected_rels_builder: &mut dyn RelationshipsBuilder,
        remaining_rels_builder: &mut dyn RelationshipsBuilder,
        remaining_rel_property_key: Option<&str>,
        selected_rel_count: &mut usize,
        remaining_rel_count: &mut usize,
        node_id: i64,
        is_valid_node_pair: &dyn Fn(i64, i64) -> bool,
        positive_samples_remaining: &mut usize,
        candidate_edges_remaining: &mut usize,
    );
}

/// Base implementation for edge splitters
pub struct BaseEdgeSplitter {
    rng: StdRng,
    selected_relationship_type: RelationshipType,
    remaining_relationship_type: RelationshipType,
    source_nodes: Arc<dyn IdMap>,
    target_nodes: Arc<dyn IdMap>,
    root_nodes: Arc<dyn IdMap>,
    concurrency: usize,
}

impl BaseEdgeSplitter {
    /// Creates a new BaseEdgeSplitter
    pub fn new(
        seed: Option<u64>,
        root_nodes: Arc<dyn IdMap>,
        source_nodes: Arc<dyn IdMap>,
        target_nodes: Arc<dyn IdMap>,
        selected_relationship_type: RelationshipType,
        remaining_relationship_type: RelationshipType,
        concurrency: usize,
    ) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };

        Self {
            rng,
            selected_relationship_type,
            remaining_relationship_type,
            source_nodes,
            target_nodes,
            root_nodes,
            concurrency,
        }
    }

    /// Checks if a node pair is valid
    pub fn is_valid_node_pair(&self, graph: &dyn Graph, source: i64, target: i64) -> bool {
        let Some(source_original) = graph.to_original_node_id(source) else {
            return false;
        };
        let Some(target_original) = graph.to_original_node_id(target) else {
            return false;
        };
        self.source_nodes.contains_original_id(source_original)
            && self.target_nodes.contains_original_id(target_original)
    }

    /// Gets the source nodes
    pub fn source_nodes(&self) -> &Arc<dyn IdMap> {
        &self.source_nodes
    }

    /// Gets the target nodes
    pub fn target_nodes(&self) -> &Arc<dyn IdMap> {
        &self.target_nodes
    }

    /// Gets the root nodes
    pub fn root_nodes(&self) -> &Arc<dyn IdMap> {
        &self.root_nodes
    }

    /// Gets the selected relationship type
    pub fn selected_relationship_type(&self) -> &RelationshipType {
        &self.selected_relationship_type
    }

    /// Gets the remaining relationship type
    pub fn remaining_relationship_type(&self) -> &RelationshipType {
        &self.remaining_relationship_type
    }

    /// Gets the concurrency level
    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

pub fn split_positive_examples_with(
    splitter: &mut dyn EdgeSplitter,
    graph: Arc<dyn Graph>,
    holdout_fraction: f64,
    remaining_rel_property_key: Option<String>,
    builder_factory: &dyn RelationshipsBuilderFactory,
    selected_relationship_type: RelationshipType,
    remaining_relationship_type: RelationshipType,
    source_nodes: Arc<dyn IdMap>,
    target_nodes: Arc<dyn IdMap>,
) -> SplitResult {
    let holdout_fraction = holdout_fraction.clamp(0.0, 1.0);
    let selected_relationships = builder_factory.create(
        selected_relationship_type,
        Orientation::Natural,
        Some(RELATIONSHIP_PROPERTY.to_string()),
    );

    let remaining_orientation = match graph.schema().direction() {
        Direction::Undirected => Orientation::Undirected,
        Direction::Directed => Orientation::Natural,
    };
    let remaining_relationships = builder_factory.create(
        remaining_relationship_type,
        remaining_orientation,
        remaining_rel_property_key.clone(),
    );

    let is_valid_node_pair = {
        let graph = graph.clone();
        let source_nodes = source_nodes.clone();
        let target_nodes = target_nodes.clone();
        Arc::new(move |source: i64, target: i64| {
            let Some(source_original) = graph.to_original_node_id(source) else {
                return false;
            };
            let Some(target_original) = graph.to_original_node_id(target) else {
                return false;
            };
            source_nodes.contains_original_id(source_original)
                && target_nodes.contains_original_id(target_original)
        })
    };

    let valid_relationship_count =
        splitter.valid_positive_relationship_candidate_count(&*graph, is_valid_node_pair.clone());
    let positive_samples = (valid_relationship_count as f64 * holdout_fraction) as usize;

    let mut positive_samples_remaining = positive_samples;
    let mut candidate_edges_remaining = valid_relationship_count;
    let mut selected_rel_count = 0usize;
    let mut remaining_rel_count = 0usize;

    let mut selected_relationships = selected_relationships;
    let mut remaining_relationships = remaining_relationships;

    for node_id in graph.iter() {
        splitter.positive_sampling(
            &*graph,
            selected_relationships.as_mut(),
            remaining_relationships.as_mut(),
            remaining_rel_property_key.as_deref(),
            &mut selected_rel_count,
            &mut remaining_rel_count,
            node_id,
            &*is_valid_node_pair,
            &mut positive_samples_remaining,
            &mut candidate_edges_remaining,
        );
    }

    SplitResult {
        selected_relationships,
        remaining_relationships,
        selected_rel_count,
        remaining_rel_count,
    }
}

impl BaseEdgeSplitter {
    pub fn sample(&mut self, probability: f64) -> bool {
        let clamped = probability.clamp(0.0, 1.0);
        self.rng.gen_bool(clamped)
    }

    pub fn to_root_ids(&self, graph: &dyn Graph, source: i64, target: i64) -> Option<(u64, u64)> {
        let source_root = graph.to_root_node_id(source)?;
        let target_root = graph.to_root_node_id(target)?;
        if source_root < 0 || target_root < 0 {
            return None;
        }
        Some((source_root as u64, target_root as u64))
    }
}
