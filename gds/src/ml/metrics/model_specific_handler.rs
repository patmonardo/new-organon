use super::Metric;

/// Handler for model-specific metrics during training.
/// Filters metrics to only those marked as model-specific and delegates to a consumer.
///
/// This is a 1:1 translation of ModelSpecificMetricsHandler from Java GDS.
#[derive(Clone)]
pub struct ModelSpecificMetricsHandler {
    metrics: Vec<String>,
    metric_consumer: MetricConsumer,
}

type MetricConsumer = std::sync::Arc<dyn Fn(&dyn Metric, f64) + Send + Sync>;

impl ModelSpecificMetricsHandler {
    /// Creates a no-op handler that does nothing
    pub fn noop() -> Self {
        Self {
            metrics: Vec::new(),
            metric_consumer: std::sync::Arc::new(|_, _| {}),
        }
    }

    /// Creates a handler from a list of metrics and a consumer function.
    /// Only metrics marked as model-specific will be included.
    pub fn new<F>(metrics: &[Box<dyn Metric>], consumer: F) -> Self
    where
        F: Fn(&dyn Metric, f64) + Send + Sync + 'static,
    {
        let filtered_metrics: Vec<String> = metrics
            .iter()
            .filter(|m| m.is_model_specific())
            .map(|m| m.name().to_string())
            .collect();

        Self {
            metrics: filtered_metrics,
            metric_consumer: std::sync::Arc::new(consumer),
        }
    }

    /// Creates a handler from a list of metrics and a ModelStatsBuilder.
    /// 1:1 with ModelSpecificMetricsHandler.of(metrics, modelStatsBuilder) in Java
    pub fn for_stats_builder(
        metrics: &[Box<dyn Metric>],
        stats_builder: std::sync::Arc<std::sync::Mutex<super::ModelStatsBuilder>>,
    ) -> Self {
        Self::new(metrics, move |metric, score| {
            if let Ok(mut builder) = stats_builder.lock() {
                builder.update(metric, score);
            }
        })
    }

    /// Creates a handler that ignores results (for testing).
    /// 1:1 with ModelSpecificMetricsHandler.ignoringResult(metrics) in Java
    #[cfg(test)]
    pub fn ignoring_result(metrics: &[Box<dyn Metric>]) -> Self {
        let filtered_metrics: Vec<String> = metrics
            .iter()
            .filter(|m| m.is_model_specific())
            .map(|m| m.name().to_string())
            .collect();

        Self {
            metrics: filtered_metrics,
            metric_consumer: std::sync::Arc::new(|_, _| {}),
        }
    }

    /// Checks if a metric is requested (should be handled)
    pub fn is_requested(&self, metric: &dyn Metric) -> bool {
        self.metrics.contains(&metric.name().to_string())
    }

    /// Handles a metric value, passing it to the consumer
    pub fn handle(&self, metric: &dyn Metric, score: f64) {
        if !self.is_requested(metric) {
            panic!("Should not handle a metric which is not requested");
        }
        (self.metric_consumer)(metric, score);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_noop_handler() {
        let handler = ModelSpecificMetricsHandler::noop();
        assert_eq!(handler.metrics.len(), 0);
    }

    #[test]
    fn test_for_stats_builder() {
        use super::super::ModelStatsBuilder;

        let stats_builder = Arc::new(Mutex::new(ModelStatsBuilder::new(2)));
        let metrics: Vec<Box<dyn Metric>> = vec![]; // Empty for this test

        let handler =
            ModelSpecificMetricsHandler::for_stats_builder(&metrics, stats_builder.clone());

        // Handler should be created successfully
        assert_eq!(handler.metrics.len(), 0);
    }

    #[test]
    fn test_ignoring_result() {
        let metrics: Vec<Box<dyn Metric>> = vec![]; // Empty for this test
        let handler = ModelSpecificMetricsHandler::ignoring_result(&metrics);
        assert_eq!(handler.metrics.len(), 0);
    }
}
