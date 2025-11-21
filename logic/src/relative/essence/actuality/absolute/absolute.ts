import type { Chunk, LogicalOperation } from '../../../types';

/**
 * CHAPTER 1: THE ABSOLUTE — Complete Structure
 *
 * NOTE: This is Chapter 1 of Section III Actuality. The absolute is a Species
 * of the Genus Actuality. Each Chapter (Species) has three sub-species that make up
 * its Concept:
 *
 * Structure:
 * Introduction: The absolute as indeterminate identity, exposition not determination
 * A. THE EXPOSITION OF THE ABSOLUTE (Species 1)
 * B. THE ABSOLUTE ATTRIBUTE (Species 2)
 * C. THE MODE OF THE ABSOLUTE (Species 3)
 *
 * Transition: The absolute passes over to actuality proper
 */

// ============================================================================
// INTRODUCTION: CHAPTER 1 — THE ABSOLUTE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'abs-intro-1-simple-solid-identity',
    title: 'Simple solid identity — indeterminate, contradiction',
    text: `The simple solid identity of the absolute is indeterminate,
or rather, every determinateness of essence and concrete existence,
or of being in general as well as of reflection,
has dissolved itself into it.
Accordingly, the determining of what is
the absolute appears to be a negating,
and the absolute itself appears only as
the negation of all predicates, as the void.
But since it must equally be spoken of
as the position of all predicates,
it appears as the most formal of contradictions.
In so far as that negating and this positing
belong to external reflection,
what we have is a formal, unsystematic dialectic
that has an easy time picking up
a variety of determinations here and there,
and is just as at ease demonstrating, on the one hand,
their finitude and relativity, as declaring, on the other,
that the absolute, which it vaguely envisages as totality,
is the dwelling place of all determinations,
yet is incapable of raising
either the positions or the negations
to a true unity.`,
    summary: 'Simple solid identity = indeterminate. Every determinateness dissolved into it. Determining appears as negating, absolute = negation of all predicates = void. But also position of all predicates = formal contradiction. External reflection = formal unsystematic dialectic, picks up determinations, demonstrates finitude/relativity, declares absolute as dwelling place, but cannot raise to true unity.'
  },
  {
    id: 'abs-intro-2-exposition-not-determination',
    title: 'Exposition not determination — absolute\'s own exposition',
    text: `The task is indeed to demonstrate what the absolute is.
But this demonstration cannot be either
a determining or an external reflection
by virtue of which determinations
of the absolute would result,
but is rather the exposition of the absolute,
more precisely the absolute's own exposition,
and only a displaying of what it is.`,
    summary: 'Task = demonstrate what absolute is. But demonstration not = determining or external reflection. Rather = exposition of absolute, absolute\'s own exposition, displaying what it is.'
  },
];

// ============================================================================
// INTRODUCTION OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'abs-op-intro-1-simple-solid-identity',
    chunkId: 'abs-intro-1-simple-solid-identity',
    label: 'Simple solid identity — indeterminate, contradiction',
    clauses: [
      'simpleSolidIdentity = indeterminate',
      'everyDeterminateness.dissolvedInto = absolute',
      'determining.appearsAs = negating',
      'absolute.appearsAs = negationOfAllPredicates',
      'absolute.appearsAs = void',
      'butAlso = positionOfAllPredicates',
      'appearsAs = formalContradiction',
      'negatingPositing.belongTo = externalReflection',
      'externalReflection = formalUnsystematicDialectic',
      'dialectic.picksUp = varietyOfDeterminations',
      'dialectic.demonstrates = {finitude, relativity}',
      'dialectic.declares = absoluteAsDwellingPlace',
      'dialectic.incapableOf = raisingToTrueUnity',
    ],
    predicates: [
      { name: 'IsIndeterminate', args: ['identity'] },
      { name: 'IsFormalContradiction', args: ['absolute'] },
    ],
    relations: [
      { predicate: 'is', from: 'identity', to: 'indeterminate' },
    ],
  },
  {
    id: 'abs-op-intro-2-exposition-not-determination',
    chunkId: 'abs-intro-2-exposition-not-determination',
    label: 'Exposition not determination — absolute\'s own exposition',
    clauses: [
      'task = demonstrateWhatAbsoluteIs',
      'demonstration.not = determining',
      'demonstration.not = externalReflection',
      'demonstration = expositionOfAbsolute',
      'exposition = absolutesOwnExposition',
      'exposition = displayingWhatItIs',
    ],
    predicates: [
      { name: 'IsExposition', args: ['demonstration'] },
    ],
    relations: [
      { predicate: 'is', from: 'demonstration', to: 'exposition' },
    ],
  },
];

// Re-export species modules
// Note: For aggregated exports, use index.ts
// Individual species modules are available for granular access

