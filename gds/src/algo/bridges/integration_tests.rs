use crate::algo::bridges::BRIDGESAlgorithmSpec;
use crate::projection::eval::algorithm::AlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridges_algorithm_spec_contract_basics() {
        let algorithm = BRIDGESAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "bridges");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }
}
