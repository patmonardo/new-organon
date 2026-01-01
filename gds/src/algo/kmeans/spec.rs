use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum KMeansSamplerType {
    #[serde(rename = "UNIFORM")]
    #[default]
    Uniform,
    #[serde(rename = "KMEANSPP")]
    KmeansPlusPlus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KMeansConfig {
    pub k: usize,
    pub max_iterations: u32,
    /// Stop when swaps <= node_count * delta_threshold (after iteration 1).
    pub delta_threshold: f64,
    pub number_of_restarts: u32,
    pub compute_silhouette: bool,
    pub concurrency: usize,
    pub node_property: String,
    pub sampler_type: KMeansSamplerType,
    /// Optional seeded centroids. If provided, must contain exactly `k` centroids.
    pub seed_centroids: Vec<Vec<f64>>,
    pub random_seed: Option<u64>,
}

impl Default for KMeansConfig {
    fn default() -> Self {
        Self {
            k: 2,
            max_iterations: 10,
            delta_threshold: 0.001,
            number_of_restarts: 1,
            compute_silhouette: false,
            concurrency: 4,
            node_property: "".to_string(),
            sampler_type: KMeansSamplerType::Uniform,
            seed_centroids: Vec::new(),
            random_seed: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KMeansResult {
    pub communities: Vec<u64>,
    pub distance_from_center: Vec<f64>,
    pub centers: Vec<Vec<f64>>,
    pub average_distance_to_centroid: f64,
    pub silhouette: Option<Vec<f64>>,
    pub average_silhouette: f64,
    pub ran_iterations: u32,
    pub restarts: u32,
}
