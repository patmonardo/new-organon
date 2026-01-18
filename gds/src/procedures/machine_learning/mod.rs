//! Machine Learning procedure facades.

pub mod facade;
pub mod kge;
pub mod split_relationships;

pub use facade::{AnyMap, LocalMachineLearningProcedureFacade, RequestScopedDependencies};
pub use kge::{KgePredictFacade, KgePredictStats, KgeStreamResult, ScoreFunction};
pub use split_relationships::{SplitRelationshipsFacade, SplitRelationshipsStats};
