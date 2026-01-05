//! Closeness Centrality Storage Runtime
//!
//! This is a minimal storage layer to keep module structure consistent.
//! Today, the procedures layer owns mutate/write behavior for closeness.

use super::spec::ClosenessCentralityResult;

pub struct ClosenessCentralityStorageRuntime {
    pub result: Option<ClosenessCentralityResult>,
}

impl Default for ClosenessCentralityStorageRuntime {
    fn default() -> Self {
        Self { result: None }
    }
}

impl ClosenessCentralityStorageRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&mut self, result: ClosenessCentralityResult) {
        self.result = Some(result);
    }
}
