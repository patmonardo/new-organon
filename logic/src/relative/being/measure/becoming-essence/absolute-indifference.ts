import type { Chunk, LogicalOperation } from '../../../types';

/**
 * A. ABSOLUTE INDIFFERENCE — Species 1 of The Becoming of Essence
 * 
 * This module covers the first species: absolute indifference as the unity
 * mediated through negation of all determinations of being.
 */

// ============================================================================
// A. ABSOLUTE INDIFFERENCE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'be-a-1-being-abstract-indifference',
    title: 'Being as abstract indifference — indifferentness',
    text: `Being is abstract indifference,
and when this trait is to be thought by itself as being,
the abstract expression "indifferentness" has been used;
in which there is not supposed to be as yet
any kind of determinateness.
Pure quantity is this indifference in the sense of
being open to any determinations,
provided that these are external to it
and that quantity itself does not have
any link with them originating in it.`,
    summary: 'Being = abstract indifference. Abstract expression = "indifferentness" (no determinateness yet). Pure quantity = indifference open to any determinations (external, no link originating in it).'
  },
  {
    id: 'be-a-2-absolute-indifference',
    title: 'Absolute indifference — mediated unity',
    text: `The indifference which can be called absolute,
however, is one which, through the negation of
every determinateness of being, of quality and quantity
and of their at first immediate unity, that is, of measure,
mediates itself with itself to form a simple unity.
Determinateness is in it still only a state, that is,
something qualitative and external
which has the indifference as a substrate.`,
    summary: 'Absolute indifference = through negation of every determinateness (being, quality, quantity, measure), mediates itself with itself = simple unity. Determinateness = state (qualitative, external), has indifference as substrate.'
  },
  {
    id: 'be-a-3-vanishing-determinateness',
    title: 'Vanishing determinateness — empty differentiation',
    text: `But that which has thus been determined
as qualitative and external
is only a vanishing something;
as thus external with respect to being,
the qualitative sphere is the opposite of itself
and, as such, only the sublating of itself.
In this way, determinateness is still
only posited in the substrate
as an empty differentiation.
But it is precisely this empty differentiation
which is the indifference itself as result.
And this indifference is indeed concrete,
in the sense that it is self-mediated
through the negation of all the determinations of being.
As such a mediation, it contains negation and relation,
and what was called "state" is a differentiation
which is immanent to it and self-referring.
It is precisely this externality and its vanishing
which make the unity of being into an indifference:
consequently, they are inside this indifference,
which thereby ceases to be only a substrate
and, within, only abstract.`,
    summary: 'Determined as qualitative/external = vanishing something. Qualitative sphere = opposite of itself, sublating itself. Determinateness = empty differentiation in substrate. Empty differentiation = indifference as result. Indifference = concrete, self-mediated through negation of all determinations. Contains negation and relation. State = differentiation immanent and self-referring. Externality and vanishing make unity into indifference. Inside indifference, ceases to be only substrate/abstract.'
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'be-a-op-1-being-abstract-indifference',
    chunkId: 'be-a-1-being-abstract-indifference',
    label: 'Being as abstract indifference — indifferentness',
    clauses: [
      'being = abstractIndifference',
      'abstractExpression = "indifferentness"',
      'indifferentness.hasNo = determinateness',
      'pureQuantity = indifference',
      'pureQuantity.openTo = anyDeterminations',
      'determinations.external = true',
      'quantity.hasNoLink = originatingInIt',
    ],
    predicates: [
      { name: 'IsAbstractIndifference', args: ['being'] },
      { name: 'IsIndifferentness', args: [] },
    ],
    relations: [
      { predicate: 'is', from: 'being', to: 'abstractIndifference' },
    ],
  },
  {
    id: 'be-a-op-2-absolute-indifference',
    chunkId: 'be-a-2-absolute-indifference',
    label: 'Absolute indifference — mediated unity',
    clauses: [
      'absoluteIndifference = throughNegationOfEveryDeterminateness',
      'determinatenesses = {being, quality, quantity, measure}',
      'absoluteIndifference.mediatesItself = withItself',
      'absoluteIndifference.forms = simpleUnity',
      'determinateness.inIt = state',
      'state = qualitativeExternal',
      'state.has = indifferenceAsSubstrate',
    ],
    predicates: [
      { name: 'IsAbsoluteIndifference', args: [] },
      { name: 'IsMediatedUnity', args: ['indifference'] },
    ],
    relations: [
      { predicate: 'mediates', from: 'absoluteIndifference', to: 'simpleUnity' },
    ],
  },
  {
    id: 'be-a-op-3-vanishing-determinateness',
    chunkId: 'be-a-3-vanishing-determinateness',
    label: 'Vanishing determinateness — empty differentiation',
    clauses: [
      'determinedAsQualitativeExternal = vanishingSomething',
      'qualitativeSphere = oppositeOfItself',
      'qualitativeSphere = sublatingOfItself',
      'determinateness.positedIn = substrate',
      'determinateness.as = emptyDifferentiation',
      'emptyDifferentiation = indifferenceAsResult',
      'indifference = concrete',
      'indifference = selfMediated',
      'indifference.throughNegationOf = allDeterminationsOfBeing',
      'mediation.contains = {negation, relation}',
      'state = differentiation',
      'differentiation.immanent = true',
      'differentiation.selfReferring = true',
      'externalityAndVanishing.make = unityIntoIndifference',
      'externalityAndVanishing.inside = indifference',
      'indifference.ceasesToBe = onlySubstrate',
      'indifference.ceasesToBe = onlyAbstract',
    ],
    predicates: [
      { name: 'IsVanishing', args: ['determinateness'] },
      { name: 'IsEmptyDifferentiation', args: ['differentiation'] },
    ],
    relations: [
      { predicate: 'is', from: 'determinateness', to: 'vanishing' },
    ],
  },
];

