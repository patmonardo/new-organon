use super::SplitRelationshipsEstimateParameters;
use crate::core::graph_dimensions::GraphDimensions;
use crate::mem::MemoryEstimation;
use crate::mem::MemoryEstimations;
use crate::mem::MemoryRange;
use crate::mem::MemoryTree;

/// Java: `SplitRelationshipsEstimateDefinition`.
#[derive(Debug, Clone)]
pub struct SplitRelationshipsEstimateDefinition {
    estimate_parameters: SplitRelationshipsEstimateParameters,
}

impl SplitRelationshipsEstimateDefinition {
    pub fn new(estimate_parameters: SplitRelationshipsEstimateParameters) -> Self {
        Self {
            estimate_parameters,
        }
    }
}

impl MemoryEstimation for SplitRelationshipsEstimateDefinition {
    fn description(&self) -> String {
        "Relationship splitter".to_string()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        let has_weight = self.estimate_parameters.has_relationship_weight_property;
        let holdout_fraction = self.estimate_parameters.holdout_fraction;
        let negative_sampling_ratio = self.estimate_parameters.negative_sampling_ratio;

        let bytes_per_rel = if has_weight {
            std::mem::size_of::<f64>() + 2 * std::mem::size_of::<i64>()
        } else {
            2 * std::mem::size_of::<i64>()
        };

        let builder = MemoryEstimations::builder("Relationship splitter");
        let builder = builder.range_per_graph_dimension(
            "Selected relationships",
            move |graph_dimensions, _threads| {
                let total = graph_dimensions.relationship_count() as f64;
                let positive_rel_count = total * holdout_fraction;
                let negative_rel_count = positive_rel_count * negative_sampling_ratio;
                let selected_rel_count = (positive_rel_count + negative_rel_count) as usize;
                let min = selected_rel_count / 2;
                let max = selected_rel_count;
                MemoryRange::of_range(min * bytes_per_rel, max * bytes_per_rel)
            },
        );

        let builder = builder.range_per_graph_dimension(
            "Remaining relationships",
            move |graph_dimensions, _threads| {
                let total = graph_dimensions.relationship_count() as f64;
                let remaining_rel_count = (total * (1.0 - holdout_fraction)) as usize;
                MemoryRange::of(remaining_rel_count * bytes_per_rel)
            },
        );

        builder.build().estimate(dimensions, 1)
    }
}
