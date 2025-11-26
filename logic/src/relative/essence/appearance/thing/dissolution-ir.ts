/**
 * Dissolution IR: Dialectic Pseudo-Code for Dissolution
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 1. The Thing - c. Dissolution of the Thing
 *
 * Covers the dialectical movement:
 * - Thing as absolutely alterable (absolute porosity)
 * - Matters as negative reference (interpenetration)
 * - Concrete existence as appearance
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'dis-1',
  title: 'Thing absolutely alterable — dissolution',
  concept: 'AbsoluteAlterability',
  phase: 'appearance',

  moments: [
    {
      name: 'absoluteAlterability',
      definition: 'Thing as quantitative combination of free matters is absolutely alterable',
      type: 'determination',
    },
    {
      name: 'externalDissolution',
      definition: 'External dissolution of external bond, binding indifferent to being bound',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'absoluteAlterability',
    },
    {
      name: 'absolutePorosity',
      definition: 'Thing is absolute porosity without measure or form of its own',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'dis-1-inv-1',
      constraint: 'thing.alterable = absolute',
      predicate: 'equals(thing.alterable, absolute)',
    },
    {
      id: 'dis-1-inv-2',
      constraint: 'bond.external = true',
      predicate: 'isExternal(bond)',
    },
    {
      id: 'dis-1-inv-3',
      constraint: 'thing.porosity = absolute',
      predicate: 'equals(thing.porosity, absolute)',
    },
  ],

  forces: [
    {
      id: 'dis-1-force-1',
      description: 'Absolute porosity drives toward matters as negative reference',
      type: 'negation',
      trigger: 'porosity.absolute = true',
      effect: 'negativeReference.emerges = true',
      targetState: 'dis-3',
    },
  ],

  transitions: [
    {
      id: 'dis-1-trans-1',
      from: 'dis-1',
      to: 'dis-3',
      mechanism: 'negation',
      description: 'From absolute porosity to matters as negative reference',
    },
  ],

  nextStates: ['dis-3'],
  previousStates: ['matter-ir'],

  provenance: {
    topicMapId: 'dis-1',
    lineRange: { start: 4, end: 18 },
    section: 'c. Dissolution',
    order: 1,
  },

  description: 'Thing as merely quantitative combination of free matters is absolutely alterable. Coming-to-be and passing-away is external dissolution of external bond. Thing itself is absolute porosity without measure or form of its own.',
};

const state2: DialecticState = {
  id: 'dis-3',
  title: 'Matters — negative reference',
  concept: 'MattersAsNegativeReference',
  phase: 'appearance',

  moments: [
    {
      name: 'negativeReference',
      definition: 'Matters themselves negative reflection, puncticity of thing',
      type: 'negation',
    },
    {
      name: 'determinateness',
      definition: 'Content is determinateness, refers to other',
      type: 'determination',
      relation: 'mediates',
      relatedTo: 'negativeReference',
    },
    {
      name: 'exclusion',
      definition: 'One matter is not what other is, one is not to extent other is',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'dis-3-inv-1',
      constraint: 'matters.lackNegativeSelfSubsistence = true',
      predicate: 'lacks(matters, negativeSelfSubsistence)',
    },
    {
      id: 'dis-3-inv-2',
      constraint: 'content = determinateness',
      predicate: 'equals(content, determinateness)',
    },
    {
      id: 'dis-3-inv-3',
      constraint: 'oneMatter ≠ otherMatter',
      predicate: 'not(equals(oneMatter, otherMatter))',
    },
  ],

  forces: [
    {
      id: 'dis-3-force-1',
      description: 'Negative reference drives toward interpenetration',
      type: 'mediation',
      trigger: 'negativeReference.established = true',
      effect: 'interpenetration.emerges = true',
      targetState: 'dis-5',
    },
  ],

  transitions: [
    {
      id: 'dis-3-trans-1',
      from: 'dis-3',
      to: 'dis-5',
      mechanism: 'mediation',
      description: 'From negative reference to interpenetration',
    },
  ],

  nextStates: ['dis-5'],
  previousStates: ['dis-1'],

  provenance: {
    topicMapId: 'dis-3',
    lineRange: { start: 32, end: 54 },
    section: 'c. Dissolution',
    order: 2,
  },

  description: 'Matters lack negative self-subsistence. Content is determinateness, refers to other. Matters themselves negative reflection, puncticity of thing. One matter is not what other is.',
};

const state3: DialecticState = {
  id: 'dis-5',
  title: 'Puncticity — porosity',
  concept: 'Interpenetration',
  phase: 'appearance',

  moments: [
    {
      name: 'interpenetration',
      definition: 'Matters interpenetrate absolutely, yet do not touch',
      type: 'mediation',
    },
    {
      name: 'porosity',
      definition: 'Matters essentially porous, one subsists in pores of others',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'interpenetration',
    },
    {
      name: 'selfContradictoryMediation',
      definition: 'Self-contradictory mediation through subsisting and non-subsisting',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'dis-5-inv-1',
      constraint: 'matters.interpenetrate = true',
      predicate: 'interpenetrate(matters)',
    },
    {
      id: 'dis-5-inv-2',
      constraint: 'matters.porous = true',
      predicate: 'isPorous(matters)',
    },
    {
      id: 'dis-5-inv-3',
      constraint: 'subsistence = sublatedness',
      predicate: 'equals(subsistence, sublatedness)',
    },
  ],

  forces: [
    {
      id: 'dis-5-force-1',
      description: 'Self-contradictory mediation drives toward appearance',
      type: 'contradiction',
      trigger: 'mediation.selfContradictory = true',
      effect: 'appearance.emerges = true',
      targetState: 'dis-6',
    },
  ],

  transitions: [
    {
      id: 'dis-5-trans-1',
      from: 'dis-5',
      to: 'dis-6',
      mechanism: 'contradiction',
      description: 'From interpenetration to appearance',
    },
  ],

  nextStates: ['dis-6'],
  previousStates: ['dis-3'],

  provenance: {
    topicMapId: 'dis-5',
    lineRange: { start: 73, end: 101 },
    section: 'c. Dissolution',
    order: 3,
  },

  description: 'Matters interpenetrate absolutely, yet do not touch. Matters essentially porous. One subsists in pores of others. Thing is self-contradictory mediation of independent self-subsistence through subsisting and non-subsisting of other.',
};

const state4: DialecticState = {
  id: 'dis-6',
  title: 'Concrete existence — appearance',
  concept: 'ConcreteExistenceAsAppearance',
  phase: 'appearance',

  moments: [
    {
      name: 'completedConcreteExistence',
      definition: 'Concrete existence has attained completion in "this" thing',
      type: 'determination',
    },
    {
      name: 'inItselfInUnessentiality',
      definition: 'Has its in-itself in unessentiality, subsists in absolute other',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'completedConcreteExistence',
    },
    {
      name: 'appearance',
      definition: 'Has nothingness for substrate, it is therefore appearance',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'dis-6-inv-1',
      constraint: 'concreteExistence.completed = true',
      predicate: 'isCompleted(concreteExistence)',
    },
    {
      id: 'dis-6-inv-2',
      constraint: 'inItself = unessentiality',
      predicate: 'equals(inItself, unessentiality)',
    },
    {
      id: 'dis-6-inv-3',
      constraint: 'substrate = nothingness',
      predicate: 'equals(substrate, nothingness)',
    },
  ],

  forces: [
    {
      id: 'dis-6-force-1',
      description: 'Appearance drives toward World',
      type: 'passover',
      trigger: 'appearance.established = true',
      effect: 'world.emerges = true',
      targetState: 'wld-1',
    },
  ],

  transitions: [
    {
      id: 'dis-6-trans-1',
      from: 'dis-6',
      to: 'wld-1',
      mechanism: 'passover',
      description: 'From appearance to world',
    },
  ],

  nextStates: ['wld-1'],
  previousStates: ['dis-5'],

  provenance: {
    topicMapId: 'dis-6',
    lineRange: { start: 103, end: 111 },
    section: 'c. Dissolution',
    order: 4,
  },

  description: 'In "this" thing, concrete existence has attained completion. Truth of concrete existence: has its in-itself in unessentiality. Subsists in absolute other, has nothingness for substrate. It is, therefore, appearance.',
};

export const dissolutionIR: DialecticIR = {
  id: 'dissolution-ir',
  title: 'Dissolution IR: Absolute Porosity, Interpenetration, Appearance',
  section: 'B. APPEARANCE - 1. The Thing - c. Dissolution',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'dissolution.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'dis-1': 'appearance',
      'dis-3': 'appearance',
      'dis-5': 'appearance',
      'dis-6': 'appearance',
    },
  },
};

export const dissolutionStates = {
  'dis-1': state1,
  'dis-3': state2,
  'dis-5': state3,
  'dis-6': state4,
};
