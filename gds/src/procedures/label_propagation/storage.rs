//! Label Propagation Storage Runtime
//!
//! Storage runtime is responsible for building graph-facing adapters and invoking the
//! computation runtime. (Matches the other procedure modules' split.)

pub struct LabelPropStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl LabelPropStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}

impl Default for LabelPropStorageRuntime {
    fn default() -> Self {
        Self::new(num_cpus::get().max(1))
    }
}
