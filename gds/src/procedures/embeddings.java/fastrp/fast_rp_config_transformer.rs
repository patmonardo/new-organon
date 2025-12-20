use super::fast_rp_parameters::FastRPParameters;

/// Mirrors Java's `FastRPConfigTransformer`.
///
/// Rust usually constructs `FastRPParameters` directly, but keeping this helper
/// preserves the translated Java surface area.
pub struct FastRPConfigTransformer;

impl FastRPConfigTransformer {
    #[allow(clippy::too_many_arguments)]
    pub fn to_parameters(
        feature_properties: Vec<String>,
        iteration_weights: Vec<f32>,
        embedding_dimension: usize,
        property_dimension: usize,
        relationship_weight_property: Option<String>,
        normalization_strength: f32,
        node_self_influence: f32,
    ) -> FastRPParameters {
        FastRPParameters {
            feature_properties,
            iteration_weights,
            embedding_dimension,
            property_dimension,
            relationship_weight_property,
            normalization_strength,
            node_self_influence,
        }
    }
}


