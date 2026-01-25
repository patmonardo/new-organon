pub mod classifier_trainer;
pub mod entropy;
pub mod feature_bagger;
pub mod gini_index;
pub mod impurity_criterion;
pub mod predictor;
pub mod regressor_trainer;
pub mod split_mse;
pub mod splitter;
pub mod tests;
pub mod trainer;
pub mod trainer_config;
pub mod tree_node;
pub mod types;

/// Helper: treat 0 as "unlimited" for max depth across tree training configs.
/// Use `is_unlimited_depth(max_depth)` for clearer checks.
pub(crate) fn is_unlimited_depth(max_depth: usize) -> bool {
    max_depth == 0
}

pub use classifier_trainer::*;
pub use entropy::*;
pub use feature_bagger::*;
pub use gini_index::*;
pub use impurity_criterion::*;
pub use predictor::*;
pub use regressor_trainer::*;
pub use split_mse::*;
pub use splitter::*;
pub use trainer::*;
pub use trainer_config::*;
pub use tree_node::*;
pub use types::*;
