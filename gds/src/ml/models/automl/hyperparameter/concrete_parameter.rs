use dyn_clone::DynClone;

/// Concrete parameter values as a tagged enum for type-safe handling.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ConcreteParameterValue {
    Integer(i32),
    Double(f64),
    String(String),
    IntList(Vec<i32>),
    DoubleList(Vec<f64>),
}

impl ConcreteParameterValue {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Integer(value) => serde_json::Value::Number((*value).into()),
            Self::Double(value) => serde_json::Number::from_f64(*value)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            Self::String(value) => serde_json::Value::String(value.clone()),
            Self::IntList(values) => serde_json::Value::Array(
                values
                    .iter()
                    .map(|value| serde_json::Value::Number((*value).into()))
                    .collect(),
            ),
            Self::DoubleList(values) => serde_json::Value::Array(
                values
                    .iter()
                    .map(|value| {
                        serde_json::Number::from_f64(*value)
                            .map(serde_json::Value::Number)
                            .unwrap_or(serde_json::Value::Null)
                    })
                    .collect(),
            ),
        }
    }
}

/// Trait for concrete parameter values
///
/// Java: `interface ConcreteParameter<T> { T value(); }`
pub trait ConcreteParameter: DynClone + Send + Sync {
    fn value(&self) -> ConcreteParameterValue;
}

dyn_clone::clone_trait_object!(ConcreteParameter);
