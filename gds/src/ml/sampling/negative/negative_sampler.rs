use crate::projection::factory::RelationshipsBuilder;

/// Constant value for negative samples
pub const NEGATIVE: f64 = 0.0;

/// Trait for negative sampling strategies.
/// 1:1 translation of NegativeSampler interface from Java GDS.
pub trait NegativeSampler: Send + Sync {
    /// Produces negative samples and adds them to the test and train set builders.
    ///
    /// # Arguments
    /// * `test_set_builder` - Builder for test set relationships
    /// * `train_set_builder` - Builder for train set relationships
    fn produce_negative_samples(
        &self,
        test_set_builder: &mut dyn RelationshipsBuilder,
        train_set_builder: &mut dyn RelationshipsBuilder,
    );
}
