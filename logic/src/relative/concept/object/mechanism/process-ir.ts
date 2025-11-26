/**
 * Mechanical Process IR: Dialectic Pseudo-Code for The Mechanical Process
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - B. The Mechanical Process
 *
 * Covers the dialectical movement:
 * - Formal Process: Communication, reaction, product (rest)
 * - Real Process: Determined opposition, resistance, power, fate
 * - Product: Center and law, rational fate, foundation
 *
 * The GPU's mechanical process - communication, resistance, center, law
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'mech-proc-1',
  title: 'Formal process — communication, reaction, product',
  concept: 'FormalMechanicalProcess',
  phase: 'object',

  moments: [
    {
      name: 'communication',
      definition: 'Communication - determinateness widens in idealizing manner, universality',
      type: 'process',
    },
    {
      name: 'reaction',
      definition: 'Reaction - particularization, reciprocal repulsion, elasticity',
      type: 'process',
      relation: 'opposite',
      relatedTo: 'communication',
    },
    {
      name: 'productAsRest',
      definition: 'Product - return to rest, mediation sublated, external determinateness',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'mech-proc-1-inv-1',
      constraint: 'communication = universality',
      predicate: 'equals(communication, universality)',
    },
    {
      id: 'mech-proc-1-inv-2',
      constraint: 'reaction = action',
      predicate: 'equals(reaction, action)',
    },
  ],

  forces: [
    {
      id: 'mech-proc-1-force-1',
      description: 'Formal process drives toward real process',
      type: 'mediation',
      trigger: 'determinateness.reflected = true',
      effect: 'realProcess.emerges = true',
      targetState: 'mech-proc-6',
    },
  ],

  transitions: [
    {
      id: 'mech-proc-1-trans-1',
      from: 'mech-proc-1',
      to: 'mech-proc-6',
      mechanism: 'mediation',
      description: 'From formal to real mechanical process',
    },
  ],

  nextStates: ['mech-proc-6'],
  previousStates: ['mechanical-object-ir'],

  provenance: {
    topicMapId: 'mech-proc-3-formal-communication',
    lineRange: { start: 96, end: 250 },
    section: 'The Formal Mechanical Process',
    order: 1,
  },

  description: 'Formal mechanical process. Communication - idealized connection, universality. Reaction - particularization, reciprocal repulsion. Product - rest, external determinateness posited.',
};

const state2: DialecticState = {
  id: 'mech-proc-6',
  title: 'Real process — resistance, power, fate',
  concept: 'RealMechanicalProcess',
  phase: 'object',

  moments: [
    {
      name: 'determinedOpposition',
      definition: 'Determined opposition: self-subsistent singularity vs non-self-subsistent universality',
      type: 'determination',
    },
    {
      name: 'resistanceAndPower',
      definition: 'Resistance overpowered, violence, power as objective universality',
      type: 'process',
      relation: 'contains',
      relatedTo: 'determinedOpposition',
    },
    {
      name: 'fate',
      definition: 'Power as fate - blind mechanism, genus, self-consciousness and deed',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'mech-proc-6-inv-1',
      constraint: 'opposition.determined = true',
      predicate: 'isDetermined(opposition)',
    },
    {
      id: 'mech-proc-6-inv-2',
      constraint: 'power = fate',
      predicate: 'equals(power, fate)',
    },
  ],

  forces: [
    {
      id: 'mech-proc-6-force-1',
      description: 'Real process drives toward center and law',
      type: 'mediation',
      trigger: 'negativity.reflected = true',
      effect: 'centerAndLaw.emerges = true',
      targetState: 'mech-proc-8',
    },
  ],

  transitions: [
    {
      id: 'mech-proc-6-trans-1',
      from: 'mech-proc-6',
      to: 'mech-proc-8',
      mechanism: 'mediation',
      description: 'From real process to center and law',
    },
  ],

  nextStates: ['mech-proc-8'],
  previousStates: ['mech-proc-1'],

  provenance: {
    topicMapId: 'mech-proc-6-real-communication',
    lineRange: { start: 252, end: 409 },
    section: 'The Real Mechanical Process',
    order: 2,
  },

  description: 'Real mechanical process. Determined opposition - self-subsistent vs non-self-subsistent. Resistance and power. Fate - blind mechanism, genus, self-consciousness.',
};

const state3: DialecticState = {
  id: 'mech-proc-8',
  title: 'Product — center and law, rational fate',
  concept: 'ProductCenterLaw',
  phase: 'object',

  moments: [
    {
      name: 'center',
      definition: 'Center - objective oneness, individual self-subsistence',
      type: 'determination',
    },
    {
      name: 'law',
      definition: 'Law - rational fate, immanently determined, universality particularizing from within',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'center',
    },
    {
      name: 'truthAndFoundation',
      definition: 'Result is truth and foundation of mechanical process',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'mech-proc-8-inv-1',
      constraint: 'center = objectiveOneness',
      predicate: 'equals(center, objectiveOneness)',
    },
    {
      id: 'mech-proc-8-inv-2',
      constraint: 'law = rationalFate',
      predicate: 'equals(law, rationalFate)',
    },
  ],

  forces: [
    {
      id: 'mech-proc-8-force-1',
      description: 'Center and law transition to absolute mechanism',
      type: 'mediation',
      trigger: 'law.established = true',
      effect: 'absoluteMechanism.emerges = true',
      targetState: 'mech-abs-1',
    },
  ],

  transitions: [
    {
      id: 'mech-proc-8-trans-1',
      from: 'mech-proc-8',
      to: 'mech-abs-1',
      mechanism: 'mediation',
      description: 'From product to absolute mechanism',
    },
  ],

  nextStates: ['mech-abs-1'],
  previousStates: ['mech-proc-6'],

  provenance: {
    topicMapId: 'mech-proc-8-product-center-law',
    lineRange: { start: 411, end: 456 },
    section: 'The Product of Mechanical Process',
    order: 3,
  },

  description: 'Product of mechanical process. Center - objective oneness, individual self-subsistence. Law - rational fate, immanently determined. Truth and foundation of mechanical process.',
};

export const mechanicalProcessIR: DialecticIR = {
  id: 'mechanical-process-ir',
  title: 'Mechanical Process IR: Formal, Real, Product (Center/Law)',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - B. The Mechanical Process',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'process.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'mech-proc-1': 'object',
      'mech-proc-6': 'object',
      'mech-proc-8': 'object',
    },
  },
};

export const mechanicalProcessStates = {
  'mech-proc-1': state1,
  'mech-proc-6': state2,
  'mech-proc-8': state3,
};
