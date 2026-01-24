use crate::collections::long_multiset::LongMultiSet;
use crate::mem::MemoryEstimation;
use crate::mem::MemoryEstimations;
use crate::mem::MemoryRange;
use crate::ml::core::subgraph::LocalIdMap;

use super::classification::Accuracy;
use super::classification::F1Macro;
use super::classification::F1Score;
use super::classification::F1Weighted;
use super::classification::GlobalAccuracy;
use super::classification::OutOfBagError;
use super::classification::Precision;
use super::classification::Recall;
use super::metric::Metric;
use std::collections::HashSet;

#[derive(Clone)]
pub struct ClassificationMetricSpecification {
    string_representation: String,
    metric_factory:
        std::sync::Arc<dyn Fn(&LocalIdMap, &LongMultiSet) -> Vec<Box<dyn Metric>> + Send + Sync>,
}

impl ClassificationMetricSpecification {
    pub fn create_metrics(
        &self,
        class_id_map: &LocalIdMap,
        class_counts: &LongMultiSet,
    ) -> Vec<Box<dyn Metric>> {
        (self.metric_factory)(class_id_map, class_counts)
    }

    pub fn memory_estimation(number_of_classes: usize) -> Box<dyn MemoryEstimation> {
        let size_of_representative_metric = 8 + 8 + 8;
        MemoryEstimations::builder("metrics")
            .fixed_range(
                "metrics",
                MemoryRange::of_range(
                    size_of_representative_metric,
                    number_of_classes * size_of_representative_metric,
                ),
            )
            .build()
    }

    pub fn specifications_to_string(specs: &[ClassificationMetricSpecification]) -> Vec<String> {
        specs.iter().map(|spec| spec.to_string()).collect()
    }

    pub fn parse_list(user_specifications: &[String]) -> Result<Vec<Self>, String> {
        if user_specifications.is_empty() {
            return Err("No metrics specified, we require at least one".to_string());
        }

        let main_metric = user_specifications[0].to_ascii_uppercase();
        let mut errors: Vec<String> = Vec::new();

        if main_metric.contains('*') {
            errors.push(format!(
                "The primary (first) metric provided must be one of {}.",
                Self::valid_primary_metric_expressions().join(", ")
            ));
        }

        let bad_specs: Vec<String> = user_specifications
            .iter()
            .filter(|spec| Self::invalid_specification(spec))
            .cloned()
            .collect();

        if !bad_specs.is_empty() {
            errors.push(Self::error_message(&bad_specs));
        }

        if !errors.is_empty() {
            return Err(errors.join(" "));
        }

        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for spec in user_specifications {
            let parsed = Self::parse(spec)?;
            if seen.insert(parsed.to_string()) {
                result.push(parsed);
            }
        }
        Ok(result)
    }

    pub fn parse(user_specification: &str) -> Result<Self, String> {
        use crate::core::utils::string_formatting::to_upper_case_with_locale;

        let input = to_upper_case_with_locale(user_specification);
        if input == OutOfBagError::NAME {
            return Ok(Self::new(
                |_, _| vec![Box::new(OutOfBagError::new())],
                input,
            ));
        }

        if let Some((metric_type, class_id)) = Self::parse_single_class_spec(&input) {
            let metric_factory = Self::single_class_metric_factory(&metric_type)
                .ok_or_else(|| Self::error_message(&vec![user_specification.to_string()]))?;

            let spec_string = format!("{}(class={})", metric_type, class_id);
            if class_id == "*" {
                return Ok(Self::new(
                    move |class_id_map, _| {
                        class_id_map
                            .mappings()
                            .map(|(original, internal)| {
                                metric_factory(original as i64, internal as i64)
                            })
                            .collect()
                    },
                    spec_string,
                ));
            }

            let class_id_value = class_id
                .parse::<i64>()
                .map_err(|_| Self::error_message(&vec![user_specification.to_string()]))?;
            if class_id_value < 0 {
                return Err(Self::error_message(&vec![user_specification.to_string()]));
            }

            return Ok(Self::new(
                move |class_id_map, _| {
                    let internal =
                        class_id_map
                            .mapped(class_id_value as u64)
                            .unwrap_or_else(|| {
                                panic!("Class id {} not found in LocalIdMap", class_id_value)
                            });
                    vec![metric_factory(class_id_value, internal as i64)]
                },
                spec_string,
            ));
        }

        if let Some(all_class_metric) = Self::all_class_metric_factory(&input) {
            return Ok(Self::new(
                move |class_id_map, class_counts| {
                    vec![all_class_metric(class_id_map, class_counts)]
                },
                input,
            ));
        }

        Err(Self::error_message(&vec![user_specification.to_string()]))
    }

    fn new<F>(metric_factory: F, string_representation: String) -> ClassificationMetricSpecification
    where
        F: Fn(&LocalIdMap, &LongMultiSet) -> Vec<Box<dyn Metric>> + Send + Sync + 'static,
    {
        ClassificationMetricSpecification {
            metric_factory: std::sync::Arc::new(metric_factory),
            string_representation,
        }
    }

    fn single_class_metric_factory(metric_type: &str) -> Option<fn(i64, i64) -> Box<dyn Metric>> {
        match metric_type {
            F1Score::NAME => Some(|original, internal| Box::new(F1Score::new(original, internal))),
            Precision::NAME => {
                Some(|original, internal| Box::new(Precision::new(original, internal)))
            }
            Recall::NAME => Some(|original, internal| Box::new(Recall::new(original, internal))),
            Accuracy::NAME => {
                Some(|original, internal| Box::new(Accuracy::new(original, internal)))
            }
            _ => None,
        }
    }

    fn all_class_metric_factory(
        metric_type: &str,
    ) -> Option<fn(&LocalIdMap, &LongMultiSet) -> Box<dyn Metric>> {
        match metric_type {
            F1Weighted::NAME => Some(|class_id_map, class_counts| {
                Box::new(F1Weighted::new(class_id_map.clone(), class_counts.clone()))
            }),
            F1Macro::NAME => Some(|class_id_map, _| Box::new(F1Macro::new(class_id_map.clone()))),
            GlobalAccuracy::NAME => Some(|_, _| Box::new(GlobalAccuracy::new())),
            _ => None,
        }
    }

    fn parse_single_class_spec(spec: &str) -> Option<(String, String)> {
        let compact: String = spec.chars().filter(|c| !c.is_whitespace()).collect();
        let open = compact.find('(')?;
        let close = compact.rfind(')')?;
        if close != compact.len() - 1 {
            return None;
        }

        let metric_type = &compact[..open];
        let inner = &compact[open + 1..close];
        if !inner.starts_with("CLASS=") {
            return None;
        }

        let class_id = &inner["CLASS=".len()..];
        if class_id.is_empty() {
            return None;
        }

        if class_id != "*" && !Self::is_signed_integer(class_id) {
            return None;
        }

        Some((metric_type.to_string(), class_id.to_string()))
    }

    fn is_signed_integer(value: &str) -> bool {
        let mut chars = value.chars();
        let first = chars.next();
        match first {
            Some('-') => {
                let mut has_digit = false;
                for c in chars {
                    if !c.is_ascii_digit() {
                        return false;
                    }
                    has_digit = true;
                }
                has_digit
            }
            Some(c) => c.is_ascii_digit() && chars.all(|c| c.is_ascii_digit()),
            None => false,
        }
    }

    fn error_message(specifications: &[String]) -> String {
        let specs = specifications
            .iter()
            .map(|s| format!("`{}`", s))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "Invalid metric expression{} {}. Available metrics are {} (case insensitive and space allowed between brackets).",
            if specifications.len() == 1 { "" } else { "s" },
            specs,
            Self::all_valid_metric_expressions().join(", ")
        )
    }

    fn invalid_specification(spec: &str) -> bool {
        let upper = spec.to_ascii_uppercase();
        if upper == OutOfBagError::NAME {
            return false;
        }

        if Self::parse_single_class_spec(&upper).is_some() {
            return false;
        }

        Self::all_class_metric_factory(&upper).is_none()
    }

    fn all_valid_metric_expressions() -> Vec<String> {
        Self::valid_metric_expressions(true)
    }

    fn valid_primary_metric_expressions() -> Vec<String> {
        Self::valid_metric_expressions(false)
    }

    fn valid_metric_expressions(include_syntactic_sugar_metrics: bool) -> Vec<String> {
        let mut valid = vec![OutOfBagError::NAME.to_string()];

        valid.extend(
            [F1Weighted::NAME, F1Macro::NAME, GlobalAccuracy::NAME]
                .iter()
                .map(|name| (*name).to_string()),
        );

        for single in [F1Score::NAME, Precision::NAME, Recall::NAME, Accuracy::NAME] {
            if include_syntactic_sugar_metrics {
                valid.push(format!("{}(class=*)", single));
            }
            valid.push(format!("{}(class=<class value>)", single));
        }

        valid
    }
}

impl std::fmt::Display for ClassificationMetricSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_representation)
    }
}

impl std::fmt::Debug for ClassificationMetricSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassificationMetricSpecification")
            .field("string_representation", &self.string_representation)
            .finish()
    }
}

impl PartialEq for ClassificationMetricSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.string_representation == other.string_representation
    }
}

impl Eq for ClassificationMetricSpecification {}

impl std::hash::Hash for ClassificationMetricSpecification {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.string_representation.hash(state);
    }
}
