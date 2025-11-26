/**
 * Law IR: Dialectic Pseudo-Code for Law of Appearance
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 2. The World - b. The Law of Appearance
 *
 * Covers: Appearance as essence in concrete existence, law as positive element,
 * law and appearance have same content, kingdom of laws
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'law-1',
  title: 'Appearance — essence in concrete existence',
  concept: 'AppearanceAsEssence',
  phase: 'appearance',

  moments: [
    {
      name: 'appearance',
      definition: 'Appearance is essence in its concrete existence',
      type: 'determination',
    },
    {
      name: 'reflectedImmediacy',
      definition: 'Concrete existence is reflected immediacy, absolute negativity within',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'appearance',
    },
  ],

  invariants: [
    {
      id: 'law-1-inv-1',
      constraint: 'appearance = essence(inConcreteExistence)',
      predicate: 'equals(appearance, essence(inConcreteExistence))',
    },
  ],

  forces: [
    {
      id: 'law-1-force-1',
      description: 'Appearance drives toward law as positive element',
      type: 'mediation',
      trigger: 'appearance.established = true',
      effect: 'law.emerges = true',
      targetState: 'law-9',
    },
  ],

  transitions: [
    {
      id: 'law-1-trans-1',
      from: 'law-1',
      to: 'law-9',
      mechanism: 'mediation',
      description: 'From appearance to law',
    },
  ],

  nextStates: ['law-9'],
  previousStates: ['world-ir'],

  provenance: {
    topicMapId: 'law-1',
    lineRange: { start: 6, end: 24 },
    section: 'b. Law of Appearance',
    order: 1,
  },

  description: 'Appearance is essence in its concrete existence. Concrete existence is reflected immediacy, absolute negativity within.',
};

const state2: DialecticState = {
  id: 'law-9',
  title: 'Law — positive element',
  concept: 'LawAsPositiveElement',
  phase: 'appearance',

  moments: [
    {
      name: 'lawAsPositive',
      definition: 'Law is positive element of mediation of what appears',
      type: 'determination',
    },
    {
      name: 'persistence',
      definition: 'Law is persistence, identical positive positedness',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'lawAsPositive',
    },
  ],

  invariants: [
    {
      id: 'law-9-inv-1',
      constraint: 'law = positiveElement(mediation)',
      predicate: 'equals(law, positiveElement(mediation))',
    },
  ],

  forces: [
    {
      id: 'law-9-force-1',
      description: 'Law drives toward kingdom of laws',
      type: 'mediation',
      trigger: 'law.positive = true',
      effect: 'kingdomOfLaws.emerges = true',
      targetState: 'law-13',
    },
  ],

  transitions: [
    {
      id: 'law-9-trans-1',
      from: 'law-9',
      to: 'law-13',
      mechanism: 'mediation',
      description: 'From law to kingdom of laws',
    },
  ],

  nextStates: ['law-13'],
  previousStates: ['law-1'],

  provenance: {
    topicMapId: 'law-9',
    lineRange: { start: 232, end: 250 },
    section: 'b. Law of Appearance',
    order: 2,
  },

  description: 'Law is positive element of mediation. Contains reflective shining and persistence.',
};

const state3: DialecticState = {
  id: 'law-13',
  title: 'Law — kingdom of laws',
  concept: 'KingdomOfLaws',
  phase: 'appearance',

  moments: [
    {
      name: 'kingdomOfLaws',
      definition: 'Kingdom of laws is restful copy of appearing world',
      type: 'determination',
    },
    {
      name: 'oneTotality',
      definition: 'Two are one totality, concretely existing world is kingdom of laws',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'kingdomOfLaws',
    },
  ],

  invariants: [
    {
      id: 'law-13-inv-1',
      constraint: 'kingdomOfLaws = restfulCopy',
      predicate: 'equals(kingdomOfLaws, restfulCopy)',
    },
  ],

  forces: [
    {
      id: 'law-13-force-1',
      description: 'Kingdom of laws drives toward disappearance',
      type: 'passover',
      trigger: 'kingdomOfLaws.established = true',
      effect: 'disappearance.emerges = true',
      targetState: 'disa-1',
    },
  ],

  transitions: [
    {
      id: 'law-13-trans-1',
      from: 'law-13',
      to: 'disa-1',
      mechanism: 'passover',
      description: 'From kingdom of laws to disappearance',
    },
  ],

  nextStates: ['disa-1'],
  previousStates: ['law-9'],

  provenance: {
    topicMapId: 'law-13',
    lineRange: { start: 331, end: 343 },
    section: 'b. Law of Appearance',
    order: 3,
  },

  description: 'Kingdom of laws is restful copy. Concretely existing world is kingdom of laws. Law is essentiality.',
};

export const lawIR: DialecticIR = {
  id: 'law-ir',
  title: 'Law IR: Appearance, Law, Kingdom of Laws',
  section: 'B. APPEARANCE - 2. The World - b. Law of Appearance',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'law.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'law-1': 'appearance',
      'law-9': 'appearance',
      'law-13': 'appearance',
    },
  },
};

export const lawStates = {
  'law-1': state1,
  'law-9': state2,
  'law-13': state3,
};
