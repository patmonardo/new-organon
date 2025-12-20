use super::hash_gnn_parameters::HashGNNParameters;

/// Mirrors Java's `HashGNNConfigTransformer`.
///
/// Rust GDS typically builds parameters directly; this keeps the Java translation surface.
pub struct HashGNNConfigTransformer;

impl HashGNNConfigTransformer {
    pub fn to_parameters(config: HashGNNParameters) -> HashGNNParameters {
        config
    }
}


