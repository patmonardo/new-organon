#![allow(clippy::module_inception)]

mod catalog;
mod model;
mod model_metadata;
mod types;
mod user_catalog;

pub use catalog::{EmptyModelCatalog, InMemoryModelCatalog, ModelCatalog, ModelCatalogListener};
pub use model::Model as MLModel;
pub use model_metadata::ModelMetaData;
pub use types::{CustomInfo as ModelCatalogCustomInfo, Model, ModelConfig, ModelData};
pub use user_catalog::UserCatalog;

// Constants
pub const MODEL_NAME_KEY: &str = "modelName";
pub const MODEL_TYPE_KEY: &str = "modelType";
pub const ALL_USERS: &str = "*";
pub const PUBLIC_SUFFIX: &str = "_public";
