//! Java: `FeatureFunction`.

use crate::collections::HugeObjectArray;
use crate::ml::core::variable::VariableRef;
use crate::types::graph::Graph;
use std::sync::Arc;

pub trait FeatureFunction: Send + Sync {
    fn apply(
        &self,
        graph: Arc<dyn Graph>,
        node_ids: &[u64],
        features: Arc<HugeObjectArray<Vec<f64>>>,
    ) -> VariableRef;
}
