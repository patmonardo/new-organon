/**
 * Particular IR: Dialectic Pseudo-Code for Particular Concept
 *
 * Architecture: Knowledge Processor (GDSL → SDSL Integration)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 2. The Particular
 *
 * Covers the dialectical movement:
 * - Determinateness as particularity (universal's immanent moment)
 * - Particular contains universality (totality, completeness)
 * - Universal itself differentiated (self-differentiation)
 * - True logical division
 * - Understanding and abstract universal
 * - Transition to singularity
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'part-1',
  title: 'Determinateness as particularity — universal\'s immanent moment',
  concept: 'ParticularityAsImmanentMoment',
  phase: 'subject',

  moments: [
    {
      name: 'particularity',
      definition: 'Determinateness of concept, universal\'s own immanent moment',
      type: 'determination',
    },
    {
      name: 'notLimit',
      definition: 'Not a limit, universal is with itself in particularity',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'particularity',
    },
  ],

  invariants: [
    {
      id: 'part-1-inv-1',
      constraint: 'particularity = immanentMoment(universal)',
      predicate: 'equals(particularity, immanentMoment(universal))',
    },
    {
      id: 'part-1-inv-2',
      constraint: 'particularity ≠ limit',
      predicate: 'not(equals(particularity, limit))',
    },
  ],

  forces: [
    {
      id: 'part-1-force-1',
      description: 'Particularity drives toward totality',
      type: 'mediation',
      trigger: 'particularity.established = true',
      effect: 'totality.emerges = true',
      targetState: 'part-2',
    },
  ],

  transitions: [
    {
      id: 'part-1-trans-1',
      from: 'part-1',
      to: 'part-2',
      mechanism: 'mediation',
      description: 'From particularity to totality',
    },
  ],

  nextStates: ['part-2'],
  previousStates: ['universal-ir'],

  provenance: {
    topicMapId: 'part-1-determinateness-immanent',
    lineRange: { start: 2, end: 9 },
    section: '2. The Particular',
    order: 1,
  },

  description: 'Determinateness as particularity is universal\'s own immanent moment. Not a limit. Universal is with itself in particularity.',
};

const state2: DialecticState = {
  id: 'part-2',
  title: 'Particular contains universality — totality',
  concept: 'ParticularContainsUniversality',
  phase: 'subject',

  moments: [
    {
      name: 'containsUniversality',
      definition: 'Particular contains universality, genus unaltered in species',
      type: 'determination',
    },
    {
      name: 'totality',
      definition: 'Diversity of particulars is totality, completeness',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'containsUniversality',
    },
  ],

  invariants: [
    {
      id: 'part-2-inv-1',
      constraint: 'particular.contains = universality',
      predicate: 'contains(particular, universality)',
    },
    {
      id: 'part-2-inv-2',
      constraint: 'diversity = totality',
      predicate: 'equals(diversity, totality)',
    },
  ],

  forces: [
    {
      id: 'part-2-force-1',
      description: 'Totality drives toward self-differentiation',
      type: 'mediation',
      trigger: 'totality.established = true',
      effect: 'selfDifferentiation.emerges = true',
      targetState: 'part-3',
    },
  ],

  transitions: [
    {
      id: 'part-2-trans-1',
      from: 'part-2',
      to: 'part-3',
      mechanism: 'mediation',
      description: 'From totality to self-differentiation',
    },
  ],

  nextStates: ['part-3'],
  previousStates: ['part-1'],

  provenance: {
    topicMapId: 'part-2-contains-universality-totality',
    lineRange: { start: 11, end: 50 },
    section: '2. The Particular',
    order: 2,
  },

  description: 'Particular contains universality. Genus unaltered in species. Diversity of particulars is totality, completeness.',
};

const state3: DialecticState = {
  id: 'part-3',
  title: 'Universal itself differentiated — self-differentiation',
  concept: 'UniversalItselfDifferentiated',
  phase: 'subject',

  moments: [
    {
      name: 'selfDifferentiation',
      definition: 'Universal determines itself, differentiated from itself',
      type: 'process',
    },
    {
      name: 'species',
      definition: 'Species are: (a) universal itself, (b) particular',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'selfDifferentiation',
    },
  ],

  invariants: [
    {
      id: 'part-3-inv-1',
      constraint: 'universal.determinesItself = true',
      predicate: 'determinesItself(universal)',
    },
    {
      id: 'part-3-inv-2',
      constraint: 'universal.differentiatedFromItself = true',
      predicate: 'differentiatedFromItself(universal)',
    },
  ],

  forces: [
    {
      id: 'part-3-force-1',
      description: 'Self-differentiation drives toward true logical division',
      type: 'mediation',
      trigger: 'selfDifferentiation.complete = true',
      effect: 'logicalDivision.emerges = true',
      targetState: 'part-4',
    },
  ],

  transitions: [
    {
      id: 'part-3-trans-1',
      from: 'part-3',
      to: 'part-4',
      mechanism: 'mediation',
      description: 'From self-differentiation to logical division',
    },
  ],

  nextStates: ['part-4'],
  previousStates: ['part-2'],

  provenance: {
    topicMapId: 'part-3-universal-itself-differentiation',
    lineRange: { start: 52, end: 71 },
    section: '2. The Particular',
    order: 3,
  },

  description: 'Universal determines itself, differentiated from itself. Species are: universal itself and particular.',
};

const state4: DialecticState = {
  id: 'part-4',
  title: 'True logical division — two particulars',
  concept: 'TrueLogicalDivision',
  phase: 'subject',

  moments: [
    {
      name: 'logicalDivision',
      definition: 'Universal/Particular as two particulars, coordinated and subordinated',
      type: 'determination',
    },
    {
      name: 'oneDeterminateness',
      definition: 'Determinateness is essentially only one determinateness, negativity',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'logicalDivision',
    },
  ],

  invariants: [
    {
      id: 'part-4-inv-1',
      constraint: 'universal = particular',
      predicate: 'equals(universal, particular)',
    },
    {
      id: 'part-4-inv-2',
      constraint: 'determinateness = one',
      predicate: 'equals(determinateness, one)',
    },
  ],

  forces: [
    {
      id: 'part-4-force-1',
      description: 'Logical division drives toward understanding',
      type: 'mediation',
      trigger: 'logicalDivision.established = true',
      effect: 'understanding.emerges = true',
      targetState: 'part-10',
    },
  ],

  transitions: [
    {
      id: 'part-4-trans-1',
      from: 'part-4',
      to: 'part-10',
      mechanism: 'mediation',
      description: 'From logical division to understanding',
    },
  ],

  nextStates: ['part-10'],
  previousStates: ['part-3'],

  provenance: {
    topicMapId: 'part-4-true-logical-division',
    lineRange: { start: 73, end: 95 },
    section: '2. The Particular',
    order: 4,
  },

  description: 'True logical division: Universal/Particular as two particulars. Determinateness is essentially one determinateness, negativity.',
};

const state5: DialecticState = {
  id: 'part-10',
  title: 'Understanding and abstract universal — dialectical force',
  concept: 'UnderstandingAndAbstractUniversal',
  phase: 'subject',

  moments: [
    {
      name: 'understanding',
      definition: 'Fixity, infinite force of splitting concrete into abstract determinacies',
      type: 'determination',
    },
    {
      name: 'dialecticalForce',
      definition: 'Understanding quickens with spirit, capacity to dissolve themselves',
      type: 'process',
      relation: 'contains',
      relatedTo: 'understanding',
    },
    {
      name: 'appearanceOfReason',
      definition: 'Determinate concept is essential moment of reason, beginning of appearance',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'part-10-inv-1',
      constraint: 'understanding = fixity',
      predicate: 'equals(understanding, fixity)',
    },
    {
      id: 'part-10-inv-2',
      constraint: 'understanding.infiniteForce = true',
      predicate: 'hasInfiniteForce(understanding)',
    },
  ],

  forces: [
    {
      id: 'part-10-force-1',
      description: 'Understanding drives toward transition to singularity',
      type: 'mediation',
      trigger: 'dialecticalForce.established = true',
      effect: 'singularity.emerges = true',
      targetState: 'part-12',
    },
  ],

  transitions: [
    {
      id: 'part-10-trans-1',
      from: 'part-10',
      to: 'part-12',
      mechanism: 'mediation',
      description: 'From understanding to transition to singularity',
    },
  ],

  nextStates: ['part-12'],
  previousStates: ['part-4'],

  provenance: {
    topicMapId: 'part-10-understanding-force-fixity',
    lineRange: { start: 293, end: 412 },
    section: '2. The Particular',
    order: 5,
  },

  description: 'Understanding is fixity, infinite force of splitting concrete. Dialectical force quickens with spirit. Determinate concept is beginning of appearance of reason.',
};

const state6: DialecticState = {
  id: 'part-12',
  title: 'Transition to singularity — absolute turning back',
  concept: 'TransitionToSingularity',
  phase: 'subject',

  moments: [
    {
      name: 'determinateUniversality',
      definition: 'Self-referring determinateness, absolute negativity posited for itself',
      type: 'determination',
    },
    {
      name: 'singularity',
      definition: 'Third moment of concept, absolute turning back into itself',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'determinateUniversality',
    },
    {
      name: 'positedLoss',
      definition: 'At same time, posited loss of itself',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'part-12-inv-1',
      constraint: 'determinateUniversality = selfReferringDeterminateness',
      predicate: 'equals(determinateUniversality, selfReferringDeterminateness)',
    },
    {
      id: 'part-12-inv-2',
      constraint: 'singularity = absoluteTurningBack',
      predicate: 'equals(singularity, absoluteTurningBack)',
    },
  ],

  forces: [
    {
      id: 'part-12-force-1',
      description: 'Transition drives toward singularity',
      type: 'passover',
      trigger: 'determinateUniversality.established = true',
      effect: 'singularity.emerges = true',
      targetState: 'sing-1',
    },
  ],

  transitions: [
    {
      id: 'part-12-trans-1',
      from: 'part-12',
      to: 'sing-1',
      mechanism: 'passover',
      description: 'From particularity to singularity',
    },
  ],

  nextStates: ['sing-1'],
  previousStates: ['part-10'],

  provenance: {
    topicMapId: 'part-12-transition-singularity',
    lineRange: { start: 428, end: 453 },
    section: '2. The Particular',
    order: 6,
  },

  description: 'Determinate universality is self-referring determinateness, absolute negativity. Singularity is third moment, absolute turning back. Posited loss of itself.',
};

export const particularIR: DialecticIR = {
  id: 'particular-ir',
  title: 'Particular IR: Immanent Moment, Totality, Understanding, Transition',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 2. The Particular',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'particular.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'part-1': 'subject',
      'part-2': 'subject',
      'part-3': 'subject',
      'part-4': 'subject',
      'part-10': 'subject',
      'part-12': 'subject',
    },
  },
};

export const particularStates = {
  'part-1': state1,
  'part-2': state2,
  'part-3': state3,
  'part-4': state4,
  'part-10': state5,
  'part-12': state6,
};
