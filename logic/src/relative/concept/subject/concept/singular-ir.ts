/**
 * Singular IR: Dialectic Pseudo-Code for Singular Concept
 *
 * Architecture: Knowledge Processor (GDSL → SDSL Integration)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 3. The Singular
 *
 * Covers the dialectical movement:
 * - Singularity posited through particularity
 * - Reflection into itself, self-mediation
 * - Unity indissoluble, products of abstraction are singulars
 * - Inseparability posited, each is totality
 * - Concept steps into actuality
 * - Posited as judgment (absolute originative partition)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'sing-1',
  title: 'Singularity posited — determinate universality',
  concept: 'SingularityPosited',
  phase: 'subject',

  moments: [
    {
      name: 'singularity',
      definition: 'Determinate universality, self-referring determinateness',
      type: 'determination',
    },
    {
      name: 'determinateDeterminate',
      definition: 'The determinate determinate',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'singularity',
    },
  ],

  invariants: [
    {
      id: 'sing-1-inv-1',
      constraint: 'singularity = determinateUniversality',
      predicate: 'equals(singularity, determinateUniversality)',
    },
    {
      id: 'sing-1-inv-2',
      constraint: 'singularity = selfReferringDeterminateness',
      predicate: 'equals(singularity, selfReferringDeterminateness)',
    },
  ],

  forces: [
    {
      id: 'sing-1-force-1',
      description: 'Singularity drives toward self-mediation',
      type: 'mediation',
      trigger: 'singularity.posited = true',
      effect: 'selfMediation.emerges = true',
      targetState: 'sing-2',
    },
  ],

  transitions: [
    {
      id: 'sing-1-trans-1',
      from: 'sing-1',
      to: 'sing-2',
      mechanism: 'mediation',
      description: 'From singularity to self-mediation',
    },
  ],

  nextStates: ['sing-2'],
  previousStates: ['particular-ir'],

  provenance: {
    topicMapId: 'sing-1-posited-through-particularity',
    lineRange: { start: 4, end: 8 },
    section: '3. The Singular',
    order: 1,
  },

  description: 'Singularity posited through particularity. Determinate universality, self-referring determinateness. The determinate determinate.',
};

const state2: DialecticState = {
  id: 'sing-2',
  title: 'Reflection into itself — self-mediation',
  concept: 'SelfMediation',
  phase: 'subject',

  moments: [
    {
      name: 'selfMediation',
      definition: 'Concept\'s self-mediation, restores itself as self-equal in absolute negativity',
      type: 'mediation',
    },
    {
      name: 'doublyReflectiveShine',
      definition: 'Doubly reflective shine: inward (singularity) vs outward (abstraction)',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'selfMediation',
    },
    {
      name: 'positedAsConcept',
      definition: 'Singularity is depth where concept grasps itself, posited as concept',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'sing-2-inv-1',
      constraint: 'singularity = reflectionIntoItself',
      predicate: 'equals(singularity, reflectionIntoItself)',
    },
    {
      id: 'sing-2-inv-2',
      constraint: 'abstraction = falseStart',
      predicate: 'equals(abstraction, falseStart)',
    },
  ],

  forces: [
    {
      id: 'sing-2-force-1',
      description: 'Self-mediation drives toward indissoluble unity',
      type: 'mediation',
      trigger: 'selfMediation.complete = true',
      effect: 'indissolubility.emerges = true',
      targetState: 'sing-5',
    },
  ],

  transitions: [
    {
      id: 'sing-2-trans-1',
      from: 'sing-2',
      to: 'sing-5',
      mechanism: 'mediation',
      description: 'From self-mediation to indissoluble unity',
    },
  ],

  nextStates: ['sing-5'],
  previousStates: ['sing-1'],

  provenance: {
    topicMapId: 'sing-2-reflection-self-mediation',
    lineRange: { start: 10, end: 41 },
    section: '3. The Singular',
    order: 2,
  },

  description: 'Reflection into itself, concept\'s self-mediation. Doubly reflective shine: inward (singularity) vs outward (abstraction). Singularity is depth where concept posited as concept.',
};

const state3: DialecticState = {
  id: 'sing-5',
  title: 'Unity indissoluble — products of abstraction are singulars',
  concept: 'IndissolublyUnity',
  phase: 'subject',

  moments: [
    {
      name: 'indissolubility',
      definition: 'Unity so indissoluble that products of abstraction are themselves singulars',
      type: 'determination',
    },
    {
      name: 'abstractionGraspsSingulars',
      definition: 'Abstraction grasps universal as determinate universality = singularity',
      type: 'process',
      relation: 'contains',
      relatedTo: 'indissolubility',
    },
  ],

  invariants: [
    {
      id: 'sing-5-inv-1',
      constraint: 'unity.indissoluble = true',
      predicate: 'isIndissoluble(unity)',
    },
    {
      id: 'sing-5-inv-2',
      constraint: 'productsOfAbstraction = singulars',
      predicate: 'equals(productsOfAbstraction, singulars)',
    },
  ],

  forces: [
    {
      id: 'sing-5-force-1',
      description: 'Indissolubility drives toward inseparability',
      type: 'mediation',
      trigger: 'indissolubility.established = true',
      effect: 'inseparability.emerges = true',
      targetState: 'sing-8',
    },
  ],

  transitions: [
    {
      id: 'sing-5-trans-1',
      from: 'sing-5',
      to: 'sing-8',
      mechanism: 'mediation',
      description: 'From indissolubility to inseparability',
    },
  ],

  nextStates: ['sing-8'],
  previousStates: ['sing-2'],

  provenance: {
    topicMapId: 'sing-5-unity-indissoluble',
    lineRange: { start: 75, end: 101 },
    section: '3. The Singular',
    order: 3,
  },

  description: 'Unity indissoluble. Products of abstraction are themselves singulars. Abstraction grasps universal as determinate universality = singularity.',
};

const state4: DialecticState = {
  id: 'sing-8',
  title: 'Inseparability posited — each is totality',
  concept: 'InseparabilityPosited',
  phase: 'subject',

  moments: [
    {
      name: 'inseparability',
      definition: 'Inseparability of determinations posited, negation of negation',
      type: 'determination',
    },
    {
      name: 'eachIsTotality',
      definition: 'Each distinct determination is the totality, whole concept',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'inseparability',
    },
  ],

  invariants: [
    {
      id: 'sing-8-inv-1',
      constraint: 'inseparability.posited = true',
      predicate: 'isPosited(inseparability)',
    },
    {
      id: 'sing-8-inv-2',
      constraint: 'eachDetermination = totality',
      predicate: 'equals(eachDetermination, totality)',
    },
  ],

  forces: [
    {
      id: 'sing-8-force-1',
      description: 'Inseparability drives toward concept stepping into actuality',
      type: 'mediation',
      trigger: 'inseparability.established = true',
      effect: 'actuality.emerges = true',
      targetState: 'sing-9',
    },
  ],

  transitions: [
    {
      id: 'sing-8-trans-1',
      from: 'sing-8',
      to: 'sing-9',
      mechanism: 'mediation',
      description: 'From inseparability to actuality',
    },
  ],

  nextStates: ['sing-9'],
  previousStates: ['sing-5'],

  provenance: {
    topicMapId: 'sing-8-dissolution-inseparability',
    lineRange: { start: 139, end: 177 },
    section: '3. The Singular',
    order: 4,
  },

  description: 'Inseparability of determinations posited. Each distinct determination is the totality, whole concept.',
};

const state5: DialecticState = {
  id: 'sing-9',
  title: 'Concept steps into actuality — immediate loss',
  concept: 'ConceptStepsIntoActuality',
  phase: 'subject',

  moments: [
    {
      name: 'immediateLoss',
      definition: 'Singularity is not only turning back but immediate loss of concept',
      type: 'negation',
    },
    {
      name: 'stepsIntoActuality',
      definition: 'Through singularity, concept becomes external to itself, steps into actuality',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'immediateLoss',
    },
    {
      name: 'soulOfSingularity',
      definition: 'Abstraction is soul of singularity, immanent in universal and particular',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'sing-9-inv-1',
      constraint: 'singularity = immediateLoss',
      predicate: 'equals(singularity, immediateLoss)',
    },
    {
      id: 'sing-9-inv-2',
      constraint: 'concept.externalToItself = true',
      predicate: 'isExternalToItself(concept)',
    },
  ],

  forces: [
    {
      id: 'sing-9-force-1',
      description: 'Stepping into actuality drives toward posited as judgment',
      type: 'passover',
      trigger: 'actuality.established = true',
      effect: 'judgment.emerges = true',
      targetState: 'sing-13',
    },
  ],

  transitions: [
    {
      id: 'sing-9-trans-1',
      from: 'sing-9',
      to: 'sing-13',
      mechanism: 'passover',
      description: 'From actuality to judgment',
    },
  ],

  nextStates: ['sing-13'],
  previousStates: ['sing-8'],

  provenance: {
    topicMapId: 'sing-9-immediate-loss-actuality',
    lineRange: { start: 179, end: 198 },
    section: '3. The Singular',
    order: 5,
  },

  description: 'Singularity is immediate loss of concept. Through singularity, concept becomes external to itself, steps into actuality. Abstraction is soul of singularity.',
};

const state6: DialecticState = {
  id: 'sing-13',
  title: 'Posited as judgment — absolute originative partition',
  concept: 'PositedAsJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'lostItself',
      definition: 'Concept as connection of self-subsistent determinations has lost itself',
      type: 'negation',
    },
    {
      name: 'absolutePartition',
      definition: 'Concept\'s turning back is absolute, originative partition of itself',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'lostItself',
    },
    {
      name: 'positedAsJudgment',
      definition: 'As singularity, concept is posited as judgment',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'sing-13-inv-1',
      constraint: 'concept.lostItself = true',
      predicate: 'lostItself(concept)',
    },
    {
      id: 'sing-13-inv-2',
      constraint: 'turningBack = absolutePartition',
      predicate: 'equals(turningBack, absolutePartition)',
    },
    {
      id: 'sing-13-inv-3',
      constraint: 'singularity = judgment',
      predicate: 'equals(singularity, judgment)',
    },
  ],

  forces: [
    {
      id: 'sing-13-force-1',
      description: 'Judgment transitions to Judgment phase (Logic of Predication)',
      type: 'passover',
      trigger: 'judgment.posited = true',
      effect: 'judgmentPhase.emerges = true',
      targetState: 'judgment-1',
    },
  ],

  transitions: [
    {
      id: 'sing-13-trans-1',
      from: 'sing-13',
      to: 'judgment-1',
      mechanism: 'passover',
      description: 'From singular to Judgment phase',
    },
  ],

  nextStates: ['judgment-1'],
  previousStates: ['sing-9'],

  provenance: {
    topicMapId: 'sing-13-posited-as-judgment',
    lineRange: { start: 268, end: 283 },
    section: '3. The Singular',
    order: 6,
  },

  description: 'Concept has lost itself. Concept\'s turning back is absolute, originative partition. As singularity, concept is posited as judgment.',
};

export const singularIR: DialecticIR = {
  id: 'singular-ir',
  title: 'Singular IR: Self-Mediation, Indissolubility, Actuality, Judgment',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 3. The Singular',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'singular.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'sing-1': 'subject',
      'sing-2': 'subject',
      'sing-5': 'subject',
      'sing-8': 'subject',
      'sing-9': 'subject',
      'sing-13': 'subject',
    },
  },
};

export const singularStates = {
  'sing-1': state1,
  'sing-2': state2,
  'sing-5': state3,
  'sing-8': state4,
  'sing-9': state5,
  'sing-13': state6,
};
