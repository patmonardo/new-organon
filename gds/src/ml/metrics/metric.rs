use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::classification::ClassificationMetric;

pub trait Metric: Send + Sync {
    fn name(&self) -> &str;
    fn comparator(&self) -> MetricComparator;

    fn as_classification_metric(&self) -> Option<&dyn ClassificationMetric> {
        None
    }

    fn is_model_specific(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricComparator {
    Natural, // Higher is better
    Inverse, // Lower is better
}

impl MetricComparator {
    pub fn compare(&self, a: f64, b: f64) -> Ordering {
        match self {
            Self::Natural => a.partial_cmp(&b).unwrap_or(Ordering::Equal),
            Self::Inverse => b.partial_cmp(&a).unwrap_or(Ordering::Equal),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationScores {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}

impl EvaluationScores {
    pub fn new(avg: f64, min: f64, max: f64) -> Self {
        Self { avg, min, max }
    }

    pub fn to_map(&self) -> serde_json::Value {
        serde_json::json!({
            "avg": self.avg,
            "min": self.min,
            "max": self.max,
        })
    }
}

pub trait MetricConsumer {
    fn consume(&mut self, metric: &dyn Metric, value: f64);
}

#[derive(Debug, Default)]
pub struct ModelStatsBuilder {
    min: HashMap<String, f64>,
    max: HashMap<String, f64>,
    sum: HashMap<String, f64>,
    num_splits: usize,
}

impl ModelStatsBuilder {
    pub fn new(num_splits: usize) -> Self {
        Self {
            min: HashMap::new(),
            max: HashMap::new(),
            sum: HashMap::new(),
            num_splits,
        }
    }

    pub fn update(&mut self, metric: &dyn Metric, value: f64) {
        let key = metric.name().to_string();
        self.min
            .entry(key.clone())
            .and_modify(|e| *e = e.min(value))
            .or_insert(value);
        self.max
            .entry(key.clone())
            .and_modify(|e| *e = e.max(value))
            .or_insert(value);
        self.sum
            .entry(key)
            .and_modify(|e| *e += value)
            .or_insert(value);
    }

    pub fn build(&self) -> HashMap<String, EvaluationScores> {
        let mut scores = HashMap::new();
        for (key, &sum) in &self.sum {
            let min = self.min[key];
            let max = self.max[key];
            let avg = sum / self.num_splits as f64;
            scores.insert(key.clone(), EvaluationScores::new(avg, min, max));
        }
        scores
    }

    pub fn build_for(&self, metric: &dyn Metric) -> Option<EvaluationScores> {
        let key = metric.name();
        if let (Some(&min), Some(&max), Some(&sum)) =
            (self.min.get(key), self.max.get(key), self.sum.get(key))
        {
            Some(EvaluationScores::new(
                sum / self.num_splits as f64,
                min,
                max,
            ))
        } else {
            None
        }
    }
}

impl MetricConsumer for ModelStatsBuilder {
    fn consume(&mut self, metric: &dyn Metric, value: f64) {
        self.update(metric, value);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCandidateStats {
    pub trainer_config: serde_json::Value,
    pub training_stats: HashMap<String, EvaluationScores>,
    pub validation_stats: HashMap<String, EvaluationScores>,
}

impl ModelCandidateStats {
    pub fn new(
        trainer_config: serde_json::Value,
        training_stats: HashMap<String, EvaluationScores>,
        validation_stats: HashMap<String, EvaluationScores>,
    ) -> Self {
        Self {
            trainer_config,
            training_stats,
            validation_stats,
        }
    }

    pub fn to_map(&self) -> serde_json::Value {
        serde_json::json!({
            "parameters": self.trainer_config,
            "metrics": self.render_metrics_with(None, None),
        })
    }

    pub fn render_metrics(&self) -> serde_json::Value {
        self.render_metrics_with(None, None)
    }

    pub fn render_metrics_with(
        &self,
        test_metrics: Option<&HashMap<String, f64>>,
        outer_train_metrics: Option<&HashMap<String, f64>>,
    ) -> serde_json::Value {
        let mut metrics = serde_json::Map::new();
        let mut keys: Vec<String> = self
            .training_stats
            .keys()
            .chain(self.validation_stats.keys())
            .cloned()
            .collect();
        keys.sort();
        keys.dedup();

        for metric in keys {
            let mut result = serde_json::Map::new();
            if let Some(scores) = self.training_stats.get(&metric) {
                result.insert("train".to_string(), scores.to_map());
            }
            if let Some(scores) = self.validation_stats.get(&metric) {
                result.insert("validation".to_string(), scores.to_map());
            }
            if let Some(test) = test_metrics.and_then(|m| m.get(&metric)) {
                result.insert("test".to_string(), json!(test));
            }
            if let Some(outer) = outer_train_metrics.and_then(|m| m.get(&metric)) {
                result.insert("outerTrain".to_string(), json!(outer));
            }

            metrics.insert(metric, serde_json::Value::Object(result));
        }

        serde_json::Value::Object(metrics)
    }
}
