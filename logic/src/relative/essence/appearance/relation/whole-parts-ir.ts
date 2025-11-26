/**
 * Whole-Parts IR: Dialectic Pseudo-Code for Whole and Parts
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 3. Essential Relation - a. Whole and Parts
 *
 * Covers: Essential relation, reciprocal conditioning, tautology, transition to force
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'wlp-1',
  title: 'Essential relation — two sides',
  concept: 'EssentialRelation',
  phase: 'appearance',

  moments: [
    {
      name: 'essentialRelation',
      definition: 'Whole is world-in-itself, parts is world of appearance',
      type: 'determination',
    },
    {
      name: 'reflectiveShining',
      definition: 'Each has other reflectively shining in it',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'essentialRelation',
    },
  ],

  invariants: [
    {
      id: 'wlp-1-inv-1',
      constraint: 'whole = worldInItself',
      predicate: 'equals(whole, worldInItself)',
    },
  ],

  forces: [
    {
      id: 'wlp-1-force-1',
      description: 'Essential relation drives toward reciprocal conditioning',
      type: 'mediation',
      trigger: 'essentialRelation.established = true',
      effect: 'reciprocalConditioning.emerges = true',
      targetState: 'wlp-5',
    },
  ],

  transitions: [
    {
      id: 'wlp-1-trans-1',
      from: 'wlp-1',
      to: 'wlp-5',
      mechanism: 'mediation',
      description: 'From essential relation to reciprocal conditioning',
    },
  ],

  nextStates: ['wlp-5'],
  previousStates: ['disappearance-ir'],

  provenance: {
    topicMapId: 'wlp-1',
    lineRange: { start: 4, end: 50 },
    section: 'a. Whole and Parts',
    order: 1,
  },

  description: 'Essential relation contains self-subsistence reflected into itself. Whole is world-in-itself, parts is world of appearance.',
};

const state2: DialecticState = {
  id: 'wlp-5',
  title: 'Reciprocal conditioning — unconditioned',
  concept: 'ReciprocalConditioning',
  phase: 'appearance',

  moments: [
    {
      name: 'reciprocalConditioning',
      definition: 'Whole and parts reciprocally condition each other',
      type: 'mediation',
    },
    {
      name: 'unconditioned',
      definition: 'Turning back of conditioning into itself, unconditioned',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'reciprocalConditioning',
    },
  ],

  invariants: [
    {
      id: 'wlp-5-inv-1',
      constraint: 'whole.conditions = parts ∧ parts.conditions = whole',
      predicate: 'and(conditions(whole, parts), conditions(parts, whole))',
    },
  ],

  forces: [
    {
      id: 'wlp-5-force-1',
      description: 'Reciprocal conditioning drives toward tautology',
      type: 'mediation',
      trigger: 'reciprocalConditioning.established = true',
      effect: 'tautology.emerges = true',
      targetState: 'wlp-8',
    },
  ],

  transitions: [
    {
      id: 'wlp-5-trans-1',
      from: 'wlp-5',
      to: 'wlp-8',
      mechanism: 'mediation',
      description: 'From reciprocal conditioning to tautology',
    },
  ],

  nextStates: ['wlp-8'],
  previousStates: ['wlp-1'],

  provenance: {
    topicMapId: 'wlp-5',
    lineRange: { start: 108, end: 131 },
    section: 'a. Whole and Parts',
    order: 2,
  },

  description: 'Whole and parts reciprocally condition each other. Turning back of conditioning into itself, unconditioned.',
};

const state3: DialecticState = {
  id: 'wlp-8',
  title: 'Whole equal to parts — tautology',
  concept: 'Tautology',
  phase: 'appearance',

  moments: [
    {
      name: 'tautology',
      definition: 'Whole equal to whole, parts equal to parts',
      type: 'determination',
    },
    {
      name: 'fallApart',
      definition: 'Whole and parts fall apart, destroy themselves',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'tautology',
    },
  ],

  invariants: [
    {
      id: 'wlp-8-inv-1',
      constraint: 'whole = whole',
      predicate: 'equals(whole, whole)',
    },
  ],

  forces: [
    {
      id: 'wlp-8-force-1',
      description: 'Tautology drives toward force and expression',
      type: 'passover',
      trigger: 'tautology.established = true',
      effect: 'forceExpression.emerges = true',
      targetState: 'fex-1',
    },
  ],

  transitions: [
    {
      id: 'wlp-8-trans-1',
      from: 'wlp-8',
      to: 'fex-1',
      mechanism: 'passover',
      description: 'From tautology to force and expression',
    },
  ],

  nextStates: ['fex-1'],
  previousStates: ['wlp-5'],

  provenance: {
    topicMapId: 'wlp-8',
    lineRange: { start: 155, end: 173 },
    section: 'a. Whole and Parts',
    order: 3,
  },

  description: 'Whole equal to whole, parts equal to parts. Tautology. Whole and parts fall apart, destroy themselves.',
};

export const wholePartsIR: DialecticIR = {
  id: 'whole-parts-ir',
  title: 'Whole-Parts IR: Essential Relation, Reciprocal Conditioning, Tautology',
  section: 'B. APPEARANCE - 3. Essential Relation - a. Whole and Parts',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'whole-parts.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'wlp-1': 'appearance',
      'wlp-5': 'appearance',
      'wlp-8': 'appearance',
    },
  },
};

export const wholePartsStates = {
  'wlp-1': state1,
  'wlp-5': state2,
  'wlp-8': state3,
};
