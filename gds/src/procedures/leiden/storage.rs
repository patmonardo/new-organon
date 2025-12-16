use super::spec::LeidenResult;

/// Storage for Leiden algorithm computation
pub struct LeidenStorage {
    communities: Vec<u64>,
    modularity: f64,
    levels: usize,
    converged: bool,
    modularities: Vec<f64>,
}

impl LeidenStorage {
    pub fn new(node_count: usize) -> Self {
        Self {
            communities: vec![0; node_count],
            modularity: 0.0,
            levels: 0,
            converged: false,
            modularities: Vec::new(),
        }
    }

    pub fn set_communities(&mut self, communities: Vec<u64>) {
        self.communities = communities;
    }

    pub fn set_modularity(&mut self, modularity: f64) {
        self.modularity = modularity;
    }

    pub fn set_levels(&mut self, levels: usize) {
        self.levels = levels;
    }

    pub fn set_converged(&mut self, converged: bool) {
        self.converged = converged;
    }

    pub fn add_modularity(&mut self, modularity: f64) {
        self.modularities.push(modularity);
    }

    pub fn into_result(self) -> LeidenResult {
        LeidenResult::new(
            self.communities,
            self.modularity,
            self.levels,
            self.converged,
            self.modularities,
        )
    }
}
