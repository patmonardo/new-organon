/**
 * Powers IR: Dialectic Pseudo-Code for The Ratio of Powers
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: C. THE RATIO OF POWERS
 *
 * Covers the dialectical movement:
 * - Quantum as being-for-itself (unit is amount)
 * - Exponent entirely qualitative
 * - Quantum concept realized
 * - Quantum returns to quality (quantity is quality itself)
 * - Double transition and measure
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'powers-being-for-itself',
  title: 'Quantum as being-for-itself: unit is amount',
  concept: 'QuantumBeingForItself',
  phase: 'quantity',

  moments: [
    {
      name: 'beingForItself',
      definition: 'Quantum self-identical in otherness (being-for-itself)',
      type: 'determination',
    },
    {
      name: 'unitIsAmount',
      definition: 'Amount determined only by unit; unit is amount as against itself',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'beingForItself',
    },
    {
      name: 'power',
      definition: 'Power is aggregate of units, each is aggregate itself',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'powers-1-inv-1',
      constraint: 'quantum.selfIdentical = otherness',
      predicate: 'selfIdentical(quantum, otherness)',
    },
    {
      id: 'powers-1-inv-2',
      constraint: 'amount.determinedBy = unit',
      predicate: 'determinedBy(amount, unit)',
    },
    {
      id: 'powers-1-inv-3',
      constraint: 'unit.asAgainstItself = amount',
      predicate: 'equals(unit.asAgainstItself, amount)',
    },
    {
      id: 'powers-1-inv-4',
      constraint: 'quantum.otherness = determinedByQuantumItself',
      predicate: 'determinedBy(quantum.otherness, quantumItself)',
    },
  ],

  forces: [
    {
      id: 'powers-1-force-1',
      description: 'Being-for-itself drives toward exponent as entirely qualitative',
      type: 'reflection',
      trigger: 'beingForItself.achieved = true',
      effect: 'exponentQualitative.emerges = true',
      targetState: 'powers-exponent-qualitative',
    },
  ],

  transitions: [
    {
      id: 'powers-1-trans-1',
      from: 'powers-being-for-itself',
      to: 'powers-exponent-qualitative',
      mechanism: 'reflection',
      description: 'From being-for-itself to exponent as qualitative',
    },
  ],

  nextStates: ['powers-exponent-qualitative'],
  previousStates: ['inverse-transition-powers'],

  provenance: {
    topicMapId: 'powers-being-for-itself',
    lineRange: { start: 4, end: 24 },
    section: 'C. THE RATIO OF POWERS',
    order: 1,
  },

  description: 'Ratio of powers: Quantum as being-for-itself (self-identical in otherness). Amount determined only by unit. Unit is amount as against itself as unit. Otherness (amount of units) is unit itself. Power is aggregate of units, each is aggregate itself. Quantum otherness determined purely by itself. Quantum returned into itself - immediately itself and its otherness.',
};

const state2: DialecticState = {
  id: 'powers-exponent-qualitative',
  title: 'Exponent: entirely qualitative nature',
  concept: 'ExponentQualitative',
  phase: 'quantity',

  moments: [
    {
      name: 'entirelyQualitative',
      definition: 'Exponent no longer immediate quantum, entirely qualitative',
      type: 'quality',
    },
    {
      name: 'simpleDeterminateness',
      definition: 'Amount is unit itself, quantum self-identical in otherness',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'entirelyQualitative',
    },
    {
      name: 'existenceContinuesInOtherness',
      definition: 'Existence posited as continuing in otherness',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'powers-2-inv-1',
      constraint: 'exponent.notImmediate = true',
      predicate: 'not(immediate(exponent))',
    },
    {
      id: 'powers-2-inv-2',
      constraint: 'exponent.nature = entirelyQualitative',
      predicate: 'is(exponent.nature, entirelyQualitative)',
    },
    {
      id: 'powers-2-inv-3',
      constraint: 'limit.notImmediate = true',
      predicate: 'not(immediate(limit))',
    },
    {
      id: 'powers-2-inv-4',
      constraint: 'truthOfQuality = quantity',
      predicate: 'equals(truthOf(quality), quantity)',
    },
  ],

  forces: [
    {
      id: 'powers-2-force-1',
      description: 'Entirely qualitative exponent drives toward concept realized',
      type: 'mediation',
      trigger: 'exponent.entirelyQualitative = true',
      effect: 'conceptRealized.emerges = true',
      targetState: 'powers-concept-realized',
    },
  ],

  transitions: [
    {
      id: 'powers-2-trans-1',
      from: 'powers-exponent-qualitative',
      to: 'powers-concept-realized',
      mechanism: 'mediation',
      description: 'From exponent qualitative to concept realized',
    },
  ],

  nextStates: ['powers-concept-realized'],
  previousStates: ['powers-being-for-itself'],

  provenance: {
    topicMapId: 'powers-exponent-qualitative',
    lineRange: { start: 26, end: 38 },
    section: 'C. THE RATIO OF POWERS',
    order: 2,
  },

  description: 'Exponent: No longer immediate quantum. Entirely qualitative nature - simple determinateness that amount is unit itself, quantum self-identical in otherness. Quantitative side: limit/negation not immediate existent, existence continues in otherness. Truth of quality is quantity (immediate determinateness as sublated).',
};

const state3: DialecticState = {
  id: 'powers-concept-realized',
  title: 'Quantum concept realized',
  concept: 'ConceptRealized',
  phase: 'quantity',

  moments: [
    {
      name: 'conceptAttained',
      definition: 'Quantum has attained/realized its concept',
      type: 'determination',
    },
    {
      name: 'indifferentDeterminenessSublated',
      definition: 'Indifferent determinateness posited as sublated',
      type: 'sublation',
      relation: 'mediates',
      relatedTo: 'conceptAttained',
    },
    {
      name: 'limitThatIsNoLimit',
      definition: 'Limit that is no limit, continues in otherness',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'powers-3-inv-1',
      constraint: 'quantum.attainedConcept = true',
      predicate: 'attained(quantum, concept)',
    },
    {
      id: 'powers-3-inv-2',
      constraint: 'indifferentDeterminateness.sublated = true',
      predicate: 'sublated(indifferentDeterminateness)',
    },
    {
      id: 'powers-3-inv-3',
      constraint: 'limit.continuesInOtherness = true',
      predicate: 'continuesIn(limit, otherness)',
    },
    {
      id: 'powers-3-inv-4',
      constraint: 'quantum.remainsIdentical = true',
      predicate: 'remainsIdentical(quantum)',
    },
  ],

  forces: [
    {
      id: 'powers-3-force-1',
      description: 'Concept realized drives toward return to quality',
      type: 'passover',
      trigger: 'concept.realized = true',
      effect: 'returnToQuality.emerges = true',
      targetState: 'powers-return-quality',
    },
  ],

  transitions: [
    {
      id: 'powers-3-trans-1',
      from: 'powers-concept-realized',
      to: 'powers-return-quality',
      mechanism: 'passover',
      description: 'From concept realized to return to quality',
    },
  ],

  nextStates: ['powers-return-quality'],
  previousStates: ['powers-exponent-qualitative'],

  provenance: {
    topicMapId: 'powers-concept-realized',
    lineRange: { start: 40, end: 58 },
    section: 'C. THE RATIO OF POWERS',
    order: 3,
  },

  description: 'Concept realized: Ratio of powers displays what quantum is implicitly. Quantum has attained/realized its concept. Quantum is indifferent determinateness posited as sublated - limit that is no limit, continues in otherness, remains identical. Quantum otherness (surpassing into another quantum) is determined through quantum itself.',
};

const state4: DialecticState = {
  id: 'powers-return-quality',
  title: 'Quantum returns to quality: quantity is quality itself',
  concept: 'QuantityIsQuality',
  phase: 'quantity',

  moments: [
    {
      name: 'quantumBecomeQuality',
      definition: 'Quantum has become other of itself (quality)',
      type: 'quality',
    },
    {
      name: 'externalityMediated',
      definition: 'Externality now mediated by quantum itself, as moment',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'quantumBecomeQuality',
    },
    {
      name: 'quantityIsQuality',
      definition: 'Quantity is quality itself, truth of quality is quantity',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'powers-4-inv-1',
      constraint: 'quantum.become = quality',
      predicate: 'become(quantum, quality)',
    },
    {
      id: 'powers-4-inv-2',
      constraint: 'externality.mediatedBy = quantumItself',
      predicate: 'mediatedBy(externality, quantumItself)',
    },
    {
      id: 'powers-4-inv-3',
      constraint: 'quantum.refersToItselfInExternality = true',
      predicate: 'refersToItself(quantum, externality)',
    },
    {
      id: 'powers-4-inv-4',
      constraint: 'quantity = quality',
      predicate: 'is(quantity, quality)',
    },
  ],

  forces: [
    {
      id: 'powers-4-force-1',
      description: 'Quantity as quality drives toward double transition and measure',
      type: 'sublation',
      trigger: 'quantity.isQuality = true',
      effect: 'measure.emerges = true',
      targetState: 'powers-double-transition-measure',
    },
  ],

  transitions: [
    {
      id: 'powers-4-trans-1',
      from: 'powers-return-quality',
      to: 'powers-double-transition-measure',
      mechanism: 'sublation',
      description: 'From quantity as quality to double transition and measure',
    },
  ],

  nextStates: ['powers-double-transition-measure'],
  previousStates: ['powers-concept-realized'],

  provenance: {
    topicMapId: 'powers-return-quality',
    lineRange: { start: 85, end: 115 },
    section: 'C. THE RATIO OF POWERS',
    order: 5,
  },

  description: 'Return to quality: Quantum passed over into another determination - determination now also as determinateness, in-itself as existence. Quantum has become other of itself (quality). Externality now mediated by quantum itself, as moment - in externality quantum refers itself to itself, is being as quality. Quantity is quality itself.',
};

const state5: DialecticState = {
  id: 'powers-double-transition-measure',
  title: 'Double transition and measure',
  concept: 'Measure',
  phase: 'quantity',

  moments: [
    {
      name: 'doubleTransition',
      definition: 'Quality → quantity, quantity → quality',
      type: 'mediation',
    },
    {
      name: 'totality',
      definition: 'For totality, double transition required',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'doubleTransition',
    },
    {
      name: 'measure',
      definition: 'Truth of quantum is to be measure',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'powers-5-inv-1',
      constraint: 'totality.requires = doubleTransition',
      predicate: 'requires(totality, doubleTransition)',
    },
    {
      id: 'powers-5-inv-2',
      constraint: 'transition1 = quality → quantity',
      predicate: 'transition(quality, quantity)',
    },
    {
      id: 'powers-5-inv-3',
      constraint: 'transition2 = quantity → quality',
      predicate: 'transition(quantity, quality)',
    },
    {
      id: 'powers-5-inv-4',
      constraint: 'truthOfQuantum = measure',
      predicate: 'equals(truthOf(quantum), measure)',
    },
    {
      id: 'powers-5-inv-5',
      constraint: 'quantum.noLongerIndifferent = sublated',
      predicate: 'noLonger(indifferent(quantum)) ∧ sublated(quantum)',
    },
  ],

  forces: [
    {
      id: 'powers-5-force-1',
      description: 'Measure as truth of quantum completes Quantity phase',
      type: 'sublation',
      trigger: 'measure.realized = true',
      effect: 'quantityPhaseComplete = true',
      targetState: 'measure-1',
    },
  ],

  transitions: [
    {
      id: 'powers-5-trans-1',
      from: 'powers-double-transition-measure',
      to: 'measure-1',
      mechanism: 'sublation',
      description: 'From ratio of powers to measure',
    },
  ],

  nextStates: ['measure-1'],
  previousStates: ['powers-return-quality'],

  provenance: {
    topicMapId: 'powers-double-transition-measure',
    lineRange: { start: 117, end: 140 },
    section: 'C. THE RATIO OF POWERS',
    order: 6,
  },

  description: 'Double transition: For totality, double transition required - (1) quality → quantity, (2) quantity → quality. First: identity in itself (quality contained in quantity). Second: quantity contained in quality (as sublated). Quantum no longer indifferent but sublated - it is quality and that by virtue of which anything is what it is. Truth of quantum is to be measure.',
};

export const powersIR: DialecticIR = {
  id: 'powers-ir',
  title: 'Powers IR: The Ratio of Powers and Transition to Measure',
  section: 'C. THE RATIO OF POWERS',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'powers.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'powers-being-for-itself': 'quantity',
      'powers-exponent-qualitative': 'quantity',
      'powers-concept-realized': 'quantity',
      'powers-return-quality': 'quantity',
      'powers-double-transition-measure': 'quantity',
    },
  },
};

export const powersStates = {
  'powers-being-for-itself': state1,
  'powers-exponent-qualitative': state2,
  'powers-concept-realized': state3,
  'powers-return-quality': state4,
  'powers-double-transition-measure': state5,
};
