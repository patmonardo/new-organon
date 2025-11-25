/**
 * Logical Operations: The Thing and Its Properties
 *
 * The Thing is the first determination of Appearance. It emerges from concrete existence
 * as the unity of being-in-itself and external concrete existence.
 *
 * Dialectical Movement:
 * - Thing-in-itself: essential immediate
 * - Property: negativity of reflection
 * - Reciprocal action: thinghood passes into property
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE THING AND ITS PROPERTIES
// ============================================================================

export const thgOp1ConcreteExistencePrinciple: LogicalOperation = {
  id: 'thg-op-1-concrete-existence-principle',
  chunkId: 'thg-1',
  label: 'Concrete existence — principle',
  clauses: [
    'whateverIs = existsConcretely',
    'truthOfBeing = essenceThatHasComeForward',
    'being = notImmediateSomething',
  ],
  predicates: [
    { name: 'existsConcretely', args: ['whateverIs'] },
    { name: 'essenceThatHasComeForward', args: ['truthOfBeing'] },
    { name: 'notImmediateSomething', args: ['being'] },
  ],
  relations: [
    { predicate: 'is', from: 'truthOfBeing', to: 'essenceThatHasComeForward' },
  ],
  candidateSummary: 'Just as principle of sufficient reason says whatever is has ground, is posited, mediated, so principle of concrete existence: whatever is, exists concretely. Truth of being is to be, not immediate something, but essence that has come forth into immediacy.',
  provenance: {
    sourceChunk: 'thg-1',
    sourceOp: 'thg-op-1-concrete-existence-principle',
  },
};

export const thgOp2GroundAndUnconditioned: LogicalOperation = {
  id: 'thg-op-2-ground-and-unconditioned',
  chunkId: 'thg-2',
  label: 'Concrete existence — ground and unconditioned',
  clauses: [
    'concreteExistence = hasGround',
    'concreteExistence = conditioned',
    'concreteExistence = unconditioned',
    'immediacy = fromSublatingOfMediation',
    'mediation = sublatesItselfInComingForth',
  ],
  predicates: [
    { name: 'hasGround', args: ['concreteExistence'] },
    { name: 'conditioned', args: ['concreteExistence'] },
    { name: 'unconditioned', args: ['concreteExistence'] },
    { name: 'fromSublatingOfMediation', args: ['immediacy'] },
    { name: 'sublatesItselfInComingForth', args: ['mediation'] },
  ],
  relations: [
    { predicate: 'is', from: 'concreteExistence', to: 'immediacyFromSublatingMediation' },
  ],
  candidateSummary: 'Whatever exists concretely has ground and is conditioned, also has no ground and is unconditioned. Concrete existence is immediacy that has come forth from sublating of mediation. Mediation results from connection of ground and condition. In coming forth, sublates this very coming forth.',
  provenance: {
    sourceChunk: 'thg-2',
    sourceOp: 'thg-op-2-ground-and-unconditioned',
  },
};

export const thgOp3ProofsOfGod: LogicalOperation = {
  id: 'thg-op-3-proofs-of-god',
  chunkId: 'thg-3',
  label: 'Proofs of God — three kinds of being',
  clauses: [
    'immediateBeing = first',
    'concreteExistence = second',
    'objectivity = third',
    'proof = mediatedCognition',
    'ontologicalProof = conceptToExistence',
  ],
  predicates: [
    { name: 'first', args: ['immediateBeing'] },
    { name: 'second', args: ['concreteExistence'] },
    { name: 'third', args: ['objectivity'] },
    { name: 'mediatedCognition', args: ['proof'] },
    { name: 'conceptToExistence', args: ['ontologicalProof'] },
  ],
  relations: [
    { predicate: 'proceedsFrom', from: 'objectivity', to: 'concept' },
  ],
  candidateSummary: 'Besides immediate being (first) and concrete existence (second), third being proceeds from concept: objectivity. Proof is mediated cognition. Various kinds of being require each its own kind of mediation. Ontological proof wants to start from concept, lays down sum total of all realities. Kant\'s objection: concrete existence is determinate immediate existence entering into context of total experience. In concept (simply self-referring), mediation missing. Ontological proof would demonstrate absolute concept attains determinate existence, or how simple essence mediates itself with mediation. Done by subsumption of concrete existence under universal, reality, as middle term.',
  provenance: {
    sourceChunk: 'thg-3',
    sourceOp: 'thg-op-3-proofs-of-god',
  },
};

export const thgOp4GroundVanishes: LogicalOperation = {
  id: 'thg-op-4-ground-vanishes',
  chunkId: 'thg-4',
  label: 'Proofs of God — ground vanishes',
  clauses: [
    'ground = vanishesInSubjectMatter',
    'absoluteEssence = proceedsFromGroundless',
    'ground = sublatesItself',
    'reflectiveShine = vanishes',
    'mediation = selfExternal',
  ],
  predicates: [
    { name: 'vanishesInSubjectMatter', args: ['ground'] },
    { name: 'proceedsFromGroundless', args: ['absoluteEssence'] },
    { name: 'sublatesItself', args: ['ground'] },
    { name: 'vanishes', args: ['reflectiveShine'] },
    { name: 'selfExternal', args: ['mediation'] },
  ],
  relations: [
    { predicate: 'proceedsFrom', from: 'absoluteEssence', to: 'groundless' },
  ],
  candidateSummary: 'Proofs adduce ground for existence of God, not objective ground but solely ground for cognition. Ground vanishes in subject matter. Ground derived from contingency entails regress into absolute essence. Accidental is groundless and self-sublating. Absolute essence proceeds from that which has no ground. Ground sublates itself, reflective shine vanishes. True mediation, but reflection does not know nature of mediation it performs. Takes itself as merely subjective, distances mediation from God. True relation: both in one, mediation as such but at same time subjective, external mediation, self-external mediation which internally sublates itself.',
  provenance: {
    sourceChunk: 'thg-4',
    sourceOp: 'thg-op-4-ground-vanishes',
  },
};

export const thgOp5NotMerelyImmediate: LogicalOperation = {
  id: 'thg-op-5-not-merely-immediate',
  chunkId: 'thg-5',
  label: 'Concrete existence — not merely immediate',
  clauses: [
    'founderingOfMediation = ground',
    'abyss = negativeGround',
    'abyss = positiveGround',
    'vanishedMediation = ground',
    'negation = selfEqual',
  ],
  predicates: [
    { name: 'ground', args: ['founderingOfMediation'] },
    { name: 'negativeGround', args: ['abyss'] },
    { name: 'positiveGround', args: ['abyss'] },
    { name: 'ground', args: ['vanishedMediation'] },
    { name: 'selfEqual', args: ['negation'] },
  ],
  relations: [
    { predicate: 'is', from: 'founderingOfMediation', to: 'ground' },
  ],
  candidateSummary: 'Concrete existence cannot be regarded merely as immediate. Taken as immediacy, comprehension declared beyond proof, immediate consciousness only, faith. But reflection ending with sublation does not have nothing for result. End itself, foundering of mediation, is at same time ground from which immediate proceeds. \'zu Grunde gehen\' unites foundering and ground. Essence of God is abyss (Abgrund) for finite reason. Reason surrenders finitude, sinks mediating movement. Abyss, negative ground, is positive ground of emergence of existent. Mediation is essential moment. Ground is vanished mediation. Only vanished mediation is ground and, through negation, self-equal and immediate.',
  provenance: {
    sourceChunk: 'thg-5',
    sourceOp: 'thg-op-5-not-merely-immediate',
  },
};

export const thgOp6EssencePassedOver: LogicalOperation = {
  id: 'thg-op-6-essence-passed-over',
  chunkId: 'thg-6',
  label: 'Concrete existence — essence passed over',
  clauses: [
    'essence = passedOverIntoConcreteExistence',
    'concreteExistence = absoluteSelfEmptyingOfEssence',
    'concreteExistence = reflectionOfGroundIntoItself',
    'mediation = positedItselfAsIdentical',
    'mediation = immediacy',
  ],
  predicates: [
    { name: 'passedOverIntoConcreteExistence', args: ['essence'] },
    { name: 'absoluteSelfEmptyingOfEssence', args: ['concreteExistence'] },
    { name: 'reflectionOfGroundIntoItself', args: ['concreteExistence'] },
    { name: 'positedItselfAsIdentical', args: ['mediation'] },
    { name: 'immediacy', args: ['mediation'] },
  ],
  relations: [
    { predicate: 'passedInto', from: 'essence', to: 'concreteExistence' },
  ],
  candidateSummary: 'Concrete existence not predicate or determination of essence. Not \'essence exists concretely\' but essence has passed over into concrete existence. Concrete existence is absolute self-emptying of essence, leaves nothing behind. Proposition: \'Essence is concrete existence; it is not distinct from its concrete existence.\' Essence passed over inasmuch as essence as ground no longer distinguishes itself from grounded, or ground has sublated itself. Negation is position, positive continuity with itself. Concrete existence is reflection of ground into itself, self-identity attained in negation. Mediation that posited itself as identical with itself and through that is immediacy.',
  provenance: {
    sourceChunk: 'thg-6',
    sourceOp: 'thg-op-6-essence-passed-over',
  },
};

export const thgOp7ConcreteExistenceThing: LogicalOperation = {
  id: 'thg-op-7-concrete-existence-thing',
  chunkId: 'thg-7',
  label: 'Concrete existence — thing',
  clauses: [
    'concreteExistence = selfIdenticalMediation',
    'determinations = reflectedIntoThemselves',
    'concreteExistence = negativeUnity',
    'concreteExistence = beingWithinItself',
    'concreteExistence = thing',
  ],
  predicates: [
    { name: 'selfIdenticalMediation', args: ['concreteExistence'] },
    { name: 'reflectedIntoThemselves', args: ['determinations'] },
    { name: 'negativeUnity', args: ['concreteExistence'] },
    { name: 'beingWithinItself', args: ['concreteExistence'] },
    { name: 'thing', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'determinesItselfAs', from: 'concreteExistence', to: 'thing' },
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
  label: 'The thing and its properties — introduction',
  clauses: [
    'concretelyExistentSomething = thing',
    'distinction = notTransitionButAnalysis',
    'thing = containsThingInItselfAndExternal',
  ],
  predicates: [
    { name: 'thing', args: ['concretelyExistentSomething'] },
    { name: 'notTransitionButAnalysis', args: ['distinction'] },
    { name: 'containsThingInItselfAndExternal', args: ['thing'] },
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

export const thgOp9ThingInItself: LogicalOperation = {
  id: 'thg-op-9-thing-in-itself',
  chunkId: 'thg-9',
  label: 'Thing in itself — essential and unessential',
  clauses: [
    'thingInItself = essentialImmediate',
    'mediation = equallyEssential',
    'distinction = fallsApart',
    'thingInItself = essentialConcreteExistence',
    'mediatedBeing = unessential',
  ],
  predicates: [
    { name: 'essentialImmediate', args: ['thingInItself'] },
    { name: 'equallyEssential', args: ['mediation'] },
    { name: 'fallsApart', args: ['distinction'] },
    { name: 'essentialConcreteExistence', args: ['thingInItself'] },
    { name: 'unessential', args: ['mediatedBeing'] },
  ],
  relations: [
    { predicate: 'is', from: 'thingInItself', to: 'essentialImmediate' },
  ],
  candidateSummary: 'Thing in itself is concrete existent as essential immediate that resulted from sublated mediation. Mediation equally essential. Distinction falls apart into indifferent determinations. One side, mediation, is non-reflected immediacy, being in general. Being, determined as mediation, is existence other to itself, manifold and external. Refers to sublated mediation and essential immediacy. Immediate existence as unessential, positedness. When thing differentiated from concrete existence, then possible, thing of representation or thought. Thing-in-itself and mediated being both contained in concrete existence, both concrete existences. Thing-in-itself is essential concrete existence, mediated being is unessential.',
  provenance: {
    sourceChunk: 'thg-9',
    sourceOp: 'thg-op-9-thing-in-itself',
  },
};

export const thgOp10ThingInItselfSubstrate: LogicalOperation = {
  id: 'thg-op-10-thing-in-itself-substrate',
  chunkId: 'thg-10',
  label: 'Thing in itself — substrate',
  clauses: [
    'thingInItself = unmovedIndeterminateUnity',
    'thingInItself = substrate',
    'manifold = obtainedFromExternalReflection',
    'diversity = aspectsOtherPicksOut',
  ],
  predicates: [
    { name: 'unmovedIndeterminateUnity', args: ['thingInItself'] },
    { name: 'substrate', args: ['thingInItself'] },
    { name: 'obtainedFromExternalReflection', args: ['manifold'] },
    { name: 'aspectsOtherPicksOut', args: ['diversity'] },
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
    'other = externalToItselfAndEssential',
    'moments = manifoldnessAndReference',
    'manifoldness = reflectiveShine',
    'other = referenceToThingInItself',
    'other = unsupportedReboundWithinItself',
  ],
  predicates: [
    { name: 'externalToItselfAndEssential', args: ['other'] },
    { name: 'manifoldnessAndReference', args: ['moments'] },
    { name: 'reflectiveShine', args: ['manifoldness'] },
    { name: 'referenceToThingInItself', args: ['other'] },
    { name: 'unsupportedReboundWithinItself', args: ['other'] },
  ],
  relations: [
    { predicate: 'is', from: 'manifoldness', to: 'reflectiveShine' },
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
    'essencelessReflection = collapses',
    'reflection = foundersToGround',
    'reflection = comesToBeEssentialIdentity',
    'thingInItself = identicalWithExternal',
  ],
  predicates: [
    { name: 'collapses', args: ['essencelessReflection'] },
    { name: 'foundersToGround', args: ['reflection'] },
    { name: 'comesToBeEssentialIdentity', args: ['reflection'] },
    { name: 'identicalWithExternal', args: ['thingInItself'] },
  ],
  relations: [
    { predicate: 'identicalWith', from: 'thingInItself', to: 'externalConcreteExistence' },
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
    'thingInItself = selfRepelling',
    'plurality = thingsInThemselves',
    'unessentialConcreteExistence = reciprocalRelation',
    'determinateness = fallsIntoExternalReflection',
  ],
  predicates: [
    { name: 'selfRepelling', args: ['thingInItself'] },
    { name: 'thingsInThemselves', args: ['plurality'] },
    { name: 'reciprocalRelation', args: ['unessentialConcreteExistence'] },
    { name: 'fallsIntoExternalReflection', args: ['determinateness'] },
  ],
  relations: [
    { predicate: 'relatesToItselfAs', from: 'thingInItself', to: 'other' },
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
    'thingsInThemselves = extremes',
    'externalConcreteExistence = middleTerm',
    'determinateness = essentialMediation',
    'determinatenessOfThingInItself = property',
  ],
  predicates: [
    { name: 'relatingOfThingsInThemselves', args: ['externalReflection'] },
    { name: 'extremes', args: ['thingsInThemselves'] },
    { name: 'middleTerm', args: ['externalConcreteExistence'] },
    { name: 'essentialMediation', args: ['determinateness'] },
    { name: 'property', args: ['determinatenessOfThingInItself'] },
  ],
  relations: [
    { predicate: 'is', from: 'determinatenessOfThingInItself', to: 'property' },
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
  label: 'Property — negativity of reflection',
  clauses: [
    'property = negativityOfReflection',
    'property = referenceToItselfAsToOther',
    'property = mediationWhichIsSelfIdentity',
    'property = excludedFromAlteration',
  ],
  predicates: [
    { name: 'negativityOfReflection', args: ['property'] },
    { name: 'referenceToItselfAsToOther', args: ['property'] },
    { name: 'mediationWhichIsSelfIdentity', args: ['property'] },
    { name: 'excludedFromAlteration', args: ['property'] },
  ],
  relations: [
    { predicate: 'is', from: 'property', to: 'negativityOfReflection' },
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
    'property = externalReflection',
    'property = sideOfPositedness',
    'property = maintainsItselfInTransition',
    'property = thingsOwnSubstrate',
    'thing = becomesCause',
  ],
  predicates: [
    { name: 'externalReflection', args: ['property'] },
    { name: 'sideOfPositedness', args: ['property'] },
    { name: 'maintainsItselfInTransition', args: ['property'] },
    { name: 'thingsOwnSubstrate', args: ['property'] },
    { name: 'becomesCause', args: ['thing'] },
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
    'thingInItself = presentInPropertiesAsGround',
    'thinghood = groundConnection',
    'property = groundPassedIntoExternality',
    'concreteExistence = atSameTimeInItselfness',
  ],
  predicates: [
    { name: 'presentInPropertiesAsGround', args: ['thingInItself'] },
    { name: 'groundConnection', args: ['thinghood'] },
    { name: 'groundPassedIntoExternality', args: ['property'] },
    { name: 'atSameTimeInItselfness', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'is', from: 'thinghood', to: 'groundConnection' },
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
  label: 'Reciprocal action — many things',
  clauses: [
    'thingInItself = existsInConcreto',
    'manyThings = standInReciprocalAction',
    'property = reciprocalConnectingReference',
    'thinghood = passedOverIntoProperty',
  ],
  predicates: [
    { name: 'existsInConcreto', args: ['thingInItself'] },
    { name: 'standInReciprocalAction', args: ['manyThings'] },
    { name: 'reciprocalConnectingReference', args: ['property'] },
    { name: 'passedOverIntoProperty', args: ['thinghood'] },
  ],
  relations: [
    { predicate: 'passesInto', from: 'thinghood', to: 'property' },
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
    'property = selfSubsistent',
    'things = unessential',
    'property = selfSubsistingMatter',
    'thing = consistsOfMatters',
  ],
  predicates: [
    { name: 'selfSubsistent', args: ['property'] },
    { name: 'unessential', args: ['things'] },
    { name: 'selfSubsistingMatter', args: ['property'] },
    { name: 'consistsOfMatters', args: ['thing'] },
  ],
  relations: [
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
  thgOp2GroundAndUnconditioned,
  thgOp3ProofsOfGod,
  thgOp4GroundVanishes,
  thgOp5NotMerelyImmediate,
  thgOp6EssencePassedOver,
  thgOp7ConcreteExistenceThing,
  thgOp8ThingIntroduction,
  thgOp9ThingInItself,
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
