//! Triadic moments for Projection/Eval.
//!
//! This file is intentionally *conceptual and type-level*.
//! Execution is implemented by:
//! - Procedure (Assertion / Universal)
//! - ML Pipeline (Problematic / Particular)
//! - Form Processor (Apodictic / Singular)
//!
//! The Form ISA consumes the Problematic artifact and returns a Certain ResultStore.

/// Kant/Fichte modality moments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModalMoment {
    /// Assertoric (fact-of-the-matter assertion)
    Assertion,
    /// Problematic (hypothesis / model-mediated)
    Problematic,
    /// Apodictic (necessitated certainty)
    Apodictic,
}

/// Concept moments (Universal → Particular → Singular).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConceptMoment {
    Universal,
    Particular,
    Singular,
}

impl ModalMoment {
    pub fn concept_moment(self) -> ConceptMoment {
        match self {
            ModalMoment::Assertion => ConceptMoment::Universal,
            ModalMoment::Problematic => ConceptMoment::Particular,
            ModalMoment::Apodictic => ConceptMoment::Singular,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modality_maps_to_concept_moment() {
        assert_eq!(ModalMoment::Assertion.concept_moment(), ConceptMoment::Universal);
        assert_eq!(ModalMoment::Problematic.concept_moment(), ConceptMoment::Particular);
        assert_eq!(ModalMoment::Apodictic.concept_moment(), ConceptMoment::Singular);
    }
}
