use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::core::User;
use crate::types::catalog::{
    CatalogError, Dropped, GraphCatalog, GraphMemoryUsage, InMemoryGraphCatalog, ListEntry,
};
use crate::types::graph_store::{DatabaseId, DefaultGraphStore};

use super::GraphStoreCatalogService;

/// Type alias for the complex catalog registry type.
/// Maps (username, database_name) to graph catalogs in a thread-safe manner.
type UserDbCatalogRegistry = Arc<RwLock<HashMap<(String, String), Arc<InMemoryGraphCatalog>>>>;

/// In-memory `GraphStoreCatalogService` that scopes catalogs by (username, databaseId).
///
/// This is an intentional **mock substrate** for the applications layer, used while the
/// real core-loading system is still being ported/implemented.
///
/// Java parity notes:
/// - Java core uses a global `GraphStoreCatalog` keyed by (username, databaseName, graphName).
/// - Admin users can list/search "across users"; non-admin users are typically scoped.
///
/// Here we model that by returning:
/// - a per-user-per-db `InMemoryGraphCatalog` for non-admin users
/// - a merged per-db `GraphCatalog` view for admin users (list/search across user catalogs)
#[derive(Clone, Default)]
pub struct PerUserDbGraphStoreCatalogService {
    catalogs: UserDbCatalogRegistry,
}

impl PerUserDbGraphStoreCatalogService {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_admin(user: &dyn User) -> bool {
        user.roles().iter().any(|r| r == "admin")
    }

    fn ensure_user_catalog(
        catalogs: &UserDbCatalogRegistry,
        username: &str,
        database_id: &DatabaseId,
    ) -> Arc<InMemoryGraphCatalog> {
        let key = (
            username.to_string(),
            database_id.database_name().to_string(),
        );
        // Fast path: read lock.
        if let Ok(map) = catalogs.read() {
            if let Some(existing) = map.get(&key) {
                return existing.clone();
            }
        }
        // Slow path: write lock.
        let mut map = catalogs.write().expect("catalogs poisoned");
        map.entry(key)
            .or_insert_with(|| Arc::new(InMemoryGraphCatalog::new()))
            .clone()
    }
}

impl GraphStoreCatalogService for PerUserDbGraphStoreCatalogService {
    fn graph_catalog(&self, user: &dyn User, database_id: &DatabaseId) -> Arc<dyn GraphCatalog> {
        if Self::is_admin(user) {
            Arc::new(MergedDbCatalog {
                catalogs: self.catalogs.clone(),
                owner_username: user.username().to_string(),
                database_id: database_id.clone(),
            })
        } else {
            Self::ensure_user_catalog(&self.catalogs, user.username(), database_id)
        }
    }
}

/// A merged `GraphCatalog` view over all user catalogs for a single database.
///
/// - `list(...)` aggregates across all usernames for `database_id`
/// - `set(...)` writes into the *owner* user's catalog (the one that requested the view)
/// - `get(...)`/`size_of(...)`/`drop(...)` search across all usernames for `database_id`
struct MergedDbCatalog {
    catalogs: UserDbCatalogRegistry,
    owner_username: String,
    database_id: DatabaseId,
}

impl MergedDbCatalog {
    fn user_db_matches((_, db): &(String, String), database_id: &DatabaseId) -> bool {
        db.as_str() == database_id.database_name()
    }

    fn owner_catalog(&self) -> Arc<InMemoryGraphCatalog> {
        PerUserDbGraphStoreCatalogService::ensure_user_catalog(
            &self.catalogs,
            &self.owner_username,
            &self.database_id,
        )
    }
}

impl GraphCatalog for MergedDbCatalog {
    fn set(&self, name: &str, store: Arc<DefaultGraphStore>) {
        self.owner_catalog().set(name, store);
    }

    fn get(&self, name: &str) -> Option<Arc<DefaultGraphStore>> {
        let map = self.catalogs.read().ok()?;
        for ((_, db), catalog) in map.iter() {
            if db.as_str() != self.database_id.database_name() {
                continue;
            }
            if let Some(store) = catalog.get(name) {
                return Some(store);
            }
        }
        None
    }

    fn drop(&self, names: &[&str], fail_if_missing: bool) -> Result<Vec<Dropped>, CatalogError> {
        let map = self.catalogs.read().expect("catalogs poisoned");
        let catalogs_for_db: Vec<Arc<InMemoryGraphCatalog>> = map
            .iter()
            .filter(|(k, _)| Self::user_db_matches(k, &self.database_id))
            .map(|(_, v)| v.clone())
            .collect();

        let mut all_dropped: Vec<Dropped> = Vec::new();
        for n in names {
            let mut dropped_any = false;
            for cat in catalogs_for_db.iter() {
                let dropped = GraphCatalog::drop(cat.as_ref(), &[*n], false)?;
                if !dropped.is_empty() {
                    dropped_any = true;
                    all_dropped.extend(dropped);
                }
            }
            if fail_if_missing && !dropped_any {
                return Err(CatalogError::NotFound((*n).to_string()));
            }
        }
        Ok(all_dropped)
    }

    fn list(&self, filter: Option<&str>, include_degree_dist: bool) -> Vec<ListEntry> {
        let map = self.catalogs.read().expect("catalogs poisoned");
        let mut out: Vec<ListEntry> = Vec::new();
        for ((_, db), catalog) in map.iter() {
            if db.as_str() != self.database_id.database_name() {
                continue;
            }
            out.extend(catalog.list(filter, include_degree_dist));
        }
        out
    }

    fn size_of(&self, name: &str) -> Result<GraphMemoryUsage, CatalogError> {
        let map = self.catalogs.read().expect("catalogs poisoned");
        for ((_, db), catalog) in map.iter() {
            if db.as_str() != self.database_id.database_name() {
                continue;
            }
            if catalog.get(name).is_some() {
                return catalog.size_of(name);
            }
        }
        Err(CatalogError::NotFound(name.to_string()))
    }
}
