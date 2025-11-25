/**
 * Logical Operations: Dissolution of the Thing
 *
 * The thing dissolves into appearance. In Realism, things "dissolve" (as opposed
 * to Idealism where worlds/laws "disappear"). This is the FormProcessor's analysis
 * of how things break down into their essential matters and become appearance.
 *
 * Dialectical Movement:
 * - Thing absolutely alterable: dissolution
 * - Matters: negative reference
 * - Connecting reference: interpenetration
 * - Concrete existence: appearance
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// DISSOLUTION OF THE THING
// ============================================================================

export const disOp1ThingAbsolutelyAlterable: LogicalOperation = {
  id: 'dis-op-1-thing-absolutely-alterable',
  chunkId: 'dis-1',
  label: 'Thing absolutely alterable — dissolution',
  clauses: [
    'thing = asMerelyQuantitativeCombination',
    'thing = ofFreeMatters',
    'thing = isAbsolutelyAlterable',
    'alteration = consistsInMatters',
    'alteration = beingDroppedFromCollection',
    'alteration = addedToAlso',
    'alteration = orRearrangement',
    'alteration = ofMattersQuantitativeRatio',
    'comingToBe = andPassingAway',
    'comingToBe = isExternalDissolution',
    'comingToBe = ofExternalBond',
    'comingToBe = bindingForWhich',
    'comingToBe = indifferentWhetherBound',
    'comingToBe = orNot',
    'stuffs = circulateUnchecked',
    'stuffs = inOrOutOfThisThing',
    'thing = itselfIsAbsolutePorosity',
    'thing = withoutMeasureOrForm',
    'thing = ofItsOwn',
  ],
  predicates: [
    { name: 'absolutelyAlterable', args: ['thing'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'absolutelyAlterable' },
  ],
  candidateSummary: 'Thing, as merely quantitative combination of free matters, is absolutely alterable. Alteration consists in matters being dropped from collection, added to "also," or rearrangement of matters\' quantitative ratio. Coming-to-be and passing-away is external dissolution of external bond, binding for which indifferent whether bound or not. Stuffs circulate unchecked in or out of "this" thing. Thing itself is absolute porosity without measure or form of its own.',
  provenance: {
    sourceChunk: 'dis-1',
    sourceOp: 'dis-op-1-thing-absolutely-alterable',
  },
};

export const disOp2AbsolutelyDissolubleThing: LogicalOperation = {
  id: 'dis-op-2-absolutely-dissoluble-thing',
  chunkId: 'dis-2',
  label: 'Absolutely dissoluble thing — externality',
  clauses: [
    'thing = inAbsoluteDeterminateness',
    'thing = throughWhichItIsThis',
    'thing = isAbsolutelyDissolubleThing',
    'dissolution = isExternalProcess',
    'dissolution = ofBeingDetermined',
    'dissolution = justLikeBeingOfThing',
    'but = dissolutionAndExternality',
    'but = ofBeing',
    'but = isEssential',
    'but = ofThisBeing',
    'thing = isOnlyAlso',
    'thing = consistsOnlyOfExternality',
    'but = consistsAlsoOfMatters',
    'not = justAbstractThis',
    'but = thisThing',
    'but = wholeIsDissolution',
    'but = ofItself',
    'thing = determinedAsExternal',
    'thing = collectionOfSelfSubsistingMatters',
  ],
  predicates: [
    { name: 'absolutelyDissoluble', args: ['thing'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'absolutelyDissoluble' },
  ],
  candidateSummary: 'Thing, in absolute determinateness through which it is "this," is absolutely dissoluble thing. Dissolution is external process of being determined, just like being of thing. But dissolution and externality of being is essential of this being. Thing is only "also," consists only of externality. But consists also of matters. Not just abstract "this" but "this" thing whole is dissolution of itself. Thing determined as external collection of self-subsisting matters.',
  provenance: {
    sourceChunk: 'dis-2',
    sourceOp: 'dis-op-2-absolutely-dissoluble-thing',
  },
};

export const disOp3MattersNegativeReference: LogicalOperation = {
  id: 'dis-op-3-matters-negative-reference',
  chunkId: 'dis-3',
  label: 'Matters — negative reference',
  clauses: [
    'matters = areNotThings',
    'matters = lackNegativeSelfSubsistence',
    'properties = areRatherSelfSubsistent',
    'properties = determinedWithBeing',
    'properties = reflectedIntoItself',
    'matters = areSimple',
    'matters = referringOnlyToThemselves',
    'but = content',
    'but = isDeterminateness',
    'immanentReflection = onlyFormOfContent',
    'content = notReflectedIntoItself',
    'content = butRefersToOther',
    'content = accordingToDeterminateness',
    'thing = notOnlyAlso',
    'thing = butEquallyNegativeReference',
    'onAccount = ofDeterminateness',
    'matters = themselvesNegativeReflection',
    'matters = whichIsPuncticityOfThing',
    'oneMatter = isNotWhatOtherIs',
    'one = isNotToExtent',
    'one = otherIs',
  ],
  predicates: [
    { name: 'negativeReference', args: ['matters'] },
  ],
  relations: [
    { predicate: 'is', from: 'matters', to: 'negativeReference' },
  ],
  candidateSummary: 'Matters are not things, lack negative self-subsistence. Properties are rather self-subsistent, determined with being reflected into itself. Matters are simple, referring only to themselves. But content is determinateness. Immanent reflection only form of content. Content not reflected-into-itself but refers to other according to determinateness. Thing not only "also" but equally negative reference. On account of determinateness, matters themselves negative reflection, which is puncticity of thing. One matter is not what other is, one is not to extent other is.',
  provenance: {
    sourceChunk: 'dis-3',
    sourceOp: 'dis-op-3-matters-negative-reference',
  },
};

export const disOp4ConnectingReference: LogicalOperation = {
  id: 'dis-op-4-connecting-reference',
  chunkId: 'dis-4',
  label: 'Connecting reference — interpenetration',
  clauses: [
    'thing = isConnectingReference',
    'thing = ofMattersToEachOther',
    'oneMatter = andOther',
    'oneMatter = alsoSubsistInIt',
    'yet = atSameTime',
    'oneMatter = doesNotSubsistInIt',
    'oneMatter = inSoFarAsOtherDoes',
    'toExtent = oneMatterIsInThing',
    'other = therebySublated',
    'but = thing',
    'but = atSameTimeAlso',
    'but = subsistenceOfOtherMatter',
    'inSubsistence = ofOneMatter',
    'other = doesNotSubsist',
    'other = andAlsoNoLess',
    'other = subsistsInIt',
    'so = withAllDiverseMatters',
    'so = inRespectToEachOther',
  ],
  predicates: [
    { name: 'interpenetration', args: ['thing'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'connectingReference' },
  ],
  candidateSummary: 'Thing is connecting reference of matters to each other. One matter and other also subsist in it. Yet at same time, one matter does not subsist in it in so far as other does. To extent one matter is in thing, other thereby sublated. But thing at same time "also," subsistence of other matter. In subsistence of one matter, other does not subsist, and also no less subsists in it. So with all diverse matters in respect to each other.',
  provenance: {
    sourceChunk: 'dis-4',
    sourceOp: 'dis-op-4-connecting-reference',
  },
};

export const disOp5PuncticityPorosity: LogicalOperation = {
  id: 'dis-op-5-puncticity-porosity',
  chunkId: 'dis-5',
  label: 'Puncticity — porosity',
  clauses: [
    'inSameRespect = asOneMatterSubsists',
    'other = subsistsAlso',
    'oneSubsistence = ofBoth',
    'oneSubsistence = isPuncticity',
    'oneSubsistence = orNegativeUnity',
    'oneSubsistence = ofThing',
    'two = interpenetrateAbsolutely',
    'thing = atSameTimeOnlyAlso',
    'thing = ofMatters',
    'these = reflectedIntoDeterminateness',
    'these = indifferentToOneAnother',
    'inInterpenetrating = doNotTouch',
    'matters = essentiallyPorous',
    'one = subsistsInPores',
    'one = orNonSubsistenceOfOthers',
    'others = themselvesPorous',
    'inTheirPores = orNonSubsistence',
    'first = andAllRest',
    'first = subsist',
    'subsistence = atSameTimeSublatedness',
    'subsistence = andSubsistenceOfOthers',
    'subsistence = ofOthers',
    'subsistence = justAsMuchSublatedness',
    'subsistence = andSubsisting',
    'subsistence = ofFirstAndAllOthers',
    'thing = isSelfContradictoryMediation',
    'thing = ofIndependentSelfSubsistence',
    'thing = throughOpposite',
    'thing = throughNegation',
    'thing = orOneSelfSubsistingMatter',
    'thing = throughSubsisting',
    'thing = andNonSubsisting',
    'thing = ofOther',
  ],
  predicates: [
    { name: 'porosity', args: ['matters'] },
  ],
  relations: [
    { predicate: 'is', from: 'oneSubsistence', to: 'puncticity' },
  ],
  candidateSummary: 'In same respect as one matter subsists, other subsists also. One subsistence of both is puncticity or negative unity of thing. Two interpenetrate absolutely. Thing at same time only "also" of matters. These reflected into determinateness, indifferent to one another. In interpenetrating do not touch. Matters essentially porous. One subsists in pores or non-subsistence of others. Others themselves porous. In their pores or non-subsistence, first and all rest subsist. Subsistence at same time sublatedness and subsistence of others. Subsistence of others just as much sublatedness and subsisting of first and all others. Thing is self-contradictory mediation of independent self-subsistence through opposite, through negation, or one self-subsisting matter through subsisting and non-subsisting of other.',
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
    'in = thisThing',
    'concreteExistence = hasAttainedCompletion',
    'atOnce = beingThatExistsInItself',
    'atOnce = independentSubsistence',
    'atOnce = andUnessentialConcreteExistence',
    'truth = ofConcreteExistence',
    'truth = hasItsInItself',
    'truth = inUnessentiality',
    'subsists = inOther',
    'subsists = indeedAbsoluteOther',
    'has = itsOwnNothingness',
    'has = forSubstrate',
    'it = isTherefore',
    'it = appearance',
  ],
  predicates: [
    { name: 'appearance', args: ['concreteExistence'] },
  ],
  relations: [
    { predicate: 'is', from: 'it', to: 'appearance' },
  ],
  candidateSummary: 'In "this" thing, concrete existence has attained completion. At once being that exists in itself, independent subsistence, and unessential concrete existence. Truth of concrete existence: has its in-itself in unessentiality. Subsists in other, indeed absolute other. Has its own nothingness for substrate. It is, therefore, appearance.',
  provenance: {
    sourceChunk: 'dis-6',
    sourceOp: 'dis-op-6-concrete-existence-appearance',
  },
};

export const dissolutionOperations: LogicalOperation[] = [
  disOp1ThingAbsolutelyAlterable,
  disOp2AbsolutelyDissolubleThing,
  disOp3MattersNegativeReference,
  disOp4ConnectingReference,
  disOp5PuncticityPorosity,
  disOp6ConcreteExistenceAppearance,
];

