use serde_json::{Map, Value};
use std::collections::HashMap;

/// A manager for global and per-user defaults.
#[derive(Debug, Default, Clone)]
pub struct DefaultsManager {
    global: HashMap<String, Value>,
    personal: HashMap<String, HashMap<String, Value>>,
}

impl DefaultsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_global(&mut self, key: impl Into<String>, value: Value) {
        self.global.insert(key.into(), value);
    }

    pub fn set_personal(
        &mut self,
        username: impl Into<String>,
        key: impl Into<String>,
        value: Value,
    ) {
        let user = username.into();
        let entry = self.personal.entry(user).or_insert_with(HashMap::new);
        entry.insert(key.into(), value);
    }

    pub fn list(&self, username: Option<&str>, key: Option<&str>) -> Map<String, Value> {
        let mut out: Map<String, Value> = Map::new();

        // start with global defaults
        for (k, v) in &self.global {
            out.insert(k.clone(), v.clone());
        }

        // overlay personal defaults
        if let Some(user) = username {
            if let Some(m) = self.personal.get(user) {
                for (k, v) in m {
                    out.insert(k.clone(), v.clone());
                }
            }
        }

        if let Some(k) = key {
            if let Some(v) = out.get(k) {
                let mut m = Map::new();
                m.insert(k.to_string(), v.clone());
                return m;
            }
            return Map::new();
        }

        out
    }

    /// Apply defaults to a mutable JSON object map. Values are only set when missing.
    pub fn apply(&self, obj: &mut Map<String, Value>, username: Option<&str>) {
        // personal overrides global
        if let Some(user) = username {
            if let Some(m) = self.personal.get(user) {
                for (k, v) in m {
                    obj.entry(k.clone()).or_insert_with(|| v.clone());
                }
            }
        }

        for (k, v) in &self.global {
            obj.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
}
