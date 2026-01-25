use gds::config::*;
use serde_json::json;

#[test]
fn defaults_apply_and_list() {
    let mut defaults = DefaultsManager::new();
    defaults.set_global("k1", json!("g"));
    defaults.set_personal("alice", "k2", json!(42));

    let mut obj = serde_json::Map::new();
    obj.insert("present".to_string(), json!(true));

    defaults.apply(&mut obj, Some("alice"));

    assert_eq!(obj.get("k1").unwrap(), &json!("g"));
    assert_eq!(obj.get("k2").unwrap(), &json!(42));
    assert_eq!(obj.get("present").unwrap(), &json!(true));
}

#[test]
fn limits_detect_violation_global_and_personal() {
    let mut limits = LimitsManager::new();
    limits.set_global("x", Limit::Long(10));
    limits.set_personal("bob", "y", Limit::Bool(false));

    let mut map = serde_json::Map::new();
    map.insert("x".to_string(), json!(20));
    map.insert("y".to_string(), json!(true));

    let viol_global = limits.validate(&map, None);
    assert!(!viol_global.is_empty());

    let viol_personal = limits.validate(&map, Some("bob"));
    assert!(!viol_personal.is_empty());
}
