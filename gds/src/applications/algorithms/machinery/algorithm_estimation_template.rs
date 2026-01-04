use crate::mem::MemoryEstimation;
use crate::config::base_types::Config;
use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::mem::memest::MemoryEstimationResultBuilder;

/// Template for algorithm memory estimation.
/// This provides a standardized way to estimate memory requirements
/// for different algorithms.
#[derive(Clone)]
pub struct AlgorithmEstimationTemplate;

impl AlgorithmEstimationTemplate {
    pub fn new() -> Self {
        Self
    }

    fn parse_dimensions(input: &str) -> Option<ConcreteGraphDimensions> {
        let s = input.trim();
        if s.is_empty() {
            return None;
        }

        // Format 1: "<nodes>,<rels>" (e.g. "1000,5000")
        if let Some((left, right)) = s.split_once(',') {
            let node_count = left.trim().parse::<usize>().ok()?;
            let relationship_count = right.trim().parse::<usize>().ok()?;
            return Some(ConcreteGraphDimensions::of(node_count, relationship_count));
        }

        // Format 2: key/value tokens (e.g. "nodeCount=1000 relationshipCount=5000")
        let mut node_count: Option<usize> = None;
        let mut relationship_count: Option<usize> = None;

        for token in s
            .split(|c: char| c.is_whitespace() || c == ';' || c == ',')
            .filter(|t| !t.is_empty())
        {
            let (k, v) = token.split_once('=')?;
            let key = k.trim();
            let value = v.trim().parse::<usize>().ok()?;

            match key {
                "nodeCount" | "nodes" | "n" => node_count = Some(value),
                "relationshipCount" | "rels" | "relationships" | "r" => {
                    relationship_count = Some(value)
                }
                _ => {}
            }
        }

        match (node_count, relationship_count) {
            (Some(n), Some(r)) => Some(ConcreteGraphDimensions::of(n, r)),
            _ => None,
        }
    }

    /// Estimates memory for an algorithm with the given configuration.
    pub fn estimate<C: Config>(
        &self,
        _config: &C,
        graph_name_or_configuration: &str,
        memory_estimation: Box<dyn MemoryEstimation>,
    ) -> crate::mem::MemoryEstimationResult {
        // Note: this template doesn't currently have access to a graph catalog.
        // For now we derive dimensions from the provided string, and default to
        // (0, 0) when dimensions are not available.
        //
        // Accepted formats:
        // - "<nodes>,<rels>" e.g. "1000,5000"
        // - "nodeCount=1000 relationshipCount=5000" (also supports nodes/n and rels/r)
        let dimensions = Self::parse_dimensions(graph_name_or_configuration)
            .unwrap_or_else(|| ConcreteGraphDimensions::of(0, 0));

        let tree = memory_estimation.estimate(&dimensions, 1);

        MemoryEstimationResultBuilder::new()
            .with_dimensions(dimensions)
            .with_memory_tree(tree)
            .build()
    }
}

impl Default for AlgorithmEstimationTemplate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::GraphDimensions;
    use crate::mem::{MemoryRange, MemoryTree};

    #[derive(Clone)]
    struct FixedEstimation;

    impl MemoryEstimation for FixedEstimation {
        fn description(&self) -> String {
            "fixed".to_string()
        }

        fn estimate(&self, _dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
            MemoryTree::leaf("fixed".to_string(), MemoryRange::of(1024))
        }
    }

    #[derive(Clone)]
    struct DummyConfig;
    impl Config for DummyConfig {}

    #[test]
    fn parse_dimensions_csv() {
        let dims = AlgorithmEstimationTemplate::parse_dimensions("1000, 5000").unwrap();
        assert_eq!(dims.node_count, 1000);
        assert_eq!(dims.relationship_count, 5000);
    }

    #[test]
    fn parse_dimensions_kv() {
        let dims =
            AlgorithmEstimationTemplate::parse_dimensions("nodeCount=42 relationshipCount=7")
                .unwrap();
        assert_eq!(dims.node_count, 42);
        assert_eq!(dims.relationship_count, 7);
    }

    #[test]
    fn estimate_no_panic() {
        let template = AlgorithmEstimationTemplate::new();
        let cfg = DummyConfig;

        let result = template.estimate(&cfg, "100,200", Box::new(FixedEstimation));
        assert_eq!(result.dimensions().node_count, 100);
        assert_eq!(result.memory_range().min(), 1024);
    }
}
