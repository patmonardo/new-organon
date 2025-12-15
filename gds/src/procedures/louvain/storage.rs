pub struct LouvainStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl LouvainStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
