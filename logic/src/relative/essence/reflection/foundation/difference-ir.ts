/**
 * Difference IR: Dialectic Pseudo-Code for Difference
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - B. Difference
 *
 * Covers the dialectical movement:
 * - Absolute difference (simple difference)
 * - Diversity (identity breaks apart, likeness/unlikeness)
 * - External reflection (comparison, negative unity)
 * - Opposition (unity of identity and diversity)
 * - Positive and Negative (self-subsisting sides of opposition)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'diff-1',
  title: 'Absolute difference as simple — difference of essence',
  concept: 'AbsoluteDifference',
  phase: 'reflection',

  moments: [
    {
      name: 'absoluteDifference',
      definition: 'Difference in and for itself, difference of essence',
      type: 'determination',
    },
    {
      name: 'simpleDifference',
      definition: 'Self-referring, simple difference; simple "not" constitutes difference',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'absoluteDifference',
    },
    {
      name: 'negativityOfReflection',
      definition: 'Difference is negativity that reflection possesses in itself',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'diff-1-inv-1',
      constraint: 'difference = absolute',
      predicate: 'is(difference, absolute)',
    },
    {
      id: 'diff-1-inv-2',
      constraint: 'difference.simple = true',
      predicate: 'isSimple(difference)',
    },
    {
      id: 'diff-1-inv-3',
      constraint: 'difference = negativity(reflection)',
      predicate: 'equals(difference, negativity(reflection))',
    },
  ],

  forces: [
    {
      id: 'diff-1-force-1',
      description: 'Absolute difference breaks apart into diversity',
      type: 'negation',
      trigger: 'difference.absolute = true',
      effect: 'diversity.emerges = true',
      targetState: 'diff-5',
    },
  ],

  transitions: [
    {
      id: 'diff-1-trans-1',
      from: 'diff-1',
      to: 'diff-5',
      mechanism: 'negation',
      description: 'From absolute difference to diversity',
    },
  ],

  nextStates: ['diff-5'],
  previousStates: ['identity-ir'],

  provenance: {
    topicMapId: 'diff-2',
    lineRange: { start: 14, end: 47 },
    section: 'B. Difference',
    order: 1,
  },

  description: 'Difference in and for itself, absolute difference, difference of essence. Not difference through something external but self-referring, simple difference. In absolute difference of A and not-A, simple \'not\' constitutes difference. Difference is negativity that reflection possesses in itself.',
};

const state2: DialecticState = {
  id: 'diff-5',
  title: 'Diversity — identity breaks apart',
  concept: 'Diversity',
  phase: 'reflection',

  moments: [
    {
      name: 'diversity',
      definition: 'Identity internally breaks apart into diversity',
      type: 'determination',
    },
    {
      name: 'indifference',
      definition: 'Different subsists as diverse, indifferent to any other',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'diversity',
    },
    {
      name: 'externalReflection',
      definition: 'Reflection has become external',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'diff-5-inv-1',
      constraint: 'identity.breaksApart = diversity',
      predicate: 'breaksApartInto(identity, diversity)',
    },
    {
      id: 'diff-5-inv-2',
      constraint: 'diverse.indifferent = true',
      predicate: 'isIndifferent(diverse)',
    },
    {
      id: 'diff-5-inv-3',
      constraint: 'reflection = external',
      predicate: 'is(reflection, external)',
    },
  ],

  forces: [
    {
      id: 'diff-5-force-1',
      description: 'Diversity drives external reflection to compare (likeness/unlikeness)',
      type: 'mediation',
      trigger: 'diversity.indifferent = true',
      effect: 'comparison.emerges = true',
      targetState: 'diff-8',
    },
  ],

  transitions: [
    {
      id: 'diff-5-trans-1',
      from: 'diff-5',
      to: 'diff-8',
      mechanism: 'mediation',
      description: 'From diversity to external reflection (comparison)',
    },
  ],

  nextStates: ['diff-8'],
  previousStates: ['diff-1'],

  provenance: {
    topicMapId: 'diff-5',
    lineRange: { start: 98, end: 121 },
    section: 'B. Difference',
    order: 2,
  },

  description: 'Identity internally breaks apart into diversity. As absolute difference in itself, posits itself as negative of itself. Different subsists as diverse, indifferent to any other. In diversity, reflection has become external.',
};

const state3: DialecticState = {
  id: 'diff-8',
  title: 'External reflection connects diversity — likeness and unlikeness',
  concept: 'ExternalComparison',
  phase: 'reflection',

  moments: [
    {
      name: 'externalReflection',
      definition: 'Connects diversity by referring it to likeness and unlikeness',
      type: 'mediation',
    },
    {
      name: 'likeness',
      definition: 'External identity is likeness',
      type: 'determination',
    },
    {
      name: 'unlikeness',
      definition: 'External difference is unlikeness',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'likeness',
    },
  ],

  invariants: [
    {
      id: 'diff-8-inv-1',
      constraint: 'comparison = externalReflection',
      predicate: 'equals(comparison, externalReflection)',
    },
    {
      id: 'diff-8-inv-2',
      constraint: 'likeness = external(identity)',
      predicate: 'equals(likeness, external(identity))',
    },
    {
      id: 'diff-8-inv-3',
      constraint: 'unlikeness = external(difference)',
      predicate: 'equals(unlikeness, external(difference))',
    },
  ],

  forces: [
    {
      id: 'diff-8-force-1',
      description: 'Comparison leads to negative unity and opposition',
      type: 'negation',
      trigger: 'comparison.complete = true',
      effect: 'opposition.emerges = true',
      targetState: 'diff-11',
    },
  ],

  transitions: [
    {
      id: 'diff-8-trans-1',
      from: 'diff-8',
      to: 'diff-11',
      mechanism: 'negation',
      description: 'From external comparison to opposition',
    },
  ],

  nextStates: ['diff-11'],
  previousStates: ['diff-5'],

  provenance: {
    topicMapId: 'diff-8',
    lineRange: { start: 210, end: 231 },
    section: 'B. Difference',
    order: 3,
  },

  description: 'External reflection connects diversity by referring it to likeness and unlikeness. Reference is comparing, moves back and forth. External identity is likeness, external difference is unlikeness. Whether like or unlike depends on point of view of third external to them.',
};

const state4: DialecticState = {
  id: 'diff-11',
  title: 'Opposition — unity of identity and diversity',
  concept: 'Opposition',
  phase: 'reflection',

  moments: [
    {
      name: 'opposition',
      definition: 'Unity of identity and diversity; difference brought to completion',
      type: 'determination',
    },
    {
      name: 'opposites',
      definition: 'Moments are diverse in one identity, so they are opposites',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'opposition',
    },
    {
      name: 'immanentReflection',
      definition: 'Identity and difference are moments of difference held inside difference itself',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'diff-11-inv-1',
      constraint: 'opposition = unity(identity, diversity)',
      predicate: 'equals(opposition, unity(identity, diversity))',
    },
    {
      id: 'diff-11-inv-2',
      constraint: 'difference.completed = opposition',
      predicate: 'equals(completed(difference), opposition)',
    },
    {
      id: 'diff-11-inv-3',
      constraint: 'moments.diverseInIdentity = true',
      predicate: 'diverseInIdentity(moments)',
    },
  ],

  forces: [
    {
      id: 'diff-11-force-1',
      description: 'Opposition determines itself as positive and negative',
      type: 'mediation',
      trigger: 'opposition.established = true',
      effect: 'positiveNegative.emerge = true',
      targetState: 'diff-13',
    },
  ],

  transitions: [
    {
      id: 'diff-11-trans-1',
      from: 'diff-11',
      to: 'diff-13',
      mechanism: 'mediation',
      description: 'From opposition to positive and negative',
    },
  ],

  nextStates: ['diff-13'],
  previousStates: ['diff-8'],

  provenance: {
    topicMapId: 'diff-11',
    lineRange: { start: 351, end: 370 },
    section: 'B. Difference',
    order: 4,
  },

  description: 'In opposition, determinate reflection, difference, brought to completion. Opposition is unity of identity and diversity. Moments are diverse in one identity, so they are opposites. Identity and difference are moments of difference held inside difference itself.',
};

const state5: DialecticState = {
  id: 'diff-13',
  title: 'Positive and negative — determinations of opposition',
  concept: 'PositiveAndNegative',
  phase: 'reflection',

  moments: [
    {
      name: 'positive',
      definition: 'Positedness reflected into self-likeness; contains reference to unlikeness',
      type: 'determination',
    },
    {
      name: 'negative',
      definition: 'Positedness reflected into unlikeness; contains reference to likeness',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'positive',
    },
    {
      name: 'selfSubsistence',
      definition: 'Positive and negative are sides of opposition that have become self-subsisting',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'diff-13-inv-1',
      constraint: 'positive = reflectedInto(selfLikeness)',
      predicate: 'equals(positive, reflectedInto(selfLikeness))',
    },
    {
      id: 'diff-13-inv-2',
      constraint: 'negative = reflectedInto(unlikeness)',
      predicate: 'equals(negative, reflectedInto(unlikeness))',
    },
    {
      id: 'diff-13-inv-3',
      constraint: 'positive.contains = unlikeness',
      predicate: 'contains(positive, unlikeness)',
    },
    {
      id: 'diff-13-inv-4',
      constraint: 'negative.contains = likeness',
      predicate: 'contains(negative, likeness)',
    },
  ],

  forces: [
    {
      id: 'diff-13-force-1',
      description: 'Self-subsisting opposition drives toward contradiction',
      type: 'negation',
      trigger: 'selfSubsistence.opposition = true',
      effect: 'contradiction.emerges = true',
      targetState: 'ctr-1',
    },
  ],

  transitions: [
    {
      id: 'diff-13-trans-1',
      from: 'diff-13',
      to: 'ctr-1',
      mechanism: 'negation',
      description: 'From positive/negative to contradiction',
    },
  ],

  nextStates: ['ctr-1'],
  previousStates: ['diff-11'],

  provenance: {
    topicMapId: 'diff-13',
    lineRange: { start: 394, end: 420 },
    section: 'B. Difference',
    order: 5,
  },

  description: 'Self-likeness, reflected into itself, containing reference to unlikeness within it, is positive. Unlikeness containing reference to its non-being, to likeness, is negative. Both are positedness. Each equally has other in it. Positive and negative are sides of opposition that have become self-subsisting.',
};

export const differenceIR: DialecticIR = {
  id: 'difference-ir',
  title: 'Difference IR: Absolute Difference, Diversity, Opposition',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - B. Difference',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'difference.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'diff-1': 'reflection',
      'diff-5': 'reflection',
      'diff-8': 'reflection',
      'diff-11': 'reflection',
      'diff-13': 'reflection',
    },
  },
};

export const differenceStates = {
  'diff-1': state1,
  'diff-5': state2,
  'diff-8': state3,
  'diff-11': state4,
  'diff-13': state5,
};
