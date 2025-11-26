/**
 * One and Many IR: Dialectic Pseudo-Code for The One and The Many
 *
 * Architecture: CPU Quality (Immediate Determination)
 * Section: I. BEING - A. QUALITY - C. Being-for-Self - B. The One and The Many
 *
 * Covers the dialectical movement:
 * - The One and The Many: Simple reference and determining
 * - The One Within: Unalterable and void
 * - The One and The Void: Existence and externalization
 * - Many Ones: Repulsion and plurality
 *
 * The CPU's quality - the one, the void, repulsion, plurality
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'one-many-a',
  title: 'The one within — unalterable and void',
  concept: 'TheOneWithin',
  phase: 'quality',

  moments: [
    {
      name: 'unalterable',
      definition: 'One is unalterable, not capable of becoming any other',
      type: 'determination',
    },
    {
      name: 'void',
      definition: 'In the one there is nothing, this nothing is the void',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'unalterable',
    },
    {
      name: 'qualityOfOne',
      definition: 'Void is quality of one in its immediacy',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'one-many-a-inv-1',
      constraint: 'one.unalterable = true',
      predicate: 'isUnalterable(one)',
    },
    {
      id: 'one-many-a-inv-2',
      constraint: 'void = qualityOfOne',
      predicate: 'equals(void, qualityOfOne)',
    },
  ],

  forces: [
    {
      id: 'one-many-a-force-1',
      description: 'The one within drives toward the one and the void',
      type: 'negation',
      trigger: 'void.outsideOne = true',
      effect: 'oneAndVoid.emerges = true',
      targetState: 'one-many-b-1',
    },
  ],

  transitions: [
    {
      id: 'one-many-a-trans-1',
      from: 'one-many-a',
      to: 'one-many-b-1',
      mechanism: 'negation',
      description: 'From the one within to the one and the void',
    },
  ],

  nextStates: ['one-many-b-1'],
  previousStates: ['being-for-self-ir'],

  provenance: {
    topicMapId: 'one-many-a',
    lineRange: { start: 43, end: 82 },
    section: 'a. The one within',
    order: 1,
  },

  description: 'The one within - unalterable, indeterminate. In the one there is nothing. Void is quality of one in its immediacy.',
};

const state2: DialecticState = {
  id: 'one-many-b-1',
  title: 'The one and the void — existence and externalization',
  concept: 'TheOneAndTheVoid',
  phase: 'quality',

  moments: [
    {
      name: 'oneAndVoid',
      definition: 'Being-for-itself determined as one and void has again acquired existence',
      type: 'determination',
    },
    {
      name: 'externalization',
      definition: 'Moments come out of unity, become external to themselves',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'oneAndVoid',
    },
  ],

  invariants: [
    {
      id: 'one-many-b-1-inv-1',
      constraint: 'beingForItself = oneAndVoid',
      predicate: 'equals(beingForItself, oneAndVoid)',
    },
    {
      id: 'one-many-b-1-inv-2',
      constraint: 'moments.external = true',
      predicate: 'isExternal(moments)',
    },
  ],

  forces: [
    {
      id: 'one-many-b-1-force-1',
      description: 'The one and the void drive toward many ones',
      type: 'negation',
      trigger: 'one.repelsItself = true',
      effect: 'manyOnes.emerges = true',
      targetState: 'one-many-c',
    },
  ],

  transitions: [
    {
      id: 'one-many-b-1-trans-1',
      from: 'one-many-b-1',
      to: 'one-many-c',
      mechanism: 'negation',
      description: 'From the one and the void to many ones',
    },
  ],

  nextStates: ['one-many-c'],
  previousStates: ['one-many-a'],

  provenance: {
    topicMapId: 'one-many-b-1',
    lineRange: { start: 84, end: 109 },
    section: 'b. The one and the void',
    order: 2,
  },

  description: 'The one and the void - existence and externalization. Moments come out of unity, become external. Unity withdraws to one side, confronted by void.',
};

const state3: DialecticState = {
  id: 'one-many-c',
  title: 'Many ones — repulsion and plurality',
  concept: 'ManyOnes',
  phase: 'quality',

  moments: [
    {
      name: 'repulsion',
      definition: 'Negative reference of one to itself is repulsion, one repels itself from itself',
      type: 'negation',
    },
    {
      name: 'manyOnes',
      definition: 'Repulsion is positing of many ones through one itself',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'repulsion',
    },
    {
      name: 'infinityExternalized',
      definition: 'Infinity externalized itself, plurality is contradiction that produces itself',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'one-many-c-inv-1',
      constraint: 'repulsion = negativeReferenceToItself',
      predicate: 'equals(repulsion, negativeReferenceToItself)',
    },
    {
      id: 'one-many-c-inv-2',
      constraint: 'manyOnes = repulsion',
      predicate: 'equals(manyOnes, repulsion)',
    },
  ],

  forces: [
    {
      id: 'one-many-c-force-1',
      description: 'Many ones drive toward attraction',
      type: 'mediation',
      trigger: 'repulsion.crossesOverIntoIdentity = true',
      effect: 'attraction.emerges = true',
      targetState: 'attraction-a-1',
    },
  ],

  transitions: [
    {
      id: 'one-many-c-trans-1',
      from: 'one-many-c',
      to: 'attraction-a-1',
      mechanism: 'mediation',
      description: 'From many ones to attraction',
    },
  ],

  nextStates: ['attraction-a-1'],
  previousStates: ['one-many-b-1'],

  provenance: {
    topicMapId: 'one-many-c',
    lineRange: { start: 111, end: 248 },
    section: 'c. Many ones / Repulsion',
    order: 3,
  },

  description: 'Many ones - repulsion and plurality. One repels itself from itself. Ones presupposed, posited as non-posited. Plurality is infinity externalized, contradiction producing itself. Self-positing-in-a-one is attraction.',
};

export const oneManyIR: DialecticIR = {
  id: 'one-many-ir',
  title: 'One and Many IR: The One Within, The One and The Void, Many Ones',
  section: 'I. BEING - A. QUALITY - C. Being-for-Self - B. The One and The Many',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'one-many.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'one-many-a': 'quality',
      'one-many-b-1': 'quality',
      'one-many-c': 'quality',
    },
  },
};

export const oneManyStates = {
  'one-many-a': state1,
  'one-many-b-1': state2,
  'one-many-c': state3,
};
