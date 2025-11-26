/**
 * Outer-Inner IR: Dialectic Pseudo-Code for Outer and Inner
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 3. Essential Relation - b. Outer and Inner
 *
 * Covers: Inner and outer as one identity, immediate conversion, actuality
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'oin-2',
  title: 'Inner and outer — one identity',
  concept: 'OneIdentity',
  phase: 'appearance',

  moments: [
    {
      name: 'oneIdentity',
      definition: 'Inner and outer only one identity, absolute fact',
      type: 'determination',
    },
    {
      name: 'substrateReplete',
      definition: 'Substrate replete of content',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'oneIdentity',
    },
  ],

  invariants: [
    {
      id: 'oin-2-inv-1',
      constraint: 'inner = outer',
      predicate: 'equals(inner, outer)',
    },
  ],

  forces: [
    {
      id: 'oin-2-force-1',
      description: 'One identity drives toward immediate conversion',
      type: 'mediation',
      trigger: 'oneIdentity.established = true',
      effect: 'immediateConversion.emerges = true',
      targetState: 'oin-3',
    },
  ],

  transitions: [
    {
      id: 'oin-2-trans-1',
      from: 'oin-2',
      to: 'oin-3',
      mechanism: 'mediation',
      description: 'From one identity to immediate conversion',
    },
  ],

  nextStates: ['oin-3'],
  previousStates: ['force-expression-ir'],

  provenance: {
    topicMapId: 'oin-2',
    lineRange: { start: 33, end: 66 },
    section: 'b. Outer and Inner',
    order: 1,
  },

  description: 'Inner and outer only one identity. Substrate replete of content, absolute fact.',
};

const state2: DialecticState = {
  id: 'oin-3',
  title: 'Form determinations — immediate conversion',
  concept: 'ImmediateConversion',
  phase: 'appearance',

  moments: [
    {
      name: 'immediateConversion',
      definition: 'One immediately other, inner immediately outer',
      type: 'mediation',
    },
    {
      name: 'negativeUnity',
      definition: 'Negative unity is simple point empty of content',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'immediateConversion',
    },
  ],

  invariants: [
    {
      id: 'oin-3-inv-1',
      constraint: 'inner.onlyInner → outer',
      predicate: 'implies(onlyInner(inner), outer)',
    },
  ],

  forces: [
    {
      id: 'oin-3-force-1',
      description: 'Immediate conversion drives toward actuality',
      type: 'mediation',
      trigger: 'immediateConversion.established = true',
      effect: 'actuality.emerges = true',
      targetState: 'oin-5',
    },
  ],

  transitions: [
    {
      id: 'oin-3-trans-1',
      from: 'oin-3',
      to: 'oin-5',
      mechanism: 'mediation',
      description: 'From immediate conversion to actuality',
    },
  ],

  nextStates: ['oin-5'],
  previousStates: ['oin-2'],

  provenance: {
    topicMapId: 'oin-3',
    lineRange: { start: 68, end: 155 },
    section: 'b. Outer and Inner',
    order: 2,
  },

  description: 'One immediately other. Something only inner, for that reason only outer. Negative unity is simple point empty of content.',
};

const state3: DialecticState = {
  id: 'oin-5',
  title: 'Each as totality — actuality',
  concept: 'Actuality',
  phase: 'appearance',

  moments: [
    {
      name: 'actuality',
      definition: 'Essence consists simply in being self-revealing',
      type: 'determination',
    },
    {
      name: 'contentFormIdentical',
      definition: 'Content and form absolutely identical',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'actuality',
    },
  ],

  invariants: [
    {
      id: 'oin-5-inv-1',
      constraint: 'externality = expression(inItself)',
      predicate: 'equals(externality, expression(inItself))',
    },
  ],

  forces: [
    {
      id: 'oin-5-force-1',
      description: 'Actuality transitions to Actuality (next phase)',
      type: 'passover',
      trigger: 'actuality.established = true',
      effect: 'actualityPhase.emerges = true',
      targetState: 'actuality-1',
    },
  ],

  transitions: [
    {
      id: 'oin-5-trans-1',
      from: 'oin-5',
      to: 'actuality-1',
      mechanism: 'passover',
      description: 'From actuality to Actuality phase',
    },
  ],

  nextStates: ['actuality-1'],
  previousStates: ['oin-3'],

  provenance: {
    topicMapId: 'oin-5',
    lineRange: { start: 183, end: 231 },
    section: 'b. Outer and Inner',
    order: 3,
  },

  description: 'Externality is expression of what it is in itself. Content and form absolutely identical. Essence consists simply in being self-revealing. Has determined itself as actuality.',
};

export const outerInnerIR: DialecticIR = {
  id: 'outer-inner-ir',
  title: 'Outer-Inner IR: One Identity, Immediate Conversion, Actuality',
  section: 'B. APPEARANCE - 3. Essential Relation - b. Outer and Inner',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'outer-inner.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'oin-2': 'appearance',
      'oin-3': 'appearance',
      'oin-5': 'appearance',
    },
  },
};

export const outerInnerStates = {
  'oin-2': state1,
  'oin-3': state2,
  'oin-5': state3,
};
