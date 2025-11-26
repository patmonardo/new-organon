/**
 * Realized Purpose IR: Dialectic Pseudo-Code for The Realized Purpose
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - C. The Realized Purpose
 *
 * Covers the dialectical movement:
 * - Mechanism under Dominance: Cunning of reason, tool lasts, power of purpose
 * - External Purposiveness: Infinite progress, objects perishable, only means
 * - Inner Purposive Connection: Objective purpose, concept identical with immediate objectivity
 *
 * The GPU's realized purpose - concept becomes identical with objectivity, transitions to Idea
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'realized-2',
  title: 'Mechanism under dominance — cunning of reason, tool lasts',
  concept: 'MechanismUnderDominance',
  phase: 'object',

  moments: [
    {
      name: 'mechanismUnderDominance',
      definition: 'Mechanical/chemical processes under dominance of purpose, return into purpose',
      type: 'mediation',
    },
    {
      name: 'cunningOfReason',
      definition: 'Purpose inserts means between itself and object, cunning of reason',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'mechanismUnderDominance',
    },
    {
      name: 'toolLasts',
      definition: 'Tool lasts while enjoyments pass away, means higher than finite purposes',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'realized-2-inv-1',
      constraint: 'mechanism.underDominanceOfPurpose = true',
      predicate: 'isUnderDominance(mechanism, purpose)',
    },
    {
      id: 'realized-2-inv-2',
      constraint: 'tool > finitePurposes',
      predicate: 'greaterThan(tool, finitePurposes)',
    },
  ],

  forces: [
    {
      id: 'realized-2-force-1',
      description: 'Mechanism under dominance drives toward external purposiveness',
      type: 'negation',
      trigger: 'purpose.external = true',
      effect: 'externalPurposiveness.emerges = true',
      targetState: 'realized-10',
    },
  ],

  transitions: [
    {
      id: 'realized-2-trans-1',
      from: 'realized-2',
      to: 'realized-10',
      mechanism: 'negation',
      description: 'From mechanism under dominance to external purposiveness',
    },
  ],

  nextStates: ['realized-10'],
  previousStates: ['means-ir'],

  provenance: {
    topicMapId: 'realized-2',
    lineRange: { start: 27, end: 151 },
    section: 'C. THE REALIZED PURPOSE',
    order: 1,
  },

  description: 'Mechanism under dominance of purpose. Cunning of reason - purpose inserts means, preserves itself. Tool lasts while enjoyments pass. Purpose keeps itself in mechanical process, power of purpose.',
};

const state2: DialecticState = {
  id: 'realized-10',
  title: 'External purposiveness — infinite progress, objects perishable, only means',
  concept: 'ExternalPurposiveness',
  phase: 'object',

  moments: [
    {
      name: 'infiniteProgress',
      definition: 'Infinite progress of mediation, product is only means, not realized purpose',
      type: 'negation',
    },
    {
      name: 'objectsPerishable',
      definition: 'Objects fulfill purpose through being used up and worn out, perishable',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'infiniteProgress',
    },
    {
      name: 'externalPurposivenessOnlyMeans',
      definition: 'External purposiveness only goes so far as to be means, not objective purpose',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'realized-10-inv-1',
      constraint: 'product = means',
      predicate: 'equals(product, means)',
    },
    {
      id: 'realized-10-inv-2',
      constraint: 'externalPurposiveness ≠ objectivePurpose',
      predicate: 'not(equals(externalPurposiveness, objectivePurpose))',
    },
  ],

  forces: [
    {
      id: 'realized-10-force-1',
      description: 'External purposiveness drives toward inner purposive connection',
      type: 'sublation',
      trigger: 'reflectiveShine.sublated = true',
      effect: 'innerPurposiveConnection.emerges = true',
      targetState: 'realized-12',
    },
  ],

  transitions: [
    {
      id: 'realized-10-trans-1',
      from: 'realized-10',
      to: 'realized-12',
      mechanism: 'sublation',
      description: 'From external to inner purposive connection',
    },
  ],

  nextStates: ['realized-12'],
  previousStates: ['realized-2'],

  provenance: {
    topicMapId: 'realized-10',
    lineRange: { start: 320, end: 397 },
    section: 'C. THE REALIZED PURPOSE',
    order: 2,
  },

  description: 'External purposiveness - infinite progress, product only means. Objects perishable, fulfill purpose through being worn out. External purposiveness only means, purpose sunk into objectivity.',
};

const state3: DialecticState = {
  id: 'realized-12',
  title: 'Inner purposive connection — concept identical with immediate objectivity',
  concept: 'InnerPurposiveConnection',
  phase: 'object',

  moments: [
    {
      name: 'innerPurposiveConnection',
      definition: 'Truth is inner purposive connection and objective purpose',
      type: 'mediation',
    },
    {
      name: 'reflectiveShineSublated',
      definition: 'Self-subsistence of object is unessential reflective shine, already sublated',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'innerPurposiveConnection',
    },
    {
      name: 'conceptIdenticalWithObjectivity',
      definition: 'Concept as concrete totality identical with immediate objectivity',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'realized-12-inv-1',
      constraint: 'connection = innerPurposive',
      predicate: 'equals(connection, innerPurposive)',
    },
    {
      id: 'realized-12-inv-2',
      constraint: 'concept = immediateObjectivity',
      predicate: 'equals(concept, immediateObjectivity)',
    },
  ],

  forces: [
    {
      id: 'realized-12-force-1',
      description: 'Inner purposive connection transitions to Idea',
      type: 'passover',
      trigger: 'concept.identicalWithObjectivity = true',
      effect: 'idea.emerges = true',
      targetState: 'idea-1',
    },
  ],

  transitions: [
    {
      id: 'realized-12-trans-1',
      from: 'realized-12',
      to: 'idea-1',
      mechanism: 'passover',
      description: 'From realized purpose to Idea',
    },
  ],

  nextStates: ['idea-1'],
  previousStates: ['realized-10'],

  provenance: {
    topicMapId: 'realized-15',
    lineRange: { start: 578, end: 606 },
    section: 'C. THE REALIZED PURPOSE',
    order: 3,
  },

  description: 'Inner purposive connection, objective purpose. Reflective shine sublated. Concept as concrete totality identical with immediate objectivity. Concept is self-determining identity in external totality. Transitions to Idea.',
};

export const realizedPurposeIR: DialecticIR = {
  id: 'realized-purpose-ir',
  title: 'Realized Purpose IR: Mechanism Under Dominance, External Purposiveness, Inner Connection',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - C. The Realized Purpose',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'realized.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'realized-2': 'object',
      'realized-10': 'object',
      'realized-12': 'object',
    },
  },
};

export const realizedPurposeStates = {
  'realized-2': state1,
  'realized-10': state2,
  'realized-12': state3,
};
