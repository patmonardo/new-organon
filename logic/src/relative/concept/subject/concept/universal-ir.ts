/**
 * Universal IR: Dialectic Pseudo-Code for Universal Concept
 *
 * Architecture: Knowledge Processor (GDSL → SDSL Integration)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 1. The Universal
 *
 * Covers the dialectical movement:
 * - Pure concept as absolutely infinite, unconditioned
 * - Universality as negation of negation
 * - Universal as creative principle, free power/love
 * - Determinateness within universal (particularity/singularity)
 * - Total reflection (outward/inward shining)
 * - Creative self-differentiation
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'univ-1',
  title: 'Pure Concept — absolutely infinite, unconditioned',
  concept: 'PureConcept',
  phase: 'subject',

  moments: [
    {
      name: 'pureConcept',
      definition: 'Absolutely infinite, unconditioned, free',
      type: 'determination',
    },
    {
      name: 'genesis',
      definition: 'Being → Essence → Concept via self-repulsion',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'pureConcept',
    },
    {
      name: 'mutualPenetration',
      definition: 'Mutual penetration of moments, immanent reflection',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'univ-1-inv-1',
      constraint: 'concept = absolutelyInfinite',
      predicate: 'equals(concept, absolutelyInfinite)',
    },
    {
      id: 'univ-1-inv-2',
      constraint: 'becoming = selfRepulsion',
      predicate: 'equals(becoming, selfRepulsion)',
    },
    {
      id: 'univ-1-inv-3',
      constraint: 'determinateness = infiniteSelfReferring',
      predicate: 'equals(determinateness, infiniteSelfReferring)',
    },
  ],

  forces: [
    {
      id: 'univ-1-force-1',
      description: 'Pure concept drives toward universality as negation of negation',
      type: 'mediation',
      trigger: 'concept.pure = true',
      effect: 'universality.emerges = true',
      targetState: 'univ-2',
    },
  ],

  transitions: [
    {
      id: 'univ-1-trans-1',
      from: 'univ-1',
      to: 'univ-2',
      mechanism: 'mediation',
      description: 'From pure concept to universality',
    },
  ],

  nextStates: ['univ-2'],
  previousStates: ['actuality-ir'],

  provenance: {
    topicMapId: 'univ-1-genesis-absoluteness',
    lineRange: { start: 1, end: 28 },
    section: '1. The Universal',
    order: 1,
  },

  description: 'Pure concept is absolutely infinite, unconditioned, free. Genesis: Being → Essence → Concept via self-repulsion. Mutual penetration of moments. Determinateness is infinite, self-referring.',
};

const state2: DialecticState = {
  id: 'univ-2',
  title: 'Universality — negation of negation',
  concept: 'UniversalityAsNegationOfNegation',
  phase: 'subject',

  moments: [
    {
      name: 'universality',
      definition: 'Absolute self-identity, negation of negation, infinite unity of negativity',
      type: 'determination',
    },
    {
      name: 'pureSelfreference',
      definition: 'Pure self-reference via negativity',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'universality',
    },
    {
      name: 'absoluteNegativity',
      definition: 'Contains difference/determinateness by absolute negativity',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'univ-2-inv-1',
      constraint: 'universality = negationOfNegation',
      predicate: 'equals(universality, negationOfNegation)',
    },
    {
      id: 'univ-2-inv-2',
      constraint: 'universality = infiniteUnity(negativity)',
      predicate: 'equals(universality, infiniteUnity(negativity))',
    },
    {
      id: 'univ-2-inv-3',
      constraint: 'universal.simple = true',
      predicate: 'isSimple(universal)',
    },
  ],

  forces: [
    {
      id: 'univ-2-force-1',
      description: 'Universality drives toward creative principle',
      type: 'mediation',
      trigger: 'universality.established = true',
      effect: 'creativePrinciple.emerges = true',
      targetState: 'univ-6',
    },
  ],

  transitions: [
    {
      id: 'univ-2-trans-1',
      from: 'univ-2',
      to: 'univ-6',
      mechanism: 'mediation',
      description: 'From universality to creative principle',
    },
  ],

  nextStates: ['univ-6'],
  previousStates: ['univ-1'],

  provenance: {
    topicMapId: 'univ-2-universality-negation-of-negation',
    lineRange: { start: 30, end: 45 },
    section: '1. The Universal',
    order: 2,
  },

  description: 'Universality is absolute self-identity, negation of negation, infinite unity of negativity. Pure self-reference via negativity. Contains difference/determinateness by absolute negativity.',
};

const state3: DialecticState = {
  id: 'univ-6',
  title: 'Universal as creative principle — essence',
  concept: 'CreativePrinciple',
  phase: 'subject',

  moments: [
    {
      name: 'creativePrinciple',
      definition: 'Universal as informing and creative principle, absolute negativity',
      type: 'determination',
    },
    {
      name: 'essence',
      definition: 'Universal is essence of its determination, positive nature',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'creativePrinciple',
    },
    {
      name: 'freePower',
      definition: 'Free power, free love, boundless blessedness',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'univ-6-inv-1',
      constraint: 'universal = creativePrinciple',
      predicate: 'equals(universal, creativePrinciple)',
    },
    {
      id: 'univ-6-inv-2',
      constraint: 'universal = essence(determination)',
      predicate: 'equals(universal, essence(determination))',
    },
    {
      id: 'univ-6-inv-3',
      constraint: 'universal = freePower',
      predicate: 'equals(universal, freePower)',
    },
  ],

  forces: [
    {
      id: 'univ-6-force-1',
      description: 'Creative principle drives toward determinateness within universal',
      type: 'mediation',
      trigger: 'creativePrinciple.established = true',
      effect: 'determinateness.emerges = true',
      targetState: 'univ-8',
    },
  ],

  transitions: [
    {
      id: 'univ-6-trans-1',
      from: 'univ-6',
      to: 'univ-8',
      mechanism: 'mediation',
      description: 'From creative principle to determinateness',
    },
  ],

  nextStates: ['univ-8'],
  previousStates: ['univ-2'],

  provenance: {
    topicMapId: 'univ-6-essence-creative-principle',
    lineRange: { start: 118, end: 168 },
    section: '1. The Universal',
    order: 3,
  },

  description: 'Universal is essence of its determination, creative principle. Free power, free love, boundless blessedness. At rest in its other as in its own.',
};

const state4: DialecticState = {
  id: 'univ-8',
  title: 'Determinateness within universal — totality',
  concept: 'DeterminatenessWithinUniversal',
  phase: 'subject',

  moments: [
    {
      name: 'determinateness',
      definition: 'Universal contains determinateness (particularity/singularity) via absolute negativity',
      type: 'determination',
    },
    {
      name: 'totality',
      definition: 'Universal is totality of concept, concrete, has content',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'determinateness',
    },
    {
      name: 'abstractUniversal',
      definition: 'Abstract universal is isolated, imperfect, void of truth',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'univ-8-inv-1',
      constraint: 'universal.contains = determinateness',
      predicate: 'contains(universal, determinateness)',
    },
    {
      id: 'univ-8-inv-2',
      constraint: 'universal = totality',
      predicate: 'equals(universal, totality)',
    },
    {
      id: 'univ-8-inv-3',
      constraint: 'abstractUniversal.voidOfTruth = true',
      predicate: 'isVoidOfTruth(abstractUniversal)',
    },
  ],

  forces: [
    {
      id: 'univ-8-force-1',
      description: 'Determinateness drives toward total reflection',
      type: 'mediation',
      trigger: 'determinateness.established = true',
      effect: 'totalReflection.emerges = true',
      targetState: 'univ-9',
    },
  ],

  transitions: [
    {
      id: 'univ-8-trans-1',
      from: 'univ-8',
      to: 'univ-9',
      mechanism: 'mediation',
      description: 'From determinateness to total reflection',
    },
  ],

  nextStates: ['univ-9'],
  previousStates: ['univ-6'],

  provenance: {
    topicMapId: 'univ-8-determinateness-totality',
    lineRange: { start: 169, end: 204 },
    section: '1. The Universal',
    order: 4,
  },

  description: 'Universal contains determinateness (particularity/singularity) via absolute negativity. Universal is totality of concept, concrete. Abstract universal is isolated, void of truth.',
};

const state5: DialecticState = {
  id: 'univ-9',
  title: 'Total reflection — outward and inward shining',
  concept: 'TotalReflection',
  phase: 'subject',

  moments: [
    {
      name: 'totalReflection',
      definition: 'Doubly reflective shine: outward (particularity) and inward (genus)',
      type: 'mediation',
    },
    {
      name: 'genus',
      definition: 'Determinateness as determinate concept, immanent character',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'totalReflection',
    },
    {
      name: 'infinitelyFree',
      definition: 'Determinate concept is infinitely free concept',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'univ-9-inv-1',
      constraint: 'reflection = doubly(outward, inward)',
      predicate: 'equals(reflection, doubly(outward, inward))',
    },
    {
      id: 'univ-9-inv-2',
      constraint: 'determinateness = genus',
      predicate: 'equals(determinateness, genus)',
    },
    {
      id: 'univ-9-inv-3',
      constraint: 'determinateConcept = infinitelyFree',
      predicate: 'equals(determinateConcept, infinitelyFree)',
    },
  ],

  forces: [
    {
      id: 'univ-9-force-1',
      description: 'Total reflection drives toward creative self-differentiation',
      type: 'mediation',
      trigger: 'totalReflection.established = true',
      effect: 'selfDifferentiation.emerges = true',
      targetState: 'univ-11',
    },
  ],

  transitions: [
    {
      id: 'univ-9-trans-1',
      from: 'univ-9',
      to: 'univ-11',
      mechanism: 'mediation',
      description: 'From total reflection to creative self-differentiation',
    },
  ],

  nextStates: ['univ-11'],
  previousStates: ['univ-8'],

  provenance: {
    topicMapId: 'univ-9-total-reflection-inward-outward',
    lineRange: { start: 206, end: 246 },
    section: '1. The Universal',
    order: 5,
  },

  description: 'Total reflection is doubly reflective shine: outward (particularity) and inward (genus). Determinateness as determinate concept is immanent character. Infinitely free concept.',
};

const state6: DialecticState = {
  id: 'univ-11',
  title: 'Creative self-differentiation — particularity emerges',
  concept: 'CreativeSelfDifferentiation',
  phase: 'subject',

  moments: [
    {
      name: 'selfDifferentiation',
      definition: 'Universal determines itself freely, differentiates internally',
      type: 'process',
    },
    {
      name: 'creativePower',
      definition: 'Creative power as self-referring absolute negativity',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'selfDifferentiation',
    },
    {
      name: 'universalDifferences',
      definition: 'Posits differences that are themselves universals, self-referring',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'univ-11-inv-1',
      constraint: 'universal.determinesFreelyItself = true',
      predicate: 'determinesFreelyItself(universal)',
    },
    {
      id: 'univ-11-inv-2',
      constraint: 'creativePower = selfReferringAbsoluteNegativity',
      predicate: 'equals(creativePower, selfReferringAbsoluteNegativity)',
    },
    {
      id: 'univ-11-inv-3',
      constraint: 'differences = universals',
      predicate: 'equals(differences, universals)',
    },
  ],

  forces: [
    {
      id: 'univ-11-force-1',
      description: 'Creative self-differentiation drives toward particularity',
      type: 'passover',
      trigger: 'selfDifferentiation.complete = true',
      effect: 'particularity.emerges = true',
      targetState: 'part-1',
    },
  ],

  transitions: [
    {
      id: 'univ-11-trans-1',
      from: 'univ-11',
      to: 'part-1',
      mechanism: 'passover',
      description: 'From universal to particularity',
    },
  ],

  nextStates: ['part-1'],
  previousStates: ['univ-9'],

  provenance: {
    topicMapId: 'univ-11-creative-differentiation',
    lineRange: { start: 279, end: 296 },
    section: '1. The Universal',
    order: 6,
  },

  description: 'Universal determines itself freely, differentiates internally. Creative power as self-referring absolute negativity. Posits differences that are themselves universals.',
};

export const universalIR: DialecticIR = {
  id: 'universal-ir',
  title: 'Universal IR: Pure Concept, Negation of Negation, Creative Principle',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 1. The Universal',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'universal.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'univ-1': 'subject',
      'univ-2': 'subject',
      'univ-6': 'subject',
      'univ-8': 'subject',
      'univ-9': 'subject',
      'univ-11': 'subject',
    },
  },
};

export const universalStates = {
  'univ-1': state1,
  'univ-2': state2,
  'univ-6': state3,
  'univ-8': state4,
  'univ-9': state5,
  'univ-11': state6,
};
