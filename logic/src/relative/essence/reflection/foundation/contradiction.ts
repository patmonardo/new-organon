/**
 * Logical Operations: Contradiction
 *
 * Contradiction is the third determination of reflection. Opposition as
 * self-subsisting is contradiction. Resolved contradiction is ground.
 *
 * Dialectical Movement:
 * - Opposition as self-subsisting: contradiction
 * - Difference implicitly contradiction: posited contradiction
 * - Contradiction resolves itself: null and positive
 * - Transition to ground: opposition foundered
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// CONTRADICTION
// ============================================================================

export const ctrOp1OppositionAsSelfSubsisting: LogicalOperation = {
  id: 'ctr-op-1-opposition-as-self-subsisting',
  chunkId: 'ctr-1',
  label: 'Opposition as self-subsisting — contradiction',
  clauses: [
    'difference = containsBothSides',
    'diversity = sidesFallApart',
    'opposition = momentsDeterminedByOther',
    'opposition = mutuallyExclusive',
    'opposition = selfSubsisting',
    'eachMoment = wholeSelfContainedOpposition',
    'selfMediated = throughOther',
    'selfMediated = throughNonBeingOfOther',
    'unity = existingForItself',
    'unity = excludingOther',
    'selfSubsistingDetermination = excludesOther',
    'selfSubsistingDetermination = excludesItsOwnSelfSubsistence',
    'it = contradiction',
  ],
  predicates: [
    { name: 'containsBothSides', args: ['difference'] },
    { name: 'sidesFallApart', args: ['diversity'] },
    { name: 'momentsDeterminedByOther', args: ['opposition'] },
    { name: 'mutuallyExclusive', args: ['opposition'] },
    { name: 'selfSubsisting', args: ['opposition'] },
    { name: 'wholeSelfContainedOpposition', args: ['eachMoment'] },
    { name: 'throughOther', args: ['selfMediated'] },
    { name: 'throughNonBeingOfOther', args: ['selfMediated'] },
    { name: 'existingForItself', args: ['unity'] },
    { name: 'excludingOther', args: ['unity'] },
    { name: 'excludesOther', args: ['selfSubsistingDetermination'] },
    { name: 'excludesItsOwnSelfSubsistence', args: ['selfSubsistingDetermination'] },
    { name: 'contradiction', args: ['it'] },
  ],
  relations: [
    { predicate: 'is', from: 'opposition', to: 'contradiction' },
  ],
  candidateSummary: 'Difference contains both sides as moments. In diversity, sides fall apart as indifferent. In opposition, moments determined by other, only moments. But in opposition moments equally determined within, mutually exclusive, self-subsisting. Positive and negative, each such within. Each has indifferent self-subsistence by virtue of reference to other moment within it. Each moment is whole self-contained opposition. Self-mediated through other and through non-being of other. Unity existing for itself and excluding other from itself. Self-subsisting determination excludes other in same respect as it contains it. In self-subsistence, determination excludes its own self-subsistence from itself. And so it is contradiction.',
  provenance: {
    sourceChunk: 'ctr-1',
    sourceOp: 'ctr-op-1-opposition-as-self-subsisting',
  },
};

export const ctrOp2DifferenceImplicitlyContradiction: LogicalOperation = {
  id: 'ctr-op-2-difference-implicitly-contradiction',
  chunkId: 'ctr-2',
  label: 'Difference implicitly contradiction — posited contradiction',
  clauses: [
    'difference = implicitlyContradiction',
    'unity = ofBeings',
    'separation = ofBeings',
    'positiveAndNegative = positedContradiction',
    'negativeUnities = selfPositing',
    'each = sublatingOfItself',
    'each = positingOfOpposite',
    'determiningReflection = exclusive',
    'each = excludesItselfInternally',
  ],
  predicates: [
    { name: 'implicitlyContradiction', args: ['difference'] },
    { name: 'ofBeings', args: ['unity'] },
    { name: 'ofBeings', args: ['separation'] },
    { name: 'positedContradiction', args: ['positiveAndNegative'] },
    { name: 'selfPositing', args: ['negativeUnities'] },
    { name: 'sublatingOfItself', args: ['each'] },
    { name: 'positingOfOpposite', args: ['each'] },
    { name: 'exclusive', args: ['determiningReflection'] },
    { name: 'excludesItselfInternally', args: ['each'] },
  ],
  relations: [
    { predicate: 'are', from: 'positiveAndNegative', to: 'positedContradiction' },
  ],
  candidateSummary: 'Difference as such already implicitly contradiction. Unity of beings which are, only in so far as they are not one. Separation of beings which are, only in so far as they are separated in same reference connecting them. Positive and negative are posited contradiction. As negative unities, precisely their self-positing. Each sublating of itself and positing of its opposite. Constitute determining reflection as exclusive. Each excludes itself internally.',
  provenance: {
    sourceChunk: 'ctr-2',
    sourceOp: 'ctr-op-2-difference-implicitly-contradiction',
  },
};

export const ctrOp3PositiveAsContradiction: LogicalOperation = {
  id: 'ctr-op-3-positive-as-contradiction',
  chunkId: 'ctr-3',
  label: 'Positive as contradiction',
  clauses: [
    'positive = positednessReflectedIntoLikeness',
    'positive = makesItselfIntoReferenceOfNonBeing',
    'positive = contradiction',
    'positive = positingSelfIdentity',
    'positive = makesItselfIntoNegative',
    'positive = makesItselfIntoOther',
    'reflection = positingOfPositive',
    'positing = positingOfItsOther',
  ],
  predicates: [
    { name: 'positednessReflectedIntoLikeness', args: ['positive'] },
    { name: 'makesItselfIntoReferenceOfNonBeing', args: ['positive'] },
    { name: 'contradiction', args: ['positive'] },
    { name: 'positingSelfIdentity', args: ['positive'] },
    { name: 'makesItselfIntoNegative', args: ['positive'] },
    { name: 'makesItselfIntoOther', args: ['positive'] },
    { name: 'positingOfPositive', args: ['reflection'] },
    { name: 'positingOfItsOther', args: ['positing'] },
  ],
  relations: [
    { predicate: 'is', from: 'positive', to: 'contradiction' },
  ],
  candidateSummary: 'Positive is positedness as reflected into likeness with itself. Positedness which is not reference to another. Subsistence inasmuch as positedness is sublated and excluded. But positive makes itself into reference of non-being into positedness. Positive is contradiction: positing self-identity by excluding negative, makes itself into negative. Makes itself into other which it excludes from itself. Reflection that excludes is positing of positive as excluding other. This positing immediately is positing of its other which excludes it.',
  provenance: {
    sourceChunk: 'ctr-3',
    sourceOp: 'ctr-op-3-positive-as-contradiction',
  },
};

export const ctrOp4AbsoluteContradiction: LogicalOperation = {
  id: 'ctr-op-4-absolute-contradiction',
  chunkId: 'ctr-4',
  label: 'Absolute contradiction of positive and negative',
  clauses: [
    'absoluteContradiction = ofPositiveAndNegative',
    'negative = positedContradiction',
    'negative = identicalWithItself',
    'determination = toBeNotIdentical',
    'negative = excludesItselfFromItself',
    'negative = determinesItselfAsIdentity',
  ],
  predicates: [
    { name: 'ofPositiveAndNegative', args: ['absoluteContradiction'] },
    { name: 'positedContradiction', args: ['negative'] },
    { name: 'identicalWithItself', args: ['negative'] },
    { name: 'toBeNotIdentical', args: ['determination'] },
    { name: 'excludesItselfFromItself', args: ['negative'] },
    { name: 'determinesItselfAsIdentity', args: ['negative'] },
  ],
  relations: [
    { predicate: 'is', from: 'negative', to: 'positedContradiction' },
  ],
  candidateSummary: 'Absolute contradiction of positive, immediately absolute contradiction of negative. Negative is positedness as reflected into unlikeness to itself. Negative is itself unlike, non-being of another. Reflection in its unlikeness is reference rather to itself. Negative is not immediate. Same contradiction as positive: positedness or negation as self-reference. Positive only implicitly contradiction, contradiction only in itself. Negative is posited contradiction. Negative identical with itself, determination is to be not-identical, exclusion of identity. To be identical with itself over against identity, excludes itself from itself. Negative is whole opposition, excludes identity from itself, but thereby excludes itself. As reference to itself determines itself as very identity which it excludes.',
  provenance: {
    sourceChunk: 'ctr-4',
    sourceOp: 'ctr-op-4-absolute-contradiction',
  },
};

export const ctrOp5ContradictionResolvesItself: LogicalOperation = {
  id: 'ctr-op-5-contradiction-resolves-itself',
  chunkId: 'ctr-5',
  label: 'Contradiction resolves itself — null and positive',
  clauses: [
    'each = selfTranslatingIntoOpposite',
    'vanishing = firstUnity',
    'vanishing = theNull',
    'contradiction = containsPositive',
    'selfExcludingReflection = positingReflection',
    'result = notOnlyNull',
    'positedness = foundersToGround',
  ],
  predicates: [
    { name: 'selfTranslatingIntoOpposite', args: ['each'] },
    { name: 'firstUnity', args: ['vanishing'] },
    { name: 'theNull', args: ['vanishing'] },
    { name: 'containsPositive', args: ['contradiction'] },
    { name: 'positingReflection', args: ['selfExcludingReflection'] },
    { name: 'notOnlyNull', args: ['result'] },
    { name: 'foundersToGround', args: ['positedness'] },
  ],
  relations: [
    { predicate: 'resolves', from: 'contradiction', to: 'itself' },
  ],
  candidateSummary: 'In self-excluding reflection, positive and negative, each in self-subsistence, sublates itself. Each is passing over, self-translating of itself into its opposite. Internal ceaseless vanishing of opposites is first unity that arises by virtue of contradiction. It is the null. But contradiction does not contain merely negative, also contains positive. Self-excluding reflection is at same time positing reflection. Result of contradiction is not only null. Positive and negative constitute positedness of self-subsistence. Their own self-negation sublates it. This positedness founders to ground in contradiction.',
  provenance: {
    sourceChunk: 'ctr-5',
    sourceOp: 'ctr-op-5-contradiction-resolves-itself',
  },
};

export const ctrOp6ImmanentReflection: LogicalOperation = {
  id: 'ctr-op-6-immanent-reflection',
  chunkId: 'ctr-6',
  label: 'Immanent reflection — self-subsistence and positedness',
  clauses: [
    'immanentReflection = turnsSidesIntoSelfSubsistent',
    'they = selfSubsistenceOnlyInThemselves',
    'excludingReflection = sublatesPositedness',
    'excludingReflection = turnsIntoSelfSubsistentBeings',
    'byBeingPosited = theyMakeThemselvesPositedness',
    'they = fateThemselvesToFounder',
    'selfIdentity = referenceToOther',
  ],
  predicates: [
    { name: 'turnsSidesIntoSelfSubsistent', args: ['immanentReflection'] },
    { name: 'selfSubsistenceOnlyInThemselves', args: ['they'] },
    { name: 'sublatesPositedness', args: ['excludingReflection'] },
    { name: 'turnsIntoSelfSubsistentBeings', args: ['excludingReflection'] },
    { name: 'theyMakeThemselvesPositedness', args: ['byBeingPosited'] },
    { name: 'fateThemselvesToFounder', args: ['they'] },
    { name: 'referenceToOther', args: ['selfIdentity'] },
  ],
  relations: [
    { predicate: 'turns', from: 'immanentReflection', to: 'sides' },
  ],
  candidateSummary: 'Immanent reflection turns sides of opposition into self-subsistent self-references. First, their self-subsistence as distinct moments. They are self-subsistence only in themselves, for they are still opposites. That they are in themselves self-subsistent constitutes their positedness. But excluding reflection sublates this positedness. Turns them into self-subsistent beings existing in and for themselves. Self-subsistent by virtue of negative reference to their other. Their self-subsistence also posited. But by being posited as self-subsistent, they make themselves into positedness. They fate themselves to founder. Determine themselves as self-identical, yet in self-identity are rather negative. Self-identity which is reference-to-other.',
  provenance: {
    sourceChunk: 'ctr-6',
    sourceOp: 'ctr-op-6-immanent-reflection',
  },
};

export const ctrOp7ExcludingReflection: LogicalOperation = {
  id: 'ctr-op-7-excluding-reflection',
  chunkId: 'ctr-7',
  label: 'Excluding reflection — self-withdrawal, positive self-unity',
  clauses: [
    'excludingReflection = selfSubsistenceExistingInItself',
    'positedness = sublatedPositedness',
    'sublatingNegative = bothPositsAndSublates',
    'sublation = selfWithdrawal',
    'sublation = positiveSelfUnity',
    'unityOfEssence = identicalWithItselfThroughNegationOfItself',
  ],
  predicates: [
    { name: 'selfSubsistenceExistingInItself', args: ['excludingReflection'] },
    { name: 'sublatedPositedness', args: ['positedness'] },
    { name: 'bothPositsAndSublates', args: ['sublatingNegative'] },
    { name: 'selfWithdrawal', args: ['sublation'] },
    { name: 'positiveSelfUnity', args: ['sublation'] },
    { name: 'identicalWithItselfThroughNegationOfItself', args: ['unityOfEssence'] },
  ],
  relations: [
    { predicate: 'is', from: 'sublation', to: 'positiveSelfUnity' },
  ],
  candidateSummary: 'Excluding reflection is not only formal determination. It is self-subsistence existing in itself. Sublating of positedness is only through this sublating a unity that exists for itself and is self-subsistent. This negation is not return to first immediate reference to other. Not positedness as sublated immediacy, but positedness as sublated positedness. Excluding reflection makes itself positedness but is sublation of its positedness. Sublating reference to itself. First sublates negative, secondly posits itself as negative, only this posited negative that it sublates. In sublating negative, both posits and sublates it at same time. Exclusive determination is itself that other of itself of which it is negation. Sublation of positedness is self-withdrawal, positive self-unity. Self-subsistence is unity that turns back into itself by virtue of its own negation. Unity of essence to be identical with itself through negation not of other, but of itself.',
  provenance: {
    sourceChunk: 'ctr-7',
    sourceOp: 'ctr-op-7-excluding-reflection',
  },
};

export const ctrOp8TransitionToGround: LogicalOperation = {
  id: 'ctr-op-8-transition-to-ground',
  chunkId: 'ctr-8',
  label: 'Transition to ground — opposition foundered',
  clauses: [
    'opposition = foundered',
    'opposition = goneBackToGround',
    'excludingReflection = turnsOppositionIntoNegative',
    'positedness = goneBackToUnity',
    'essence = restored',
    'essence = exclusiveReflectiveUnity',
    'simpleUnity = determinesItselfAsNegation',
    'simpleUnity = likeItself',
  ],
  predicates: [
    { name: 'foundered', args: ['opposition'] },
    { name: 'goneBackToGround', args: ['opposition'] },
    { name: 'turnsOppositionIntoNegative', args: ['excludingReflection'] },
    { name: 'goneBackToUnity', args: ['positedness'] },
    { name: 'restored', args: ['essence'] },
    { name: 'exclusiveReflectiveUnity', args: ['essence'] },
    { name: 'determinesItselfAsNegation', args: ['simpleUnity'] },
    { name: 'likeItself', args: ['simpleUnity'] },
  ],
  relations: [
    { predicate: 'transitionsTo', from: 'opposition', to: 'ground' },
  ],
  candidateSummary: 'Since self-subsistence in opposition, as excluding reflection, makes itself into positedness and equally sublates it. Not only has opposition foundered but in foundering has gone back to foundation, to ground. Excluding reflection turns self-subsisting opposition into negative, something only posited. Reduces formerly self-subsisting determinations (positive and negative) to determinations which are only determinations. Positedness has gone back to its unity with itself. Simple essence, but essence as ground. Through sublating of determinations of essence, which are in themselves self-contradictory, essence is restored. Restored in determination of exclusive, reflective unity. Simple unity which determines itself as negation, but in positedness is immediately like itself and withdrawn into itself.',
  provenance: {
    sourceChunk: 'ctr-8',
    sourceOp: 'ctr-op-8-transition-to-ground',
  },
};

export const ctrOp9EssenceAsGround: LogicalOperation = {
  id: 'ctr-op-9-essence-as-ground',
  chunkId: 'ctr-9',
  label: 'Essence as ground — positedness',
  clauses: [
    'selfSubsistingOpposition = goesBackIntoGround',
    'essence = asGroundPositedness',
    'essence = excludingReflection',
    'essence = excludesItselfFromItself',
    'essence = positsItself',
    'selfSubsistent = negativePositedAsNegative',
    'selfSubsistent = remainsInEssence',
  ],
  predicates: [
    { name: 'goesBackIntoGround', args: ['selfSubsistingOpposition'] },
    { name: 'asGroundPositedness', args: ['essence'] },
    { name: 'excludingReflection', args: ['essence'] },
    { name: 'excludesItselfFromItself', args: ['essence'] },
    { name: 'positsItself', args: ['essence'] },
    { name: 'negativePositedAsNegative', args: ['selfSubsistent'] },
    { name: 'remainsInEssence', args: ['selfSubsistent'] },
  ],
  relations: [
    { predicate: 'is', from: 'essence', to: 'ground' },
  ],
  candidateSummary: 'Because of contradiction, self-subsisting opposition goes back into ground. Opposition is what comes first, immediate from which beginning is made. Sublated opposition or sublated positedness is itself positedness. Essence is as ground positedness, something that has become. But conversely, only this has been posited: opposition or positedness is something sublated, only is as positedness. As ground, essence is excluding reflection because makes itself into positedness. Opposition from which start was made is merely posited determinate self-subsistence of essence. Opposition only sublates itself within, whereas essence is in determinateness reflected into itself. As ground, essence excludes itself from itself, posits itself. Positedness which is excluded is only as positedness, as identity of negative with itself. Self-subsistent is negative posited as negative, something self-contradictory. Remains in essence as in its ground.',
  provenance: {
    sourceChunk: 'ctr-9',
    sourceOp: 'ctr-op-9-essence-as-ground',
  },
};

export const ctrOp10ResolvedContradictionIsGround: LogicalOperation = {
  id: 'ctr-op-10-resolved-contradiction-is-ground',
  chunkId: 'ctr-10',
  label: 'Resolved contradiction is ground — unity of positive and negative',
  clauses: [
    'resolvedContradiction = ground',
    'resolvedContradiction = essenceAsUnity',
    'ground = selfSubsistenceCompleted',
    'ground = oppositionRemovedAndPreserved',
    'ground = positiveSelfIdentity',
    'selfContradictoryOpposition = alreadyGround',
    'essence = reflectedIntoItself',
    'essence = selfIdentical',
  ],
  predicates: [
    { name: 'ground', args: ['resolvedContradiction'] },
    { name: 'essenceAsUnity', args: ['resolvedContradiction'] },
    { name: 'selfSubsistenceCompleted', args: ['ground'] },
    { name: 'oppositionRemovedAndPreserved', args: ['ground'] },
    { name: 'positiveSelfIdentity', args: ['ground'] },
    { name: 'alreadyGround', args: ['selfContradictoryOpposition'] },
    { name: 'reflectedIntoItself', args: ['essence'] },
    { name: 'selfIdentical', args: ['essence'] },
  ],
  relations: [
    { predicate: 'is', from: 'resolvedContradiction', to: 'ground' },
  ],
  candidateSummary: 'Resolved contradiction is ground, essence as unity of positive and negative. In opposition, determinateness has progressed to self-subsistence. Ground is this self-subsistence as completed. In it, negative is self-subsistent essence, but as negative. As self-identical in this negativity, ground is equally positive. In ground, opposition and its contradiction just as much removed as preserved. Ground is essence as positive self-identity which refers itself to itself as negativity. Determines itself, making itself into excluded positedness. But positedness is whole self-subsisting essence. Essence is ground, self-identical in its negation and positive. Self-contradictory self-subsistent opposition was itself already ground. All that was added was determination of self-unity. Which emerges as each self-subsisting opposite sublates itself and makes itself into its other. Founders and sinks to ground but therein reunites itself with itself. In foundering, in positedness or in negation, rather is for first time essence that is reflected into itself and self-identical.',
  provenance: {
    sourceChunk: 'ctr-10',
    sourceOp: 'ctr-op-10-resolved-contradiction-is-ground',
  },
};

export const contradictionOperations: LogicalOperation[] = [
  ctrOp1OppositionAsSelfSubsisting,
  ctrOp2DifferenceImplicitlyContradiction,
  ctrOp3PositiveAsContradiction,
  ctrOp4AbsoluteContradiction,
  ctrOp5ContradictionResolvesItself,
  ctrOp6ImmanentReflection,
  ctrOp7ExcludingReflection,
  ctrOp8TransitionToGround,
  ctrOp9EssenceAsGround,
  ctrOp10ResolvedContradictionIsGround,
];
