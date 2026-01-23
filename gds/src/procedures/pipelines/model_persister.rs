use crate::applications::services::logging::Log;
use crate::core::model::{Model, ModelCatalog, ModelCatalogCustomInfo, ModelConfig, ModelData};
use crate::procedures::model_catalog::ModelCatalogFacade;

pub struct ModelPersister {
    log: Log,
    model_catalog: std::sync::Arc<ModelCatalogFacade>,
}

impl ModelPersister {
    pub fn new(log: Log, model_catalog: std::sync::Arc<ModelCatalogFacade>) -> Self {
        Self { log, model_catalog }
    }

    pub fn persist_model<D, C, I>(&self, model: Model<D, C, I>, persist_to_disk: bool)
    where
        D: ModelData + 'static,
        C: ModelConfig + 'static,
        I: ModelCatalogCustomInfo + 'static,
    {
        self.model_catalog
            .set(model)
            .unwrap_or_else(|e| panic!("{e}"));
        if persist_to_disk {
            self.log
                .warn("Model persistence to disk is not yet wired in Rust pipelines");
        }
    }

    pub fn warn_unimplemented(&self, model_kind: &str) {
        self.log.warn(&format!(
            "Model persistence for {model_kind} is not yet wired in Rust pipelines",
        ));
    }
}
