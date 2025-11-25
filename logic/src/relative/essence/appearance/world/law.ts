/**
 * Logical Operations: The Law of Appearance
 *
 * The Law of Appearance is the positive element of mediation, the identity remaining
 * self-equal in the flux of appearance. It is the essential appearance, the unity
 * of reflective shine and concrete existence.
 *
 * Dialectical Movement:
 * - Appearance: essence in concrete existence
 * - Law: positive identity
 * - Kingdom of laws: restful copy
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE LAW OF APPEARANCE
// ============================================================================

export const lawOp1EssenceInConcreteExistence: LogicalOperation = {
  id: 'law-op-1-essence-in-concrete-existence',
  chunkId: 'law-1',
  label: 'Appearance — essence in concrete existence',
  clauses: [
    'concreteExistence = reflectedImmediacy',
    'essence = steppedOutOfGround',
    'concreteExistence = absoluteNegativityWithin',
    'appearance = essenceInConcreteExistence',
  ],
  predicates: [
    { name: 'reflectedImmediacy', args: ['concreteExistence'] },
    { name: 'steppedOutOfGround', args: ['essence'] },
    { name: 'absoluteNegativityWithin', args: ['concreteExistence'] },
    { name: 'essenceInConcreteExistence', args: ['appearance'] },
  ],
  relations: [
    { predicate: 'is', from: 'appearance', to: 'essenceInConcreteExistence' },
  ],
  candidateSummary: 'Concrete existence is immediacy of being to which essence has restored itself. In itself immediacy is reflection of essence into itself. As concrete existence, essence has stepped out of ground which passed over into it. Concrete existence is reflected immediacy in so far as within it is absolute negativity. Posited as such, determined itself as appearance. Appearance is essence in its concrete existence. Essence immediately present in it. Not immediate but reflected concrete existence constitutes moment of essence. Concrete existence, as essential concrete existence, is appearance.',
  provenance: {
    sourceChunk: 'law-1',
    sourceOp: 'law-op-1-essence-in-concrete-existence',
  },
};

export const lawOp2OnlyAppearance: LogicalOperation = {
  id: 'law-op-2-only-appearance',
  chunkId: 'law-2',
  label: 'Appearance — only appearance',
  clauses: [
    'something = onlyAppearance',
    'concreteExistence = onlyPositedBeing',
    'essentiality = ownTruth',
    'reflection = itsOwn',
  ],
  predicates: [
    { name: 'onlyAppearance', args: ['something'] },
    { name: 'onlyPositedBeing', args: ['concreteExistence'] },
    { name: 'ownTruth', args: ['essentiality'] },
    { name: 'itsOwn', args: ['reflection'] },
  ],
  relations: [
    { predicate: 'is', from: 'essentiality', to: 'ownTruth' },
  ],
  candidateSummary: 'Something is only appearance in sense concrete existence is only posited being, not in-and-for-itself. This constitutes essentiality: have negativity of reflection, nature of essence, within it. No alien, external reflection. Essentiality of concrete existence, that it is appearance, is concrete existence\'s own truth. Reflection by virtue of which it is this is its own.',
  provenance: {
    sourceChunk: 'law-2',
    sourceOp: 'law-op-2-only-appearance',
  },
};

export const lawOp3HigherTruth: LogicalOperation = {
  id: 'law-op-3-higher-truth',
  chunkId: 'law-3',
  label: 'Appearance — higher truth',
  clauses: [
    'appearance = higherTruth',
    'concreteExistence = essential',
    'concreteExistence = ceasesToBeEssenceless',
  ],
  predicates: [
    { name: 'higherTruth', args: ['appearance'] },
    { name: 'essential', args: ['concreteExistence'] },
    { name: 'ceasesToBeEssenceless', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'is', from: 'appearance', to: 'higherTruth' },
  ],
  candidateSummary: 'If said something is only appearance, meaning contrasted with it immediate concrete existence is truth, then appearance is higher truth. It is concrete existence as essential, whereas concrete existence is appearance still void of essence. Only contains one moment: concrete existence as immediate, not yet negative reflection. Quality is transient; determination of reflection is essential. When appearance said to be essenceless, thinks immediate were positive and true. But immediate does not contain essential truth. Concrete existence ceases to be essenceless by passing over into appearance.',
  provenance: {
    sourceChunk: 'law-3',
    sourceOp: 'law-op-3-higher-truth',
  },
};

export const lawOp4RealShine: LogicalOperation = {
  id: 'law-op-4-real-shine',
  chunkId: 'law-4',
  label: 'Essence appears — real shine',
  clauses: [
    'essence = appears',
    'shine = real',
    'appearance = negativeMediation',
    'appearance = unityOfReflectiveShineAndExistence',
  ],
  predicates: [
    { name: 'appears', args: ['essence'] },
    { name: 'real', args: ['shine'] },
    { name: 'negativeMediation', args: ['appearance'] },
    { name: 'unityOfReflectiveShineAndExistence', args: ['appearance'] },
  ],
  relations: [
    { predicate: 'is', from: 'appearance', to: 'unityOfReflectiveShineAndExistence' },
  ],
  candidateSummary: 'Essence reflectively shines at first just within, in simple identity. Abstract reflection, pure movement of nothing through nothing back to itself. Essence appears, now real shine, since moments of shine have concrete existence. Appearance is thing as negative mediation of itself with itself. Differences are self-subsisting matters. Contradiction: immediate subsistence, yet obtaining subsistence only in alien self-subsistence. In negation of own, also in negation of alien self-subsistence. Reflective shine is same mediation. Fleeting moments obtain shape of immediate self-subsistence. Immediate self-subsistence reduced to moment. Appearance is unity of reflective shine and concrete existence.',
  provenance: {
    sourceChunk: 'law-4',
    sourceOp: 'law-op-4-real-shine',
  },
};

export const lawOp5ThreeMoments: LogicalOperation = {
  id: 'law-op-5-three-moments',
  chunkId: 'law-5',
  label: 'Appearance determines itself — three moments',
  clauses: [
    'first = lawOfAppearance',
    'second = worldInItself',
    'third = appearanceBecomesRelation',
  ],
  predicates: [
    { name: 'lawOfAppearance', args: ['first'] },
    { name: 'worldInItself', args: ['second'] },
    { name: 'appearanceBecomesRelation', args: ['third'] },
  ],
  relations: [
    { predicate: 'has', from: 'appearance', to: 'threeMoments' },
  ],
  candidateSummary: 'Appearance determines itself further. Concrete existence as essential differs from unessential. Two sides refer to each other. First, simple self-identity containing diverse content determinations. Remains self-equal in flux of appearance. This is law of appearance. Second, law simple in diversity passes over into opposition. Essential moment opposed to appearance itself. Confronting world of appearance, world that exists in itself comes onto scene. Third, opposition returns into ground. That in itself is in appearance. That appears determined as taken up into being-in-itself. Appearance becomes relation.',
  provenance: {
    sourceChunk: 'law-5',
    sourceOp: 'law-op-5-three-moments',
  },
};

export const lawOp6AppearanceMediated: LogicalOperation = {
  id: 'law-op-6-appearance-mediated',
  chunkId: 'law-6',
  label: 'The law of appearance — appearance mediated',
  clauses: [
    'appearance = mediatedThroughNegation',
    'selfSubsistence = returnOfNothingThroughNothing',
    'linkage = reciprocalNegation',
    'subsistence = connectionOfPositedness',
  ],
  predicates: [
    { name: 'mediatedThroughNegation', args: ['appearance'] },
    { name: 'returnOfNothingThroughNothing', args: ['selfSubsistence'] },
    { name: 'reciprocalNegation', args: ['linkage'] },
    { name: 'connectionOfPositedness', args: ['subsistence'] },
  ],
  relations: [
    { predicate: 'is', from: 'selfSubsistence', to: 'returnOfNothingThroughNothing' },
  ],
  candidateSummary: 'Appearance is concrete existent mediated through its negation, which constitutes subsistence. Negation is another self-subsistent, but essentially sublated. Concrete existent is turning back into itself through negation and negation of negation. Has essential self-subsistence, equally immediately absolute positedness. Appearance is concrete existence with essentiality, positedness with ground. Ground is negation, other self-subsistent equally only positedness. Concrete existent reflected into other, has other for ground. Ground itself only to be reflected into another. Essential self-subsistence is return of nothing through nothing back to itself. Self-subsistence only reflective shine of essence. Linkage consists in reciprocal negation. Subsistence of one is not subsistence of other but its positedness. Connection of positedness alone constitutes subsistence.',
  provenance: {
    sourceChunk: 'law-6',
    sourceOp: 'law-op-6-appearance-mediated',
  },
};

export const lawOp7PositiveIdentity: LogicalOperation = {
  id: 'law-op-7-positive-identity',
  chunkId: 'law-7',
  label: 'Positive identity — essential content',
  clauses: [
    'positiveIdentity = inNegativeMediation',
    'positedness = referringItselfToPositedness',
    'essentialContent = positednessAndPermanent',
  ],
  predicates: [
    { name: 'inNegativeMediation', args: ['positiveIdentity'] },
    { name: 'referringItselfToPositedness', args: ['positedness'] },
    { name: 'positednessAndPermanent', args: ['essentialContent'] },
  ],
  relations: [
    { predicate: 'containedIn', from: 'positiveIdentity', to: 'negativeMediation' },
  ],
  candidateSummary: 'In negative mediation, immediately contained positive identity of concrete existent with itself. Concrete existent not positedness vis-à-vis essential ground but positedness referring itself to positedness. Reflective shine only in reflective shine. In negation, in other which is sublated, refers to itself. Self-identical or positive essentiality. Essential content has two sides: in form of positedness or external immediacy, and positedness as self-identical. According to first, determinate being, accidental, unessential, subject to transition. According to other, simple content determination exempted from flux, permanent element.',
  provenance: {
    sourceChunk: 'law-7',
    sourceOp: 'law-op-7-positive-identity',
  },
};

export const lawOp8CompleteDeterminateness: LogicalOperation = {
  id: 'law-op-8-complete-determinateness',
  chunkId: 'law-8',
  label: 'Content — complete determinateness',
  clauses: [
    'content = reflectionOfAppearance',
    'determinateness = oneAndItsOther',
    'unity = lawOfAppearance',
  ],
  predicates: [
    { name: 'reflectionOfAppearance', args: ['content'] },
    { name: 'oneAndItsOther', args: ['determinateness'] },
    { name: 'lawOfAppearance', args: ['unity'] },
  ],
  relations: [
    { predicate: 'is', from: 'unity', to: 'lawOfAppearance' },
  ],
  candidateSummary: 'Content is reflection of appearance, negative determinate being, into itself. Contains determinateness essentially. Appearance is multifarious diversity, unessential manifoldness. Reflected content is manifoldness reduced to simple difference. Determinate essential content is complete determinateness: one and its other. Each has subsistence in other, but only in other\'s non-subsistence. Contradiction sublates itself. Reflection into itself is identity of two-sided subsistence. Positedness of one also positedness of other. Two constitute one subsistence, each different content indifferent to other. In essential side, negativity gone back into identity. Indifferent subsistence which is not sublatedness of other but its subsistence. This unity is law of appearance.',
  provenance: {
    sourceChunk: 'law-8',
    sourceOp: 'law-op-8-complete-determinateness',
  },
};

export const lawOp9PositiveElement: LogicalOperation = {
  id: 'law-op-9-positive-element',
  chunkId: 'law-9',
  label: 'Law — positive element',
  clauses: [
    'law = positiveElementOfMediation',
    'appearance = negativeMediation',
    'law = persistenceInFlux',
  ],
  predicates: [
    { name: 'positiveElementOfMediation', args: ['law'] },
    { name: 'negativeMediation', args: ['appearance'] },
    { name: 'persistenceInFlux', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'positiveElementOfMediation' },
  ],
  candidateSummary: 'Law is positive element of mediation of what appears. Appearance is concrete existence as negative self-mediation. Concrete existent through own non-subsistence, through other, through non-subsistence of other, mediated with itself. Contains, first, merely reflective shining and disappearing, unessential appearance. Second, persistence or law. Each concretely exists in sublation of other. Positedness as negativity at same time identical positive positedness of both.',
  provenance: {
    sourceChunk: 'law-9',
    sourceOp: 'law-op-9-positive-element',
  },
};

export const lawOp10OpposedToImmediacy: LogicalOperation = {
  id: 'law-op-10-opposed-to-immediacy',
  chunkId: 'law-10',
  label: 'Law — opposed to immediacy',
  clauses: [
    'law = opposedToImmediacy',
    'thing = becomeOpposition',
    'positiveElement = selfIdentity',
  ],
  predicates: [
    { name: 'opposedToImmediacy', args: ['law'] },
    { name: 'becomeOpposition', args: ['thing'] },
    { name: 'selfIdentity', args: ['positiveElement'] },
  ],
  relations: [
    { predicate: 'opposedTo', from: 'law', to: 'immediacy' },
  ],
  candidateSummary: 'Permanent subsistence appearance obtains in law. First, opposed to immediacy of being which concrete existence has. Immediacy is in itself reflected, ground gone back into itself. In appearance simple immediacy distinguished from reflected immediacy. That first began to separate in \'thing.\' Concretely existing thing in dissolution has become this opposition. Positive element of dissolution is self-identity of what appears, positedness in positedness of its other.',
  provenance: {
    sourceChunk: 'law-10',
    sourceOp: 'law-op-10-opposed-to-immediacy',
  },
};

export const lawOp11Positedness: LogicalOperation = {
  id: 'law-op-11-positedness',
  chunkId: 'law-11',
  label: 'Law — positedness',
  clauses: [
    'positedness = essentialAndTrue',
    'gesetz = containsPositedness',
    'positedness = essentialConnection',
    'positedness = onlyToExtentOtherIs',
  ],
  predicates: [
    { name: 'essentialAndTrue', args: ['positedness'] },
    { name: 'containsPositedness', args: ['gesetz'] },
    { name: 'essentialConnection', args: ['positedness'] },
    { name: 'onlyToExtentOtherIs', args: ['positedness'] },
  ],
  relations: [
    { predicate: 'is', from: 'positedness', to: 'essentialAndTrue' },
  ],
  candidateSummary: 'Second, reflected immediacy determined as positedness over against immediate determinate being. Positedness is essential and true positive. German Gesetz contains note of positedness or Gesetztsein. In positedness lies essential connection of two sides of difference. Diverse content, each immediate with respect to other. As reflection of disappearing content. As essential difference, simple, self-referring determinations. Neither immediate, just for itself, but essential positedness. Only to extent other is.',
  provenance: {
    sourceChunk: 'law-11',
    sourceOp: 'law-op-11-positedness',
  },
};

export const lawOp12SameContent: LogicalOperation = {
  id: 'law-op-12-same-content',
  chunkId: 'law-12',
  label: 'Appearance and law — same content',
  clauses: [
    'appearanceAndLaw = sameContent',
    'law = substrateItself',
    'identity = essentialContent',
  ],
  predicates: [
    { name: 'sameContent', args: ['appearanceAndLaw'] },
    { name: 'substrateItself', args: ['law'] },
    { name: 'essentialContent', args: ['identity'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'substrateItself' },
  ],
  candidateSummary: 'Third, appearance and law have one and same content. Law is reflection of appearance into self-identity. Appearance, as immediate which is null, opposed to immanently reflected. Distinguished according to form. Reflection is essential identity of appearance and reflection. Nature of reflection: in positedness self-identical and indifferent to difference. Content continuous from appearance to law. Content constitutes substrate of appearance. Law is substrate itself. Appearance is same content but contains more: unessential content. Form determination is content. Concrete existence is thinghood with properties and matters. Content whose self-subsisting immediacy is also only non-subsistence. Self-identity of content in non-subsistence is other, essential content. Identity, substrate, constitutes law. Is appearance\'s own moment, positive side of essentiality.',
  provenance: {
    sourceChunk: 'law-12',
    sourceOp: 'law-op-12-same-content',
  },
};

export const lawOp13KingdomOfLaws: LogicalOperation = {
  id: 'law-op-13-kingdom-of-laws',
  chunkId: 'law-13',
  label: 'Law — kingdom of laws',
  clauses: [
    'law = immediatelyPresent',
    'kingdomOfLaws = restfulCopy',
    'concretelyExistingWorld = kingdomOfLaws',
    'law = essentiality',
  ],
  predicates: [
    { name: 'immediatelyPresent', args: ['law'] },
    { name: 'restfulCopy', args: ['kingdomOfLaws'] },
    { name: 'kingdomOfLaws', args: ['concretelyExistingWorld'] },
    { name: 'essentiality', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'concretelyExistingWorld', to: 'kingdomOfLaws' },
  ],
  candidateSummary: 'Law not beyond appearance but immediately present in it. Kingdom of laws is restful copy of concretely existing or appearing world. Two are one totality. Concretely existing world is itself kingdom of laws. Simple identity, self-identical in positedness or self-dissolving self-subsistence. In law, concrete existence returns to ground. Appearance contains both: simple ground and dissolving movement of appearing universe. Law is essentiality.',
  provenance: {
    sourceChunk: 'law-13',
    sourceOp: 'law-op-13-kingdom-of-laws',
  },
};

export const lawOp14EssentialAppearance: LogicalOperation = {
  id: 'law-op-14-essential-appearance',
  chunkId: 'law-14',
  label: 'Law — essential appearance',
  clauses: [
    'law = essentialAppearance',
    'law = indifferentToConcreteExistence',
    'appearance = contentNotPositedByLaw',
  ],
  predicates: [
    { name: 'essentialAppearance', args: ['law'] },
    { name: 'indifferentToConcreteExistence', args: ['law'] },
    { name: 'contentNotPositedByLaw', args: ['appearance'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'essentialAppearance' },
  ],
  candidateSummary: 'Law is essential appearance. Latter\'s reflection into itself in positedness. Identical content of itself and unessential concrete existence. Identity immediate, simple identity. Law indifferent with respect to concrete existence. Appearance still has another content contrasted with content of law. Unessential and return into latter. For law original starting point not posited by it. Externally bound up with law. Appearance is aggregate of more detailed determinations. Belong to \'this\' or concrete, not contained in law. Determined each by other.',
  provenance: {
    sourceChunk: 'law-14',
    sourceOp: 'law-op-14-essential-appearance',
  },
};

export const lawOp15RestlessForm: LogicalOperation = {
  id: 'law-op-15-restless-form',
  chunkId: 'law-15',
  label: 'Law — restless form',
  clauses: [
    'kingdomOfLaws = restfulContent',
    'appearance = restlessFlux',
    'appearance = containsLawAndMore',
  ],
  predicates: [
    { name: 'restfulContent', args: ['kingdomOfLaws'] },
    { name: 'restlessFlux', args: ['appearance'] },
    { name: 'containsLawAndMore', args: ['appearance'] },
  ],
  relations: [
    { predicate: 'contains', from: 'appearance', to: 'lawAndMore' },
  ],
  candidateSummary: 'That which appearance contains distinct from law determined as positive or another content. But essentially negative. Form and movement belongs to appearance. Kingdom of laws is restful content of appearance. Appearance is same content but displayed in restless flux. Reflection-into-other. Law as negative, relentlessly self-mutating concrete existence. Movement of passing over into opposite, self-sublation and return into unity. Restless form or negativity does not contain law. Appearance is totality, contains law but more: moment of self-moving form.',
  provenance: {
    sourceChunk: 'law-15',
    sourceOp: 'law-op-15-restless-form',
  },
};

export const lawOp16DiversityAndNecessity: LogicalOperation = {
  id: 'law-op-16-diversity-and-necessity',
  chunkId: 'law-16',
  label: 'Law — diversity and necessity',
  clauses: [
    'law = mereDiversity',
    'connection = immediateNotNecessary',
    'law = drawnFromExperience',
    'law = doesNotContainProof',
  ],
  predicates: [
    { name: 'mereDiversity', args: ['law'] },
    { name: 'immediateNotNecessary', args: ['connection'] },
    { name: 'drawnFromExperience', args: ['law'] },
    { name: 'doesNotContainProof', args: ['law'] },
  ],
  relations: [
    { predicate: 'lacks', from: 'law', to: 'objectiveNecessity' },
  ],
  candidateSummary: 'Shortcoming manifested in law in mere diversity at first. Consequent internal indifference of content. Identity of sides immediate and hence inner, not yet necessary. In law two content determinations essentially bound together. Example: spatial and temporal magnitudes in law of falling bodies. Traversed spaces vary as squares of elapsed times. Bound together, connection at first only immediate, posited. Essential unity would be negativity: each contains other in it. But in law essential unity has not yet come to fore. Not contained in concept of space that time corresponds as square. Time refers to space and space to time does not lie in determination of time itself. Time can be represented without space, space without time. One comes to other externally, external reference is movement. More particular determination indifferent. Law drawn from experience, immediate. Proof required, mediation, to know law not only occurs but is necessary. Law as such does not contain proof and objective necessity.',
  provenance: {
    sourceChunk: 'law-16',
    sourceOp: 'law-op-16-diversity-and-necessity',
  },
};

export const lawOp17EssentialForm: LogicalOperation = {
  id: 'law-op-17-essential-form',
  chunkId: 'law-17',
  label: 'Law — essential form',
  clauses: [
    'law = onlyPositiveEssentiality',
    'content = indifferentToConnection',
    'law = essentialFormNotYetRealForm',
  ],
  predicates: [
    { name: 'onlyPositiveEssentiality', args: ['law'] },
    { name: 'indifferentToConnection', args: ['content'] },
    { name: 'essentialFormNotYetRealForm', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'essentialForm' },
  ],
  candidateSummary: 'Law is only positive essentiality of appearance. Not negative essentiality according to which content determinations are moments of form. Pass over into other, in own selves not themselves but their other. In law, although positedness of one side is positedness of other side, content of two sides indifferent to connection. Does not contain positedness in it. Law is indeed essential form, but not as yet real form which is reflected into sides as content.',
  provenance: {
    sourceChunk: 'law-17',
    sourceOp: 'law-op-17-essential-form',
  },
};

export const lawOperations: LogicalOperation[] = [
  lawOp1EssenceInConcreteExistence,
  lawOp2OnlyAppearance,
  lawOp3HigherTruth,
  lawOp4RealShine,
  lawOp5ThreeMoments,
  lawOp6AppearanceMediated,
  lawOp7PositiveIdentity,
  lawOp8CompleteDeterminateness,
  lawOp9PositiveElement,
  lawOp10OpposedToImmediacy,
  lawOp11Positedness,
  lawOp12SameContent,
  lawOp13KingdomOfLaws,
  lawOp14EssentialAppearance,
  lawOp15RestlessForm,
  lawOp16DiversityAndNecessity,
  lawOp17EssentialForm,
];
