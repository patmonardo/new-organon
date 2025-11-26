/**
 * Being-for-Self IR: Dialectic Pseudo-Code for Being-for-Itself
 *
 * Architecture: CPU Quality (Immediate Determination)
 * Section: I. BEING - A. QUALITY - C. Being-for-Self - A. Being-for-Itself as Such
 *
 * Covers the dialectical movement:
 * - Being-for-Itself: Sublates otherness, infinite turning back into itself
 * - Existence and Being-for-Itself: Infinity sunk into simple being
 * - Being-for-One: Idealization, unity with infinite
 * - The One: Simple unity, abstract limit
 *
 * The CPU's quality - being-for-itself, idealization, the one
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'being-for-self-a',
  title: 'Being-for-itself — sublates otherness, infinite turning back',
  concept: 'BeingForItself',
  phase: 'quality',

  moments: [
    {
      name: 'sublatesOtherness',
      definition: 'Something is for itself inasmuch as it sublates otherness, connection with other',
      type: 'sublation',
    },
    {
      name: 'infiniteTurningBack',
      definition: 'Being-for-itself is infinite turning back into itself, negation of negation',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'sublatesOtherness',
    },
    {
      name: 'idealization',
      definition: 'Content of object is idealization, self-consciousness is being-for-itself completed',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'being-for-self-a-inv-1',
      constraint: 'beingForItself = sublatesOtherness',
      predicate: 'sublates(beingForItself, otherness)',
    },
    {
      id: 'being-for-self-a-inv-2',
      constraint: 'beingForItself = infiniteTurningBack',
      predicate: 'equals(beingForItself, infiniteTurningBack)',
    },
  ],

  forces: [
    {
      id: 'being-for-self-a-force-1',
      description: 'Being-for-itself drives toward being-for-one',
      type: 'mediation',
      trigger: 'existence.bentBack = true',
      effect: 'beingForOne.emerges = true',
      targetState: 'being-for-self-b',
    },
  ],

  transitions: [
    {
      id: 'being-for-self-a-trans-1',
      from: 'being-for-self-a',
      to: 'being-for-self-b',
      mechanism: 'mediation',
      description: 'From being-for-itself to being-for-one',
    },
  ],

  nextStates: ['being-for-self-b'],
  previousStates: ['determinate-being-ir'],

  provenance: {
    topicMapId: 'being-for-self-a',
    lineRange: { start: 49, end: 127 },
    section: 'A. BEING-FOR-ITSELF AS SUCH',
    order: 1,
  },

  description: 'Being-for-itself sublates otherness, infinite turning back into itself. Infinity sunk into simple being. Existence bent back into infinite unity. Being-for-one emerges.',
};

const state2: DialecticState = {
  id: 'being-for-self-b',
  title: 'Being-for-one — idealization, unity with infinite',
  concept: 'BeingForOne',
  phase: 'quality',

  moments: [
    {
      name: 'idealization',
      definition: 'Finite in unity with infinite as idealization, being-for-one',
      type: 'determination',
    },
    {
      name: 'unityOfMoments',
      definition: 'Being-for-itself and being-for-one are essential, inseparable moments of ideality',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'idealization',
    },
    {
      name: 'selfReference',
      definition: 'Being-for-itself refers itself to itself as sublated other, is for-one',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'being-for-self-b-inv-1',
      constraint: 'beingForOne = idealization',
      predicate: 'equals(beingForOne, idealization)',
    },
    {
      id: 'being-for-self-b-inv-2',
      constraint: 'beingForItself = beingForOne',
      predicate: 'inseparable(beingForItself, beingForOne)',
    },
  ],

  forces: [
    {
      id: 'being-for-self-b-force-1',
      description: 'Being-for-one drives toward the one',
      type: 'mediation',
      trigger: 'moments.sunkIntoIndifferentiation = true',
      effect: 'theOne.emerges = true',
      targetState: 'being-for-self-c',
    },
  ],

  transitions: [
    {
      id: 'being-for-self-b-trans-1',
      from: 'being-for-self-b',
      to: 'being-for-self-c',
      mechanism: 'mediation',
      description: 'From being-for-one to the one',
    },
  ],

  nextStates: ['being-for-self-c'],
  previousStates: ['being-for-self-a'],

  provenance: {
    topicMapId: 'being-for-self-b',
    lineRange: { start: 129, end: 192 },
    section: 'b. Being-for-one',
    order: 2,
  },

  description: 'Being-for-one - idealization, finite in unity with infinite. Being-for-itself and being-for-one are inseparable moments. I, spirit, God are idealizations.',
};

const state3: DialecticState = {
  id: 'being-for-self-c',
  title: 'The one — simple unity, abstract limit',
  concept: 'TheOne',
  phase: 'quality',

  moments: [
    {
      name: 'simpleUnity',
      definition: 'Being-for-itself is simple unity of itself and its moments',
      type: 'determination',
    },
    {
      name: 'indifferentiation',
      definition: 'Moments sunk into indifferentiation which is immediacy or being',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'simpleUnity',
    },
    {
      name: 'abstractLimit',
      definition: 'Totally abstract limit of itself: the one',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'being-for-self-c-inv-1',
      constraint: 'one = simpleUnity',
      predicate: 'equals(one, simpleUnity)',
    },
    {
      id: 'being-for-self-c-inv-2',
      constraint: 'one = abstractLimit',
      predicate: 'equals(one, abstractLimit)',
    },
  ],

  forces: [
    {
      id: 'being-for-self-c-force-1',
      description: 'The one drives toward the many',
      type: 'negation',
      trigger: 'moments.occurApart = true',
      effect: 'oneAndMany.emerges = true',
      targetState: 'one-many-b',
    },
  ],

  transitions: [
    {
      id: 'being-for-self-c-trans-1',
      from: 'being-for-self-c',
      to: 'one-many-b',
      mechanism: 'negation',
      description: 'From the one to the one and the many',
    },
  ],

  nextStates: ['one-many-b'],
  previousStates: ['being-for-self-b'],

  provenance: {
    topicMapId: 'being-for-self-c',
    lineRange: { start: 194, end: 231 },
    section: 'c. The one',
    order: 3,
  },

  description: 'The one - simple unity, abstract limit. Moments occur apart: negation, two negations, self-reference, negative reference. Contradiction causes difficulty.',
};

export const beingForSelfIR: DialecticIR = {
  id: 'being-for-self-ir',
  title: 'Being-for-Self IR: Being-for-Itself, Being-for-One, The One',
  section: 'I. BEING - A. QUALITY - C. Being-for-Self - A. Being-for-Itself as Such',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'being-for-self.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'being-for-self-a': 'quality',
      'being-for-self-b': 'quality',
      'being-for-self-c': 'quality',
    },
  },
};

export const beingForSelfStates = {
  'being-for-self-a': state1,
  'being-for-self-b': state2,
  'being-for-self-c': state3,
};
