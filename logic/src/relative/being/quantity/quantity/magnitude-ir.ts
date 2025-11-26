/**
 * Magnitude IR: Dialectic Pseudo-Code for Continuous and Discrete Magnitude
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: B. CONTINUOUS AND DISCRETE MAGNITUDE
 *
 * Covers the dialectical movement:
 * - Quantity contains both moments (continuity and discreteness)
 * - Continuous magnitude as immediate quantity
 * - Discrete magnitude as discontinuous outsideness-of-one-another
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'quantity-continuous-discrete-intro',
  title: 'Continuous and Discrete Magnitude: two moments of quantity',
  concept: 'ContinuousDiscreteMoments',
  phase: 'quantity',

  moments: [
    {
      name: 'continuity',
      definition: 'Compact unity holding discrete together',
      type: 'moment',
    },
    {
      name: 'discreteness',
      definition: 'Other moment, brought to completion',
      type: 'moment',
      relation: 'opposite',
      relatedTo: 'continuity',
    },
    {
      name: 'continuousMagnitude',
      definition: 'Whole quantity posited as continuity',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'magnitude-1-inv-1',
      constraint: 'quantity.contains = {continuity, discreteness}',
      predicate: 'contains(quantity, continuity) ∧ contains(quantity, discreteness)',
    },
    {
      id: 'magnitude-1-inv-2',
      constraint: 'continuousMagnitude = wholequantity',
      predicate: 'equals(continuousMagnitude, wholeQuantity)',
    },
    {
      id: 'magnitude-1-inv-3',
      constraint: 'continuity.holdsTogetherDiscrete = true',
      predicate: 'holdsTogether(continuity, discrete)',
    },
  ],

  forces: [
    {
      id: 'magnitude-1-force-1',
      description: 'Posited continuity drives toward continuous magnitude',
      type: 'passover',
      trigger: 'continuity.posited = true',
      effect: 'continuousMagnitude.emerges = true',
      targetState: 'quantity-continuous-magnitude',
    },
  ],

  transitions: [
    {
      id: 'magnitude-1-trans-1',
      from: 'quantity-continuous-discrete-intro',
      to: 'quantity-continuous-magnitude',
      mechanism: 'passover',
      description: 'From two moments to continuous magnitude',
    },
  ],

  nextStates: ['quantity-continuous-magnitude'],
  previousStates: ['quantity-pure-unity'],

  provenance: {
    topicMapId: 'quantity-continuous-discrete-intro',
    lineRange: { start: 3, end: 23 },
    section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE',
    order: 1,
  },

  description: 'Continuous/Discrete Magnitude: Quantity contains both moments. Posited at first as continuity (continuous magnitude). Continuity is compact unity holding discrete together; posited as such, it is whole quantity, not just moment.',
};

const state2: DialecticState = {
  id: 'quantity-continuous-magnitude',
  title: 'Continuous Magnitude: immediate quantity',
  concept: 'ContinuousMagnitude',
  phase: 'quantity',

  moments: [
    {
      name: 'immediateQuantity',
      definition: 'Continuous magnitude as immediate',
      type: 'determination',
    },
    {
      name: 'sublatedImmediacy',
      definition: 'Immediacy as determinateness is sublated',
      type: 'sublation',
      relation: 'negates',
      relatedTo: 'immediateQuantity',
    },
    {
      name: 'immanentDeterminateness',
      definition: 'The one, as immanent determinateness',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'magnitude-2-inv-1',
      constraint: 'immediateQuantity = continuousMagnitude',
      predicate: 'equals(immediateQuantity, continuousMagnitude)',
    },
    {
      id: 'magnitude-2-inv-2',
      constraint: 'quantity.isImmediate = false',
      predicate: 'not(isImmediate(quantity))',
    },
    {
      id: 'magnitude-2-inv-3',
      constraint: 'immediacy.sublated = true',
      predicate: 'sublated(immediacy)',
    },
    {
      id: 'magnitude-2-inv-4',
      constraint: 'quantity.mustBePosited = immanentDeterminateness',
      predicate: 'mustBePosited(quantity, immanentDeterminateness)',
    },
  ],

  forces: [
    {
      id: 'magnitude-2-force-1',
      description: 'Sublation of immediacy drives toward discrete magnitude',
      type: 'negation',
      trigger: 'immediacy.sublated = true',
      effect: 'discreteMagnitude.required = true',
      targetState: 'quantity-discrete-magnitude',
    },
  ],

  transitions: [
    {
      id: 'magnitude-2-trans-1',
      from: 'quantity-continuous-magnitude',
      to: 'quantity-discrete-magnitude',
      mechanism: 'negation',
      description: 'From continuous to discrete magnitude via sublation',
    },
  ],

  nextStates: ['quantity-discrete-magnitude'],
  previousStates: ['quantity-continuous-discrete-intro'],

  provenance: {
    topicMapId: 'quantity-continuous-magnitude',
    lineRange: { start: 25, end: 32 },
    section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE',
    order: 2,
  },

  description: 'Continuous Magnitude: Immediate quantity is continuous magnitude. But quantity is not immediate; immediacy is sublated. Quantity must be posited in immanent determinateness (the one) - discrete magnitude.',
};

const state3: DialecticState = {
  id: 'quantity-discrete-magnitude',
  title: 'Discrete Magnitude: outsideness-of-one-another as discontinuous',
  concept: 'DiscreteMagnitude',
  phase: 'quantity',

  moments: [
    {
      name: 'discreteness',
      definition: 'Moment and whole of quantity',
      type: 'moment',
    },
    {
      name: 'discontinuous',
      definition: 'Outsideness-of-one-another broken off',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'discreteness',
    },
    {
      name: 'continuous',
      definition: 'Discreteness is itself continuous (ones same unity)',
      type: 'quality',
      relation: 'mediates',
      relatedTo: 'discreteness',
    },
    {
      name: 'manyOnesOfUnity',
      definition: 'Not many ones in general, but many of a unity',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'magnitude-3-inv-1',
      constraint: 'discreteness = momentAndWhole(quantity)',
      predicate: 'momentAndWhole(discreteness, quantity)',
    },
    {
      id: 'magnitude-3-inv-2',
      constraint: 'discreteness.continuous = true',
      predicate: 'continuous(discreteness)',
    },
    {
      id: 'magnitude-3-inv-3',
      constraint: 'ones.haveSameUnity = true',
      predicate: 'haveSameUnity(ones)',
    },
    {
      id: 'magnitude-3-inv-4',
      constraint: 'discreteMagnitude = manyOnesOfUnity',
      predicate: 'equals(discreteMagnitude, manyOnesOfUnity)',
    },
  ],

  forces: [
    {
      id: 'magnitude-3-force-1',
      description: 'Many ones of unity prepares quantum as determined magnitude',
      type: 'passover',
      trigger: 'manyOnesOfUnity.posited = true',
      effect: 'quantum.emerges = true',
      targetState: 'quantum-1',
    },
  ],

  transitions: [
    {
      id: 'magnitude-3-trans-1',
      from: 'quantity-discrete-magnitude',
      to: 'quantum-1',
      mechanism: 'passover',
      description: 'From discrete magnitude to quantum',
    },
  ],

  nextStates: ['quantum-1'],
  previousStates: ['quantity-continuous-magnitude'],

  provenance: {
    topicMapId: 'quantity-discrete-magnitude',
    lineRange: { start: 34, end: 57 },
    section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE',
    order: 3,
  },

  description: 'Discrete Magnitude: Moment and whole of quantity. Outsideness-of-one-another as discontinuous/broken off. But discreteness is itself continuous (ones are same, have same unity). Discrete magnitude is many ones of a unity, not many ones in general.',
};

export const magnitudeIR: DialecticIR = {
  id: 'magnitude-ir',
  title: 'Magnitude IR: Continuous and Discrete Magnitude',
  section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'continuous-discrete-magnitude.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'quantity-continuous-discrete-intro': 'quantity',
      'quantity-continuous-magnitude': 'quantity',
      'quantity-discrete-magnitude': 'quantity',
    },
  },
};

export const magnitudeStates = {
  'quantity-continuous-discrete-intro': state1,
  'quantity-continuous-magnitude': state2,
  'quantity-discrete-magnitude': state3,
};
