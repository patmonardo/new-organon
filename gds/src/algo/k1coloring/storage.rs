pub struct K1ColoringStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl K1ColoringStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
