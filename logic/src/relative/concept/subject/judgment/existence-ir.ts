/**
 * Existence IR: Dialectic Pseudo-Code for Judgment of Existence
 *
 * Architecture: Knowledge Processor (Predicative Logic / MVC Applications)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - A. Judgment of Existence
 *
 * Covers the dialectical movement:
 * - Positive Judgment: "The singular is universal"
 * - Negative Judgment: "The singular is a particular"
 * - Infinite Judgment: "The singular is singular" (judgment sublated)
 * - Transition to Judgment of Reflection
 *
 * This is where the Concept becomes predicative - the foundation for MVC applications!
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'exist-1',
  title: 'Judgment as truth — agreement of concept and reality',
  concept: 'JudgmentAsTruth',
  phase: 'subject',

  moments: [
    {
      name: 'judgment',
      definition: 'Judgment is truth, agreement of concept and reality',
      type: 'determination',
    },
    {
      name: 'singularActuality',
      definition: 'Singular actuality and essential identity (concept)',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'judgment',
    },
    {
      name: 'immediate',
      definition: 'At first immediate, no reflection, no movement',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'exist-1-inv-1',
      constraint: 'judgment = truth',
      predicate: 'equals(judgment, truth)',
    },
    {
      id: 'exist-1-inv-2',
      constraint: 'truth = agreement(concept, reality)',
      predicate: 'equals(truth, agreement(concept, reality))',
    },
    {
      id: 'exist-1-inv-3',
      constraint: 'judgment.immediate = true',
      predicate: 'isImmediate(judgment)',
    },
  ],

  forces: [
    {
      id: 'exist-1-force-1',
      description: 'Immediate judgment drives toward positive judgment',
      type: 'mediation',
      trigger: 'judgment.immediate = true',
      effect: 'positiveJudgment.emerges = true',
      targetState: 'exist-4',
    },
  ],

  transitions: [
    {
      id: 'exist-1-trans-1',
      from: 'exist-1',
      to: 'exist-4',
      mechanism: 'mediation',
      description: 'From judgment as truth to positive judgment',
    },
  ],

  nextStates: ['exist-4'],
  previousStates: ['singular-ir'],

  provenance: {
    topicMapId: 'exist-1-introduction-truth',
    lineRange: { start: 3, end: 24 },
    section: 'The Judgment of Existence',
    order: 1,
  },

  description: 'Judgment is truth, agreement of concept and reality. Singular actuality and essential identity. At first immediate, no reflection. Judgment of immediate existence is qualitative judgment.',
};

const state2: DialecticState = {
  id: 'exist-4',
  title: 'Positive judgment — "The singular is universal"',
  concept: 'PositiveJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'singularIsUniversal',
      definition: 'First pure expression: "the singular is universal"',
      type: 'determination',
    },
    {
      name: 'determinationsOfConcept',
      definition: 'Judgment has determinations of concept for extremes',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'singularIsUniversal',
    },
    {
      name: 'abstractJudgment',
      definition: 'Every judgment in principle also abstract judgment',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'exist-4-inv-1',
      constraint: 'judgment = "singular is universal"',
      predicate: 'equals(judgment, singularIsUniversal)',
    },
    {
      id: 'exist-4-inv-2',
      constraint: 'extremes = determinationsOfConcept',
      predicate: 'equals(extremes, determinationsOfConcept)',
    },
  ],

  forces: [
    {
      id: 'exist-4-force-1',
      description: 'Positive judgment drives toward resolution',
      type: 'mediation',
      trigger: 'positiveJudgment.established = true',
      effect: 'resolution.emerges = true',
      targetState: 'exist-6',
    },
  ],

  transitions: [
    {
      id: 'exist-4-trans-1',
      from: 'exist-4',
      to: 'exist-6',
      mechanism: 'mediation',
      description: 'From positive judgment to twofold result',
    },
  ],

  nextStates: ['exist-6'],
  previousStates: ['exist-1'],

  provenance: {
    topicMapId: 'exist-4-first-pure-expression',
    lineRange: { start: 89, end: 127 },
    section: 'The Positive Judgment',
    order: 2,
  },

  description: 'First pure expression: "the singular is universal". Judgment has determinations of concept for extremes. Every judgment in principle also abstract judgment.',
};

const state3: DialecticState = {
  id: 'exist-6',
  title: 'Twofold result — form and content',
  concept: 'TwofoldResult',
  phase: 'subject',

  moments: [
    {
      name: 'subjectDeterminedAsUniversal',
      definition: 'Subject determined as universal by predicate',
      type: 'determination',
    },
    {
      name: 'predicateDeterminedAsSingular',
      definition: 'Predicate determined in subject as singular',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'subjectDeterminedAsUniversal',
    },
    {
      name: 'formAndContent',
      definition: 'Form (immediately given) and content (isolated determination, totality)',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'exist-6-inv-1',
      constraint: 'subject = universal',
      predicate: 'equals(subject, universal)',
    },
    {
      id: 'exist-6-inv-2',
      constraint: 'predicate = singular',
      predicate: 'equals(predicate, singular)',
    },
    {
      id: 'exist-6-inv-3',
      constraint: 'form.united = content',
      predicate: 'united(form, content)',
    },
  ],

  forces: [
    {
      id: 'exist-6-force-1',
      description: 'Twofold result drives toward contradiction',
      type: 'negation',
      trigger: 'formAndContent.distinguished = true',
      effect: 'contradiction.emerges = true',
      targetState: 'exist-8',
    },
  ],

  transitions: [
    {
      id: 'exist-6-trans-1',
      from: 'exist-6',
      to: 'exist-8',
      mechanism: 'negation',
      description: 'From twofold result to positive judgment not true',
    },
  ],

  nextStates: ['exist-8'],
  previousStates: ['exist-4'],

  provenance: {
    topicMapId: 'exist-6-twofold-result-form-content',
    lineRange: { start: 192, end: 272 },
    section: 'The Positive Judgment',
    order: 3,
  },

  description: 'Twofold result: subject determined as universal, predicate as singular. Form and content united in immediate positive judgment. Difference of form/content present implicitly.',
};

const state4: DialecticState = {
  id: 'exist-8',
  title: 'Positive judgment not true — must be posited as negative',
  concept: 'PositiveJudgmentNotTrue',
  phase: 'subject',

  moments: [
    {
      name: 'formFails',
      definition: 'Form: immediate singular not universal (wider extension)',
      type: 'negation',
    },
    {
      name: 'contentFails',
      definition: 'Content: subject is universe of qualities, bad infinite plurality',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'formFails',
    },
    {
      name: 'positedAsNegative',
      definition: 'Both propositions must be united, positive judgment posited as negative',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'exist-8-inv-1',
      constraint: 'singular ≠ universal',
      predicate: 'not(equals(singular, universal))',
    },
    {
      id: 'exist-8-inv-2',
      constraint: 'subject = badInfinitePlurality',
      predicate: 'equals(subject, badInfinitePlurality)',
    },
  ],

  forces: [
    {
      id: 'exist-8-force-1',
      description: 'Contradiction drives toward negative judgment',
      type: 'negation',
      trigger: 'positiveJudgment.notTrue = true',
      effect: 'negativeJudgment.emerges = true',
      targetState: 'exist-9',
    },
  ],

  transitions: [
    {
      id: 'exist-8-trans-1',
      from: 'exist-8',
      to: 'exist-9',
      mechanism: 'negation',
      description: 'From positive not true to negative judgment',
    },
  ],

  nextStates: ['exist-9'],
  previousStates: ['exist-6'],

  provenance: {
    topicMapId: 'exist-8-positive-not-true',
    lineRange: { start: 310, end: 335 },
    section: 'The Positive Judgment',
    order: 4,
  },

  description: 'Positive judgment not true. Form: singular not universal. Content: bad infinite plurality. Both propositions must be united, posited as negative.',
};

const state5: DialecticState = {
  id: 'exist-9',
  title: 'Negative judgment — "The singular is a particular"',
  concept: 'NegativeJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'singularIsParticular',
      definition: 'Truth of positive judgment in negative: "the singular is a particular"',
      type: 'determination',
    },
    {
      name: 'logicalContent',
      definition: 'Two concepts relate as singular and universal (truly logical content)',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'singularIsParticular',
    },
    {
      name: 'particularity',
      definition: 'Particularity arises through negative connection',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'exist-9-inv-1',
      constraint: 'singular = particular',
      predicate: 'equals(singular, particular)',
    },
    {
      id: 'exist-9-inv-2',
      constraint: 'universal = particular',
      predicate: 'equals(universal, particular)',
    },
    {
      id: 'exist-9-inv-3',
      constraint: 'particularity = mediated',
      predicate: 'isMediated(particularity)',
    },
  ],

  forces: [
    {
      id: 'exist-9-force-1',
      description: 'Negative judgment drives toward infinite judgment',
      type: 'negation',
      trigger: 'negativeJudgment.established = true',
      effect: 'infiniteJudgment.emerges = true',
      targetState: 'exist-15',
    },
  ],

  transitions: [
    {
      id: 'exist-9-trans-1',
      from: 'exist-9',
      to: 'exist-15',
      mechanism: 'negation',
      description: 'From negative judgment to infinite judgment',
    },
  ],

  nextStates: ['exist-15'],
  previousStates: ['exist-8'],

  provenance: {
    topicMapId: 'exist-9-truth-in-negative',
    lineRange: { start: 338, end: 411 },
    section: 'The Negative Judgment',
    order: 5,
  },

  description: 'Truth of positive judgment in negative: "the singular is a particular". Particularity arises through negative connection, mediated determination.',
};

const state6: DialecticState = {
  id: 'exist-15',
  title: 'Infinite judgment — "The singular is singular"',
  concept: 'InfiniteJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'negationOfNegation',
      definition: 'Second negation, infinite turning back of singularity into itself',
      type: 'negation',
    },
    {
      name: 'singularIsSingular',
      definition: 'The singular is singular, the universal is universal',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'negationOfNegation',
    },
    {
      name: 'judgmentSublated',
      definition: 'Through reflection, judgment sublated - only identity, no longer judgment',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'exist-15-inv-1',
      constraint: 'singular = singular',
      predicate: 'equals(singular, singular)',
    },
    {
      id: 'exist-15-inv-2',
      constraint: 'universal = universal',
      predicate: 'equals(universal, universal)',
    },
    {
      id: 'exist-15-inv-3',
      constraint: 'judgment.sublated = true',
      predicate: 'isSublated(judgment)',
    },
  ],

  forces: [
    {
      id: 'exist-15-force-1',
      description: 'Infinite judgment drives toward judgment of reflection',
      type: 'passover',
      trigger: 'judgment.sublated = true',
      effect: 'judgmentOfReflection.emerges = true',
      targetState: 'reflection-1',
    },
  ],

  transitions: [
    {
      id: 'exist-15-trans-1',
      from: 'exist-15',
      to: 'reflection-1',
      mechanism: 'passover',
      description: 'From infinite judgment to judgment of reflection',
    },
  ],

  nextStates: ['reflection-1'],
  previousStates: ['exist-9'],

  provenance: {
    topicMapId: 'exist-15-negation-negative',
    lineRange: { start: 641, end: 798 },
    section: 'The Infinite Judgment',
    order: 6,
  },

  description: 'Negation of negation, infinite turning back. "The singular is singular". Judgment sublated - only identity, total lack of difference, no longer judgment. Transition to judgment of reflection.',
};

export const existenceIR: DialecticIR = {
  id: 'existence-ir',
  title: 'Existence IR: Positive, Negative, Infinite Judgment',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - A. Judgment of Existence',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'existence.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'exist-1': 'subject',
      'exist-4': 'subject',
      'exist-6': 'subject',
      'exist-8': 'subject',
      'exist-9': 'subject',
      'exist-15': 'subject',
    },
  },
};

export const existenceStates = {
  'exist-1': state1,
  'exist-4': state2,
  'exist-6': state3,
  'exist-8': state4,
  'exist-9': state5,
  'exist-15': state6,
};
