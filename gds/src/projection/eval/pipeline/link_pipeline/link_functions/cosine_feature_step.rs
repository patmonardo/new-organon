// Phase 2.2: CosineFeatureStep - Angular similarity via cosine distance

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use super::abstract_link_feature_appender_factory::AbstractLinkFeatureAppenderFactory;
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// Cosine similarity link feature.
///
/// Computes cosine similarity between node property vectors:
/// ```text
/// cosine(v1, v2) = dot(v1, v2) / (||v1|| * ||v2||)
///                = Σ(v1[i] * v2[i]) / sqrt(Σ(v1[i]²) * Σ(v2[i]²))
/// ```
///
/// # Use Case
///
/// Cosine measures **angular similarity** - do vectors point in the same direction?
/// - Range: [-1, 1] (or [0, 1] for positive vectors)
/// - 1.0 = same direction (perfectly similar)
/// - 0.0 = orthogonal (no similarity)
/// - -1.0 = opposite direction (perfectly dissimilar)
///
/// Common for embeddings where magnitude doesn't matter, only direction.
///
/// # Example
///
/// ```text
/// Node A: [3, 4, 0]  (||A|| = 5)
/// Node B: [6, 8, 0]  (||B|| = 10)
/// Cosine: (18 + 32) / (5 * 10) = 50/50 = 1.0 (same direction!)
/// ```
///
/// # Implementation Note
///
/// Computes dot product and norms in single pass for efficiency:
/// - Accumulate: dot_product, source_norm², target_norm²
/// - Final: dot_product / sqrt(source_norm² * target_norm²)
#[derive(Debug, Clone)]
pub struct CosineFeatureStep {
    /// Node properties to compute cosine similarity on
    node_property_names: Vec<String>,
}

impl CosineFeatureStep {
    /// Creates a new CosineFeatureStep for the given node properties.
    pub fn new(node_properties: Vec<String>) -> Self {
        Self {
            node_property_names: node_properties,
        }
    }
}

impl LinkFeatureStep for CosineFeatureStep {
    fn link_feature_appender(&self, graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        let factory = CosineLinkFeatureAppenderFactory;
        let appenders = factory
            .create_appenders(graph, &self.node_property_names)
            .expect("Failed to create cosine appenders");
        Box::new(UnionLinkFeatureAppender::new(appenders))
    }

    fn name(&self) -> &str {
        "COSINE"
    }

    fn configuration(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "nodeProperties".to_string(),
            serde_json::json!(self.node_property_names),
        );
        config
    }

    fn input_node_properties(&self) -> Vec<String> {
        self.node_property_names.clone()
    }

    fn clone_box(&self) -> Box<dyn LinkFeatureStep> {
        Box::new(self.clone())
    }
}

/// Factory for creating cosine similarity appenders.
struct CosineLinkFeatureAppenderFactory;

impl AbstractLinkFeatureAppenderFactory for CosineLinkFeatureAppenderFactory {
    fn double_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(CosineDoubleArrayAppender { props, dimension })
    }

    fn float_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(CosineFloatArrayAppender { props, dimension })
    }

    fn long_array_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(CosineLongArrayAppender { props, dimension })
    }

    fn long_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(CosineLongAppender {
            props,
            _dimension: dimension,
        })
    }

    fn double_appender(
        &self,
        props: Arc<dyn NodePropertyValues>,
        dimension: usize,
    ) -> Box<dyn LinkFeatureAppender> {
        Box::new(CosineDoubleAppender {
            props,
            _dimension: dimension,
        })
    }
}

/// Union appender that combines multiple LinkFeatureAppenders.
struct UnionLinkFeatureAppender {
    appenders: Vec<Box<dyn LinkFeatureAppender>>,
}

impl UnionLinkFeatureAppender {
    fn new(appenders: Vec<Box<dyn LinkFeatureAppender>>) -> Self {
        Self { appenders }
    }
}

impl LinkFeatureAppender for UnionLinkFeatureAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        let mut current_offset = offset;
        for appender in &self.appenders {
            appender.append_features(source, target, link_features, current_offset);
            current_offset += appender.dimension();
        }
    }

    fn dimension(&self) -> usize {
        self.appenders.iter().map(|a| a.dimension()).sum()
    }

    fn is_symmetric(&self) -> bool {
        // Union is symmetric only if all appenders are symmetric
        self.appenders.iter().all(|a| a.is_symmetric())
    }
}

// Placeholder appender for Gamma quality
#[allow(dead_code)]
struct CosinePlaceholderAppender;

impl LinkFeatureAppender for CosinePlaceholderAppender {
    fn append_features(&self, _source: u64, _target: u64, _features: &mut [f64], _offset: usize) {
        // TODO: Implement cosine computation:
        // 1. Accumulate dot_product, source_square_norm, target_square_norm
        // 2. Compute l2_norm = sqrt(source_square_norm * target_square_norm)
        // 3. If l2_norm != 0.0: features[offset] = dot_product / l2_norm
        // 4. Validate not NaN
    }

    fn dimension(&self) -> usize {
        1 // Cosine returns single similarity value
    }
}

/// Cosine similarity appender for f64[] properties.
struct CosineDoubleArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for CosineDoubleArrayAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        // Get source and target vectors
        let source_vec = match self.props.double_array_value(source) {
            Ok(vec) => vec,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };
        let target_vec = match self.props.double_array_value(target) {
            Ok(vec) => vec,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };

        if source_vec.is_empty() || target_vec.is_empty() {
            link_features[offset] = 0.0;
            return;
        }

        // Compute cosine similarity: dot(v1, v2) / (||v1|| * ||v2||)
        let mut dot_product = 0.0;
        let mut source_norm_sq = 0.0;
        let mut target_norm_sq = 0.0;

        for i in 0..self.dimension {
            let s_val = source_vec[i];
            let t_val = target_vec[i];
            dot_product += s_val * t_val;
            source_norm_sq += s_val * s_val;
            target_norm_sq += t_val * t_val;
        }

        let norm_product = (source_norm_sq * target_norm_sq).sqrt();
        let cosine = if norm_product > 0.0 {
            dot_product / norm_product
        } else {
            0.0
        };

        // Validate result
        let result = if cosine.is_nan() { 0.0 } else { cosine };
        link_features[offset] = result;
    }

    fn dimension(&self) -> usize {
        1 // Cosine returns single similarity value
    }
}

/// Cosine similarity appender for f32[] properties.
struct CosineFloatArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for CosineFloatArrayAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        // Get source and target vectors
        let source_vec = match self.props.float_array_value(source) {
            Ok(vec) => vec,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };
        let target_vec = match self.props.float_array_value(target) {
            Ok(vec) => vec,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };

        if source_vec.is_empty() || target_vec.is_empty() {
            link_features[offset] = 0.0;
            return;
        }

        // Compute cosine similarity: dot(v1, v2) / (||v1|| * ||v2||)
        let mut dot_product = 0.0;
        let mut source_norm_sq = 0.0;
        let mut target_norm_sq = 0.0;

        for i in 0..self.dimension {
            let s_val = source_vec[i] as f64;
            let t_val = target_vec[i] as f64;
            dot_product += s_val * t_val;
            source_norm_sq += s_val * s_val;
            target_norm_sq += t_val * t_val;
        }

        let norm_product = (source_norm_sq * target_norm_sq).sqrt();
        let cosine = if norm_product > 0.0 {
            dot_product / norm_product
        } else {
            0.0
        };

        // Validate result
        let result = if cosine.is_nan() { 0.0 } else { cosine };
        link_features[offset] = result;
    }

    fn dimension(&self) -> usize {
        1 // Cosine returns single similarity value
    }
}

/// Cosine similarity appender for i64[] properties.
struct CosineLongArrayAppender {
    props: Arc<dyn NodePropertyValues>,
    dimension: usize,
}

impl LinkFeatureAppender for CosineLongArrayAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        // Get source and target vectors
        let source_vec = match self.props.long_array_value(source) {
            Ok(vec) => vec,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };
        let target_vec = match self.props.long_array_value(target) {
            Ok(vec) => vec,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };

        if source_vec.is_empty() || target_vec.is_empty() {
            link_features[offset] = 0.0;
            return;
        }

        // Compute cosine similarity: dot(v1, v2) / (||v1|| * ||v2||)
        let mut dot_product = 0.0;
        let mut source_norm_sq = 0.0;
        let mut target_norm_sq = 0.0;

        for i in 0..self.dimension {
            let s_val = source_vec[i] as f64;
            let t_val = target_vec[i] as f64;
            dot_product += s_val * t_val;
            source_norm_sq += s_val * s_val;
            target_norm_sq += t_val * t_val;
        }

        let norm_product = (source_norm_sq * target_norm_sq).sqrt();
        let cosine = if norm_product > 0.0 {
            dot_product / norm_product
        } else {
            0.0
        };

        // Validate result
        let result = if cosine.is_nan() { 0.0 } else { cosine };
        link_features[offset] = result;
    }

    fn dimension(&self) -> usize {
        1 // Cosine returns single similarity value
    }
}

/// Cosine similarity appender for i64 scalar properties.
struct CosineLongAppender {
    props: Arc<dyn NodePropertyValues>,
    _dimension: usize,
}

impl LinkFeatureAppender for CosineLongAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        // For scalar properties, cosine similarity doesn't make much sense
        // but we can treat them as 1D vectors
        let source_val = match self.props.long_value(source) {
            Ok(val) => val as f64,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };
        let target_val = match self.props.long_value(target) {
            Ok(val) => val as f64,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };

        // Cosine of scalars: if both non-zero and same sign, 1.0; if different signs, -1.0; else 0.0
        let cosine = if source_val != 0.0 && target_val != 0.0 {
            if (source_val > 0.0) == (target_val > 0.0) {
                1.0
            } else {
                -1.0
            }
        } else {
            0.0
        };

        link_features[offset] = cosine;
    }

    fn dimension(&self) -> usize {
        1 // Cosine returns single similarity value
    }
}

/// Cosine similarity appender for f64 scalar properties.
struct CosineDoubleAppender {
    props: Arc<dyn NodePropertyValues>,
    _dimension: usize,
}

impl LinkFeatureAppender for CosineDoubleAppender {
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize) {
        // For scalar properties, cosine similarity doesn't make much sense
        // but we can treat them as 1D vectors
        let source_val = match self.props.double_value(source) {
            Ok(val) => val,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };
        let target_val = match self.props.double_value(target) {
            Ok(val) => val,
            Err(_) => {
                link_features[offset] = 0.0;
                return;
            }
        };

        // Cosine of scalars: if both non-zero and same sign, 1.0; if different signs, -1.0; else 0.0
        let cosine = if source_val != 0.0 && target_val != 0.0 {
            if (source_val > 0.0) == (target_val > 0.0) {
                1.0
            } else {
                -1.0
            }
        } else {
            0.0
        };

        link_features[offset] = cosine;
    }

    fn dimension(&self) -> usize {
        1 // Cosine returns single similarity value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_creation() {
        let step = CosineFeatureStep::new(vec!["embedding".to_string()]);
        assert_eq!(step.node_property_names.len(), 1);
    }

    #[test]
    fn test_cosine_name() {
        let step = CosineFeatureStep::new(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "COSINE");
    }

    #[test]
    fn test_cosine_configuration() {
        let step = CosineFeatureStep::new(vec!["prop1".to_string(), "prop2".to_string()]);

        let config = step.configuration();
        assert!(config.contains_key("nodeProperties"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["embedding".to_string(), "features".to_string()];
        let step = CosineFeatureStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_dimension_is_one() {
        // Cosine similarity always returns single scalar value
        let appender = CosinePlaceholderAppender;
        assert_eq!(appender.dimension(), 1);
    }

    #[test]
    fn test_multiple_properties() {
        // Cosine can combine multiple properties
        // (computes overall cosine across concatenated vectors)
        let step = CosineFeatureStep::new(vec!["embedding1".to_string(), "embedding2".to_string()]);

        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_clone() {
        let step1 = CosineFeatureStep::new(vec!["prop".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }
}
