/**
 * Logical Operations: The Dissolution of Appearance
 *
 * Disappearance (Dissolution) is the culmination of Appearance's development, where
 * the two worlds collapse into each other, revealing appearance as essential relation.
 *
 * Dialectical Movement:
 * - Opposition and inversion: two worlds
 * - Difference disappeared: each becomes other
 * - Essential relation: consummation of unity
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE DISSOLUTION OF APPEARANCE
// ============================================================================

export const disaOp1OppositionAndInversion: LogicalOperation = {
  id: 'disa-op-1-opposition-and-inversion',
  chunkId: 'disa-1',
  label: 'Two worlds — opposition and inversion',
  clauses: [
    'worldInItself = determinateGround',
    'worldInItself = negativeMoment',
    'twoWorlds = completelyOpposed',
    'positive = negativeInWorldInItself',
  ],
  predicates: [
    { name: 'determinateGround', args: ['worldInItself'] },
    { name: 'negativeMoment', args: ['worldInItself'] },
    { name: 'completelyOpposed', args: ['twoWorlds'] },
    { name: 'negativeInWorldInItself', args: ['positive'] },
  ],
  relations: [
    { predicate: 'is', from: 'worldInItself', to: 'determinateGround' },
  ],
  candidateSummary: 'World in-and-for-itself is determinate ground of world of appearance, only in so far as within it is negative moment, totality of content determinations. Constitutes completely opposed side. Two worlds relate: what in world of appearance is positive, in world-in-itself is negative, and conversely. North pole in appearance is south pole in-and-for-itself, vice-versa. Positive electricity in itself negative. What is evil in appearance is in-and-for-itself goodness and good luck.',
  provenance: {
    sourceChunk: 'disa-1',
    sourceOp: 'disa-op-1-opposition-and-inversion',
  },
};

export const disaOp2DifferenceDisappeared: LogicalOperation = {
  id: 'disa-op-2-difference-disappeared',
  chunkId: 'disa-2',
  label: 'Difference disappeared — each becomes other',
  clauses: [
    'difference = disappeared',
    'worldInItself = worldOfAppearance',
    'worldOfAppearance = worldEssential',
    'each = becomesOther',
  ],
  predicates: [
    { name: 'disappeared', args: ['difference'] },
    { name: 'worldOfAppearance', args: ['worldInItself'] },
    { name: 'worldEssential', args: ['worldOfAppearance'] },
    { name: 'becomesOther', args: ['each'] },
  ],
  relations: [
    { predicate: 'is', from: 'worldInItself', to: 'worldOfAppearance' },
  ],
  candidateSummary: 'Precisely in opposition of two worlds their difference has disappeared. What was supposed to be world-in-itself is itself world of appearance. This last, conversely, world essential within.',
  provenance: {
    sourceChunk: 'disa-2',
    sourceOp: 'disa-op-2-difference-disappeared',
  },
};

export const disaOp3LawEqualToItself: LogicalOperation = {
  id: 'disa-op-3-law-equal-to-itself',
  chunkId: 'disa-3',
  label: 'World of appearance — law equal to itself',
  clauses: [
    'worldOfAppearance = reflectionIntoOtherness',
    'other = sublatesItselfAsOther',
    'worldOfAppearance = lawEqualToItself',
  ],
  predicates: [
    { name: 'reflectionIntoOtherness', args: ['worldOfAppearance'] },
    { name: 'sublatesItselfAsOther', args: ['other'] },
    { name: 'lawEqualToItself', args: ['worldOfAppearance'] },
  ],
  relations: [
    { predicate: 'is', from: 'worldOfAppearance', to: 'lawEqualToItself' },
  ],
  candidateSummary: 'World of appearance determined as reflection into otherness. Determinations have ground and subsistence in other. But because other, as other, likewise reflected into other, other to which both refer sublates itself as other. Two consequently refer to themselves. World of appearance is within it, therefore, law equal to itself.',
  provenance: {
    sourceChunk: 'disa-3',
    sourceOp: 'disa-op-3-law-equal-to-itself',
  },
};

export const disaOp4SelfOpposed: LogicalOperation = {
  id: 'disa-op-4-self-opposed',
  chunkId: 'disa-4',
  label: 'World-in-itself — self-opposed',
  clauses: [
    'worldInItself = selfIdentical',
    'worldInItself = becomesSelOpposed',
    'worldInItself = selfInverting',
    'worldInItself = sublatedGroundAndImmediacy',
  ],
  predicates: [
    { name: 'selfIdentical', args: ['worldInItself'] },
    { name: 'becomesSelOpposed', args: ['worldInItself'] },
    { name: 'selfInverting', args: ['worldInItself'] },
    { name: 'sublatedGroundAndImmediacy', args: ['worldInItself'] },
  ],
  relations: [
    { predicate: 'becomes', from: 'worldInItself', to: 'selfOpposed' },
  ],
  candidateSummary: 'World-in-itself in first instance self-identical content, exempt from otherness and change. But content, as complete reflection of world of appearance into itself, or because diversity is difference reflected into itself and absolute, consequently contains negativity as moment. Self-reference as reference to otherness. Thereby becomes self-opposed, self-inverting, essenceless content. Content has retained form of immediate concrete existence. At first ground of world of appearance. But since has opposition in it, equally sublated ground and immediate concrete existence.',
  provenance: {
    sourceChunk: 'disa-4',
    sourceOp: 'disa-op-4-self-opposed',
  },
};

export const disaOp5TwoTotalities: LogicalOperation = {
  id: 'disa-op-5-two-totalities',
  chunkId: 'disa-5',
  label: 'Two totalities — identity of moments',
  clauses: [
    'worldOfAppearance = totality',
    'essentialWorld = totality',
    'each = continuesIntoOther',
    'selfSubsistence = inUnityOfTwo',
  ],
  predicates: [
    { name: 'totality', args: ['worldOfAppearance'] },
    { name: 'totality', args: ['essentialWorld'] },
    { name: 'continuesIntoOther', args: ['each'] },
    { name: 'inUnityOfTwo', args: ['selfSubsistence'] },
  ],
  relations: [
    { predicate: 'has', from: 'selfSubsistence', to: 'unityOfTwo' },
  ],
  candidateSummary: 'World of appearance and essential world each, each within it, totality of self-identical reflection and reflection-into-other, being-in-and-for-itself. Both self-subsisting wholes of concrete existence. One supposed to be only reflected concrete existence, other immediate concrete existence. But each continues into other and, within, identity of these two moments. Totality that splits into two totalities: reflected totality and immediate totality. Both self-subsistent but only as totalities, inasmuch as each essentially contains moment of other in it. Distinct self-subsistence now posited as essentially reference to other, has self-subsistence in unity of two.',
  provenance: {
    sourceChunk: 'disa-5',
    sourceOp: 'disa-op-5-two-totalities',
  },
};

export const disaOp6LawRealized: LogicalOperation = {
  id: 'disa-op-6-law-realized',
  chunkId: 'disa-6',
  label: 'Law realized — essential relation',
  clauses: [
    'law = realized',
    'innerIdentity = existent',
    'content = raisedToIdeality',
    'law = essentialRelation',
  ],
  predicates: [
    { name: 'realized', args: ['law'] },
    { name: 'existent', args: ['innerIdentity'] },
    { name: 'raisedToIdeality', args: ['content'] },
    { name: 'essentialRelation', args: ['law'] },
  ],
  relations: [
    { predicate: 'is', from: 'law', to: 'essentialRelation' },
  ],
  candidateSummary: 'Started from law of appearance. Law is identity of content and another content different from it. Positedness of one is positedness of other. Still present in law: identity of sides at first only internal identity, two sides do not yet have in them. Identity not realized, content indifferent, diversified. Content only in itself so determined, determination not yet present. But now law is realized. Inner identity existent at same time. Content of law raised to ideality. Sublated within, reflected into itself. Each side has other in it. Truly identical with it and with itself. Law is essential relation.',
  provenance: {
    sourceChunk: 'disa-6',
    sourceOp: 'disa-op-6-law-realized',
  },
};

export const disaOp7WorldFoundered: LogicalOperation = {
  id: 'disa-op-7-world-foundered',
  chunkId: 'disa-7',
  label: 'World foundered — essential relation',
  clauses: [
    'world = foundered',
    'world = stillTotalityAsRelation',
    'form = connectingReference',
    'essentialRelation = consummationOfUnity',
  ],
  predicates: [
    { name: 'foundered', args: ['world'] },
    { name: 'stillTotalityAsRelation', args: ['world'] },
    { name: 'connectingReference', args: ['form'] },
    { name: 'consummationOfUnity', args: ['essentialRelation'] },
  ],
  relations: [
    { predicate: 'is', from: 'essentialRelation', to: 'consummationOfUnity' },
  ],
  candidateSummary: 'Truth of unessential world is at first world-in-itself and other to it. But this world is totality, for it is itself and first world. Both immediate concrete existences, consequently reflections in their otherness, equally truly reflected into themselves. \'World\' signifies formless totality of manifoldness. This world has foundered both as essential world and as world of appearance. Still totality or universe but as essential relation. Two totalities of content arisen in appearance. At first determined as indifferently self-subsisting vis-à-vis each other. Each having form within it but not with respect to other. Form has demonstrated itself to be their connecting reference. Essential relation is consummation of their unity of form.',
  provenance: {
    sourceChunk: 'disa-7',
    sourceOp: 'disa-op-7-world-foundered',
  },
};

export const disappearanceOperations: LogicalOperation[] = [
  disaOp1OppositionAndInversion,
  disaOp2DifferenceDisappeared,
  disaOp3LawEqualToItself,
  disaOp4SelfOpposed,
  disaOp5TwoTotalities,
  disaOp6LawRealized,
  disaOp7WorldFoundered,
];
