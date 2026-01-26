use super::config::GATConfig;
use super::layer::GATLayer;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use std::collections::HashMap;

pub struct GATAggregator {
    pub layers: Vec<GATLayer>,
}

impl GATAggregator {
    pub fn new(config: GATConfig) -> Self {
        let layers = (0..config.num_layers)
            .map(|_| GATLayer::new(config.clone()))
            .collect();
        Self { layers }
    }

    pub fn aggregate(
        &mut self,
        graph: &dyn Graph,
        initial_features: HashMap<NodeId, Vec<f64>>,
    ) -> HashMap<NodeId, Vec<f64>> {
        let mut features = initial_features;
        for layer in &mut self.layers {
            layer.forward(graph, &mut features);
        }
        features
    }
}
