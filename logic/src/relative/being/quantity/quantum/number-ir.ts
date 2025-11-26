/**
 * Number IR: Dialectic Pseudo-Code for Number
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: A. NUMBER
 *
 * Covers the dialectical movement:
 * - Quantum as number (complete positedness)
 * - Amount and unit (two moments of number)
 * - Amount in the limit (many ones present)
 * - Number indifference and exteriority (intrinsic contradiction)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'number-quantum-as-number',
  title: 'Quantum as number: complete positedness',
  concept: 'QuantumAsNumber',
  phase: 'quantity',

  moments: [
    {
      name: 'number',
      definition: 'Complete positedness with limit as plurality distinguished from unity',
      type: 'determination',
    },
    {
      name: 'oneAsPrinciple',
      definition: 'The one as principle: self-referring, enclosing, other-excluding',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'number',
    },
    {
      name: 'discreteMagnitudeWithContinuity',
      definition: 'Discrete magnitude with continuity in unity',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'number-1-inv-1',
      constraint: 'number = completePositedness',
      predicate: 'is(number, completePositedness)',
    },
    {
      id: 'number-1-inv-2',
      constraint: 'limit.as = pluralityDistinguishedFromUnity',
      predicate: 'as(limit, pluralityDistinguishedFromUnity)',
    },
    {
      id: 'number-1-inv-3',
      constraint: 'one.principle = {selfReferring, enclosing, otherExcluding}',
      predicate: 'principle(one, selfReferring) ∧ principle(one, enclosing) ∧ principle(one, otherExcluding)',
    },
    {
      id: 'number-1-inv-4',
      constraint: 'number = discreteMagnitude(continuityInUnity)',
      predicate: 'equals(number, discreteMagnitude(continuityInUnity))',
    },
  ],

  forces: [
    {
      id: 'number-1-force-1',
      description: 'Complete positedness drives toward explicit moments: amount and unit',
      type: 'reflection',
      trigger: 'completePositedness.achieved = true',
      effect: 'moments.emerge = {amount, unit}',
      targetState: 'number-amount-and-unit',
    },
  ],

  transitions: [
    {
      id: 'number-1-trans-1',
      from: 'number-quantum-as-number',
      to: 'number-amount-and-unit',
      mechanism: 'reflection',
      description: 'From quantum as number to amount and unit as moments',
    },
  ],

  nextStates: ['number-amount-and-unit'],
  previousStates: ['quantity-limiting-quantum'],

  provenance: {
    topicMapId: 'number-quantum-as-number',
    lineRange: { start: 4, end: 52 },
    section: 'A. NUMBER',
    order: 1,
  },

  description: 'Quantum as number: Complete positedness with limit as plurality distinguished from unity. Number is discrete magnitude with continuity in unity. The one is principle: (a) self-referring, (b) enclosing, (c) other-excluding limit.',
};

const state2: DialecticState = {
  id: 'number-amount-and-unit',
  title: 'Amount and unit: the two moments of number',
  concept: 'AmountAndUnit',
  phase: 'quantity',

  moments: [
    {
      name: 'amount',
      definition: 'The how many times, determinate aggregate',
      type: 'moment',
    },
    {
      name: 'unit',
      definition: 'The continuity of the amount',
      type: 'moment',
      relation: 'mediates',
      relatedTo: 'amount',
    },
    {
      name: 'twoMomentsOfNumber',
      definition: 'Amount and unit constitute the moments of number',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'number-2-inv-1',
      constraint: 'limit.manifold = true',
      predicate: 'manifold(limit)',
    },
    {
      id: 'number-2-inv-2',
      constraint: 'limit.contains = manyOnesAsDeterminateAggregate',
      predicate: 'contains(limit, manyOnes, determinateAggregate)',
    },
    {
      id: 'number-2-inv-3',
      constraint: 'amount = howManyTimes',
      predicate: 'is(amount, howManyTimes)',
    },
    {
      id: 'number-2-inv-4',
      constraint: 'unit = continuityOfAmount',
      predicate: 'is(unit, continuityOfAmount)',
    },
  ],

  forces: [
    {
      id: 'number-2-force-1',
      description: 'Two moments drive toward amount in the limit',
      type: 'mediation',
      trigger: 'momentsExplicit = true',
      effect: 'amountInLimit.emerges = true',
      targetState: 'number-amount-in-limit',
    },
  ],

  transitions: [
    {
      id: 'number-2-trans-1',
      from: 'number-amount-and-unit',
      to: 'number-amount-in-limit',
      mechanism: 'mediation',
      description: 'From abstract moments to amount in the limit',
    },
  ],

  nextStates: ['number-amount-in-limit'],
  previousStates: ['number-quantum-as-number'],

  provenance: {
    topicMapId: 'number-amount-and-unit',
    lineRange: { start: 53, end: 65 },
    section: 'A. NUMBER',
    order: 2,
  },

  description: 'Amount and unit: Number limit is manifold, containing many ones as determinate aggregate. Amount is the how many times; unit is the continuity of the amount. These are the two moments of number.',
};

const state3: DialecticState = {
  id: 'number-amount-in-limit',
  title: 'Amount in the limit: many ones present',
  concept: 'AmountInLimit',
  phase: 'quantity',

  moments: [
    {
      name: 'manyOnesPresent',
      definition: 'Many ones are present (not sublated), posited with excluding limit',
      type: 'determination',
    },
    {
      name: 'allOnesEqual',
      definition: 'All ones are equal; each is the hundredth',
      type: 'quality',
    },
    {
      name: 'manyConstituteDelimitation',
      definition: 'The many constitute the delimitation itself',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'manyOnesPresent',
    },
  ],

  invariants: [
    {
      id: 'number-3-inv-1',
      constraint: 'manyOnes.present = true ∧ manyOnes.sublated = false',
      predicate: 'present(manyOnes) ∧ not(sublated(manyOnes))',
    },
    {
      id: 'number-3-inv-2',
      constraint: 'all(ones).equal = true',
      predicate: 'all(ones, equal)',
    },
    {
      id: 'number-3-inv-3',
      constraint: 'each(one) = hundredth',
      predicate: 'equals(each(one), hundredth)',
    },
    {
      id: 'number-3-inv-4',
      constraint: 'many.constitute = delimitation',
      predicate: 'constitute(many, delimitation)',
    },
  ],

  forces: [
    {
      id: 'number-3-force-1',
      description: 'Delimitation not external drives toward intrinsic contradiction',
      type: 'contradiction',
      trigger: 'delimitation.internal = true',
      effect: 'intrinsicContradiction.emerges = true',
      targetState: 'number-indifference-and-exteriority',
    },
  ],

  transitions: [
    {
      id: 'number-3-trans-1',
      from: 'number-amount-in-limit',
      to: 'number-indifference-and-exteriority',
      mechanism: 'contradiction',
      description: 'From amount in limit to intrinsic contradiction',
    },
  ],

  nextStates: ['number-indifference-and-exteriority'],
  previousStates: ['number-amount-and-unit'],

  provenance: {
    topicMapId: 'number-amount-in-limit',
    lineRange: { start: 67, end: 109 },
    section: 'A. NUMBER',
    order: 3,
  },

  description: 'Amount in limit: Many ones are present (not sublated), posited with excluding limit. All ones are equal; each is the hundredth. The many constitute the delimitation itself - the number is not plurality over against limiting one, but is the delimitation.',
};

const state4: DialecticState = {
  id: 'number-indifference-and-exteriority',
  title: 'Number indifference and exteriority',
  concept: 'NumberContradiction',
  phase: 'quantity',

  moments: [
    {
      name: 'indifference',
      definition: 'Number is indifferent to others, essential determination',
      type: 'quality',
    },
    {
      name: 'exteriority',
      definition: 'Reference to other remains completely external',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'indifference',
    },
    {
      name: 'intrinsicContradiction',
      definition: 'Absolute exteriority is in the one itself',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'number-4-inv-1',
      constraint: 'distinguishing.quantitative = true',
      predicate: 'quantitative(distinguishing)',
    },
    {
      id: 'number-4-inv-2',
      constraint: 'number.indifferent = others',
      predicate: 'indifferent(number, others)',
    },
    {
      id: 'number-4-inv-3',
      constraint: 'referenceToOther.external = completely',
      predicate: 'external(referenceToOther, completely)',
    },
    {
      id: 'number-4-inv-4',
      constraint: 'absoluteExteriority ∈ oneItself',
      predicate: 'in(absoluteExteriority, oneItself)',
    },
    {
      id: 'number-4-inv-5',
      constraint: 'intrinsicContradiction = qualityOfQuantum',
      predicate: 'equals(intrinsicContradiction, qualityOfQuantum)',
    },
  ],

  forces: [
    {
      id: 'number-4-force-1',
      description: 'Intrinsic contradiction prepares transition to extensive/intensive quantum',
      type: 'passover',
      trigger: 'contradiction.develops = true',
      effect: 'extensiveIntensiveQuantum.emerges = true',
      targetState: 'quantum-extensive-magnitude',
    },
  ],

  transitions: [
    {
      id: 'number-4-trans-1',
      from: 'number-indifference-and-exteriority',
      to: 'quantum-extensive-magnitude',
      mechanism: 'passover',
      description: 'From number contradiction to extensive/intensive quantum',
    },
  ],

  nextStates: ['quantum-extensive-magnitude'],
  previousStates: ['number-amount-in-limit'],

  provenance: {
    topicMapId: 'number-indifference-and-exteriority',
    lineRange: { start: 111, end: 136 },
    section: 'A. NUMBER',
    order: 4,
  },

  description: 'Number indifference/exteriority: Distinguishing remains quantitative (external reflection). Number is absolutely determined but has simple immediacy; reference to other is external. Yet amount is plurality of ones - absolute exteriority is in the one itself. This intrinsic contradiction is the quality of quantum.',
};

export const numberIR: DialecticIR = {
  id: 'number-ir',
  title: 'Number IR: Complete Positedness, Amount and Unit',
  section: 'A. NUMBER',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'number.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'number-quantum-as-number': 'quantity',
      'number-amount-and-unit': 'quantity',
      'number-amount-in-limit': 'quantity',
      'number-indifference-and-exteriority': 'quantity',
    },
  },
};

export const numberStates = {
  'number-quantum-as-number': state1,
  'number-amount-and-unit': state2,
  'number-amount-in-limit': state3,
  'number-indifference-and-exteriority': state4,
};
