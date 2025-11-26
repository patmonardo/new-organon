/**
 * World IR: Dialectic Pseudo-Code for World
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 2. The World - a. The World-In-Itself
 *
 * Covers the dialectical movement:
 * - World raises itself to kingdom of laws
 * - Law as negative reflection
 * - World-in-itself as suprasensible world
 * - World splits into world-in-itself and world of appearance
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'wld-1',
  title: 'World raises itself to kingdom of laws',
  concept: 'KingdomOfLaws',
  phase: 'appearance',

  moments: [
    {
      name: 'kingdomOfLaws',
      definition: 'Concrete existing world raises itself to kingdom of laws',
      type: 'determination',
    },
    {
      name: 'lawAsSimpleIdentity',
      definition: 'Law is simple identity of appearance with itself',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'kingdomOfLaws',
    },
    {
      name: 'substrateNotGround',
      definition: 'Law is substrate and not ground, not negative unity',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'wld-1-inv-1',
      constraint: 'law = simpleIdentity(appearance)',
      predicate: 'equals(law, simpleIdentity(appearance))',
    },
    {
      id: 'wld-1-inv-2',
      constraint: 'law = substrate ∧ law ≠ ground',
      predicate: 'and(equals(law, substrate), not(equals(law, ground)))',
    },
    {
      id: 'wld-1-inv-3',
      constraint: 'appearingThings.haveGrounds = true',
      predicate: 'haveGrounds(appearingThings)',
    },
  ],

  forces: [
    {
      id: 'wld-1-force-1',
      description: 'Law as substrate drives toward law as negative reflection',
      type: 'mediation',
      trigger: 'law.substrate = true',
      effect: 'negativeReflection.emerges = true',
      targetState: 'wld-4',
    },
  ],

  transitions: [
    {
      id: 'wld-1-trans-1',
      from: 'wld-1',
      to: 'wld-4',
      mechanism: 'mediation',
      description: 'From kingdom of laws to law as negative unity',
    },
  ],

  nextStates: ['wld-4'],
  previousStates: ['dissolution-ir'],

  provenance: {
    topicMapId: 'wld-1',
    lineRange: { start: 4, end: 23 },
    section: 'a. World-In-Itself',
    order: 1,
  },

  description: 'Concrete existing world tranquilly raises itself to kingdom of laws. Law is simple identity of appearance with itself. Substrate and not ground. Appearing things have grounds and conditions in other appearing things.',
};

const state2: DialecticState = {
  id: 'wld-4',
  title: 'Law obtains negative moment — world-in-itself',
  concept: 'WorldInItself',
  phase: 'appearance',

  moments: [
    {
      name: 'negativeForm',
      definition: 'Law obtains missing moment of negative form of sides',
      type: 'negation',
    },
    {
      name: 'essentialTotality',
      definition: 'Law is essential totality of appearance',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'negativeForm',
    },
    {
      name: 'worldInItself',
      definition: 'World that discloses itself above world of appearance, in-and-for-itself',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'wld-4-inv-1',
      constraint: 'law.hasNegativeForm = true',
      predicate: 'has(law, negativeForm)',
    },
    {
      id: 'wld-4-inv-2',
      constraint: 'law = essentialTotality',
      predicate: 'equals(law, essentialTotality)',
    },
    {
      id: 'wld-4-inv-3',
      constraint: 'content.connected = totalizing',
      predicate: 'is(content.connected, totalizing)',
    },
  ],

  forces: [
    {
      id: 'wld-4-force-1',
      description: 'World-in-itself drives toward suprasensible world',
      type: 'mediation',
      trigger: 'worldInItself.disclosed = true',
      effect: 'suprasensibleWorld.emerges = true',
      targetState: 'wld-5',
    },
  ],

  transitions: [
    {
      id: 'wld-4-trans-1',
      from: 'wld-4',
      to: 'wld-5',
      mechanism: 'mediation',
      description: 'From world-in-itself to suprasensible world',
    },
  ],

  nextStates: ['wld-5'],
  previousStates: ['wld-1'],

  provenance: {
    topicMapId: 'wld-4',
    lineRange: { start: 79, end: 104 },
    section: 'a. World-In-Itself',
    order: 2,
  },

  description: 'Law obtains missing moment of negative form. Essential totality of appearance. Content every determinateness essentially connected. World-in-itself discloses itself above world of appearance.',
};

const state3: DialecticState = {
  id: 'wld-5',
  title: 'Kingdom of laws — suprasensible world',
  concept: 'SuprasensibleWorld',
  phase: 'appearance',

  moments: [
    {
      name: 'suprasensibleWorld',
      definition: 'World-in-itself called suprasensible world, reflected concrete existence',
      type: 'determination',
    },
    {
      name: 'sensibleWorld',
      definition: 'Concretely existing world characterized as sensible, for intuition',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'suprasensibleWorld',
    },
    {
      name: 'trueConcreteExistence',
      definition: 'Things of suprasensible world are true concrete existences',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'wld-5-inv-1',
      constraint: 'worldInItself = suprasensibleWorld',
      predicate: 'equals(worldInItself, suprasensibleWorld)',
    },
    {
      id: 'wld-5-inv-2',
      constraint: 'sensibleWorld.immediate = true',
      predicate: 'isImmediate(sensibleWorld)',
    },
    {
      id: 'wld-5-inv-3',
      constraint: 'suprasensibleWorld.reflected = true',
      predicate: 'isReflected(suprasensibleWorld)',
    },
  ],

  forces: [
    {
      id: 'wld-5-force-1',
      description: 'Suprasensible world drives toward totality and splitting',
      type: 'negation',
      trigger: 'suprasensibleWorld.established = true',
      effect: 'splitting.emerges = true',
      targetState: 'wld-7',
    },
  ],

  transitions: [
    {
      id: 'wld-5-trans-1',
      from: 'wld-5',
      to: 'wld-7',
      mechanism: 'negation',
      description: 'From suprasensible world to splitting/opposition',
    },
  ],

  nextStates: ['wld-7'],
  previousStates: ['wld-4'],

  provenance: {
    topicMapId: 'wld-5',
    lineRange: { start: 106, end: 155 },
    section: 'a. World-In-Itself',
    order: 3,
  },

  description: 'Kingdom of laws contains simple, unchanging but diversified content. World-in-itself called suprasensible world. Sensible world for intuition, suprasensible world reflected concrete existence. Things of suprasensible world are true concrete existences.',
};

const state4: DialecticState = {
  id: 'wld-7',
  title: 'Determinate ground — opposition',
  concept: 'WorldSplitsIntoOpposition',
  phase: 'appearance',

  moments: [
    {
      name: 'splitting',
      definition: 'Kingdom of laws splits into world-in-itself and world of appearance',
      type: 'negation',
    },
    {
      name: 'essentialConnection',
      definition: 'Identity is essential connection of opposition',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'splitting',
    },
    {
      name: 'becomingAndTransition',
      definition: 'Identity essentially becoming and transition',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'wld-7-inv-1',
      constraint: 'kingdomOfLaws.splits = true',
      predicate: 'splits(kingdomOfLaws)',
    },
    {
      id: 'wld-7-inv-2',
      constraint: 'identity = essentialConnection(opposition)',
      predicate: 'equals(identity, essentialConnection(opposition))',
    },
    {
      id: 'wld-7-inv-3',
      constraint: 'identity = becoming',
      predicate: 'is(identity, becoming)',
    },
  ],

  forces: [
    {
      id: 'wld-7-force-1',
      description: 'Opposition drives toward law of appearance',
      type: 'passover',
      trigger: 'opposition.established = true',
      effect: 'lawOfAppearance.emerges = true',
      targetState: 'law-1',
    },
  ],

  transitions: [
    {
      id: 'wld-7-trans-1',
      from: 'wld-7',
      to: 'law-1',
      mechanism: 'passover',
      description: 'From opposition to law of appearance',
    },
  ],

  nextStates: ['law-1'],
  previousStates: ['wld-5'],

  provenance: {
    topicMapId: 'wld-7',
    lineRange: { start: 183, end: 240 },
    section: 'a. World-In-Itself',
    order: 4,
  },

  description: 'Kingdom of laws splits into world-in-itself and world of appearance. Identity is essential connection of opposition. Identity essentially becoming and transition.',
};

export const worldIR: DialecticIR = {
  id: 'world-ir',
  title: 'World IR: Kingdom of Laws, Suprasensible World, Opposition',
  section: 'B. APPEARANCE - 2. The World - a. World-In-Itself',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'world.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'wld-1': 'appearance',
      'wld-4': 'appearance',
      'wld-5': 'appearance',
      'wld-7': 'appearance',
    },
  },
};

export const worldStates = {
  'wld-1': state1,
  'wld-4': state2,
  'wld-5': state3,
  'wld-7': state4,
};
