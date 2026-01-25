use crate::algo::closeness::CLOSENESSAlgorithmSpec;
use crate::projection::eval::algorithm::AlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closeness_algorithm_spec_contract_basics() {
        let algorithm = CLOSENESSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "closeness");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }
}
