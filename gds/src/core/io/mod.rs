//! IO helpers for loading/saving graph data.
//!
//! Keep IO concerns separate from projection factories: IO produces in-memory
//! tables/structures that factories consume. This module houses lightweight
//! readers/writers (e.g., Arrow IPC/Parquet) without coupling to factory logic.

pub mod arrow;
