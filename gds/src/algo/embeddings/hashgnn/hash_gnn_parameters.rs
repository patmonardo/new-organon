use crate::concurrency::Concurrency;

/// Config for generating random base features when no feature properties are provided.
#[derive(Debug, Clone)]
pub struct GenerateFeaturesConfig {
    pub dimension: usize,
    pub density_level: usize,
}

/// Config for binarizing scalar/array properties into binary embeddings.
#[derive(Debug, Clone)]
pub struct BinarizeFeaturesConfig {
    pub dimension: usize,
    pub threshold: f64,
}

/// Parameters for HashGNN.
///
/// Java: `record HashGNNParameters(Concurrency concurrency, int iterations, int embeddingDensity, double neighborInfluence, List<String> featureProperties, boolean heterogeneous, Optional<Integer> outputDimension, Optional<BinarizeFeaturesConfig> binarizeFeatures, Optional<GenerateFeaturesConfig> generateFeatures, Optional<Long> randomSeed)`
#[derive(Debug, Clone)]
pub struct HashGNNParameters {
    pub concurrency: Concurrency,
    pub iterations: usize,
    pub embedding_density: usize,
    pub neighbor_influence: f64,
    pub feature_properties: Vec<String>,
    pub heterogeneous: bool,
    pub output_dimension: Option<usize>,
    pub binarize_features: Option<BinarizeFeaturesConfig>,
    pub generate_features: Option<GenerateFeaturesConfig>,
    pub random_seed: Option<u64>,
}
