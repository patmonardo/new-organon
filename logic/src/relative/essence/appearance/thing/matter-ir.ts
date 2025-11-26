/**
 * Matter IR: Dialectic Pseudo-Code for Matter
 *
 * Architecture: GPU (Appearance)
 * Section: B. APPEARANCE - 1. The Thing - b. The Constitution of the Thing Out of Matters
 *
 * Covers the dialectical movement:
 * - Property transitions to matter (chemistry)
 * - Matter as self-subsistent (porous matter)
 * - "This" as quantitative connection
 * - Matter dissolves into properties
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'mat-1',
  title: 'Transition of property into matter — chemistry',
  concept: 'PropertyToMatter',
  phase: 'appearance',

  moments: [
    {
      name: 'propertyAsMatter',
      definition: 'Properties represented as matters (luminous, coloring, odorific matter)',
      type: 'determination',
    },
    {
      name: 'chemicalTransition',
      definition: 'Chemistry seeks to represent properties as they truly are',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'propertyAsMatter',
    },
    {
      name: 'componentsAsConcretes',
      definition: 'Components are at least concretes in general',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'mat-1-inv-1',
      constraint: 'property.transitions = matter',
      predicate: 'transitions(property, matter)',
    },
    {
      id: 'mat-1-inv-2',
      constraint: 'components.concrete = true',
      predicate: 'isConcrete(components)',
    },
    {
      id: 'mat-1-inv-3',
      constraint: 'distinction(things, components) = unclear',
      predicate: 'unclear(distinction(things, components))',
    },
  ],

  forces: [
    {
      id: 'mat-1-force-1',
      description: 'Property as essential drives toward matter as self-subsistent',
      type: 'mediation',
      trigger: 'property.essential = true',
      effect: 'matter.selfSubsistent = true',
      targetState: 'mat-3',
    },
  ],

  transitions: [
    {
      id: 'mat-1-trans-1',
      from: 'mat-1',
      to: 'mat-3',
      mechanism: 'mediation',
      description: 'From property to matter as self-subsistent',
    },
  ],

  nextStates: ['mat-3'],
  previousStates: ['thing-ir'],

  provenance: {
    topicMapId: 'mat-1',
    lineRange: { start: 4, end: 25 },
    section: 'b. Matter',
    order: 1,
  },

  description: 'Transition of property into matter is familiar transition performed by chemistry. Properties represented as luminous matter, coloring matter, odorific matter. Components are at least concretes in general.',
};

const state2: DialecticState = {
  id: 'mat-3',
  title: 'Negative moment preserved — matter continuous with itself',
  concept: 'MatterAsSelfSubsistent',
  phase: 'appearance',

  moments: [
    {
      name: 'continuousMatter',
      definition: 'Property becomes matter continuous with itself and self-subsisting',
      type: 'determination',
    },
    {
      name: 'negativeUnity',
      definition: 'Continuity contains moment of negative, negative unity',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'continuousMatter',
    },
    {
      name: 'restoredThinghood',
      definition: 'Self-subsistence is restored something of thinghood',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'mat-3-inv-1',
      constraint: 'matter.continuous = true',
      predicate: 'isContinuous(matter)',
    },
    {
      id: 'mat-3-inv-2',
      constraint: 'continuity.contains = negativeUnity',
      predicate: 'contains(continuity, negativeUnity)',
    },
    {
      id: 'mat-3-inv-3',
      constraint: 'negativeSelfSubsistence ≠ positiveSelfSubsistence',
      predicate: 'not(equals(negativeSelfSubsistence, positiveSelfSubsistence))',
    },
  ],

  forces: [
    {
      id: 'mat-3-force-1',
      description: 'Matter as continuous drives toward "this" as determinate',
      type: 'mediation',
      trigger: 'matter.selfSubsistent = true',
      effect: 'thisThing.emerges = true',
      targetState: 'mat-7',
    },
  ],

  transitions: [
    {
      id: 'mat-3-trans-1',
      from: 'mat-3',
      to: 'mat-7',
      mechanism: 'mediation',
      description: 'From matter to "this" as quantitative connection',
    },
  ],

  nextStates: ['mat-7'],
  previousStates: ['mat-1'],

  provenance: {
    topicMapId: 'mat-3',
    lineRange: { start: 45, end: 55 },
    section: 'b. Matter',
    order: 2,
  },

  description: 'Negative moment preserved. Property becomes matter continuous with itself and self-subsisting. Continuity contains moment of negative. Self-subsistence is restored something of thinghood. Negative self-subsistence versus positive self-subsistence.',
};

const state3: DialecticState = {
  id: 'mat-7',
  title: '"This" — complete determinateness',
  concept: 'ThisAsQuantitative',
  phase: 'appearance',

  moments: [
    {
      name: 'thisThing',
      definition: '"This" constitutes complete determinateness, external determinateness',
      type: 'determination',
    },
    {
      name: 'quantitativeConnection',
      definition: 'Thing is merely quantitative connection, mere collection, "also"',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'thisThing',
    },
    {
      name: 'porousMatter',
      definition: 'Matters overrun thing, self-subsistent, impenetrable to each other',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'mat-7-inv-1',
      constraint: 'this = completeDeterminateness',
      predicate: 'equals(this, completeDeterminateness)',
    },
    {
      id: 'mat-7-inv-2',
      constraint: 'thing = quantitativeConnection',
      predicate: 'equals(thing, quantitativeConnection)',
    },
    {
      id: 'mat-7-inv-3',
      constraint: 'matters.overrun = true',
      predicate: 'overrun(matters)',
    },
  ],

  forces: [
    {
      id: 'mat-7-force-1',
      description: 'Porous matter drives toward dissolution',
      type: 'negation',
      trigger: 'matters.indifferent = true',
      effect: 'dissolution.emerges = true',
      targetState: 'dis-1',
    },
  ],

  transitions: [
    {
      id: 'mat-7-trans-1',
      from: 'mat-7',
      to: 'dis-1',
      mechanism: 'negation',
      description: 'From porous matter to dissolution',
    },
  ],

  nextStates: ['dis-1'],
  previousStates: ['mat-3'],

  provenance: {
    topicMapId: 'mat-7',
    lineRange: { start: 120, end: 152 },
    section: 'b. Matter',
    order: 3,
  },

  description: '"This" constitutes complete determinateness, external determinateness. Thing consists of self-subsistent matters indifferent to connection. Matters overrun thing, continue into others. Thing is merely quantitative connection, mere collection, "also". Combination of not having any combination constitutes thing.',
};

export const matterIR: DialecticIR = {
  id: 'matter-ir',
  title: 'Matter IR: Property to Matter, Porous Matter',
  section: 'B. APPEARANCE - 1. The Thing - b. Matter',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'matter.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'mat-1': 'appearance',
      'mat-3': 'appearance',
      'mat-7': 'appearance',
    },
  },
};

export const matterStates = {
  'mat-1': state1,
  'mat-3': state2,
  'mat-7': state3,
};
