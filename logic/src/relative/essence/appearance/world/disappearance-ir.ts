/**
 * Disappearance IR: Dialectic Pseudo-Code for Dissolution of Appearance
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 2. The World - c. Dissolution of Appearance
 *
 * Covers: Two worlds in opposition, difference disappears, law realized,
 * world foundered, essential relation emerges
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'disa-1',
  title: 'Two worlds — opposition and inversion',
  concept: 'TwoWorldsOpposition',
  phase: 'appearance',

  moments: [
    {
      name: 'twoWorlds',
      definition: 'World-in-itself and world of appearance in complete opposition',
      type: 'determination',
    },
    {
      name: 'inversion',
      definition: 'What is positive in appearance is negative in world-in-itself',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'twoWorlds',
    },
  ],

  invariants: [
    {
      id: 'disa-1-inv-1',
      constraint: 'positive(appearance) = negative(worldInItself)',
      predicate: 'equals(positive(appearance), negative(worldInItself))',
    },
  ],

  forces: [
    {
      id: 'disa-1-force-1',
      description: 'Opposition drives toward disappearance of difference',
      type: 'negation',
      trigger: 'opposition.complete = true',
      effect: 'differenceDisappears = true',
      targetState: 'disa-5',
    },
  ],

  transitions: [
    {
      id: 'disa-1-trans-1',
      from: 'disa-1',
      to: 'disa-5',
      mechanism: 'negation',
      description: 'From opposition to identity of moments',
    },
  ],

  nextStates: ['disa-5'],
  previousStates: ['law-ir'],

  provenance: {
    topicMapId: 'disa-1',
    lineRange: { start: 4, end: 20 },
    section: 'c. Dissolution of Appearance',
    order: 1,
  },

  description: 'Two worlds in complete opposition. What is positive in appearance is negative in world-in-itself.',
};

const state2: DialecticState = {
  id: 'disa-5',
  title: 'Two totalities — identity of moments',
  concept: 'IdentityOfMoments',
  phase: 'appearance',

  moments: [
    {
      name: 'twoTotalities',
      definition: 'Each is totality, continues into other',
      type: 'determination',
    },
    {
      name: 'identityOfMoments',
      definition: 'Identity of reflected and immediate moments',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'twoTotalities',
    },
  ],

  invariants: [
    {
      id: 'disa-5-inv-1',
      constraint: 'selfSubsistence = unity(two)',
      predicate: 'equals(selfSubsistence, unity(two))',
    },
  ],

  forces: [
    {
      id: 'disa-5-force-1',
      description: 'Identity of moments drives toward law realized',
      type: 'mediation',
      trigger: 'identityOfMoments.established = true',
      effect: 'lawRealized = true',
      targetState: 'disa-6',
    },
  ],

  transitions: [
    {
      id: 'disa-5-trans-1',
      from: 'disa-5',
      to: 'disa-6',
      mechanism: 'mediation',
      description: 'From identity of moments to law realized',
    },
  ],

  nextStates: ['disa-6'],
  previousStates: ['disa-1'],

  provenance: {
    topicMapId: 'disa-5',
    lineRange: { start: 56, end: 75 },
    section: 'c. Dissolution of Appearance',
    order: 2,
  },

  description: 'Each is totality, continues into other. Self-subsistence in unity of two.',
};

const state3: DialecticState = {
  id: 'disa-6',
  title: 'Law realized — essential relation',
  concept: 'LawRealized',
  phase: 'appearance',

  moments: [
    {
      name: 'lawRealized',
      definition: 'Inner identity existent, content raised to ideality',
      type: 'determination',
    },
    {
      name: 'essentialRelation',
      definition: 'Law is essential relation',
      type: 'mediation',
      relation: 'transforms',
      relatedTo: 'lawRealized',
    },
  ],

  invariants: [
    {
      id: 'disa-6-inv-1',
      constraint: 'law = essentialRelation',
      predicate: 'equals(law, essentialRelation)',
    },
  ],

  forces: [
    {
      id: 'disa-6-force-1',
      description: 'Law realized drives toward world foundered',
      type: 'sublation',
      trigger: 'law.realized = true',
      effect: 'worldFoundered = true',
      targetState: 'disa-7',
    },
  ],

  transitions: [
    {
      id: 'disa-6-trans-1',
      from: 'disa-6',
      to: 'disa-7',
      mechanism: 'sublation',
      description: 'From law realized to world foundered',
    },
  ],

  nextStates: ['disa-7'],
  previousStates: ['disa-5'],

  provenance: {
    topicMapId: 'disa-6',
    lineRange: { start: 77, end: 99 },
    section: 'c. Dissolution of Appearance',
    order: 3,
  },

  description: 'Law realized. Inner identity existent. Content raised to ideality. Law is essential relation.',
};

const state4: DialecticState = {
  id: 'disa-7',
  title: 'World foundered — essential relation',
  concept: 'WorldFoundered',
  phase: 'appearance',

  moments: [
    {
      name: 'worldFoundered',
      definition: 'World has foundered both as essential world and world of appearance',
      type: 'sublation',
    },
    {
      name: 'essentialRelation',
      definition: 'Still totality but as essential relation',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'worldFoundered',
    },
  ],

  invariants: [
    {
      id: 'disa-7-inv-1',
      constraint: 'world.foundered = true',
      predicate: 'foundered(world)',
    },
  ],

  forces: [
    {
      id: 'disa-7-force-1',
      description: 'Essential relation drives toward Relation (Whole/Parts)',
      type: 'passover',
      trigger: 'essentialRelation.established = true',
      effect: 'relation.emerges = true',
      targetState: 'wlp-1',
    },
  ],

  transitions: [
    {
      id: 'disa-7-trans-1',
      from: 'disa-7',
      to: 'wlp-1',
      mechanism: 'passover',
      description: 'From world foundered to essential relation (Whole/Parts)',
    },
  ],

  nextStates: ['wlp-1'],
  previousStates: ['disa-6'],

  provenance: {
    topicMapId: 'disa-7',
    lineRange: { start: 101, end: 123 },
    section: 'c. Dissolution of Appearance',
    order: 4,
  },

  description: 'World has foundered. Still totality but as essential relation. Essential relation is consummation of unity of form.',
};

export const disappearanceIR: DialecticIR = {
  id: 'disappearance-ir',
  title: 'Disappearance IR: Opposition, Law Realized, World Foundered',
  section: 'B. APPEARANCE - 2. The World - c. Dissolution of Appearance',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'disappearance.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'disa-1': 'appearance',
      'disa-5': 'appearance',
      'disa-6': 'appearance',
      'disa-7': 'appearance',
    },
  },
};

export const disappearanceStates = {
  'disa-1': state1,
  'disa-5': state2,
  'disa-6': state3,
  'disa-7': state4,
};
