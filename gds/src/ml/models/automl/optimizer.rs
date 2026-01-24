use super::config::TunableTrainerConfig;
use super::hyperparameter::DoubleRangeParameter;
use super::hyperparameter::IntegerRangeParameter;
use super::hyperparameter::NumericalRangeParameter;
use crate::ml::models::TrainerConfig;
use crate::ml::models::TrainingMethod;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashMap;

/// HyperParameterOptimizer defines a strategy for searching through a hyperparameter space
///
/// Java: `interface HyperParameterOptimizer extends Iterator<TrainerConfig>`
pub trait HyperParameterOptimizer: Iterator<Item = Box<dyn TrainerConfig>> {}

/// A basic implementation of random search through a hyperparameter space
///
/// 1:1 with RandomSearch.java
pub struct RandomSearch {
    concrete_configs: Vec<TunableTrainerConfig>,
    tunable_configs: Vec<TunableTrainerConfig>,
    total_number_of_trials: usize,
    number_of_concrete_trials: usize,
    random: StdRng,
    number_of_finished_trials: usize,
}

impl RandomSearch {
    pub fn new(
        parameter_space: HashMap<TrainingMethod, Vec<TunableTrainerConfig>>,
        max_trials: usize,
        random_seed: u64,
    ) -> Self {
        Self::new_with_seed(parameter_space, max_trials, Some(random_seed))
    }

    pub fn new_with_seed(
        parameter_space: HashMap<TrainingMethod, Vec<TunableTrainerConfig>>,
        max_trials: usize,
        random_seed: Option<u64>,
    ) -> Self {
        let concrete_configs: Vec<TunableTrainerConfig> = parameter_space
            .values()
            .flatten()
            .filter(|config| config.is_concrete())
            .cloned()
            .collect();

        let tunable_configs: Vec<TunableTrainerConfig> = parameter_space
            .values()
            .flatten()
            .filter(|config| !config.is_concrete())
            .cloned()
            .collect();

        let number_of_concrete_trials = concrete_configs.len();
        let total_number_of_trials = max_trials + number_of_concrete_trials;
        let random = random_seed
            .map(StdRng::seed_from_u64)
            .unwrap_or_else(StdRng::from_entropy);

        Self {
            concrete_configs,
            tunable_configs,
            total_number_of_trials,
            number_of_concrete_trials,
            random,
            number_of_finished_trials: 0,
        }
    }

    pub fn has_next(&self) -> bool {
        (self.number_of_finished_trials < self.number_of_concrete_trials)
            || (self.number_of_finished_trials < self.total_number_of_trials
                && !self.tunable_configs.is_empty())
    }

    fn sample_integer(&mut self, range: &IntegerRangeParameter) -> i32 {
        self.random.gen_range(range.min()..range.max())
    }

    fn sample_double(&mut self, range: &DoubleRangeParameter) -> f64 {
        if range.log_scale() {
            let min = if range.min() < 1e-20 {
                (1e-20_f64).ln()
            } else {
                range.min().ln()
            };
            let max = range.max().ln();
            self.random.gen_range(min..max).exp()
        } else {
            self.random.gen_range(range.min()..range.max())
        }
    }

    fn sample(&mut self, tunable_config: &TunableTrainerConfig) -> Box<dyn TrainerConfig> {
        let mut hyper_parameter_values = HashMap::new();
        tunable_config
            .double_ranges()
            .iter()
            .for_each(|(name, range)| {
                hyper_parameter_values.insert(
                    name.clone(),
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(self.sample_double(range)).unwrap(),
                    ),
                );
            });
        tunable_config
            .integer_ranges()
            .iter()
            .for_each(|(name, range)| {
                hyper_parameter_values.insert(
                    name.clone(),
                    serde_json::Value::Number(self.sample_integer(range).into()),
                );
            });
        tunable_config.materialize(hyper_parameter_values)
    }
}

impl Iterator for RandomSearch {
    type Item = Box<dyn TrainerConfig>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            panic!("RandomSearch has already exhausted the maximum trials or the parameter space.");
        }

        let config = if self.number_of_finished_trials < self.concrete_configs.len() {
            let config =
                self.concrete_configs[self.number_of_finished_trials].materialize(HashMap::new());
            self.number_of_finished_trials += 1;
            config
        } else {
            self.number_of_finished_trials += 1;
            let idx = self.random.gen_range(0..self.tunable_configs.len());
            let tunable = self.tunable_configs[idx].clone();
            self.sample(&tunable)
        };

        Some(config)
    }
}

impl HyperParameterOptimizer for RandomSearch {}
