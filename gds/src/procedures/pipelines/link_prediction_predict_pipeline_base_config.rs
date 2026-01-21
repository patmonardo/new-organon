use serde_json::Value;

use crate::procedures::pipelines::types::AnyMap;

pub trait LinkPredictionPredictPipelineConfig {
    fn graph_name(&self) -> &str;
    fn concurrency(&self) -> usize;
    fn model_name(&self) -> &str;
    fn model_user(&self) -> &str;
    fn username(&self) -> String;
    fn sample_rate(&self) -> f64;
    fn source_node_label(&self) -> Option<&str>;
    fn target_node_label(&self) -> Option<&str>;
    fn relationship_types(&self) -> &[String];

    fn top_n(&self) -> Option<usize>;
    fn threshold(&self) -> Option<f64>;
    fn top_k(&self) -> Option<usize>;
    fn delta_threshold(&self) -> Option<f64>;
    fn max_iterations(&self) -> Option<usize>;
    fn random_joins(&self) -> Option<usize>;
    fn initial_sampler(&self) -> Option<&str>;

    fn is_approximate_strategy(&self) -> bool {
        self.sample_rate() < 1.0
    }

    fn threshold_or_default(&self) -> f64 {
        self.threshold().unwrap_or(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct LinkPredictionPredictPipelineBaseConfig {
    graph_name: String,
    concurrency: usize,
    model_name: String,
    model_user: String,
    username_override: Option<String>,
    sample_rate: f64,
    source_node_label: Option<String>,
    target_node_label: Option<String>,
    relationship_types: Vec<String>,
    top_n: Option<usize>,
    threshold: Option<f64>,
    top_k: Option<usize>,
    delta_threshold: Option<f64>,
    max_iterations: Option<usize>,
    random_joins: Option<usize>,
    initial_sampler: Option<String>,
}

impl LinkPredictionPredictPipelineBaseConfig {
    pub fn new(
        graph_name: String,
        concurrency: usize,
        model_name: String,
        model_user: String,
        username_override: Option<String>,
        sample_rate: f64,
        source_node_label: Option<String>,
        target_node_label: Option<String>,
        relationship_types: Vec<String>,
        top_n: Option<usize>,
        threshold: Option<f64>,
        top_k: Option<usize>,
        delta_threshold: Option<f64>,
        max_iterations: Option<usize>,
        random_joins: Option<usize>,
        initial_sampler: Option<String>,
    ) -> Self {
        Self {
            graph_name,
            concurrency,
            model_name,
            model_user,
            username_override,
            sample_rate,
            source_node_label,
            target_node_label,
            relationship_types,
            top_n,
            threshold,
            top_k,
            delta_threshold,
            max_iterations,
            random_joins,
            initial_sampler,
        }
    }

    pub fn from_map(username: String, mut config: AnyMap) -> Self {
        let graph_name = config
            .remove("graphName")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "graph".to_string());

        let concurrency = config
            .remove("concurrency")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or_else(num_cpus::get);

        let model_name = config
            .remove("modelName")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "default_model".to_string());

        let model_user = config
            .remove("modelUser")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| username.clone());

        let sample_rate = config
            .remove("sampleRate")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        let source_node_label = config
            .remove("sourceNodeLabel")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let target_node_label = config
            .remove("targetNodeLabel")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let relationship_types = config
            .remove("relationshipTypes")
            .and_then(|v| v.as_array().cloned())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let top_n = config
            .remove("topN")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        let threshold = config.remove("threshold").and_then(|v| v.as_f64());

        let top_k = config
            .remove("topK")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        let delta_threshold = config.remove("deltaThreshold").and_then(|v| v.as_f64());

        let max_iterations = config
            .remove("maxIterations")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        let random_joins = config
            .remove("randomJoins")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        let initial_sampler = config
            .remove("initialSampler")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        Self::new(
            graph_name,
            concurrency,
            model_name,
            model_user,
            None,
            sample_rate,
            source_node_label,
            target_node_label,
            relationship_types,
            top_n,
            threshold,
            top_k,
            delta_threshold,
            max_iterations,
            random_joins,
            initial_sampler,
        )
    }

    pub fn to_map(&self) -> AnyMap {
        let mut map = AnyMap::new();
        map.insert(
            "graphName".to_string(),
            Value::String(self.graph_name.clone()),
        );
        map.insert(
            "concurrency".to_string(),
            Value::Number(serde_json::Number::from(self.concurrency as i64)),
        );
        map.insert(
            "modelName".to_string(),
            Value::String(self.model_name.clone()),
        );
        map.insert(
            "modelUser".to_string(),
            Value::String(self.model_user.clone()),
        );
        map.insert(
            "sampleRate".to_string(),
            Value::Number(serde_json::Number::from_f64(self.sample_rate).unwrap()),
        );
        if let Some(label) = &self.source_node_label {
            map.insert("sourceNodeLabel".to_string(), Value::String(label.clone()));
        }
        if let Some(label) = &self.target_node_label {
            map.insert("targetNodeLabel".to_string(), Value::String(label.clone()));
        }
        if !self.relationship_types.is_empty() {
            map.insert(
                "relationshipTypes".to_string(),
                Value::Array(
                    self.relationship_types
                        .iter()
                        .map(|v| Value::String(v.clone()))
                        .collect(),
                ),
            );
        }
        if let Some(top_n) = self.top_n {
            map.insert(
                "topN".to_string(),
                Value::Number(serde_json::Number::from(top_n as i64)),
            );
        }
        if let Some(threshold) = self.threshold {
            map.insert(
                "threshold".to_string(),
                Value::Number(serde_json::Number::from_f64(threshold).unwrap()),
            );
        }
        if let Some(top_k) = self.top_k {
            map.insert(
                "topK".to_string(),
                Value::Number(serde_json::Number::from(top_k as i64)),
            );
        }
        if let Some(delta_threshold) = self.delta_threshold {
            map.insert(
                "deltaThreshold".to_string(),
                Value::Number(serde_json::Number::from_f64(delta_threshold).unwrap()),
            );
        }
        if let Some(max_iterations) = self.max_iterations {
            map.insert(
                "maxIterations".to_string(),
                Value::Number(serde_json::Number::from(max_iterations as i64)),
            );
        }
        if let Some(random_joins) = self.random_joins {
            map.insert(
                "randomJoins".to_string(),
                Value::Number(serde_json::Number::from(random_joins as i64)),
            );
        }
        if let Some(initial_sampler) = &self.initial_sampler {
            map.insert(
                "initialSampler".to_string(),
                Value::String(initial_sampler.clone()),
            );
        }
        map
    }
}

impl LinkPredictionPredictPipelineConfig for LinkPredictionPredictPipelineBaseConfig {
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn concurrency(&self) -> usize {
        self.concurrency
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn model_user(&self) -> &str {
        &self.model_user
    }

    fn username(&self) -> String {
        self.username_override
            .clone()
            .unwrap_or_else(|| self.model_user.clone())
    }

    fn sample_rate(&self) -> f64 {
        self.sample_rate
    }

    fn source_node_label(&self) -> Option<&str> {
        self.source_node_label.as_deref()
    }

    fn target_node_label(&self) -> Option<&str> {
        self.target_node_label.as_deref()
    }

    fn relationship_types(&self) -> &[String] {
        &self.relationship_types
    }

    fn top_n(&self) -> Option<usize> {
        self.top_n
    }

    fn threshold(&self) -> Option<f64> {
        self.threshold
    }

    fn top_k(&self) -> Option<usize> {
        self.top_k
    }

    fn delta_threshold(&self) -> Option<f64> {
        self.delta_threshold
    }

    fn max_iterations(&self) -> Option<usize> {
        self.max_iterations
    }

    fn random_joins(&self) -> Option<usize> {
        self.random_joins
    }

    fn initial_sampler(&self) -> Option<&str> {
        self.initial_sampler.as_deref()
    }
}
