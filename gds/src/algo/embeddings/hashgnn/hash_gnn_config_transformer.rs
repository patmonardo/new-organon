use super::hash_gnn_parameters::HashGNNParameters;

/// Mirrors Java's `HashGNNConfigTransformer`.
///
/// Rust GDS typically builds parameters directly; this keeps the Java translation surface.
#[allow(dead_code)]
pub struct HashGNNConfigTransformer;

#[allow(dead_code)]
impl HashGNNConfigTransformer {
    pub fn to_parameters(config: HashGNNParameters) -> HashGNNParameters {
        config
    }
}
