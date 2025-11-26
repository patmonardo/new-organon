/**
 * Affirmative Infinity IR: Dialectic Pseudo-Code for the True Infinite
 *
 * Architecture: CPU Quality (Immediate Determination)
 * Section: I. BEING - A. QUALITY - C. Infinity - C. Affirmative Infinity
 *
 * Covers the dialectical movement:
 * - Unity of Finite and Infinite: Each contains other, negation of negation
 * - True Infinite as Becoming: Further determined moments
 * - True Infinite as Being: Circle vs straight line, ideality
 *
 * The CPU's quality - true infinite, ideality, transition to quantity
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'affirmative-infinity-7',
  title: 'Unity of finite and infinite — each contains other, negation of negation',
  concept: 'UnityOfFiniteAndInfinite',
  phase: 'quality',

  moments: [
    {
      name: 'eachContainsOther',
      definition: 'Each is itself this unity, as sublating of itself. Finitude only as transcending of itself, contains infinite',
      type: 'sublation',
    },
    {
      name: 'negationOfNegation',
      definition: 'Negation of negation in itself is self-reference, affirmation, turning back to itself',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'eachContainsOther',
    },
    {
      name: 'selfMediation',
      definition: 'Finite not sublated by infinite as external power, its infinity consists in sublating itself',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'affirmative-infinity-7-inv-1',
      constraint: 'finite.contains(infinite) = true',
      predicate: 'contains(finite, infinite)',
    },
    {
      id: 'affirmative-infinity-7-inv-2',
      constraint: 'negationOfNegation = selfReference',
      predicate: 'equals(negationOfNegation, selfReference)',
    },
  ],

  forces: [
    {
      id: 'affirmative-infinity-7-force-1',
      description: 'Unity drives toward true infinite as becoming',
      type: 'sublation',
      trigger: 'both.returnToSelfThroughNegation = true',
      effect: 'trueInfiniteAsBecoming.emerges = true',
      targetState: 'affirmative-infinity-15',
    },
  ],

  transitions: [
    {
      id: 'affirmative-infinity-7-trans-1',
      from: 'affirmative-infinity-7',
      to: 'affirmative-infinity-15',
      mechanism: 'sublation',
      description: 'From unity to true infinite as becoming',
    },
  ],

  nextStates: ['affirmative-infinity-15'],
  previousStates: ['alternating-infinity-ir'],

  provenance: {
    topicMapId: 'affirmative-infinity-7',
    lineRange: { start: 140, end: 165 },
    section: 'c. Affirmative infinity',
    order: 1,
  },

  description: 'Unity of finite and infinite - each is itself this unity as sublating of itself. Finitude contains infinite, infinite contains finite. Negation of negation is self-reference, affirmation. Finite sublates itself.',
};

const state2: DialecticState = {
  id: 'affirmative-infinity-15',
  title: 'True infinite as becoming — further determined moments',
  concept: 'TrueInfiniteAsBecoming',
  phase: 'quality',

  moments: [
    {
      name: 'essentiallyBecoming',
      definition: 'Infinite essentially only as becoming, further determined in its moments',
      type: 'sublation',
    },
    {
      name: 'momentsOfBecoming',
      definition: 'Becoming: first being and nothing; as alteration, something and other; now as infinite, finite and infinite',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'essentiallyBecoming',
    },
  ],

  invariants: [
    {
      id: 'affirmative-infinity-15-inv-1',
      constraint: 'trueInfinite = becoming',
      predicate: 'equals(trueInfinite, becoming)',
    },
    {
      id: 'affirmative-infinity-15-inv-2',
      constraint: 'becoming.moments = [finite, infinite]',
      predicate: 'equals(becoming.moments, [finite, infinite])',
    },
  ],

  forces: [
    {
      id: 'affirmative-infinity-15-force-1',
      description: 'True infinite as becoming transitions to true infinite as being',
      type: 'sublation',
      trigger: 'becomingTurnedBackIntoItself = true',
      effect: 'trueInfiniteAsBeing.emerges = true',
      targetState: 'affirmative-infinity-16',
    },
  ],

  transitions: [
    {
      id: 'affirmative-infinity-15-trans-1',
      from: 'affirmative-infinity-15',
      to: 'affirmative-infinity-16',
      mechanism: 'sublation',
      description: 'From true infinite as becoming to true infinite as being',
    },
  ],

  nextStates: ['affirmative-infinity-16'],
  previousStates: ['affirmative-infinity-7'],

  provenance: {
    topicMapId: 'affirmative-infinity-15',
    lineRange: { start: 335, end: 347 },
    section: 'c. Affirmative infinity',
    order: 2,
  },

  description: 'True infinite as becoming - essentially only as becoming, further determined in moments. Becoming has finite and infinite as its moments, themselves as in becoming.',
};

const state3: DialecticState = {
  id: 'affirmative-infinity-16',
  title: 'True infinite as being — circle vs straight line, ideality, transition to quantity',
  concept: 'TrueInfiniteAsBeing',
  phase: 'quality',

  moments: [
    {
      name: 'beingTurnedBackIntoItself',
      definition: 'Infinite as being-turned-back-unto-itself, reference of itself to itself, is being',
      type: 'sublation',
    },
    {
      name: 'circleNotStraightLine',
      definition: 'Bad infinite as straight line (infinite only at limits). True infinite as circle, bent back upon itself, closed and wholly present',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'beingTurnedBackIntoItself',
    },
    {
      name: 'ideality',
      definition: 'Negation determined as ideality. Idealized is finite as it is in true infinite (moment, not subsistent existent)',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'affirmative-infinity-16-inv-1',
      constraint: 'trueInfinite.image = circle',
      predicate: 'equals(trueInfinite.image, circle)',
    },
    {
      id: 'affirmative-infinity-16-inv-2',
      constraint: 'negation = ideality',
      predicate: 'equals(negation, ideality)',
    },
  ],

  forces: [
    {
      id: 'affirmative-infinity-16-force-1',
      description: 'True infinite transitions to being-for-self',
      type: 'sublation',
      trigger: 'ideality.realized = true',
      effect: 'beingForSelf.emerges = true',
      targetState: 'being-for-self-a',
    },
  ],

  transitions: [
    {
      id: 'affirmative-infinity-16-trans-1',
      from: 'affirmative-infinity-16',
      to: 'being-for-self-a',
      mechanism: 'sublation',
      description: 'From true infinite to being-for-self',
    },
  ],

  nextStates: ['being-for-self-a'],
  previousStates: ['affirmative-infinity-15'],

  provenance: {
    topicMapId: 'affirmative-infinity-17',
    lineRange: { start: 384, end: 427 },
    section: 'c. Affirmative infinity',
    order: 3,
  },

  description: 'True infinite as being - being-turned-back-unto-itself, negating negation. Image is circle (closed, wholly present) not straight line. Negation as ideality. Not finite which is real, but infinite. Transitions to being-for-self.',
};

export const affirmativeInfinityIR: DialecticIR = {
  id: 'affirmative-infinity-ir',
  title: 'Affirmative Infinity IR: Unity, True Infinite as Becoming, True Infinite as Being',
  section: 'I. BEING - A. QUALITY - C. Infinity - C. Affirmative Infinity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'affirmative-infinity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'affirmative-infinity-7': 'quality',
      'affirmative-infinity-15': 'quality',
      'affirmative-infinity-16': 'quality',
    },
  },
};

export const affirmativeInfinityStates = {
  'affirmative-infinity-7': state1,
  'affirmative-infinity-15': state2,
  'affirmative-infinity-16': state3,
};
