/**
 * Mechanism IR: Dialectic Pseudo-Code for Absolute Mechanism
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - C. Absolute Mechanism
 *
 * Covers the dialectical movement:
 * - Center: Objective singularity, central body, striving, centrality
 * - Syllogistic Structure: Absolute center, relative centers, free mechanism
 * - Law: Idealized reality vs external reality, free necessity
 * - Transition to Chemism: Objectified opposition, reciprocally negative
 *
 * The GPU's mechanical object - self-subsisting, law-governed
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'mech-abs-1',
  title: 'Center — objective singularity, striving towards center',
  concept: 'MechanicalCenter',
  phase: 'object',

  moments: [
    {
      name: 'objectiveSingularity',
      definition: 'Empty manifoldness gathered into objective singularity, middle point',
      type: 'determination',
    },
    {
      name: 'objectiveUniversality',
      definition: 'Central body is genus, objective universality pervading objects',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'objectiveSingularity',
    },
    {
      name: 'strivingTowardsCenter',
      definition: 'Striving towards center is absolute universality, friction brings body back',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'mech-abs-1-inv-1',
      constraint: 'center = objectiveSingularity',
      predicate: 'equals(center, objectiveSingularity)',
    },
    {
      id: 'mech-abs-1-inv-2',
      constraint: 'centralBody = genus',
      predicate: 'equals(centralBody, genus)',
    },
  ],

  forces: [
    {
      id: 'mech-abs-1-force-1',
      description: 'Center drives toward syllogistic structure',
      type: 'mediation',
      trigger: 'center.objective = true',
      effect: 'syllogisticStructure.emerges = true',
      targetState: 'mech-abs-2',
    },
  ],

  transitions: [
    {
      id: 'mech-abs-1-trans-1',
      from: 'mech-abs-1',
      to: 'mech-abs-2',
      mechanism: 'mediation',
      description: 'From center to syllogistic structure',
    },
  ],

  nextStates: ['mech-abs-2'],
  previousStates: ['necessity-syllogism-ir'],

  provenance: {
    topicMapId: 'mech-abs-1-center-singularity',
    lineRange: { start: 4, end: 64 },
    section: 'Absolute Mechanism - The Center',
    order: 1,
  },

  description: 'Center as objective singularity. Central body is genus, objective universality. Striving towards center is absolute universality. Friction is phenomenon of centrality.',
};

const state2: DialecticState = {
  id: 'mech-abs-2',
  title: 'Syllogistic structure — absolute center, relative centers, free mechanism',
  concept: 'MechanicalSyllogism',
  phase: 'object',

  moments: [
    {
      name: 'absoluteCenter',
      definition: 'Absolute individual is objectively universal middle term',
      type: 'determination',
    },
    {
      name: 'relativeCenters',
      definition: 'Relative individual centers constitute middle term of second syllogism',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'absoluteCenter',
    },
    {
      name: 'freeMechanism',
      definition: 'Free mechanism - objective universality as fundamental determination, law',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'mech-abs-2-inv-1',
      constraint: 'absoluteCenter = objectivelyUniversalMiddle',
      predicate: 'equals(absoluteCenter, objectivelyUniversalMiddle)',
    },
    {
      id: 'mech-abs-2-inv-2',
      constraint: 'mechanism.free = true',
      predicate: 'isFree(mechanism)',
    },
  ],

  forces: [
    {
      id: 'mech-abs-2-force-1',
      description: 'Free mechanism drives toward law',
      type: 'mediation',
      trigger: 'mechanism.free = true',
      effect: 'law.emerges = true',
      targetState: 'mech-abs-3',
    },
  ],

  transitions: [
    {
      id: 'mech-abs-2-trans-1',
      from: 'mech-abs-2',
      to: 'mech-abs-3',
      mechanism: 'mediation',
      description: 'From syllogistic structure to law',
    },
  ],

  nextStates: ['mech-abs-3'],
  previousStates: ['mech-abs-1'],

  provenance: {
    topicMapId: 'mech-abs-2-center-syllogistic',
    lineRange: { start: 66, end: 165 },
    section: 'Absolute Mechanism - The Center',
    order: 2,
  },

  description: 'Syllogistic structure. Absolute center, relative centers, formal objects. Three syllogisms. Free mechanism - objective universality as fundamental determination. Order passed over into law.',
};

const state3: DialecticState = {
  id: 'mech-abs-3',
  title: 'Law — idealized reality vs external reality, free necessity',
  concept: 'MechanicalLaw',
  phase: 'object',

  moments: [
    {
      name: 'idealizedReality',
      definition: 'Idealized reality - real ideality is soul of objective totality',
      type: 'determination',
    },
    {
      name: 'externalReality',
      definition: 'External reality does not correspond to concept, mere striving',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'idealizedReality',
    },
    {
      name: 'freeNecessity',
      definition: 'Law is free necessity, self-igniting fire, refers only to itself',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'mech-abs-3-inv-1',
      constraint: 'idealizedReality ≠ externalReality',
      predicate: 'not(equals(idealizedReality, externalReality))',
    },
    {
      id: 'mech-abs-3-inv-2',
      constraint: 'law = freeNecessity',
      predicate: 'equals(law, freeNecessity)',
    },
  ],

  forces: [
    {
      id: 'mech-abs-3-force-1',
      description: 'Law drives toward chemism',
      type: 'negation',
      trigger: 'difference.objectified = true',
      effect: 'chemism.emerges = true',
      targetState: 'chem-1',
    },
  ],

  transitions: [
    {
      id: 'mech-abs-3-trans-1',
      from: 'mech-abs-3',
      to: 'chem-1',
      mechanism: 'negation',
      description: 'From mechanism to chemism',
    },
  ],

  nextStates: ['chem-1'],
  previousStates: ['mech-abs-2'],

  provenance: {
    topicMapId: 'mech-abs-3-law',
    lineRange: { start: 167, end: 278 },
    section: 'Absolute Mechanism - The Law',
    order: 3,
  },

  description: 'Law - idealized reality vs external reality. Idealized reality is soul of objective totality. Free necessity, self-igniting fire. Centrality falls apart, negativity passed over into objectified opposition. Mechanism determines itself to chemism.',
};

export const mechanismIR: DialecticIR = {
  id: 'mechanism-ir',
  title: 'Mechanism IR: Center, Syllogistic Structure, Law, Chemism',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - C. Absolute Mechanism',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'mechanism.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'mech-abs-1': 'object',
      'mech-abs-2': 'object',
      'mech-abs-3': 'object',
    },
  },
};

export const mechanismStates = {
  'mech-abs-1': state1,
  'mech-abs-2': state2,
  'mech-abs-3': state3,
};
