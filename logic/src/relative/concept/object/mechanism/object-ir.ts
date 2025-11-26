/**
 * Mechanical Object IR: Dialectic Pseudo-Code for The Mechanical Object
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - A. The Mechanical Object
 *
 * Covers the dialectical movement:
 * - Object as Syllogism: Equilibrium, immediate identity, totality
 * - Indeterminate Plurality: Composite, aggregate, indifferent determinations
 * - Contradiction: Indifference + Identity → Mechanical Process
 *
 * The GPU's mechanical object - indifferent, external, self-subsistent
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'mech-obj-1',
  title: 'Object as syllogism — equilibrium, immediate identity',
  concept: 'MechanicalObjectAsSyllogism',
  phase: 'object',

  moments: [
    {
      name: 'syllogismWithEquilibrium',
      definition: 'Object is syllogism whose mediation attained equilibrium, immediate identity',
      type: 'determination',
    },
    {
      name: 'universalInAndForItself',
      definition: 'Universal in and for itself, pervading particularity, immediate singularity',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'syllogismWithEquilibrium',
    },
  ],

  invariants: [
    {
      id: 'mech-obj-1-inv-1',
      constraint: 'object = syllogism',
      predicate: 'equals(object, syllogism)',
    },
    {
      id: 'mech-obj-1-inv-2',
      constraint: 'mediation.equilibrium = true',
      predicate: 'hasEquilibrium(mediation)',
    },
  ],

  forces: [
    {
      id: 'mech-obj-1-force-1',
      description: 'Object drives toward indeterminate plurality',
      type: 'negation',
      trigger: 'differentiation.collapsed = true',
      effect: 'plurality.emerges = true',
      targetState: 'mech-obj-3',
    },
  ],

  transitions: [
    {
      id: 'mech-obj-1-trans-1',
      from: 'mech-obj-1',
      to: 'mech-obj-3',
      mechanism: 'negation',
      description: 'From syllogism to indeterminate plurality',
    },
  ],

  nextStates: ['mech-obj-3'],
  previousStates: ['necessity-syllogism-ir'],

  provenance: {
    topicMapId: 'mech-obj-1-syllogism-equilibrium',
    lineRange: { start: 2, end: 34 },
    section: 'The Mechanical Object',
    order: 1,
  },

  description: 'Object as syllogism with equilibrium. Universal in and for itself. Does not differentiate into matter/form. No properties, no accidents. Totality.',
};

const state2: DialecticState = {
  id: 'mech-obj-3',
  title: 'Indeterminate plurality — composite, aggregate, indifferent',
  concept: 'IndeterminatePlurality',
  phase: 'object',

  moments: [
    {
      name: 'indeterminatePlurality',
      definition: 'Object is indeterminate, plurality, composite, aggregate',
      type: 'determination',
    },
    {
      name: 'indifferentDeterminations',
      definition: 'Totality of determinateness yet indifferent to determinations, external form',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'indeterminatePlurality',
    },
    {
      name: 'determinatenessOutside',
      definition: 'Determinateness outside in other objects, infinite progression',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'mech-obj-3-inv-1',
      constraint: 'object.indeterminate = true',
      predicate: 'isIndeterminate(object)',
    },
    {
      id: 'mech-obj-3-inv-2',
      constraint: 'determinateness.outside = true',
      predicate: 'isOutside(determinateness)',
    },
  ],

  forces: [
    {
      id: 'mech-obj-3-force-1',
      description: 'Indeterminate plurality drives toward contradiction',
      type: 'negation',
      trigger: 'indifference.contradicts(identity) = true',
      effect: 'mechanicalProcess.emerges = true',
      targetState: 'mech-proc-1',
    },
  ],

  transitions: [
    {
      id: 'mech-obj-3-trans-1',
      from: 'mech-obj-3',
      to: 'mech-proc-1',
      mechanism: 'negation',
      description: 'From indeterminate plurality to mechanical process',
    },
  ],

  nextStates: ['mech-proc-1'],
  previousStates: ['mech-obj-1'],

  provenance: {
    topicMapId: 'mech-obj-3-indeterminate-plurality',
    lineRange: { start: 36, end: 157 },
    section: 'The Mechanical Object',
    order: 2,
  },

  description: 'Object is indeterminate plurality, composite, aggregate. Indifferent to determinations. Determinateness outside in other objects. Contradiction: perfect indifference + identity of determinateness. Mechanical process.',
};

export const mechanicalObjectIR: DialecticIR = {
  id: 'mechanical-object-ir',
  title: 'Mechanical Object IR: Syllogism, Plurality, Contradiction',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - A. The Mechanical Object',
  states: [state1, state2],
  metadata: {
    sourceFile: 'object.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'mech-obj-1': 'object',
      'mech-obj-3': 'object',
    },
  },
};

export const mechanicalObjectStates = {
  'mech-obj-1': state1,
  'mech-obj-3': state2,
};
