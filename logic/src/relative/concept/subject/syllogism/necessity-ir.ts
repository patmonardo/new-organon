/**
 * Necessity Syllogism IR: Dialectic Pseudo-Code for Syllogism of Necessity
 *
 * Architecture: CPU Inference Engine (Representation / Disambiguation)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - C. Syllogism of Necessity
 *
 * Covers the dialectical movement:
 * - Categorical: Through substance, objectivity begins
 * - Hypothetical: Negative unity, identity of mediator and mediated
 * - Disjunctive: Middle as totality, formalism sublated, concept realized as objectivity
 *
 * Concept realized - transitions to Objectivity (GPU)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'syl-nec-1',
  title: 'Categorical — through substance, objectivity begins',
  concept: 'CategoricalSyllogism',
  phase: 'subject',

  moments: [
    {
      name: 'throughSubstance',
      definition: 'First syllogism of necessity, subject conjoined through substance',
      type: 'determination',
    },
    {
      name: 'objectivityBegins',
      definition: 'No longer subjective, objectivity begins',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'throughSubstance',
    },
    {
      name: 'stillSubjective',
      definition: 'Still subjective element - identity is substantial, not yet identity of form',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'syl-nec-1-inv-1',
      constraint: 'middle = objectiveUniversality',
      predicate: 'equals(middle, objectiveUniversality)',
    },
    {
      id: 'syl-nec-1-inv-2',
      constraint: 'objectivity.begins = true',
      predicate: 'begins(objectivity)',
    },
  ],

  forces: [
    {
      id: 'syl-nec-1-force-1',
      description: 'Categorical drives toward hypothetical',
      type: 'mediation',
      trigger: 'identity.formalInner = true',
      effect: 'hypothetical.emerges = true',
      targetState: 'syl-nec-5',
    },
  ],

  transitions: [
    {
      id: 'syl-nec-1-trans-1',
      from: 'syl-nec-1',
      to: 'syl-nec-5',
      mechanism: 'mediation',
      description: 'From categorical to hypothetical',
    },
  ],

  nextStates: ['syl-nec-5'],
  previousStates: ['reflection-syllogism-ir'],

  provenance: {
    topicMapId: 'syl-nec-1-introduction-objective',
    lineRange: { start: 5, end: 200 },
    section: 'The Categorical Syllogism',
    order: 1,
  },

  description: 'First syllogism of necessity, through substance. Objectivity begins. No contingency, no demand for proof. Still subjective - identity is substantial, not yet identity of form. Passes to hypothetical.',
};

const state2: DialecticState = {
  id: 'syl-nec-5',
  title: 'Hypothetical — negative unity, identity of mediator and mediated',
  concept: 'HypotheticalSyllogism',
  phase: 'subject',

  moments: [
    {
      name: 'negativeUnity',
      definition: 'First to display necessary connection through form/negative unity',
      type: 'determination',
    },
    {
      name: 'identityOfMediatorAndMediated',
      definition: 'Identity of mediating term and mediated, absolute content same',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'negativeUnity',
    },
    {
      name: 'selfReferringNegativity',
      definition: 'Mediation is singularity/immediacy/self-referring negativity',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'syl-nec-5-inv-1',
      constraint: 'connection = negativeUnity',
      predicate: 'equals(connection, negativeUnity)',
    },
    {
      id: 'syl-nec-5-inv-2',
      constraint: 'mediator = mediated',
      predicate: 'equals(mediator, mediated)',
    },
  ],

  forces: [
    {
      id: 'syl-nec-5-force-1',
      description: 'Hypothetical drives toward disjunctive',
      type: 'mediation',
      trigger: 'mediation.selfReferringNegativity = true',
      effect: 'disjunctive.emerges = true',
      targetState: 'syl-nec-9',
    },
  ],

  transitions: [
    {
      id: 'syl-nec-5-trans-1',
      from: 'syl-nec-5',
      to: 'syl-nec-9',
      mechanism: 'mediation',
      description: 'From hypothetical to disjunctive',
    },
  ],

  nextStates: ['syl-nec-9'],
  previousStates: ['syl-nec-1'],

  provenance: {
    topicMapId: 'syl-nec-5-hypothetical-immediacy',
    lineRange: { start: 204, end: 364 },
    section: 'The Hypothetical Syllogism',
    order: 2,
  },

  description: 'Hypothetical syllogism. First to display necessary connection through form/negative unity. Identity of mediating term and mediated. Mediation is self-referring negativity. Passes to disjunctive.',
};

const state3: DialecticState = {
  id: 'syl-nec-9',
  title: 'Disjunctive — middle as totality, concept realized as objectivity',
  concept: 'DisjunctiveSyllogism',
  phase: 'subject',

  moments: [
    {
      name: 'middleAsTotality',
      definition: 'Middle as totality, universality replete with form',
      type: 'determination',
    },
    {
      name: 'formalismSublated',
      definition: 'Formalism sublated, subjectivity sublated, no longer syllogism',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'middleAsTotality',
    },
    {
      name: 'conceptRealizedAsObjectivity',
      definition: 'Concept realized, reality is objectivity',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'syl-nec-9-inv-1',
      constraint: 'middle = totality',
      predicate: 'equals(middle, totality)',
    },
    {
      id: 'syl-nec-9-inv-2',
      constraint: 'formalism.sublated = true',
      predicate: 'isSublated(formalism)',
    },
    {
      id: 'syl-nec-9-inv-3',
      constraint: 'concept.realized = objectivity',
      predicate: 'equals(concept.realized, objectivity)',
    },
  ],

  forces: [
    {
      id: 'syl-nec-9-force-1',
      description: 'Disjunctive transitions to objectivity',
      type: 'passover',
      trigger: 'concept.realized = true',
      effect: 'objectivity.emerges = true',
      targetState: 'objectivity-1',
    },
  ],

  transitions: [
    {
      id: 'syl-nec-9-trans-1',
      from: 'syl-nec-9',
      to: 'objectivity-1',
      mechanism: 'passover',
      description: 'From syllogism to objectivity',
    },
  ],

  nextStates: ['objectivity-1'],
  previousStates: ['syl-nec-5'],

  provenance: {
    topicMapId: 'syl-nec-9-disjunctive-totality',
    lineRange: { start: 368, end: 538 },
    section: 'The Disjunctive Syllogism',
    order: 3,
  },

  description: 'Disjunctive syllogism S-U-P. Middle as totality, universality replete with form. No longer syllogism. Formalism sublated, subjectivity sublated. Concept realized as objectivity.',
};

export const necessitySyllogismIR: DialecticIR = {
  id: 'necessity-syllogism-ir',
  title: 'Necessity Syllogism IR: Categorical, Hypothetical, Disjunctive, Objectivity',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - C. Syllogism of Necessity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'necessity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'syl-nec-1': 'subject',
      'syl-nec-5': 'subject',
      'syl-nec-9': 'subject',
    },
  },
};

export const necessitySyllogismStates = {
  'syl-nec-1': state1,
  'syl-nec-5': state2,
  'syl-nec-9': state3,
};
