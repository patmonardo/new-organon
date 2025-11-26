/**
 * Reflection Syllogism IR: Dialectic Pseudo-Code for Syllogism of Reflection
 *
 * Architecture: CPU Inference Engine (Representation / Disambiguation)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - B. Syllogism of Reflection
 *
 * Covers the dialectical movement:
 * - Allness: Understanding in perfection, external universality, mere illusion
 * - Induction: Singularity as completed, experience, bad infinity
 * - Analogy: Singular in universal nature, essential universality
 * - Transition to Syllogism of Necessity
 *
 * Middle term as totality, sublated abstractness
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'syl-refl-1',
  title: 'Allness — understanding in perfection, mere illusion',
  concept: 'SyllogismOfAllness',
  phase: 'subject',

  moments: [
    {
      name: 'allness',
      definition: 'Middle as "all", understanding in perfection, external universality',
      type: 'determination',
    },
    {
      name: 'mereIllusion',
      definition: 'Major premise presupposes conclusion, mere illusion',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'allness',
    },
  ],

  invariants: [
    {
      id: 'syl-refl-1-inv-1',
      constraint: 'middle = allness',
      predicate: 'equals(middle, allness)',
    },
    {
      id: 'syl-refl-1-inv-2',
      constraint: 'major.presupposes = conclusion',
      predicate: 'presupposes(major, conclusion)',
    },
  ],

  forces: [
    {
      id: 'syl-refl-1-force-1',
      description: 'Allness drives toward induction',
      type: 'mediation',
      trigger: 'major.presupposesConclusion = true',
      effect: 'induction.emerges = true',
      targetState: 'syl-refl-8',
    },
  ],

  transitions: [
    {
      id: 'syl-refl-1-trans-1',
      from: 'syl-refl-1',
      to: 'syl-refl-8',
      mechanism: 'mediation',
      description: 'From allness to induction',
    },
  ],

  nextStates: ['syl-refl-8'],
  previousStates: ['existence-syllogism-ir'],

  provenance: {
    topicMapId: 'syl-refl-1-introduction-sublated',
    lineRange: { start: 4, end: 185 },
    section: 'The Syllogism of Allness',
    order: 1,
  },

  description: 'Allness - understanding in perfection. Middle as "all". Major premise presupposes conclusion. Mere illusion. Essence rests on subjective singularity.',
};

const state2: DialecticState = {
  id: 'syl-refl-8',
  title: 'Induction — singularity as completed, bad infinity',
  concept: 'SyllogismOfInduction',
  phase: 'subject',

  moments: [
    {
      name: 'induction',
      definition: 'U-S-P, singularity as completed, syllogism of experience',
      type: 'determination',
    },
    {
      name: 'badInfinity',
      definition: 'Progression into bad infinity, perpetual ought, problematic conclusion',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'induction',
    },
    {
      name: 'presupposesGenus',
      definition: 'Presupposes genus in and for itself, presupposes conclusion',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'syl-refl-8-inv-1',
      constraint: 'schema = U-S-P',
      predicate: 'equals(schema, USP)',
    },
    {
      id: 'syl-refl-8-inv-2',
      constraint: 'induction.presupposes = genus',
      predicate: 'presupposes(induction, genus)',
    },
  ],

  forces: [
    {
      id: 'syl-refl-8-force-1',
      description: 'Induction drives toward analogy',
      type: 'mediation',
      trigger: 'singularity.immediatelyIdentical = universality',
      effect: 'analogy.emerges = true',
      targetState: 'syl-refl-12',
    },
  ],

  transitions: [
    {
      id: 'syl-refl-8-trans-1',
      from: 'syl-refl-8',
      to: 'syl-refl-12',
      mechanism: 'mediation',
      description: 'From induction to analogy',
    },
  ],

  nextStates: ['syl-refl-12'],
  previousStates: ['syl-refl-1'],

  provenance: {
    topicMapId: 'syl-refl-8-induction-schema',
    lineRange: { start: 189, end: 320 },
    section: 'The Syllogism of Induction',
    order: 2,
  },

  description: 'Induction U-S-P. Singularity as completed. Syllogism of experience. Bad infinity, perpetual ought. Presupposes genus in and for itself. Truth is analogy.',
};

const state3: DialecticState = {
  id: 'syl-refl-12',
  title: 'Analogy — singular in universal nature, essential universality',
  concept: 'SyllogismOfAnalogy',
  phase: 'subject',

  moments: [
    {
      name: 'analogy',
      definition: 'S-U-P, singular in universal nature, essential universality',
      type: 'determination',
    },
    {
      name: 'superficiality',
      definition: 'More superficial the more universal is mere quality, form vs content',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'analogy',
    },
    {
      name: 'demandsSublation',
      definition: 'Demands sublation of moment of singularity, passes to necessity',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'syl-refl-12-inv-1',
      constraint: 'schema = S-U-P',
      predicate: 'equals(schema, SUP)',
    },
    {
      id: 'syl-refl-12-inv-2',
      constraint: 'middle = universality(immanentReflection)',
      predicate: 'equals(middle, universality(immanentReflection))',
    },
  ],

  forces: [
    {
      id: 'syl-refl-12-force-1',
      description: 'Analogy drives toward syllogism of necessity',
      type: 'passover',
      trigger: 'singularity.sublated = true',
      effect: 'necessitySyllogism.emerges = true',
      targetState: 'syl-nec-1',
    },
  ],

  transitions: [
    {
      id: 'syl-refl-12-trans-1',
      from: 'syl-refl-12',
      to: 'syl-nec-1',
      mechanism: 'passover',
      description: 'From analogy to syllogism of necessity',
    },
  ],

  nextStates: ['syl-nec-1'],
  previousStates: ['syl-refl-8'],

  provenance: {
    topicMapId: 'syl-refl-12-analogy-schema',
    lineRange: { start: 324, end: 543 },
    section: 'The Syllogism of Analogy',
    order: 3,
  },

  description: 'Analogy S-U-P. Singular in universal nature. Essential universality. Demands sublation of moment of singularity. Genus posited. Passed over into syllogism of necessity.',
};

export const reflectionSyllogismIR: DialecticIR = {
  id: 'reflection-syllogism-ir',
  title: 'Reflection Syllogism IR: Allness, Induction, Analogy',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - B. Syllogism of Reflection',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'reflection.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'syl-refl-1': 'subject',
      'syl-refl-8': 'subject',
      'syl-refl-12': 'subject',
    },
  },
};

export const reflectionSyllogismStates = {
  'syl-refl-1': state1,
  'syl-refl-8': state2,
  'syl-refl-12': state3,
};
