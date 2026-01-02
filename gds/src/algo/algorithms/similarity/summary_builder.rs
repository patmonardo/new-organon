//! Similarity summary builders
//!
//! **Translation Sources**:
//! - `org.neo4j.gds.algorithms.similarity.SimilaritySummaryBuilder`
//! - `ActualSimilaritySummaryBuilder`
//! - `EmptySimilaritySummaryBuilder`
//!
//! Provides streaming histogram collection for similarity scores, delegating to
//! the core similarity statistics for summary generation.

use hdrhistogram::Histogram;
use std::collections::HashMap;

/// Default histogram precision (matches Java `HISTOGRAM_PRECISION_DEFAULT`)
const HISTOGRAM_PRECISION: u8 = 5;
/// Scaling factor to convert f64 to u64 for histogram (5 decimal places)
const SCALE_FACTOR: f64 = 100_000.0;

/// Builder for similarity summaries (histogram + percentiles)
pub enum SimilaritySummaryBuilder {
    /// Collect histogram; may fall back to error on bounds overflow
    Actual {
        histogram: Histogram<u64>,
        crashed: bool,
    },
    /// No-op collector
    Empty,
}

impl SimilaritySummaryBuilder {
    /// Create a builder; disable collection when `should_compute` is false
    pub fn new(should_compute: bool) -> Self {
        if should_compute {
            let histogram = Histogram::new_with_max(SCALE_FACTOR as u64, HISTOGRAM_PRECISION)
                .expect("failed to create similarity histogram");
            Self::Actual {
                histogram,
                crashed: false,
            }
        } else {
            Self::Empty
        }
    }

    /// Consume a similarity score; returns `true` to keep streaming
    pub fn consume(&mut self, similarity: f64) -> bool {
        match self {
            SimilaritySummaryBuilder::Empty => true,
            SimilaritySummaryBuilder::Actual { histogram, crashed } => {
                if *crashed {
                    return true;
                }
                if similarity.is_nan() {
                    return true;
                }
                let scaled = (similarity * SCALE_FACTOR) as u64;
                if let Err(e) = histogram.record(scaled) {
                    if e.to_string().contains("out of bounds") {
                        *crashed = true;
                        return true;
                    }
                    // Unexpected failure: propagate by marking crashed
                    *crashed = true;
                }
                true
            }
        }
    }

    /// Final summary map (percentiles) using core stats semantics
    pub fn summary(&self) -> HashMap<String, f64> {
        match self {
            SimilaritySummaryBuilder::Empty => HashMap::new(),
            SimilaritySummaryBuilder::Actual { histogram, crashed } => {
                if *crashed {
                    let mut map = HashMap::new();
                    map.insert("error".to_string(), 1.0);
                    return map;
                }
                similarity_summary(histogram)
            }
        }
    }

    /// Whether histogram collection failed due to bounds
    pub fn crashed(&self) -> bool {
        match self {
            SimilaritySummaryBuilder::Actual { crashed, .. } => *crashed,
            SimilaritySummaryBuilder::Empty => false,
        }
    }
}

fn similarity_summary(histogram: &Histogram<u64>) -> HashMap<String, f64> {
    let mut summary = HashMap::new();
    summary.insert("min".to_string(), histogram.min() as f64 / SCALE_FACTOR);
    summary.insert("max".to_string(), histogram.max() as f64 / SCALE_FACTOR);
    summary.insert("mean".to_string(), histogram.mean() / SCALE_FACTOR);
    summary.insert(
        "p50".to_string(),
        histogram.value_at_quantile(0.50) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p75".to_string(),
        histogram.value_at_quantile(0.75) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p90".to_string(),
        histogram.value_at_quantile(0.90) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p95".to_string(),
        histogram.value_at_quantile(0.95) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p99".to_string(),
        histogram.value_at_quantile(0.99) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p999".to_string(),
        histogram.value_at_quantile(0.999) as f64 / SCALE_FACTOR,
    );

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_summary_when_enabled() {
        let mut builder = SimilaritySummaryBuilder::new(true);
        for v in [0.5, 0.75, 0.9, 0.95, 0.99] {
            builder.consume(v);
        }
        let summary = builder.summary();
        assert!(summary.get("mean").is_some());
        assert_eq!(summary.get("error"), None);
    }

    #[test]
    fn returns_empty_when_disabled() {
        let mut builder = SimilaritySummaryBuilder::new(false);
        builder.consume(0.8);
        assert!(builder.summary().is_empty());
    }

    #[test]
    fn marks_crash_on_bounds_error() {
        // Force an out-of-bounds error by using a huge scaled value
        let mut builder = SimilaritySummaryBuilder::new(true);
        let big = f64::MAX;
        builder.consume(big);
        assert!(builder.crashed());
        let summary = builder.summary();
        assert_eq!(summary.get("error"), Some(&1.0));
    }
}
