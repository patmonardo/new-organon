/**
 * Logical Operations: Becoming
 *
 * Becoming - the unity of Being and Nothing, the third moment of the Logic.
 * This is the CPU's synthesis where Being and Nothing unite in their vanishing
 * movement, ultimately collapsing into existence.
 *
 * Dialectical Movement:
 * - Unity of being and nothing: same yet distinct
 * - The moments of becoming: unseparatedness, two unities, interpenetration
 * - Sublation of becoming: equilibrium, contradiction, transition to existence
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// BECOMING
// ============================================================================

export const becomingOp1UnityOfBeingAndNothing: LogicalOperation = {
  id: 'becoming-op-1-unity-of-being-and-nothing',
  chunkId: 'becoming-1',
  label: 'C.1. Unity of being and nothing — same yet distinct',
  clauses: [
    'pureBeing = andPureNothing',
    'pureBeing = areThereforeTheSame',
    'truth = isNeitherBeingNorNothing',
    'truth = butRatherThatBeing',
    'truth = hasPassedOverIntoNothing',
    'truth = andNothingIntoBeing',
    'hasPassedOver = notPassesOver',
    'but = truthIsJustAsMuch',
    'but = thatTheyAreNot',
    'but = withoutDistinction',
    'it = isRatherThatTheyAreNotTheSame',
    'it = thatTheyAreAbsolutelyDistinct',
    'it = yetEquallyUnseparated',
    'it = andInseparable',
    'it = andThatEach',
    'it = immediatelyVanishes',
    'it = inItsOpposite',
    'theirTruth = isThereforeThisMovement',
    'theirTruth = ofTheImmediateVanishing',
    'theirTruth = ofTheOneIntoTheOther',
    'becoming = movementInWhich',
    'becoming = theTwoAreDistinguished',
    'becoming = butByADistinction',
    'becoming = whichHasJustAsImmediately',
    'becoming = dissolvedItself',
  ],
  predicates: [
    { name: 'becoming', args: ['being', 'nothing'] },
  ],
  relations: [
    { predicate: 'is', from: 'pureBeing', to: 'sameAsPureNothing' },
    { predicate: 'is', from: 'truth', to: 'movementOfBecoming' },
  ],
  candidateSummary: 'Pure being and pure nothing are therefore the same. The truth is neither being nor nothing, but rather that being has passed over into nothing and nothing into being; "has passed over," not passes over. But the truth is just as much that they are not without distinction; it is rather that they are not the same, that they are absolutely distinct yet equally unseparated and inseparable, and that each immediately vanishes in its opposite. Their truth is therefore this movement of the immediate vanishing of the one into the other: becoming, a movement in which the two are distinguished, but by a distinction which has just as immediately dissolved itself.',
  provenance: {
    sourceChunk: 'becoming-1',
    sourceOp: 'becoming-op-1-unity-of-being-and-nothing',
  },
};

export const becomingOp2BecomingAsUnseparatedness: LogicalOperation = {
  id: 'becoming-op-2-becoming-as-unseparatedness',
  chunkId: 'becoming-2',
  label: 'C.2. Becoming as unseparatedness — determinate unity',
  clauses: [
    'becoming = isTheUnseparatedness',
    'becoming = ofBeingAndNothing',
    'becoming = notTheUnity',
    'becoming = thatAbstractsFromBeingAndNothing',
    'as = theUnityOfBeingAndNothing',
    'it = isRatherThisDeterminateUnity',
    'it = orOneInWhich',
    'it = beingAndNothingEquallyAre',
    'however = inasmuchAsBeingAndNothing',
    'however = areEachUnseparated',
    'however = fromItsOther',
    'however = eachIsNot',
    'in = thisUnity',
    'therefore = theyAre',
    'but = asVanishing',
    'only = asSublated',
    'they = sinkFromTheirInitially',
    'they = representedSelfSubsistence',
    'they = intoMoments',
    'they = whichAreStillDistinguished',
    'they = butAtSameTimeSublated',
  ],
  predicates: [
    { name: 'unseparatedness', args: ['becoming'] },
  ],
  relations: [
    { predicate: 'is', from: 'becoming', to: 'unseparatednessOfBeingAndNothing' },
  ],
  candidateSummary: 'Becoming is the unseparatedness of being and nothing, not the unity that abstracts from being and nothing; as the unity of being and nothing it is rather this determinate unity, or one in which being and nothing equally are. However, inasmuch as being and nothing are each unseparated from its other, each is not. In this unity, therefore, they are, but as vanishing, only as sublated. They sink from their initially represented self-subsistence into moments which are still distinguished but at the same time sublated.',
  provenance: {
    sourceChunk: 'becoming-2',
    sourceOp: 'becoming-op-2-becoming-as-unseparatedness',
  },
};

export const becomingOp3TwoUnities: LogicalOperation = {
  id: 'becoming-op-3-two-unities',
  chunkId: 'becoming-3',
  label: 'C.2. Two unities — being and nothing as moments',
  clauses: [
    'grasped = asThusDistinguished',
    'each = isInTheirDistinguishedness',
    'each = aUnityWithTheOther',
    'becoming = thusContainsBeingAndNothing',
    'becoming = asTwoSuchUnities',
    'becoming = eachOfWhich',
    'becoming = isItselfUnity',
    'becoming = ofBeingAndNothing',
    'theOne = isBeingAsImmediate',
    'theOne = andAsReferenceToNothing',
    'theOther = isNothingAsImmediate',
    'theOther = andAsReferenceToBeing',
    'in = theseUnities',
    'theDeterminations = areOfUnequalValue',
    'becoming = isInThisWay',
    'becoming = doublyDetermined',
    'in = oneDetermination',
    'nothing = isTheImmediate',
    'that = isTheDetermination',
    'that = beginsWithNothing',
    'that = andThisRefersToBeing',
    'that = isToSay',
    'that = itPassesOverIntoIt',
    'in = theOtherDetermination',
    'being = isTheImmediate',
    'that = isTheDetermination',
    'that = beginsWithBeing',
    'that = andThisPassesOver',
    'that = intoNothing',
    'comingToBe = andCeasingToBe',
  ],
  predicates: [
    { name: 'twoUnities', args: ['becoming'] },
  ],
  relations: [
    { predicate: 'contains', from: 'becoming', to: 'twoUnities' },
  ],
  candidateSummary: 'Grasped as thus distinguished, each is in their distinguishedness a unity with the other. Becoming thus contains being and nothing as two such unities, each of which is itself unity of being and nothing; the one is being as immediate and as reference to nothing; the other is nothing as immediate and as reference to being; in these unities the determinations are of unequal value. Becoming is in this way doubly determined. In one determination, nothing is the immediate, that is, the determination begins with nothing and this refers to being; that is to say, it passes over into it. In the other determination, being is the immediate, that is, the determination begins with being and this passes over into nothing: coming-to-be and ceasing-to-be.',
  provenance: {
    sourceChunk: 'becoming-3',
    sourceOp: 'becoming-op-3-two-unities',
  },
};

export const becomingOp4ComingToBeAndCeasingToBe: LogicalOperation = {
  id: 'becoming-op-4-coming-to-be-and-ceasing-to-be',
  chunkId: 'becoming-4',
  label: 'C.2. Coming-to-be and ceasing-to-be — interpenetration',
  clauses: [
    'both = areTheSame',
    'both = becoming',
    'even = asDirections',
    'even = thatAreSoDifferent',
    'they = interpenetrate',
    'they = andParalyzeEachOther',
    'theOne = isCeasingToBe',
    'being = passesOverIntoNothing',
    'but = nothingIsJustAsMuch',
    'but = theOppositeOfItself',
    'but = thePassingOverIntoBeing',
    'but = comingToBe',
    'this = comingToBe',
    'this = isTheOtherDirection',
    'nothing = goesOverIntoBeing',
    'but = beingEquallySublatesItself',
    'but = andIsRatherThePassingOver',
    'but = intoNothing',
    'it = isCeasingToBe',
    'they = doNotSublateThemselves',
    'they = reciprocally',
    'they = theOneSublatingTheOther',
    'they = externally',
    'but = eachRatherSublatesItself',
    'but = inItself',
    'but = andIsWithinIt',
    'but = theOppositeOfItself',
  ],
  predicates: [
    { name: 'comingToBe', args: ['nothing'] },
    { name: 'ceasingToBe', args: ['being'] },
  ],
  relations: [
    { predicate: 'is', from: 'both', to: 'becoming' },
    { predicate: 'interpenetrate', from: 'they', to: 'eachOther' },
  ],
  candidateSummary: 'Both are the same, becoming, and even as directions that are so different they interpenetrate and paralyze each other. The one is ceasing-to-be; being passes over into nothing, but nothing is just as much the opposite of itself, the passing-over into being, coming-to-be. This coming-to-be is the other direction; nothing goes over into being, but being equally sublates itself and is rather the passing-over into nothing; it is ceasing-to-be. They do not sublate themselves reciprocally [the one sublating the other externally] but each rather sublates itself in itself and is within it the opposite of itself.',
  provenance: {
    sourceChunk: 'becoming-4',
    sourceOp: 'becoming-op-4-coming-to-be-and-ceasing-to-be',
  },
};

export const becomingOp5EquilibriumAndQuiescentUnity: LogicalOperation = {
  id: 'becoming-op-5-equilibrium-and-quiescent-unity',
  chunkId: 'becoming-5',
  label: 'C.3. Equilibrium and quiescent unity — vanishing of becoming',
  clauses: [
    'theEquilibrium = inWhich',
    'theEquilibrium = comingToBeAndCeasingToBe',
    'theEquilibrium = arePoised',
    'theEquilibrium = isInTheFirstPlace',
    'theEquilibrium = becomingItself',
    'but = thisBecoming',
    'but = equallyCollectsItself',
    'but = inQuiescentUnity',
    'being = andNothing',
    'being = areInIt',
    'being = onlyAsVanishing',
    'becoming = itselfHowever',
    'becoming = isOnlyByVirtue',
    'becoming = ofTheirBeingDistinguished',
    'theirVanishing = isTherefore',
    'theirVanishing = theVanishingOfBecoming',
    'theirVanishing = orTheVanishing',
    'theirVanishing = ofTheVanishingItself',
    'becoming = isACeaselessUnrest',
    'becoming = thatCollapses',
    'becoming = intoAQuiescentResult',
  ],
  predicates: [
    { name: 'equilibrium', args: ['becoming'] },
  ],
  relations: [
    { predicate: 'is', from: 'theEquilibrium', to: 'becomingItself' },
  ],
  candidateSummary: 'The equilibrium in which coming-to-be and ceasing-to-be are poised is in the first place becoming itself. But this becoming equally collects itself in quiescent unity. Being and nothing are in it only as vanishing; becoming itself, however, is only by virtue of their being distinguished. Their vanishing is therefore the vanishing of becoming, or the vanishing of the vanishing itself. Becoming is a ceaseless unrest that collapses into a quiescent result.',
  provenance: {
    sourceChunk: 'becoming-5',
    sourceOp: 'becoming-op-5-equilibrium-and-quiescent-unity',
  },
};

export const becomingOp6ContradictionAndVanishedness: LogicalOperation = {
  id: 'becoming-op-6-contradiction-and-vanishedness',
  chunkId: 'becoming-6',
  label: 'C.3. Contradiction and vanishedness — not nothing',
  clauses: [
    'this = canAlsoBeExpressed',
    'this = thus',
    'becoming = isTheVanishing',
    'becoming = ofBeingIntoNothing',
    'becoming = andOfNothingIntoBeing',
    'becoming = andTheVanishing',
    'becoming = ofBeingAndNothing',
    'becoming = inGeneral',
    'but = atTheSameTime',
    'but = itRestsOnTheirBeingDistinct',
    'it = thereforeContradictsItself',
    'it = inItself',
    'because = whatItUnites',
    'because = withinItself',
    'because = isSelfOpposed',
    'but = suchAUnion',
    'but = destroysItself',
    'thisResult = isAVanishedness',
    'but = itIsNotNothing',
    'as = such',
    'it = wouldBeOnlyARelapse',
    'it = intoOneOfTheAlready',
    'it = sublatedDeterminations',
    'it = andNotTheResult',
    'it = ofNothingAndOfBeing',
  ],
  predicates: [
    { name: 'contradiction', args: ['becoming'] },
  ],
  relations: [
    { predicate: 'is', from: 'thisResult', to: 'vanishedness' },
  ],
  candidateSummary: 'This can also be expressed thus: becoming is the vanishing of being into nothing, and of nothing into being, and the vanishing of being and nothing in general; but at the same time it rests on their being distinct. It therefore contradicts itself in itself, because what it unites within itself is self-opposed; but such a union destroys itself. This result is a vanishedness, but it is not nothing; as such, it would be only a relapse into one of the already sublated determinations and not the result of nothing and of being.',
  provenance: {
    sourceChunk: 'becoming-6',
    sourceOp: 'becoming-op-6-contradiction-and-vanishedness',
  },
};

export const becomingOp7TransitionToExistence: LogicalOperation = {
  id: 'becoming-op-7-transition-to-existence',
  chunkId: 'becoming-7',
  label: 'C.3. Transition to existence — quiescent simplicity',
  clauses: [
    'it = isTheUnity',
    'it = ofBeingAndNothing',
    'it = thatHasBecomeQuiescentSimplicity',
    'but = thisQuiescentSimplicity',
    'but = isBeing',
    'yet = noLongerForItself',
    'but = asDetermination',
    'but = ofTheWhole',
    'becoming = asTransition',
    'becoming = intoTheUnity',
    'becoming = ofBeingAndNothing',
    'becoming = aUnityWhichIs',
    'becoming = asExistent',
    'becoming = orHasTheShape',
    'becoming = ofTheOneSidedImmediateUnity',
    'becoming = ofTheseMoments',
    'becoming = isExistence',
  ],
  predicates: [
    { name: 'existence', args: ['becoming'] },
  ],
  relations: [
    { predicate: 'is', from: 'becoming', to: 'existence' },
  ],
  candidateSummary: 'It is the unity of being and nothing that has become quiescent simplicity. But this quiescent simplicity is being, yet no longer for itself but as determination of the whole. Becoming, as transition into the unity of being and nothing, a unity which is as existent or has the shape of the one-sided immediate unity of these moments, is existence.',
  provenance: {
    sourceChunk: 'becoming-7',
    sourceOp: 'becoming-op-7-transition-to-existence',
  },
};

export const becomingOperations: LogicalOperation[] = [
  becomingOp1UnityOfBeingAndNothing,
  becomingOp2BecomingAsUnseparatedness,
  becomingOp3TwoUnities,
  becomingOp4ComingToBeAndCeasingToBe,
  becomingOp5EquilibriumAndQuiescentUnity,
  becomingOp6ContradictionAndVanishedness,
  becomingOp7TransitionToExistence,
];

