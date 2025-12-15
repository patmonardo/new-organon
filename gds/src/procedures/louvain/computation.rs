#[derive(Clone)]
pub struct LouvainResult {
    pub data: Vec<u64>,
}

pub struct LouvainComputationRuntime {
}

impl LouvainComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute(
        &mut self,
        node_count: usize,
        _get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> LouvainResult {
        LouvainResult {
            data: vec![0u64; node_count],
        }
    }
}

impl Default for LouvainComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
