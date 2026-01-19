//! Procedure registry for node property step execution.
//!
//! Provides a lightweight mapping from normalized procedure names
//! (e.g. `gds.pagerank.mutate`) to executable handlers. This mirrors
//! Java's mutate-mode registry/stubs without adopting the same naming.

use crate::projection::eval::pipeline::node_property_step::{
    DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE, DEGREE_CENTRALITY_MUTATE, FASTRP_MUTATE, HARMONIC_MUTATE,
    HITS_MUTATE, PAGERANK_MUTATE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcedureKind {
    DebugWriteConstantDouble,
    PageRankMutate,
    FastRPMutate,
    DegreeCentralityMutate,
    HitsMutate,
    HarmonicMutate,
}

pub struct ProcedureRegistry;

impl ProcedureRegistry {
    pub fn resolve(name: &str) -> Option<ProcedureKind> {
        match name {
            DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE => Some(ProcedureKind::DebugWriteConstantDouble),
            PAGERANK_MUTATE => Some(ProcedureKind::PageRankMutate),
            FASTRP_MUTATE => Some(ProcedureKind::FastRPMutate),
            DEGREE_CENTRALITY_MUTATE => Some(ProcedureKind::DegreeCentralityMutate),
            HITS_MUTATE => Some(ProcedureKind::HitsMutate),
            HARMONIC_MUTATE => Some(ProcedureKind::HarmonicMutate),
            _ => None,
        }
    }

    pub fn supported_names() -> &'static [&'static str] {
        &[
            DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE,
            PAGERANK_MUTATE,
            FASTRP_MUTATE,
            DEGREE_CENTRALITY_MUTATE,
            HITS_MUTATE,
            HARMONIC_MUTATE,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_supported() {
        assert_eq!(
            ProcedureRegistry::resolve(DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE),
            Some(ProcedureKind::DebugWriteConstantDouble)
        );
        assert_eq!(
            ProcedureRegistry::resolve(PAGERANK_MUTATE),
            Some(ProcedureKind::PageRankMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(FASTRP_MUTATE),
            Some(ProcedureKind::FastRPMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(DEGREE_CENTRALITY_MUTATE),
            Some(ProcedureKind::DegreeCentralityMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(HITS_MUTATE),
            Some(ProcedureKind::HitsMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(HARMONIC_MUTATE),
            Some(ProcedureKind::HarmonicMutate)
        );
    }

    #[test]
    fn test_resolve_unknown() {
        assert_eq!(ProcedureRegistry::resolve("gds.unknown.mutate"), None);
    }
}
