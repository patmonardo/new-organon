use crate::collections::{long_multiset::LongMultiSet, HugeIntArray};
use crate::ml::core::subgraph::LocalIdMap;
use crate::types::properties::node::NodePropertyValues;

/// Result of extracting labels and class counts from target node property.
#[derive(Debug, Clone)]
pub struct LabelsAndClassCounts {
    labels: HugeIntArray,
    class_counts: LongMultiSet,
}

impl LabelsAndClassCounts {
    pub fn new(labels: HugeIntArray, class_counts: LongMultiSet) -> Self {
        Self {
            labels,
            class_counts,
        }
    }

    pub fn labels(&self) -> &HugeIntArray {
        &self.labels
    }

    pub fn class_counts(&self) -> &LongMultiSet {
        &self.class_counts
    }
}

/// Utility for extracting labels and class counts from target node properties.
///
/// This is a stateless utility with private constructor (module-level functions in Rust).
pub struct LabelsAndClassCountsExtractor;

impl LabelsAndClassCountsExtractor {
    /// Extract labels and class counts from target node property.
    ///
    /// This creates:
    /// 1. A HugeIntArray of labels (mapped from original class IDs to local consecutive IDs)
    /// 2. A LongMultiSet of class counts (how many nodes per class)
    /// 3. A LocalIdMap for bidirectional mapping between original and local class IDs
    pub fn extract_labels_and_class_counts(
        target_node_property: &dyn NodePropertyValues,
        node_count: u64,
    ) -> LabelsAndClassCounts {
        let class_counts = Self::extract_class_counts(target_node_property, node_count);

        // Get unique class IDs and sort them for deterministic mapping
        let class_ids_i64 = class_counts.keys();
        let mut class_ids: Vec<u64> = class_ids_i64
            .into_iter()
            .map(|id| {
                assert!(id >= 0, "Class IDs must be non-negative, got {}", id);
                id as u64
            })
            .collect();
        class_ids.sort_unstable();

        // Create local ID mapping
        let mut local_id_map = LocalIdMap::of_sorted(&class_ids);

        // Create labels array
        let mut labels = HugeIntArray::new(node_count as usize);

        // Map each node's class ID to local ID
        for node_id in 0..node_count {
            let class_id = target_node_property
                .long_value(node_id as u64)
                .expect("Failed to get long value for node property")
                as u64;
            let mapped_id = local_id_map.to_mapped(class_id) as i32;
            labels.set(node_id as usize, mapped_id);
        }

        LabelsAndClassCounts::new(labels, class_counts)
    }

    /// Extract class counts from target node property.
    ///
    /// Returns a multiset (map from class ID to count of nodes with that class).
    pub fn extract_class_counts(
        target_node_property: &dyn NodePropertyValues,
        node_count: u64,
    ) -> LongMultiSet {
        let mut class_counts = LongMultiSet::new();
        for node_id in 0..node_count {
            let class_id = target_node_property
                .long_value(node_id)
                .expect("Failed to get long value for node property");
            class_counts.add(class_id);
        }
        class_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecLong;
    use crate::types::properties::node::DefaultLongNodePropertyValues;

    #[test]
    fn test_labels_and_class_counts_new() {
        let labels = HugeIntArray::from_vec(vec![0, 1, 0, 2]);
        let mut class_counts = LongMultiSet::new();
        class_counts.add(0);
        class_counts.add(0);
        class_counts.add(1);
        class_counts.add(2);

        let result = LabelsAndClassCounts::new(labels.clone(), class_counts.clone());

        assert_eq!(result.labels().get(0), 0);
        assert_eq!(result.labels().get(1), 1);
        assert_eq!(result.labels().get(2), 0);
        assert_eq!(result.labels().get(3), 2);
        assert_eq!(result.class_counts().count(0), 2);
        assert_eq!(result.class_counts().count(1), 1);
        assert_eq!(result.class_counts().count(2), 1);
    }

    #[test]
    fn test_extract_class_counts() {
        let backend = VecLong::from(vec![0, 1, 0, 2, 0]);
        let target_property = DefaultLongNodePropertyValues::from_collection(backend, 5);
        let node_count = 5;

        let class_counts =
            LabelsAndClassCountsExtractor::extract_class_counts(&target_property, node_count);

        // Should have 3 classes: 0 (3 times), 1 (1 time), 2 (1 time)
        assert_eq!(class_counts.count(0), 3);
        assert_eq!(class_counts.count(1), 1);
        assert_eq!(class_counts.count(2), 1);
        assert_eq!(class_counts.size(), 3);
    }

    #[test]
    fn test_extract_labels_and_class_counts() {
        let backend = VecLong::from(vec![10, 5, 10, 15]);
        let target_property = DefaultLongNodePropertyValues::from_collection(backend, 4);
        let node_count = 4;

        let result = LabelsAndClassCountsExtractor::extract_labels_and_class_counts(
            &target_property,
            node_count,
        );

        // Classes should be mapped: 5->0, 10->1, 15->2
        assert_eq!(result.labels().get(0), 1); // 10 -> 1
        assert_eq!(result.labels().get(1), 0); // 5 -> 0
        assert_eq!(result.labels().get(2), 1); // 10 -> 1
        assert_eq!(result.labels().get(3), 2); // 15 -> 2

        // Class counts: 5 appears 1 time, 10 appears 2 times, 15 appears 1 time
        assert_eq!(result.class_counts().count(5), 1);
        assert_eq!(result.class_counts().count(10), 2);
        assert_eq!(result.class_counts().count(15), 1);
    }

    #[test]
    fn test_labels_and_class_counts_accessors() {
        let labels = HugeIntArray::from_vec(vec![0, 1, 2]);
        let mut class_counts = LongMultiSet::new();
        class_counts.add(0);
        class_counts.add(1);
        class_counts.add(2);

        let result = LabelsAndClassCounts::new(labels.clone(), class_counts.clone());

        // Verify accessors return correct references
        assert_eq!(result.labels().get(0), 0);
        assert_eq!(result.class_counts().size(), 3);
        assert_eq!(result.class_counts().count(1), 1);
    }
}
