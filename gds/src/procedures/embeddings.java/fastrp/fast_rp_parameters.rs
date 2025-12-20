//! FastRP parameters.
//!
//! Java: `record FastRPParameters(...)`

#[derive(Debug, Clone)]
pub struct FastRPParameters {
    pub feature_properties: Vec<String>,
    pub iteration_weights: Vec<f32>,
    pub embedding_dimension: usize,
    pub property_dimension: usize,
    pub relationship_weight_property: Option<String>,
    pub normalization_strength: f32,
    pub node_self_influence: f32,
}


