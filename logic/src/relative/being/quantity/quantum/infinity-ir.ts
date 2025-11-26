/**
 * Infinity IR: Dialectic Pseudo-Code for Quantitative Infinity
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: D. QUANTITATIVE INFINITY
 *
 * Covers the dialectical movement:
 * - Quantum as self-contradictory (inherently goes to infinity)
 * - Bad infinity (expression of contradiction, not resolution)
 * - Infinitely great and small (unsubstantial shadows)
 * - Concept of quantum posited in infinite progress
 * - Quantum returns to quality (negation of negation)
 *
 * Note: Selecting key states; full infinity has 10 topic map entries
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'infinity-concept-self-contradictory',
  title: 'Quantum as inherently self-contradictory',
  concept: 'QuantumSelfContradiction',
  phase: 'quantity',

  moments: [
    {
      name: 'selfContradictory',
      definition: 'Quantum is inherently self-contradictory, goes to infinity',
      type: 'negation',
    },
    {
      name: 'ought',
      definition: 'Quantum is ought: being-determined-for-itself is being determined in other',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'selfContradictory',
    },
    {
      name: 'unlimitedness',
      definition: 'Negative of itself as limited, hence unlimitedness/infinity',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'infinity-1-inv-1',
      constraint: 'quantum.inherentlySelfContradictory = true',
      predicate: 'selfContradictory(quantum)',
    },
    {
      id: 'infinity-1-inv-2',
      constraint: 'quantum.other = negativeOfItselfAsLimited',
      predicate: 'equals(quantum.other, negativeOfItself(limited))',
    },
    {
      id: 'infinity-1-inv-3',
      constraint: 'quantum.ought = beingDeterminedInOther',
      predicate: 'ought(quantum, beingDeterminedInOther)',
    },
  ],

  forces: [
    {
      id: 'infinity-1-force-1',
      description: 'Self-contradiction drives toward infinite process',
      type: 'contradiction',
      trigger: 'quantum.selfContradictory = true',
      effect: 'infiniteProcess.emerges = true',
      targetState: 'infinity-process-contradiction',
    },
  ],

  transitions: [
    {
      id: 'infinity-1-trans-1',
      from: 'infinity-concept-self-contradictory',
      to: 'infinity-process-contradiction',
      mechanism: 'contradiction',
      description: 'From self-contradiction to infinite process',
    },
  ],

  nextStates: ['infinity-process-contradiction'],
  previousStates: ['quantum-alteration-necessity'],

  provenance: {
    topicMapId: 'infinity-concept-self-contradictory',
    lineRange: { start: 6, end: 22 },
    section: 'a. Its concept',
    order: 1,
  },

  description: 'Quantum as self-contradictory: Quantum alters to another quantum, goes to infinity because inherently self-contradictory. Other is negative of itself as limited, hence unlimitedness/infinity. Quantum is ought: being-determined-for-itself is being determined in other.',
};

const state2: DialecticState = {
  id: 'infinity-process-contradiction',
  title: 'Infinite process: expression of contradiction',
  concept: 'InfiniteProcess',
  phase: 'quantity',

  moments: [
    {
      name: 'expression',
      definition: 'Infinite process as expression of contradiction',
      type: 'process',
    },
    {
      name: 'reciprocalDetermination',
      definition: 'Reciprocal determination of finite/infinite',
      type: 'mediation',
    },
    {
      name: 'limitSendsBeyond',
      definition: 'Limit sends itself beyond and continues there',
      type: 'process',
      relation: 'mediates',
      relatedTo: 'reciprocalDetermination',
    },
  ],

  invariants: [
    {
      id: 'infinity-2-inv-1',
      constraint: 'infiniteProcess = expression(contradiction)',
      predicate: 'expression(infiniteProcess, contradiction)',
    },
    {
      id: 'infinity-2-inv-2',
      constraint: 'finite ↔ infinite (reciprocal)',
      predicate: 'reciprocal(finite, infinite)',
    },
    {
      id: 'infinity-2-inv-3',
      constraint: 'quantum ∈ quantitativeInfinite',
      predicate: 'in(quantum, quantitativeInfinite)',
    },
  ],

  forces: [
    {
      id: 'infinity-2-force-1',
      description: 'Expression without resolution drives toward bad infinity',
      type: 'negation',
      trigger: 'expression.withoutResolution = true',
      effect: 'badInfinity.emerges = true',
      targetState: 'infinity-process-bad-infinity',
    },
  ],

  transitions: [
    {
      id: 'infinity-2-trans-1',
      from: 'infinity-process-contradiction',
      to: 'infinity-process-bad-infinity',
      mechanism: 'negation',
      description: 'From infinite process to bad infinity',
    },
  ],

  nextStates: ['infinity-process-bad-infinity'],
  previousStates: ['infinity-concept-self-contradictory'],

  provenance: {
    topicMapId: 'infinity-process-contradiction',
    lineRange: { start: 78, end: 89 },
    section: 'b. The quantitative infinite process',
    order: 4,
  },

  description: 'Infinite process: Expression of contradiction in quantitative finite/quantum. Reciprocal determination of finite/infinite. In quantity, limit sends itself beyond and continues there. Quantitative infinite has quantum in it - in externality quantum is itself.',
};

const state3: DialecticState = {
  id: 'infinity-process-bad-infinity',
  title: 'Bad infinity: perpetual generation, not attainment',
  concept: 'BadInfinity',
  phase: 'quantity',

  moments: [
    {
      name: 'perpetualGeneration',
      definition: 'Perpetual generation without attainment',
      type: 'process',
    },
    {
      name: 'beyondBecomesQuantum',
      definition: 'Beyond becomes quantum, which again flees to beyond',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'perpetualGeneration',
    },
    {
      name: 'repetition',
      definition: 'Positing, sublating, positing again',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'infinity-3-inv-1',
      constraint: 'badInfinity = expression ∧ not(resolution)',
      predicate: 'expression(contradiction) ∧ not(resolution(contradiction))',
    },
    {
      id: 'infinity-3-inv-2',
      constraint: 'attainment = perpetualGeneration',
      predicate: 'equals(attainment, perpetualGeneration)',
    },
    {
      id: 'infinity-3-inv-3',
      constraint: 'beyond.continuous = quantum',
      predicate: 'continuous(beyond, quantum)',
    },
  ],

  forces: [
    {
      id: 'infinity-3-force-1',
      description: 'Perpetual movement drives toward infinitely great/small',
      type: 'passover',
      trigger: 'movement.perpetual = true',
      effect: 'infinitelyGreatSmall.emerges = true',
      targetState: 'infinity-process-infinitely-great-small',
    },
  ],

  transitions: [
    {
      id: 'infinity-3-trans-1',
      from: 'infinity-process-bad-infinity',
      to: 'infinity-process-infinitely-great-small',
      mechanism: 'passover',
      description: 'From bad infinity to infinitely great and small',
    },
  ],

  nextStates: ['infinity-process-infinitely-great-small'],
  previousStates: ['infinity-process-contradiction'],

  provenance: {
    topicMapId: 'infinity-process-bad-infinity',
    lineRange: { start: 91, end: 129 },
    section: 'b. The quantitative infinite process',
    order: 5,
  },

  description: 'Bad infinity: Expression of contradiction, not resolution. Perpetual generation without attainment. Beyond is non-being of quantum, but quantum is continuous with beyond (other of itself). Beyond becomes quantum, which again flees to beyond - perpetual cycle.',
};

const state4: DialecticState = {
  id: 'infinity-process-infinitely-great-small',
  title: 'Infinitely great and small: unsubstantial shadows',
  concept: 'InfinitelyGreatSmall',
  phase: 'quantity',

  moments: [
    {
      name: 'infinitelyGreat',
      definition: 'Great enlarged shrinks to insignificance',
      type: 'negation',
    },
    {
      name: 'infinitelySmall',
      definition: 'Small remains quantum, absolutely qualitatively opposed',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'infinitelyGreat',
    },
    {
      name: 'unsubstantialShadows',
      definition: 'Both remain alterable, absolute determinateness not attained',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'infinity-4-inv-1',
      constraint: 'infinitelyGreat.alterable = true ∧ infinitelySmall.alterable = true',
      predicate: 'alterable(infinitelyGreat) ∧ alterable(infinitelySmall)',
    },
    {
      id: 'infinity-4-inv-2',
      constraint: 'increase ≠ approximation(infinite)',
      predicate: 'not(equals(increase, approximation(infinite)))',
    },
    {
      id: 'infinity-4-inv-3',
      constraint: 'contradiction: quantum.great ∧ quantum.nonFinite',
      predicate: 'contradiction(quantum.great, quantum.nonFinite)',
    },
  ],

  forces: [
    {
      id: 'infinity-4-force-1',
      description: 'Unsubstantial shadows drive toward concept posited',
      type: 'sublation',
      trigger: 'contradiction.explicit = true',
      effect: 'conceptPostited.emerges = true',
      targetState: 'infinity-quantum-concept-posited',
    },
  ],

  transitions: [
    {
      id: 'infinity-4-trans-1',
      from: 'infinity-process-infinitely-great-small',
      to: 'infinity-quantum-concept-posited',
      mechanism: 'sublation',
      description: 'From unsubstantial shadows to concept posited',
    },
  ],

  nextStates: ['infinity-quantum-concept-posited'],
  previousStates: ['infinity-process-bad-infinity'],

  provenance: {
    topicMapId: 'infinity-process-infinitely-great-small',
    lineRange: { start: 131, end: 157 },
    section: 'b. The quantitative infinite process',
    order: 6,
  },

  description: 'Infinitely great/small: Both remain alterable - absolute determinateness not attained. Great enlarged shrinks to insignificance (opposition qualitative). Increase not approximation. Contradiction: quantum ought to be great (quantum) and non-finite (not quantum).',
};

const state5: DialecticState = {
  id: 'infinity-quantum-concept-posited',
  title: 'Concept of quantum posited in infinite progress',
  concept: 'ConceptPostited',
  phase: 'quantity',

  moments: [
    {
      name: 'conceptPostited',
      definition: 'Through infinite progress, quantum nature posited',
      type: 'mediation',
    },
    {
      name: 'quantumContinues',
      definition: 'Quantum continues in non-being (externality)',
      type: 'mediation',
    },
    {
      name: 'beyondSublated',
      definition: 'Beyond sublated, determined as quantum',
      type: 'sublation',
      relation: 'transforms',
      relatedTo: 'quantumContinues',
    },
  ],

  invariants: [
    {
      id: 'infinity-5-inv-1',
      constraint: 'contradiction.explicit = true',
      predicate: 'explicit(contradiction)',
    },
    {
      id: 'infinity-5-inv-2',
      constraint: 'nonBeing = magnitude',
      predicate: 'is(nonBeing, magnitude)',
    },
    {
      id: 'infinity-5-inv-3',
      constraint: 'quantum.continuesInNonBeing = true',
      predicate: 'continuesIn(quantum, nonBeing)',
    },
    {
      id: 'infinity-5-inv-4',
      constraint: 'throughExternality(quantum) = quantumItself',
      predicate: 'equals(throughExternality(quantum), quantumItself)',
    },
  ],

  forces: [
    {
      id: 'infinity-5-force-1',
      description: 'Concept posited drives toward restoration via negation of negation',
      type: 'sublation',
      trigger: 'concept.posited = true',
      effect: 'restoration.emerges = true',
      targetState: 'infinity-quantum-return-quality',
    },
  ],

  transitions: [
    {
      id: 'infinity-5-trans-1',
      from: 'infinity-quantum-concept-posited',
      to: 'infinity-quantum-return-quality',
      mechanism: 'sublation',
      description: 'From concept posited to quantum returns to quality',
    },
  ],

  nextStates: ['infinity-quantum-return-quality'],
  previousStates: ['infinity-process-infinitely-great-small'],

  provenance: {
    topicMapId: 'infinity-quantum-concept-posited',
    lineRange: { start: 184, end: 222 },
    section: 'c. The infinity of quantum',
    order: 8,
  },

  description: 'Concept posited: In infinite progress, contradiction explicit - quantum nature posited. Non-being is magnitude - quantum continues in non-being. Beyond sublated, determined as quantum. Through externality quantum is itself - concept posited.',
};

const state6: DialecticState = {
  id: 'infinity-quantum-return-quality',
  title: 'Quantum returns to quality: qualitatively determined',
  concept: 'ReturnToQuality',
  phase: 'quantity',

  moments: [
    {
      name: 'negationOfNegation',
      definition: 'Sublating restores concept of magnitude',
      type: 'sublation',
    },
    {
      name: 'qualitativelyDetermined',
      definition: 'Quantum has determinateness in another quantum through infinity',
      type: 'quality',
    },
    {
      name: 'beingForItself',
      definition: 'Infinity is in it, not outside',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'qualitativelyDetermined',
    },
  ],

  invariants: [
    {
      id: 'infinity-6-inv-1',
      constraint: 'negation(negation) = restoration(concept)',
      predicate: 'equals(negationOfNegation, restoration(concept))',
    },
    {
      id: 'infinity-6-inv-2',
      constraint: 'quantum.determinedAccordingToConcept = true',
      predicate: 'determinedAccordingToConcept(quantum)',
    },
    {
      id: 'infinity-6-inv-3',
      constraint: 'externality.posited = momentOfMagnitude',
      predicate: 'posited(externality, momentOfMagnitude)',
    },
    {
      id: 'infinity-6-inv-4',
      constraint: 'quantum.qualitativelyWhatItIs = true',
      predicate: 'qualitativelyWhatItIs(quantum)',
    },
  ],

  forces: [
    {
      id: 'infinity-6-force-1',
      description: 'Return to quality prepares ratio as qualitatively determined',
      type: 'passover',
      trigger: 'quantum.returnsToQuality = true',
      effect: 'ratio.emerges = true',
      targetState: 'ratio-intro-qualitative-moment',
    },
  ],

  transitions: [
    {
      id: 'infinity-6-trans-1',
      from: 'infinity-quantum-return-quality',
      to: 'ratio-intro-qualitative-moment',
      mechanism: 'passover',
      description: 'From quantum return to quality to ratio as qualitative relation',
    },
  ],

  nextStates: ['ratio-intro-qualitative-moment'],
  previousStates: ['infinity-quantum-concept-posited'],

  provenance: {
    topicMapId: 'infinity-quantum-return-quality',
    lineRange: { start: 246, end: 296 },
    section: 'c. The infinity of quantum',
    order: 10,
  },

  description: 'Return to quality: Negation of negation restores concept. Quantum determined according to concept - externality posited as moment. Quantum has determinateness in another quantum through infinity - qualitatively what it is. Externality (its quality) is being itself in externality, referring to itself. Infinity is in it, not outside.',
};

export const infinityIR: DialecticIR = {
  id: 'infinity-ir',
  title: 'Infinity IR: Quantitative Infinity and Return to Quality',
  section: 'D. QUANTITATIVE INFINITY',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'infinity.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'infinity-concept-self-contradictory': 'quantity',
      'infinity-process-contradiction': 'quantity',
      'infinity-process-bad-infinity': 'quantity',
      'infinity-process-infinitely-great-small': 'quantity',
      'infinity-quantum-concept-posited': 'quantity',
      'infinity-quantum-return-quality': 'quantity',
    },
  },
};

export const infinityStates = {
  'infinity-concept-self-contradictory': state1,
  'infinity-process-contradiction': state2,
  'infinity-process-bad-infinity': state3,
  'infinity-process-infinitely-great-small': state4,
  'infinity-quantum-concept-posited': state5,
  'infinity-quantum-return-quality': state6,
};
