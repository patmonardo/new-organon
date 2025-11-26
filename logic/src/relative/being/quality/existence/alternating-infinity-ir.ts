/**
 * Alternating Infinity IR: Dialectic Pseudo-Code for the Bad Infinite
 *
 * Architecture: CPU Quality (Immediate Determination)
 * Section: I. BEING - A. QUALITY - C. Infinity - B. Alternating Determination
 *
 * Covers the dialectical movement:
 * - Bad Infinite: Infinite falls back into category of something, finite resurrected
 * - Contradiction: Two worlds, inseparable yet absolutely other
 * - Progress to Infinity: Repetitious monotony, unresolved contradiction
 *
 * The CPU's quality - bad infinite, infinite progression, ought
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'alternating-infinity-1',
  title: 'Bad infinite — infinite falls back into something, finite resurrected',
  concept: 'BadInfinite',
  phase: 'quality',

  moments: [
    {
      name: 'fallsBackIntoSomething',
      definition: 'Infinite as existent and non-being of other, fallen back into category of something with limit',
      type: 'negation',
    },
    {
      name: 'finiteResurrected',
      definition: 'Immediate being of infinite resurrects being of its negation, the finite',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'fallsBackIntoSomething',
    },
    {
      name: 'qualitativeMutualReference',
      definition: 'Finite stands over against infinite as real existence, qualitative mutual reference',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'alternating-infinity-1-inv-1',
      constraint: 'badInfinite = somethingWithLimit',
      predicate: 'equals(badInfinite, somethingWithLimit)',
    },
    {
      id: 'alternating-infinity-1-inv-2',
      constraint: 'infinite.resurrects(finite) = true',
      predicate: 'resurrects(infinite, finite)',
    },
  ],

  forces: [
    {
      id: 'alternating-infinity-1-force-1',
      description: 'Bad infinite drives toward contradiction',
      type: 'negation',
      trigger: 'infinite.burdenedWithOpposition = true',
      effect: 'contradiction.emerges = true',
      targetState: 'alternating-infinity-5',
    },
  ],

  transitions: [
    {
      id: 'alternating-infinity-1-trans-1',
      from: 'alternating-infinity-1',
      to: 'alternating-infinity-5',
      mechanism: 'negation',
      description: 'From bad infinite to contradiction',
    },
  ],

  nextStates: ['alternating-infinity-5'],
  previousStates: ['finitude-ir'],

  provenance: {
    topicMapId: 'alternating-infinity-4',
    lineRange: { start: 73, end: 88 },
    section: 'b. Alternating determination of finite and infinite',
    order: 1,
  },

  description: 'Bad infinite - infinite falls back into category of something with limit. Finite resurrected. Infinite of understanding, counts as highest truth but entangled in unresolved contradictions.',
};

const state2: DialecticState = {
  id: 'alternating-infinity-5',
  title: 'Contradiction — two worlds, inseparable yet absolutely other',
  concept: 'ContradictionOfFiniteAndInfinite',
  phase: 'quality',

  moments: [
    {
      name: 'twoWorlds',
      definition: 'Two determinacies, two worlds: one infinite, one finite. Infinite only limit of finite, itself finite infinite',
      type: 'negation',
    },
    {
      name: 'inseparableYetOther',
      definition: 'Two inseparable and at same time absolutely other, each has other in itself',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'twoWorlds',
    },
    {
      name: 'unityHidden',
      definition: 'Unity rests hidden in qualitative otherness, inner unity lying at base',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'alternating-infinity-5-inv-1',
      constraint: 'infinite.inseparableFrom(finite) = true',
      predicate: 'inseparableFrom(infinite, finite)',
    },
    {
      id: 'alternating-infinity-5-inv-2',
      constraint: 'unity.hidden = true',
      predicate: 'isHidden(unity)',
    },
  ],

  forces: [
    {
      id: 'alternating-infinity-5-force-1',
      description: 'Contradiction drives toward infinite progression',
      type: 'negation',
      trigger: 'unity.notReflectedUpon = true',
      effect: 'infiniteProgression.emerges = true',
      targetState: 'alternating-infinity-12',
    },
  ],

  transitions: [
    {
      id: 'alternating-infinity-5-trans-1',
      from: 'alternating-infinity-5',
      to: 'alternating-infinity-12',
      mechanism: 'negation',
      description: 'From contradiction to progress to infinity',
    },
  ],

  nextStates: ['alternating-infinity-12'],
  previousStates: ['alternating-infinity-1'],

  provenance: {
    topicMapId: 'alternating-infinity-8',
    lineRange: { start: 145, end: 173 },
    section: 'b. Alternating determination of finite and infinite',
    order: 2,
  },

  description: 'Contradiction - two worlds (infinite and finite), inseparable yet absolutely other. Each posits its other. Unity rests hidden in qualitative otherness, lies only at base.',
};

const state3: DialecticState = {
  id: 'alternating-infinity-12',
  title: 'Progress to infinity — repetitious monotony, unresolved contradiction',
  concept: 'ProgressToInfinity',
  phase: 'quality',

  moments: [
    {
      name: 'alternatingDetermination',
      definition: 'Alternating determination of self-negating and negating the negating, progress to infinity',
      type: 'negation',
    },
    {
      name: 'unresolvedContradiction',
      definition: 'Contradiction not resolved but always pronounced simply as present',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'alternatingDetermination',
    },
    {
      name: 'perpetualOught',
      definition: 'Bad infinite same as perpetual ought, unable to free itself from finite',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'alternating-infinity-12-inv-1',
      constraint: 'progressToInfinity = unresolvedContradiction',
      predicate: 'equals(progressToInfinity, unresolvedContradiction)',
    },
    {
      id: 'alternating-infinity-12-inv-2',
      constraint: 'badInfinite = perpetualOught',
      predicate: 'equals(badInfinite, perpetualOught)',
    },
  ],

  forces: [
    {
      id: 'alternating-infinity-12-force-1',
      description: 'Progress to infinity transitions to affirmative infinity',
      type: 'sublation',
      trigger: 'unity.reflectedUpon = true',
      effect: 'affirmativeInfinity.emerges = true',
      targetState: 'affirmative-infinity-1',
    },
  ],

  transitions: [
    {
      id: 'alternating-infinity-12-trans-1',
      from: 'alternating-infinity-12',
      to: 'affirmative-infinity-1',
      mechanism: 'sublation',
      description: 'From progress to infinity to affirmative infinity',
    },
  ],

  nextStates: ['affirmative-infinity-1'],
  previousStates: ['alternating-infinity-5'],

  provenance: {
    topicMapId: 'alternating-infinity-14',
    lineRange: { start: 257, end: 288 },
    section: 'b. Alternating determination of finite and infinite',
    order: 3,
  },

  description: 'Progress to infinity - repetitious monotony, tedious alternation. Contradiction not resolved. Bad infinite as perpetual ought, unable to free itself from finite. Unity not reflected upon, beyond unattainable.',
};

export const alternatingInfinityIR: DialecticIR = {
  id: 'alternating-infinity-ir',
  title: 'Alternating Infinity IR: Bad Infinite, Contradiction, Progress to Infinity',
  section: 'I. BEING - A. QUALITY - C. Infinity - B. Alternating Determination',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'alternating-infinity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'alternating-infinity-1': 'quality',
      'alternating-infinity-5': 'quality',
      'alternating-infinity-12': 'quality',
    },
  },
};

export const alternatingInfinityStates = {
  'alternating-infinity-1': state1,
  'alternating-infinity-5': state2,
  'alternating-infinity-12': state3,
};
