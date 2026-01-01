//! Bridges Storage Runtime

pub struct BridgesStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl BridgesStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
