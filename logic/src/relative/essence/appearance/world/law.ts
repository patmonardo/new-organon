/**
 * Logical Operations: The Law of Appearance
 *
 * Law is the positive element of mediation of what appears. It is the
 * simple identity of appearance with itself, the restful copy of the
 * concretely existing world. Law is essentiality.
 *
 * Dialectical Movement:
 * - Appearance as essence in concrete existence
 * - Law as positive element, kingdom of laws
 * - Law as essential form (not yet real form)
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// THE LAW OF APPEARANCE
// ============================================================================

export const lawOp1AppearanceEssence: LogicalOperation = {
  id: 'law-op-1-appearance-essence',
  chunkId: 'law-1',
  label: 'Appearance — essence in concrete existence',
  clauses: [
    'concreteExistence = immediacyOfBeing',
    'concreteExistence = toWhichEssenceHasRestoredItself',
    'inItself = immediacyIsReflectionOfEssenceIntoItself',
    'asConcreteExistence = essenceHasSteppedOutOfGround',
    'asConcreteExistence = whichPassedOverIntoIt',
    'concreteExistence = reflectedImmediacy',
    'inSoFarAs = withinItIsAbsoluteNegativity',
    'positedAsSuch = determinedItselfAsAppearance',
    'appearance = essenceInItsConcreteExistence',
    'essence = immediatelyPresentInIt',
    'notImmediate = butReflectedConcreteExistence',
    'notImmediate = constitutesMomentOfEssence',
    'concreteExistence = asEssentialConcreteExistence',
    'concreteExistence = isAppearance',
  ],
  predicates: [
    { name: 'appearance', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'is', from: 'appearance', to: 'essenceInItsConcreteExistence' },
  ],
  candidateSummary: 'Concrete existence is immediacy of being to which essence has restored itself. In itself immediacy is reflection of essence into itself. As concrete existence, essence has stepped out of ground which passed over into it. Concrete existence is reflected immediacy in so far as within it is absolute negativity. Posited as such, determined itself as appearance. Appearance is essence in its concrete existence. Essence immediately present in it. Not immediate but reflected concrete existence constitutes moment of essence. Concrete existence, as essential concrete existence, is appearance.',
  provenance: {
    sourceChunk: 'law-1',
    sourceOp: 'law-op-1-appearance-essence',
  },
};

export const lawOp2AppearanceOnly: LogicalOperation = {
  id: 'law-op-2-appearance-only',
  chunkId: 'law-2',
  label: 'Appearance — only appearance',
  clauses: [
    'something = onlyAppearance',
    'inSense = concreteExistenceIsOnlyPositedBeing',
    'inSense = notInAndForItself',
    'this = constitutesEssentiality',
    'have = negativityOfReflection',
    'have = natureOfEssence',
    'have = withinIt',
    'no = alienExternalReflection',
    'essentialityOfConcreteExistence = thatItIsAppearance',
    'essentialityOfConcreteExistence = concreteExistencesOwnTruth',
    'reflection = byVirtueOfWhichItIsThis',
    'reflection = isItsOwn',
  ],
  predicates: [
    { name: 'onlyAppearance', args: ['something'] },
  ],
  relations: [
    { predicate: 'is', from: 'essentialityOfConcreteExistence', to: 'concreteExistencesOwnTruth' },
  ],
  candidateSummary: 'Something is only appearance in sense concrete existence is only posited being, not in-and-for-itself. This constitutes essentiality: have negativity of reflection, nature of essence, within it. No alien, external reflection. Essentiality of concrete existence, that it is appearance, is concrete existence\'s own truth. Reflection by virtue of which it is this is its own.',
  provenance: {
    sourceChunk: 'law-2',
    sourceOp: 'law-op-2-appearance-only',
  },
};

export const lawOp3AppearanceHigherTruth: LogicalOperation = {
  id: 'law-op-3-appearance-higher-truth',
  chunkId: 'law-3',
  label: 'Appearance — higher truth',
  clauses: [
    'ifSaid = somethingIsOnlyAppearance',
    'meaning = contrastedWithIt',
    'meaning = immediateConcreteExistenceIsTruth',
    'then = appearanceIsHigherTruth',
    'appearance = concreteExistenceAsEssential',
    'whereas = concreteExistenceIsAppearance',
    'whereas = stillVoidOfEssence',
    'onlyContains = oneMoment',
    'onlyContains = concreteExistenceAsImmediate',
    'onlyContains = notYetNegativeReflection',
    'whenAppearance = saidToBeEssenceless',
    'thinks = immediateWerePositiveAndTrue',
    'but = immediateDoesNotContainEssentialTruth',
    'concreteExistence.ceases = toBeEssenceless',
    'concreteExistence.ceases = byPassingOverIntoAppearance',
  ],
  predicates: [
    { name: 'higherTruth', args: ['appearance'] },
  ],
  relations: [
    { predicate: 'is', from: 'appearance', to: 'higherTruth' },
  ],
  candidateSummary: 'If said something is only appearance, meaning contrasted with it immediate concrete existence is truth, then appearance is higher truth. It is concrete existence as essential, whereas concrete existence is appearance still void of essence. Only contains one moment: concrete existence as immediate, not yet negative reflection. When appearance said to be essenceless, thinks immediate were positive and true. But immediate does not contain essential truth. Concrete existence ceases to be essenceless by passing over into appearance.',
  provenance: {
    sourceChunk: 'law-3',
    sourceOp: 'law-op-3-appearance-higher-truth',
  },
};

export const lawOp4EssenceAppearsRealShine: LogicalOperation = {
  id: 'law-op-4-essence-appears-real-shine',
  chunkId: 'law-4',
  label: 'Essence appears — real shine',
  clauses: [
    'essence.reflectivelyShines = atFirstJustWithin',
    'essence.reflectivelyShines = inSimpleIdentity',
    'abstractReflection = pureMovementOfNothing',
    'abstractReflection = throughNothingBackToItself',
    'essence.appears = nowRealShine',
    'since = momentsOfShineHaveConcreteExistence',
    'appearance = thingAsNegativeMediationOfItselfWithItself',
    'differences = selfSubsistingMatters',
    'contradiction = immediateSubsistence',
    'contradiction = yetObtainingSubsistenceOnlyInAlienSelfSubsistence',
    'inNegationOfOwn = alsoInNegationOfAlienSelfSubsistence',
    'reflectiveShine = sameMediation',
    'fleetingMoments = obtainShapeOfImmediateSelfSubsistence',
    'immediateSelfSubsistence = reducedToMoment',
    'appearance = unityOfReflectiveShineAndConcreteExistence',
  ],
  predicates: [
    { name: 'realShine', args: ['essence'] },
  ],
  relations: [
    { predicate: 'is', from: 'appearance', to: 'unityOfReflectiveShineAndConcreteExistence' },
  ],
  candidateSummary: 'Essence reflectively shines at first just within, in simple identity. Abstract reflection, pure movement of nothing through nothing back to itself. Essence appears, now real shine, since moments of shine have concrete existence. Appearance is thing as negative mediation of itself with itself. Differences are self-subsisting matters. Contradiction: immediate subsistence, yet obtaining subsistence only in alien self-subsistence. In negation of own, also in negation of alien self-subsistence. Reflective shine is same mediation. Fleeting moments obtain shape of immediate self-subsistence. Immediate self-subsistence reduced to moment. Appearance is unity of reflective shine and concrete existence.',
  provenance: {
    sourceChunk: 'law-4',
    sourceOp: 'law-op-4-essence-appears-real-shine',
  },
};

export const lawOp5AppearanceDeterminesThreeMoments: LogicalOperation = {
  id: 'law-op-5-appearance-determines-three-moments',
  chunkId: 'law-5',
  label: 'Appearance determines itself — three moments',
  clauses: [
    'appearance.determines = itselfFurther',
    'concreteExistence = asEssential',
    'concreteExistence.differs = fromUnessential',
    'twoSides = referToEachOther',
    'first = simpleSelfIdentity',
    'first = containingDiverseContentDeterminations',
    'first = remainsSelfEqualInFluxOfAppearance',
    'this = lawOfAppearance',
    'second = lawSimpleInDiversity',
    'second = passesOverIntoOpposition',
    'second = essentialMomentOpposedToAppearanceItself',
    'second = confrontingWorldOfAppearance',
    'second = worldThatExistsInItself',
    'second = comesOntoScene',
    'third = oppositionReturnsIntoGround',
    'third = thatInItselfIsInAppearance',
    'third = thatAppears',
    'third = determinedAsTakenUpIntoBeingInItself',
    'appearance = becomesRelation',
  ],
  predicates: [
    { name: 'lawOfAppearance', args: ['first'] },
  ],
  relations: [
    { predicate: 'is', from: 'this', to: 'lawOfAppearance' },
    { predicate: 'becomes', from: 'appearance', to: 'relation' },
  ],
  candidateSummary: 'Appearance determines itself further. Concrete existence as essential differs from unessential. Two sides refer to each other. First, simple self-identity containing diverse content determinations. Remains self-equal in flux of appearance. This is law of appearance. Second, law simple in diversity passes over into opposition. Essential moment opposed to appearance itself. Confronting world of appearance, world that exists in itself comes onto scene. Third, opposition returns into ground. That in itself is in appearance. That appears determined as taken up into being-in-itself. Appearance becomes relation.',
  provenance: {
    sourceChunk: 'law-5',
    sourceOp: 'law-op-5-appearance-determines-three-moments',
  },
};

export const lawOp6LawAppearanceMediated: LogicalOperation = {
  id: 'law-op-6-law-appearance-mediated',
  chunkId: 'law-6',
  label: 'A. THE LAW OF APPEARANCE — appearance mediated',
  clauses: [
    'appearance = concreteExistentMediated',
    'appearance = throughItsNegation',
    'appearance = whichConstitutesSubsistence',
    'negation = anotherSelfSubsistent',
    'negation = butEssentiallySublated',
    'concreteExistent = turningBackIntoItself',
    'concreteExistent = throughNegation',
    'concreteExistent = andNegationOfNegation',
    'has = essentialSelfSubsistence',
    'equally = immediatelyAbsolutePositedness',
    'appearance = concreteExistenceWithEssentiality',
    'appearance = positednessWithGround',
    'ground = negation',
    'ground = otherSelfSubsistent',
    'ground = equallyOnlyPositedness',
    'concreteExistent = reflectedIntoOther',
    'concreteExistent.has = otherForGround',
    'groundItself = onlyToBeReflectedIntoAnother',
    'essentialSelfSubsistence = returnOfNothing',
    'essentialSelfSubsistence = throughNothingBackToItself',
    'selfSubsistence = onlyReflectiveShineOfEssence',
    'linkage = consistsInReciprocalNegation',
    'subsistenceOfOne = notSubsistenceOfOther',
    'subsistenceOfOne = butItsPositedness',
    'connectionOfPositedness = aloneConstitutesSubsistence',
  ],
  predicates: [
    { name: 'mediated', args: ['appearance'] },
  ],
  relations: [
    { predicate: 'is', from: 'appearance', to: 'concreteExistenceWithEssentiality' },
  ],
  candidateSummary: 'Appearance is concrete existent mediated through its negation, which constitutes subsistence. Negation is another self-subsistent, but essentially sublated. Concrete existent is turning back into itself through negation and negation of negation. Has essential self-subsistence, equally immediately absolute positedness. Appearance is concrete existence with essentiality, positedness with ground. Ground is negation, other self-subsistent equally only positedness. Concrete existent reflected into other, has other for ground. Ground itself only to be reflected into another. Essential self-subsistence is return of nothing through nothing back to itself. Self-subsistence only reflective shine of essence. Linkage consists in reciprocal negation. Subsistence of one is not subsistence of other but its positedness. Connection of positedness alone constitutes subsistence.',
  provenance: {
    sourceChunk: 'law-6',
    sourceOp: 'law-op-6-law-appearance-mediated',
  },
};

export const lawOp7PositiveIdentityEssentialContent: LogicalOperation = {
  id: 'law-op-7-positive-identity-essential-content',
  chunkId: 'law-7',
  label: 'Positive identity — essential content',
  clauses: [
    'inNegativeMediation = immediatelyContained',
    'inNegativeMediation = positiveIdentityOfConcreteExistent',
    'inNegativeMediation = withItself',
    'concreteExistent != positedness',
    'concreteExistent != visAVisEssentialGround',
    'concreteExistent = butPositedness',
    'concreteExistent = referringItselfToPositedness',
    'reflectiveShine = onlyInReflectiveShine',
    'inNegation = inOtherWhichIsSublated',
    'inNegation = refersToItself',
    'selfIdentical = orPositiveEssentiality',
    'essentialContent = hasTwoSides',
    'first = inFormOfPositedness',
    'first = orExternalImmediacy',
    'second = positednessAsSelfIdentical',
    'accordingToFirst = determinateBeing',
    'accordingToFirst = accidental',
    'accordingToFirst = unessential',
    'accordingToFirst = subjectToTransition',
    'accordingToOther = simpleContentDetermination',
    'accordingToOther = exemptedFromFlux',
    'accordingToOther = permanentElement',
  ],
  predicates: [
    { name: 'positiveIdentity', args: ['concreteExistent'] },
  ],
  relations: [
    { predicate: 'is', from: 'concreteExistent', to: 'positedness' },
  ],
  candidateSummary: 'In negative mediation, immediately contained positive identity of concrete existent with itself. Concrete existent not positedness vis-à-vis essential ground but positedness referring itself to positedness. Reflective shine only in reflective shine. In negation, in other which is sublated, refers to itself. Self-identical or positive essentiality. Essential content has two sides: in form of positedness or external immediacy, and positedness as self-identical. According to first, determinate being, accidental, unessential, subject to transition. According to other, simple content determination exempted from flux, permanent element.',
  provenance: {
    sourceChunk: 'law-7',
    sourceOp: 'law-op-7-positive-identity-essential-content',
  },
};

export const lawOp8ContentCompleteDeterminateness: LogicalOperation = {
  id: 'law-op-8-content-complete-determinateness',
  chunkId: 'law-8',
  label: 'Content — complete determinateness',
  clauses: [
    'content = reflectionOfAppearance',
    'content = negativeDeterminateBeing',
    'content = intoItself',
    'contains = determinatenessEssentially',
    'appearance = multifariousDiversity',
    'appearance = unessentialManifoldness',
    'reflectedContent = manifoldnessReducedToSimpleDifference',
    'determinateEssentialContent = completeDeterminateness',
    'determinateEssentialContent = oneAndItsOther',
    'each.has = subsistenceInOther',
    'each.has = butOnlyInOthersNonSubsistence',
    'contradiction = sublatesItself',
    'reflectionIntoItself = identityOfTwoSidedSubsistence',
    'positednessOfOne = alsoPositednessOfOther',
    'two = constituteOneSubsistence',
    'each = differentContent',
    'each = indifferentToOther',
    'inEssentialSide = negativityGoneBackIntoIdentity',
    'indifferentSubsistence = whichIsNotSublatednessOfOther',
    'indifferentSubsistence = butItsSubsistence',
    'thisUnity = lawOfAppearance',
  ],
  predicates: [
    { name: 'completeDeterminateness', args: ['content'] },
  ],
  relations: [
    { predicate: 'is', from: 'thisUnity', to: 'lawOfAppearance' },
  ],
  candidateSummary: 'Content is reflection of appearance, negative determinate being, into itself. Contains determinateness essentially. Appearance is multifarious diversity, unessential manifoldness. Reflected content is manifoldness reduced to simple difference. Determinate essential content is complete determinateness: one and its other. Each has subsistence in other, but only in other\'s non-subsistence. Contradiction sublates itself. Reflection into itself is identity of two-sided subsistence. Positedness of one also positedness of other. Two constitute one subsistence, each different content indifferent to other. In essential side, negativity gone back into identity. Indifferent subsistence which is not sublatedness of other but its subsistence. This unity is law of appearance.',
  provenance: {
    sourceChunk: 'law-8',
    sourceOp: 'law-op-8-content-complete-determinateness',
  },
};

export const lawOp9LawPositiveElement: LogicalOperation = {
  id: 'law-op-9-law-positive-element',
  chunkId: 'law-9',
  label: 'Law — positive element',
  clauses: [
    'law = positiveElementOfMediation',
    'law = ofWhatAppears',
    'appearance = concreteExistence',
    'appearance = asNegativeSelfMediation',
    'concreteExistent = throughOwnNonSubsistence',
    'concreteExistent = throughOther',
    'concreteExistent = throughNonSubsistenceOfOther',
    'concreteExistent = mediatedWithItself',
    'contains = first',
    'contains = merelyReflectiveShiningAndDisappearing',
    'contains = unessentialAppearance',
    'contains = second',
    'contains = persistenceOrLaw',
    'each = concretelyExistsInSublationOfOther',
    'positedness = asNegativity',
    'positedness = atSameTimeIdenticalPositivePositedness',
    'positedness = ofBoth',
  ],
  predicates: [
    { name: 'positiveElement', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'positiveElementOfMediation' },
  ],
  candidateSummary: 'Law is positive element of mediation of what appears. Appearance is concrete existence as negative self-mediation. Concrete existent through own non-subsistence, through other, through non-subsistence of other, mediated with itself. Contains, first, merely reflective shining and disappearing, unessential appearance. Second, persistence or law. Each concretely exists in sublation of other. Positedness as negativity at same time identical positive positedness of both.',
  provenance: {
    sourceChunk: 'law-9',
    sourceOp: 'law-op-9-law-positive-element',
  },
};

export const lawOp10LawOpposedToImmediacy: LogicalOperation = {
  id: 'law-op-10-law-opposed-to-immediacy',
  chunkId: 'law-10',
  label: 'Law — opposed to immediacy',
  clauses: [
    'permanentSubsistence = appearanceObtainsInLaw',
    'first = opposedToImmediacyOfBeing',
    'first = whichConcreteExistenceHas',
    'immediacy = inItselfReflected',
    'immediacy = groundGoneBackIntoItself',
    'inAppearance = simpleImmediacy',
    'inAppearance = distinguishedFromReflectedImmediacy',
    'thatFirst = beganToSeparateInThing',
    'concretelyExistingThing = inDissolution',
    'concretelyExistingThing = hasBecomeThisOpposition',
    'positiveElement = ofDissolution',
    'positiveElement = selfIdentityOfWhatAppears',
    'positiveElement = positedness',
    'positiveElement = inPositednessOfItsOther',
  ],
  predicates: [
    { name: 'opposedToImmediacy', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'positiveElement', to: 'selfIdentityOfWhatAppears' },
  ],
  candidateSummary: 'Permanent subsistence appearance obtains in law. First, opposed to immediacy of being which concrete existence has. Immediacy is in itself reflected, ground gone back into itself. In appearance simple immediacy distinguished from reflected immediacy. That first began to separate in "thing." Concretely existing thing in dissolution has become this opposition. Positive element of dissolution is self-identity of what appears, positedness in positedness of its other.',
  provenance: {
    sourceChunk: 'law-10',
    sourceOp: 'law-op-10-law-opposed-to-immediacy',
  },
};

export const lawOp11LawPositedness: LogicalOperation = {
  id: 'law-op-11-law-positedness',
  chunkId: 'law-11',
  label: 'Law — positedness',
  clauses: [
    'second = reflectedImmediacy',
    'second = determinedAsPositedness',
    'second = overAgainstImmediateDeterminateBeing',
    'positedness = essentialAndTruePositive',
    'germanGesetz = containsNoteOfPositedness',
    'germanGesetz = orGesetztsein',
    'inPositedness = liesEssentialConnection',
    'inPositedness = ofTwoSidesOfDifference',
    'diverseContent = eachImmediate',
    'diverseContent = withRespectToOther',
    'asReflection = ofDisappearingContent',
    'asEssentialDifference = simple',
    'asEssentialDifference = selfReferringDeterminations',
    'neither = immediate',
    'neither = justForItself',
    'but = essentialPositedness',
    'only = toExtentOtherIs',
  ],
  predicates: [
    { name: 'positedness', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'positedness', to: 'essentialAndTruePositive' },
  ],
  candidateSummary: 'Second, reflected immediacy determined as positedness over against immediate determinate being. Positedness is essential and true positive. German Gesetz contains note of positedness or Gesetztsein. In positedness lies essential connection of two sides of difference. Diverse content, each immediate with respect to other. As reflection of disappearing content. As essential difference, simple, self-referring determinations. Neither immediate, just for itself, but essential positedness. Only to extent other is.',
  provenance: {
    sourceChunk: 'law-11',
    sourceOp: 'law-op-11-law-positedness',
  },
};

export const lawOp12AppearanceLawSameContent: LogicalOperation = {
  id: 'law-op-12-appearance-law-same-content',
  chunkId: 'law-12',
  label: 'Appearance and law — same content',
  clauses: [
    'third = appearanceAndLaw',
    'third = haveOneAndSameContent',
    'law = reflectionOfAppearance',
    'law = intoSelfIdentity',
    'appearance = asImmediateWhichIsNull',
    'appearance = opposedToImmanentlyReflected',
    'distinguished = accordingToForm',
    'reflection = essentialIdentityOfAppearanceAndReflection',
    'natureOfReflection = inPositednessSelfIdentical',
    'natureOfReflection = andIndifferentToDifference',
    'content = continuousFromAppearanceToLaw',
    'content = constitutesSubstrateOfAppearance',
    'law = substrateItself',
    'appearance = sameContent',
    'appearance = butContainsMore',
    'appearance = unessentialContent',
    'formDetermination = isContent',
    'concreteExistence = thinghoodWithPropertiesAndMatters',
    'content = whoseSelfSubsistingImmediacy',
    'content = isAlsoOnlyNonSubsistence',
    'selfIdentityOfContent = inNonSubsistence',
    'selfIdentityOfContent = isOther',
    'selfIdentityOfContent = essentialContent',
    'identity = substrate',
    'identity = constitutesLaw',
    'identity = isAppearancesOwnMoment',
    'identity = positiveSideOfEssentiality',
  ],
  predicates: [
    { name: 'sameContent', args: ['appearance', 'law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'substrateItself' },
  ],
  candidateSummary: 'Third, appearance and law have one and same content. Law is reflection of appearance into self-identity. Appearance, as immediate which is null, opposed to immanently reflected. Distinguished according to form. Reflection is essential identity of appearance and reflection. Nature of reflection: in positedness self-identical and indifferent to difference. Content continuous from appearance to law. Content constitutes substrate of appearance. Law is substrate itself. Appearance is same content but contains more: unessential content. Form determination is content. Concrete existence is thinghood with properties and matters. Content whose self-subsisting immediacy is also only non-subsistence. Self-identity of content in non-subsistence is other, essential content. Identity, substrate, constitutes law. Is appearance\'s own moment, positive side of essentiality.',
  provenance: {
    sourceChunk: 'law-12',
    sourceOp: 'law-op-12-appearance-law-same-content',
  },
};

export const lawOp13LawKingdomOfLaws: LogicalOperation = {
  id: 'law-op-13-law-kingdom-of-laws',
  chunkId: 'law-13',
  label: 'Law — kingdom of laws',
  clauses: [
    'law != beyondAppearance',
    'law = immediatelyPresentInIt',
    'kingdomOfLaws = restfulCopy',
    'kingdomOfLaws = ofConcretelyExistingOrAppearingWorld',
    'two = areOneTotality',
    'concretelyExistingWorld = itselfKingdomOfLaws',
    'simpleIdentity = selfIdenticalInPositedness',
    'simpleIdentity = orSelfDissolvingSelfSubsistence',
    'inLaw = concreteExistenceReturnsToGround',
    'appearance.contains = both',
    'appearance.contains = simpleGround',
    'appearance.contains = dissolvingMovementOfAppearingUniverse',
    'law = essentiality',
  ],
  predicates: [
    { name: 'kingdomOfLaws', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'essentiality' },
  ],
  candidateSummary: 'Law not beyond appearance but immediately present in it. Kingdom of laws is restful copy of concretely existing or appearing world. Two are one totality. Concretely existing world is itself kingdom of laws. Simple identity, self-identical in positedness or self-dissolving self-subsistence. In law, concrete existence returns to ground. Appearance contains both: simple ground and dissolving movement of appearing universe. Law is essentiality.',
  provenance: {
    sourceChunk: 'law-13',
    sourceOp: 'law-op-13-law-kingdom-of-laws',
  },
};

export const lawOp14LawEssentialAppearance: LogicalOperation = {
  id: 'law-op-14-law-essential-appearance',
  chunkId: 'law-14',
  label: 'Law — essential appearance',
  clauses: [
    'law = essentialAppearance',
    'lattersReflection = intoItselfInPositedness',
    'identicalContent = ofItselfAndUnessentialConcreteExistence',
    'identity = immediate',
    'identity = simpleIdentity',
    'law = indifferentWithRespectToConcreteExistence',
    'appearance = stillHasAnotherContent',
    'appearance = contrastedWithContentOfLaw',
    'appearance = unessential',
    'appearance = returnIntoLatter',
    'forLaw = originalStartingPoint',
    'forLaw = notPositedByIt',
    'forLaw = externallyBoundUpWithLaw',
    'appearance = aggregateOfMoreDetailedDeterminations',
    'belong = toThis',
    'belong = orConcrete',
    'notContained = inLaw',
    'determined = eachByOther',
  ],
  predicates: [
    { name: 'essentialAppearance', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'essentialAppearance' },
  ],
  candidateSummary: 'Law is essential appearance. Latter\'s reflection into itself in positedness. Identical content of itself and unessential concrete existence. Identity immediate, simple identity. Law indifferent with respect to concrete existence. Appearance still has another content contrasted with content of law. Unessential and return into latter. For law original starting point not posited by it. Externally bound up with law. Appearance is aggregate of more detailed determinations. Belong to "this" or concrete, not contained in law. Determined each by other.',
  provenance: {
    sourceChunk: 'law-14',
    sourceOp: 'law-op-14-law-essential-appearance',
  },
};

export const lawOp15LawRestlessForm: LogicalOperation = {
  id: 'law-op-15-law-restless-form',
  chunkId: 'law-15',
  label: 'Law — restless form',
  clauses: [
    'thatWhichAppearance = containsDistinctFromLaw',
    'thatWhichAppearance = determinedAsPositive',
    'thatWhichAppearance = orAnotherContent',
    'but = essentiallyNegative',
    'form = andMovement',
    'form = belongsToAppearance',
    'kingdomOfLaws = restfulContentOfAppearance',
    'appearance = sameContent',
    'appearance = butDisplayedInRestlessFlux',
    'appearance = reflectionIntoOther',
    'law = asNegative',
    'law = relentlesslySelfMutatingConcreteExistence',
    'movement = ofPassingOverIntoOpposite',
    'movement = selfSublation',
    'movement = returnIntoUnity',
    'restlessForm = orNegativity',
    'restlessForm = doesNotContainLaw',
    'appearance = totality',
    'appearance.contains = law',
    'appearance.contains = butMore',
    'appearance.contains = momentOfSelfMovingForm',
  ],
  predicates: [
    { name: 'restlessForm', args: ['appearance'] },
  ],
  relations: [
    { predicate: 'contains', from: 'appearance', to: 'law' },
  ],
  candidateSummary: 'That which appearance contains distinct from law determined as positive or another content. But essentially negative. Form and movement belongs to appearance. Kingdom of laws is restful content of appearance. Appearance is same content but displayed in restless flux. Reflection-into-other. Law as negative, relentlessly self-mutating concrete existence. Movement of passing over into opposite, self-sublation and return into unity. Restless form or negativity does not contain law. Appearance is totality, contains law but more: moment of self-moving form.',
  provenance: {
    sourceChunk: 'law-15',
    sourceOp: 'law-op-15-law-restless-form',
  },
};

export const lawOp16LawDiversityNecessity: LogicalOperation = {
  id: 'law-op-16-law-diversity-necessity',
  chunkId: 'law-16',
  label: 'Law — diversity and necessity',
  clauses: [
    'shortcoming = manifestedInLaw',
    'shortcoming = inMereDiversityAtFirst',
    'consequent = internalIndifferenceOfContent',
    'identityOfSides = immediate',
    'identityOfSides = andHenceInner',
    'identityOfSides = notYetNecessary',
    'inLaw = twoContentDeterminations',
    'inLaw = essentiallyBoundTogether',
    'example = spatialAndTemporalMagnitudes',
    'example = inLawOfFallingBodies',
    'traversedSpaces = varyAsSquaresOfElapsedTimes',
    'boundTogether = connectionAtFirstOnlyImmediate',
    'boundTogether = posited',
    'essentialUnity = wouldBeNegativity',
    'essentialUnity = eachContainsOtherInIt',
    'butInLaw = essentialUnityHasNotYetComeToFore',
    'notContained = inConceptOfSpace',
    'notContained = thatTimeCorrespondsAsSquare',
    'time.refers = toSpace',
    'space.refers = toTime',
    'doesNotLie = inDeterminationOfTimeItself',
    'time = canBeRepresentedWithoutSpace',
    'space = withoutTime',
    'one = comesToOtherExternally',
    'externalReference = isMovement',
    'moreParticularDetermination = indifferent',
    'law = drawnFromExperience',
    'law = immediate',
    'proof = required',
    'proof = mediation',
    'toKnow = lawNotOnlyOccurs',
    'toKnow = butIsNecessary',
    'lawAsSuch = doesNotContainProof',
    'lawAsSuch = andObjectiveNecessity',
  ],
  predicates: [
    { name: 'diversity', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'drawnFromExperience' },
  ],
  candidateSummary: 'Shortcoming manifested in law in mere diversity at first. Consequent internal indifference of content. Identity of sides immediate and hence inner, not yet necessary. In law two content determinations essentially bound together. Example: spatial and temporal magnitudes in law of falling bodies. Traversed spaces vary as squares of elapsed times. Bound together, connection at first only immediate, posited. Essential unity would be negativity: each contains other in it. But in law essential unity has not yet come to fore. Not contained in concept of space that time corresponds as square. Time refers to space and space to time does not lie in determination of time itself. Time can be represented without space, space without time. One comes to other externally, external reference is movement. More particular determination indifferent. Law drawn from experience, immediate. Proof required, mediation, to know law not only occurs but is necessary. Law as such does not contain proof and objective necessity.',
  provenance: {
    sourceChunk: 'law-16',
    sourceOp: 'law-op-16-law-diversity-necessity',
  },
};

export const lawOp17LawEssentialForm: LogicalOperation = {
  id: 'law-op-17-law-essential-form',
  chunkId: 'law-17',
  label: 'Law — essential form',
  clauses: [
    'law = onlyPositiveEssentialityOfAppearance',
    'law != negativeEssentiality',
    'law != accordingToWhich',
    'law != contentDeterminationsAreMomentsOfForm',
    'law != passOverIntoOther',
    'law != inOwnSelvesNotThemselves',
    'law != butTheirOther',
    'inLaw = althoughPositednessOfOneSide',
    'inLaw = isPositednessOfOtherSide',
    'content = ofTwoSides',
    'content = indifferentToConnection',
    'content = doesNotContainPositednessInIt',
    'law = indeedEssentialForm',
    'law = butNotAsYetRealForm',
    'law = whichIsReflectedIntoSidesAsContent',
  ],
  predicates: [
    { name: 'essentialForm', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'essentialForm' },
  ],
  candidateSummary: 'Law is only positive essentiality of appearance. Not negative essentiality according to which content determinations are moments of form. Pass over into other, in own selves not themselves but their other. In law, although positedness of one side is positedness of other side, content of two sides indifferent to connection. Does not contain positedness in it. Law is indeed essential form, but not as yet real form which is reflected into sides as content.',
  provenance: {
    sourceChunk: 'law-17',
    sourceOp: 'law-op-17-law-essential-form',
  },
};

export const lawOperations: LogicalOperation[] = [
  lawOp1AppearanceEssence,
  lawOp2AppearanceOnly,
  lawOp3AppearanceHigherTruth,
  lawOp4EssenceAppearsRealShine,
  lawOp5AppearanceDeterminesThreeMoments,
  lawOp6LawAppearanceMediated,
  lawOp7PositiveIdentityEssentialContent,
  lawOp8ContentCompleteDeterminateness,
  lawOp9LawPositiveElement,
  lawOp10LawOpposedToImmediacy,
  lawOp11LawPositedness,
  lawOp12AppearanceLawSameContent,
  lawOp13LawKingdomOfLaws,
  lawOp14LawEssentialAppearance,
  lawOp15LawRestlessForm,
  lawOp16LawDiversityNecessity,
  lawOp17LawEssentialForm,
];

