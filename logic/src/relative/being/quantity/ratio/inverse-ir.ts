/**
 * Inverse IR: Dialectic Pseudo-Code for The Inverse Ratio
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: B. THE INVERSE RATIO
 *
 * Covers the dialectical movement:
 * - Inverse ratio as sublated direct ratio (exponent as product)
 * - Qualitative nature (reciprocal limiting)
 * - Bad infinity (sides approximate exponent)
 * - Transition to ratio of powers (exponent rejoins itself in externality)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'inverse-sublated-direct',
  title: 'Inverse ratio: sublated direct ratio',
  concept: 'InverseRatio',
  phase: 'quantity',

  moments: [
    {
      name: 'sublatedDirectRatio',
      definition: 'Exponent has value of product (unity of unit/amount)',
      type: 'sublation',
    },
    {
      name: 'qualitativeMoment',
      definition: 'Qualitative moment comes to fore, exponent as limit',
      type: 'quality',
      relation: 'mediates',
      relatedTo: 'sublatedDirectRatio',
    },
    {
      name: 'alterationContained',
      definition: 'Alteration contained within ratio, limited by exponent',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'inverse-1-inv-1',
      constraint: 'exponent.value = product(unit, amount)',
      predicate: 'equals(exponent.value, product(unit, amount))',
    },
    {
      id: 'inverse-1-inv-2',
      constraint: 'exponent.posited = negativeTowardsItself',
      predicate: 'negative(exponent, itself)',
    },
    {
      id: 'inverse-1-inv-3',
      constraint: 'qualitativeMoment.comesToFore = true',
      predicate: 'comesToFore(qualitativeMoment)',
    },
    {
      id: 'inverse-1-inv-4',
      constraint: 'alteration.limitedBy = exponent',
      predicate: 'limitedBy(alteration, exponent)',
    },
  ],

  forces: [
    {
      id: 'inverse-1-force-1',
      description: 'Qual itative moment drives toward reciprocal limiting',
      type: 'mediation',
      trigger: 'qualitativeMoment.explicit = true',
      effect: 'reciprocalLimiting.emerges = true',
      targetState: 'inverse-reciprocal-limiting',
    },
  ],

  transitions: [
    {
      id: 'inverse-1-trans-1',
      from: 'inverse-sublated-direct',
      to: 'inverse-reciprocal-limiting',
      mechanism: 'mediation',
      description: 'From sublated direct ratio to reciprocal limiting',
    },
  ],

  nextStates: ['inverse-reciprocal-limiting'],
  previousStates: ['ratio-direct-incompleteness'],

  provenance: {
    topicMapId: 'inverse-sublated-direct',
    lineRange: { start: 4, end: 70 },
    section: 'B. THE INVERSE RATIO',
    order: 1,
  },

  description: 'Inverse ratio: Sublated direct ratio. Exponent has value of product (unity of unit/amount). Exponent is immediate quantum but posited as negative towards itself as quantum of ratio - hence qualitative, as limit. Qualitative moment comes to fore. Alteration contained within ratio, limited by exponent as limit.',
};

const state2: DialecticState = {
  id: 'inverse-reciprocal-limiting',
  title: 'Reciprocal limiting: each contains other',
  concept: 'ReciprocalLimiting',
  phase: 'quantity',

  moments: [
    {
      name: 'twoMomentsLimit',
      definition: 'Two moments limit themselves inside exponent',
      type: 'determination',
    },
    {
      name: 'eachContainsOther',
      definition: 'Each continues into other negatively, contains it',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'twoMomentsLimit',
    },
    {
      name: 'magnitudeIndispensable',
      definition: 'Magnitude of other indispensable',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'inverse-2-inv-1',
      constraint: 'each = negativeOfOther',
      predicate: 'negativeOf(each, other)',
    },
    {
      id: 'inverse-2-inv-2',
      constraint: 'one.smaller ⇔ other.greater',
      predicate: 'iff(smaller(one), greater(other))',
    },
    {
      id: 'inverse-2-inv-3',
      constraint: 'each.magnitude = extentOtherLacksMagnitude',
      predicate: 'equals(each.magnitude, extent(other.lacks(magnitude)))',
    },
    {
      id: 'inverse-2-inv-4',
      constraint: 'each.proportionedTo = other',
      predicate: 'proportionedTo(each, other)',
    },
  ],

  forces: [
    {
      id: 'inverse-2-force-1',
      description: 'Reciprocal limiting drives toward bad infinity',
      type: 'contradiction',
      trigger: 'each.continuesIntoOther = negatively',
      effect: 'badInfinity.emerges = true',
      targetState: 'inverse-bad-infinity',
    },
  ],

  transitions: [
    {
      id: 'inverse-2-trans-1',
      from: 'inverse-reciprocal-limiting',
      to: 'inverse-bad-infinity',
      mechanism: 'contradiction',
      description: 'From reciprocal limiting to bad infinity',
    },
  ],

  nextStates: ['inverse-bad-infinity'],
  previousStates: ['inverse-sublated-direct'],

  provenance: {
    topicMapId: 'inverse-reciprocal-limiting',
    lineRange: { start: 102, end: 123 },
    section: 'B. THE INVERSE RATIO',
    order: 3,
  },

  description: 'Reciprocal limiting: Two moments limit themselves inside exponent. Each is negative of other. One becomes smaller as other becomes greater. Each possesses magnitude to extent it is magnitude other lacks. Each continues into other negatively. Each contains other, is proportioned to it - magnitude of other indispensable.',
};

const state3: DialecticState = {
  id: 'inverse-bad-infinity',
  title: 'Bad infinity: sides approximate exponent',
  concept: 'InverseBadInfinity',
  phase: 'quantity',

  moments: [
    {
      name: 'exponentAsInItself',
      definition: 'Unity (exponent) is in-itself of each',
      type: 'determination',
    },
    {
      name: 'contradiction',
      definition: 'In-itself vs moment - each loses determination making equal to in-itself',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'exponentAsInItself',
    },
    {
      name: 'approximation',
      definition: 'Sides approximate exponent but cannot attain (bad infinity)',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'inverse-3-inv-1',
      constraint: 'exponent = inItself(each)',
      predicate: 'equals(exponent, inItself(each))',
    },
    {
      id: 'inverse-3-inv-2',
      constraint: 'each.makingEqualToInItself = losesDetermination',
      predicate: 'implies(makingEqual(inItself), loses(determination))',
    },
    {
      id: 'inverse-3-inv-3',
      constraint: 'sides.approximate = exponent ∧ not(canAttain)',
      predicate: 'approximate(sides, exponent) ∧ not(canAttain(sides, exponent))',
    },
    {
      id: 'inverse-3-inv-4',
      constraint: 'infinite.affirmativelyPresent = simpleQuantumOfExponent',
      predicate: 'affirmativelyPresent(infinite, simpleQuantum(exponent))',
    },
  ],

  forces: [
    {
      id: 'inverse-3-force-1',
      description: 'Affirmatively present exponent drives transition to powers',
      type: 'mediation',
      trigger: 'exponent.affirmativelyPresent = true',
      effect: 'ratioOfPowers.emerges = true',
      targetState: 'inverse-transition-powers',
    },
  ],

  transitions: [
    {
      id: 'inverse-3-trans-1',
      from: 'inverse-bad-infinity',
      to: 'inverse-transition-powers',
      mechanism: 'mediation',
      description: 'From bad infinity to transition to powers',
    },
  ],

  nextStates: ['inverse-transition-powers'],
  previousStates: ['inverse-reciprocal-limiting'],

  provenance: {
    topicMapId: 'inverse-bad-infinity',
    lineRange: { start: 125, end: 192 },
    section: 'B. THE INVERSE RATIO',
    order: 4,
  },

  description: 'Bad infinity: Unity (exponent) is in-itself of each. Each can take from other only to make equal to in-itself (maximum in exponent). Each loses determination making equal to in-itself - contradiction (in-itself vs moment). Sides approximate but cannot attain (bad infinity). But infinite (exponent) is affirmatively present - simple quantum of exponent.',
};

const state4: DialecticState = {
  id: 'inverse-transition-powers',
  title: 'Transition to ratio of powers',
  concept: 'TransitionToPowers',
  phase: 'quantity',

  moments: [
    {
      name: 'exponentMediation',
      definition: 'Exponent fixity developed as mediation of itself with itself in its other',
      type: 'mediation',
    },
    {
      name: 'negationOfNegation',
      definition: 'Negation of negation posited (affirmative self-relation)',
      type: 'sublation',
      relation: 'mediates',
      relatedTo: 'exponentMediation',
    },
    {
      name: 'rejoining',
      definition: 'Qualitative being rejoins itself in externally existent otherness',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'inverse-4-inv-1',
      constraint: 'exponent.fixity = mediationWithItselfInOther',
      predicate: 'equals(exponent.fixity, mediationWithItself(other))',
    },
    {
      id: 'inverse-4-inv-2',
      constraint: 'negation(negation) = affirmativeSelfRelation',
      predicate: 'equals(negationOfNegation, affirmativeSelfRelation)',
    },
    {
      id: 'inverse-4-inv-3',
      constraint: 'exponent.enclosesImplicitly = momentsRatio',
      predicate: 'enclosesImplicitly(exponent, moments(ratio))',
    },
    {
      id: 'inverse-4-inv-4',
      constraint: 'qualitativeBeing.rejoins = externalOtherness',
      predicate: 'rejoins(qualitativeBeing, externalOtherness)',
    },
  ],

  forces: [
    {
      id: 'inverse-4-force-1',
      description: 'Rejoining in externality determines ratio as ratio of powers',
      type: 'passover',
      trigger: 'exponent.rejoinsInExternality = true',
      effect: 'ratioOfPowers.determined = true',
      targetState: 'powers-being-for-itself',
    },
  ],

  transitions: [
    {
      id: 'inverse-4-trans-1',
      from: 'inverse-transition-powers',
      to: 'powers-being-for-itself',
      mechanism: 'passover',
      description: 'From inverse ratio to ratio of powers',
    },
  ],

  nextStates: ['powers-being-for-itself'],
  previousStates: ['inverse-bad-infinity'],

  provenance: {
    topicMapId: 'inverse-transition-powers',
    lineRange: { start: 194, end: 268 },
    section: 'B. THE INVERSE RATIO',
    order: 5,
  },

  description: 'Transition to powers: Exponent fixity develops as mediation of itself with itself in its other (finite moments). Negation of negation posited (affirmative self-relation). Exponent encloses moments, refers itself to itself. Qualitative being rejoins itself in externally existent otherness. Exponent preserves itself in negation - determining factor in self-surpassing. Ratio determined as ratio of powers.',
};

export const inverseIR: DialecticIR = {
  id: 'inverse-ir',
  title: 'Inverse IR: The Inverse Ratio',
  section: 'B. THE INVERSE RATIO',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'inverse.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'inverse-sublated-direct': 'quantity',
      'inverse-reciprocal-limiting': 'quantity',
      'inverse-bad-infinity': 'quantity',
      'inverse-transition-powers': 'quantity',
    },
  },
};

export const inverseStates = {
  'inverse-sublated-direct': state1,
  'inverse-reciprocal-limiting': state2,
  'inverse-bad-infinity': state3,
  'inverse-transition-powers': state4,
};
