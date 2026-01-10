pub mod algorithms_dispatch;
pub mod applications_dispatch;
pub mod graph_store_catalog_dispatch;
pub mod logging;

pub mod graph_store_dispatch;
pub mod tsjson_support;

// TS-JSON is a JSON-in / JSON-out protocol used at the kernel boundary.
// The core implementation is Rust-only (no N-API dependency) and is the single
// supported boundary for external callers.
#[path = "tsjson_napi.rs"]
pub mod tsjson;

// Optional Node/N-API adapter for the TS-JSON core.
#[cfg(feature = "node")]
pub mod tsjson_node;
