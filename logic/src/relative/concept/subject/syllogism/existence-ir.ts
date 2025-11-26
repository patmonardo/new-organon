/**
 * Existence Syllogism IR: Dialectic Pseudo-Code for Syllogism of Existence
 *
 * Architecture: CPU Inference Engine (Representation / Disambiguation)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - A. Syllogism of Existence
 *
 * Covers the dialectical movement:
 * - First Figure (S-P-U): Singular emerges into existence through particularity
 * - Second Figure (P-S-U): Negative unity, singular as middle
 * - Third Figure (S-U-P): Reciprocal mediation, abstract universal
 * - Fourth Figure (U-U-U): Mathematical syllogism, quantitative equality
 * - Transition to Syllogism of Reflection
 *
 * The CPU's inference engine - holds representation, disambiguates GPU processes
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'syl-exist-1',
  title: 'First figure S-P-U — singular emerges into existence',
  concept: 'FirstFigureSPU',
  phase: 'subject',

  moments: [
    {
      name: 'SPUSchema',
      definition: 'S-P-U general schema, particularity as middle term',
      type: 'determination',
    },
    {
      name: 'singularEmerges',
      definition: 'Singular emerges through particularity into existence (universality)',
      type: 'process',
      relation: 'mediates',
      relatedTo: 'SPUSchema',
    },
    {
      name: 'allThingsAreSyllogism',
      definition: 'All things are syllogism - universal united through particularity with singularity',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'syl-exist-1-inv-1',
      constraint: 'middle = particularity',
      predicate: 'equals(middle, particularity)',
    },
    {
      id: 'syl-exist-1-inv-2',
      constraint: 'singular.connects(universal) = via(particularity)',
      predicate: 'connects(singular, universal, particularity)',
    },
  ],

  forces: [
    {
      id: 'syl-exist-1-force-1',
      description: 'First figure drives toward second figure',
      type: 'negation',
      trigger: 'mediation.contingent = true',
      effect: 'secondFigure.emerges = true',
      targetState: 'syl-exist-14',
    },
  ],

  transitions: [
    {
      id: 'syl-exist-1-trans-1',
      from: 'syl-exist-1',
      to: 'syl-exist-14',
      mechanism: 'negation',
      description: 'From first to second figure',
    },
  ],

  nextStates: ['syl-exist-14'],
  previousStates: ['concept-judgment-ir'],

  provenance: {
    topicMapId: 'syl-exist-1-introduction-immediate',
    lineRange: { start: 4, end: 210 },
    section: 'First Figure',
    order: 1,
  },

  description: 'S-P-U schema. Singular emerges through particularity into existence. All things are syllogism. Premises need proof (infinite progression, bad infinity). Contingency and contradiction.',
};

const state2: DialecticState = {
  id: 'syl-exist-14',
  title: 'Second figure P-S-U — negative unity, singular as middle',
  concept: 'SecondFigurePSU',
  phase: 'subject',

  moments: [
    {
      name: 'PSUSchema',
      definition: 'P-S-U, singular as middle term',
      type: 'determination',
    },
    {
      name: 'negativeUnity',
      definition: 'Mediation contains negative moment, sublation of immediacy',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'PSUSchema',
    },
    {
      name: 'particularConclusion',
      definition: 'Conclusion can only be particular (positive and negative)',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'syl-exist-14-inv-1',
      constraint: 'middle = singularity',
      predicate: 'equals(middle, singularity)',
    },
    {
      id: 'syl-exist-14-inv-2',
      constraint: 'mediation = negativeUnity',
      predicate: 'equals(mediation, negativeUnity)',
    },
  ],

  forces: [
    {
      id: 'syl-exist-14-force-1',
      description: 'Second figure drives toward third figure',
      type: 'mediation',
      trigger: 'singularity.selfSublating = true',
      effect: 'thirdFigure.emerges = true',
      targetState: 'syl-exist-20',
    },
  ],

  transitions: [
    {
      id: 'syl-exist-14-trans-1',
      from: 'syl-exist-14',
      to: 'syl-exist-20',
      mechanism: 'mediation',
      description: 'From second to third figure',
    },
  ],

  nextStates: ['syl-exist-20'],
  previousStates: ['syl-exist-1'],

  provenance: {
    topicMapId: 'syl-exist-14-second-figure',
    lineRange: { start: 486, end: 670 },
    section: 'Second Figure',
    order: 2,
  },

  description: 'P-S-U, singular as middle. Negative unity, sublation of immediacy. Conclusion necessarily particular. Points beyond to mediation by means of universal.',
};

const state3: DialecticState = {
  id: 'syl-exist-20',
  title: 'Third figure S-U-P — reciprocal mediation, abstract universal',
  concept: 'ThirdFigureSUP',
  phase: 'subject',

  moments: [
    {
      name: 'SUPSchema',
      definition: 'S-U-P, universal as middle term',
      type: 'determination',
    },
    {
      name: 'reciprocalMediation',
      definition: 'Each mediation but not totality, reciprocal presupposing',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'SUPSchema',
    },
    {
      name: 'abstractUniversal',
      definition: 'Middle is abstract universal, extremes not contained according essential determinateness',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'syl-exist-20-inv-1',
      constraint: 'middle = universality',
      predicate: 'equals(middle, universality)',
    },
    {
      id: 'syl-exist-20-inv-2',
      constraint: 'universal.abstract = true',
      predicate: 'isAbstract(universal)',
    },
  ],

  forces: [
    {
      id: 'syl-exist-20-force-1',
      description: 'Third figure drives toward fourth figure',
      type: 'negation',
      trigger: 'universal.abstract = true',
      effect: 'fourthFigure.emerges = true',
      targetState: 'syl-exist-23',
    },
  ],

  transitions: [
    {
      id: 'syl-exist-20-trans-1',
      from: 'syl-exist-20',
      to: 'syl-exist-23',
      mechanism: 'negation',
      description: 'From third to fourth figure',
    },
  ],

  nextStates: ['syl-exist-23'],
  previousStates: ['syl-exist-14'],

  provenance: {
    topicMapId: 'syl-exist-20-third-figure',
    lineRange: { start: 674, end: 783 },
    section: 'Third Figure',
    order: 3,
  },

  description: 'S-U-P, universal as middle. Reciprocal mediation, truth of formal syllogism. Middle is abstract universal. Conclusion necessarily negative. Formalism.',
};

const state4: DialecticState = {
  id: 'syl-exist-23',
  title: 'Fourth figure U-U-U — mathematical syllogism',
  concept: 'FourthFigureUUU',
  phase: 'subject',

  moments: [
    {
      name: 'UUUSchema',
      definition: 'U-U-U, mathematical syllogism - if two equal to third, then equal to each other',
      type: 'determination',
    },
    {
      name: 'quantitativeEquality',
      definition: 'Abstracts from qualitative, only quantitative equality',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'UUUSchema',
    },
    {
      name: 'noConceptualComprehension',
      definition: 'No conceptual comprehension, self-evidence rests on indigence and abstractness',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'syl-exist-23-inv-1',
      constraint: 'middle.determination = none',
      predicate: 'equals(middle.determination, none)',
    },
    {
      id: 'syl-exist-23-inv-2',
      constraint: 'equality = quantitative',
      predicate: 'equals(equality, quantitative)',
    },
  ],

  forces: [
    {
      id: 'syl-exist-23-force-1',
      description: 'Fourth figure drives toward syllogism of reflection',
      type: 'passover',
      trigger: 'mediation.concrete = true',
      effect: 'syllogismOfReflection.emerges = true',
      targetState: 'syl-refl-1',
    },
  ],

  transitions: [
    {
      id: 'syl-exist-23-trans-1',
      from: 'syl-exist-23',
      to: 'syl-refl-1',
      mechanism: 'passover',
      description: 'From syllogism of existence to syllogism of reflection',
    },
  ],

  nextStates: ['syl-refl-1'],
  previousStates: ['syl-exist-20'],

  provenance: {
    topicMapId: 'syl-exist-23-fourth-mathematical',
    lineRange: { start: 787, end: 896 },
    section: 'Fourth Figure',
    order: 4,
  },

  description: 'U-U-U, mathematical syllogism. Quantitative equality, abstracts from qualitative. Mediation based on mediation, circle of reciprocal presupposing. Passed over into syllogism of reflection.',
};

export const existenceSyllogismIR: DialecticIR = {
  id: 'existence-syllogism-ir',
  title: 'Existence Syllogism IR: S-P-U, P-S-U, S-U-P, U-U-U',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - A. Syllogism of Existence',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'existence.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'syl-exist-1': 'subject',
      'syl-exist-14': 'subject',
      'syl-exist-20': 'subject',
      'syl-exist-23': 'subject',
    },
  },
};

export const existenceSyllogismStates = {
  'syl-exist-1': state1,
  'syl-exist-14': state2,
  'syl-exist-20': state3,
  'syl-exist-23': state4,
};
