use crate::collections::HugeDoubleArray;
use crate::ml::metrics::Metric;
use crate::ml::metrics::MetricComparator;

#[derive(Debug, Clone, Copy)]
pub enum RegressionMetric {
    MeanSquaredError,
    RootMeanSquaredError,
    MeanAbsoluteError,
}

impl RegressionMetric {
    pub fn compute(&self, targets: &HugeDoubleArray, predictions: &HugeDoubleArray) -> f64 {
        match self {
            Self::MeanSquaredError => self.compute_mse(targets, predictions),
            Self::RootMeanSquaredError => self.compute_rmse(targets, predictions),
            Self::MeanAbsoluteError => self.compute_mae(targets, predictions),
        }
    }

    fn compute_mse(&self, targets: &HugeDoubleArray, predictions: &HugeDoubleArray) -> f64 {
        let number_of_examples = targets.size();
        debug_assert_eq!(number_of_examples, predictions.size());
        if number_of_examples == 0 {
            return 0.0;
        }

        let mut squared_error = 0.0;
        for i in 0..number_of_examples {
            let error = predictions.get(i) - targets.get(i);
            squared_error += error * error;
        }

        squared_error / number_of_examples as f64
    }

    fn compute_rmse(&self, targets: &HugeDoubleArray, predictions: &HugeDoubleArray) -> f64 {
        self.compute_mse(targets, predictions).sqrt()
    }

    fn compute_mae(&self, targets: &HugeDoubleArray, predictions: &HugeDoubleArray) -> f64 {
        let number_of_examples = targets.size();
        debug_assert_eq!(number_of_examples, predictions.size());
        if number_of_examples == 0 {
            return 0.0;
        }

        let mut total_error = 0.0;
        for i in 0..number_of_examples {
            total_error += (targets.get(i) - predictions.get(i)).abs();
        }

        total_error / number_of_examples as f64
    }

    pub fn parse(input: &str) -> Self {
        let input_upper = input.to_ascii_uppercase();
        match input_upper.as_str() {
            "MEAN_SQUARED_ERROR" => Self::MeanSquaredError,
            "ROOT_MEAN_SQUARED_ERROR" => Self::RootMeanSquaredError,
            "MEAN_ABSOLUTE_ERROR" => Self::MeanAbsoluteError,
            _ => panic!(
                "RegressionMetric `{}` is not supported. Must be one of: {:?}.",
                input,
                Self::values()
            ),
        }
    }

    pub fn values() -> Vec<&'static str> {
        vec![
            "MEAN_SQUARED_ERROR",
            "ROOT_MEAN_SQUARED_ERROR",
            "MEAN_ABSOLUTE_ERROR",
        ]
    }
}

impl Metric for RegressionMetric {
    fn name(&self) -> &str {
        match self {
            Self::MeanSquaredError => "MEAN_SQUARED_ERROR",
            Self::RootMeanSquaredError => "ROOT_MEAN_SQUARED_ERROR",
            Self::MeanAbsoluteError => "MEAN_ABSOLUTE_ERROR",
        }
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Inverse
    }
}

impl std::fmt::Display for RegressionMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
