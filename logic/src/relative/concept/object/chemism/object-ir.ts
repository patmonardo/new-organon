/**
 * Chemical Object IR: Dialectic Pseudo-Code for The Chemical Object
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - A. The Chemical Object
 *
 * Covers the dialectical movement:
 * - Distinguished from Mechanical: Determinateness belongs to nature, non-indifference
 * - Self-Subsistent Totality: Reflected into itself, negative unity in two objects
 * - Contradiction and Striving: Tension, impulse to sublate, self-determining
 *
 * The GPU's chemical object - non-indifferent, tension, striving
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'chem-obj-1',
  title: 'Chemical object — determinateness belongs to nature, non-indifference',
  concept: 'ChemicalObjectNonIndifference',
  phase: 'object',

  moments: [
    {
      name: 'distinguishedFromMechanical',
      definition: 'Determinateness belongs to nature, reference to other is principle',
      type: 'determination',
    },
    {
      name: 'particularizationAsUniversality',
      definition: 'Determinateness is particularization taken up into universality, universal principle',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'distinguishedFromMechanical',
    },
    {
      name: 'necessityAndImpulse',
      definition: 'Necessity and impulse to sublate opposed one-sided subsistence',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'chem-obj-1-inv-1',
      constraint: 'determinateness.belongsToNature = true',
      predicate: 'belongsToNature(determinateness)',
    },
    {
      id: 'chem-obj-1-inv-2',
      constraint: 'object.nonIndifferent = true',
      predicate: 'isNonIndifferent(object)',
    },
  ],

  forces: [
    {
      id: 'chem-obj-1-force-1',
      description: 'Non-indifference drives toward contradiction and striving',
      type: 'mediation',
      trigger: 'determinateness.reflected = true',
      effect: 'contradiction.emerges = true',
      targetState: 'chem-obj-4',
    },
  ],

  transitions: [
    {
      id: 'chem-obj-1-trans-1',
      from: 'chem-obj-1',
      to: 'chem-obj-4',
      mechanism: 'mediation',
      description: 'From non-indifference to contradiction',
    },
  ],

  nextStates: ['chem-obj-4'],
  previousStates: ['mechanical-process-ir'],

  provenance: {
    topicMapId: 'chem-obj-1-distinguished-mechanical',
    lineRange: { start: 2, end: 66 },
    section: 'The Chemical Object',
    order: 1,
  },

  description: 'Chemical object distinguished from mechanical. Determinateness belongs to nature. Particularization as universality, principle. Chemism includes sex relation, love, friendship.',
};

const state2: DialecticState = {
  id: 'chem-obj-4',
  title: 'Contradiction and striving — tension, self-determining',
  concept: 'ChemicalContradiction',
  phase: 'object',

  moments: [
    {
      name: 'absolutelyReflected',
      definition: 'Determinateness absolutely reflected into itself, real genus',
      type: 'determination',
    },
    {
      name: 'contradiction',
      definition: 'Contradiction of immediate positedness and immanent individual concept',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'absolutelyReflected',
    },
    {
      name: 'strivingAndTension',
      definition: 'Striving to sublate immediate determinateness, tension with lack of self-subsistence',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'chem-obj-4-inv-1',
      constraint: 'determinateness.absolutelyReflected = true',
      predicate: 'isAbsolutelyReflected(determinateness)',
    },
    {
      id: 'chem-obj-4-inv-2',
      constraint: 'object = contradiction',
      predicate: 'isContradiction(object)',
    },
  ],

  forces: [
    {
      id: 'chem-obj-4-force-1',
      description: 'Contradiction drives toward chemical process',
      type: 'mediation',
      trigger: 'striving.selfDetermining = true',
      effect: 'chemicalProcess.emerges = true',
      targetState: 'chem-proc-1',
    },
  ],

  transitions: [
    {
      id: 'chem-obj-4-trans-1',
      from: 'chem-obj-4',
      to: 'chem-proc-1',
      mechanism: 'mediation',
      description: 'From contradiction to chemical process',
    },
  ],

  nextStates: ['chem-proc-1'],
  previousStates: ['chem-obj-1'],

  provenance: {
    topicMapId: 'chem-obj-4-contradiction-striving',
    lineRange: { start: 67, end: 84 },
    section: 'The Chemical Object',
    order: 2,
  },

  description: 'Determinateness absolutely reflected, real genus. Contradiction of immediate positedness and immanent concept. Striving to sublate, tension. Initiates process as self-determining.',
};

export const chemicalObjectIR: DialecticIR = {
  id: 'chemical-object-ir',
  title: 'Chemical Object IR: Non-Indifference, Contradiction, Striving',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - A. The Chemical Object',
  states: [state1, state2],
  metadata: {
    sourceFile: 'object.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'chem-obj-1': 'object',
      'chem-obj-4': 'object',
    },
  },
};

export const chemicalObjectStates = {
  'chem-obj-1': state1,
  'chem-obj-4': state2,
};
