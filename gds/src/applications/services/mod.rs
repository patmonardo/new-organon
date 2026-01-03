pub mod algorithms_dispatch;
pub mod applications_dispatch;
pub mod graph_store_catalog_dispatch;
pub mod logging;
mod graph_store_dispatch;
mod tsjson_support;

// TS-JSON is a JSON-in / JSON-out protocol used at the kernel boundary.
// The core implementation is Rust-only (no N-API dependency).
#[path = "tsjson_napi.rs"]
pub mod tsjson;

// Optional Node/N-API adapter for the TS-JSON core.
#[cfg(feature = "node")]
pub mod tsjson_node;
