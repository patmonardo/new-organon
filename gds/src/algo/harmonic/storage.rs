//! Harmonic Storage Runtime
pub struct HarmonicStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl HarmonicStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
