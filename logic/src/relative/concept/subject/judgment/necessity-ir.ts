/**
 * Necessity IR: Dialectic Pseudo-Code for Judgment of Necessity
 *
 * Architecture: Knowledge Processor (Predicative Logic / MVC Applications)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - C. Judgment of Necessity
 *
 * Covers the dialectical movement:
 * - Categorical Judgment: Genus divides into species (substantial identity)
 * - Hypothetical Judgment: "If A is, then B is" (being of an other)
 * - Disjunctive Judgment: "A is either B or C" (proximate genus, negative unity)
 * - Transition to Judgment of Concept
 *
 * Connection becomes necessary, judgment reaches completion
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'nec-1',
  title: 'Objective universality — genus and species',
  concept: 'ObjectiveUniversalityGenusSpecies',
  phase: 'subject',

  moments: [
    {
      name: 'objectiveUniversality',
      definition: 'Universality advanced to objective universality, exists in and for itself',
      type: 'determination',
    },
    {
      name: 'positedNecessity',
      definition: 'Posited necessity, distinction immanent (not just accidents)',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'objectiveUniversality',
    },
    {
      name: 'genusAndSpecies',
      definition: 'Determined as genus and species',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'nec-1-inv-1',
      constraint: 'universality.objective = true',
      predicate: 'isObjective(universality)',
    },
    {
      id: 'nec-1-inv-2',
      constraint: 'necessity.posited = true',
      predicate: 'isPosited(necessity)',
    },
  ],

  forces: [
    {
      id: 'nec-1-force-1',
      description: 'Objective universality drives toward categorical judgment',
      type: 'mediation',
      trigger: 'objectiveUniversality.established = true',
      effect: 'categoricalJudgment.emerges = true',
      targetState: 'nec-2',
    },
  ],

  transitions: [
    {
      id: 'nec-1-trans-1',
      from: 'nec-1',
      to: 'nec-2',
      mechanism: 'mediation',
      description: 'From objective universality to categorical judgment',
    },
  ],

  nextStates: ['nec-2'],
  previousStates: ['reflection-ir'],

  provenance: {
    topicMapId: 'nec-1-introduction-objective',
    lineRange: { start: 4, end: 24 },
    section: 'The Judgment of Necessity',
    order: 1,
  },

  description: 'Universality advanced to objective universality, exists in and for itself. Posited necessity, distinction immanent. Determined as genus and species.',
};

const state2: DialecticState = {
  id: 'nec-2',
  title: 'Categorical judgment — genus divides into species',
  concept: 'CategoricalJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'genusDivides',
      definition: 'Genus essentially divides/repels into species',
      type: 'process',
    },
    {
      name: 'substantialIdentity',
      definition: 'Substantial identity of subject and predicate',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'genusDivides',
    },
    {
      name: 'totalityOfForm',
      definition: 'Totality of form reflected into itself, copula is necessity',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'nec-2-inv-1',
      constraint: 'genus.divides = species',
      predicate: 'divides(genus, species)',
    },
    {
      id: 'nec-2-inv-2',
      constraint: 'subject = predicate',
      predicate: 'equals(subject, predicate)',
    },
    {
      id: 'nec-2-inv-3',
      constraint: 'copula = necessity',
      predicate: 'equals(copula, necessity)',
    },
  ],

  forces: [
    {
      id: 'nec-2-force-1',
      description: 'Categorical judgment drives toward hypothetical judgment',
      type: 'mediation',
      trigger: 'necessity.inner = true',
      effect: 'hypotheticalJudgment.emerges = true',
      targetState: 'nec-5',
    },
  ],

  transitions: [
    {
      id: 'nec-2-trans-1',
      from: 'nec-2',
      to: 'nec-5',
      mechanism: 'mediation',
      description: 'From categorical to hypothetical judgment',
    },
  ],

  nextStates: ['nec-5'],
  previousStates: ['nec-1'],

  provenance: {
    topicMapId: 'nec-2-categorical-genus-divides',
    lineRange: { start: 28, end: 107 },
    section: 'The Categorical Judgment',
    order: 2,
  },

  description: 'Genus essentially divides into species. Substantial identity of subject and predicate. Totality of form reflected into itself. Copula is necessity. Passes to hypothetical.',
};

const state3: DialecticState = {
  id: 'nec-5',
  title: 'Hypothetical judgment — "If A is, then B is"',
  concept: 'HypotheticalJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'ifAThenB',
      definition: '"If A is, then B is" - being of A is being of an other (B)',
      type: 'determination',
    },
    {
      name: 'necessaryConnectedness',
      definition: 'Necessary connectedness of immediate determinacies',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'ifAThenB',
    },
    {
      name: 'beingOfAnOther',
      definition: 'Each extreme is being of an other, concrete self-identity',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'nec-5-inv-1',
      constraint: 'being(A) = being(B)',
      predicate: 'equals(being(A), being(B))',
    },
    {
      id: 'nec-5-inv-2',
      constraint: 'connectedness.necessary = true',
      predicate: 'isNecessary(connectedness)',
    },
  ],

  forces: [
    {
      id: 'nec-5-force-1',
      description: 'Hypothetical judgment drives toward disjunctive judgment',
      type: 'mediation',
      trigger: 'unity.concrete = true',
      effect: 'disjunctiveJudgment.emerges = true',
      targetState: 'nec-7',
    },
  ],

  transitions: [
    {
      id: 'nec-5-trans-1',
      from: 'nec-5',
      to: 'nec-7',
      mechanism: 'mediation',
      description: 'From hypothetical to disjunctive judgment',
    },
  ],

  nextStates: ['nec-7'],
  previousStates: ['nec-2'],

  provenance: {
    topicMapId: 'nec-5-hypothetical-if-then',
    lineRange: { start: 111, end: 204 },
    section: 'The Hypothetical Judgment',
    order: 3,
  },

  description: '"If A is, then B is" - being of A is being of an other. Necessary connectedness of immediate determinacies. Each extreme is being of an other. Passes to disjunctive.',
};

const state4: DialecticState = {
  id: 'nec-7',
  title: 'Disjunctive judgment — "A is either B or C"',
  concept: 'DisjunctiveJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'eitherBOrC',
      definition: '"A is either B or C" - necessity of concept',
      type: 'determination',
    },
    {
      name: 'genusAndTotality',
      definition: 'Contains genus in simple form and totality of differentiated determinations',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'eitherBOrC',
    },
    {
      name: 'positiveAndNegativeIdentity',
      definition: 'Positive identity ("as well as") and negative connection ("either or")',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'nec-7-inv-1',
      constraint: 'A = B ∨ A = C',
      predicate: 'or(equals(A, B), equals(A, C))',
    },
    {
      id: 'nec-7-inv-2',
      constraint: 'genus = unity(B, C)',
      predicate: 'equals(genus, unity(B, C))',
    },
  ],

  forces: [
    {
      id: 'nec-7-force-1',
      description: 'Disjunctive judgment drives toward proximate genus',
      type: 'mediation',
      trigger: 'negativeUnity.established = true',
      effect: 'proximateGenus.emerges = true',
      targetState: 'nec-11',
    },
  ],

  transitions: [
    {
      id: 'nec-7-trans-1',
      from: 'nec-7',
      to: 'nec-11',
      mechanism: 'mediation',
      description: 'From disjunctive to proximate genus',
    },
  ],

  nextStates: ['nec-11'],
  previousStates: ['nec-5'],

  provenance: {
    topicMapId: 'nec-7-disjunctive-either-or',
    lineRange: { start: 208, end: 260 },
    section: 'The Disjunctive Judgment',
    order: 4,
  },

  description: '"A is either B or C" - necessity of concept. Contains genus and totality of differentiated determinations. Positive identity ("as well as") and negative connection ("either or").',
};

const state5: DialecticState = {
  id: 'nec-11',
  title: 'Proximate genus — concrete essentially determined universality',
  concept: 'ProximateGenus',
  phase: 'subject',

  moments: [
    {
      name: 'proximateGenus',
      definition: 'Through negative unity, genus becomes proximate genus',
      type: 'determination',
    },
    {
      name: 'concreteEssentiallyDetermined',
      definition: 'When genus is concrete, essentially determined: simple determinateness = unity of moments',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'proximateGenus',
    },
    {
      name: 'realDifference',
      definition: 'Real difference in species, specific difference in essential determinateness',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'nec-11-inv-1',
      constraint: 'genus.proximate = true',
      predicate: 'isProximate(genus)',
    },
    {
      id: 'nec-11-inv-2',
      constraint: 'genus.concrete = true',
      predicate: 'isConcrete(genus)',
    },
  ],

  forces: [
    {
      id: 'nec-11-force-1',
      description: 'Proximate genus drives toward concept as posited',
      type: 'mediation',
      trigger: 'genus.proximate = true',
      effect: 'conceptAsPosited.emerges = true',
      targetState: 'nec-15',
    },
  ],

  transitions: [
    {
      id: 'nec-11-trans-1',
      from: 'nec-11',
      to: 'nec-15',
      mechanism: 'mediation',
      description: 'From proximate genus to concept as posited',
    },
  ],

  nextStates: ['nec-15'],
  previousStates: ['nec-7'],

  provenance: {
    topicMapId: 'nec-11-proximate-genus',
    lineRange: { start: 318, end: 353 },
    section: 'The Disjunctive Judgment',
    order: 5,
  },

  description: 'Through negative unity, genus becomes proximate genus. When genus is concrete, essentially determined: simple determinateness = unity of moments. Real difference in species.',
};

const state6: DialecticState = {
  id: 'nec-15',
  title: 'Transition to judgment of concept — concept as posited',
  concept: 'TransitionToJudgmentOfConcept',
  phase: 'subject',

  moments: [
    {
      name: 'judgmentDisjoined',
      definition: 'Judgment itself disjoined, subject/predicate are members',
      type: 'determination',
    },
    {
      name: 'momentsOfConcept',
      definition: 'Subject/predicate are moments of concept, identical',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'judgmentDisjoined',
    },
    {
      name: 'conceptAsPosited',
      definition: 'Unity (copula) is concept itself, concept as posited',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'nec-15-inv-1',
      constraint: 'subject = moment(concept)',
      predicate: 'equals(subject, moment(concept))',
    },
    {
      id: 'nec-15-inv-2',
      constraint: 'predicate = moment(concept)',
      predicate: 'equals(predicate, moment(concept))',
    },
    {
      id: 'nec-15-inv-3',
      constraint: 'copula = concept',
      predicate: 'equals(copula, concept)',
    },
  ],

  forces: [
    {
      id: 'nec-15-force-1',
      description: 'Concept as posited transitions to judgment of concept',
      type: 'passover',
      trigger: 'concept.posited = true',
      effect: 'judgmentOfConcept.emerges = true',
      targetState: 'concept-judgment-1',
    },
  ],

  transitions: [
    {
      id: 'nec-15-trans-1',
      from: 'nec-15',
      to: 'concept-judgment-1',
      mechanism: 'passover',
      description: 'From judgment of necessity to judgment of concept',
    },
  ],

  nextStates: ['concept-judgment-1'],
  previousStates: ['nec-11'],

  provenance: {
    topicMapId: 'nec-15-transition-concept',
    lineRange: { start: 424, end: 450 },
    section: 'The Disjunctive Judgment',
    order: 6,
  },

  description: 'Judgment itself disjoined. Subject/predicate are moments of concept, identical. Unity (copula) is concept itself, concept as posited. Judgment of necessity risen to judgment of concept.',
};

export const necessityIR: DialecticIR = {
  id: 'necessity-ir',
  title: 'Necessity IR: Categorical, Hypothetical, Disjunctive, Concept as Posited',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - C. Judgment of Necessity',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'necessity.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'nec-1': 'subject',
      'nec-2': 'subject',
      'nec-5': 'subject',
      'nec-7': 'subject',
      'nec-11': 'subject',
      'nec-15': 'subject',
    },
  },
};

export const necessityStates = {
  'nec-1': state1,
  'nec-2': state2,
  'nec-5': state3,
  'nec-7': state4,
  'nec-11': state5,
  'nec-15': state6,
};
