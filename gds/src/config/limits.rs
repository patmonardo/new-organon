use serde_json::Value;
use std::collections::HashMap;

/// Limit types supported (mirrors Java design: bool/double/long)
#[derive(Debug, Clone)]
pub enum Limit {
    Bool(bool),
    Double(f64),
    Long(i64),
}

impl Limit {
    /// Returns true when the provided value violates the limit.
    pub fn is_violated(&self, v: &Value) -> Result<bool, String> {
        match self {
            Limit::Bool(expected) => v
                .as_bool()
                .map(|b| b != *expected)
                .ok_or_else(|| "Type mismatch: expected bool".to_string()),
            Limit::Double(max) => v
                .as_f64()
                .map(|d| d > *max)
                .ok_or_else(|| "Type mismatch: expected number".to_string()),
            Limit::Long(max) => v
                .as_i64()
                .map(|l| l > *max)
                .ok_or_else(|| "Type mismatch: expected integer".to_string()),
        }
    }

    pub fn as_value(&self) -> Value {
        match self {
            Limit::Bool(b) => Value::Bool(*b),
            Limit::Double(d) => Value::Number(serde_json::Number::from_f64(*d).unwrap()),
            Limit::Long(l) => Value::Number(serde_json::Number::from(*l)),
        }
    }
}

/// Violation discovered when validating limits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimitViolation {
    pub message: String,
}

/// Manager containing global and personal limits
#[derive(Debug, Default, Clone)]
pub struct LimitsManager {
    global: HashMap<String, Limit>,
    personal: HashMap<String, HashMap<String, Limit>>,
}

impl LimitsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_global(&mut self, key: impl Into<String>, limit: Limit) {
        self.global.insert(key.into(), limit);
    }

    pub fn set_personal(
        &mut self,
        username: impl Into<String>,
        key: impl Into<String>,
        limit: Limit,
    ) {
        let user = username.into();
        let entry = self.personal.entry(user).or_insert_with(HashMap::new);
        entry.insert(key.into(), limit);
    }

    /// Validate a JSON object map against configured limits for username.
    pub fn validate(
        &self,
        obj: &serde_json::Map<String, Value>,
        username: Option<&str>,
    ) -> Vec<LimitViolation> {
        let mut violations = Vec::new();

        for (k, v) in obj.iter() {
            // personal first
            if let Some(user) = username {
                if let Some(m) = self.personal.get(user) {
                    if let Some(limit) = m.get(k) {
                        match limit.is_violated(v) {
                            Ok(true) => violations.push(LimitViolation {
                                message: limit.as_value().to_string() + "",
                            }),
                            Ok(false) => {}
                            Err(e) => violations.push(LimitViolation {
                                message: format!("{}: {}", k, e),
                            }),
                        }
                        continue;
                    }
                }
            }

            if let Some(limit) = self.global.get(k) {
                match limit.is_violated(v) {
                    Ok(true) => violations.push(LimitViolation {
                        message: limit.as_value().to_string() + "",
                    }),
                    Ok(false) => {}
                    Err(e) => violations.push(LimitViolation {
                        message: format!("{}: {}", k, e),
                    }),
                }
            }
        }

        violations
    }
}
