//! Miscellaneous algorithm procedure facades.
//!
//! This module mirrors the Java GDS "miscellaneous-algorithms" package.
//!
//! The intent is to provide a stable facade surface (stream/stats/mutate/write/estimate)
//! that the applications layer can depend on, while allowing the internals to evolve.

pub mod collapse_path;
pub mod index_inverse;
pub mod scale_properties;
pub mod indirect_exposure;
pub mod to_undirected;

pub use collapse_path::CollapsePathFacade;
pub use index_inverse::IndexInverseFacade;
pub use scale_properties::{ScalePropertiesFacade, ScalePropertiesStats, ScalePropertiesStreamRow};
pub use indirect_exposure::IndirectExposureFacade;
pub use to_undirected::{ToUndirectedFacade, ToUndirectedStats};
