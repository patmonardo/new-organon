/**
 * Limiting Quantity IR: Dialectic Pseudo-Code for The Limiting of Quantity
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: C. THE LIMITING OF QUANTITY
 *
 * Covers the dialectical movement:
 * - Discrete magnitude as quantum (one as determinateness/limit)
 * - Encompassing limit (enclosing/encompassing, negative point)
 * - Quantum as limit encompassing many ones, passing over
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'quantity-limiting-intro',
  title: 'Limiting of Quantity: discrete magnitude as quantum',
  concept: 'LimitingQuantity',
  phase: 'quantity',

  moments: [
    {
      name: 'discreteMagnitude',
      definition: 'One as principle, plurality of ones, essentially continuous',
      type: 'determination',
    },
    {
      name: 'oneMagnitude',
      definition: 'Posited as one magnitude; one is determinateness/limit',
      type: 'determination',
      relation: 'mediates',
      relatedTo: 'discreteMagnitude',
    },
    {
      name: 'quantum',
      definition: 'Existence/something with one as limit',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'discreteMagnitude',
    },
  ],

  invariants: [
    {
      id: 'limiting-1-inv-1',
      constraint: 'discreteMagnitude.principle = one',
      predicate: 'principle(discreteMagnitude, one)',
    },
    {
      id: 'limiting-1-inv-2',
      constraint: 'discreteMagnitude.essentiallyContinuous = true',
      predicate: 'essentiallyContinuous(discreteMagnitude)',
    },
    {
      id: 'limiting-1-inv-3',
      constraint: 'one.sublatedAs = unity',
      predicate: 'sublatedAs(one, unity)',
    },
    {
      id: 'limiting-1-inv-4',
      constraint: 'quantum = existence(something, oneAsLimit)',
      predicate: 'existence(quantum, something, oneAsLimit)',
    },
  ],

  forces: [
    {
      id: 'limiting-1-force-1',
      description: 'One as determinateness drives toward encompassing limit',
      type: 'mediation',
      trigger: 'one.posited = determinateExistence',
      effect: 'encompassingLimit.emerges = true',
      targetState: 'quantity-limiting-encompassing',
    },
  ],

  transitions: [
    {
      id: 'limiting-1-trans-1',
      from: 'quantity-limiting-intro',
      to: 'quantity-limiting-encompassing',
      mechanism: 'mediation',
      description: 'From discrete magnitude to encompassing limit',
    },
  ],

  nextStates: ['quantity-limiting-encompassing'],
  previousStates: ['quantity-discrete-magnitude'],

  provenance: {
    topicMapId: 'quantity-limiting-intro',
    lineRange: { start: 3, end: 21 },
    section: 'C. THE LIMITING OF QUANTITY',
    order: 1,
  },

  description: 'Limiting of Quantity: Discrete magnitude has one as principle, plurality of ones, essentially continuous (one sublated as unity). Posited as one magnitude; "one" is determinateness/limit. Distinguished from continuous magnitude, it is existence/something with "one" as limit.',
};

const state2: DialecticState = {
  id: 'quantity-limiting-encompassing',
  title: 'Encompassing limit: one as enclosing limit',
  concept: 'EncompassingLimit',
  phase: 'quantity',

  moments: [
    {
      name: 'encompassingLimit',
      definition: 'Limit as enclosing/encompassing, self-referred as one',
      type: 'determination',
    },
    {
      name: 'negativePoint',
      definition: 'Limit as negative point itself (not distinct from something)',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'encompassingLimit',
    },
    {
      name: 'continuityTranscending',
      definition: 'Limited being is continuity, transcending limit',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'limiting-2-inv-1',
      constraint: 'limit.referToUnity = negationMoment',
      predicate: 'referTo(limit, unity, negationMoment)',
    },
    {
      id: 'limiting-2-inv-2',
      constraint: 'limit.selfReferred = one',
      predicate: 'selfReferred(limit, one)',
    },
    {
      id: 'limiting-2-inv-3',
      constraint: 'limit = negativePointItself',
      predicate: 'is(limit, negativePointItself)',
    },
    {
      id: 'limiting-2-inv-4',
      constraint: 'limitedBeing.transcendsLimit = true',
      predicate: 'transcends(limitedBeing, limit)',
    },
  ],

  forces: [
    {
      id: 'limiting-2-force-1',
      description: 'Continuity transcending limit drives toward quantum as many ones',
      type: 'passover',
      trigger: 'continuity.transcendsLimit = true',
      effect: 'quantumAsManyOnes.emerges = true',
      targetState: 'quantity-limiting-quantum',
    },
  ],

  transitions: [
    {
      id: 'limiting-2-trans-1',
      from: 'quantity-limiting-encompassing',
      to: 'quantity-limiting-quantum',
      mechanism: 'passover',
      description: 'From encompassing limit to quantum encompassing many ones',
    },
  ],

  nextStates: ['quantity-limiting-quantum'],
  previousStates: ['quantity-limiting-intro'],

  provenance: {
    topicMapId: 'quantity-limiting-encompassing',
    lineRange: { start: 23, end: 34 },
    section: 'C. THE LIMITING OF QUANTITY',
    order: 2,
  },

  description: 'Encompassing Limit: Limit refers to unity (negation moment) and is self-referred as one - enclosing/encompassing. Limit is negative point itself (not distinct from something). But limited being is continuity, transcending limit, indifferent to it. Discrete quantity is quantum: quantity as existence/something.',
};

const state3: DialecticState = {
  id: 'quantity-limiting-quantum',
  title: 'Quantum: limit encompassing many ones, passing over',
  concept: 'QuantumEncompassing',
  phase: 'quantity',

  moments: [
    {
      name: 'quantumAsEncompassing',
      definition: 'One as limit encompasses many ones (sublated in it)',
      type: 'determination',
    },
    {
      name: 'limitToContinuity',
      definition: 'Limit to continuity; distinction becomes indifferent',
      type: 'mediation',
    },
    {
      name: 'passingOver',
      definition: 'Both pass over into quanta',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'limitToContinuity',
    },
  ],

  invariants: [
    {
      id: 'limiting-3-inv-1',
      constraint: 'oneAsLimit.encompasses = manyOnes',
      predicate: 'encompasses(oneAsLimit, manyOnes)',
    },
    {
      id: 'limiting-3-inv-2',
      constraint: 'manyOnes.sublatedIn = limit',
      predicate: 'sublatedIn(manyOnes, limit)',
    },
    {
      id: 'limiting-3-inv-3',
      constraint: 'distinction(continuous, discrete).indifferent = true',
      predicate: 'indifferent(distinction(continuous, discrete))',
    },
    {
      id: 'limiting-3-inv-4',
      constraint: 'one ∧ other → quanta',
      predicate: 'passOver(one, quanta) ∧ passOver(other, quanta)',
    },
  ],

  forces: [
    {
      id: 'limiting-3-force-1',
      description: 'Passing over into quanta prepares for number as quantum',
      type: 'passover',
      trigger: 'distinction.becomesIndifferent = true',
      effect: 'number.emerges = true',
      targetState: 'number-quantum-as-number',
    },
  ],

  transitions: [
    {
      id: 'limiting-3-trans-1',
      from: 'quantity-limiting-quantum',
      to: 'number-quantum-as-number',
      mechanism: 'passover',
      description: 'From quantum to number as complete positedness',
    },
  ],

  nextStates: ['number-quantum-as-number'],
  previousStates: ['quantity-limiting-encompassing'],

  provenance: {
    topicMapId: 'quantity-limiting-quantum',
    lineRange: { start: 36, end: 45 },
    section: 'C. THE LIMITING OF QUANTITY',
    order: 3,
  },

  description: 'Quantum: One as limit encompasses many ones (sublated in it). Limit to continuity; distinction continuous/discrete becomes indifferent. Limit to continuity of one and other; both pass over into quanta.',
};

export const limitingQuantityIR: DialecticIR = {
  id: 'limiting-quantity-ir',
  title: 'Limiting Quantity IR: Discrete Magnitude as Quantum',
  section: 'C. THE LIMITING OF QUANTITY',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'limiting-quantity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'quantity-limiting-intro': 'quantity',
      'quantity-limiting-encompassing': 'quantity',
      'quantity-limiting-quantum': 'quantity',
    },
  },
};

export const limitingQuantityStates = {
  'quantity-limiting-intro': state1,
  'quantity-limiting-encompassing': state2,
  'quantity-limiting-quantum': state3,
};
