/**
 * Concept IR: Dialectic Pseudo-Code for Judgment of the Concept
 *
 * Architecture: Knowledge Processor (Predicative Logic / MVC Applications)
 * Section: C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - D. Judgment of the Concept
 *
 * Covers the dialectical movement:
 * - Assertoric Judgment: Immediate, subjective assurance, ought and constitution
 * - Problematic Judgment: Subject differentiated, duplicity, contingency
 * - Apodictic Judgment: Truly objective, ground in constitution, absolute judgment
 * - Transition to Syllogism (copula replete of content)
 *
 * Concept as ought, true adjudication - judgment becomes objective truth
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'conc-1',
  title: 'Concept as ought — true adjudication',
  concept: 'ConceptAsOught',
  phase: 'subject',

  moments: [
    {
      name: 'conceptAsOught',
      definition: 'Concept is ought to which reality may or may not conform',
      type: 'determination',
    },
    {
      name: 'trueAdjudication',
      definition: 'First contains true adjudication, fact measured against concept',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'conceptAsOught',
    },
    {
      name: 'predicatesOfValue',
      definition: 'Predicates: "good," "bad," "true," "right"',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'conc-1-inv-1',
      constraint: 'concept = ought',
      predicate: 'equals(concept, ought)',
    },
    {
      id: 'conc-1-inv-2',
      constraint: 'judgment = trueAdjudication',
      predicate: 'equals(judgment, trueAdjudication)',
    },
  ],

  forces: [
    {
      id: 'conc-1-force-1',
      description: 'Concept as ought drives toward assertoric judgment',
      type: 'mediation',
      trigger: 'concept.asOught = true',
      effect: 'assertoricJudgment.emerges = true',
      targetState: 'conc-4',
    },
  ],

  transitions: [
    {
      id: 'conc-1-trans-1',
      from: 'conc-1',
      to: 'conc-4',
      mechanism: 'mediation',
      description: 'From concept as ought to assertoric judgment',
    },
  ],

  nextStates: ['conc-4'],
  previousStates: ['necessity-ir'],

  provenance: {
    topicMapId: 'conc-1-introduction-ought',
    lineRange: { start: 4, end: 19 },
    section: 'The Judgment of the Concept',
    order: 1,
  },

  description: 'Concept is ought to which reality may or may not conform. First contains true adjudication. Predicates: "good," "bad," "true," "right" - fact measured against concept.',
};

const state2: DialecticState = {
  id: 'conc-4',
  title: 'Assertoric judgment — ought and constitution',
  concept: 'AssertorcJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'assertoric',
      definition: 'At first immediate, assertoric - subjective assurance',
      type: 'determination',
    },
    {
      name: 'oughtAndConstitution',
      definition: 'Subject ought to be (universal nature), constitution (particularity, may or may not conform)',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'assertoric',
    },
    {
      name: 'externalThird',
      definition: 'Good/bad/right hangs on external third, connectedness externally posited',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'conc-4-inv-1',
      constraint: 'judgment.assertoric = true',
      predicate: 'isAssertoric(judgment)',
    },
    {
      id: 'conc-4-inv-2',
      constraint: 'subject = ought + constitution',
      predicate: 'equals(subject, union(ought, constitution))',
    },
  ],

  forces: [
    {
      id: 'conc-4-force-1',
      description: 'Assertoric judgment drives toward problematic judgment',
      type: 'negation',
      trigger: 'conformity.contingent = true',
      effect: 'problematicJudgment.emerges = true',
      targetState: 'conc-7',
    },
  ],

  transitions: [
    {
      id: 'conc-4-trans-1',
      from: 'conc-4',
      to: 'conc-7',
      mechanism: 'negation',
      description: 'From assertoric to problematic judgment',
    },
  ],

  nextStates: ['conc-7'],
  previousStates: ['conc-1'],

  provenance: {
    topicMapId: 'conc-4-assertoric-immediate',
    lineRange: { start: 95, end: 164 },
    section: 'The Assertoric Judgment',
    order: 2,
  },

  description: 'Assertoric judgment is immediate, subjective assurance. Subject ought to be (universal nature), constitution (particularity). Contingent whether conformity to concept. Essentially problematic.',
};

const state3: DialecticState = {
  id: 'conc-7',
  title: 'Problematic judgment — subject differentiated',
  concept: 'ProblematicJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'problematic',
      definition: 'Problematic is assertoric positive and negative, more immanent',
      type: 'determination',
    },
    {
      name: 'subjectDifferentiated',
      definition: 'Subject differentiated into universality/ought and particularized constitution',
      type: 'process',
      relation: 'contains',
      relatedTo: 'problematic',
    },
    {
      name: 'duplicity',
      definition: 'Subject is duplicity - two meanings of subjectivity in one',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'conc-7-inv-1',
      constraint: 'judgment.problematic = true',
      predicate: 'isProblematic(judgment)',
    },
    {
      id: 'conc-7-inv-2',
      constraint: 'subject = ought + constitution',
      predicate: 'equals(subject, differentiated(ought, constitution))',
    },
  ],

  forces: [
    {
      id: 'conc-7-force-1',
      description: 'Problematic judgment drives toward apodictic judgment',
      type: 'mediation',
      trigger: 'problematicCharacter.posited = true',
      effect: 'apodeictic.emerges = true',
      targetState: 'conc-10',
    },
  ],

  transitions: [
    {
      id: 'conc-7-trans-1',
      from: 'conc-7',
      to: 'conc-10',
      mechanism: 'mediation',
      description: 'From problematic to apodictic judgment',
    },
  ],

  nextStates: ['conc-10'],
  previousStates: ['conc-4'],

  provenance: {
    topicMapId: 'conc-7-problematic-differentiated',
    lineRange: { start: 168, end: 258 },
    section: 'The Problematic Judgment',
    order: 3,
  },

  description: 'Problematic is assertoric positive and negative. Subject differentiated into ought and constitution. Subject is duplicity - two meanings of subjectivity. When problematic character posited, becomes apodictic.',
};

const state4: DialecticState = {
  id: 'conc-10',
  title: 'Apodictic judgment — truly objective',
  concept: 'ApodeiticJudgment',
  phase: 'subject',

  moments: [
    {
      name: 'apodictic',
      definition: 'Truly objective, subject and predicate correspond, same concept',
      type: 'determination',
    },
    {
      name: 'groundInConstitution',
      definition: 'Subject includes universal/ought and constitution (ground)',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'apodictic',
    },
    {
      name: 'concreteUniversality',
      definition: 'Content is concrete universality, universal continues through opposite',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'conc-10-inv-1',
      constraint: 'judgment.objective = true',
      predicate: 'isObjective(judgment)',
    },
    {
      id: 'conc-10-inv-2',
      constraint: 'subject = predicate',
      predicate: 'corresponds(subject, predicate)',
    },
  ],

  forces: [
    {
      id: 'conc-10-force-1',
      description: 'Apodictic judgment drives toward transition to syllogism',
      type: 'sublation',
      trigger: 'copula.accomplished = true',
      effect: 'syllogism.emerges = true',
      targetState: 'conc-14',
    },
  ],

  transitions: [
    {
      id: 'conc-10-trans-1',
      from: 'conc-10',
      to: 'conc-14',
      mechanism: 'sublation',
      description: 'From apodictic to transition to syllogism',
    },
  ],

  nextStates: ['conc-14'],
  previousStates: ['conc-7'],

  provenance: {
    topicMapId: 'conc-10-apodictic-objective',
    lineRange: { start: 262, end: 288 },
    section: 'The Apodictic Judgment',
    order: 4,
  },

  description: 'Truly objective, truth of judgment. Subject and predicate correspond, same concept. Content is concrete universality. Ground in constitution.',
};

const state5: DialecticState = {
  id: 'conc-14',
  title: 'Transition to syllogism — copula replete of content',
  concept: 'TransitionToSyllogism',
  phase: 'subject',

  moments: [
    {
      name: 'accomplishedCopula',
      definition: 'Accomplished copula, abstract "is" developed into ground',
      type: 'determination',
    },
    {
      name: 'rupleteOfContent',
      definition: 'Copula replete of content, contains form determinations and determinate connection',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'accomplishedCopula',
    },
    {
      name: 'becomeSyllogism',
      definition: 'By virtue of repletion, judgment has become syllogism',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'conc-14-inv-1',
      constraint: 'copula.accomplished = true',
      predicate: 'isAccomplished(copula)',
    },
    {
      id: 'conc-14-inv-2',
      constraint: 'copula.replete = true',
      predicate: 'isReplete(copula)',
    },
    {
      id: 'conc-14-inv-3',
      constraint: 'judgment = syllogism',
      predicate: 'equals(judgment, syllogism)',
    },
  ],

  forces: [
    {
      id: 'conc-14-force-1',
      description: 'Copula replete transitions to syllogism',
      type: 'passover',
      trigger: 'copula.replete = true',
      effect: 'syllogism.emerges = true',
      targetState: 'syllogism-1',
    },
  ],

  transitions: [
    {
      id: 'conc-14-trans-1',
      from: 'conc-14',
      to: 'syllogism-1',
      mechanism: 'passover',
      description: 'From judgment to syllogism',
    },
  ],

  nextStates: ['syllogism-1'],
  previousStates: ['conc-10'],

  provenance: {
    topicMapId: 'conc-14-transition-syllogism',
    lineRange: { start: 346, end: 374 },
    section: 'The Apodictic Judgment',
    order: 5,
  },

  description: 'Accomplished copula, replete of content. Contains form determinations and determinate connection. Unity re-emerging. By virtue of repletion, judgment has become syllogism.',
};

export const conceptJudgmentIR: DialecticIR = {
  id: 'concept-judgment-ir',
  title: 'Concept Judgment IR: Assertoric, Problematic, Apodictic, Syllogism',
  section: 'C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - D. Judgment of the Concept',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'concept.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'conc-1': 'subject',
      'conc-4': 'subject',
      'conc-7': 'subject',
      'conc-10': 'subject',
      'conc-14': 'subject',
    },
  },
};

export const conceptJudgmentStates = {
  'conc-1': state1,
  'conc-4': state2,
  'conc-7': state3,
  'conc-10': state4,
  'conc-14': state5,
};
