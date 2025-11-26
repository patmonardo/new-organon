/**
 * Ratio IR: Dialectic Pseudo-Code for Ratio (Quantitative Relation)
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: D. THE QUANTITATIVE RATIO
 *
 * Covers the dialectical movement:
 * - Infinite quantum as unity of quantitative/qualitative (ratio)
 * - Direct ratio (qualitative moment not yet explicit)
 * - Exponent as one determinateness of both quanta
 * - Incompleteness driving transition to inverse ratio
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'ratio-intro-qualitative-moment',
  title: 'Ratio: quantum qualit atively determined',
  concept: 'QuantitativeRelation',
  phase: 'quantity',

  moments: [
    {
      name: 'infiniteQuantum',
      definition: 'Unity of quantitative and qualitative determinateness',
      type: 'determination',
    },
    {
      name: 'ratio',
      definition: 'Quantum qualitatively determined, referring to beyond',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'infiniteQuantum',
    },
    {
      name: 'selfEnclosedTotality',
      definition: 'Quantum as relation, infinite within',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'ratio-1-inv-1',
      constraint: 'infiniteQuantum = unity(quantitative, qualitative)',
      predicate: 'unity(infiniteQuantum, quantitative, qualitative)',
    },
    {
      id: 'ratio-1-inv-2',
      constraint: 'quantum.qualitativelyDetermined = true',
      predicate: 'qualitativelyDetermined(quantum)',
    },
    {
      id: 'ratio-1-inv-3',
      constraint: 'quantum.refersToBeyond = true',
      predicate: 'refersToBeyond(quantum)',
    },
    {
      id: 'ratio-1-inv-4',
      constraint: 'ratio = selfEnclosedTotality',
      predicate: 'is(ratio, selfEnclosedTotality)',
    },
  ],

  forces: [
    {
      id: 'ratio-1-force-1',
      description: 'Self-enclosed totality drives toward direct ratio',
      type: 'passover',
      trigger: 'quantumAsRelation = true',
      effect: 'directRatio.emerges = true',
      targetState: 'ratio-direct-intro',
    },
  ],

  transitions: [
    {
      id: 'ratio-1-trans-1',
      from: 'ratio-intro-qualitative-moment',
      to: 'ratio-direct-intro',
      mechanism: 'passover',
      description: 'From infinite quantum to direct ratio',
    },
  ],

  nextStates: ['ratio-direct-intro'],
  previousStates: ['quantum-alteration-necessity'],

  provenance: {
    topicMapId: 'ratio-intro-qualitative-moment',
    lineRange: { start: 6, end: 50 },
    section: 'Introduction',
    order: 1,
  },

  description: 'Ratio: Infinite quantum is unity of quantitative/qualitative determinateness. Quantum qualitatively determined, referring to beyond. Quantum is itself posited as relation - self-enclosed totality, infinite within.',
};

const state2: DialecticState = {
  id: 'ratio-direct-intro',
  title: 'Direct ratio: qualitative moment not yet explicit',
  concept: 'DirectRatio',
  phase: 'quantity',

  moments: [
    {
      name: 'directRatio',
      definition: 'Qualitative moment not yet explicit',
      type: 'determination',
    },
    {
      name: 'contradiction',
      definition: 'Externality and self-reference in quantitative relation',
      type: 'negation',
    },
    {
      name: 'sublation',
      definition: 'Contradiction sublates toward inverse ratio and ratio of powers',
      type: 'sublation',
      relation: 'transforms',
      relatedTo: 'contradiction',
    },
  ],

  invariants: [
    {
      id: 'ratio-2-inv-1',
      constraint: 'qualitativeMoment.explicit = false',
      predicate: 'not(explicit(qualitativeMoment))',
    },
    {
      id: 'ratio-2-inv-2',
      constraint: 'quantitativeRelation = contradiction(externality, selfReference)',
      predicate: 'contradiction(quantitativeRelation, externality, selfReference)',
    },
    {
      id: 'ratio-2-inv-3',
      constraint: 'contradiction.sublates = true',
      predicate: 'sublates(contradiction)',
    },
  ],

  forces: [
    {
      id: 'ratio-2-force-1',
      description: 'Quantitative relation requires exponent',
      type: 'mediation',
      trigger: 'determinatenessInExternality = true',
      effect: 'exponent.required = true',
      targetState: 'ratio-direct-exponent',
    },
  ],

  transitions: [
    {
      id: 'ratio-2-trans-1',
      from: 'ratio-direct-intro',
      to: 'ratio-direct-exponent',
      mechanism: 'mediation',
      description: 'From direct ratio to exponent as determinateness',
    },
  ],

  nextStates: ['ratio-direct-exponent'],
  previousStates: ['ratio-intro-qualitative-moment'],

  provenance: {
    topicMapId: 'ratio-direct-intro',
    lineRange: { start: 52, end: 74 },
    section: 'Ratio in general',
    order: 2,
  },

  description: 'Direct ratio: Qualitative moment not yet explicit. Quantum posited as having determinateness in externality. Quantitative relation is contradiction of externality/self-reference. Contradiction sublates toward inverse ratio and ratio of powers.',
};

const state3: DialecticState = {
  id: 'ratio-direct-exponent',
  title: 'Exponent: one determinateness of both quanta',
  concept: 'Exponent',
  phase: 'quantity',

  moments: [
    {
      name: 'exponent',
      definition: 'One determinateness/limit of both quanta',
      type: 'determination',
    },
    {
      name: 'qualitativelyDetermined',
      definition: 'Quantum but qualitatively determined',
      type: 'quality',
      relation: 'contains',
      relatedTo: 'exponent',
    },
    {
      name: 'unitAndAmount',
      definition: 'Difference as unit and amount',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'ratio-3-inv-1',
      constraint: 'exponent = oneDeterminateness(both)',
      predicate: 'oneDeterminateness(exponent, both)',
    },
    {
      id: 'ratio-3-inv-2',
      constraint: 'exponent.qualitativelyDetermined = true',
      predicate: 'qualitativelyDetermined(exponent)',
    },
    {
      id: 'ratio-3-inv-3',
      constraint: 'difference = {unit, amount}',
      predicate: 'equals(difference, pair(unit, amount))',
    },
    {
      id: 'ratio-3-inv-4',
      constraint: 'unit = beingDeterminedForItself',
      predicate: 'is(unit, beingDeterminedForItself)',
    },
  ],

  forces: [
    {
      id: 'ratio-3-force-1',
      description: 'Incompleteness of sides drives toward inverse ratio',
      type: 'negation',
      trigger: 'sides.incomplete = true',
      effect: 'inverseRatio.emerges = true',
      targetState: 'ratio-direct-incompleteness',
    },
  ],

  transitions: [
    {
      id: 'ratio-3-trans-1',
      from: 'ratio-direct-exponent',
      to: 'ratio-direct-incompleteness',
      mechanism: 'negation',
      description: 'From exponent to incompleteness of sides',
    },
  ],

  nextStates: ['ratio-direct-incompleteness'],
  previousStates: ['ratio-direct-intro'],

  provenance: {
    topicMapId: 'ratio-direct-exponent-qualitative',
    lineRange: { start: 92, end: 106 },
    section: 'A. THE DIRECT RATIO',
    order: 4,
  },

  description: 'Exponent: Some quantum, but qualitatively determined (difference/beyond/otherness in it). Difference is unit and amount. In ratio, each moment appears as quantum on its own - delimitations.',
};

const state4: DialecticState = {
  id: 'ratio-direct-incompleteness',
  title: 'Incompleteness: each side only one moment',
  concept: 'IncompleteSides',
  phase: 'quantity',

  moments: [
    {
      name: 'incompleteness',
      definition: 'Two sides constitute one quantum but each only one moment',
      type: 'negation',
    },
    {
      name: 'determinedNegation',
      definition: 'Not general variability but determined alteration',
      type: 'negation',
      relation: 'negates',
      relatedTo: 'self-subsistence',
    },
    {
      name: 'qualitativeCombination',
      definition: 'Sides negative with respect to each other',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'ratio-4-inv-1',
      constraint: 'sides.constitute = oneQuantum',
      predicate: 'constitute(sides, oneQuantum)',
    },
    {
      id: 'ratio-4-inv-2',
      constraint: 'each.isCompleteQuanta = false',
      predicate: 'not(isCompleteQuanta(each))',
    },
    {
      id: 'ratio-4-inv-3',
      constraint: 'negation = determined (not general)',
      predicate: 'determined(negation) ∧ not(general(negation))',
    },
    {
      id: 'ratio-4-inv-4',
      constraint: 'selfSubsistence.negated = true',
      predicate: 'negated(selfSubsistence)',
    },
  ],

  forces: [
    {
      id: 'ratio-4-force-1',
      description: 'Incompleteness drives exponent to become product (inverse ratio)',
      type: 'sublation',
      trigger: 'negation.qualifying = true',
      effect: 'inverseRatio.emerges = true',
      targetState: 'measure-1',
    },
  ],

  transitions: [
    {
      id: 'ratio-4-trans-1',
      from: 'ratio-direct-incompleteness',
      to: 'measure-1',
      mechanism: 'sublation',
      description: 'From incomplete direct ratio to measure',
    },
  ],

  nextStates: ['measure-1'],
  previousStates: ['ratio-direct-exponent'],

  provenance: {
    topicMapId: 'ratio-direct-transition-inverse',
    lineRange: { start: 157, end: 191 },
    section: 'A. THE DIRECT RATIO',
    order: 7,
  },

  description: 'Incompleteness: Two constitute one quantum. Each side only one moment - self-subsistence negated, negative with respect to each other. Exponent ought to be complete quantum but as quotient only value of amount or unit. More real ratio arises: exponent as product - inverse ratio.',
};

export const ratioIR: DialecticIR = {
  id: 'ratio-ir',
  title: 'Ratio IR: Quantitative Relation',
  section: 'D. THE QUANTITATIVE RATIO',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'ratio.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'ratio-intro-qualitative-moment': 'quantity',
      'ratio-direct-intro': 'quantity',
      'ratio-direct-exponent': 'quantity',
      'ratio-direct-incompleteness': 'quantity',
    },
  },
};

export const ratioStates = {
  'ratio-intro-qualitative-moment': state1,
  'ratio-direct-intro': state2,
  'ratio-direct-exponent': state3,
  'ratio-direct-incompleteness': state4,
};
