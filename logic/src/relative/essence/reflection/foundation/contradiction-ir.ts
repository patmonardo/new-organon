/**
 * Contradiction IR: Dialectic Pseudo-Code for Contradiction
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - C. Contradiction
 *
 * Covers the dialectical movement:
 * - Opposition as contradiction (self-excluding determination)
 * - Posited contradiction (positive and negative as contradiction)
 * - Resolution of contradiction (foundering to ground)
 * - Essence as ground (restored unity, positedness as ground)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'ctr-1',
  title: 'Opposition as self-subsisting — contradiction',
  concept: 'Contradiction',
  phase: 'reflection',

  moments: [
    {
      name: 'contradiction',
      definition: 'Self-subsisting determination excludes other in same respect as it contains it',
      type: 'negation',
    },
    {
      name: 'selfExclusion',
      definition: 'In self-subsistence, determination excludes its own self-subsistence from itself',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'contradiction',
    },
    {
      name: 'mutuallyExclusive',
      definition: 'Moments equally determined within, mutually exclusive, self-subsisting',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'ctr-1-inv-1',
      constraint: 'selfSubsistence.excludes = selfSubsistence',
      predicate: 'excludes(selfSubsistence, selfSubsistence)',
    },
    {
      id: 'ctr-1-inv-2',
      constraint: 'determination.contains = determination.excludes',
      predicate: 'containsAndExcludes(determination, other)',
    },
    {
      id: 'ctr-1-inv-3',
      constraint: 'opposition = contradiction',
      predicate: 'is(opposition, contradiction)',
    },
  ],

  forces: [
    {
      id: 'ctr-1-force-1',
      description: 'Contradiction becomes posited in positive and negative',
      type: 'negation',
      trigger: 'contradiction.implicit = true',
      effect: 'contradiction.posited = true',
      targetState: 'ctr-4',
    },
  ],

  transitions: [
    {
      id: 'ctr-1-trans-1',
      from: 'ctr-1',
      to: 'ctr-4',
      mechanism: 'negation',
      description: 'From implicit contradiction to posited contradiction',
    },
  ],

  nextStates: ['ctr-4'],
  previousStates: ['difference-ir'],

  provenance: {
    topicMapId: 'ctr-1',
    lineRange: { start: 4, end: 39 },
    section: 'C. Contradiction',
    order: 1,
  },

  description: 'In opposition moments equally determined within, mutually exclusive, self-subsisting. Self-subsisting determination excludes other in same respect as it contains it. In self-subsistence, determination excludes its own self-subsistence from itself. And so it is contradiction.',
};

const state2: DialecticState = {
  id: 'ctr-4',
  title: 'Absolute contradiction of positive and negative',
  concept: 'PositedContradiction',
  phase: 'reflection',

  moments: [
    {
      name: 'absoluteContradiction',
      definition: 'Absolute contradiction of positive and negative',
      type: 'negation',
    },
    {
      name: 'negativeAsContradiction',
      definition: 'Negative is posited contradiction; identical with itself yet excludes identity',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'absoluteContradiction',
    },
    {
      name: 'positiveAsContradiction',
      definition: 'Positive makes itself into negative by excluding negative',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'ctr-4-inv-1',
      constraint: 'negative.excludes = identity',
      predicate: 'excludes(negative, identity)',
    },
    {
      id: 'ctr-4-inv-2',
      constraint: 'negative.is = identity',
      predicate: 'is(negative, identity)',
    },
    {
      id: 'ctr-4-inv-3',
      constraint: 'positive.makesItself = negative',
      predicate: 'makesItself(positive, negative)',
    },
  ],

  forces: [
    {
      id: 'ctr-4-force-1',
      description: 'Absolute contradiction drives toward resolution (foundering)',
      type: 'sublation',
      trigger: 'contradiction.absolute = true',
      effect: 'resolution.emerges = true',
      targetState: 'ctr-5',
    },
  ],

  transitions: [
    {
      id: 'ctr-4-trans-1',
      from: 'ctr-4',
      to: 'ctr-5',
      mechanism: 'sublation',
      description: 'From absolute contradiction to resolution',
    },
  ],

  nextStates: ['ctr-5'],
  previousStates: ['ctr-1'],

  provenance: {
    topicMapId: 'ctr-4',
    lineRange: { start: 82, end: 130 },
    section: 'C. Contradiction',
    order: 2,
  },

  description: 'Absolute contradiction of positive, immediately absolute contradiction of negative. Negative is posited contradiction. Negative identical with itself, determination is to be not-identical, exclusion of identity. To be identical with itself over against identity, excludes itself from itself.',
};

const state3: DialecticState = {
  id: 'ctr-5',
  title: 'Contradiction resolves itself — foundering to ground',
  concept: 'ResolutionOfContradiction',
  phase: 'reflection',

  moments: [
    {
      name: 'foundering',
      definition: 'Positedness founders to ground in contradiction',
      type: 'sublation',
    },
    {
      name: 'theNull',
      definition: 'Internal ceaseless vanishing of opposites is the null',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'foundering',
    },
    {
      name: 'restoration',
      definition: 'Essence is restored through sublating of determinations',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'ctr-5-inv-1',
      constraint: 'opposites.vanish = true',
      predicate: 'vanish(opposites)',
    },
    {
      id: 'ctr-5-inv-2',
      constraint: 'result ≠ null',
      predicate: 'not(equals(result, null))',
    },
    {
      id: 'ctr-5-inv-3',
      constraint: 'positedness.founders = ground',
      predicate: 'foundersTo(positedness, ground)',
    },
  ],

  forces: [
    {
      id: 'ctr-5-force-1',
      description: 'Foundering of opposition restores essence as ground',
      type: 'mediation',
      trigger: 'foundering.complete = true',
      effect: 'ground.emerges = true',
      targetState: 'ctr-10',
    },
  ],

  transitions: [
    {
      id: 'ctr-5-trans-1',
      from: 'ctr-5',
      to: 'ctr-10',
      mechanism: 'mediation',
      description: 'From foundering to ground',
    },
  ],

  nextStates: ['ctr-10'],
  previousStates: ['ctr-4'],

  provenance: {
    topicMapId: 'ctr-5',
    lineRange: { start: 132, end: 154 },
    section: 'C. Contradiction',
    order: 3,
  },

  description: 'In self-excluding reflection, positive and negative sublate themselves. Internal ceaseless vanishing of opposites is first unity that arises by virtue of contradiction. It is the null. But contradiction does not contain merely negative, also contains positive. Positedness founders to ground in contradiction.',
};

const state4: DialecticState = {
  id: 'ctr-10',
  title: 'Resolved contradiction is ground — unity of positive and negative',
  concept: 'EssenceAsGround',
  phase: 'reflection',

  moments: [
    {
      name: 'ground',
      definition: 'Resolved contradiction is ground, essence as unity of positive and negative',
      type: 'determination',
    },
    {
      name: 'completedSelfSubsistence',
      definition: 'Ground is this self-subsistence as completed',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'ground',
    },
    {
      name: 'positiveSelfIdentity',
      definition: 'Ground is essence as positive self-identity which refers itself to itself as negativity',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'ctr-10-inv-1',
      constraint: 'ground = resolved(contradiction)',
      predicate: 'equals(ground, resolved(contradiction))',
    },
    {
      id: 'ctr-10-inv-2',
      constraint: 'ground = unity(positive, negative)',
      predicate: 'equals(ground, unity(positive, negative))',
    },
    {
      id: 'ctr-10-inv-3',
      constraint: 'essence.reflectedIntoItself = true',
      predicate: 'reflectedIntoItself(essence)',
    },
  ],

  forces: [
    {
      id: 'ctr-10-force-1',
      description: 'Essence as ground drives toward Ground determinations',
      type: 'passover',
      trigger: 'ground.established = true',
      effect: 'groundDeterminations.emerge = true',
      targetState: 'ground-1',
    },
  ],

  transitions: [
    {
      id: 'ctr-10-trans-1',
      from: 'ctr-10',
      to: 'ground-1',
      mechanism: 'passover',
      description: 'From essence as ground to Ground determinations',
    },
  ],

  nextStates: ['ground-1'],
  previousStates: ['ctr-5'],

  provenance: {
    topicMapId: 'ctr-10',
    lineRange: { start: 267, end: 293 },
    section: 'C. Contradiction',
    order: 4,
  },

  description: 'Resolved contradiction is ground, essence as unity of positive and negative. In ground, opposition and its contradiction just as much removed as preserved. Ground is essence as positive self-identity which refers itself to itself as negativity. In foundering, in positedness or in negation, rather is for first time essence that is reflected into itself and self-identical.',
};

export const contradictionIR: DialecticIR = {
  id: 'contradiction-ir',
  title: 'Contradiction IR: Opposition, Posited Contradiction, Ground',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - C. Contradiction',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'contradiction.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'ctr-1': 'reflection',
      'ctr-4': 'reflection',
      'ctr-5': 'reflection',
      'ctr-10': 'reflection',
    },
  },
};

export const contradictionStates = {
  'ctr-1': state1,
  'ctr-4': state2,
  'ctr-5': state3,
  'ctr-10': state4,
};
