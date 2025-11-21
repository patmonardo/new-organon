import type { Chunk, LogicalOperation } from '../../types';

/**
 * SECTION III: ACTUALITY — Genus
 * 
 * NOTE: This is Section III Actuality, the Genus containing three Chapters:
 * Chapter 1: The absolute
 * Chapter 2: Actuality (proper)
 * Chapter 3: The absolute relation (substance)
 * 
 * Structure:
 * Introduction: Actuality as unity of essence and concrete existence
 * First: The absolute as such
 * Second: Actuality proper (possibility, necessity)
 * Third: The absolute relation (substance)
 */

// ============================================================================
// SECTION III: ACTUALITY — INTRODUCTION
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'act-intro-1-actuality-unity',
    title: 'Actuality as unity of essence and concrete existence',
    text: `Actuality is the unity of essence and concrete existence;
in it, shapeless essence and unstable appearance
(subsistence without determination
and manifoldness without permanence)
have their truth.
Although concrete existence is the immediacy
that has proceeded from ground,
it still does not have form explicitly posited in it;
inasmuch as it determines and informs itself, it is appearance;
and in developing this subsistence that otherwise only is
a reflection-into-other into an immanent reflection,
it becomes two worlds, two totalities of content,
one determined as reflected into itself
and the other as reflected into other.
But the essential relation exposes
the formality of their connection,
and the consummation of the latter is
the relation of the inner and the outer
in which the content of both is equally
only one identical substrate
and only one identity of form.
This identity has come about also in regard to form,
the form determination of their difference is sublated,
and that they are one absolute totality is posited.`,
    summary: 'Actuality = unity of essence and concrete existence. Shapeless essence and unstable appearance have truth in it. Concrete existence = immediacy from ground, determines itself = appearance. Develops into two worlds (reflected into itself, reflected into other). Essential relation exposes formality, consummation = relation of inner and outer (one identical substrate, one identity of form). Form determination sublated, one absolute totality posited.'
  },
  {
    id: 'act-intro-2-three-moments',
    title: 'Three moments of actuality',
    text: `This unity of the inner and outer is absolute actuality.
But this actuality is, first, the absolute as such
(in so far as it is posited as a unity
in which the form has sublated itself)
making itself into the empty or external
distinction of an outer and inner.
Reflection relates to this absolute
as external to it;
it only contemplates it
rather than being its own movement.
But it is essentially this movement
and is, therefore, as the absolute's
negative turning back into itself.

Second, it is actuality proper.
Actuality, possibility, and necessity constitute
the formal moments of the absolute,
or its reflection.

Third, the unity of the absolute
and its reflection is
the absolute relation,
or rather the absolute as
relation to itself, substance.`,
    summary: 'Unity of inner and outer = absolute actuality. First: absolute as such (form sublated, empty/external distinction). Reflection external, contemplates, but essentially movement = negative turning back. Second: actuality proper (actuality, possibility, necessity = formal moments). Third: unity of absolute and reflection = absolute relation = substance.'
  },
];

// ============================================================================
// SECTION III OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'act-op-intro-1-actuality-unity',
    chunkId: 'act-intro-1-actuality-unity',
    label: 'Actuality as unity of essence and concrete existence',
    clauses: [
      'actuality = unityOfEssenceAndConcreteExistence',
      'shapelessEssence.hasTruth = inActuality',
      'unstableAppearance.hasTruth = inActuality',
      'concreteExistence = immediacyFromGround',
      'concreteExistence.determinesItself = appearance',
      'subsistence.developsInto = immanentReflection',
      'becomes = twoWorlds',
      'twoWorlds = {reflectedIntoItself, reflectedIntoOther}',
      'essentialRelation.exposes = formalityOfConnection',
      'consummation = relationOfInnerAndOuter',
      'content = oneIdenticalSubstrate',
      'content = oneIdentityOfForm',
      'formDetermination.sublated = true',
      'posited = oneAbsoluteTotality',
    ],
    predicates: [
      { name: 'IsActuality', args: [] },
      { name: 'IsAbsoluteTotality', args: ['unity'] },
    ],
    relations: [
      { predicate: 'is', from: 'unity', to: 'actuality' },
    ],
  },
  {
    id: 'act-op-intro-2-three-moments',
    chunkId: 'act-intro-2-three-moments',
    label: 'Three moments of actuality',
    clauses: [
      'unityOfInnerAndOuter = absoluteActuality',
      'actuality.first = absoluteAsSuch',
      'absoluteAsSuch.formSublated = true',
      'absoluteAsSuch.makesItselfInto = emptyExternalDistinction',
      'reflection.relatesTo = absolute',
      'reflection.external = true',
      'reflection.contemplates = true',
      'reflection.essentially = movement',
      'movement = negativeTurningBack',
      'actuality.second = actualityProper',
      'formalMoments = {actuality, possibility, necessity}',
      'formalMoments.of = absolute',
      'formalMoments = reflection',
      'actuality.third = absoluteRelation',
      'absoluteRelation = absoluteAsRelationToItself',
      'absoluteRelation = substance',
    ],
    predicates: [
      { name: 'HasThreeMoments', args: ['actuality'] },
      { name: 'IsSubstance', args: ['absoluteRelation'] },
    ],
    relations: [
      { predicate: 'has', from: 'actuality', to: 'threeMoments' },
      { predicate: 'is', from: 'absoluteRelation', to: 'substance' },
    ],
  },
];

// Note: For aggregated exports, use index.ts
// Individual chapter modules are available for granular access

