/**
 * Chemism IR: Dialectic Pseudo-Code for Transition of Chemism
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - C. Transition of Chemism
 *
 * Covers the dialectical movement:
 * - Three Syllogisms: Formal neutrality, real neutrality, self-realizing concept
 * - Neutralization and Dissolution: Two essential moments
 * - Concept Liberated: Completely freed from objective externality, purpose
 *
 * The GPU's chemical object - tension, neutralization, transition to purpose
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'chem-1',
  title: 'Three syllogisms — neutralization and dissolution',
  concept: 'ChemicalSyllogisms',
  phase: 'object',

  moments: [
    {
      name: 'threeSyllogisms',
      definition: 'Three syllogisms constitute totality: formal neutrality, real neutrality, self-realizing concept',
      type: 'determination',
    },
    {
      name: 'neutralizationAndDissolution',
      definition: 'Two essential moments: neutralization and dissolution/reduction',
      type: 'process',
      relation: 'contains',
      relatedTo: 'threeSyllogisms',
    },
    {
      name: 'externalityAndConditionality',
      definition: 'Still burdened by externality, processes fall apart, conditioned from outside',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'chem-1-inv-1',
      constraint: 'syllogisms.count = 3',
      predicate: 'equals(syllogisms.count, 3)',
    },
    {
      id: 'chem-1-inv-2',
      constraint: 'chemism = firstNegation(indifferentObjectivity)',
      predicate: 'equals(chemism, firstNegation(indifferentObjectivity))',
    },
  ],

  forces: [
    {
      id: 'chem-1-force-1',
      description: 'Chemical processes drive toward concept liberated',
      type: 'sublation',
      trigger: 'externality.sublated = true',
      effect: 'conceptLiberated.emerges = true',
      targetState: 'chem-2',
    },
  ],

  transitions: [
    {
      id: 'chem-1-trans-1',
      from: 'chem-1',
      to: 'chem-2',
      mechanism: 'sublation',
      description: 'From chemical syllogisms to concept liberated',
    },
  ],

  nextStates: ['chem-2'],
  previousStates: ['mechanism-ir'],

  provenance: {
    topicMapId: 'chemism-2',
    lineRange: { start: 25, end: 91 },
    section: 'C. TRANSITION OF CHEMISM',
    order: 1,
  },

  description: 'Chemism is first negation of indifferent objectivity. Three syllogisms constitute totality. Two essential moments: neutralization and dissolution. Processes are stages sublating externality and conditionality.',
};

const state2: DialecticState = {
  id: 'chem-2',
  title: 'Concept liberated — objective free concept is purpose',
  concept: 'ConceptLiberated',
  phase: 'object',

  moments: [
    {
      name: 'conceptLiberated',
      definition: 'Concept completely liberated from objective externality',
      type: 'sublation',
    },
    {
      name: 'objectiveFreeConceptIsPurpose',
      definition: 'Objective free concept is purpose',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'conceptLiberated',
    },
  ],

  invariants: [
    {
      id: 'chem-2-inv-1',
      constraint: 'concept.liberated = true',
      predicate: 'isLiberated(concept)',
    },
    {
      id: 'chem-2-inv-2',
      constraint: 'objectiveFreeConcept = purpose',
      predicate: 'equals(objectiveFreeConcept, purpose)',
    },
  ],

  forces: [
    {
      id: 'chem-2-force-1',
      description: 'Concept liberated transitions to teleology',
      type: 'passover',
      trigger: 'concept.objectiveFree = true',
      effect: 'teleology.emerges = true',
      targetState: 'tele-1',
    },
  ],

  transitions: [
    {
      id: 'chem-2-trans-1',
      from: 'chem-2',
      to: 'tele-1',
      mechanism: 'passover',
      description: 'From chemism to teleology',
    },
  ],

  nextStates: ['tele-1'],
  previousStates: ['chem-1'],

  provenance: {
    topicMapId: 'chemism-6',
    lineRange: { start: 106, end: 119 },
    section: 'C. TRANSITION OF CHEMISM',
    order: 2,
  },

  description: 'Third syllogism sublates whole abstract external immediacy. Concept sublated as external all moments of objective existence. Completely liberated from objective externality. Objective free concept is purpose.',
};

export const chemismIR: DialecticIR = {
  id: 'chemism-ir',
  title: 'Chemism IR: Three Syllogisms, Concept Liberated, Purpose',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - C. Transition of Chemism',
  states: [state1, state2],
  metadata: {
    sourceFile: 'chemism.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'chem-1': 'object',
      'chem-2': 'object',
    },
  },
};

export const chemismStates = {
  'chem-1': state1,
  'chem-2': state2,
};
