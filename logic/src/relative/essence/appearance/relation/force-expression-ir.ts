/**
 * Force-Expression IR: Dialectic Pseudo-Code for Force and Expression
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 3. Essential Relation - c. Force and Expression
 *
 * Covers: Force as negative unity, solicitation, infinity of force
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'fex-1',
  title: 'Force — truth of whole and parts',
  concept: 'ForceAsNegativeUnity',
  phase: 'appearance',

  moments: [
    {
      name: 'force',
      definition: 'Force is negative unity, truth of whole and parts',
      type: 'determination',
    },
    {
      name: 'immanentTurningBack',
      definition: 'Higher immanent turning back',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'force',
    },
  ],

  invariants: [
    {
      id: 'fex-1-inv-1',
      constraint: 'force = negativeUnity',
      predicate: 'equals(force, negativeUnity)',
    },
  ],

  forces: [
    {
      id: 'fex-1-force-1',
      description: 'Force drives toward solicitation',
      type: 'mediation',
      trigger: 'force.established = true',
      effect: 'solicitation.emerges = true',
      targetState: 'fex-9',
    },
  ],

  transitions: [
    {
      id: 'fex-1-trans-1',
      from: 'fex-1',
      to: 'fex-9',
      mechanism: 'mediation',
      description: 'From force to solicitation',
    },
  ],

  nextStates: ['fex-9'],
  previousStates: ['whole-parts-ir'],

  provenance: {
    topicMapId: 'fex-1',
    lineRange: { start: 4, end: 17 },
    section: 'c. Force and Expression',
    order: 1,
  },

  description: 'Force is negative unity, truth of whole and parts. Higher immanent turning back.',
};

const state2: DialecticState = {
  id: 'fex-9',
  title: 'Solicitation — stimulus',
  concept: 'Solicitation',
  phase: 'appearance',

  moments: [
    {
      name: 'solicitation',
      definition: 'Force reciprocally stimulus for other force',
      type: 'mediation',
    },
    {
      name: 'repelling',
      definition: 'Repelling of itself from itself is own positing',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'solicitation',
    },
  ],

  invariants: [
    {
      id: 'fex-9-inv-1',
      constraint: 'force.stimulus = otherForce',
      predicate: 'equals(force.stimulus, otherForce)',
    },
  ],

  forces: [
    {
      id: 'fex-9-force-1',
      description: 'Solicitation drives toward infinity of force',
      type: 'mediation',
      trigger: 'solicitation.established = true',
      effect: 'infinity.emerges = true',
      targetState: 'fex-12',
    },
  ],

  transitions: [
    {
      id: 'fex-9-trans-1',
      from: 'fex-9',
      to: 'fex-12',
      mechanism: 'mediation',
      description: 'From solicitation to infinity',
    },
  ],

  nextStates: ['fex-12'],
  previousStates: ['fex-1'],

  provenance: {
    topicMapId: 'fex-9',
    lineRange: { start: 211, end: 237 },
    section: 'c. Force and Expression',
    order: 2,
  },

  description: 'Force reciprocally stimulus for other force. Repelling of itself from itself is own positing.',
};

const state3: DialecticState = {
  id: 'fex-12',
  title: 'The infinity of force — externality identical with inwardness',
  concept: 'InfinityOfForce',
  phase: 'appearance',

  moments: [
    {
      name: 'infinityOfForce',
      definition: 'Externality is identical with inwardness',
      type: 'determination',
    },
    {
      name: 'passivityIsActivity',
      definition: 'Passivity consists in activity',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'infinityOfForce',
    },
  ],

  invariants: [
    {
      id: 'fex-12-inv-1',
      constraint: 'externality = inwardness',
      predicate: 'equals(externality, inwardness)',
    },
  ],

  forces: [
    {
      id: 'fex-12-force-1',
      description: 'Infinity of force drives toward outer-inner',
      type: 'passover',
      trigger: 'infinity.established = true',
      effect: 'outerInner.emerges = true',
      targetState: 'oin-2',
    },
  ],

  transitions: [
    {
      id: 'fex-12-trans-1',
      from: 'fex-12',
      to: 'oin-2',
      mechanism: 'passover',
      description: 'From infinity of force to outer-inner',
    },
  ],

  nextStates: ['oin-2'],
  previousStates: ['fex-9'],

  provenance: {
    topicMapId: 'fex-12',
    lineRange: { start: 344, end: 369 },
    section: 'c. Force and Expression',
    order: 3,
  },

  description: 'Reference to other is reference to itself. Passivity consists in activity. Externality is identical with inwardness.',
};

export const forceExpressionIR: DialecticIR = {
  id: 'force-expression-ir',
  title: 'Force-Expression IR: Force, Solicitation, Infinity',
  section: 'B. APPEARANCE - 3. Essential Relation - c. Force and Expression',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'force-expression.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'fex-1': 'appearance',
      'fex-9': 'appearance',
      'fex-12': 'appearance',
    },
  },
};

export const forceExpressionStates = {
  'fex-1': state1,
  'fex-9': state2,
  'fex-12': state3,
};
