use crate::algo::degree_centrality::DEGREE_CENTRALITYAlgorithmSpec;
use crate::projection::eval::algorithm::AlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_centrality_algorithm_spec_contract_basics() {
        let algorithm = DEGREE_CENTRALITYAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "degree_centrality");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }
}
