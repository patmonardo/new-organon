pub struct TriangleCountStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl TriangleCountStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
