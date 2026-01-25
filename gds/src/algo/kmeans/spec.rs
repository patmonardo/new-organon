//! K-Means config + result types.

use crate::config::validation::ConfigError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KMeansSamplerType {
    #[serde(rename = "UNIFORM")]
    Uniform,

    #[serde(rename = "KMEANSPP")]
    KmeansPlusPlus,
}

impl Default for KMeansSamplerType {
    fn default() -> Self {
        Self::KmeansPlusPlus
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KMeansConfig {
    #[serde(default)]
    pub k: usize,

    #[serde(default, rename = "maxIterations")]
    pub max_iterations: u32,

    /// Java parity: `deltaSwaps` in [0,1].
    ///
    /// Stop when `iteration > 1 && swaps <= nodeCount * delta_threshold`.
    #[serde(default, rename = "deltaThreshold")]
    pub delta_threshold: f64,

    #[serde(default, rename = "numberOfRestarts")]
    pub number_of_restarts: u32,

    #[serde(default, rename = "computeSilhouette")]
    pub compute_silhouette: bool,

    #[serde(default)]
    pub concurrency: usize,

    #[serde(default, rename = "nodeProperty")]
    pub node_property: String,

    #[serde(default, rename = "samplerType")]
    pub sampler_type: KMeansSamplerType,

    #[serde(default, rename = "seedCentroids")]
    pub seed_centroids: Vec<Vec<f64>>,

    #[serde(default, rename = "randomSeed")]
    pub random_seed: Option<u64>,
}

impl Default for KMeansConfig {
    fn default() -> Self {
        Self {
            k: 2,
            max_iterations: 10,
            delta_threshold: 0.0,
            number_of_restarts: 1,
            compute_silhouette: false,
            concurrency: 1,
            node_property: String::new(),
            sampler_type: KMeansSamplerType::default(),
            seed_centroids: Vec::new(),
            random_seed: None,
        }
    }
}

impl KMeansConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.k == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "k".to_string(),
                reason: "k must be > 0".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "maxIterations must be > 0".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.delta_threshold) {
            return Err(ConfigError::InvalidParameter {
                parameter: "deltaThreshold".to_string(),
                reason: "deltaThreshold must be in [0.0, 1.0]".to_string(),
            });
        }
        if self.number_of_restarts == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "numberOfRestarts".to_string(),
                reason: "numberOfRestarts must be > 0".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        if self.node_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "nodeProperty".to_string(),
                reason: "nodeProperty cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for KMeansConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        KMeansConfig::validate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KMeansResult {
    pub communities: Vec<u64>,
    pub distance_from_center: Vec<f64>,
    pub centers: Vec<Vec<f64>>,

    pub average_distance_to_centroid: f64,

    /// When enabled, per-node silhouette score.
    pub silhouette: Option<Vec<f64>>,
    pub average_silhouette: f64,

    pub ran_iterations: u32,
    pub restarts: u32,
}
