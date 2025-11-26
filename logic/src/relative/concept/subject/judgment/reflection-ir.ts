/**
 * Reflection IR: Dialectic Pseudo-Code for Judgment of Reflection
 *
 * Architecture: Knowledge Processor (Predicative Logic / MVC Applications)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - B. Judgment of Reflection
 *
 * Covers the dialectical movement:
 * - Singular Judgment: "This is essential universal" (truth in particular)
 * - Particular Judgment: "Some singulars are universal" (indeterminate)
 * - Universal Judgment: "All humans" → "The human being" (genus, objective universality)
 * - Transition to Judgment of Necessity
 *
 * Movement in subject (reflected in-itselfness), judgments of subsumption
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'refl-1',
  title: 'Universal collected into unity — determinate content',
  concept: 'UniversalCollectedIntoUnity',
  phase: 'subject',

  moments: [
    {
      name: 'collectedIntoUnity',
      definition: 'Universal collected into unity through connection of different terms',
      type: 'mediation',
    },
    {
      name: 'determinateContent',
      definition: 'First have determinate content, form determination reflected into identity',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'collectedIntoUnity',
    },
    {
      name: 'relationalDetermination',
      definition: 'Express essentiality as relational determination, comprehensive universality',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'refl-1-inv-1',
      constraint: 'universal = collectedIntoUnity',
      predicate: 'equals(universal, collectedIntoUnity)',
    },
    {
      id: 'refl-1-inv-2',
      constraint: 'content.determinate = true',
      predicate: 'isDeterminate(content)',
    },
  ],

  forces: [
    {
      id: 'refl-1-force-1',
      description: 'Collected unity drives toward singular judgment',
      type: 'mediation',
      trigger: 'unity.collected = true',
      effect: 'singularJudgment.emerges = true',
      targetState: 'refl-5',
    },
  ],

  transitions: [
    {
      id: 'refl-1-trans-1',
      from: 'refl-1',
      to: 'refl-5',
      mechanism: 'mediation',
      description: 'From collected unity to singular judgment',
    },
  ],

  nextStates: ['refl-5'],
  previousStates: ['existence-ir'],

  provenance: {
    topicMapId: 'refl-1-introduction-unity',
    lineRange: { start: 3, end: 57 },
    section: 'The Judgment of Reflection',
    order: 1,
  },

  description: 'Universal collected into unity through connection. Determinate content, form determination reflected into identity. Relational determination, comprehensive universality.',
};

const state2: DialecticState = {
  id: 'refl-5',
  title: 'Singular judgment — "This is essential universal"',
  concept: 'SingularJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'thisIsEssentialUniversal',
      definition: 'Immediate judgment: "this is an essential universal"',
      type: 'determination',
    },
    {
      name: 'notAThis',
      definition: 'Negation: "not a this" = universal of reflection',
      type: 'negation',
      relation: 'transforms',
      relatedTo: 'thisIsEssentialUniversal',
    },
    {
      name: 'truthInParticular',
      definition: 'Singular judgment has proximate truth in particular judgment',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'refl-5-inv-1',
      constraint: '"this" ≠ essentialUniversal',
      predicate: 'not(equals(this, essentialUniversal))',
    },
    {
      id: 'refl-5-inv-2',
      constraint: 'subject.alterable = true',
      predicate: 'isAlterable(subject)',
    },
  ],

  forces: [
    {
      id: 'refl-5-force-1',
      description: 'Singular judgment drives toward particular judgment',
      type: 'negation',
      trigger: 'singularJudgment.notTrue = true',
      effect: 'particularJudgment.emerges = true',
      targetState: 'refl-6',
    },
  ],

  transitions: [
    {
      id: 'refl-5-trans-1',
      from: 'refl-5',
      to: 'refl-6',
      mechanism: 'negation',
      description: 'From singular to particular judgment',
    },
  ],

  nextStates: ['refl-6'],
  previousStates: ['refl-1'],

  provenance: {
    topicMapId: 'refl-5-singular-judgment',
    lineRange: { start: 117, end: 142 },
    section: 'The Singular Judgment',
    order: 2,
  },

  description: 'Immediate judgment: "this is an essential universal". But "this" not essential universal. Negation: "not a this" = universal of reflection. Truth in particular judgment.',
};

const state3: DialecticState = {
  id: 'refl-6',
  title: 'Particular judgment — "Some singulars are universal"',
  concept: 'ParticularJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'someSingularsAreUniversal',
      definition: 'Extension of singular: "some singulars are universal"',
      type: 'determination',
    },
    {
      name: 'positiveAndNegative',
      definition: '"Some" contains universality but disproportionate, positive and negative',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'someSingularsAreUniversal',
    },
    {
      name: 'indeterminate',
      definition: 'Particular judgment is indeterminate',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'refl-6-inv-1',
      constraint: 'some = particularity',
      predicate: 'equals(some, particularity)',
    },
    {
      id: 'refl-6-inv-2',
      constraint: 'particularity.disproportionate = true',
      predicate: 'isDisproportionate(particularity)',
    },
  ],

  forces: [
    {
      id: 'refl-6-force-1',
      description: 'Particular judgment drives toward universal judgment',
      type: 'mediation',
      trigger: 'particularity.indeterminate = true',
      effect: 'universalJudgment.emerges = true',
      targetState: 'refl-9',
    },
  ],

  transitions: [
    {
      id: 'refl-6-trans-1',
      from: 'refl-6',
      to: 'refl-9',
      mechanism: 'mediation',
      description: 'From particular to universal judgment',
    },
  ],

  nextStates: ['refl-9'],
  previousStates: ['refl-5'],

  provenance: {
    topicMapId: 'refl-6-particular-extension',
    lineRange: { start: 146, end: 183 },
    section: 'The Particular Judgment',
    order: 3,
  },

  description: 'Extension of singular: "some singulars are universal". "Some" contains universality but disproportionate. Positive and negative no longer fall outside one another. Indeterminate.',
};

const state4: DialecticState = {
  id: 'refl-9',
  title: 'Universal judgment — "Allness" as external universality',
  concept: 'UniversalJudgmentAllness',
  phase: 'subject',

  moments: [
    {
      name: 'allness',
      definition: 'Universality as external universality of reflection, "allness"',
      type: 'determination',
    },
    {
      name: 'badInfinity',
      definition: 'Allness exhausted in singulars = relapse into bad infinity',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'allness',
    },
    {
      name: 'intimationOfConcept',
      definition: 'Obscure intimation: universality of concept in and for itself',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'refl-9-inv-1',
      constraint: 'allness = externalUniversality',
      predicate: 'equals(allness, externalUniversality)',
    },
    {
      id: 'refl-9-inv-2',
      constraint: 'allness = badInfinity',
      predicate: 'equals(allness, badInfinity)',
    },
  ],

  forces: [
    {
      id: 'refl-9-force-1',
      description: 'Allness drives toward objective universality',
      type: 'mediation',
      trigger: 'allness.exhausted = true',
      effect: 'objectiveUniversality.emerges = true',
      targetState: 'refl-12',
    },
  ],

  transitions: [
    {
      id: 'refl-9-trans-1',
      from: 'refl-9',
      to: 'refl-12',
      mechanism: 'mediation',
      description: 'From allness to objective universality',
    },
  ],

  nextStates: ['refl-12'],
  previousStates: ['refl-6'],

  provenance: {
    topicMapId: 'refl-9-allness-external',
    lineRange: { start: 243, end: 295 },
    section: 'The Universal Judgment',
    order: 4,
  },

  description: 'Universality as "allness", external universality. Only commonality of self-subsisting singulars. Bad infinity: allness exhausted in singulars. Obscure intimation of concept in and for itself.',
};

const state5: DialecticState = {
  id: 'refl-12',
  title: 'Objective universality — "The human being"',
  concept: 'ObjectiveUniversality',
  phase: 'subject',

  moments: [
    {
      name: 'objectiveUniversality',
      definition: 'Singularity expanded to allness = objective universality',
      type: 'determination',
    },
    {
      name: 'absoluteDeterminateness',
      definition: 'Determination identical with universality, absolute determinateness',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'objectiveUniversality',
    },
    {
      name: 'shedFormDetermination',
      definition: 'Subject shed form determination: "All humans" → "the human being"',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'refl-12-inv-1',
      constraint: 'singularity.expanded = allness',
      predicate: 'equals(singularity.expanded, allness)',
    },
    {
      id: 'refl-12-inv-2',
      constraint: 'determination = universality',
      predicate: 'equals(determination, universality)',
    },
  ],

  forces: [
    {
      id: 'refl-12-force-1',
      description: 'Objective universality drives toward genus',
      type: 'mediation',
      trigger: 'objectiveUniversality.established = true',
      effect: 'genus.emerges = true',
      targetState: 'refl-13',
    },
  ],

  transitions: [
    {
      id: 'refl-12-trans-1',
      from: 'refl-12',
      to: 'refl-13',
      mechanism: 'mediation',
      description: 'From objective universality to genus',
    },
  ],

  nextStates: ['refl-13'],
  previousStates: ['refl-9'],

  provenance: {
    topicMapId: 'refl-12-objective-universality',
    lineRange: { start: 342, end: 370 },
    section: 'The Universal Judgment',
    order: 5,
  },

  description: 'Singularity expanded to allness = objective universality. Determination identical with universality, absolute determinateness. Subject shed form determination: "All humans" → "the human being".',
};

const state6: DialecticState = {
  id: 'refl-13',
  title: 'Genus — concrete universality, judgment sublated',
  concept: 'Genus',
  phase: 'subject',

  moments: [
    {
      name: 'genus',
      definition: 'Universality as genus, concrete in universality',
      type: 'determination',
    },
    {
      name: 'relationReversed',
      definition: 'Subject not subsumed, predicate particular, relation reversed',
      type: 'negation',
      relation: 'transforms',
      relatedTo: 'genus',
    },
    {
      name: 'judgmentSublated',
      definition: 'Judgment sublated, transition to judgment of necessity',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'refl-13-inv-1',
      constraint: 'universality = genus',
      predicate: 'equals(universality, genus)',
    },
    {
      id: 'refl-13-inv-2',
      constraint: 'subject.notSubsumed = true',
      predicate: 'notSubsumed(subject)',
    },
    {
      id: 'refl-13-inv-3',
      constraint: 'judgment.sublated = true',
      predicate: 'isSublated(judgment)',
    },
  ],

  forces: [
    {
      id: 'refl-13-force-1',
      description: 'Genus drives toward judgment of necessity',
      type: 'passover',
      trigger: 'judgment.sublated = true',
      effect: 'judgmentOfNecessity.emerges = true',
      targetState: 'necessity-1',
    },
  ],

  transitions: [
    {
      id: 'refl-13-trans-1',
      from: 'refl-13',
      to: 'necessity-1',
      mechanism: 'passover',
      description: 'From genus to judgment of necessity',
    },
  ],

  nextStates: ['necessity-1'],
  previousStates: ['refl-12'],

  provenance: {
    topicMapId: 'refl-13-genus',
    lineRange: { start: 372, end: 432 },
    section: 'The Universal Judgment',
    order: 6,
  },

  description: 'Universality as genus, concrete in universality. Subject not subsumed, predicate particular, relation reversed. Judgment sublated. Transition to judgment of necessity.',
};

export const reflectionIR: DialecticIR = {
  id: 'reflection-ir',
  title: 'Reflection IR: Singular, Particular, Universal Judgment, Genus',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - B. Judgment of Reflection',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'reflection.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'refl-1': 'subject',
      'refl-5': 'subject',
      'refl-6': 'subject',
      'refl-9': 'subject',
      'refl-12': 'subject',
      'refl-13': 'subject',
    },
  },
};

export const reflectionStates = {
  'refl-1': state1,
  'refl-5': state2,
  'refl-6': state3,
  'refl-9': state4,
  'refl-12': state5,
  'refl-13': state6,
};
