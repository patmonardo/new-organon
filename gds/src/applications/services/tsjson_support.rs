use once_cell::sync::Lazy;

use crate::applications::graph_store_catalog::loaders::{
    GraphStoreCatalogService, PerUserDbGraphStoreCatalogService,
};
use crate::core::User;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;

use serde_json::Value;
use std::sync::Arc;

pub(super) static TSJSON_CATALOG_SERVICE: Lazy<Arc<PerUserDbGraphStoreCatalogService>> =
    Lazy::new(|| Arc::new(PerUserDbGraphStoreCatalogService::new()));

#[derive(Clone, Debug)]
pub(super) struct TsjsonUser {
    username: String,
    roles: Vec<String>,
    permissions: Vec<String>,
}

impl TsjsonUser {
    pub(super) fn new(username: String, is_admin: bool) -> Self {
        Self {
            username,
            roles: if is_admin {
                vec!["admin".to_string()]
            } else {
                vec![]
            },
            permissions: vec![],
        }
    }
}

impl User for TsjsonUser {
    fn username(&self) -> &str {
        &self.username
    }
    fn roles(&self) -> &[String] {
        &self.roles
    }
    fn is_authenticated(&self) -> bool {
        true
    }
    fn permissions(&self) -> &[String] {
        &self.permissions
    }
}

pub(super) fn ok(op: &str, data: Value) -> Value {
    serde_json::json!({
        "ok": true,
        "op": op,
        "data": data,
    })
}

pub(super) fn err(op: &str, code: &str, message: impl Into<String>) -> Value {
    serde_json::json!({
        "ok": false,
        "op": op,
        "error": {
            "code": code,
            "message": message.into(),
        }
    })
}

pub(super) struct FacadeContext {
    pub(super) op: String,
    pub(super) user: TsjsonUser,
    pub(super) db: DatabaseId,
    pub(super) catalog: Arc<dyn GraphCatalog>,
}

pub(super) fn parse_facade_context(request: &Value) -> Result<FacadeContext, Value> {
    let op = request
        .get("op")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    if op.is_empty() {
        return Err(err("", "INVALID_REQUEST", "Missing required field: op"));
    }

    let user_obj = request
        .get("user")
        .and_then(|v| v.as_object())
        .ok_or_else(|| err(&op, "INVALID_REQUEST", "Missing required field: user"))?;

    let username = user_obj
        .get("username")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| err(&op, "INVALID_REQUEST", "user.username missing or empty"))?
        .to_string();

    let is_admin = user_obj
        .get("isAdmin")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let user = TsjsonUser::new(username, is_admin);

    let database_id = request
        .get("databaseId")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| err(&op, "INVALID_REQUEST", "databaseId missing or empty"))?;

    let db = DatabaseId::new(database_id);

    let catalog = TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db);

    Ok(FacadeContext {
        op,
        user,
        db,
        catalog,
    })
}
