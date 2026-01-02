//! Machine Learning procedure facades.

pub mod facade;
pub mod kge;

pub use facade::{AnyMap, LocalMachineLearningProcedureFacade, RequestScopedDependencies};
pub use kge::{KgePredictFacade, KgePredictStats, KgeStreamResult, ScoreFunction};
