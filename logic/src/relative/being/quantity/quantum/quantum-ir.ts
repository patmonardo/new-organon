/**
 * Quantum IR: Dialectic Pseudo-Code for Extensive and Intensive Quantum
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: C. QUANTUM - Extensive and Intensive Quantum
 *
 * Covers the dialectical movement:
 * - Extensive magnitude as limit in plurality (amount)
 * - Number's determinateness as self-referring limit
 * - Intensive magnitude (degree) as simple determinateness
 * - Identity of extensive and intensive magnitude
 * - Quantum must alter, sends itself beyond to infinity
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'quantum-extensive-magnitude',
  title: 'Extensive magnitude: limit as plurality',
  concept: 'ExtensiveMagnitude',
  phase: 'quantity',

  moments: [
    {
      name: 'extensiveMagnitude',
      definition: 'Quantum has determinateness as limit in amount (plurality)',
      type: 'determination',
    },
    {
      name: 'amount',
      definition: 'Essentially amount of one unit',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'extensiveMagnitude',
    },
    {
      name: 'plurality',
      definition: 'Limit as many, concentrated into one',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'quantum-1-inv-1',
      constraint: 'quantum.determinateness = limitInAmount',
      predicate: 'determinateness(quantum, limitInAmount)',
    },
    {
      id: 'quantum-1-inv-2',
      constraint: 'extensiveMagnitude.amount = essentially(oneUnit)',
      predicate: 'essentially(extensiveMagnitude.amount, oneUnit)',
    },
    {
      id: 'quantum-1-inv-3',
      constraint: 'extensiveMagnitude ≈ number (differ only in explicit plurality)',
      predicate: 'similar(extensiveMagnitude, number, explicitPlurality)',
    },
  ],

  forces: [
    {
      id: 'quantum-1-force-1',
      description: 'Limit concentrated into one drives toward number as self-referring limit',
      type: 'mediation',
      trigger: 'many.concentratedIntoOne = true',
      effect: 'numberDeterminateness.emerges = true',
      targetState: 'quantum-number-determinateness',
    },
  ],

  transitions: [
    {
      id: 'quantum-1-trans-1',
      from: 'quantum-extensive-magnitude',
      to: 'quantum-number-determinateness',
      mechanism: 'mediation',
      description: 'From extensive magnitude to number determinateness',
    },
  ],

  nextStates: ['quantum-number-determinateness'],
  previousStates: ['quantity-discrete-magnitude'],

  provenance: {
    topicMapId: 'quantum-extensive-magnitude',
    lineRange: { start: 6, end: 54 },
    section: 'a. Their difference',
    order: 1,
  },

  description: 'Extensive magnitude: Quantum has determinateness as limit in amount (plurality). Extensive quantum is amount essentially (amount of one unit), distinguished from number only by explicit plurality.',
};

const state2: DialecticState = {
  id: 'quantum-number-determinateness',
  title: 'Number\'s determinateness: self-referring limit',
  concept: 'NumberDeterminateness',
  phase: 'quantity',

  moments: [
    {
      name: 'selfReferringLimit',
      definition: 'Limit determined for itself, indifferent and self-referring',
      type: 'determination',
    },
    {
      name: 'enclosedExternality',
      definition: 'Externality/reference to other is inside number',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'selfReferringLimit',
    },
    {
      name: 'collapsedMany',
      definition: 'Many collapses into continuity, simple unity',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'quantum-2-inv-1',
      constraint: 'limit.determinedForItself = true',
      predicate: 'determinedForItself(limit)',
    },
    {
      id: 'quantum-2-inv-2',
      constraint: 'externality ∈ number (inside)',
      predicate: 'inside(externality, number)',
    },
    {
      id: 'quantum-2-inv-3',
      constraint: 'many.collapses = continuity',
      predicate: 'collapses(many, continuity)',
    },
    {
      id: 'quantum-2-inv-4',
      constraint: 'ones.sublated = number.selfReference',
      predicate: 'sublated(ones, number.selfReference)',
    },
  ],

  forces: [
    {
      id: 'quantum-2-force-1',
      description: 'Self-reference drives toward intensive magnitude',
      type: 'passover',
      trigger: 'externalityTurnedBackIntoItself = true',
      effect: 'intensiveMagnitude.emerges = true',
      targetState: 'quantum-intensive-magnitude-degree',
    },
  ],

  transitions: [
    {
      id: 'quantum-2-trans-1',
      from: 'quantum-number-determinateness',
      to: 'quantum-intensive-magnitude-degree',
      mechanism: 'passover',
      description: 'From number self-reference to intensive magnitude (degree)',
    },
  ],

  nextStates: ['quantum-intensive-magnitude-degree'],
  previousStates: ['quantum-extensive-magnitude'],

  provenance: {
    topicMapId: 'quantum-number-determinateness',
    lineRange: { start: 56, end: 86 },
    section: 'a. Their difference',
    order: 2,
  },

  description: 'Number\'s determinateness: Limit is determined for itself, indifferent and self-referring. In number, limit is enclosed within one - externality/reference to other is inside it. Many collapses into continuity, becomes simple unity.',
};

const state3: DialecticState = {
  id: 'quantum-intensive-magnitude-degree',
  title: 'Intensive magnitude: degree as simple determinateness',
  concept: 'IntensiveMagnitude',
  phase: 'quantity',

  moments: [
    {
      name: 'degree',
      definition: 'Quantum but not aggregate - simple determinateness',
      type: 'determination',
    },
    {
      name: 'simpleDeterminateness',
      definition: 'Plurality gathered into simple determination',
      type: 'sublation',
      relation: 'mediates',
      relatedTo: 'degree',
    },
    {
      name: 'beingForItself',
      definition: 'Existence returned into being-for-itself',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'quantum-3-inv-1',
      constraint: 'degree = quantum ∧ not(aggregate)',
      predicate: 'is(degree, quantum) ∧ not(aggregate(degree))',
    },
    {
      id: 'quantum-3-inv-2',
      constraint: 'plurality.gatheredInto = simpleDetermination',
      predicate: 'gatheredInto(plurality, simpleDetermination)',
    },
    {
      id: 'quantum-3-inv-3',
      constraint: 'existence.returnedInto = beingForItself',
      predicate: 'returnedInto(existence, beingForItself)',
    },
  ],

  forces: [
    {
      id: 'quantum-3-force-1',
      description: 'Degree externality drives toward identity with extensive',
      type: 'contradiction',
      trigger: 'degree.hasExternality = true',
      effect: 'identityEmerges = extensiveIntensive',
      targetState: 'quantum-identity-two-sides',
    },
  ],

  transitions: [
    {
      id: 'quantum-3-trans-1',
      from: 'quantum-intensive-magnitude-degree',
      to: 'quantum-identity-two-sides',
      mechanism: 'reflection',
      description: 'From degree to identity of extensive and intensive',
    },
  ],

  nextStates: ['quantum-identity-two-sides'],
  previousStates: ['quantum-number-determinateness'],

  provenance: {
    topicMapId: 'quantum-intensive-magnitude-degree',
    lineRange: { start: 88, end: 117 },
    section: 'a. Their difference',
    order: 3,
  },

  description: 'Intensive magnitude (degree): Limit passes over into simple determinateness. Degree is quantum but not aggregate - plurality gathered into simple determination, existence returned into being-for-itself.',
};

const state4: DialecticState = {
  id: 'quantum-identity-two-sides',
  title: 'Identity: extensive and intensive are same determinateness',
  concept: 'ExtensiveIntensiveIdentity',
  phase: 'quantity',

  moments: [
    {
      name: 'identity',
      definition: 'Extensive/intensive are same determinateness',
      type: 'determination',
    },
    {
      name: 'amountWithinWithout',
      definition: 'Distinguished only by amount within vs without',
      type: 'determination',
      relation: 'mediates',
      relatedTo: 'identity',
    },
    {
      name: 'qualitativeSomething',
      definition: 'Unity through negation of distinct terms',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'quantum-4-inv-1',
      constraint: 'extensiveMagnitude.determinateness = intensiveMagnitude.determinateness',
      predicate: 'equals(extensive.determinateness, intensive.determinateness)',
    },
    {
      id: 'quantum-4-inv-2',
      constraint: 'distinction = amountWithin ∨ amountWithout',
      predicate: 'distinction(amountWithin, amountWithout)',
    },
    {
      id: 'quantum-4-inv-3',
      constraint: 'qualitativeSomething = unityThroughNegation',
      predicate: 'equals(qualitativeSomething, unityThroughNegation)',
    },
  ],

  forces: [
    {
      id: 'quantum-4-force-1',
      description: 'Identity as indifferent limit drives quantum to alter',
      type: 'contradiction',
      trigger: 'quantum.indifferentLimit = true',
      effect: 'alteration.necessary = true',
      targetState: 'quantum-alteration-necessity',
    },
  ],

  transitions: [
    {
      id: 'quantum-4-trans-1',
      from: 'quantum-identity-two-sides',
      to: 'quantum-alteration-necessity',
      mechanism: 'contradiction',
      description: 'From identity to necessary alteration of quantum',
    },
  ],

  nextStates: ['quantum-alteration-necessity'],
  previousStates: ['quantum-intensive-magnitude-degree'],

  provenance: {
    topicMapId: 'quantum-identity-two-sides',
    lineRange: { start: 199, end: 254 },
    section: 'b. Identity of extensive and intensive magnitude',
    order: 6,
  },

  description: 'Identity: Extensive/intensive are same determinateness. Distinguished only by amount within vs without. With this identity, qualitative something appears - unity through negation of distinct terms.',
};

const state5: DialecticState = {
  id: 'quantum-alteration-necessity',
  title: 'Alteration of quantum: quantum must alter',
  concept: 'QuantumAlteration',
  phase: 'quantity',

  moments: [
    {
      name: 'indifferentLimit',
      definition: 'Determinateness posited as sublated',
      type: 'negation',
    },
    {
      name: 'absoluteContinuity',
      definition: 'Quantum in continuity with externality/otherness',
      type: 'mediation',
    },
    {
      name: 'becoming',
      definition: 'Limit becomes, not just exists',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'quantum-5-inv-1',
      constraint: 'quantum.determinateness = sublated',
      predicate: 'sublated(quantum.determinateness)',
    },
    {
      id: 'quantum-5-inv-2',
      constraint: 'quantum.inAbsoluteContinuity = externality',
      predicate: 'inAbsoluteContinuity(quantum, externality)',
    },
    {
      id: 'quantum-5-inv-3',
      constraint: 'quantum.mustAlter = true',
      predicate: 'mustAlter(quantum)',
    },
    {
      id: 'quantum-5-inv-4',
      constraint: 'limit.becomes = true (not just exists)',
      predicate: 'becomes(limit) ∧ not(justExists(limit))',
    },
  ],

  forces: [
    {
      id: 'quantum-5-force-1',
      description: 'Self-referring negation drives quantum beyond itself to infinity',
      type: 'passover',
      trigger: 'quantum.selfReferringNegation = true',
      effect: 'infinityEmerges = true',
      targetState: 'ratio-1',
    },
  ],

  transitions: [
    {
      id: 'quantum-5-trans-1',
      from: 'quantum-alteration-necessity',
      to: 'ratio-1',
      mechanism: 'passover',
      description: 'Quantum sends itself beyond to infinity (ratio)',
    },
  ],

  nextStates: ['ratio-1'],
  previousStates: ['quantum-identity-two-sides'],

  provenance: {
    topicMapId: 'quantum-alteration-necessity',
    lineRange: { start: 256, end: 281 },
    section: 'c. Alteration of quantum',
    order: 7,
  },

  description: 'Alteration: Quantum is determinateness posited as sublated - indifferent limit that is negation of itself. Quantum is in absolute continuity with externality/otherness. It must alter - has being only in continuity with other.',
};

export const quantumIR: DialecticIR = {
  id: 'quantum-ir',
  title: 'Quantum IR: Extensive and Intensive Quantum',
  section: 'C. QUANTUM - Extensive and Intensive',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'quantum.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'quantum-extensive-magnitude': 'quantity',
      'quantum-number-determinateness': 'quantity',
      'quantum-intensive-magnitude-degree': 'quantity',
      'quantum-identity-two-sides': 'quantity',
      'quantum-alteration-necessity': 'quantity',
    },
  },
};

export const quantumStates = {
  'quantum-extensive-magnitude': state1,
  'quantum-number-determinateness': state2,
  'quantum-intensive-magnitude-degree': state3,
  'quantum-identity-two-sides': state4,
  'quantum-alteration-necessity': state5,
};
