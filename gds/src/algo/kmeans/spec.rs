//! K-Means config + result types.

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
