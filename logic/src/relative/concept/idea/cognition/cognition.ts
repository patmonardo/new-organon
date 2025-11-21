import type { Chunk, LogicalOperation } from '../../../types';

/**
 * CHAPTER 2: THE IDEA OF COGNITION — Complete Structure
 * 
 * NOTE: This is Chapter 2 of Section III The Idea. Cognition is a Species
 * of the Genus Idea. Each Chapter (Species) has sub-species that make up
 * its Concept:
 * 
 * Structure:
 * Introduction: Cognition as judgment, concept liberated into universality
 * A. THE IDEA OF THE TRUE (Species 1) — theoretical idea
 * B. THE IDEA OF THE GOOD (Species 2) — practical idea
 * 
 * Transition: The idea of the good transitions to the absolute idea
 */

// ============================================================================
// INTRODUCTION: CHAPTER 2 — THE IDEA OF COGNITION
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'cognition-ch2-intro-1-judgment',
    title: 'Cognition as judgment — concept liberated',
    text: `Life is the immediate idea, or the idea as
its still internally unrealized concept.
In its judgment, the idea is cognition in general.

The concept is for itself as concept
inasmuch as it freely and concretely exists
as abstract universality or a genus.
As such, it is its pure self-identity
that internally differentiates itself
in such a way that the differentiated is
not an objectivity but is rather
equally liberated into subjectivity
or into the form of simple self-equality;
consequently, the object facing
the concept is the concept itself.
Its reality in general is the form of its existence;
all depends on the determination of this form;
on it rests the difference between
what the concept is in itself, or as subjective,
and what it is when immersed in objectivity,
and then in the idea of life.
In this last, the concept is indeed distinguished
from its external reality and posited for itself;
however, this being-for-itself which it now has,
it has only as an identity that refers to itself
as immersed in the objectivity subjugated to it,
or to itself as indwelling, substantial form.
The elevation of the concept above life consists in this,
that its reality is the concept-form liberated into universality.
Through this judgment the idea is doubled,
into the subjective concept whose reality is the concept itself,
and the objective concept which is as life.
Thought, spirit, self-consciousness, are determinations
of the idea inasmuch as the latter has itself
as the subject matter, and its existence, that is, the determinateness
of its being, is its own difference from itself.`,
    summary: 'Life = immediate idea = internally unrealized concept. In judgment, idea = cognition. Concept for itself = abstract universality/genus, pure self-identity, internally differentiates, liberated into subjectivity. Object = concept itself. Reality = form of existence. Difference: concept in itself/subjective vs immersed in objectivity/life. Elevation above life = concept-form liberated into universality. Judgment doubles idea: subjective concept (reality = concept itself) + objective concept (life). Thought/spirit/self-consciousness = determinations of idea having itself as subject matter, existence = own difference from itself.'
  },
];

// ============================================================================
// INTRODUCTION OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'cognition-op-intro-1-judgment',
    chunkId: 'cognition-ch2-intro-1-judgment',
    label: 'Cognition as judgment',
    clauses: [
      'life = immediateIdea',
      'life = internallyUnrealizedConcept',
      'inJudgment.idea = cognition',
      'conceptForItself = abstractUniversality',
      'conceptForItself = genus',
      'conceptForItself = pureSelfIdentity',
      'concept.differentiates = internally',
      'differentiated = liberatedIntoSubjectivity',
      'object = conceptItself',
      'reality = formOfExistence',
      'elevationAboveLife = conceptFormLiberatedIntoUniversality',
      'judgment.doublesIdea = true',
      'doubledIdea = {subjectiveConcept, objectiveConcept}',
      'subjectiveConcept.reality = conceptItself',
      'objectiveConcept = life',
      'thought = determinationOfIdea',
      'spirit = determinationOfIdea',
      'selfConsciousness = determinationOfIdea',
      'idea.hasItselfAs = subjectMatter',
      'existence = ownDifferenceFromItself',
    ],
    predicates: [
      { name: 'IsCognition', args: ['ideaInJudgment'] },
      { name: 'IsLiberated', args: ['conceptForm'] },
    ],
    relations: [
      { predicate: 'is', from: 'ideaInJudgment', to: 'cognition' },
      { predicate: 'doublesInto', from: 'idea', to: ['subjectiveConcept', 'objectiveConcept'] },
    ],
  },
];

