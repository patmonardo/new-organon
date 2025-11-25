/**
 * Logical Operations: The World of Appearance and the World-In-Itself
 *
 * The World emerges as the kingdom of laws, the suprasensible world standing in
 * relation to the world of appearance as its ground and inversion.
 *
 * Dialectical Movement:
 * - Kingdom of laws: simple identity
 * - World-in-itself: suprasensible world
 * - Inversion: world-in-itself as opposite
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE WORLD OF APPEARANCE AND THE WORLD-IN-ITSELF
// ============================================================================

export const wldOp1KingdomOfLaws: LogicalOperation = {
  id: 'wld-op-1-kingdom-of-laws',
  chunkId: 'wld-1',
  label: 'World raises itself to kingdom of laws',
  clauses: [
    'world = raisesItselfToKingdomOfLaws',
    'law = simpleIdentityOfAppearance',
    'law = substrateNotGround',
    'appearingThings = haveGroundsInOthers',
  ],
  predicates: [
    { name: 'raisesItselfToKingdomOfLaws', args: ['world'] },
    { name: 'simpleIdentityOfAppearance', args: ['law'] },
    { name: 'substrateNotGround', args: ['law'] },
    { name: 'haveGroundsInOthers', args: ['appearingThings'] },
  ],
  relations: [
    { predicate: 'raisesTo', from: 'world', to: 'kingdomOfLaws' },
  ],
  candidateSummary: 'Concrete existing world tranquilly raises itself to kingdom of laws. Null content of manifold determinate being has subsistence in other. Subsistence is dissolution. In other, that which appears comes to itself. Appearance in changing also enduring, positedness is law. Law is simple identity of appearance with itself. Substrate and not ground, not negative unity. Simple identity, immediate unity, abstract unity. Other content also occurs alongside. Content holds together internally, negative reflection inside itself. Reflected into other, other is concrete existence of appearance. Appearing things have grounds and conditions in other appearing things.',
  provenance: {
    sourceChunk: 'wld-1',
    sourceOp: 'wld-op-1-kingdom-of-laws',
  },
};

export const wldOp2LawNegativeReflection: LogicalOperation = {
  id: 'wld-op-2-law-negative-reflection',
  chunkId: 'wld-2',
  label: 'Law — negative reflection',
  clauses: [
    'law = otherOfAppearance',
    'law = negativeReflection',
    'reflectionOfPositedness = law',
    'law = negativeUnity',
  ],
  predicates: [
    { name: 'otherOfAppearance', args: ['law'] },
    { name: 'negativeReflection', args: ['law'] },
    { name: 'law', args: ['reflectionOfPositedness'] },
    { name: 'negativeUnity', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'negativeUnity' },
  ],
  candidateSummary: 'Law is also other of appearance as appearance, negative reflection as in its other. Content of appearance differing from content of law is concrete existent which has negativity for ground, reflected into non-being. Other is concrete existent likewise reflected into non-being. Same, that which appears reflected not into other but into itself. Reflection of positedness into itself is law. As something that appears, essentially reflected into non-being. Identity essentially just as much negativity and other. Immanent reflection, law, not only identical substrate. Appearance has in law its opposite, law is negative unity.',
  provenance: {
    sourceChunk: 'wld-2',
    sourceOp: 'wld-op-2-law-negative-reflection',
  },
};

export const wldOp3DeterminationAltered: LogicalOperation = {
  id: 'wld-op-3-determination-altered',
  chunkId: 'wld-3',
  label: 'Determination of law altered — negative unity',
  clauses: [
    'determination = alteredWithinLaw',
    'sides = negativelyReferring',
    'eachSide = negativeUnity',
    'identity = positedAndReal',
  ],
  predicates: [
    { name: 'alteredWithinLaw', args: ['determination'] },
    { name: 'negativelyReferring', args: ['sides'] },
    { name: 'negativeUnity', args: ['eachSide'] },
    { name: 'positedAndReal', args: ['identity'] },
  ],
  relations: [
    { predicate: 'is', from: 'identity', to: 'positedAndReal' },
  ],
  candidateSummary: 'Determination of law altered within law itself. At first, law only diversified content and formal reflection of positedness into itself. Positedness of one side is positedness of other side. But because also negative reflection into itself, sides behave not only as different but negatively referring to each other. If law considered for itself, sides indifferent. But no less sublated through identity. Positedness of one is positedness of other. Subsistence of each also non-subsistence of itself. Positedness of one side in other is negative unity. Each side itself negative unity. Positive identity at first only inner unity, stands in need of proof. But since sides determined as different in negative unity, each contains other within while repelling otherness. Identity of law now posited and real.',
  provenance: {
    sourceChunk: 'wld-3',
    sourceOp: 'wld-op-3-determination-altered',
  },
};

export const wldOp4LawObtainsNegativeMoment: LogicalOperation = {
  id: 'wld-op-4-law-obtains-negative-moment',
  chunkId: 'wld-4',
  label: 'Law obtains negative moment — world-in-itself',
  clauses: [
    'law = obtainedNegativeMoment',
    'law = essentialTotalityOfAppearance',
    'content = essentiallyConnected',
    'worldInItself = disclosesItself',
  ],
  predicates: [
    { name: 'obtainedNegativeMoment', args: ['law'] },
    { name: 'essentialTotalityOfAppearance', args: ['law'] },
    { name: 'essentiallyConnected', args: ['content'] },
    { name: 'disclosesItself', args: ['worldInItself'] },
  ],
  relations: [
    { predicate: 'disclosesItself', from: 'worldInItself', to: 'aboveWorldOfAppearance' },
  ],
  candidateSummary: 'Law has obtained missing moment of negative form of sides. Moment previously belonged to appearance. Concrete existence returned into itself fully. Reflected itself into absolute otherness which has determinate being-in-and-for-itself. That which was law no longer only one side of whole. Essential totality of appearance. Obtains moment of unessentiality as reflected unessentiality that has determinate being in itself, essential negativity. As immediate content, law determined in general, distinguished from other laws. But because explicitly essential negativity, no longer contains indifferent, accidental content determination. Content rather every determinateness essentially connected together, totalizing connection. Appearance reflected-into-itself is world that discloses itself above world of appearance, one which is in and for itself.',
  provenance: {
    sourceChunk: 'wld-4',
    sourceOp: 'wld-op-4-law-obtains-negative-moment',
  },
};

export const wldOp5SuprasensibleWorld: LogicalOperation = {
  id: 'wld-op-5-suprasensible-world',
  chunkId: 'wld-5',
  label: 'Kingdom of laws — suprasensible world',
  clauses: [
    'kingdomOfLaws = simpleUnchangingContent',
    'kingdomOfLaws = containsEssencelessManifoldness',
    'worldInItself = suprasensibleWorld',
    'sensibleWorld = forIntuition',
    'suprasensibleWorld = reflectedConcreteExistence',
  ],
  predicates: [
    { name: 'simpleUnchangingContent', args: ['kingdomOfLaws'] },
    { name: 'containsEssencelessManifoldness', args: ['kingdomOfLaws'] },
    { name: 'suprasensibleWorld', args: ['worldInItself'] },
    { name: 'forIntuition', args: ['sensibleWorld'] },
    { name: 'reflectedConcreteExistence', args: ['suprasensibleWorld'] },
  ],
  relations: [
    { predicate: 'is', from: 'worldInItself', to: 'suprasensibleWorld' },
  ],
  candidateSummary: 'Kingdom of laws contains simple, unchanging but diversified content of concretely existing world. But because total reflection, also contains moment of essenceless manifoldness. Moment of alterability and alteration, reflected into itself and essential, is absolute negativity or form in general. Moments have reality of self-subsisting but reflected concrete existence in world that has determinate being in-and-for-itself. World in and for itself also called suprasensible world. Concretely existing world characterized as sensible, intended for intuition, immediate attitude of consciousness. Suprasensible world likewise has immediate, concrete existence, but reflected, essential concrete existence. Essence has no immediate existence yet, but it is, in more profound sense than being. Things posited only as things of another, suprasensible, world, first as true concrete existences, second as truth in contrast to that which just is. Recognized: being distinguished from immediate being, this being is true concrete existence.',
  provenance: {
    sourceChunk: 'wld-5',
    sourceOp: 'wld-op-5-suprasensible-world',
  },
};

export const wldOp6WorldInItselfTotality: LogicalOperation = {
  id: 'wld-op-6-world-in-itself-totality',
  chunkId: 'wld-6',
  label: 'World-in-itself — totality',
  clauses: [
    'worldInItself = totality',
    'worldInItself = splitsInternally',
    'essentialWorld = positingGround',
    'essentialWorld = makesItselfIntoPositedness',
  ],
  predicates: [
    { name: 'totality', args: ['worldInItself'] },
    { name: 'splitsInternally', args: ['worldInItself'] },
    { name: 'positingGround', args: ['essentialWorld'] },
    { name: 'makesItselfIntoPositedness', args: ['essentialWorld'] },
  ],
  relations: [
    { predicate: 'is', from: 'worldInItself', to: 'totality' },
  ],
  candidateSummary: 'World in and for itself is totality of concrete existence. Outside it nothing. Within it, absolute negativity or form. Immanent reflection is negative self-reference. Contains opposition, splits internally as world of senses and world of otherness or world of appearance. Since totality, also only one side of totality. Constitutes self-subsistence different from world of appearance. World of appearance has negative unity in essential world, to which it founders and into which returns as to ground. Essential world is positing ground of world of appearances. Contains absolute form essentially. Sublates self-identity, makes itself into positedness. As posited immediacy, is world of appearance.',
  provenance: {
    sourceChunk: 'wld-6',
    sourceOp: 'wld-op-6-world-in-itself-totality',
  },
};

export const wldOp7DeterminateGround: LogicalOperation = {
  id: 'wld-op-7-determinate-ground',
  chunkId: 'wld-7',
  label: 'Determinate ground — opposition',
  clauses: [
    'ground = determinateGround',
    'kingdomOfLaws = splitsIntoTwoWorlds',
    'identity = essentialConnectionOfOpposition',
    'identity = becomingAndTransition',
  ],
  predicates: [
    { name: 'determinateGround', args: ['ground'] },
    { name: 'splitsIntoTwoWorlds', args: ['kingdomOfLaws'] },
    { name: 'essentialConnectionOfOpposition', args: ['identity'] },
    { name: 'becomingAndTransition', args: ['identity'] },
  ],
  relations: [
    { predicate: 'is', from: 'identity', to: 'becomingAndTransition' },
  ],
  candidateSummary: 'Not only ground in general but determinate ground. Already as kingdom of laws manifold of content, essential content of world of appearance. As ground with content, determinate ground of other world. But such only according to content. World of appearance had other and manifold content than kingdom of laws, negative moment still peculiarly its own. But because kingdom of laws now has this moment in it, totality of content, ground of all manifoldness. But at same time negative of manifoldness, world opposed to it. In identity of two worlds, connection of ground restored as ground-connection of appearance. Not connection of identical content nor mere diversified content, but total connection, negative identity and essential connection of opposed sides. Kingdom of laws has negative moment, opposition. As totality, splits into world in-and-for-itself and world of appearance. Identity is essential connection of opposition. Connection of ground is opposition which in contradiction foundered to ground. Concrete existence becomes appearance. Ground reinstates itself as return of appearance into itself, as sublated ground, ground-connection of opposite determinations. Identity essentially becoming and transition, no longer connection of ground as such.',
  provenance: {
    sourceChunk: 'wld-7',
    sourceOp: 'wld-op-7-determinate-ground',
  },
};

export const wldOp8Inversion: LogicalOperation = {
  id: 'wld-op-8-inversion',
  chunkId: 'wld-8',
  label: 'World-in-itself — inversion',
  clauses: [
    'worldInItself = distinguishedWithinItself',
    'worldInItself = identicalWithAppearance',
    'connection = determinedAsOpposition',
    'worldInItself = inversionOfAppearance',
  ],
  predicates: [
    { name: 'distinguishedWithinItself', args: ['worldInItself'] },
    { name: 'identicalWithAppearance', args: ['worldInItself'] },
    { name: 'determinedAsOpposition', args: ['connection'] },
    { name: 'inversionOfAppearance', args: ['worldInItself'] },
  ],
  relations: [
    { predicate: 'is', from: 'worldInItself', to: 'inversionOfAppearance' },
  ],
  candidateSummary: 'World in-and-for-itself itself world distinguished within itself, in total compass of manifold content. Identical with world of appearance or posited world, to this extent its ground. But identity connection at same time determined as opposition. Form of world of appearance is reflection into its otherness. World of appearance in world in-and-for-itself truly returned into itself, in such manner that other world is its opposite. Connection specifically this: world in-and-for-itself is inversion of world of appearance.',
  provenance: {
    sourceChunk: 'wld-8',
    sourceOp: 'wld-op-8-inversion',
  },
};

export const worldOperations: LogicalOperation[] = [
  wldOp1KingdomOfLaws,
  wldOp2LawNegativeReflection,
  wldOp3DeterminationAltered,
  wldOp4LawObtainsNegativeMoment,
  wldOp5SuprasensibleWorld,
  wldOp6WorldInItselfTotality,
  wldOp7DeterminateGround,
  wldOp8Inversion,
];
