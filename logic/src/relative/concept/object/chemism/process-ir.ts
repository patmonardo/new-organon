/**
 * Chemical Process IR: Dialectic Pseudo-Code for The Chemical Process
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - B. The Process
 *
 * Covers the dialectical movement:
 * - Presupposition: Objects in tension, affinity, middle term (element of communication)
 * - Product: Neutral object, tension dissolved, negativity outside
 * - Disjunctive Syllogism: Disruption, elemental objects, chemism sublates itself
 *
 * The GPU's chemical process - affinity, neutralization, transition to higher sphere
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'chem-proc-1',
  title: 'Presupposition — objects in tension, affinity, communication',
  concept: 'ChemicalPresupposition',
  phase: 'object',

  moments: [
    {
      name: 'objectsInTension',
      definition: 'Objects in tension, tensed against themselves and each other, affinity',
      type: 'determination',
    },
    {
      name: 'middleTerm',
      definition: 'Middle term - implicit nature, element of communication (water, language)',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'objectsInTension',
    },
    {
      name: 'communication',
      definition: 'Communication - tranquil coming-together and negative relating',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'chem-proc-1-inv-1',
      constraint: 'objects.inTension = true',
      predicate: 'areInTension(objects)',
    },
    {
      id: 'chem-proc-1-inv-2',
      constraint: 'middleTerm = elementOfCommunication',
      predicate: 'equals(middleTerm, elementOfCommunication)',
    },
  ],

  forces: [
    {
      id: 'chem-proc-1-force-1',
      description: 'Communication drives toward neutral product',
      type: 'mediation',
      trigger: 'striving.attainsTranquilNeutrality = true',
      effect: 'neutralProduct.emerges = true',
      targetState: 'chem-proc-4',
    },
  ],

  transitions: [
    {
      id: 'chem-proc-1-trans-1',
      from: 'chem-proc-1',
      to: 'chem-proc-4',
      mechanism: 'mediation',
      description: 'From affinity to neutral product',
    },
  ],

  nextStates: ['chem-proc-4'],
  previousStates: ['chemical-object-ir'],

  provenance: {
    topicMapId: 'process-1',
    lineRange: { start: 4, end: 60 },
    section: 'B. THE PROCESS',
    order: 1,
  },

  description: 'Objects in tension, affinity. Middle term - implicit nature, element of communication. Communication - tranquil coming-together, negative relating. Striving attains tranquil neutrality.',
};

const state2: DialecticState = {
  id: 'chem-proc-4',
  title: 'Product — neutral object, negativity outside',
  concept: 'ChemicalNeutralProduct',
  phase: 'object',

  moments: [
    {
      name: 'neutralProduct',
      definition: 'Product is neutral, tension dissolved, formal unity',
      type: 'determination',
    },
    {
      name: 'negativityOutside',
      definition: 'Negativity stepped outside neutral object, restless activity',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'neutralProduct',
    },
  ],

  invariants: [
    {
      id: 'chem-proc-4-inv-1',
      constraint: 'product.neutral = true',
      predicate: 'isNeutral(product)',
    },
    {
      id: 'chem-proc-4-inv-2',
      constraint: 'negativity.outside = true',
      predicate: 'isOutside(negativity)',
    },
  ],

  forces: [
    {
      id: 'chem-proc-4-force-1',
      description: 'Negativity outside drives toward disjunctive syllogism',
      type: 'mediation',
      trigger: 'negativity.restlessActivity = true',
      effect: 'disjunctiveSyllogism.emerges = true',
      targetState: 'chem-proc-6',
    },
  ],

  transitions: [
    {
      id: 'chem-proc-4-trans-1',
      from: 'chem-proc-4',
      to: 'chem-proc-6',
      mechanism: 'mediation',
      description: 'From neutral product to disjunctive syllogism',
    },
  ],

  nextStates: ['chem-proc-6'],
  previousStates: ['chem-proc-1'],

  provenance: {
    topicMapId: 'process-4',
    lineRange: { start: 62, end: 108 },
    section: 'B. THE PROCESS',
    order: 2,
  },

  description: 'Product is neutral, tension dissolved. Negativity stepped outside object, restless activity. Process does not spontaneously restart.',
};

const state3: DialecticState = {
  id: 'chem-proc-6',
  title: 'Disjunctive syllogism — disruption, elemental objects, chemism sublates itself',
  concept: 'ChemicalDisjunctiveSyllogism',
  phase: 'object',

  moments: [
    {
      name: 'disruption',
      definition: 'Disruption - real neutrality breaks up into neutral moments',
      type: 'mediation',
    },
    {
      name: 'disjunctiveSyllogism',
      definition: 'Disjunctive syllogism - totality of chemism, negative unity/real unity/abstract moments',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'disruption',
    },
    {
      name: 'elementalObjects',
      definition: 'Elemental objects liberated, chemism sublates itself, transitions to higher sphere',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'chem-proc-6-inv-1',
      constraint: 'syllogism.disjunctive = true',
      predicate: 'isDisjunctive(syllogism)',
    },
    {
      id: 'chem-proc-6-inv-2',
      constraint: 'chemism.sublatesItself = true',
      predicate: 'sublatesItself(chemism)',
    },
  ],

  forces: [
    {
      id: 'chem-proc-6-force-1',
      description: 'Chemism sublates itself, transitions to higher sphere',
      type: 'sublation',
      trigger: 'chemism.goesBackToConcept = true',
      effect: 'higherSphere.emerges = true',
      targetState: 'chem-1',
    },
  ],

  transitions: [
    {
      id: 'chem-proc-6-trans-1',
      from: 'chem-proc-6',
      to: 'chem-1',
      mechanism: 'sublation',
      description: 'From disjunctive syllogism to transition of chemism',
    },
  ],

  nextStates: ['chem-1'],
  previousStates: ['chem-proc-4'],

  provenance: {
    topicMapId: 'process-6',
    lineRange: { start: 110, end: 180 },
    section: 'B. THE PROCESS',
    order: 3,
  },

  description: 'Disjunctive syllogism - totality of chemism. Disruption, elemental objects liberated. Chemism gone back to beginning, sublates itself, transitions to higher sphere.',
};

export const chemicalProcessIR: DialecticIR = {
  id: 'chemical-process-ir',
  title: 'Chemical Process IR: Affinity, Neutral Product, Disjunctive Syllogism',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - B. The Process',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'process.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'chem-proc-1': 'object',
      'chem-proc-4': 'object',
      'chem-proc-6': 'object',
    },
  },
};

export const chemicalProcessStates = {
  'chem-proc-1': state1,
  'chem-proc-4': state2,
  'chem-proc-6': state3,
};
