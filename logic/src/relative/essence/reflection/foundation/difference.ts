/**
 * Logical Operations: Difference
 *
 * Difference is the second determination of reflection, negativity that
 * reflection possesses in itself. It develops through absolute difference,
 * diversity, and opposition (positive and negative).
 *
 * Dialectical Movement:
 * - Difference as negativity of reflection
 * - Absolute difference as simple
 * - Diversity: identity breaks apart
 * - Opposition: positive and negative
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// DIFFERENCE
// ============================================================================

export const diffOp1Introduction: LogicalOperation = {
  id: 'diff-op-1-introduction',
  chunkId: 'diff-1',
  label: 'Introduction — difference as negativity of reflection',
  clauses: [
    'difference = negativityOfReflection',
    'difference = reflectionPossessesInItself',
    'difference = nothingInIdentityDiscourse',
    'difference = essentialMomentOfIdentity',
    'difference = negativityOfItself',
    'difference = determinesItself',
    'difference = differentiatedFromDifference',
  ],
  predicates: [
    { name: 'negativityOfReflection', args: ['difference'] },
    { name: 'reflectionPossessesInItself', args: ['difference'] },
    { name: 'essentialMomentOfIdentity', args: ['difference'] },
    { name: 'negativityOfItself', args: ['difference'] },
    { name: 'determinesItself', args: ['difference'] },
  ],
  relations: [
    { predicate: 'is', from: 'difference', to: 'negativityOfReflection' },
    { predicate: 'is', from: 'difference', to: 'essentialMomentOfIdentity' },
  ],
  candidateSummary: 'Difference is negativity that reflection possesses in itself. Nothing which is said in identity discourse. Essential moment of identity itself. As negativity of itself, determines itself and is differentiated from difference.',
  provenance: {
    sourceChunk: 'diff-1',
    sourceOp: 'diff-op-1-introduction',
  },
};

export const diffOp2AbsoluteDifferenceAsSimple: LogicalOperation = {
  id: 'diff-op-2-absolute-difference-as-simple',
  chunkId: 'diff-2',
  label: 'Absolute difference as simple',
  clauses: [
    'difference = inAndForItself',
    'difference = absoluteDifference',
    'difference = differenceOfEssence',
    'difference != differenceThroughExternal',
    'difference = selfReferring',
    'difference = simpleDifference',
    'simpleNot = constitutesDifference',
    'difference = simpleConcept',
    'difference = differenceOfReflection',
    'difference != othernessOfExistence',
    'otherOfEssence = otherInAndForItself',
    'otherOfEssence = simpleDeterminatenessInItself',
  ],
  predicates: [
    { name: 'absoluteDifference', args: ['difference'] },
    { name: 'differenceOfEssence', args: ['difference'] },
    { name: 'selfReferring', args: ['difference'] },
    { name: 'simpleDifference', args: ['difference'] },
    { name: 'simpleConcept', args: ['difference'] },
    { name: 'differenceOfReflection', args: ['difference'] },
    { name: 'otherInAndForItself', args: ['otherOfEssence'] },
  ],
  relations: [
    { predicate: 'is', from: 'difference', to: 'absoluteDifference' },
    { predicate: 'constitutes', from: 'simpleNot', to: 'difference' },
  ],
  candidateSummary: 'Difference in and for itself, absolute difference, difference of essence. Not difference through something external but self-referring, simple difference. Essential to grasp absolute difference as simple. In absolute difference of A and not-A, simple "not" constitutes difference. Difference itself is simple concept. Difference of reflection, not otherness of existence. Other of essence is other in and for itself, simple determinateness in itself.',
  provenance: {
    sourceChunk: 'diff-2',
    sourceOp: 'diff-op-2-absolute-difference-as-simple',
  },
};

export const diffOp3DifferenceInItselfUnity: LogicalOperation = {
  id: 'diff-op-3-difference-in-itself-unity',
  chunkId: 'diff-3',
  label: 'Difference in itself — unity with identity',
  clauses: [
    'difference = refersItselfToItself',
    'difference = negativityOfItself',
    'difference = ofItselfFromItself',
    'difference = itsOther',
    'whatIsDifferent = identity',
    'difference = itselfAndIdentity',
    'twoTogether = constituteDifference',
    'difference = wholeAndItsMoment',
    'difference = essentialNatureOfReflection',
    'difference = primordialOriginOfActivity',
    'difference = unityOfItselfAndIdentity',
    'difference = internallyDetermined',
  ],
  predicates: [
    { name: 'refersItselfToItself', args: ['difference'] },
    { name: 'negativityOfItself', args: ['difference'] },
    { name: 'itsOther', args: ['difference'] },
    { name: 'itselfAndIdentity', args: ['difference'] },
    { name: 'wholeAndItsMoment', args: ['difference'] },
    { name: 'essentialNatureOfReflection', args: ['difference'] },
    { name: 'internallyDetermined', args: ['difference'] },
  ],
  relations: [
    { predicate: 'is', from: 'whatIsDifferent', to: 'identity' },
    { predicate: 'constitute', from: 'twoTogether', to: 'difference' },
  ],
  candidateSummary: 'Difference in itself refers itself to itself. Negativity of itself, difference not from another but of itself from itself. Not itself but its other. What is different from difference is identity. Difference is itself and identity. Two together constitute difference; difference is whole and its moment. Essential nature of reflection, determined primordial origin of all activity and self-movement. Difference as unity of itself and identity is internally determined difference.',
  provenance: {
    sourceChunk: 'diff-3',
    sourceOp: 'diff-op-3-difference-in-itself-unity',
  },
};

export const diffOp4DifferenceHasBothMoments: LogicalOperation = {
  id: 'diff-op-4-difference-has-both-moments',
  chunkId: 'diff-4',
  label: 'Difference has both moments — diversity',
  clauses: [
    'difference = hasBothMoments',
    'moments = identityAndDifference',
    'moments = positedness',
    'moments = determinateness',
    'eachMoment = refersToItself',
    'identity = momentOfImmanentReflection',
    'difference = reflectedDifference',
    'difference = diversity',
  ],
  predicates: [
    { name: 'hasBothMoments', args: ['difference'] },
    { name: 'positedness', args: ['moments'] },
    { name: 'determinateness', args: ['moments'] },
    { name: 'refersToItself', args: ['eachMoment'] },
    { name: 'momentOfImmanentReflection', args: ['identity'] },
    { name: 'reflectedDifference', args: ['difference'] },
    { name: 'diversity', args: ['difference'] },
  ],
  relations: [
    { predicate: 'is', from: 'difference', to: 'diversity' },
  ],
  candidateSummary: 'Difference has both moments, identity and difference. Two are both positedness, determinateness. In this positedness each refers to itself. Identity is moment of immanent reflection. Difference is difference in itself, reflected difference. Difference, having two moments which are reflections into themselves, is diversity.',
  provenance: {
    sourceChunk: 'diff-4',
    sourceOp: 'diff-op-4-difference-has-both-moments',
  },
};

export const diffOp5DiversityIdentityBreaksApart: LogicalOperation = {
  id: 'diff-op-5-diversity-identity-breaks-apart',
  chunkId: 'diff-5',
  label: 'Diversity — identity breaks apart',
  clauses: [
    'identity = breaksApartIntoDiversity',
    'identity = positsItselfAsNegative',
    'moments = reflectionsIntoThemselves',
    'different = subsistsAsDiverse',
    'different = indifferentToOther',
    'identity = baseAndElement',
    'diverse = remainsWhatItIs',
    'diversity = othernessOfReflection',
  ],
  predicates: [
    { name: 'breaksApartIntoDiversity', args: ['identity'] },
    { name: 'positsItselfAsNegative', args: ['identity'] },
    { name: 'reflectionsIntoThemselves', args: ['moments'] },
    { name: 'subsistsAsDiverse', args: ['different'] },
    { name: 'indifferentToOther', args: ['different'] },
    { name: 'baseAndElement', args: ['identity'] },
    { name: 'othernessOfReflection', args: ['diversity'] },
  ],
  relations: [
    { predicate: 'breaksApart', from: 'identity', to: 'diversity' },
  ],
  candidateSummary: 'Identity internally breaks apart into diversity. As absolute difference in itself, posits itself as negative of itself. Two moments (itself and negative of itself) are reflections into themselves. Different subsists as diverse, indifferent to any other. Because identical with itself, identity constitutes base and element. Diverse remains what it is even in its opposite, identity. Diversity constitutes otherness as such of reflection.',
  provenance: {
    sourceChunk: 'diff-5',
    sourceOp: 'diff-op-5-diversity-identity-breaks-apart',
  },
};

export const diffOp6DiversityMomentsNotDetermined: LogicalOperation = {
  id: 'diff-op-6-diversity-moments-not-determined',
  chunkId: 'diff-6',
  label: 'Diversity — moments not determined with respect to each other',
  clauses: [
    'moments = identityAndDifference',
    'moments = diverse',
    'moments = reflectedIntoThemselves',
    'identity = selfReferring',
    'identity = notReferredToDifference',
    'difference = notReferredToIdentity',
    'two = notDeterminedWithRespectToEachOther',
    'difference = externalToThem',
  ],
  predicates: [
    { name: 'diverse', args: ['moments'] },
    { name: 'reflectedIntoThemselves', args: ['moments'] },
    { name: 'selfReferring', args: ['identity'] },
    { name: 'notReferredToDifference', args: ['identity'] },
    { name: 'notReferredToIdentity', args: ['difference'] },
    { name: 'notDeterminedWithRespectToEachOther', args: ['two'] },
    { name: 'externalToThem', args: ['difference'] },
  ],
  relations: [
    { predicate: 'is', from: 'difference', to: 'externalToThem' },
  ],
  candidateSummary: 'Moments of difference are identity and difference itself. These moments are diverse when reflected into themselves. In determination of identity, only self-referring. Identity not referred to difference, difference not referred to identity. Each referred only to itself, two not determined with respect to each other. Because two not differentiated within, difference is external to them.',
  provenance: {
    sourceChunk: 'diff-6',
    sourceOp: 'diff-op-6-diversity-moments-not-determined',
  },
};

export const diffOp7DiversityReflectionExternal: LogicalOperation = {
  id: 'diff-op-7-diversity-reflection-external',
  chunkId: 'diff-7',
  label: 'Diversity — reflection becomes external, likeness and unlikeness',
  clauses: [
    'reflection = external',
    'identityAndDifference = reflections',
    'each = whole',
    'immanentReflection = identity',
    'immanentReflection = indifferentToDifference',
    'immanentReflection = diversity',
    'externalReflection = determinateDifference',
    'externalIdentity = likeness',
    'externalDifference = unlikeness',
    'comparison = dependsOnThird',
  ],
  predicates: [
    { name: 'external', args: ['reflection'] },
    { name: 'reflections', args: ['identityAndDifference'] },
    { name: 'whole', args: ['each'] },
    { name: 'identity', args: ['immanentReflection'] },
    { name: 'diversity', args: ['immanentReflection'] },
    { name: 'determinateDifference', args: ['externalReflection'] },
    { name: 'likeness', args: ['externalIdentity'] },
    { name: 'unlikeness', args: ['externalDifference'] },
  ],
  relations: [
    { predicate: 'is', from: 'externalIdentity', to: 'likeness' },
    { predicate: 'is', from: 'externalDifference', to: 'unlikeness' },
  ],
  candidateSummary: 'In diversity, as indifference of difference, reflection has become external. Both identity and difference are reflections, each is whole. Determinateness to be only identity or only difference is sublated something. Duplicity: immanent reflection as such and determinateness as negation or positedness. Immanent reflection is identity, indifferent to difference, is diversity. External reflection is determinate difference. External identity is likeness, external difference is unlikeness. Whether like or unlike depends on point of view of third external to them.',
  provenance: {
    sourceChunk: 'diff-7',
    sourceOp: 'diff-op-7-diversity-reflection-external',
  },
};

export const diffOp8ExternalReflectionConnects: LogicalOperation = {
  id: 'diff-op-8-external-reflection-connects',
  chunkId: 'diff-8',
  label: 'External reflection connects diversity — comparing',
  clauses: [
    'externalReflection = connectsDiversity',
    'externalReflection = refersToLikenessAndUnlikeness',
    'reference = comparing',
    'reference = movesBackAndForth',
    'reference = externalToDeterminations',
    'externalReflection = externalToItself',
    'determinateDifference = negatedAbsoluteDifference',
    'determinateDifference = hasReflectionOutsideIt',
  ],
  predicates: [
    { name: 'connectsDiversity', args: ['externalReflection'] },
    { name: 'comparing', args: ['reference'] },
    { name: 'externalToDeterminations', args: ['reference'] },
    { name: 'externalToItself', args: ['externalReflection'] },
    { name: 'negatedAbsoluteDifference', args: ['determinateDifference'] },
    { name: 'hasReflectionOutsideIt', args: ['determinateDifference'] },
  ],
  relations: [
    { predicate: 'connects', from: 'externalReflection', to: 'diversity' },
  ],
  candidateSummary: 'External reflection connects diversity by referring it to likeness and unlikeness. Reference is comparing, moves back and forth from likeness to unlikeness. Back and forth referring external to determinations themselves. Each, for itself, referred to third. External reflection external to itself. Determinate difference is negated absolute difference. Not simple difference, not immanent reflection, has reflection outside it.',
  provenance: {
    sourceChunk: 'diff-8',
    sourceOp: 'diff-op-8-external-reflection-connects',
  },
};

export const diffOp9LikenessUnlikenessSeparation: LogicalOperation = {
  id: 'diff-op-9-likeness-unlikeness-separation',
  chunkId: 'diff-9',
  label: 'Likeness and unlikeness — separation and self-sublation',
  clauses: [
    'likenessAndUnlikeness = unconnected',
    'reflection = keepsApart',
    'separation = causesSelfSublation',
    'keepingApart = destruction',
    'both = determinationsOfDifference',
    'both = referencesToEachOther',
    'each = referredToItself',
    'each = likeItself',
    'difference = vanished',
    'each = onlyLikeness',
  ],
  predicates: [
    { name: 'unconnected', args: ['likenessAndUnlikeness'] },
    { name: 'keepsApart', args: ['reflection'] },
    { name: 'causesSelfSublation', args: ['separation'] },
    { name: 'destruction', args: ['keepingApart'] },
    { name: 'determinationsOfDifference', args: ['both'] },
    { name: 'referencesToEachOther', args: ['both'] },
    { name: 'referredToItself', args: ['each'] },
    { name: 'likeItself', args: ['each'] },
    { name: 'vanished', args: ['difference'] },
    { name: 'onlyLikeness', args: ['each'] },
  ],
  relations: [
    { predicate: 'keepsApart', from: 'reflection', to: 'likenessAndUnlikeness' },
  ],
  candidateSummary: 'In reflection alienated from itself, likeness and unlikeness present as unconnected. Reflection keeps them apart by "in so far," "from this side or that." Diverse things from one side like, from another unlike. Because of separation, they sublate themselves. Keeping likeness and unlikeness apart is their destruction. Both are determinations of difference, references to each other. But because of indifference, each referred to itself. Each is like itself; difference has vanished, each is only likeness.',
  provenance: {
    sourceChunk: 'diff-9',
    sourceOp: 'diff-op-9-likeness-unlikeness-separation',
  },
};

export const diffOp10NegativeUnityTransition: LogicalOperation = {
  id: 'diff-op-10-negative-unity-transition',
  chunkId: 'diff-10',
  label: 'Negative unity — transition to opposition',
  clauses: [
    'externalDifference = sublatesItself',
    'negativity = belongsToComparing',
    'unity = negativeUnity',
    'unity = natureOfLikenessAndUnlikeness',
    'like = notLikeOfItself',
    'unlike = like',
    'each = unlikeOfItself',
    'each = itselfAndItsOther',
    'merelyDiverse = passesIntoNegativeReflection',
    'diversity = opposition',
  ],
  predicates: [
    { name: 'sublatesItself', args: ['externalDifference'] },
    { name: 'belongsToComparing', args: ['negativity'] },
    { name: 'negativeUnity', args: ['unity'] },
    { name: 'natureOfLikenessAndUnlikeness', args: ['unity'] },
    { name: 'notLikeOfItself', args: ['like'] },
    { name: 'like', args: ['unlike'] },
    { name: 'unlikeOfItself', args: ['each'] },
    { name: 'itselfAndItsOther', args: ['each'] },
    { name: 'passesIntoNegativeReflection', args: ['merelyDiverse'] },
    { name: 'opposition', args: ['diversity'] },
  ],
  relations: [
    { predicate: 'is', from: 'diversity', to: 'opposition' },
  ],
  candidateSummary: 'Indifferent viewpoint or external difference sublates itself. Negativity which in comparing belongs to that which does comparing. Oscillates from likeness to unlikeness, lets one disappear into other. Negative unity of both. Unity is nature of likeness and unlikeness themselves. Like is not like of itself, unlike is like. Each is unlike of itself. Each is itself and its other (likeness/unlikeness). Merely diverse passes over through positedness into negative reflection. Diverse is difference merely posited, difference which is no difference. Negation that negates itself within. Diversity, indifferent sides of which are moments of negative unity, is opposition.',
  provenance: {
    sourceChunk: 'diff-10',
    sourceOp: 'diff-op-10-negative-unity-transition',
  },
};

export const diffOp11OppositionIntroduction: LogicalOperation = {
  id: 'diff-op-11-opposition-introduction',
  chunkId: 'diff-11',
  label: 'Opposition — introduction',
  clauses: [
    'opposition = determinateReflection',
    'opposition = differenceCompleted',
    'opposition = unityOfIdentityAndDiversity',
    'moments = diverseInOneIdentity',
    'moments = opposites',
    'identityAndDifference = momentsOfDifference',
    'likenessAndUnlikeness = externalizedReflection',
  ],
  predicates: [
    { name: 'determinateReflection', args: ['opposition'] },
    { name: 'differenceCompleted', args: ['opposition'] },
    { name: 'unityOfIdentityAndDiversity', args: ['opposition'] },
    { name: 'diverseInOneIdentity', args: ['moments'] },
    { name: 'opposites', args: ['moments'] },
    { name: 'momentsOfDifference', args: ['identityAndDifference'] },
    { name: 'externalizedReflection', args: ['likenessAndUnlikeness'] },
  ],
  relations: [
    { predicate: 'is', from: 'opposition', to: 'unityOfIdentityAndDiversity' },
  ],
  candidateSummary: 'In opposition, determinate reflection, difference, brought to completion. Opposition is unity of identity and diversity. Moments are diverse in one identity, so they are opposites. Identity and difference are moments of difference held inside difference itself. Reflected moments of its unity. Likeness and unlikeness are externalized reflection. Their self-identity contrasts with identity reflected into itself. Immediacy which is not reflected into itself.',
  provenance: {
    sourceChunk: 'diff-11',
    sourceOp: 'diff-op-11-opposition-introduction',
  },
};

export const diffOp12MomentsOfOpposition: LogicalOperation = {
  id: 'diff-op-12-moments-of-opposition',
  chunkId: 'diff-12',
  label: 'Moments of opposition — positedness reflected into itself',
  clauses: [
    'moments = positednessReflectedIntoItself',
    'positedness = likenessAndUnlikeness',
    'immanentReflection = unityOfLikenessAndUnlikeness',
    'each = whole',
    'whole = containsOtherMoment',
    'reflectionIntoItself = referringToNonBeing',
  ],
  predicates: [
    { name: 'positednessReflectedIntoItself', args: ['moments'] },
    { name: 'likenessAndUnlikeness', args: ['positedness'] },
    { name: 'unityOfLikenessAndUnlikeness', args: ['immanentReflection'] },
    { name: 'whole', args: ['each'] },
    { name: 'containsOtherMoment', args: ['whole'] },
    { name: 'referringToNonBeing', args: ['reflectionIntoItself'] },
  ],
  relations: [
    { predicate: 'are', from: 'moments', to: 'positednessReflectedIntoItself' },
  ],
  candidateSummary: 'Moments of opposition are positedness reflected into itself or determination in general. Positedness is likeness and unlikeness. These two, reflected into themselves, constitute determinations of opposition. Their immanent reflection: each is within it unity of likeness and unlikeness. Likeness only in reflection which compares according to unlikeness. Each, in its determinateness, is whole. Whole because contains its other moment. But other is indifferent existent. Each contains reference to its non-being. Reflection-into-itself, or whole, only as essentially referring to its non-being.',
  provenance: {
    sourceChunk: 'diff-12',
    sourceOp: 'diff-op-12-moments-of-opposition',
  },
};

export const diffOp13PositiveAndNegative: LogicalOperation = {
  id: 'diff-op-13-positive-and-negative',
  chunkId: 'diff-13',
  label: 'Positive and negative — determinations of opposition',
  clauses: [
    'positive = selfLikenessReflectedIntoItself',
    'positive = containsReferenceToUnlikeness',
    'negative = unlikenessReflectedIntoItself',
    'negative = containsReferenceToLikeness',
    'both = positedness',
    'positive = positednessReflectedIntoSelfLikeness',
    'negative = positednessReflectedIntoUnlikeness',
    'each = hasOtherInIt',
  ],
  predicates: [
    { name: 'selfLikenessReflectedIntoItself', args: ['positive'] },
    { name: 'containsReferenceToUnlikeness', args: ['positive'] },
    { name: 'unlikenessReflectedIntoItself', args: ['negative'] },
    { name: 'containsReferenceToLikeness', args: ['negative'] },
    { name: 'positedness', args: ['both'] },
    { name: 'positednessReflectedIntoSelfLikeness', args: ['positive'] },
    { name: 'positednessReflectedIntoUnlikeness', args: ['negative'] },
    { name: 'hasOtherInIt', args: ['each'] },
  ],
  relations: [
    { predicate: 'is', from: 'positive', to: 'positednessReflectedIntoSelfLikeness' },
    { predicate: 'is', from: 'negative', to: 'positednessReflectedIntoUnlikeness' },
  ],
  candidateSummary: 'Self-likeness, reflected into itself, containing reference to unlikeness within it, is positive. Unlikeness containing reference to its non-being, to likeness, is negative. Both are positedness. Opposition: positedness reflected into likeness with itself, and into inequality with itself. Positive and negative. Positive is positedness as reflected into self-likeness. Negative is positedness as reflected into unlikeness. Each equally has other in it: positive has unlikeness, negative has likeness.',
  provenance: {
    sourceChunk: 'diff-13',
    sourceOp: 'diff-op-13-positive-and-negative',
  },
};

export const diffOp14PositiveNegativeSelfSubsisting: LogicalOperation = {
  id: 'diff-op-14-positive-negative-self-subsisting',
  chunkId: 'diff-14',
  label: 'Positive and negative as self-subsisting',
  clauses: [
    'positiveAndNegative = selfSubsisting',
    'selfSubsisting = reflectionOfWholeIntoItself',
    'opposition = implicitlyDeterminate',
    'each = itselfAndItsOther',
    'each = hasDeterminatenessWithin',
    'each = refersToItselfAsReferringToOther',
    'twofoldAspect = referenceToNonBeing',
    'twofoldAspect = sublatingOtherness',
    'positedness = indifferentSubsistence',
  ],
  predicates: [
    { name: 'selfSubsisting', args: ['positiveAndNegative'] },
    { name: 'reflectionOfWholeIntoItself', args: ['selfSubsisting'] },
    { name: 'implicitlyDeterminate', args: ['opposition'] },
    { name: 'itselfAndItsOther', args: ['each'] },
    { name: 'hasDeterminatenessWithin', args: ['each'] },
    { name: 'refersToItselfAsReferringToOther', args: ['each'] },
    { name: 'indifferentSubsistence', args: ['positedness'] },
  ],
  relations: [
    { predicate: 'are', from: 'positiveAndNegative', to: 'selfSubsisting' },
  ],
  candidateSummary: 'Positive and negative are sides of opposition that have become self-subsisting. Self-subsisting because reflection of whole into itself. Belong to opposition as determinateness reflected into itself. Because of self-subsistence, opposition implicitly determinate. Each is itself and its other. Each has determinateness not in other but within. Each refers itself to itself only as referring itself to its other. Twofold aspect: reference to non-being as sublating otherness in itself. But positedness has become being, indifferent subsistence. Each only to extent that its non-being is, two in identical reference.',
  provenance: {
    sourceChunk: 'diff-14',
    sourceOp: 'diff-op-14-positive-negative-self-subsisting',
  },
};

export const diffOp15ThreeAspects: LogicalOperation = {
  id: 'diff-op-15-three-aspects',
  chunkId: 'diff-15',
  label: 'Three aspects: moments, diverse, in and for themselves',
  clauses: [
    'first = absoluteMomentsOfOpposition',
    'first = oneMediation',
    'first = opposites',
    'second = merePositedness',
    'second = diverse',
    'second = interchangeable',
    'third = selfSubsistentUnity',
    'third = existingForItself',
    'positive = immanentReflection',
    'positive = negatesOtherness',
    'negative = absoluteReflection',
    'negative = excludesPositive',
    'reference = exclusive',
    'reference = inItselfness',
  ],
  predicates: [
    { name: 'absoluteMomentsOfOpposition', args: ['first'] },
    { name: 'oneMediation', args: ['first'] },
    { name: 'opposites', args: ['first'] },
    { name: 'merePositedness', args: ['second'] },
    { name: 'diverse', args: ['second'] },
    { name: 'interchangeable', args: ['second'] },
    { name: 'selfSubsistentUnity', args: ['third'] },
    { name: 'existingForItself', args: ['third'] },
    { name: 'immanentReflection', args: ['positive'] },
    { name: 'negatesOtherness', args: ['positive'] },
    { name: 'absoluteReflection', args: ['negative'] },
    { name: 'excludesPositive', args: ['negative'] },
    { name: 'exclusive', args: ['reference'] },
    { name: 'inItselfness', args: ['reference'] },
  ],
  relations: [
    { predicate: 'constitutes', from: 'reference', to: 'inItselfness' },
  ],
  candidateSummary: 'First: positive and negative are absolute moments of opposition. Subsistence indivisibly one reflection, one mediation. Each by virtue of non-being of its other. Simply opposites, each only opposite of other. Second: mere positedness reflected into itself. Two sides merely diverse, can be interchanged, each can be taken equally as positive or negative. Third: positive and negative not only posited being, nor merely indifferent. Their positedness, reference to other in unity, taken back into each. Each itself positive and negative within. Each is self-subsistent unity existing for itself. Positive: positedness for it posited being as sublated, immanent reflection that negates otherness. Negative: absolute reflection, negative as sublated positedness, negatively rests upon itself, excludes positive from itself. Positive and negative in themselves and in and for themselves. Reference, precisely as exclusive, constitutes their determination or in-itselfness.',
  provenance: {
    sourceChunk: 'diff-15',
    sourceOp: 'diff-op-15-three-aspects',
  },
};

export const differenceOperations: LogicalOperation[] = [
  diffOp1Introduction,
  diffOp2AbsoluteDifferenceAsSimple,
  diffOp3DifferenceInItselfUnity,
  diffOp4DifferenceHasBothMoments,
  diffOp5DiversityIdentityBreaksApart,
  diffOp6DiversityMomentsNotDetermined,
  diffOp7DiversityReflectionExternal,
  diffOp8ExternalReflectionConnects,
  diffOp9LikenessUnlikenessSeparation,
  diffOp10NegativeUnityTransition,
  diffOp11OppositionIntroduction,
  diffOp12MomentsOfOpposition,
  diffOp13PositiveAndNegative,
  diffOp14PositiveNegativeSelfSubsisting,
  diffOp15ThreeAspects,
];
