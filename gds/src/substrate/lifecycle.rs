use crate::substrate::{CoreGraphStore, SubstrateResult};

/// Import capability: build a graph store from some source/config.
pub trait Import {
    type Source;
    type Config;

    fn import(&self, source: &Self::Source, config: &Self::Config) -> SubstrateResult<CoreGraphStore>;
}

/// Serve capability: provide stable read-only views/handles over a store.
///
/// Roots/GC/pinning semantics belong here when Huge substrate arrives.
pub trait Serve {
    type Graph;

    fn union(&self, store: &CoreGraphStore) -> SubstrateResult<Self::Graph>;
}

/// Export capability: materialize a store into an external format.
pub trait Export {
    type Target;
    type Config;

    fn export(&self, store: &CoreGraphStore, target: &Self::Target, config: &Self::Config) -> SubstrateResult<()>;
}
