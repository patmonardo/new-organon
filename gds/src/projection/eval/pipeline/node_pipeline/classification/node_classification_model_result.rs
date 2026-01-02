use super::node_classification_train_result::TrainingStatistics;

// Placeholder types until ml-models and model catalog are translated
pub type Classifier = ();
pub type ClassifierData = ();
pub type CatalogModel = ();
pub type NodeClassificationPipelineTrainConfig = ();
pub type NodeClassificationPipelineModelInfo = ();

/// Result of node classification model creation containing the catalog model and training statistics.
///
/// This is a value class that wraps the model catalog entry with training statistics.
#[derive(Debug, Clone)]
pub struct NodeClassificationModelResult {
    // Note: When model catalog is implemented, this should become a typed catalog entry, e.g.
    // model: Model<ClassifierData, NodeClassificationPipelineTrainConfig, NodeClassificationPipelineModelInfo>
    catalog_model: CatalogModel,
    training_statistics: TrainingStatistics,
}

impl NodeClassificationModelResult {
    pub fn new(catalog_model: CatalogModel, training_statistics: TrainingStatistics) -> Self {
        Self {
            catalog_model,
            training_statistics,
        }
    }

    pub fn catalog_model(&self) -> &CatalogModel {
        &self.catalog_model
    }

    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model_result() {
        let catalog_model = ();
        let training_statistics = ();

        let result = NodeClassificationModelResult::new(catalog_model, training_statistics);

        // Verify accessors work
        let _model = result.catalog_model();
        let _stats = result.training_statistics();
    }

    #[test]
    fn test_accessors() {
        let catalog_model = ();
        let training_statistics = ();

        let result = NodeClassificationModelResult::new(catalog_model, training_statistics);

        // Verify both accessors return references
        assert!(std::ptr::eq(result.catalog_model(), &result.catalog_model));
        assert!(std::ptr::eq(
            result.training_statistics(),
            &result.training_statistics
        ));
    }
}
