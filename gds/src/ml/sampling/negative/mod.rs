//! Negative sampling for machine learning.
//!
//! 1:1 translation of org.neo4j.gds.ml.negativeSampling package from Java GDS.

mod factory;
mod negative_sampler;
mod random;
mod user_input;

pub use factory::create_sampler;
pub use negative_sampler::{NegativeSampler, NEGATIVE};
pub use random::RandomNegativeSampler;
pub use user_input::UserInputNegativeSampler;
