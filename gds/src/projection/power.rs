//! Projection power: Concealing vs Revealing.
//!
//! In this repo, “Projection” has two complementary meanings:
//!
//! - **Concealing power** (Factory): external/native source → Image (`GraphStore`)
//! - **Revealing power** (Eval): Image (`GraphStore`) → derived result
//!
//! This module provides small, stable vocabulary types that let code and docs
//! agree on what “projection” is referring to.

/// Which side of Projection is being exercised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionPower {
    /// Produce an internal Image while withholding source contingencies.
    Concealing,
    /// Make explicit what is implicit in an Image.
    Revealing,
}

/// Where in the runtime the projection act occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionLayer {
    /// `projection/factory/**`
    Factory,
    /// `projection/eval/**`
    Eval,
}

/// Optional intent tag that can be used by higher layers.
///
/// This mirrors the conceptual mapping used in the Logic package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionIntent {
    /// Reveal previously implicit structure as a new construct.
    Creation,
    /// Conceal details while preserving identity.
    Preservation,
    /// Conceal by revoking/removing.
    Destruction,
}

/// A small tag that can be attached to traces/events.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionTag {
    pub power: ProjectionPower,
    pub layer: ProjectionLayer,
    pub intent: Option<ProjectionIntent>,
}

impl ProjectionTag {
    pub const fn concealing_factory() -> Self {
        Self {
            power: ProjectionPower::Concealing,
            layer: ProjectionLayer::Factory,
            intent: None,
        }
    }

    pub const fn revealing_eval() -> Self {
        Self {
            power: ProjectionPower::Revealing,
            layer: ProjectionLayer::Eval,
            intent: None,
        }
    }
}
