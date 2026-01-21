use std::collections::HashMap;

use serde_json::json;

use crate::ml::models::TrainingMethod;
use crate::procedures::pipelines::types::{AnyMap, AnyMapList};

pub fn default_split_config() -> AnyMap {
    let mut map = AnyMap::new();
    map.insert("testFraction".to_string(), json!(0.3));
    map.insert("validationFolds".to_string(), json!(3));
    map
}

pub fn default_param_config() -> HashMap<String, AnyMapList> {
    let mut map = HashMap::new();
    map.insert(TrainingMethod::LogisticRegression.to_string(), vec![]);
    map.insert(
        TrainingMethod::RandomForestClassification.to_string(),
        vec![],
    );
    map.insert(TrainingMethod::MLPClassification.to_string(), vec![]);
    map
}
