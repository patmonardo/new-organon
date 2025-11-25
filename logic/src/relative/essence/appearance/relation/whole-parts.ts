/**
 * Logical Operations: The Relation of Whole and Parts
 *
 * Essential relation contains self-subsistence reflected into itself.
 * Whole is world-in-itself, parts is world of appearance. This relation
 * passes over into force and its expressions.
 *
 * Dialectical Movement:
 * - Essential relation: two sides
 * - Contradiction: each is relative of other
 * - Transition: force and expressions
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// THE RELATION OF WHOLE AND PARTS
// ============================================================================

export const wlpOp1EssentialRelationTwoSides: LogicalOperation = {
  id: 'wlp-op-1-essential-relation-two-sides',
  chunkId: 'wlp-1',
  label: 'Essential relation — two sides',
  clauses: [
    'essentialRelation.contains = selfSubsistenceOfConcreteExistence',
    'essentialRelation.contains = reflectedIntoItself',
    'simpleForm = whoseDeterminationsAreConcreteExistences',
    'simpleForm = positedAtSameTime',
    'simpleForm = momentsHeldInUnity',
    'selfSubsistence = reflectedIntoItself',
    'selfSubsistence = isReflectionIntoOpposite',
    'selfSubsistence = immediateSelfSubsistence',
    'subsistence = identityWithOpposite',
    'subsistence = noLessThanOwnSelfSubsistence',
    'otherSide = immediatelyPosited',
    'otherSide = immediateSelfSubsistence',
    'otherSide = determinedAsOther',
    'otherSide = multifariousManifold',
    'manifold = essentiallyHasReferenceOfOtherSide',
    'manifold = unityOfReflectedSelfSubsistence',
    'whole = selfSubsistence',
    'whole = thatConstitutesWorldExistingInAndForItself',
    'parts = immediateConcreteExistence',
    'parts = whichWasWorldOfAppearance',
    'inRelation = twoSidesAreSelfSubsistences',
    'inRelation = eachHasOtherReflectivelyShiningInIt',
    'inRelation = onlyIsAsIdentityOfBoth',
    'whole = constitutesUnityOfTwoSides',
    'whole = substrate',
    'immediateConcreteExistence = asPositedness',
    'onSideOfParts = immediateManifold',
    'onSideOfParts = isSelfSubsistentSubstrate',
    'onSideOfParts = wholeOnlyExternalReference',
  ],
  predicates: [
    { name: 'essentialRelation', args: ['whole', 'parts'] },
  ],
  relations: [
    { predicate: 'is', from: 'whole', to: 'worldExistingInAndForItself' },
    { predicate: 'is', from: 'parts', to: 'worldOfAppearance' },
  ],
  candidateSummary: 'Essential relation contains self-subsistence of concrete existence reflected into itself. Simple form whose determinations are concrete existences, posited at same time, moments held in unity. Self-subsistence reflected into itself is reflection into opposite, immediate self-subsistence. Subsistence is identity with opposite no less than own self-subsistence. Other side immediately posited: immediate self-subsistence determined as other, multifarious manifold. Manifold essentially has reference of other side, unity of reflected self-subsistence. Whole is self-subsistence that constitutes world existing in-and-for-itself. Parts is immediate concrete existence which was world of appearance. In relation, two sides are self-subsistences, each has other reflectively shining in it, only is as identity of both. Whole constitutes unity of two sides, substrate. Immediate concrete existence as positedness. On side of parts, immediate manifold is self-subsistent substrate, whole only external reference.',
  provenance: {
    sourceChunk: 'wlp-1',
    sourceOp: 'wlp-op-1-essential-relation-two-sides',
  },
};

export const wlpOp2RelationContainsContradiction: LogicalOperation = {
  id: 'wlp-op-2-relation-contains-contradiction',
  chunkId: 'wlp-2',
  label: 'Relation contains contradiction',
  clauses: [
    'relation.contains = selfSubsistenceOfSides',
    'relation.contains = andTheirSublatedness',
    'relation.contains = noLess',
    'relation.contains = twoSimplyInOneReference',
    'whole = selfSubsistent',
    'parts = onlyMomentsOfUnity',
    'parts = butAlsoEquallySelfSubsistent',
    'reflectedUnity = onlyMoment',
    'each = inSelfSubsistence',
    'each = simplyRelativeOfOther',
    'relation = withinIt',
    'relation = immediateContradiction',
    'relation = sublatesItself',
  ],
  predicates: [
    { name: 'contradiction', args: ['relation'] },
  ],
  relations: [
    { predicate: 'contains', from: 'relation', to: 'contradiction' },
  ],
  candidateSummary: 'Relation contains self-subsistence of sides and their sublatedness no less, two simply in one reference. Whole is self-subsistent. Parts are only moments of unity, but also equally self-subsistent. Reflected unity only moment. Each, in self-subsistence, simply relative of other. Relation within it immediate contradiction, sublates itself.',
  provenance: {
    sourceChunk: 'wlp-2',
    sourceOp: 'wlp-op-2-relation-contains-contradiction',
  },
};

export const wlpOp3WholeSelfExternalized: LogicalOperation = {
  id: 'wlp-op-3-whole-self-externalized',
  chunkId: 'wlp-3',
  label: 'Whole — self-externalized',
  clauses: [
    'whole = reflectedUnity',
    'whole = thatStandsIndependentlyOnOwn',
    'but = subsistence',
    'but = equallyRepelledByIt',
    'whole = selfExternalized',
    'whole.has = subsistenceInOpposite',
    'whole.has = manifoldImmediacy',
    'whole.has = parts',
    'whole = consistsOfParts',
    'whole = apartFromThemNotAnything',
    'whole = relationAndSelfSubsistentTotality',
    'but = forPreciselyThisReason',
    'whole = onlyRelative',
    'whatMakesItTotality = ratherOther',
    'whatMakesItTotality = parts',
    'whole = doesNotHaveSubsistenceWithinIt',
    'whole = butInOther',
  ],
  predicates: [
    { name: 'selfExternalized', args: ['whole'] },
  ],
  relations: [
    { predicate: 'has', from: 'whole', to: 'subsistenceInParts' },
  ],
  candidateSummary: 'Whole is reflected unity that stands independently on own. But subsistence equally repelled by it. Self-externalized. Has subsistence in opposite, manifold immediacy, parts. Whole consists of parts, apart from them not anything. Whole relation and self-subsistent totality. But for precisely this reason, only relative. What makes it totality is rather other, parts. Does not have subsistence within it but in other.',
  provenance: {
    sourceChunk: 'wlp-3',
    sourceOp: 'wlp-op-3-whole-self-externalized',
  },
};

export const wlpOp4PartsWholeRelation: LogicalOperation = {
  id: 'wlp-op-4-parts-whole-relation',
  chunkId: 'wlp-4',
  label: 'Parts — whole relation',
  clauses: [
    'parts = likewiseWholeRelation',
    'parts = immediateAsAgainstReflectedSelfSubsistence',
    'parts = doNotSubsistInWhole',
    'parts = butForThemselves',
    'parts = haveWholeWithinThemAsMoment',
    'whole = constitutesConnectingReference',
    'withoutWhole = noParts',
    'but = becauseSelfSubsistent',
    'connection = onlyExternalMoment',
    'connection = withRespectToWhichIndifferent',
    'but = parts',
    'but = asManifoldConcreteExistence',
    'but = collapseTogether',
    'concreteExistence = reflectionlessBeing',
    'parts = haveSelfSubsistence',
    'parts = onlyInReflectedUnity',
    'parts = whichIsUnity',
    'parts = asWellAsConcreteExistentManifoldness',
    'parts = haveSelfSubsistence',
    'parts = onlyInWhole',
    'but = whole',
    'but = atSameTimeSelfSubsistence',
    'but = whichIsOtherToParts',
  ],
  predicates: [
    { name: 'wholeRelation', args: ['parts'] },
  ],
  relations: [
    { predicate: 'have', from: 'parts', to: 'wholeWithinThemAsMoment' },
  ],
  candidateSummary: 'Parts likewise whole relation. Immediate as against reflected self-subsistence. Do not subsist in whole but for themselves. Have whole within them as moment. Whole constitutes connecting reference. Without whole no parts. But because self-subsistent, connection only external moment, with respect to which indifferent. But parts, as manifold concrete existence, collapse together. Concrete existence is reflectionless being. Have self-subsistence only in reflected unity, which is unity as well as concrete existent manifoldness. Have self-subsistence only in whole. But whole at same time self-subsistence which is other to parts.',
  provenance: {
    sourceChunk: 'wlp-4',
    sourceOp: 'wlp-op-4-parts-whole-relation',
  },
};

export const wlpOp5ReciprocalConditioning: LogicalOperation = {
  id: 'wlp-op-5-reciprocal-conditioning',
  chunkId: 'wlp-5',
  label: 'Reciprocal conditioning — unconditioned',
  clauses: [
    'wholeAndParts = reciprocallyConditionEachOther',
    'relation = higherThanReference',
    'relation = ofConditionedAndCondition',
    'reference = realized',
    'condition = essentialSelfSubsistenceOfConditioned',
    'condition = presupposedByLatter',
    'condition = asSuchOnlyImmediate',
    'condition = implicitlyPresupposed',
    'whole = throughConditionOfParts',
    'whole = entailsItOnlyInSoFarAs',
    'whole = hasPartsForPresupposition',
    'bothSides = positedAsConditioningEachOtherReciprocally',
    'each = immediateSelfSubsistence',
    'each = butSelfSubsistence',
    'each = equallyMediatedOrPosited',
    'each = throughOther',
    'wholeRelation = becauseOfReciprocity',
    'wholeRelation = isTurningBackOfConditioning',
    'wholeRelation = intoItself',
    'wholeRelation = nonRelative',
    'wholeRelation = unconditioned',
  ],
  predicates: [
    { name: 'unconditioned', args: ['wholeRelation'] },
  ],
  relations: [
    { predicate: 'condition', from: 'whole', to: 'parts' },
    { predicate: 'condition', from: 'parts', to: 'whole' },
  ],
  candidateSummary: 'Whole and parts reciprocally condition each other. Relation higher than reference of conditioned and condition. Reference realized: condition is essential self-subsistence of conditioned, presupposed by latter. Condition as such only immediate, implicitly presupposed. Whole, through condition of parts, entails it only in so far as has parts for presupposition. Both sides posited as conditioning each other reciprocally. Each immediate self-subsistence, but self-subsistence equally mediated or posited through other. Whole relation, because of reciprocity, is turning back of conditioning into itself, non-relative, unconditioned.',
  provenance: {
    sourceChunk: 'wlp-5',
    sourceOp: 'wlp-op-5-reciprocal-conditioning',
  },
};

export const wlpOp6TwoRespects: LogicalOperation = {
  id: 'wlp-op-6-two-respects',
  chunkId: 'wlp-6',
  label: 'Two respects — identity and indifference',
  clauses: [
    'inasmuchAs = eachSide',
    'inasmuchAs = hasSelfSubsistence',
    'inasmuchAs = notInIt',
    'inasmuchAs = butInOther',
    'only = oneIdentityOfTwo',
    'only = inWhichBothOnlyMoments',
    'but = inasmuchAs',
    'but = eachSelfSubsistentOnOwn',
    'two = areTwoSelfSubsistentConcreteExistences',
    'two = indifferentToEachOther',
  ],
  predicates: [
    { name: 'twoRespects', args: ['relation'] },
  ],
  relations: [
    { predicate: 'is', from: 'two', to: 'indifferentToEachOther' },
  ],
  candidateSummary: 'Inasmuch as each side has self-subsistence not in it but in other, only one identity of two in which both only moments. But inasmuch as each self-subsistent on own, two are two self-subsistent concrete existences indifferent to each other.',
  provenance: {
    sourceChunk: 'wlp-6',
    sourceOp: 'wlp-op-6-two-respects',
  },
};

export const wlpOp7EssentialIdentityEquality: LogicalOperation = {
  id: 'wlp-op-7-essential-identity-equality',
  chunkId: 'wlp-7',
  label: 'Essential identity — equality',
  clauses: [
    'inFirstRespect = essentialIdentityOfTwoSides',
    'whole = equalToParts',
    'parts = equalToWhole',
    'nothing = inWhole',
    'nothing = whichIsNotInParts',
    'nothing = inParts',
    'nothing = whichIsNotInWhole',
    'whole != abstractUnity',
    'whole = unityOfDiversifiedManifoldness',
    'unity = withinWhichManifoldHeldTogether',
    'unity = isDeterminateness',
    'unity = byVirtueOfWhich',
    'unity = latterIsParts',
    'relation = hasIndivisibleIdentity',
    'relation = andOnlyOneSelfSubsistence',
  ],
  predicates: [
    { name: 'equality', args: ['whole', 'parts'] },
  ],
  relations: [
    { predicate: 'equal', from: 'whole', to: 'parts' },
    { predicate: 'equal', from: 'parts', to: 'whole' },
  ],
  candidateSummary: 'In first respect, essential identity of two sides. Whole equal to parts and parts equal to whole. Nothing in whole which is not in parts, nothing in parts which is not in whole. Whole not abstract unity but unity of diversified manifoldness. Unity within which manifold held together is determinateness by virtue of which latter is parts. Relation has indivisible identity and only one self-subsistence.',
  provenance: {
    sourceChunk: 'wlp-7',
    sourceOp: 'wlp-op-7-essential-identity-equality',
  },
};

export const wlpOp8WholeEqualToPartsTautology: LogicalOperation = {
  id: 'wlp-op-8-whole-equal-to-parts-tautology',
  chunkId: 'wlp-8',
  label: 'Whole equal to parts — tautology',
  clauses: [
    'whole = equalToParts',
    'whole = butNotToThemAsParts',
    'whole = reflectedUnity',
    'parts = constituteDeterminateMoment',
    'parts = orOthernessOfUnity',
    'parts = diversifiedManifold',
    'whole != equalToThem',
    'whole != asSelfSubsistentDiversity',
    'whole = toThemTogether',
    'theirTogether = nothingElseButUnity',
    'theirTogether = wholeAsSuch',
    'inParts = wholeOnlyEqualToItself',
    'equality = expressesOnlyTautology',
    'equality = wholeAsWhole',
    'equality = equalNotToParts',
    'equality = butToWhole',
  ],
  predicates: [
    { name: 'tautology', args: ['equality'] },
  ],
  relations: [
    { predicate: 'expresses', from: 'equality', to: 'tautology' },
  ],
  candidateSummary: 'Whole equal to parts but not to them as parts. Whole is reflected unity. Parts constitute determinate moment or otherness of unity, diversified manifold. Whole not equal to them as self-subsistent diversity but to them together. Their "together" nothing else but unity, whole as such. In parts, whole only equal to itself. Equality expresses only tautology: whole as whole equal not to parts but to whole.',
  provenance: {
    sourceChunk: 'wlp-8',
    sourceOp: 'wlp-op-8-whole-equal-to-parts-tautology',
  },
};

export const wlpOp9PartsEqualToWholeTautology: LogicalOperation = {
  id: 'wlp-op-9-parts-equal-to-whole-tautology',
  chunkId: 'wlp-9',
  label: 'Parts equal to whole — tautology',
  clauses: [
    'parts = equalToWhole',
    'but = because',
    'but = asParts',
    'but = momentOfOtherness',
    'parts = notEqualToIt',
    'parts = asUnity',
    'but = oneOfWholesManifoldDeterminations',
    'but = mapsOverPart',
    'parts = equalToWhole',
    'parts = asManifold',
    'parts = asApportionedWhole',
    'parts = asParts',
    'sameTautology = partsAsParts',
    'sameTautology = equalNotToWholeAsSuch',
    'sameTautology = butInWhole',
    'sameTautology = toThemselves',
  ],
  predicates: [
    { name: 'tautology', args: ['sameTautology'] },
  ],
  relations: [
    { predicate: 'is', from: 'sameTautology', to: 'tautology' },
  ],
  candidateSummary: 'Parts equal to whole. But because, as parts, moment of otherness, not equal to it as unity. But one of whole\'s manifold determinations maps over part. Equal to whole as manifold, as apportioned whole, as parts. Same tautology: parts as parts equal not to whole as such but in whole, to themselves.',
  provenance: {
    sourceChunk: 'wlp-9',
    sourceOp: 'wlp-op-9-parts-equal-to-whole-tautology',
  },
};

export const wlpOp10FallApartDestroy: LogicalOperation = {
  id: 'wlp-op-10-fall-apart-destroy',
  chunkId: 'wlp-10',
  label: 'Fall apart — destroy themselves',
  clauses: [
    'wholeAndParts = fallIndifferentlyApart',
    'eachSide = refersOnlyToItself',
    'asHeldApart = destroyThemselves',
    'whole = indifferentTowardsParts',
    'whole = abstractIdentity',
    'whole = undifferentiatedInItself',
    'identity = wholeOnlyInasmuchAs',
    'identity = differentiatedInItself',
    'identity = ofReflection',
    'identity = hasShownThroughMovement',
    'identity = thatItHasReflectionIntoOther',
    'identity = forTruth',
    'parts = indifferentToUnity',
    'parts = onlyUnconnectedManifold',
    'parts = inherentlyOther',
    'parts = whichSublatesItself',
    'selfReference = ofEachSide',
    'selfReference = isSelfSubsistence',
    'but = selfSubsistence',
    'but = eachSideHasForItself',
    'but = isNegationOfRespectiveSelves',
    'eachSide = hasSelfSubsistence',
    'eachSide = notWithin',
    'eachSide = butInOtherSide',
    'other = constitutesSubsistence',
    'other = presupposedImmediate',
    'other = supposedToBeFirst',
    'but = first',
    'but = ofEachSide',
    'but = onlyFirst',
    'but = whichIsNotFirst',
    'but = hasBeginningInOther',
  ],
  predicates: [
    { name: 'destroyThemselves', args: ['whole', 'parts'] },
  ],
  relations: [
    { predicate: 'has', from: 'eachSide', to: 'selfSubsistenceInOther' },
  ],
  candidateSummary: 'Whole and parts fall indifferently apart. Each side refers only to itself. As held apart, destroy themselves. Whole indifferent towards parts is abstract identity, undifferentiated in itself. Identity whole only inasmuch as differentiated in itself. Identity of reflection has shown through movement that it has reflection into other for truth. Parts, indifferent to unity, only unconnected manifold, inherently other which sublates itself. Self-reference of each side is self-subsistence. But self-subsistence each side has for itself is negation of respective selves. Each side has self-subsistence not within but in other side. Other constitutes subsistence, presupposed immediate, supposed to be first. But first of each side only first which is not first, has beginning in other.',
  provenance: {
    sourceChunk: 'wlp-10',
    sourceOp: 'wlp-op-10-fall-apart-destroy',
  },
};

export const wlpOp11TruthOfRelationMediation: LogicalOperation = {
  id: 'wlp-op-11-truth-of-relation-mediation',
  chunkId: 'wlp-11',
  label: 'Truth of relation — mediation',
  clauses: [
    'truthOfRelation = consistsInMediation',
    'essence = negativeUnity',
    'essence = inWhichBoth',
    'essence = reflectedAndExistentImmediacy',
    'essence = equallySublated',
    'relation = contradiction',
    'relation = thatReturnsToGround',
    'relation = intoUnity',
    'relation = whichAsTurningBack',
    'relation = isReflectedUnity',
    'but = sinceEqually',
    'but = positedItselfAsSublated',
    'but = refersToItselfNegatively',
    'but = makesItselfIntoExistentImmediacy',
    'unity = negativeReference',
    'unity = inSoFarAsFirstAndImmediate',
    'unity = onlyIsAsMediatedByOther',
    'unity = andEquallyPosited',
    'other = existentImmediacy',
    'other = equallyOnlyAsSublated',
    'selfSubsistence = isFirst',
    'selfSubsistence = butOnlyInOrderToDisappear',
    'selfSubsistence = hasExistence',
    'selfSubsistence = whichIsPositedAndMediated',
  ],
  predicates: [
    { name: 'mediation', args: ['truthOfRelation'] },
  ],
  relations: [
    { predicate: 'consists', from: 'truthOfRelation', to: 'inMediation' },
  ],
  candidateSummary: 'Truth of relation consists in mediation. Essence is negative unity in which both reflected and existent immediacy equally sublated. Relation is contradiction that returns to ground, into unity which, as turning back, is reflected unity. But since equally posited itself as sublated, refers to itself negatively, makes itself into existent immediacy. Unity\'s negative reference, in so far as first and immediate, only is as mediated by other and equally posited. Other, existent immediacy, equally only as sublated. Self-subsistence is first, but only in order to disappear. Has existence which is posited and mediated.',
  provenance: {
    sourceChunk: 'wlp-11',
    sourceOp: 'wlp-op-11-truth-of-relation-mediation',
  },
};

export const wlpOp12TransitionForceExpressions: LogicalOperation = {
  id: 'wlp-op-12-transition-force-expressions',
  chunkId: 'wlp-12',
  label: 'Transition — force and expressions',
  clauses: [
    'determined = inThisWay',
    'relation = noLongerOneOfWholeAndParts',
    'previousImmediacy = ofSides',
    'previousImmediacy = passedOverIntoPositedness',
    'previousImmediacy = andMediation',
    'eachSide = posited',
    'eachSide = inSoFarAsImmediate',
    'eachSide = asSelfSublating',
    'eachSide = andPassingOverIntoOther',
    'eachSide = inSoFarAsNegativeReference',
    'eachSide = positedAsConditioned',
    'eachSide = throughOther',
    'eachSide = throughPositive',
    'immediateTransition = ofEach',
    'immediateTransition = equallyMediation',
    'immediateTransition = sublatingPosited',
    'immediateTransition = throughOther',
    'relation = ofWholeAndParts',
    'relation = passedOver',
    'relation = intoRelationOfForceAndItsExpressions',
  ],
  predicates: [
    { name: 'forceAndExpressions', args: ['relation'] },
  ],
  relations: [
    { predicate: 'passedOver', from: 'relation', to: 'intoRelationOfForceAndItsExpressions' },
  ],
  candidateSummary: 'Determined in this way, relation no longer one of whole and parts. Previous immediacy of sides passed over into positedness and mediation. Each side posited, in so far as immediate, as self-sublating and passing over into other. In so far as negative reference, posited as conditioned through other, through positive. Immediate transition of each equally mediation, sublating posited through other. Relation of whole and parts passed over into relation of force and its expressions.',
  provenance: {
    sourceChunk: 'wlp-12',
    sourceOp: 'wlp-op-12-transition-force-expressions',
  },
};

export const wholePartsOperations: LogicalOperation[] = [
  wlpOp1EssentialRelationTwoSides,
  wlpOp2RelationContainsContradiction,
  wlpOp3WholeSelfExternalized,
  wlpOp4PartsWholeRelation,
  wlpOp5ReciprocalConditioning,
  wlpOp6TwoRespects,
  wlpOp7EssentialIdentityEquality,
  wlpOp8WholeEqualToPartsTautology,
  wlpOp9PartsEqualToWholeTautology,
  wlpOp10FallApartDestroy,
  wlpOp11TruthOfRelationMediation,
  wlpOp12TransitionForceExpressions,
];

