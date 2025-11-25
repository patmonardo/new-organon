/**
 * Logical Operations: The Constitution of the Thing Out of Matters
 *
 * Property transitions into matter. The thing consists of self-subsistent matters.
 * This is the FormProcessor's analysis of how things are constituted from their
 * essential properties as matters.
 *
 * Dialectical Movement:
 * - Transition of property into matter: chemistry
 * - Necessity of transition: property as essential
 * - Thing progressed to determinateness
 * - "This": complete determinateness
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// THE CONSTITUTION OF THE THING OUT OF MATTERS
// ============================================================================

export const matOp1TransitionChemistry: LogicalOperation = {
  id: 'mat-op-1-transition-chemistry',
  chunkId: 'mat-1',
  label: 'Transition of property into matter — chemistry',
  clauses: [
    'transition = ofPropertyIntoMatter',
    'transition = orSelfSubsistentStuff',
    'transition = isFamiliarTransition',
    'transition = performedOnSensibleMatter',
    'transition = byChemistry',
    'seeks = toRepresentProperties',
    'seeks = ofColorSmellEtc',
    'seeks = asLuminousMatter',
    'seeks = coloringMatter',
    'seeks = odorificMatter',
    'seeks = sourBitterMatter',
    'assumes = others',
    'assumes = likeCalorificMatter',
    'assumes = electricalMagneticMatter',
    'assumes = inConviction',
    'assumes = thatHasGottenHoldOfProperties',
    'assumes = asTheyTrulyAre',
    'saying = thatThingsConsist',
    'saying = ofVariousMattersOrStuffs',
    'careful = aboutCallingMattersOrStuffs',
    'careful = things',
    'distinction = madeBetweenThings',
    'distinction = andComponents',
    'distinction = withoutExactStatement',
    'distinction = whetherComponentsAreThings',
    'distinction = orHalfThings',
    'they = areAtLeastConcretes',
    'they = inGeneral',
  ],
  predicates: [
    { name: 'transition', args: ['property', 'matter'] },
  ],
  relations: [
    { predicate: 'transitions', from: 'property', to: 'matter' },
  ],
  candidateSummary: 'Transition of property into matter or self-subsistent stuff is familiar transition performed on sensible matter by chemistry. Seeks to represent properties of color, smell, etc. as luminous matter, coloring matter, odorific matter, sour, bitter matter. Assumes others like calorific matter, electrical, magnetic matter, in conviction that has gotten hold of properties as they truly are. Saying that things consist of various matters or stuffs. Careful about calling matters or stuffs "things." Distinction made between things and components without exact statement whether components are things or half-things. They are at least concretes in general.',
  provenance: {
    sourceChunk: 'mat-1',
    sourceOp: 'mat-op-1-transition-chemistry',
  },
};

export const matOp2NecessityOfTransition: LogicalOperation = {
  id: 'mat-op-2-necessity-of-transition',
  chunkId: 'mat-2',
  label: 'Necessity of transition — property as essential',
  clauses: [
    'necessity = ofMakingTransition',
    'necessity = fromPropertiesToMatters',
    'necessity = orAssumingProperties',
    'necessity = areTrulyMatters',
    'necessity = resultedFromFact',
    'necessity = theyAreWhatIsEssential',
    'necessity = inThings',
    'necessity = andConsequently',
    'necessity = theirTrueSelfSubsistence',
    'atSameTime = reflectionOfProperty',
    'atSameTime = intoItself',
    'atSameTime = constitutesOnlyOneSide',
    'atSameTime = ofWholeReflection',
    'atSameTime = sublationOfDistinction',
    'atSameTime = andContinuityOfProperty',
    'atSameTime = concreteExistenceForOther',
    'atSameTime = withItself',
    'thinghood = asImmanentNegativeReflection',
    'thinghood = andDistinguishing',
    'thinghood = thatRepelsItself',
    'thinghood = fromOther',
    'thinghood = hasBeenReduced',
    'thinghood = toUnessentialMoment',
    'atSameTime = hasFurtherDeterminedItself',
  ],
  predicates: [
    { name: 'necessity', args: ['transition'] },
  ],
  relations: [
    { predicate: 'is', from: 'necessity', to: 'ofMakingTransition' },
  ],
  candidateSummary: 'Necessity of making transition from properties to matters, or assuming properties are truly matters, resulted from fact they are what is essential in things and consequently their true self-subsistence. At same time, reflection of property into itself constitutes only one side of whole reflection: sublation of distinction and continuity of property (concrete existence for other) with itself. Thinghood, as immanent negative reflection and distinguishing that repels itself from other, has been reduced to unessential moment. At same time, has further determined itself.',
  provenance: {
    sourceChunk: 'mat-2',
    sourceOp: 'mat-op-2-necessity-of-transition',
  },
};

export const matOp3FirstNegativeMoment: LogicalOperation = {
  id: 'mat-op-3-first-negative-moment',
  chunkId: 'mat-3',
  label: 'First — negative moment preserved',
  clauses: [
    'negativeMoment = hasPreservedItself',
    'property = hasBecomeMatter',
    'property = continuousWithItself',
    'property = andSelfSubsisting',
    'property = onlyInasmuchAs',
    'property = differenceOfThings',
    'property = hasSublatedItself',
    'continuity = ofProperty',
    'continuity = inOthernessItself',
    'continuity = containsMomentOfNegative',
    'asNegativeUnity = selfSubsistence',
    'asNegativeUnity = isAtSameTime',
    'asNegativeUnity = restoredSomething',
    'asNegativeUnity = ofThinghood',
    'negativeSelfSubsistence = versusPositiveSelfSubsistence',
    'negativeSelfSubsistence = ofStuff',
  ],
  predicates: [
    { name: 'negativeMoment', args: ['property'] },
  ],
  relations: [
    { predicate: 'is', from: 'negativeMoment', to: 'preserved' },
  ],
  candidateSummary: 'Negative moment has preserved itself. Property has become matter continuous with itself and self-subsisting only inasmuch as difference of things has sublated itself. Continuity of property in otherness itself contains moment of negative. As negative unity, self-subsistence is at same time restored something of thinghood. Negative self-subsistence versus positive self-subsistence of stuff.',
  provenance: {
    sourceChunk: 'mat-3',
    sourceOp: 'mat-op-3-first-negative-moment',
  },
};

export const matOp4SecondThingProgressed: LogicalOperation = {
  id: 'mat-op-4-second-thing-progressed',
  chunkId: 'mat-4',
  label: 'Second — thing progressed to determinateness',
  clauses: [
    'thing = hasProgressed',
    'thing = fromIndeterminacy',
    'thing = toFullDeterminateness',
    'asThingInItself = abstractIdentity',
    'asThingInItself = simpleNegativeConcreteExistence',
    'asThingInItself = concreteExistence',
    'asThingInItself = determinedAsIndeterminate',
    'then = determinedThroughProperties',
    'then = byVirtueOfWhich',
    'then = supposedToBeDistinguished',
    'then = fromOtherThings',
    'but = sinceThroughProperty',
    'but = thingRatherContinuous',
    'but = withOtherThings',
    'but = imperfectDistinction',
    'but = sublated',
    'thing = returnedIntoItself',
    'now = determinedAsDetermined',
    'now = determinedInItself',
    'now = orIsThisThing',
  ],
  predicates: [
    { name: 'determinateness', args: ['thing'] },
  ],
  relations: [
    { predicate: 'progressed', from: 'thing', to: 'toFullDeterminateness' },
  ],
  candidateSummary: 'Thing has progressed from indeterminacy to full determinateness. As thing in itself, abstract identity, simple negative concrete existence, concrete existence determined as indeterminate. Then determined through properties, by virtue of which supposed to be distinguished from other things. But since through property thing rather continuous with other things, imperfect distinction sublated. Thing returned into itself. Now determined as determined, determined in itself or is this thing.',
  provenance: {
    sourceChunk: 'mat-4',
    sourceOp: 'mat-op-4-second-thing-progressed',
  },
};

export const matOp5ThirdTurningBack: LogicalOperation = {
  id: 'mat-op-5-third-turning-back',
  chunkId: 'mat-5',
  label: 'Third — turning back unessential',
  clauses: [
    'turningBack = intoItself',
    'turningBack = thoughSelfReferringDetermination',
    'turningBack = atSameTime',
    'turningBack = unessentialDetermination',
    'selfContinuousSubsistence = makesUp',
    'selfContinuousSubsistence = selfSubsistentMatter',
    'selfContinuousSubsistence = inWhich',
    'selfContinuousSubsistence = differenceOfThings',
    'selfContinuousSubsistence = determinatenessExisting',
    'selfContinuousSubsistence = inAndForItself',
    'selfContinuousSubsistence = isSublated',
    'selfContinuousSubsistence = andIsSomethingExternal',
    'therefore = althoughThing',
    'therefore = asThisThing',
    'therefore = isCompleteDeterminateness',
    'determinateness = isSuch',
    'determinateness = inElementOfInessentiality',
  ],
  predicates: [
    { name: 'unessential', args: ['turningBack'] },
  ],
  relations: [
    { predicate: 'is', from: 'turningBack', to: 'unessentialDetermination' },
  ],
  candidateSummary: 'Turning back into itself, though self-referring determination, at same time unessential determination. Self-continuous subsistence makes up self-subsistent matter in which difference of things, determinateness existing in and for itself, is sublated and is something external. Therefore, although thing as this thing is complete determinateness, determinateness is such in element of inessentiality.',
  provenance: {
    sourceChunk: 'mat-5',
    sourceOp: 'mat-op-5-third-turning-back',
  },
};

export const matOp6MovementOfProperty: LogicalOperation = {
  id: 'mat-op-6-movement-of-property',
  chunkId: 'mat-6',
  label: 'Movement of property — two moments',
  clauses: [
    'property = isNotOnlyExternalDetermination',
    'property = butConcreteExistence',
    'property = immediatelyExistingInItself',
    'unity = ofExternalityAndEssentiality',
    'unity = repelsItselfFromItself',
    'unity = containsReflectionIntoItself',
    'unity = andReflectionIntoOther',
    'onOneHand = determinationAsSimple',
    'onOneHand = selfIdenticalAndSelfReferring',
    'onOneHand = selfSubsistent',
    'onOneHand = inWhichNegativeUnity',
    'onOneHand = oneOfThing',
    'onOneHand = isSublated',
    'onOtherHand = determination',
    'onOtherHand = overAgainstOther',
    'onOtherHand = butLikewiseAsOne',
    'onOtherHand = reflectedIntoItself',
    'onOtherHand = andDeterminedInItself',
    'therefore = mattersAndThisThing',
    'twoMoments = ofSelfIdenticalExternality',
    'twoMoments = propertyReflectedIntoItself',
    'property = wasThatByWhich',
    'property = thingsSupposedToBeDistinguished',
    'thing = freedItself',
    'thing = ofNegativeSide',
    'thing = ofInheringInOther',
    'thing = becomeFree',
    'thing = fromBeingDetermined',
    'thing = byOtherThings',
    'thing = returnedIntoItself',
    'atSameTime = onlyThingInItself',
    'atSameTime = nowBecomeOther',
    'atSameTime = ofItself',
    'manifoldProperties = haveBecomeSelfSubsistent',
    'negativeConnection = inOneOfThing',
    'negativeConnection = nowOnlySublatedConnection',
    'thing = isSelfIdenticalNegation',
    'thing = onlyAsAgainst',
    'thing = positiveContinuity',
    'thing = ofMaterial',
  ],
  predicates: [
    { name: 'twoMoments', args: ['property'] },
  ],
  relations: [
    { predicate: 'is', from: 'property', to: 'twoMoments' },
  ],
  candidateSummary: 'Property is not only external determination but concrete existence immediately existing in itself. Unity of externality and essentiality repels itself from itself, contains reflection-into-itself and reflection-into-other. On one hand, determination as simple, self-identical and self-referring self-subsistent in which negative unity, one of thing, is sublated. On other hand, determination over against other, but likewise as one reflected into itself and determined in itself. Therefore, matters and this thing. Two moments of self-identical externality, property reflected into itself. Property was that by which things supposed to be distinguished. Thing freed itself of negative side of inhering in other, become free from being determined by other things, returned into itself. At same time, only thing-in-itself now become other of itself. Manifold properties have become self-subsistent, negative connection in one of thing now only sublated connection. Thing is self-identical negation only as against positive continuity of material.',
  provenance: {
    sourceChunk: 'mat-6',
    sourceOp: 'mat-op-6-movement-of-property',
  },
};

export const matOp7ThisCompleteDeterminateness: LogicalOperation = {
  id: 'mat-op-7-this-complete-determinateness',
  chunkId: 'mat-7',
  label: '"This" — complete determinateness',
  clauses: [
    'this = constitutesCompleteDeterminateness',
    'this = ofThing',
    'this = determinatenessWhichIs',
    'this = atSameTimeExternalDeterminateness',
    'thing = consistsOfSelfSubsistentMatters',
    'thing = indifferentToConnection',
    'thing = theyHaveInThing',
    'connection = onlyUnessentialLinking',
    'difference = ofOneThingFromAnother',
    'difference = dependsOnMoreOrLess',
    'difference = ofParticularMatters',
    'difference = andInWhatAmount',
    'matters = overrunThisThing',
    'matters = continueIntoOthers',
    'that = theyBelongToThisThing',
    'that = isNoRestrictionForThem',
    'justAsLittle = restriction',
    'justAsLittle = forOneAnother',
    'negativeConnection = onlyImpotent',
    'negativeConnection = this',
    'inBeingLinkedTogether = doNotSublateThemselves',
    'selfSubsistent = impenetrableToEachOther',
    'inDeterminateness = referOnlyToThemselves',
    'mutuallyIndifferent = manifoldOfSubsistence',
    'only = limitQuantitativeOne',
    'thing = asThis',
    'thing = isMerelyQuantitativeConnection',
    'thing = mereCollection',
    'thing = theirAlso',
    'thing = consistsOfSomeQuantum',
    'thing = ofMatter',
    'thing = alsoQuantumOfAnother',
    'thing = alsoYetAnother',
    'combination = ofNotHavingAnyCombination',
    'combination = aloneConstitutesThing',
  ],
  predicates: [
    { name: 'completeDeterminateness', args: ['this'] },
  ],
  relations: [
    { predicate: 'is', from: 'this', to: 'completeDeterminateness' },
  ],
  candidateSummary: '"This" constitutes complete determinateness of thing, determinateness which is at same time external determinateness. Thing consists of self-subsistent matters indifferent to connection they have in thing. Connection only unessential linking. Difference of one thing from another depends on more or less of particular matters and in what amount. Matters overrun this thing, continue into others. That they belong to this thing is no restriction for them. Just as little restriction for one another. Negative connection only impotent "this." In being linked together, do not sublate themselves. Self-subsistent, impenetrable to each other. In determinateness refer only to themselves, mutually indifferent manifold of subsistence. Only limit quantitative one. Thing as this is merely quantitative connection, mere collection, their "also." Thing consists of some quantum of matter, also quantum of another, also yet another. Combination, of not having any combination, alone constitutes thing.',
  provenance: {
    sourceChunk: 'mat-7',
    sourceOp: 'mat-op-7-this-complete-determinateness',
  },
};

export const matterOperations: LogicalOperation[] = [
  matOp1TransitionChemistry,
  matOp2NecessityOfTransition,
  matOp3FirstNegativeMoment,
  matOp4SecondThingProgressed,
  matOp5ThirdTurningBack,
  matOp6MovementOfProperty,
  matOp7ThisCompleteDeterminateness,
];

