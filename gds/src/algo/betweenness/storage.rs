//! Betweenness Centrality Storage Runtime
pub struct BetweennessCentralityStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl BetweennessCentralityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
