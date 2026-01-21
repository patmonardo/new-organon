use crate::core::utils::progress::ProgressTracker;
use crate::ml::link_models::LinkPredictionResult;
use crate::ml::models::Classifier;
use crate::projection::eval::pipeline::link_pipeline::LinkFeatureExtractor;

use super::link_prediction_similarity_computer::LinkPredictionSimilarityComputer;

pub trait LinkPredictionAlgorithm {
    fn classifier(&self) -> &dyn Classifier;
    fn link_feature_extractor(&self) -> &LinkFeatureExtractor;
    fn progress_tracker(&mut self) -> &mut dyn ProgressTracker;
    fn predict_links(
        &self,
        similarity_computer: &LinkPredictionSimilarityComputer<'_>,
    ) -> Box<dyn LinkPredictionResult>;

    fn compute(&mut self) -> Box<dyn LinkPredictionResult> {
        {
            self.progress_tracker().begin_subtask();
        }

        let similarity_computer =
            LinkPredictionSimilarityComputer::new(self.link_feature_extractor(), self.classifier());
        let result = self.predict_links(&similarity_computer);

        {
            self.progress_tracker().end_subtask();
        }

        result
    }
}
