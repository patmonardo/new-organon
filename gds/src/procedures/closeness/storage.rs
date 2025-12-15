//! Closeness Centrality Storage Runtime
pub struct ClosenessCentralityStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl ClosenessCentralityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
