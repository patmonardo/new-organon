/**
 * Logical Operations: The Thing and Its Properties
 *
 * The Thing is concrete existence that has come forth into immediacy.
 * It is the FormProcessor's output from Essence/Reflection - what appears
 * as scientific, what shows itself. The Thing is the foundation of Appearance.
 *
 * Dialectical Movement:
 * - Concrete Existence → Thing-in-Itself → Property → Reciprocal Action
 * - Thing-in-itself as essential, mediated being as unessential
 * - Property as negativity of reflection
 * - Thinghood passes over into property
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// THE THING AND ITS PROPERTIES
// ============================================================================

export const thgOp1ConcreteExistencePrinciple: LogicalOperation = {
  id: 'thg-op-1-concrete-existence-principle',
  chunkId: 'thg-1',
  label: 'Concrete existence — principle',
  clauses: [
    'principleOfSufficientReason = whateverIsHasGround',
    'principleOfSufficientReason = whateverIsIsPosited',
    'principleOfSufficientReason = whateverIsIsMediated',
    'principleOfConcreteExistence = whateverIsExistsConcretely',
    'truthOfBeing = notImmediateSomething',
    'truthOfBeing = essenceThatHasComeForthIntoImmediacy',
  ],
  predicates: [
    { name: 'principleOfConcreteExistence', args: ['whateverIs'] },
  ],
  relations: [
    { predicate: 'is', from: 'truthOfBeing', to: 'essenceThatHasComeForthIntoImmediacy' },
  ],
  candidateSummary: 'Just as principle of sufficient reason says whatever is has ground, is posited, mediated, so principle of concrete existence: whatever is, exists concretely. Truth of being is to be, not immediate something, but essence that has come forth into immediacy.',
  provenance: {
    sourceChunk: 'thg-1',
    sourceOp: 'thg-op-1-concrete-existence-principle',
  },
};

export const thgOp2ConcreteExistenceGround: LogicalOperation = {
  id: 'thg-op-2-concrete-existence-ground',
  chunkId: 'thg-2',
  label: 'Concrete existence — ground and unconditioned',
  clauses: [
    'whateverExistsConcretely.has = ground',
    'whateverExistsConcretely.is = conditioned',
    'whateverExistsConcretely.has = noGround',
    'whateverExistsConcretely.is = unconditioned',
    'concreteExistence = immediacy',
    'concreteExistence = comeForthFromSublatingOfMediation',
    'mediation.results = fromConnectionOfGroundAndCondition',
    'inComingForth = sublatesThisVeryComingForth',
  ],
  predicates: [
    { name: 'unconditioned', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'has', from: 'whateverExistsConcretely', to: 'ground' },
    { predicate: 'is', from: 'concreteExistence', to: 'immediacy' },
  ],
  candidateSummary: 'Whatever exists concretely has ground and is conditioned, also has no ground and is unconditioned. Concrete existence is immediacy that has come forth from sublating of mediation. Mediation results from connection of ground and condition. In coming forth, sublates this very coming forth.',
  provenance: {
    sourceChunk: 'thg-2',
    sourceOp: 'thg-op-2-concrete-existence-ground',
  },
};

export const thgOp3ProofsOfGodThreeKinds: LogicalOperation = {
  id: 'thg-op-3-proofs-of-god-three-kinds',
  chunkId: 'thg-3',
  label: 'Proofs of God — three kinds of being',
  clauses: [
    'first = immediateBeing',
    'second = concreteExistence',
    'third = objectivity',
    'third.proceeds = fromConcept',
    'proof = mediatedCognition',
    'variousKindsOfBeing.require = eachItsOwnKindOfMediation',
    'ontologicalProof.wants = toStartFromConcept',
    'ontologicalProof.laysDown = sumTotalOfAllRealities',
    'kantObjection = concreteExistenceIsDeterminateImmediateExistence',
    'kantObjection = enteringIntoContextOfTotalExperience',
    'inConcept = simplySelfReferring',
    'inConcept.mediation = missing',
    'ontologicalProof.wouldDemonstrate = absoluteConceptAttainsDeterminateExistence',
    'ontologicalProof.wouldDemonstrate = howSimpleEssenceMediatesItself',
    'doneBy = subsumptionOfConcreteExistenceUnderUniversal',
    'doneBy = realityAsMiddleTerm',
  ],
  predicates: [
    { name: 'mediatedCognition', args: ['proof'] },
  ],
  relations: [
    { predicate: 'proceeds', from: 'third', to: 'fromConcept' },
  ],
  candidateSummary: 'Besides immediate being (first) and concrete existence (second), third being proceeds from concept: objectivity. Proof is mediated cognition. Various kinds of being require each its own kind of mediation. Ontological proof wants to start from concept, lays down sum total of all realities. Kant\'s objection: concrete existence is determinate immediate existence entering into context of total experience. In concept (simply self-referring), mediation missing. Ontological proof would demonstrate absolute concept attains determinate existence, or how simple essence mediates itself with mediation. Done by subsumption of concrete existence under universal, reality, as middle term.',
  provenance: {
    sourceChunk: 'thg-3',
    sourceOp: 'thg-op-3-proofs-of-god-three-kinds',
  },
};

export const thgOp4ProofsOfGodGroundVanishes: LogicalOperation = {
  id: 'thg-op-4-proofs-of-god-ground-vanishes',
  chunkId: 'thg-4',
  label: 'Proofs of God — ground vanishes',
  clauses: [
    'proofs.adduce = groundForExistenceOfGod',
    'proofs.adduce = notObjectiveGround',
    'proofs.adduce = solelyGroundForCognition',
    'ground.vanishes = inSubjectMatter',
    'ground.derivedFromContingency = entailsRegressIntoAbsoluteEssence',
    'accidental = groundless',
    'accidental = selfSublating',
    'absoluteEssence.proceeds = fromThatWhichHasNoGround',
    'ground.sublates = itself',
    'reflectiveShine.vanishes',
    'trueMediation',
    'reflection.doesNotKnow = natureOfMediationItPerforms',
    'reflection.takesItself = asMerelySubjective',
    'reflection.distances = mediationFromGod',
    'trueRelation = bothInOne',
    'trueRelation = mediationAsSuch',
    'trueRelation = butAtSameTimeSubjective',
    'trueRelation = externalMediation',
    'trueRelation = selfExternalMediation',
    'trueRelation = whichInternallySublatesItself',
  ],
  predicates: [
    { name: 'groundVanishes', args: ['ground', 'inSubjectMatter'] },
  ],
  relations: [
    { predicate: 'vanishes', from: 'ground', to: 'inSubjectMatter' },
  ],
  candidateSummary: 'Proofs adduce ground for existence of God, not objective ground but solely ground for cognition. Ground vanishes in subject matter. Ground derived from contingency entails regress into absolute essence. Accidental is groundless and self-sublating. Absolute essence proceeds from that which has no ground. Ground sublates itself, reflective shine vanishes. True mediation, but reflection does not know nature of mediation it performs. Takes itself as merely subjective, distances mediation from God. True relation: both in one, mediation as such but at same time subjective, external mediation, self-external mediation which internally sublates itself.',
  provenance: {
    sourceChunk: 'thg-4',
    sourceOp: 'thg-op-4-proofs-of-god-ground-vanishes',
  },
};

export const thgOp5ConcreteExistenceNotMerelyImmediate: LogicalOperation = {
  id: 'thg-op-5-concrete-existence-not-merely-immediate',
  chunkId: 'thg-5',
  label: 'Concrete existence — not merely immediate',
  clauses: [
    'concreteExistence.cannotBeRegarded = merelyAsImmediate',
    'takenAsImmediacy = comprehensionDeclaredBeyondProof',
    'takenAsImmediacy = immediateConsciousnessOnly',
    'takenAsImmediacy = faith',
    'reflectionEndingWithSublation != hasNothingForResult',
    'endItself = founderingOfMediation',
    'endItself = atSameTimeGround',
    'endItself = fromWhichImmediateProceeds',
    'zuGrundeGehen = unitesFounderingAndGround',
    'essenceOfGod = abyss',
    'essenceOfGod = forFiniteReason',
    'reason.surrenders = finitude',
    'reason.sinks = mediatingMovement',
    'abyss = negativeGround',
    'abyss = positiveGround',
    'abyss = ofEmergenceOfExistent',
    'mediation = essentialMoment',
    'ground = vanishedMediation',
    'onlyVanishedMediation = ground',
    'onlyVanishedMediation = throughNegation',
    'onlyVanishedMediation = selfEqualAndImmediate',
  ],
  predicates: [
    { name: 'abyss', args: ['essenceOfGod'] },
  ],
  relations: [
    { predicate: 'is', from: 'endItself', to: 'ground' },
    { predicate: 'is', from: 'ground', to: 'vanishedMediation' },
  ],
  candidateSummary: 'Concrete existence cannot be regarded merely as immediate. Taken as immediacy, comprehension declared beyond proof, immediate consciousness only, faith. But reflection ending with sublation does not have nothing for result. End itself, foundering of mediation, is at same time ground from which immediate proceeds. "zu Grunde gehen" unites foundering and ground. Essence of God is abyss (Abgrund) for finite reason. Reason surrenders finitude, sinks mediating movement. Abyss, negative ground, is positive ground of emergence of existent. Mediation is essential moment. Ground is vanished mediation. Only vanished mediation is ground and, through negation, self-equal and immediate.',
  provenance: {
    sourceChunk: 'thg-5',
    sourceOp: 'thg-op-5-concrete-existence-not-merely-immediate',
  },
};

export const thgOp6ConcreteExistenceEssencePassedOver: LogicalOperation = {
  id: 'thg-op-6-concrete-existence-essence-passed-over',
  chunkId: 'thg-6',
  label: 'Concrete existence — essence passed over',
  clauses: [
    'concreteExistence != predicateOfEssence',
    'concreteExistence != determinationOfEssence',
    'not = essenceExistsConcretely',
    'essence.hasPassedOver = intoConcreteExistence',
    'concreteExistence = absoluteSelfEmptyingOfEssence',
    'concreteExistence.leaves = nothingBehind',
    'proposition = essenceIsConcreteExistence',
    'proposition = itIsNotDistinctFromItsConcreteExistence',
    'essence.passedOver = inasmuchAsEssenceAsGround',
    'essence.passedOver = noLongerDistinguishesItselfFromGrounded',
    'essence.passedOver = orGroundHasSublatedItself',
    'negation = position',
    'negation = positiveContinuityWithItself',
    'concreteExistence = reflectionOfGroundIntoItself',
    'concreteExistence = selfIdentityAttainedInNegation',
    'concreteExistence = mediation',
    'mediation = positedItselfAsIdenticalWithItself',
    'mediation = throughThatIsImmediacy',
  ],
  predicates: [
    { name: 'absoluteSelfEmptyingOfEssence', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'hasPassedOver', from: 'essence', to: 'intoConcreteExistence' },
  ],
  candidateSummary: 'Concrete existence not predicate or determination of essence. Not "essence exists concretely" but essence has passed over into concrete existence. Concrete existence is absolute self-emptying of essence, leaves nothing behind. Proposition: "Essence is concrete existence; it is not distinct from its concrete existence." Essence passed over inasmuch as essence as ground no longer distinguishes itself from grounded, or ground has sublated itself. Negation is position, positive continuity with itself. Concrete existence is reflection of ground into itself, self-identity attained in negation. Mediation that posited itself as identical with itself and through that is immediacy.',
  provenance: {
    sourceChunk: 'thg-6',
    sourceOp: 'thg-op-6-concrete-existence-essence-passed-over',
  },
};

export const thgOp7ConcreteExistenceThing: LogicalOperation = {
  id: 'thg-op-7-concrete-existence-thing',
  chunkId: 'thg-7',
  label: 'Concrete existence — thing',
  clauses: [
    'concreteExistence = essentiallySelfIdenticalMediation',
    'concreteExistence.has = determinationsOfMediation',
    'determinations = atSameTimeReflectedIntoThemselves',
    'determinations.have = essentialAndImmediateSubsistence',
    'asImmediacyPosited = throughSublation',
    'concreteExistence = negativeUnity',
    'concreteExistence = beingWithinItself',
    'immediatelyDeterminesItself = asConcreteExistent',
    'immediatelyDeterminesItself = asThing',
  ],
  predicates: [
    { name: 'thing', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'is', from: 'concreteExistence', to: 'negativeUnity' },
    { predicate: 'determinesItself', from: 'concreteExistence', to: 'asThing' },
  ],
  candidateSummary: 'Because concrete existence essentially self-identical mediation, has determinations of mediation in it. But determinations at same time reflected into themselves, have essential and immediate subsistence. As immediacy posited through sublation, concrete existence is negative unity and being-within-itself. Immediately determines itself as concrete existent and as thing.',
  provenance: {
    sourceChunk: 'thg-7',
    sourceOp: 'thg-op-7-concrete-existence-thing',
  },
};

export const thgOp8ThingIntroduction: LogicalOperation = {
  id: 'thg-op-8-thing-introduction',
  chunkId: 'thg-8',
  label: 'A. THE THING AND ITS PROPERTIES — introduction',
  clauses: [
    'concreteExistence = asConcreteExistent',
    'concreteExistence = positedInFormOfNegativeUnity',
    'negativeUnity = atFirstOnlyImmediateDetermination',
    'negativeUnity = onenessOfSomethingInGeneral',
    'concretelyExistentSomething != somethingThatExistsImmediately',
    'concretelyExistentSomething = essentiallyImmediacy',
    'concretelyExistentSomething = arisenThroughReflectionOfMediationIntoItself',
    'concretelyExistentSomething = thing',
    'thing.distinct = fromConcreteExistence',
    'thing = immediatelyOneAndSame',
    'concreteExistence != firstImmediacy',
    'concreteExistence.has = momentOfMediation',
    'furtherDetermination != transition',
    'furtherDetermination = analysis',
    'contains = distinctionOfThingInItself',
    'contains = distinctionOfExternalConcreteExistence',
  ],
  predicates: [
    { name: 'thing', args: ['concretelyExistentSomething'] },
  ],
  relations: [
    { predicate: 'is', from: 'concretelyExistentSomething', to: 'thing' },
  ],
  candidateSummary: 'Concrete existence as concrete existent posited in form of negative unity. Negative unity at first only immediate determination, oneness of something in general. Concretely existent something different from something that exists immediately. Former essentially immediacy arisen through reflection of mediation into itself. Concretely existent something is thing. Thing distinct from concrete existence, but immediately one and same. Because concrete existence not first immediacy but has moment of mediation, further determination not transition but analysis. Contains distinction of thing-in-itself and external concrete existence.',
  provenance: {
    sourceChunk: 'thg-8',
    sourceOp: 'thg-op-8-thing-introduction',
  },
};

export const thgOp9ThingInItselfEssential: LogicalOperation = {
  id: 'thg-op-9-thing-in-itself-essential',
  chunkId: 'thg-9',
  label: 'a. The thing in itself — essential and unessential',
  clauses: [
    'thingInItself = concreteExistentAsEssentialImmediate',
    'thingInItself = resultedFromSublatedMediation',
    'mediation = equallyEssential',
    'distinction.fallsApart = intoIndifferentDeterminations',
    'oneSide = mediation',
    'mediation = nonReflectedImmediacy',
    'mediation = beingInGeneral',
    'being.determinedAsMediation = existenceOtherToItself',
    'being.determinedAsMediation = manifoldAndExternal',
    'being.refers = toSublatedMediation',
    'being.refers = toEssentialImmediacy',
    'immediateExistence = asUnessential',
    'immediateExistence = positedness',
    'whenThingDifferentiated = fromConcreteExistence',
    'thenPossible = thingOfRepresentationOrThought',
    'thingInItself = andMediatedBeing',
    'both = containedInConcreteExistence',
    'both = concreteExistences',
    'thingInItself = essentialConcreteExistence',
    'mediatedBeing = unessential',
  ],
  predicates: [
    { name: 'thingInItself', args: ['concreteExistent'] },
  ],
  relations: [
    { predicate: 'is', from: 'thingInItself', to: 'essentialConcreteExistence' },
  ],
  candidateSummary: 'Thing in itself is concrete existent as essential immediate that resulted from sublated mediation. Mediation equally essential. Distinction falls apart into indifferent determinations. One side, mediation, is non-reflected immediacy, being in general. Being, determined as mediation, is existence other to itself, manifold and external. Refers to sublated mediation and essential immediacy. Immediate existence as unessential, positedness. When thing differentiated from concrete existence, then possible, thing of representation or thought. Thing-in-itself and mediated being both contained in concrete existence, both concrete existences. Thing-in-itself is essential concrete existence, mediated being is unessential.',
  provenance: {
    sourceChunk: 'thg-9',
    sourceOp: 'thg-op-9-thing-in-itself-essential',
  },
};

export const thgOp10ThingInItselfSubstrate: LogicalOperation = {
  id: 'thg-op-10-thing-in-itself-substrate',
  chunkId: 'thg-10',
  label: 'Thing in itself — substrate',
  clauses: [
    'thingInItself = asSimpleReflectedness',
    'thingInItself = ofConcreteExistenceWithinItself',
    'thingInItself != groundOfUnessentialExistence',
    'thingInItself = unmoved',
    'thingInItself = indeterminateUnity',
    'thingInItself.has = determinationOfBeingSublatedMediation',
    'thingInItself = thereforeSubstrate',
    'reflection = asImmediateExistence',
    'reflection = mediatedThroughOther',
    'reflection.falls = outsideThingInItself',
    'thingInItself.notSupposed = toHaveDeterminateManifoldInIt',
    'thingInItself.obtains = onlyWhenExposedToExternalReflection',
    'thingInItself.remains = indifferent',
    'hasColor = onlyWhenExposedToEye',
    'hasSmell = onlyWhenExposedToNose',
    'diversity.consists = ofAspectsWhichOtherPicksOut',
    'diversity.consists = ofSpecificPointsOfReference',
    'diversity.consists = whichOtherAssumes',
    'diversity != thingOwnDeterminations',
  ],
  predicates: [
    { name: 'substrate', args: ['thingInItself'] },
  ],
  relations: [
    { predicate: 'is', from: 'thingInItself', to: 'substrate' },
  ],
  candidateSummary: 'Thing in itself, as simple reflectedness of concrete existence within itself, not ground of unessential existence. Unmoved, indeterminate unity. Has determination of being sublated mediation, therefore substrate. Reflection, as immediate existence mediated through other, falls outside thing-in-itself. Not supposed to have determinate manifold in it. Obtains it only when exposed to external reflection, remains indifferent. Has color only when exposed to eye, smell when exposed to nose. Diversity consists of aspects which other picks out, specific points of reference which other assumes. Not thing\'s own determinations.',
  provenance: {
    sourceChunk: 'thg-10',
    sourceOp: 'thg-op-10-thing-in-itself-substrate',
  },
};

export const thgOp11ExternalReflectionTwoMoments: LogicalOperation = {
  id: 'thg-op-11-external-reflection-two-moments',
  chunkId: 'thg-11',
  label: 'External reflection — two moments',
  clauses: [
    'other = reflectionDeterminedAsExternal',
    'first = externalToItself',
    'first = determinateManifoldness',
    'second = externalToEssentialConcreteExistent',
    'second.refers = toItAsAbsolutePresupposition',
    'twoMoments = ownManifoldness',
    'twoMoments = referenceToThingInItselfAsOther',
    'twoMoments = oneAndSame',
    'concreteExistence.external = onlyInSoFarAsRefersToEssentialIdentityAsOther',
    'manifoldness.doesNotHave = independentSubsistence',
    'manifoldness = besidesThingInItself',
    'manifoldness = onlyAsReflectiveShine',
    'overAgainstIt = onlyAsReflectiveShine',
    'inNecessaryReference = likeReflexRefractingItselfInIt',
    'diversity.present = asReferenceOfOtherToThingInItself',
    'other = nothingThatSubsistsOnOwn',
    'other = onlyAsReference',
    'other.onlyIs = inBeingRepelledFromIt',
    'other = unsupportedReboundOfItselfWithinItself',
  ],
  predicates: [
    { name: 'externalReflection', args: ['other'] },
  ],
  relations: [
    { predicate: 'is', from: 'other', to: 'externalReflection' },
  ],
  candidateSummary: 'Other is reflection determined as external. First, external to itself and determinate manifoldness. Second, external to essential concrete existent, refers to it as absolute presupposition. Two moments: own manifoldness and reference to thing-in-itself as other, one and same. Concrete existence external only in so far as refers to essential identity as other. Manifoldness does not have independent subsistence besides thing-in-itself. Over against it, only as reflective shine. In necessary reference, like reflex refracting itself in it. Diversity present as reference of other to thing-in-itself. Other nothing that subsists on own, only as reference, only is in being repelled from it. Unsupported rebound of itself within itself.',
  provenance: {
    sourceChunk: 'thg-11',
    sourceOp: 'thg-op-11-external-reflection-two-moments',
  },
};

export const thgOp12ExternalReflectionCollapses: LogicalOperation = {
  id: 'thg-op-12-external-reflection-collapses',
  chunkId: 'thg-12',
  label: 'External reflection — collapses',
  clauses: [
    'sinceThingInItself = essentialIdentityOfConcreteExistence',
    'essencelessReflection.doesNotAccrue = toIt',
    'essencelessReflection.collapses = withinItself',
    'essencelessReflection = externallyToIt',
    'essencelessReflection.founders = toGround',
    'essencelessReflection.itself = comesToBeEssentialIdentity',
    'essencelessReflection.itself = comesToBeThingInItself',
    'essencelessConcreteExistence.has = inThingInItself',
    'essencelessConcreteExistence.has = itsReflectionIntoItself',
    'essencelessConcreteExistence.refers = toItAsOther',
    'essencelessConcreteExistence = butAsOtherOverAgainstInItself',
    'essencelessConcreteExistence = onlySublationOfSelf',
    'essencelessConcreteExistence = comingToBeInInItself',
    'thingInItself = identicalWithExternalConcreteExistence',
  ],
  predicates: [
    { name: 'collapses', args: ['essencelessReflection'] },
  ],
  relations: [
    { predicate: 'collapses', from: 'essencelessReflection', to: 'withinItself' },
    { predicate: 'is', from: 'thingInItself', to: 'identicalWithExternalConcreteExistence' },
  ],
  candidateSummary: 'Since thing-in-itself is essential identity of concrete existence, essenceless reflection does not accrue to it but collapses within itself externally to it. Founders to ground, itself comes to be essential identity or thing-in-itself. Essenceless concrete existence has in thing-in-itself its reflection into itself. Refers to it as other, but as other over against in-itself, only sublation of self, coming to be in in-itself. Thing-in-itself identical with external concrete existence.',
  provenance: {
    sourceChunk: 'thg-12',
    sourceOp: 'thg-op-12-external-reflection-collapses',
  },
};

export const thgOp13ThingInItselfPlurality: LogicalOperation = {
  id: 'thg-op-13-thing-in-itself-plurality',
  chunkId: 'thg-13',
  label: 'Thing-in-itself — plurality',
  clauses: [
    'thingInItself = selfReferringEssentialConcreteExistence',
    'selfIdentity = onlyInSoFarAsHoldsNegativityReflectionInItself',
    'thatWhichAppeared = asConcreteExistenceExternal',
    'thatWhichAppeared = momentInIt',
    'selfRepellingThingInItself = relatesItselfToItselfAsOther',
    'plurality = ofThingsInThemselves',
    'plurality = standingInReciprocalReferenceOfExternalReflection',
    'unessentialConcreteExistence = theirReciprocalRelationAsOthers',
    'unessentialConcreteExistence = alsoEssentialToThem',
    'unessentialConcreteExistence = collapsingInternally',
    'unessentialConcreteExistence = thingInItself',
    'unessentialConcreteExistence = butOtherThanFirst',
    'first = immediateEssentiality',
    'present = proceedsFromUnessential',
    'otherThingInItself = onlyOtherInGeneral',
    'otherThingInItself = noFurtherDeterminateness',
    'otherThingInItself = visAVisFirst',
    'determinateness.falls = intoExternalReflection',
  ],
  predicates: [
    { name: 'plurality', args: ['thingsInThemselves'] },
  ],
  relations: [
    { predicate: 'is', from: 'selfRepellingThingInItself', to: 'relatesItselfToItselfAsOther' },
  ],
  candidateSummary: 'Thing-in-itself is self-referring essential concrete existence. Self-identity only in so far as holds negativity\'s reflection in itself. That which appeared as concrete existence external is moment in it. Self-repelling thing-in-itself which relates itself to itself as other. Plurality of things-in-themselves standing in reciprocal reference of external reflection. Unessential concrete existence is their reciprocal relation as others, also essential to them. Unessential concrete existence, collapsing internally, is thing-in-itself, but other than first. First is immediate essentiality, present proceeds from unessential. Other thing-in-itself only other in general, no further determinateness vis-à-vis first. Determinateness falls into external reflection.',
  provenance: {
    sourceChunk: 'thg-13',
    sourceOp: 'thg-op-13-thing-in-itself-plurality',
  },
};

export const thgOp14ExternalReflectionSyllogism: LogicalOperation = {
  id: 'thg-op-14-external-reflection-syllogism',
  chunkId: 'thg-14',
  label: 'External reflection — syllogism',
  clauses: [
    'externalReflection = relatingOfThingsInThemselves',
    'externalReflection = toOneAnother',
    'externalReflection = reciprocalMediation',
    'thingsInThemselves = extremeTermsOfSyllogism',
    'middleTerm = madeUpByExternalConcreteExistence',
    'difference.falls = onlyInConnectingReference',
    'sendDeterminations = fromSurfaceIntoReference',
    'sendDeterminations.remaining = indifferent',
    'thingInItself.drawn = intoReflectionExternalToIt',
    'thingInItself.has = manifold',
    'repellingOfItself = intoAnotherThingInItself',
    'repellingOfItself = reboundingBackIntoItself',
    'each = otherOnlyAsReflectedBackFromOther',
    'each.has = suppositionNotInItself',
    'each.has = suppositionInOther',
    'twoThingsInThemselves = sinceEachHasDifferenceNotInItButInOther',
    'twoThingsInThemselves = notDistinctThings',
    'thingInItself.relates = toOtherAsToSomethingNonDistinguished',
    'externalReflection = relationOnlyToItself',
    'externalReflection = essentiallyItsReflectionWithinItself',
    'determinateness = existingInItself',
    'determinateness = determinatenessOfThingInItself',
    'determinateness.not = inReferenceExternalToIt',
    'determinateness = essentialMediationOfItselfWithItself',
    'determinateness = asWithOther',
    'two.collapse = intoOne',
    'onlyOneThingInItself = relatesItselfToItselfInExternalReflection',
    'ownReferenceToItself = asToAnother',
    'ownReferenceToItself = constitutesDeterminateness',
    'determinatenessOfThingInItself = propertyOfThing',
  ],
  predicates: [
    { name: 'syllogism', args: ['externalReflection'] },
  ],
  relations: [
    { predicate: 'is', from: 'externalReflection', to: 'syllogism' },
    { predicate: 'is', from: 'determinatenessOfThingInItself', to: 'propertyOfThing' },
  ],
  candidateSummary: 'External reflection is relating of things-in-themselves to one another, reciprocal mediation. Things-in-themselves are extreme terms of syllogism, middle term made up by external concrete existence. Difference falls only in connecting reference. Send determinations from surface into reference, remaining indifferent. Thing-in-itself drawn into reflection external to it, has manifold. Repelling of itself into another thing-in-itself, rebounding back into itself. Each is other only as reflected back from other. Has supposition not in itself but in other. Two things-in-themselves, since each has difference not in it but in other, not distinct things. Thing-in-itself relates to other as to something non-distinguished. External reflection is relation only to itself, essentially its reflection within itself. Determinateness existing in itself, determinateness of thing-in-itself. Not in reference external to it but essential mediation of itself with itself as with other. Two collapse into one. Only one thing-in-itself relates itself to itself in external reflection. Own reference to itself as to another constitutes determinateness. Determinateness of thing-in-itself is property of thing.',
  provenance: {
    sourceChunk: 'thg-14',
    sourceOp: 'thg-op-14-external-reflection-syllogism',
  },
};

export const thgOp15PropertyNegativity: LogicalOperation = {
  id: 'thg-op-15-property-negativity',
  chunkId: 'thg-15',
  label: 'b. Property — negativity of reflection',
  clauses: [
    'quality = immediateDeterminatenessOfSomething',
    'quality = negativeByVirtueOfWhichBeingIsSomething',
    'propertyOfThing = negativityOfReflection',
    'propertyOfThing = byVirtueOfWhichConcreteExistenceIsConcreteExistent',
    'propertyOfThing = asSimpleSelfIdentityIsThingInItself',
    'negativityOfReflection = sublatedMediation',
    'negativityOfReflection = itselfEssentiallyMediation',
    'negativityOfReflection = reference',
    'negativityOfReflection != toOtherInGeneral',
    'negativityOfReflection != likeQuality',
    'negativityOfReflection != notReflectedDeterminateness',
    'negativityOfReflection = referenceToItselfAsToOther',
    'negativityOfReflection = mediation',
    'mediation = whichImmediatelyIsNoLessSelfIdentity',
    'abstractThingInItself = relation',
    'abstractThingInItself = whichTurnsFromAnotherBackToItself',
    'abstractThingInItself = determinedInItself',
    'determinateness = constitution',
    'constitution = itselfDetermination',
    'inRelatingToOther = doesNotPassOverIntoOtherness',
    'inRelatingToOther = excludedFromAlteration',
  ],
  predicates: [
    { name: 'negativityOfReflection', args: ['propertyOfThing'] },
  ],
  relations: [
    { predicate: 'is', from: 'propertyOfThing', to: 'negativityOfReflection' },
  ],
  candidateSummary: 'Quality is immediate determinateness of something, negative by virtue of which being is something. Property of thing is negativity of reflection, by virtue of which concrete existence is concrete existent, as simple self-identity is thing-in-itself. Negativity of reflection, sublated mediation, itself essentially mediation and reference. Not to other in general like quality (not reflected determinateness). Reference to itself as to other, mediation which immediately is no less self-identity. Abstract thing-in-itself is relation which turns from another back to itself. Determined in itself. Determinateness is constitution, itself determination. In relating to other does not pass over into otherness, excluded from alteration.',
  provenance: {
    sourceChunk: 'thg-15',
    sourceOp: 'thg-op-15-property-negativity',
  },
};

export const thgOp16ThingHasProperties: LogicalOperation = {
  id: 'thg-op-16-thing-has-properties',
  chunkId: 'thg-16',
  label: 'Thing has properties — two sides',
  clauses: [
    'thing.has = properties',
    'first = determinateReferencesToSomethingOther',
    'property.there = onlyAsWayOfReciprocalRelating',
    'property = externalReflectionOfThing',
    'property = sideOfPositedness',
    'second = inPositednessThingIsInItself',
    'second = maintainsItselfInReferenceToOther',
    'admittedly = onlySurface',
    'admittedly = whereConcreteExistenceExposedToBecomingAndAlteration',
    'property.notLost',
    'thing.hasProperty = toEffectThisOrThatInOther',
    'thing.hasProperty = toExpressItself',
    'thing.demonstratesProperty = onlyUnderCondition',
    'thing.demonstratesProperty = anotherThingHasCorrespondingConstitution',
    'property = characteristicallyThingOwn',
    'property = selfIdenticalSubstrate',
    'reflectedQuality = calledProperty',
    'thing.passesOver = intoExternality',
    'property.maintains = itself',
    'throughProperties = thingBecomesCause',
    'toBeCause = toPreserveItselfAsEffect',
    'thing.still = staticThingOfManyProperties',
    'thing.still != yetActualCause',
    'thing.still = onlyReflectionOfDeterminations',
    'thing.still = immediatelyExistingInItself',
    'thing.still != yetReflectionThatPositsThem',
  ],
  predicates: [
    { name: 'hasProperties', args: ['thing'] },
  ],
  relations: [
    { predicate: 'has', from: 'thing', to: 'properties' },
  ],
  candidateSummary: 'Thing has properties. First, determinate references to something other. Property there only as way of reciprocal relating. External reflection of thing, side of positedness. Second, in positedness thing is in itself. Maintains itself in reference to other. Admittedly only surface where concrete existence exposed to becoming and alteration. Property not lost. Thing has property to effect this or that in other, express itself. Demonstrates property only under condition another thing has corresponding constitution. Property characteristically thing\'s own and self-identical substrate. Reflected quality called property. Thing passes over into externality, property maintains itself. Through properties thing becomes cause. To be cause is to preserve itself as effect. Thing still static thing of many properties, not yet actual cause. Only reflection of determinations immediately existing in itself, not yet reflection that posits them.',
  provenance: {
    sourceChunk: 'thg-16',
    sourceOp: 'thg-op-16-thing-has-properties',
  },
};

export const thgOp17ThingInItselfGround: LogicalOperation = {
  id: 'thg-op-17-thing-in-itself-ground',
  chunkId: 'thg-17',
  label: 'Thing-in-itself — ground',
  clauses: [
    'thingInItself.hasShown = suchThatPropertiesAreOwnDeterminations',
    'thingInItself = byVirtueOfWhichRelatesInDeterminateManner',
    'thingInItself != indeterminateSubstrate',
    'thingInItself = presentInPropertiesAsGround',
    'selfIdentity = inPositedness',
    'selfIdentity = butAtSameTimeConditionedGround',
    'positedness = equallyReflectionExternalToItself',
    'reflectedIntoItself = onlyToExtentExternal',
    'throughConcreteExistence = entersIntoExternalReferences',
    'concreteExistence.consists = inExternality',
    'concreteExistence.consists = immediacyOfBeing',
    'concreteExistence = thingSubjectedToAlteration',
    'concreteExistence = alsoReflectedImmediacyOfGround',
    'concreteExistence = thingInItselfInAlteration',
    'thinghoodItself = groundConnection',
    'property = groundThatPassedOverIntoExternality',
    'property = trulyReflectedIntoItself',
    'property = ground',
    'property = implicitlyExistentPositedness',
    'ground.constitutes = formOfPropertyIdentity',
    'propertyDeterminateness = selfExternalReflectionOfGround',
    'whole = ground',
    'whole = whichInRepellingAndDetermining',
    'whole = inExternalImmediacy',
    'whole = refersItselfToItself',
    'thingInItself = concretelyExistsEssentially',
    'concreteExistence = asExternalImmediacy',
    'concreteExistence = atSameTimeInItselfness',
  ],
  predicates: [
    { name: 'ground', args: ['thingInItself'] },
  ],
  relations: [
    { predicate: 'is', from: 'thingInItself', to: 'presentInPropertiesAsGround' },
    { predicate: 'is', from: 'property', to: 'ground' },
  ],
  candidateSummary: 'Thing-in-itself has shown itself such that properties are own determinations by virtue of which relates in determinate manner. Not indeterminate substrate but present in properties as ground. Self-identity in positedness, but at same time conditioned ground. Positedness equally reflection external to itself. Reflected into itself only to extent external. Through concrete existence enters into external references. Concrete existence consists in externality, immediacy of being, thing subjected to alteration. Also reflected immediacy of ground, thing in itself in alteration. Thinghood itself is ground-connection. Property is ground that passed over into externality, truly reflected into itself. Property is ground, implicitly existent positedness. Ground constitutes form of property\'s identity. Property\'s determinateness is self-external reflection of ground. Whole is ground which in repelling and determining, in external immediacy, refers itself to itself. Thing-in-itself concretely exists essentially. Concrete existence, as external immediacy, is at same time in-itselfness.',
  provenance: {
    sourceChunk: 'thg-17',
    sourceOp: 'thg-op-17-thing-in-itself-ground',
  },
};

export const thgOp18ReciprocalAction: LogicalOperation = {
  id: 'thg-op-18-reciprocal-action',
  chunkId: 'thg-18',
  label: 'c. Reciprocal action — many things',
  clauses: [
    'thingInItself.exists = inConcretoByEssence',
    'externalImmediacy = belongsToBeingInItself',
    'externalImmediacy = immanentReflection',
    'thingInItself = thingThatHasProperties',
    'numberOfThings = distinctFromOneAnother',
    'numberOfThings = throughThemselves',
    'manyDiverseThings = standInEssentialReciprocalAction',
    'manyDiverseThings = byVirtueOfProperties',
    'property = reciprocalConnectingReferenceItself',
    'property = apartFromWhichThingIsNothing',
    'reciprocalDetermination = middleTerm',
    'reciprocalDetermination = itselfSelfIdenticalReflection',
    'reciprocalDetermination = thingInItself',
    'thinghood.reduced = toFormOfIndeterminateSelfIdentity',
    'essentiality = onlyInProperty',
    'thingWithoutDeterminateProperty = differenceMerelyIndifferent',
    'thingWithoutDeterminateProperty = quantitative',
    'determinateness = inVirtueOfWhichThingIsThisThing',
    'determinateness = onlyLiesSolelyInProperties',
    'throughThem = thingDifferentiatesItself',
    'property = negativeReflection',
    'property = differentiating',
    'onlyInProperty = thingPossessesDifferenceOfItselfFromOthers',
    'difference = reflectedIntoItself',
    'withoutProperties = nothingRemainsExceptUnessentialCompass',
    'withoutProperties = externalGatheringOfAbstractInItselfness',
    'thinghood.passedOver = intoProperty',
  ],
  predicates: [
    { name: 'reciprocalAction', args: ['manyDiverseThings'] },
  ],
  relations: [
    { predicate: 'standIn', from: 'manyDiverseThings', to: 'essentialReciprocalAction' },
    { predicate: 'passedOver', from: 'thinghood', to: 'intoProperty' },
  ],
  candidateSummary: 'Thing-in-itself exists in concreto by essence. External immediacy and determinateness belong to being-in-itself, immanent reflection. Thing-in-itself is thing that has properties. Number of things distinct from one another, through themselves. Many diverse things stand in essential reciprocal action by virtue of properties. Property is reciprocal connecting reference itself, apart from which thing is nothing. Reciprocal determination, middle term, is itself self-identical reflection and thing-in-itself. Thinghood reduced to form of indeterminate self-identity, essentiality only in property. Thing without determinate property: difference merely indifferent, quantitative. Determinateness in virtue of which thing is this thing only lies solely in properties. Through them thing differentiates itself. Property is negative reflection and differentiating. Only in property does thing possess difference of itself from others. Difference reflected into itself. Without properties, nothing remains except unessential compass, external gathering of abstract in-itselfness. Thinghood passed over into property.',
  provenance: {
    sourceChunk: 'thg-18',
    sourceOp: 'thg-op-18-reciprocal-action',
  },
};

export const thgOp19PropertySelfSubsistent: LogicalOperation = {
  id: 'thg-op-19-property-self-subsistent',
  chunkId: 'thg-19',
  label: 'Property — self-subsistent',
  clauses: [
    'thing = asExtremeTerm',
    'thing.supposed = toRelateToProperty',
    'property = toConstituteMiddleTermBetweenThings',
    'connection = whereThingsMeetAsSelfRepellingReflection',
    'connection = distinguishedAndConnected',
    'distinction = andConnectingReference',
    'distinction = oneReflectionAndContinuity',
    'things.fall = onlyWithinContinuity',
    'continuity = whichIsProperty',
    'things.vanish = asWouldBeSelfSubsistingExtremes',
    'property = supposedToConnectExtremes',
    'property = itselfSelfSubsistent',
    'things = unessential',
    'essential = onlyAsSelfDifferentiatingAndSelfReferringReflection',
    'essential = butThisIsProperty',
    'truthOfThing = onlyUnessentialCompass',
    'truthOfThing = negativeUnity',
    'truthOfThing = likeOneOfSomething',
    'truthOfThing = immediate',
    'earlierAbstraction = envisagedAbstractThingAsEssential',
    'earlierAbstraction = propertyAsExternal',
    'now = thingReducedThroughItself',
    'now = toIndifferentExternalFormOfProperty',
    'property.freed = ofIndeterminateBond',
    'property.constitutes = subsistenceOfThing',
    'property = selfSubsistingMatter',
    'matter = simpleContinuityWithItself',
    'matter = onlyFormOfDiversity',
    'manifold = ofSelfSubsistingMatters',
    'thing.consists = ofThem',
  ],
  predicates: [
    { name: 'selfSubsistent', args: ['property'] },
    { name: 'selfSubsistingMatter', args: ['property'] },
  ],
  relations: [
    { predicate: 'is', from: 'property', to: 'selfSubsistent' },
    { predicate: 'is', from: 'property', to: 'selfSubsistingMatter' },
  ],
  candidateSummary: 'Thing, as extreme term, supposed to relate to property. Property to constitute middle term between things. Connection is where things meet as self-repelling reflection, distinguished and connected. Distinction and connecting reference one reflection and continuity. Things fall only within continuity which is property. Vanish as would-be self-subsisting extremes. Property, supposed to connect extremes, is itself self-subsistent. Things are unessential. Essential only as self-differentiating and self-referring reflection, but this is property. Truth of thing is only unessential compass, negative unity like one of something, immediate. Earlier abstraction envisaged abstract thing as essential, property as external. Now thing reduced through itself to indifferent external form of property. Property freed of indeterminate bond, constitutes subsistence of thing. Self-subsisting matter. Matter is simple continuity with itself, only form of diversity. Manifold of self-subsisting matters. Thing consists of them.',
  provenance: {
    sourceChunk: 'thg-19',
    sourceOp: 'thg-op-19-property-self-subsistent',
  },
};

export const thingOperations: LogicalOperation[] = [
  thgOp1ConcreteExistencePrinciple,
  thgOp2ConcreteExistenceGround,
  thgOp3ProofsOfGodThreeKinds,
  thgOp4ProofsOfGodGroundVanishes,
  thgOp5ConcreteExistenceNotMerelyImmediate,
  thgOp6ConcreteExistenceEssencePassedOver,
  thgOp7ConcreteExistenceThing,
  thgOp8ThingIntroduction,
  thgOp9ThingInItselfEssential,
  thgOp10ThingInItselfSubstrate,
  thgOp11ExternalReflectionTwoMoments,
  thgOp12ExternalReflectionCollapses,
  thgOp13ThingInItselfPlurality,
  thgOp14ExternalReflectionSyllogism,
  thgOp15PropertyNegativity,
  thgOp16ThingHasProperties,
  thgOp17ThingInItselfGround,
  thgOp18ReciprocalAction,
  thgOp19PropertySelfSubsistent,
];

