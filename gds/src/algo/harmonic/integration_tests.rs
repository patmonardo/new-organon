use crate::algo::harmonic::HARMONICAlgorithmSpec;
use crate::projection::eval::algorithm::AlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_algorithm_spec_contract_basics() {
        let algorithm = HARMONICAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "harmonic");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }
}
