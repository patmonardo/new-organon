//! Miscellaneous algorithm dispatch handlers.
//!
//! Mirrors the Java `miscellaneous-algorithms` application surface.

pub mod collapse_path;
pub mod index_inverse;
pub mod scale_properties;
pub mod indirect_exposure;
pub mod to_undirected;

pub use collapse_path::handle_collapse_path;
pub use index_inverse::handle_index_inverse;
pub use scale_properties::handle_scale_properties;
pub use indirect_exposure::handle_indirect_exposure;
pub use to_undirected::handle_to_undirected;
