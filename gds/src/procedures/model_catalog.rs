//! Java-shaped Model Catalog facade.
//!
//! This mirrors (a small subset of) Neo4j GDS `model-catalog-facade`.
//! For now we keep this as an in-memory, process-local catalog with per-user scoping.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::model::{
    InMemoryModelCatalog, Model, ModelCatalog, ModelCatalogCustomInfo, ModelCatalogListener,
    ModelConfig, ModelData,
};
use crate::types::user::User;

/// A loose map type for Java `Map<String, Object>`.
pub type AnyMap = HashMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCatalogResult {
    pub model_name: String,
    pub model_type: String,
    pub model_info: AnyMap,
    pub creation_time: DateTime<FixedOffset>,
    pub train_config: AnyMap,
    pub graph_schema: AnyMap,
    pub loaded: bool,
    pub stored: bool,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelExistsResult {
    pub model_name: String,
    pub model_type: String,
    pub exists: bool,
}

pub trait ModelCatalogProcedureFacade {
    const NO_VALUE: &'static str = "__NO_VALUE";

    fn drop(&self, model_name: &str, fail_if_missing: bool) -> Vec<ModelCatalogResult>;

    fn exists(&self, model_name: &str) -> Vec<ModelExistsResult>;

    fn list(&self, model_name: &str) -> Vec<ModelCatalogResult>;
}

fn validate_model_name(model_name: &str) -> &str {
    if model_name.trim().is_empty() {
        panic!("modelName must not be blank");
    }
    model_name
}

#[derive(Debug, Clone)]
struct ModelMeta {
    model_type: String,
    model_info: AnyMap,
    creation_time: DateTime<FixedOffset>,
    train_config: AnyMap,
    graph_schema: AnyMap,
    loaded: bool,
    stored: bool,
    published: bool,
}

impl ModelMeta {
    fn to_result(&self, model_name: &str) -> ModelCatalogResult {
        ModelCatalogResult {
            model_name: model_name.to_string(),
            model_type: self.model_type.clone(),
            model_info: self.model_info.clone(),
            creation_time: self.creation_time,
            train_config: self.train_config.clone(),
            graph_schema: self.graph_schema.clone(),
            loaded: self.loaded,
            stored: self.stored,
            published: self.published,
        }
    }
}

/// A model catalog implementation that stores actual models (type-erased) and keeps
/// a parallel metadata index used by the facade `list/exists/drop` results.
///
/// This lets algorithm code use the `ModelCatalog` trait, while the facade can
/// render results without downcasting models.
#[derive(Default)]
pub struct ModelCatalogFacade {
    inner: InMemoryModelCatalog,
    meta: parking_lot::RwLock<HashMap<(String, String), ModelMeta>>,
}

impl ModelCatalogFacade {
    pub fn new() -> Self {
        Self::default()
    }

    fn set_meta_for_model<D, C, I>(&self, model: &Model<D, C, I>)
    where
        D: ModelData,
        C: ModelConfig,
        I: ModelCatalogCustomInfo,
    {
        let model_name = model.name().to_string();
        let username = model.creator().to_string();

        let model_type = model.algo_type().to_string();

        let model_info = value_to_map(model.custom_info().to_map());

        let creation_time = model
            .creation_time()
            .with_timezone(&Utc)
            .with_timezone(&FixedOffset::east_opt(0).expect("UTC offset"));

        let train_config = model
            .train_config()
            .parameters()
            .into_iter()
            .collect::<AnyMap>();

        let graph_schema = value_to_map(model.graph_schema().to_map());

        // `published` mirrors Java: `!model.sharedWith().isEmpty()`.
        // We don't currently expose `shared_with` directly on the core Model;
        // `is_published` is the closest stable approximation (published-to-all).
        let meta = ModelMeta {
            model_type,
            model_info,
            creation_time,
            train_config,
            graph_schema,
            loaded: model.is_loaded(),
            stored: model.is_stored(),
            published: model.is_published(),
        };

        self.meta.write().insert((username, model_name), meta);
    }

    fn meta_for(&self, username: &str, model_name: &str) -> Option<ModelMeta> {
        self.meta
            .read()
            .get(&(username.to_string(), model_name.to_string()))
            .cloned()
    }

    fn list_meta(&self, username: &str) -> Vec<(String, ModelMeta)> {
        let mut entries: Vec<_> = self
            .meta
            .read()
            .iter()
            .filter(|((u, _), _)| u == username)
            .map(|((_, name), meta)| (name.clone(), meta.clone()))
            .collect();

        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
        entries
    }

    fn drop_meta(&self, username: &str, model_name: &str) {
        self.meta
            .write()
            .remove(&(username.to_string(), model_name.to_string()));
    }
}

fn value_to_map(value: Value) -> AnyMap {
    match value {
        Value::Object(map) => map.into_iter().collect(),
        Value::Null => AnyMap::new(),
        other => AnyMap::from([("value".to_string(), other)]),
    }
}

impl ModelCatalog for ModelCatalogFacade {
    fn register_listener(&self, listener: Box<dyn ModelCatalogListener>) {
        self.inner.register_listener(listener)
    }

    fn unregister_listener(&self, listener: &dyn ModelCatalogListener) {
        self.inner.unregister_listener(listener)
    }

    fn set<D, C, I>(&self, model: Model<D, C, I>) -> anyhow::Result<()>
    where
        D: ModelData + 'static,
        C: ModelConfig + 'static,
        I: ModelCatalogCustomInfo + 'static,
    {
        self.set_meta_for_model(&model);
        self.inner.set(model)
    }

    fn get<D, C, I>(&self, username: &str, model_name: &str) -> anyhow::Result<Arc<Model<D, C, I>>>
    where
        D: ModelData + 'static,
        C: ModelConfig + 'static,
        I: ModelCatalogCustomInfo + 'static,
    {
        self.inner.get(username, model_name)
    }

    fn get_untyped(
        &self,
        username: &str,
        model_name: &str,
    ) -> anyhow::Result<Arc<dyn std::any::Any + Send + Sync>> {
        self.inner.get_untyped(username, model_name)
    }

    fn exists(&self, username: &str, model_name: &str) -> bool {
        self.inner.exists(username, model_name)
    }

    fn drop(
        &self,
        username: &str,
        model_name: &str,
    ) -> anyhow::Result<Arc<dyn std::any::Any + Send + Sync>> {
        let dropped = self.inner.drop(username, model_name)?;
        self.drop_meta(username, model_name);
        Ok(dropped)
    }

    fn list(&self, username: &str) -> Vec<Arc<dyn std::any::Any + Send + Sync>> {
        self.inner.list(username)
    }

    fn publish(
        &self,
        username: &str,
        model_name: &str,
    ) -> anyhow::Result<Arc<dyn std::any::Any + Send + Sync>> {
        self.inner.publish(username, model_name)
    }

    fn store(
        &self,
        username: &str,
        model_name: &str,
        model_dir: &std::path::Path,
    ) -> anyhow::Result<Arc<dyn std::any::Any + Send + Sync>> {
        self.inner.store(username, model_name, model_dir)
    }

    fn model_count(&self) -> usize {
        self.inner.model_count()
    }

    fn remove_all_loaded(&self) {
        self.inner.remove_all_loaded();
        // Best-effort: keep meta aligned with inner list.
        // In-memory store() isn't implemented, so this is currently a no-op for stored models.
        self.meta.write().clear();
    }

    fn verify_model_can_be_stored(
        &self,
        username: &str,
        model_name: &str,
        model_type: &str,
    ) -> anyhow::Result<()> {
        self.inner
            .verify_model_can_be_stored(username, model_name, model_type)
    }
}

/// Local, request-scoped facade implementation.
pub struct LocalModelCatalogProcedureFacade {
    username: String,
    model_catalog: Arc<ModelCatalogFacade>,
}

fn shared_in_memory_model_catalog() -> Arc<ModelCatalogFacade> {
    static CATALOG: OnceLock<Arc<ModelCatalogFacade>> = OnceLock::new();
    Arc::clone(CATALOG.get_or_init(|| Arc::new(ModelCatalogFacade::new())))
}

impl Default for LocalModelCatalogProcedureFacade {
    fn default() -> Self {
        Self::new(User::from("anonymous"), shared_in_memory_model_catalog())
    }
}

impl LocalModelCatalogProcedureFacade {
    pub fn new(user: User, model_catalog: Arc<ModelCatalogFacade>) -> Self {
        Self {
            username: user.username().to_string(),
            model_catalog,
        }
    }

    fn exists_impl(&self, model_name: &str) -> Option<ModelMeta> {
        self.model_catalog.meta_for(&self.username, model_name)
    }
}

impl ModelCatalogProcedureFacade for LocalModelCatalogProcedureFacade {
    fn drop(&self, model_name: &str, fail_if_missing: bool) -> Vec<ModelCatalogResult> {
        let model_name = validate_model_name(model_name);

        if !self.model_catalog.exists(&self.username, model_name) {
            if fail_if_missing {
                panic!("Model not found");
            }
            return vec![];
        }

        // Match Java semantics: return the dropped model's info.
        // Our catalog drop removes metadata, so capture it first.
        let meta = self.exists_impl(model_name);

        // Drop from the underlying catalog; metadata is removed by ModelCatalogFacade::drop.
        <ModelCatalogFacade as ModelCatalog>::drop(
            self.model_catalog.as_ref(),
            &self.username,
            model_name,
        )
        .unwrap_or_else(|e| panic!("{e}"));

        meta.map(|m| vec![m.to_result(model_name)])
            .unwrap_or_default()
    }

    fn exists(&self, model_name: &str) -> Vec<ModelExistsResult> {
        let model_name = validate_model_name(model_name);

        let Some(meta) = self.exists_impl(model_name) else {
            return vec![ModelExistsResult {
                model_name: model_name.to_string(),
                model_type: "n/a".to_string(),
                exists: false,
            }];
        };

        vec![ModelExistsResult {
            model_name: model_name.to_string(),
            model_type: meta.model_type,
            exists: true,
        }]
    }

    fn list(&self, model_name: &str) -> Vec<ModelCatalogResult> {
        if model_name == Self::NO_VALUE {
            return self
                .model_catalog
                .list_meta(&self.username)
                .into_iter()
                .map(|(name, meta)| meta.to_result(&name))
                .collect();
        }

        let model_name = validate_model_name(model_name);

        self.exists_impl(model_name)
            .map(|meta| vec![meta.to_result(model_name)])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{BaseConfig, Config};
    use crate::types::schema::GraphSchema;
    use serde_json::json;

    #[derive(Debug, Clone, Serialize)]
    struct DummyData;

    impl ModelData for DummyData {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[derive(Debug, Clone, Serialize)]
    struct DummyInfo;

    impl ModelCatalogCustomInfo for DummyInfo {
        fn to_map(&self) -> Value {
            json!({"foo": "bar"})
        }
    }

    #[derive(Debug, Clone, Serialize)]
    struct DummyConfig {
        model_name: String,
        model_user: String,
    }

    impl Config for DummyConfig {}

    impl BaseConfig for DummyConfig {
        fn parameters(&self) -> AnyMap {
            AnyMap::from([
                (
                    "modelName".to_string(),
                    Value::String(self.model_name.clone()),
                ),
                (
                    "modelUser".to_string(),
                    Value::String(self.model_user.clone()),
                ),
            ])
        }
    }

    impl ModelConfig for DummyConfig {
        fn model_name(&self) -> &str {
            &self.model_name
        }

        fn model_user(&self) -> &str {
            &self.model_user
        }
    }

    #[test]
    fn model_catalog_facade_list_exists_drop() {
        let catalog = Arc::new(ModelCatalogFacade::new());
        let facade =
            LocalModelCatalogProcedureFacade::new(User::from("alice"), Arc::clone(&catalog));

        // Insert a model via the underlying ModelCatalog API.
        let model = Model::new(
            "alice".to_string(),
            "m1".to_string(),
            "DummyAlgo".to_string(),
            GraphSchema::empty(),
            Some(DummyData),
            DummyConfig {
                model_name: "m1".to_string(),
                model_user: "alice".to_string(),
            },
            "0.0-test".to_string(),
            DummyInfo,
        );

        catalog.set(model).unwrap();

        let exists = facade.exists("m1");
        assert_eq!(exists.len(), 1);
        assert!(exists[0].exists);
        assert_eq!(exists[0].model_type, "DummyAlgo");

        let listed = facade
            .list(<LocalModelCatalogProcedureFacade as ModelCatalogProcedureFacade>::NO_VALUE);
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].model_name, "m1");
        assert_eq!(listed[0].model_type, "DummyAlgo");
        assert!(listed[0].loaded);

        // Drop should return a single result when present.
        let dropped = facade.drop("m1", true);
        assert_eq!(dropped.len(), 1);
        assert_eq!(dropped[0].model_name, "m1");

        let exists_after = facade.exists("m1");
        assert_eq!(exists_after.len(), 1);
        assert!(!exists_after[0].exists);
    }
}
