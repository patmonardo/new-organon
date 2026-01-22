//! Graph-store-catalog wiring for task/progress registries.
//!
//! This module exists to mirror the Java dependency graph used by the
//! graph-store-catalog applications, but it must not introduce placeholder
//! progress types.

use std::sync::Arc;

use crate::core::utils::progress::{
    TaskRegistryFactories, TaskRegistryFactory as TaskRegistryFactoryTrait,
};
use crate::core::utils::warnings::{
    EmptyUserLogRegistryFactory, UserLogRegistryFactory as UserLogRegistryFactoryTrait,
};

/// Factory handle for creating [`crate::core::utils::progress::TaskRegistry`] instances.
pub type TaskRegistryFactory = Arc<dyn TaskRegistryFactoryTrait>;

/// Factory handle for creating [`crate::core::utils::warnings::UserLogRegistry`] instances.
pub type UserLogRegistryFactory = Arc<dyn UserLogRegistryFactoryTrait>;

pub fn default_task_registry_factory() -> TaskRegistryFactory {
    TaskRegistryFactories::empty()
}

pub fn default_user_log_registry_factory() -> UserLogRegistryFactory {
    Arc::new(EmptyUserLogRegistryFactory::new())
}
