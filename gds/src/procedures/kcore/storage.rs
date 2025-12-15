pub struct KCoreStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl KCoreStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
