use crate::procedures::pipelines::types::PipelineCatalogResult;
use crate::projection::eval::pipeline::TrainingPipeline;

pub fn create_pipeline_catalog_result<P: TrainingPipeline>(
    pipeline: &P,
    pipeline_name: &str,
) -> PipelineCatalogResult {
    PipelineCatalogResult {
        pipeline_info: pipeline.to_map(),
        pipeline_name: pipeline_name.to_string(),
        pipeline_type: pipeline.pipeline_type().to_string(),
        creation_time: chrono::Utc::now()
            .with_timezone(&chrono::FixedOffset::east_opt(0).expect("UTC offset")),
    }
}
