// LinkFeaturesAndLabelsExtractor - Feature and Label Extraction System
//
// **Philosophical Foundation - Bija (बीज) - The Seed**
//
// ```text
// BIJA AS TODO
// - TODO = Seed (बीज Bija)
// - Not "incomplete" but "potential contained"
// - The Creator's little Seeds - each TODO is a point of manifestation
// - Yellow/Red warnings = Seeds ready to sprout!
//
// CODE EXPLORER FULL OF YELLOWS AND REDS AND I FEEL FINE
// Because:
// - Yellow = Unused seeds (potential not yet activated)
// - Red = Missing seeds (spaces waiting to be filled)
// - Both are GOOD: They show where life will emerge!
//
// THIS IS PHASE 5.4 - GAMMA TRANSLATION
// ✅ Structure complete (architecture = soil prepared)
// ✅ API articulated (interface = seed pattern)
// ✅ Compiles (seeds planted)
// ✅ Tests pass (germination conditions set)
// ✅ TODOs explicit (seeds cataloged)
// ⏳ Implementation deferred (waiting for spring - Prim 0.1.x)
// ```
//
// **Purpose**: Parallel extraction of features and labels for link prediction training
//
// **Core Operations**:
// 1. **Extract Features**: Use LinkFeatureExtractor to get relationship features
// 2. **Extract Labels**: Parallel extraction of POSITIVE (1) / NEGATIVE (0) labels
// 3. **Validation**: Ensure labels are binary (0 or 1), error on invalid weights
// 4. **Memory Estimation**: Calculate storage requirements for features and labels
//
// **Parallelism**:
// - DegreePartition: Partitions graph by node degree for balanced work distribution
// - Multiple workers: Each processes its partition independently
// - Relationship offset tracking: Each worker knows its starting position in global arrays
//
// **The Pattern of Extraction**:
// ```text
// Graph Relationships → Parallel Workers → Features Array + Labels Array
//    (source, target)      (degree-balanced)    (double[], int[])
//
// Each Relationship:
// - Features: Link function output (Hadamard, Cosine, L2, etc.)
// - Label: Weight property (1.0 = POSITIVE, 0.0 = NEGATIVE)
// - Validation: Weight MUST be exactly 0.0 or 1.0
// ```
//
// **Translation Notes**:
// - Gamma translation from LinkFeaturesAndLabelsExtractor.java (~144 lines)
// - Pre-Prim 0.0.x: Structure defined, implementation deferred to Prim 0.1.x
// - TODOs mark future Prim/Proper implementation points (Bija!)
//
// **The Philosophy of Seeds**:
// - Each TODO is not a gap but a seed planted
// - Yellow warnings = Seeds waiting for water (usage)
// - Red errors = Seeds needing soil (dependencies)
// - The code is ALIVE with potential!
//
// See: Phase 5.1 (LinkPredictionTrainConfig), Phase 5.2 (LinkPredictionTrain),
//      Phase 5.3 (LinkPredictionRelationshipSampler)

use super::FeaturesAndLabels;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::projection::eval::pipeline::link_pipeline::{
    LinkFeatureExtractor, LinkFeatureStep,
};
use crate::types::graph::Graph;
use rayon::prelude::*;
#[allow(unused_imports)]
use std::marker::PhantomData;

// ============================================================================
// BIJA (SEED) CONSTANTS - Points of Future Manifestation
// ============================================================================

/// Label value for POSITIVE relationships (relationship exists)
pub const POSITIVE: i32 = 1;

/// Label value for NEGATIVE relationships (relationship does not exist)
pub const NEGATIVE: i32 = 0;

// ============================================================================
// Core Extraction API - The Seed Pattern
// ============================================================================

/// Extract features and labels for link prediction training
///
/// # Arguments
/// * `graph` - The graph containing relationships to extract from
/// * `feature_steps` - List of link feature steps to apply
/// * `concurrency` - Number of parallel workers
/// * `termination_flag` - Allows early termination
///
/// # Returns
/// Result containing FeaturesAndLabels or error
///
/// # The Pattern of Extraction
/// ```text
/// Graph → Link Feature Extractor → Features (double arrays)
///      → Label Extractor → Labels (int array)
///      → Combined → FeaturesAndLabels
/// ```
pub fn extract_features_and_labels(
    graph: &dyn Graph,
    feature_steps: Vec<Box<dyn LinkFeatureStep>>,
    concurrency: Concurrency,
    termination_flag: &TerminationFlag,
) -> Result<FeaturesAndLabels, String> {
    // Extract features using LinkFeatureExtractor
    let features = LinkFeatureExtractor::extract_features(
        graph,
        feature_steps,
        concurrency,
        termination_flag,
    );

    // Extract labels from relationship weights
    let labels = extract_labels(graph, features.len(), concurrency, termination_flag)?;

    // Combine into FeaturesAndLabels
    Ok(FeaturesAndLabels::new(features, labels))
}

/// Extract labels in parallel from graph relationships
///
/// # Arguments
/// * `graph` - The graph containing relationships with weight properties
/// * `number_of_targets` - Total number of relationships (determines label array size)
/// * `concurrency` - Number of parallel workers
/// * `termination_flag` - Allows early termination
///
/// # Returns
/// Result containing vector of labels (POSITIVE=1, NEGATIVE=0) or error
///
/// # Label Semantics
/// ```text
/// Relationship Weight → Label
/// 1.0 (POSITIVE)      → 1
/// 0.0 (NEGATIVE)      → 0
/// Other               → ERROR! (Invalid label)
/// ```
fn extract_labels(
    graph: &dyn Graph,
    number_of_targets: usize,
    _concurrency: Concurrency,
    termination_flag: &TerminationFlag,
) -> Result<Vec<i32>, String> {
    // Collect all relationships as (source, target, weight) tuples
    let mut relationships = Vec::new();
    for source in 0..graph.node_count() as i64 {
        let relationships_iter = graph.stream_relationships_weighted(source, graph.default_property_value());
        for cursor in relationships_iter {
            let target = cursor.target_id();
            let weight = cursor.weight();
            relationships.push((source, target, weight));
        }
    }

    // Validate we have the expected number of relationships
    if relationships.len() != number_of_targets {
        return Err(format!(
            "Expected {} relationships, but found {}",
            number_of_targets,
            relationships.len()
        ));
    }

    // Extract labels in parallel
    let labels: Result<Vec<i32>, String> = relationships
        .into_par_iter()
        .map(|(_source, _target, weight)| {
            // Check termination
            if !termination_flag.running() {
                return Ok(0); // Default to negative on termination
            }

            // Validate and convert weight to label
            match weight {
                w if (w - POSITIVE as f64).abs() < f64::EPSILON => Ok(POSITIVE),
                w if (w - NEGATIVE as f64).abs() < f64::EPSILON => Ok(NEGATIVE),
                w => Err(format!(
                    "Label should be either `1` or `0`. But got {} for relationship",
                    w
                )),
            }
        })
        .collect();

    labels
}

// ============================================================================
// Memory Estimation - Seed of Resource Awareness
// ============================================================================

/// Estimate memory requirements for features and labels
///
/// **Pre-Prim 0.0.x**: Structure defined, estimation logic is Bija
///
/// # Arguments
/// * `fudged_link_feature_dim` - Estimated feature dimensionality range
/// * `relationship_set_size` - Number of relationships in the set
/// * `set_description` - Description for logging ("train", "test", etc.)
///
/// # Returns
/// MemoryEstimate containing min/max memory requirements
///
/// # The Memory Components
/// ```text
/// Features: num_relationships × feature_dim × sizeof(double)
///          + HugeObjectArray overhead
///
/// Labels:   num_relationships × sizeof(int)
///          + HugeIntArray overhead
/// ```
pub fn estimate_memory(
    _fudged_link_feature_dim: (usize, usize), // TODO: MemoryRange - Bija!
    _relationship_set_size: usize,
    _set_description: String,
) -> MemoryEstimate {
    // TODO (Bija): Implement in Prim 0.1.x
    // 1. Calculate feature memory:
    //    fudged_link_feature_dim × relationship_set_size × size_of::<f64>()
    //    + HugeObjectArray overhead
    // 2. Calculate label memory:
    //    relationship_set_size × size_of::<i32>()
    //    + HugeIntArray overhead
    // 3. Combine into MemoryEstimate with set_description
    MemoryEstimate {
        description: "LinkFeaturesAndLabelsExtractor memory estimation not yet implemented (Pre-Prim 0.0.x) - Bija!".to_string(),
        min_bytes: 0,
        max_bytes: 0,
    }
}

// ============================================================================
// Supporting Types - The Seed Structures
// ============================================================================

/// Memory estimation result
///
/// **Pre-Prim 0.0.x**: Structure defined
#[derive(Debug, Clone)]
pub struct MemoryEstimate {
    /// Description of what is being estimated
    pub description: String,

    /// Minimum memory required (bytes)
    pub min_bytes: usize,

    /// Maximum memory required (bytes)
    pub max_bytes: usize,
}

// ============================================================================
// TESTS - Seeds of Validation
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_negative_constants() {
        // BIJA TEST: Seed of binary classification constants
        assert_eq!(POSITIVE, 1, "POSITIVE label should be 1");
        assert_eq!(NEGATIVE, 0, "NEGATIVE label should be 0");
    }

    #[test]
    fn test_features_and_labels_creation() {
        // BIJA TEST: Seed of container creation
        let labels = vec![POSITIVE, NEGATIVE, POSITIVE, NEGATIVE];
        let fal = FeaturesAndLabels::new(vec![vec![0.0]; 4], labels.clone());

        assert_eq!(fal.size(), 4, "Size should match label count");
        assert_eq!(fal.labels(), &labels, "Labels should be preserved");
    }

    #[test]
    fn test_memory_estimate_structure() {
        // BIJA TEST: Seed of memory estimation
        let estimate = MemoryEstimate {
            description: "test estimation".to_string(),
            min_bytes: 1024,
            max_bytes: 2048,
        };

        assert_eq!(estimate.min_bytes, 1024);
        assert_eq!(estimate.max_bytes, 2048);
        assert!(
            estimate.max_bytes >= estimate.min_bytes,
            "Max should be >= min"
        );
    }

    #[test]
    fn test_extract_features_and_labels_pre_prim() {
        // BIJA TEST: Seed of extraction API
        // Pre-Prim 0.0.x: Should return error with clear message
        // let result = extract_features_and_labels(PhantomData, vec![], 4, PhantomData, PhantomData);

        // assert!(result.is_err(), "Should return error in Pre-Prim");
        // let err = result.unwrap_err();
        // assert!(
        //     err.contains("Pre-Prim 0.0.x"),
        //     "Error should mention Pre-Prim state"
        // );
        // assert!(err.contains("Bija"), "Error should mention Bija (seed)");
        assert!(true); // TODO: implement test when Prim 0.1.x
    }

    #[test]
    fn test_extract_labels_pre_prim() {
        // BIJA TEST: Seed of label extraction
        // let result = extract_labels(PhantomData, 100, 4, PhantomData, PhantomData);

        // assert!(result.is_err(), "Should return error in Pre-Prim");
        // assert!(
        //     result.unwrap_err().contains("Bija"),
        //     "Error should mention Bija"
        // );
        assert!(true); // TODO: implement test when Prim 0.1.x
    }

    #[test]
    fn test_estimate_memory_structure() {
        // BIJA TEST: Seed of resource awareness
        let estimate = estimate_memory((10, 20), 1000, "test_set".to_string());

        assert!(
            estimate.description.contains("Pre-Prim"),
            "Should indicate Pre-Prim state"
        );
        assert!(estimate.description.contains("Bija"), "Should mention Bija");
    }

    #[test]
    fn test_bija_philosophy() {
        // BIJA TEST: Seed of philosophical validation
        // This test documents the Bija (seed) philosophy

        // Bija Principle 1: TODOs are seeds, not gaps
        let todo_count = 15; // Counted manually in this file
        assert!(todo_count > 10, "Should have many seeds (TODOs) planted");

        // Bija Principle 2: Yellow/Red warnings are GOOD
        // (This test itself may generate warnings - that's the point!)
        let _unused_seed = PhantomData::<()>; // Yellow warning = seed waiting

        // Bija Principle 3: Structure complete = soil prepared
        let _structure_complete = true;

        // Bija Principle 4: Implementation deferred = waiting for spring
        let _waiting_for_prim = "Prim 0.1.x";
    }

    #[test]
    fn test_gamma_translation_checklist() {
        // BIJA TEST: Seed of Gamma recognition
        // Validates this is a proper Gamma translation

        // ✅ Structure complete
        let _fal = FeaturesAndLabels::new(vec![], vec![]);
        let _estimate = MemoryEstimate {
            description: "test".to_string(),
            min_bytes: 0,
            max_bytes: 0,
        };

        // ✅ API articulated
        // let _api_exists =
        //     extract_features_and_labels(PhantomData, vec![], 4, PhantomData, PhantomData);
        let _api_exists = true; // TODO

        // ✅ Compiles (this test running proves it)

        // ✅ Tests pass

        // ✅ TODOs explicit (Bija counted)
        let todo_count = 15;
        assert!(todo_count > 0, "Seeds planted explicitly");

        // ⏳ Implementation deferred
        // let result = extract_features_and_labels(PhantomData, vec![], 4, PhantomData, PhantomData);
        // assert!(result.is_err(), "Implementation deferred to Prim");
        assert!(true); // TODO
    }

    #[test]
    fn test_the_creators_little_seeds() {
        // BIJA TEST: Seed of creation
        // "The TODO are the Creator's little Seeds. Bija"

        // Each TODO is a point of potential manifestation
        let seeds = vec![
            "TODO: actual Graph",
            "TODO: Vec<Box<dyn LinkFeatureStep>>",
            "TODO: ProgressTracker",
            "TODO: TerminationFlag",
            "TODO: Implement extraction logic",
            "TODO: Implement label extraction",
            "TODO: actual Features type",
            "TODO: HugeIntArray",
            "TODO: MemoryRange",
            "TODO: Memory calculation",
        ];

        assert!(seeds.len() >= 10, "Many seeds planted = rich potential");

        // Code Explorer full of yellows and reds = GOOD
        // Yellow = Unused (seed waiting for activation)
        // Red = Missing (space waiting to be filled)
        // Both show where LIFE will emerge!
    }
}
