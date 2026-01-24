use super::config::is_log_scale_parameter;
use super::config::non_numeric_parameter_type;
use super::config::NonNumericParameterType;
use super::hyperparameter::ConcreteParameter;
use super::hyperparameter::DoubleParameter;
use super::hyperparameter::DoubleRangeParameter;
use super::hyperparameter::IntegerParameter;
use super::hyperparameter::IntegerRangeParameter;
use super::hyperparameter::ListParameter;
use super::hyperparameter::ListParameterValue;
use super::hyperparameter::StringParameter;
use crate::core::utils::format_with_locale;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RangeParameters {
    double_ranges: HashMap<String, DoubleRangeParameter>,
    integer_ranges: HashMap<String, IntegerRangeParameter>,
}

impl RangeParameters {
    pub fn double_ranges(&self) -> &HashMap<String, DoubleRangeParameter> {
        &self.double_ranges
    }

    pub fn integer_ranges(&self) -> &HashMap<String, IntegerRangeParameter> {
        &self.integer_ranges
    }
}

pub fn parse_range_parameters(
    input: &HashMap<String, serde_json::Value>,
) -> Result<RangeParameters, String> {
    let mut double_ranges = HashMap::new();
    let mut integer_ranges = HashMap::new();
    let mut incorrect_parameters: HashMap<String, serde_json::Value> = HashMap::new();
    let mut incorrect_maps: HashMap<String, serde_json::Value> = HashMap::new();

    for (key, value) in input {
        if let serde_json::Value::Object(map) = value {
            if non_numeric_parameter_type(key).is_some() {
                incorrect_parameters.insert(key.clone(), value.clone());
                continue;
            }

            if map.keys().collect::<Vec<_>>() != vec!["range"] {
                incorrect_maps.insert(key.clone(), value.clone());
                continue;
            }

            let range_value = map.get("range");
            let range = match range_value {
                Some(serde_json::Value::Array(values)) => values,
                _ => {
                    incorrect_maps.insert(key.clone(), value.clone());
                    continue;
                }
            };

            if range.len() != 2 {
                incorrect_maps.insert(key.clone(), value.clone());
                continue;
            }

            let min_object = &range[0];
            let max_object = &range[1];

            if !type_is_supported_in_range(min_object) || !type_is_supported_in_range(max_object) {
                incorrect_maps.insert(key.clone(), value.clone());
                continue;
            }

            let min_is_float = is_float_or_double(min_object);
            let max_is_float = is_float_or_double(max_object);
            if min_is_float || max_is_float {
                let min = min_object.as_f64().unwrap();
                let max = max_object.as_f64().unwrap();
                let log_scale = is_log_scale_parameter(key);
                double_ranges.insert(
                    key.clone(),
                    DoubleRangeParameter::of_with_log_scale(min, max, log_scale),
                );
                continue;
            }

            let min = min_object.as_i64().unwrap() as i32;
            let max = max_object.as_i64().unwrap() as i32;
            integer_ranges.insert(key.clone(), IntegerRangeParameter::of(min, max));
        }
    }

    if !incorrect_parameters.is_empty() {
        let rendered = incorrect_parameters
            .iter()
            .map(|(key, value)| {
                let type_name = match non_numeric_parameter_type(key) {
                    Some(NonNumericParameterType::String) => "String",
                    Some(NonNumericParameterType::List) => "List",
                    None => "Unknown",
                };
                format!("`{}` (`{}` is of type {})", value, key, type_name)
            })
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format_with_locale(
            "The following parameters have been given the wrong type: [%s]",
            &[rendered],
        ));
    }

    if !incorrect_maps.is_empty() {
        let rendered = incorrect_maps
            .iter()
            .map(|(_, value)| format!("`{}`", value))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format_with_locale(
            "Ranges for training hyper-parameters must be of the form {range: {min, max}}, where both min and max are numerical. Invalid parameters: [%s]",
            &[rendered],
        ));
    }

    Ok(RangeParameters {
        double_ranges,
        integer_ranges,
    })
}

fn type_is_supported_in_range(value: &serde_json::Value) -> bool {
    matches!(value, serde_json::Value::Number(_))
}

fn is_float_or_double(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Number(number) => number.as_f64().is_some() && number.as_i64().is_none(),
        _ => false,
    }
}

pub fn parse_concrete_parameters(
    input: &HashMap<String, serde_json::Value>,
) -> Result<HashMap<String, Box<dyn ConcreteParameter>>, String> {
    let mut out = HashMap::new();
    for (key, value) in input {
        if matches!(value, serde_json::Value::Object(_)) {
            continue;
        }
        out.insert(key.clone(), parse_concrete_parameter(key, value)?);
    }
    Ok(out)
}

fn parse_concrete_parameter(
    key: &str,
    value: &serde_json::Value,
) -> Result<Box<dyn ConcreteParameter>, String> {
    if non_numeric_parameter_type(key).is_some() {
        parse_concrete_non_numeric_parameter(key, value)
    } else {
        parse_concrete_numeric_parameter(key, value)
    }
}

fn parse_concrete_non_numeric_parameter(
    key: &str,
    value: &serde_json::Value,
) -> Result<Box<dyn ConcreteParameter>, String> {
    match non_numeric_parameter_type(key) {
        Some(NonNumericParameterType::String) => match value {
            serde_json::Value::String(text) => Ok(Box::new(StringParameter::of(text.clone()))),
            _ => Err(format_with_locale(
                "Parameter `%s` must be of the type `%s`.",
                &[key, "String"],
            )),
        },
        Some(NonNumericParameterType::List) => match value {
            serde_json::Value::Array(values) => {
                if matches!(key, "hiddenLayerSizes" | "hidden_layer_sizes") {
                    let mut out = Vec::new();
                    for value in values {
                        let val = value.as_i64().ok_or_else(|| {
                            format_with_locale(
                                "Parameter `%s` must be of the type `%s`.",
                                &[key, "List"],
                            )
                        })?;
                        out.push(val as i32);
                    }
                    Ok(Box::new(ListParameter::of(ListParameterValue::Int(out))))
                } else if matches!(key, "classWeights" | "class_weights") {
                    let mut out = Vec::new();
                    for value in values {
                        let val = value
                            .as_f64()
                            .or_else(|| value.as_i64().map(|v| v as f64))
                            .ok_or_else(|| {
                                format_with_locale(
                                    "Parameter `%s` must be of the type `%s`.",
                                    &[key, "List"],
                                )
                            })?;
                        out.push(val);
                    }
                    Ok(Box::new(ListParameter::of(ListParameterValue::Double(out))))
                } else {
                    Err(format_with_locale(
                        "Was not able to resolve type of parameter `%s`.",
                        &[key],
                    ))
                }
            }
            _ => Err(format_with_locale(
                "Parameter `%s` must be of the type `%s`.",
                &[key, "List"],
            )),
        },
        None => Err(format_with_locale(
            "Was not able to resolve type of parameter `%s`.",
            &[key],
        )),
    }
}

fn parse_concrete_numeric_parameter(
    key: &str,
    value: &serde_json::Value,
) -> Result<Box<dyn ConcreteParameter>, String> {
    match value {
        serde_json::Value::Number(number) => {
            if let Some(integer) = number.as_i64() {
                Ok(Box::new(IntegerParameter::of(integer as i32)))
            } else if let Some(double) = number.as_f64() {
                Ok(Box::new(DoubleParameter::of(double)))
            } else {
                Err(format_with_locale(
                    "Parameter `%s` must be numeric or a map of the form {range: [min, max]}.",
                    &[key],
                ))
            }
        }
        _ => Err(format_with_locale(
            "Parameter `%s` must be numeric or a map of the form {range: [min, max]}.",
            &[key],
        )),
    }
}
