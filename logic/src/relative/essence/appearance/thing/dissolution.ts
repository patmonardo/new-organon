/**
 * Logical Operations: Dissolution of the Thing
 *
 * Dissolution is the culmination of the Thing's development, where the thing
 * reveals itself as absolutely alterable, porous, and self-contradictory,
 * leading to the transition to Appearance.
 *
 * Dialectical Movement:
 * - Absolute alterability: external dissolution
 * - Porosity: interpenetration of matters
 * - Appearance: truth of concrete existence
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// DISSOLUTION OF THE THING
// ============================================================================

export const disOp1AbsolutelyAlterable: LogicalOperation = {
  id: 'dis-op-1-absolutely-alterable',
  chunkId: 'dis-1',
  label: 'Thing absolutely alterable — dissolution',
  clauses: [
    'thing = absolutelyAlterable',
    'alteration = mattersDroppedAdded',
    'comingToBeAndPassingAway = externalDissolution',
    'thing = absolutePorosity',
    'thing = withoutMeasureOrForm',
  ],
  predicates: [
    { name: 'absolutelyAlterable', args: ['thing'] },
    { name: 'mattersDroppedAdded', args: ['alteration'] },
    { name: 'externalDissolution', args: ['comingToBeAndPassingAway'] },
    { name: 'absolutePorosity', args: ['thing'] },
    { name: 'withoutMeasureOrForm', args: ['thing'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'absolutelyAlterable' },
  ],
  candidateSummary: 'Thing, as merely quantitative combination of free matters, is absolutely alterable. Alteration consists in matters being dropped from collection, added to \'also,\' or rearrangement of matters\' quantitative ratio. Coming-to-be and passing-away is external dissolution of external bond, binding for which indifferent whether bound or not. Stuffs circulate unchecked in or out of \'this\' thing. Thing itself is absolute porosity without measure or form of its own.',
  provenance: {
    sourceChunk: 'dis-1',
    sourceOp: 'dis-op-1-absolutely-alterable',
  },
};

export const disOp2AbsolutelyDissoluble: LogicalOperation = {
  id: 'dis-op-2-absolutely-dissoluble',
  chunkId: 'dis-2',
  label: 'Absolutely dissoluble thing — externality',
  clauses: [
    'thing = absolutelyDissoluble',
    'dissolution = externalProcess',
    'thing = onlyAlso',
    'thing = consistsOnlyOfExternality',
    'thingWhole = dissolutionOfItself',
  ],
  predicates: [
    { name: 'absolutelyDissoluble', args: ['thing'] },
    { name: 'externalProcess', args: ['dissolution'] },
    { name: 'onlyAlso', args: ['thing'] },
    { name: 'consistsOnlyOfExternality', args: ['thing'] },
    { name: 'dissolutionOfItself', args: ['thingWhole'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'dissolutionOfItself' },
  ],
  candidateSummary: 'Thing, in absolute determinateness through which it is \'this,\' is absolutely dissoluble thing. Dissolution is external process of being determined, just like being of thing. But dissolution and externality of being is essential of this being. Thing is only \'also,\' consists only of externality. But consists also of matters. Not just abstract \'this\' but \'this\' thing whole is dissolution of itself. Thing determined as external collection of self-subsisting matters.',
  provenance: {
    sourceChunk: 'dis-2',
    sourceOp: 'dis-op-2-absolutely-dissoluble',
  },
};

export const disOp3MattersNegativeReference: LogicalOperation = {
  id: 'dis-op-3-matters-negative-reference',
  chunkId: 'dis-3',
  label: 'Matters — negative reference',
  clauses: [
    'matters = lackNegativeSelfSubsistence',
    'content = determinateness',
    'content = refersToOther',
    'matters = negativeReflection',
    'oneMatter = notWhatOtherIs',
  ],
  predicates: [
    { name: 'lackNegativeSelfSubsistence', args: ['matters'] },
    { name: 'determinateness', args: ['content'] },
    { name: 'refersToOther', args: ['content'] },
    { name: 'negativeReflection', args: ['matters'] },
    { name: 'notWhatOtherIs', args: ['oneMatter'] },
  ],
  relations: [
    { predicate: 'are', from: 'matters', to: 'negativeReflection' },
  ],
  candidateSummary: 'Matters are not things, lack negative self-subsistence. Properties are rather self-subsistent, determined with being reflected into itself. Matters are simple, referring only to themselves. But content is determinateness. Immanent reflection only form of content. Content not reflected-into-itself but refers to other according to determinateness. Thing not only \'also\' but equally negative reference. On account of determinateness, matters themselves negative reflection, which is puncticity of thing. One matter is not what other is, one is not to extent other is.',
  provenance: {
    sourceChunk: 'dis-3',
    sourceOp: 'dis-op-3-matters-negative-reference',
  },
};

export const disOp4ConnectingReferenceInterpenetration: LogicalOperation = {
  id: 'dis-op-4-connecting-reference-interpenetration',
  chunkId: 'dis-4',
  label: 'Connecting reference — interpenetration',
  clauses: [
    'thing = connectingReferenceOfMatters',
    'oneSubsists = otherSublated',
    'oneSubsists = otherAlsoSubsists',
    'subsistence = selfContradictory',
  ],
  predicates: [
    { name: 'connectingReferenceOfMatters', args: ['thing'] },
    { name: 'otherSublated', args: ['oneSubsists'] },
    { name: 'otherAlsoSubsists', args: ['oneSubsists'] },
    { name: 'selfContradictory', args: ['subsistence'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'connectingReferenceOfMatters' },
  ],
  candidateSummary: 'Thing is connecting reference of matters to each other. One matter and other also subsist in it. Yet at same time, one matter does not subsist in it in so far as other does. To extent one matter is in thing, other thereby sublated. But thing at same time \'also,\' subsistence of other matter. In subsistence of one matter, other does not subsist, and also no less subsists in it. So with all diverse matters in respect to each other.',
  provenance: {
    sourceChunk: 'dis-4',
    sourceOp: 'dis-op-4-connecting-reference-interpenetration',
  },
};

export const disOp5PuncticityPorosity: LogicalOperation = {
  id: 'dis-op-5-puncticity-porosity',
  chunkId: 'dis-5',
  label: 'Puncticity — porosity',
  clauses: [
    'subsistence = puncticity',
    'subsistence = negativeUnity',
    'matters = interpolate',
    'matters = essentiallyPorous',
    'thing = selfContradictoryMediation',
  ],
  predicates: [
    { name: 'puncticity', args: ['subsistence'] },
    { name: 'negativeUnity', args: ['subsistence'] },
    { name: 'interpolate', args: ['matters'] },
    { name: 'essentiallyPorous', args: ['matters'] },
    { name: 'selfContradictoryMediation', args: ['thing'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'selfContradictoryMediation' },
  ],
  candidateSummary: 'In same respect as one matter subsists, other subsists also. One subsistence of both is puncticity or negative unity of thing. Two interpenetrate absolutely. Thing at same time only \'also\' of matters. These reflected into determinateness, indifferent to one another. In interpenetrating do not touch. Matters essentially porous. One subsists in pores or non-subsistence of others. Others themselves porous. In their pores or non-subsistence, first and all rest subsist. Subsistence at same time sublatedness and subsistence of others. Subsistence of others just as much sublatedness and subsisting of first and all others. Thing is self-contradictory mediation of independent self-subsistence through opposite, through negation, or one self-subsisting matter through subsisting and non-subsisting of other.',
  provenance: {
    sourceChunk: 'dis-5',
    sourceOp: 'dis-op-5-puncticity-porosity',
  },
};

export const disOp6ConcreteExistenceAppearance: LogicalOperation = {
  id: 'dis-op-6-concrete-existence-appearance',
  chunkId: 'dis-6',
  label: 'Concrete existence — appearance',
  clauses: [
    'concreteExistence = attainedCompletion',
    'truth = hasInItselfInUnessentiality',
    'existent = subsistsInAbsoluteOther',
    'existent = hasNothingnessForSubstrate',
    'existent = appearance',
  ],
  predicates: [
    { name: 'attainedCompletion', args: ['concreteExistence'] },
    { name: 'hasInItselfInUnessentiality', args: ['truth'] },
    { name: 'subsistsInAbsoluteOther', args: ['existent'] },
    { name: 'hasNothingnessForSubstrate', args: ['existent'] },
    { name: 'appearance', args: ['existent'] },
  ],
  relations: [
    { predicate: 'is', from: 'existent', to: 'appearance' },
  ],
  candidateSummary: 'In \'this\' thing, concrete existence has attained completion. At once being that exists in itself, independent subsistence, and unessential concrete existence. Truth of concrete existence: has its in-itself in unessentiality. Subsists in other, indeed absolute other. Has its own nothingness for substrate. It is, therefore, appearance.',
  provenance: {
    sourceChunk: 'dis-6',
    sourceOp: 'dis-op-6-concrete-existence-appearance',
  },
};

export const dissolutionOperations: LogicalOperation[] = [
  disOp1AbsolutelyAlterable,
  disOp2AbsolutelyDissoluble,
  disOp3MattersNegativeReference,
  disOp4ConnectingReferenceInterpenetration,
  disOp5PuncticityPorosity,
  disOp6ConcreteExistenceAppearance,
];
