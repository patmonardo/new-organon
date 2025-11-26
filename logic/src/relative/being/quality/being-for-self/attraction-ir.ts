/**
 * Repulsion and Attraction IR: Dialectic Pseudo-Code for Repulsion and Attraction
 *
 * Architecture: CPU Quality (Immediate Determination)
 * Section: I. BEING - A. QUALITY - C. Being-for-Self - C. Repulsion and Attraction
 *
 * Covers the dialectical movement:
 * - Exclusion of the One: Mutual repulsion, self-preservation, dissolution
 * - The One One of Attraction: Ideality realized, mediated one
 * - Connection of Repulsion and Attraction: Inseparable unity, self-presupposing
 *
 * The CPU's quality - repulsion, attraction, transition to quantity
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'attraction-a',
  title: 'Exclusion of the one — mutual repulsion, self-preservation, dissolution',
  concept: 'ExclusionOfTheOne',
  phase: 'quality',

  moments: [
    {
      name: 'mutualRepulsion',
      definition: 'Mutual repulsion is posited existence of many ones, their own distinguishing',
      type: 'negation',
    },
    {
      name: 'selfPreservation',
      definition: 'Self-preservation through mediation of mutual repulsion, sublate reciprocally',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'mutualRepulsion',
    },
    {
      name: 'dissolution',
      definition: 'Self-preservation through negative reference is rather their dissolution',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'attraction-a-inv-1',
      constraint: 'mutualRepulsion = positedExistence',
      predicate: 'equals(mutualRepulsion, positedExistence)',
    },
    {
      id: 'attraction-a-inv-2',
      constraint: 'selfPreservation = dissolution',
      predicate: 'equals(selfPreservation, dissolution)',
    },
  ],

  forces: [
    {
      id: 'attraction-a-force-1',
      description: 'Exclusion drives toward attraction',
      type: 'mediation',
      trigger: 'repelling.crossesOverIntoIdentity = true',
      effect: 'attraction.emerges = true',
      targetState: 'attraction-b',
    },
  ],

  transitions: [
    {
      id: 'attraction-a-trans-1',
      from: 'attraction-a',
      to: 'attraction-b',
      mechanism: 'mediation',
      description: 'From exclusion to attraction',
    },
  ],

  nextStates: ['attraction-b'],
  previousStates: ['one-many-ir'],

  provenance: {
    topicMapId: 'attraction-a-2',
    lineRange: { start: 65, end: 136 },
    section: 'C. a. Exclusion of the one',
    order: 1,
  },

  description: 'Exclusion of the one - mutual repulsion, self-preservation. Ones maintain themselves through reciprocal exclusion. Self-preservation is dissolution. Negative relating is coming-together-with-oneself. Self-positing-in-a-one is attraction.',
};

const state2: DialecticState = {
  id: 'attraction-b',
  title: 'The one one of attraction — ideality realized, mediated one',
  concept: 'TheOneOneOfAttraction',
  phase: 'quality',

  moments: [
    {
      name: 'idealityRealized',
      definition: 'Repulsion passes over into attraction, ideality realized in attraction',
      type: 'sublation',
    },
    {
      name: 'mediatedOne',
      definition: 'One of attraction is mediated one, one posited as one',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'idealityRealized',
    },
    {
      name: 'unityOfRepulsionAndAttraction',
      definition: 'Attraction contains repulsion, unity of repulsion and attraction',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'attraction-b-inv-1',
      constraint: 'attraction = idealityRealized',
      predicate: 'equals(attraction, idealityRealized)',
    },
    {
      id: 'attraction-b-inv-2',
      constraint: 'attraction.inseparableFrom(repulsion) = true',
      predicate: 'inseparableFrom(attraction, repulsion)',
    },
  ],

  forces: [
    {
      id: 'attraction-b-force-1',
      description: 'Attraction drives toward connection with repulsion',
      type: 'mediation',
      trigger: 'attraction.containsRepulsion = true',
      effect: 'connectionOfRepulsionAndAttraction.emerges = true',
      targetState: 'attraction-c',
    },
  ],

  transitions: [
    {
      id: 'attraction-b-trans-1',
      from: 'attraction-b',
      to: 'attraction-c',
      mechanism: 'mediation',
      description: 'From attraction to connection',
    },
  ],

  nextStates: ['attraction-c'],
  previousStates: ['attraction-a'],

  provenance: {
    topicMapId: 'attraction-b',
    lineRange: { start: 139, end: 211 },
    section: 'b. The one one of attraction',
    order: 2,
  },

  description: 'The one one of attraction - ideality realized, mediated one. Repulsion passes over into attraction. Attraction inseparable from repulsion. Contains repulsion, preserves ones as many. Unity of repulsion and attraction.',
};

const state3: DialecticState = {
  id: 'attraction-c',
  title: 'Connection of repulsion and attraction — inseparable unity, transition to quantity',
  concept: 'ConnectionOfRepulsionAndAttraction',
  phase: 'quality',

  moments: [
    {
      name: 'inseparableUnity',
      definition: 'Repulsion and attraction essentially joined together, inseparable',
      type: 'mediation',
    },
    {
      name: 'selfPresupposing',
      definition: 'Each presupposes itself, each contains other as moment',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'inseparableUnity',
    },
    {
      name: 'transitionToQuantity',
      definition: 'Each is its own self-mediation, transition to quantity',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'attraction-c-inv-1',
      constraint: 'repulsion.inseparableFrom(attraction) = true',
      predicate: 'inseparableFrom(repulsion, attraction)',
    },
    {
      id: 'attraction-c-inv-2',
      constraint: 'each.presupposesItself = true',
      predicate: 'presupposesItself(each)',
    },
  ],

  forces: [
    {
      id: 'attraction-c-force-1',
      description: 'Connection transitions to quantity',
      type: 'sublation',
      trigger: 'each.selfMediation = true',
      effect: 'quantity.emerges = true',
      targetState: 'quantity-1',
    },
  ],

  transitions: [
    {
      id: 'attraction-c-trans-1',
      from: 'attraction-c',
      to: 'quantity-1',
      mechanism: 'sublation',
      description: 'From repulsion and attraction to quantity',
    },
  ],

  nextStates: ['quantity-1'],
  previousStates: ['attraction-b'],

  provenance: {
    topicMapId: 'attraction-c-2',
    lineRange: { start: 303, end: 393 },
    section: 'c. The connection of repulsion and attraction',
    order: 3,
  },

  description: 'Connection of repulsion and attraction - inseparable unity. Each presupposes itself, each contains other as moment. Each is self-mediation. Repulsion is positing of many, attraction positing of one. Transitions to quantity.',
};

export const attractionIR: DialecticIR = {
  id: 'attraction-ir',
  title: 'Repulsion and Attraction IR: Exclusion, Attraction, Connection, Quantity',
  section: 'I. BEING - A. QUALITY - C. Being-for-Self - C. Repulsion and Attraction',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'attraction.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'attraction-a': 'quality',
      'attraction-b': 'quality',
      'attraction-c': 'quality',
    },
  },
};

export const attractionStates = {
  'attraction-a': state1,
  'attraction-b': state2,
  'attraction-c': state3,
};
