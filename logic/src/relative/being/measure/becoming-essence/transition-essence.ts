import type { Chunk, LogicalOperation } from '../../../types';

/**
 * C. TRANSITION INTO ESSENCE — Species 3 of The Becoming of Essence
 * 
 * This module covers the third species: the transition from absolute indifference
 * to essence, the self-sublation of indifference, and being determined as essence.
 */

// ============================================================================
// C. TRANSITION INTO ESSENCE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'be-c-1-absolute-indifference-final-determination',
    title: 'Absolute indifference as final determination — still belongs to being',
    text: `Absolute indifference is the final determination of being
before the latter becomes essence;
but it does not attain essence.
It shows that it still belongs to the sphere of being
because it is still determined as indifferent,
and therefore difference is external to it, quantitative.
This externality is its existence, by which
it finds itself at the same time in the opposition of
being determined over against it as existing in itself,
not as being thought as the absolute that exists for itself.
Or again, it is external reflection
which insists that specific determinations,
whether in themselves or in the absolute,
are one and the same;
that their difference is only an indifferent one,
not a difference in itself.
What is still missing here is that
this reflection should sublate itself,
that it would cease to be the external
reflection of thought, of a subjective consciousness,
but that it would be rather the very determination of
the difference of that unity;
a unity which would then prove itself to be the absolute negativity,
the unity's indifference towards itself,
towards its own indifference no less than towards otherness.`,
    summary: 'Absolute indifference = final determination of being before essence, does not attain essence. Still belongs to being (determined as indifferent), difference external/quantitative. Externality = existence, opposition. External reflection: determinations one and same, difference indifferent. Missing: reflection should sublate itself, become determination of difference of unity. Unity = absolute negativity, indifference towards itself and otherness.'
  },
  {
    id: 'be-c-2-self-sublation-manifested',
    title: 'Self-sublation manifested — contradiction',
    text: `But this self-sublation of the determination of
indifference has already manifested itself;
in the progressive positing of its being
it has shown itself on all sides to be contradiction.
Indifference is in itself the totality
in which all the determinations of being are sublated and contained;
thus it is the substrate,
but at first only in the one-sided determination of being-in-itself,
and consequently the differences,
the quantitative difference and the inverseratio of factors,
are present in it as external.
As thus the contradiction of itself and its determinateness,
of its implicitly existent determination
and of its posited determinateness,
it is the negative totality whose determinacies
have internally sublated themselves,
consequently, have also sublated the one-sidedness
of their substrate, their in-itselfness.
Indifference, now posited as what it in fact is,
is simple and infinitely negative self-reference,
the incompatibility of itself with itself,
the repelling of itself from itself.
Determining and being determined are
not a transition,
nor an external alteration,
nor again an emergence of determinations in it,
but its own referring to itself
which is the negativity of itself,
of its in-itselfness.`,
    summary: 'Self-sublation already manifested. Progressive positing shows contradiction on all sides. Indifference = totality (determinations sublated/contained) = substrate (one-sided being-in-itself). Differences present as external. Contradiction of itself and determinateness (implicit vs posited). Negative totality: determinacies internally sublated, sublated one-sidedness of substrate. Indifference = simple infinitely negative self-reference, incompatibility with itself, repelling from itself. Determining/determined = own referring to itself = negativity of itself, of in-itselfness.'
  },
  {
    id: 'be-c-3-determinations-as-moments',
    title: 'Determinations as moments — posited',
    text: `But as so repelled,
the determinations are not self-possessed;
do not emerge as self-subsistent or external
but are rather as moments:
first, as belonging to the unity
whose existence is still only implicit,
they are not let go by it but are rather borne by it
as their substrate and are filled by it alone;
and, second, as determinations immanent
to the unity as it exists for itself,
they are only through their repulsion from themselves.
Instead of some existent or other,
as they are in the whole sphere of being,
they now are simply and solely as posited,
with the sole determination and significance of
being referred to their unity
and hence each to the other and to negation,
marked by this their relativity.`,
    summary: 'As repelled: determinations not self-possessed, not self-subsistent/external, but as moments. First: belonging to unity (existence implicit), borne as substrate, filled by it alone. Second: immanent to unity existing for itself, only through repulsion from themselves. Instead of existent (sphere of being), now simply as posited, sole determination = referred to unity, each to other and negation, marked by relativity.'
  },
  {
    id: 'be-c-4-being-determined-as-essence',
    title: 'Being determined as essence — simple being with itself',
    text: `Being in general and the being or immediacy
of the different determinacies have thereby vanished
just as much as the in-itselfness,
and the unity is being,
immediately presupposed totality,
so that it is only this simple self-reference,
mediated by the sublation of this presupposition,
and this pre-supposedness, the immediate being,
is itself only a moment of its repelling:
the original self-subsistence and self-identity are only
as the resulting infinite self-rejoining.
And so is being determined as essence:
being which, through the sublation of being,
is simple being with itself.`,
    summary: 'Being in general and immediacy of determinacies vanished (as in-itselfness). Unity = being, immediately presupposed totality = simple self-reference (mediated by sublation of presupposition). Pre-supposedness (immediate being) = moment of repelling. Original self-subsistence/self-identity = resulting infinite self-rejoining. Being determined as essence: being which, through sublation of being, is simple being with itself.'
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'be-c-op-1-absolute-indifference-final-determination',
    chunkId: 'be-c-1-absolute-indifference-final-determination',
    label: 'Absolute indifference as final determination — still belongs to being',
    clauses: [
      'absoluteIndifference = finalDeterminationOfBeing',
      'absoluteIndifference.before = essence',
      'absoluteIndifference.notAttains = essence',
      'absoluteIndifference.stillBelongsTo = sphereOfBeing',
      'absoluteIndifference.determinedAs = indifferent',
      'difference.externalTo = absoluteIndifference',
      'difference = quantitative',
      'externality = existence',
      'externality.opposition = true',
      'externalReflection.insists = determinationsOneAndSame',
      'difference.only = indifferent',
      'difference.not = differenceInItself',
      'missing = reflectionShouldSublateItself',
      'reflection.shouldCeaseToBe = external',
      'reflection.shouldBecome = determinationOfDifference',
      'unity = absoluteNegativity',
      'unity.indifferenceTowards = {itself, ownIndifference, otherness}',
    ],
    predicates: [
      { name: 'IsFinalDetermination', args: ['absoluteIndifference'] },
      { name: 'StillBelongsToBeing', args: ['absoluteIndifference'] },
    ],
    relations: [
      { predicate: 'is', from: 'absoluteIndifference', to: 'finalDetermination' },
    ],
  },
  {
    id: 'be-c-op-2-self-sublation-manifested',
    chunkId: 'be-c-2-self-sublation-manifested',
    label: 'Self-sublation manifested — contradiction',
    clauses: [
      'selfSublation.alreadyManifested = true',
      'progressivePositing.shows = contradiction',
      'contradiction.onAllSides = true',
      'indifference = totality',
      'totality.contains = allDeterminationsOfBeing',
      'determinations.sublated = true',
      'indifference = substrate',
      'substrate.atFirst = oneSidedBeingInItself',
      'differences.presentAs = external',
      'contradiction.of = {itself, determinateness}',
      'contradiction.of = {implicitDetermination, positedDetermination}',
      'indifference = negativeTotality',
      'determinacies.internallySublated = true',
      'determinacies.sublated = oneSidednessOfSubstrate',
      'indifference.positedAs = whatItIs',
      'indifference = simpleInfinitelyNegativeSelfReference',
      'indifference = incompatibilityWithItself',
      'indifference = repellingFromItself',
      'determiningDetermined.not = transition',
      'determiningDetermined.not = externalAlteration',
      'determiningDetermined.not = emergence',
      'determiningDetermined = ownReferringToItself',
      'ownReferringToItself = negativityOfItself',
      'negativityOfItself.of = inItselfness',
    ],
    predicates: [
      { name: 'IsContradiction', args: ['indifference'] },
      { name: 'IsSelfSublating', args: ['determination'] },
    ],
    relations: [
      { predicate: 'is', from: 'indifference', to: 'contradiction' },
    ],
  },
  {
    id: 'be-c-op-3-determinations-as-moments',
    chunkId: 'be-c-3-determinations-as-moments',
    label: 'Determinations as moments — posited',
    clauses: [
      'determinations.asRepelled.not = selfPossessed',
      'determinations.not = selfSubsistent',
      'determinations.not = external',
      'determinations = moments',
      'moments.first = belongingToUnity',
      'unity.existence = implicit',
      'moments.borneBy = unity',
      'moments.as = substrate',
      'moments.filledBy = unityAlone',
      'moments.second = immanentToUnity',
      'unity.existsForItself = true',
      'moments.onlyThrough = repulsionFromThemselves',
      'insteadOf = existent',
      'now = posited',
      'soleDetermination = referredToUnity',
      'referredTo = {unity, eachOther, negation}',
      'markedBy = relativity',
    ],
    predicates: [
      { name: 'IsPosited', args: ['determination'] },
      { name: 'IsMoment', args: ['determination'] },
    ],
    relations: [
      { predicate: 'is', from: 'determination', to: 'moment' },
    ],
  },
  {
    id: 'be-c-op-4-being-determined-as-essence',
    chunkId: 'be-c-4-being-determined-as-essence',
    label: 'Being determined as essence — simple being with itself',
    clauses: [
      'beingInGeneral.vanished = true',
      'immediacyOfDeterminacies.vanished = true',
      'inItselfness.vanished = true',
      'unity = being',
      'unity = immediatelyPresupposedTotality',
      'unity = simpleSelfReference',
      'simpleSelfReference.mediatedBy = sublationOfPresupposition',
      'preSupposedness = immediateBeing',
      'preSupposedness = momentOfRepelling',
      'originalSelfSubsistence = resultingInfiniteSelfRejoining',
      'originalSelfIdentity = resultingInfiniteSelfRejoining',
      'being.determinedAs = essence',
      'essence = being',
      'essence.through = sublationOfBeing',
      'essence = simpleBeingWithItself',
    ],
    predicates: [
      { name: 'IsEssence', args: ['being'] },
      { name: 'IsSimpleBeingWithItself', args: ['essence'] },
    ],
    relations: [
      { predicate: 'is', from: 'being', to: 'essence' },
    ],
  },
];

