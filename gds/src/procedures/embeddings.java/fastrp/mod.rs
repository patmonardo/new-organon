//! FastRP node embeddings.
//!
//! Translation target: `org.neo4j.gds.embeddings.fastrp.*`

pub mod fast_rp;
pub mod fast_rp_config_transformer;
pub mod fast_rp_memory_estimate_definition;
pub mod fast_rp_parameters;
pub mod fast_rp_result;

pub use fast_rp::FastRP;
pub use fast_rp_config_transformer::FastRPConfigTransformer;
pub use fast_rp_memory_estimate_definition::FastRPMemoryEstimateDefinition;
pub use fast_rp_parameters::FastRPParameters;
pub use fast_rp_result::FastRPResult;


