use crate::ml::metrics::EvaluationScores;
use crate::ml::metrics::Metric;
use crate::ml::metrics::MetricComparator;
use crate::ml::metrics::ModelCandidateStats;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MetricSpec {
    name: String,
    comparator: MetricComparator,
}

/// Statistics collected during model training
#[derive(Debug, Clone)]
pub struct TrainingStatistics {
    model_candidate_stats: Vec<ModelCandidateStats>,
    metrics: Vec<MetricSpec>,
    test_scores: HashMap<String, f64>,
    outer_train_scores: HashMap<String, f64>,
}

impl TrainingStatistics {
    /// Creates a new TrainingStatistics with the given metrics
    pub fn new(metrics: &[Box<dyn Metric>]) -> Self {
        let metric_specs = metrics
            .iter()
            .map(|m| MetricSpec {
                name: m.name().to_string(),
                comparator: m.comparator(),
            })
            .collect();
        Self {
            model_candidate_stats: Vec::new(),
            metrics: metric_specs,
            test_scores: HashMap::new(),
            outer_train_scores: HashMap::new(),
        }
    }

    /// Returns the main evaluation metric name
    pub fn evaluation_metric(&self) -> &str {
        &self.metrics.first().expect("No metrics defined").name
    }

    pub fn evaluation_comparator(&self) -> MetricComparator {
        self.metrics.first().expect("No metrics defined").comparator
    }

    /// Adds statistics for a model candidate
    pub fn add_candidate_stats(&mut self, statistics: ModelCandidateStats) {
        self.model_candidate_stats.push(statistics);
    }

    /// Adds a test score for a metric
    pub fn add_test_score(&mut self, metric_name: String, score: f64) {
        self.test_scores.insert(metric_name, score);
    }

    /// Adds an outer train score for a metric
    pub fn add_outer_train_score(&mut self, metric_name: String, score: f64) {
        self.outer_train_scores.insert(metric_name, score);
    }

    /// Gets the main metric value for a trial
    pub fn get_main_metric(&self, trial: usize) -> f64 {
        self.model_candidate_stats[trial]
            .validation_stats
            .get(self.evaluation_metric())
            .map(|s| s.avg)
            .unwrap_or(0.0)
    }

    /// Gets the validation metrics averages for a trial
    pub fn validation_metrics_avg(&self, trial: usize) -> HashMap<String, f64> {
        self.extract_average(&self.model_candidate_stats[trial].validation_stats)
    }

    /// Gets the training metrics averages for a trial
    pub fn train_metrics_avg(&self, trial: usize) -> HashMap<String, f64> {
        self.extract_average(&self.model_candidate_stats[trial].training_stats)
    }

    /// Gets the test metrics for the winning model
    pub fn winning_model_test_metrics(&self) -> &HashMap<String, f64> {
        &self.test_scores
    }

    /// Gets the outer train metrics for the winning model
    pub fn winning_model_outer_train_metrics(&self) -> &HashMap<String, f64> {
        &self.outer_train_scores
    }

    /// Gets the index of the best trial
    pub fn best_trial_idx(&self) -> usize {
        let scores: Vec<f64> = self
            .model_candidate_stats
            .iter()
            .map(|stats| {
                stats
                    .validation_stats
                    .get(self.evaluation_metric())
                    .map(|s| s.avg)
                    .unwrap_or(0.0)
            })
            .collect();

        if scores.is_empty() {
            return 0;
        }

        let best_score = self.best_trial_score();
        scores
            .iter()
            .position(|&score| (score - best_score).abs() < f64::EPSILON)
            .unwrap_or(0)
    }

    /// Gets the score of the best trial
    pub fn best_trial_score(&self) -> f64 {
        let comparator = self.evaluation_comparator();
        let mut iter = self
            .model_candidate_stats
            .iter()
            .filter_map(|stats| stats.validation_stats.get(self.evaluation_metric()))
            .map(|s| s.avg);

        let Some(first) = iter.next() else {
            return 0.0;
        };

        iter.fold(first, |best, candidate| {
            if comparator.compare(candidate, best) == Ordering::Greater {
                candidate
            } else {
                best
            }
        })
    }

    /// Gets the best candidate's statistics
    pub fn best_candidate(&self) -> &ModelCandidateStats {
        &self.model_candidate_stats[self.best_trial_idx()]
    }

    /// Gets the parameters of the best model
    pub fn best_parameters(&self) -> &serde_json::Value {
        &self.best_candidate().trainer_config
    }

    /// Converts the statistics to a map representation
    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("bestParameters".to_string(), self.best_parameters().clone());
        map.insert(
            "bestTrial".to_string(),
            serde_json::to_value(self.best_trial_idx() + 1).unwrap(),
        );
        map.insert(
            "modelCandidates".to_string(),
            serde_json::to_value(
                self.model_candidate_stats
                    .iter()
                    .map(ModelCandidateStats::to_map)
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        );
        map
    }

    fn extract_average(
        &self,
        stats_map: &HashMap<String, EvaluationScores>,
    ) -> HashMap<String, f64> {
        stats_map
            .iter()
            .map(|(metric, scores)| (metric.clone(), scores.avg))
            .collect()
    }
}
