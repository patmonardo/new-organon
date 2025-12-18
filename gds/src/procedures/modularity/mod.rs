//! Modularity: community quality metric based on edge density
//!
//! Measures the quality of a division of a network into communities
//! by comparing actual edges within communities to expected edges
//! if the network were random.
//!
//! Formula: Q = Σ[(ec - Kc²/2m) / 2m]
//! - ec: edges within community c
//! - Kc: sum of degrees in community c
//! - m: total edges
//!
//! Higher modularity (closer to 1) indicates stronger community structure.

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;
