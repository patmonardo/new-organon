//! Articulation Points Storage Runtime

pub struct ArticulationPointsStorageRuntime {
    #[allow(dead_code)]
    concurrency: usize,
}

impl ArticulationPointsStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
