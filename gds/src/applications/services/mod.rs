pub mod logging;
pub mod pathfinding_dispatch;

// TS-JSON is a JSON-in / JSON-out protocol used at the kernel boundary.
// The core implementation is Rust-only (no N-API dependency).
#[path = "tsjson_napi.rs"]
pub mod tsjson;

// Optional Node/N-API adapter for the TS-JSON core.
#[cfg(feature = "node")]
pub mod tsjson_node;
