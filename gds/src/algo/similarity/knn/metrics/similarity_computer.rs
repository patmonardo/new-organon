use super::combined_similarity_computer::CombinedSimilarityComputer;
use super::double_array_property_similarity_computer::DoubleArrayPropertySimilarityComputer;
use super::double_property_similarity_computer::DoublePropertySimilarityComputer;
use super::float_array_property_similarity_computer::FloatArrayPropertySimilarityComputer;
use super::long_array_property_similarity_computer::LongArrayPropertySimilarityComputer;
use super::long_property_similarity_computer::LongPropertySimilarityComputer;
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum SimilarityMetric {
    #[serde(rename = "DEFAULT")]
    #[default]
    Default,
    #[serde(rename = "COSINE")]
    Cosine,
    #[serde(rename = "EUCLIDEAN")]
    Euclidean,
    #[serde(rename = "PEARSON")]
    Pearson,
    #[serde(rename = "JACCARD")]
    Jaccard,
    #[serde(rename = "OVERLAP")]
    Overlap,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnnNodePropertySpec {
    pub name: String,
    #[serde(default)]
    pub metric: SimilarityMetric,
}

impl KnnNodePropertySpec {
    pub fn new(name: impl Into<String>, metric: SimilarityMetric) -> Self {
        Self {
            name: name.into(),
            metric,
        }
    }
}

pub trait SimilarityComputer: Send + Sync {
    fn safe_similarity(&self, first_node_id: u64, second_node_id: u64) -> f64 {
        let similarity = self.similarity(first_node_id, second_node_id);
        if similarity.is_finite() {
            similarity
        } else {
            0.0
        }
    }

    fn similarity(&self, first_node_id: u64, second_node_id: u64) -> f64;

    fn is_symmetric(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct SimilarityComputerBuildError {
    message: String,
}

impl SimilarityComputerBuildError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for SimilarityComputerBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SimilarityComputerBuildError {}

impl dyn SimilarityComputer {
    pub fn of_properties(
        properties: Vec<(String, Arc<dyn NodePropertyValues>, SimilarityMetric)>,
    ) -> Result<Arc<dyn SimilarityComputer>, SimilarityComputerBuildError> {
        if properties.is_empty() {
            return Err(SimilarityComputerBuildError::new(
                "KNN requires at least one node property",
            ));
        }
        if properties.len() == 1 {
            let (name, values, metric) = properties.into_iter().next().unwrap();
            return Self::of_property_values(&name, values, metric);
        }

        let mut computers: Vec<Arc<dyn SimilarityComputer>> = Vec::with_capacity(properties.len());
        for (name, values, metric) in properties {
            computers.push(Self::of_property_values(&name, values, metric)?);
        }
        Ok(Arc::new(CombinedSimilarityComputer::new(computers)))
    }

    pub fn of_property_values(
        property_name: &str,
        values: Arc<dyn NodePropertyValues>,
        metric: SimilarityMetric,
    ) -> Result<Arc<dyn SimilarityComputer>, SimilarityComputerBuildError> {
        let value_type = values.value_type();

        match value_type {
            ValueType::Double => {
                if metric != SimilarityMetric::Default {
                    return Err(unsupported_similarity_metric(
                        property_name,
                        value_type,
                        metric,
                    ));
                }
                Ok(Arc::new(DoublePropertySimilarityComputer::new(values)))
            }
            ValueType::Long => {
                if metric != SimilarityMetric::Default {
                    return Err(unsupported_similarity_metric(
                        property_name,
                        value_type,
                        metric,
                    ));
                }
                Ok(Arc::new(LongPropertySimilarityComputer::new(values)))
            }
            ValueType::FloatArray => {
                let effective = match metric {
                    SimilarityMetric::Default => SimilarityMetric::Cosine,
                    m => m,
                };
                match effective {
                    SimilarityMetric::Cosine
                    | SimilarityMetric::Euclidean
                    | SimilarityMetric::Pearson => Ok(Arc::new(
                        FloatArrayPropertySimilarityComputer::new(values, effective),
                    )),
                    _ => Err(unsupported_similarity_metric(
                        property_name,
                        value_type,
                        metric,
                    )),
                }
            }
            ValueType::DoubleArray => {
                let effective = match metric {
                    SimilarityMetric::Default => SimilarityMetric::Cosine,
                    m => m,
                };
                match effective {
                    SimilarityMetric::Cosine
                    | SimilarityMetric::Euclidean
                    | SimilarityMetric::Pearson => Ok(Arc::new(
                        DoubleArrayPropertySimilarityComputer::new(values, effective),
                    )),
                    _ => Err(unsupported_similarity_metric(
                        property_name,
                        value_type,
                        metric,
                    )),
                }
            }
            ValueType::LongArray => {
                let effective = match metric {
                    SimilarityMetric::Default => SimilarityMetric::Jaccard,
                    m => m,
                };
                match effective {
                    SimilarityMetric::Jaccard | SimilarityMetric::Overlap => Ok(Arc::new(
                        LongArrayPropertySimilarityComputer::new(values, effective),
                    )),
                    _ => Err(unsupported_similarity_metric(
                        property_name,
                        value_type,
                        metric,
                    )),
                }
            }
            _ => Err(SimilarityComputerBuildError::new(format!(
                "Unsupported node property type {:?} for property `{}`",
                value_type, property_name
            ))),
        }
    }
}

fn unsupported_similarity_metric(
    property_name: &str,
    value_type: ValueType,
    metric: SimilarityMetric,
) -> SimilarityComputerBuildError {
    SimilarityComputerBuildError::new(format!(
        "Unsupported similarity metric {:?} for property `{}` of type {:?}",
        metric, property_name, value_type
    ))
}
