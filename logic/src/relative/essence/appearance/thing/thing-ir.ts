/**
 * Thing IR: Dialectic Pseudo-Code for The Thing and Its Properties
 *
 * Architecture: GPU (Appearance / Mathematical Coprocessor)
 * Section: ESSENCE - B. APPEARANCE - A. THE THING
 *
 * Covers the dialectical movement:
 * - Concrete existence as principle (essence come forth into immediacy)
 * - Thing-in-itself as essential immediate (sublated mediation)
 * - External reflection and its collapse
 * - Property as negativity of reflection
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';
import { thingTopicMap } from './sources/thing-topic-map';

const state1: DialecticState = {
  id: 'thg-1',
  title: 'Concrete existence — principle',
  concept: 'ConcreteExistence',
  phase: 'appearance', // GPU: Appearance / Mathematical Coprocessor

  moments: [
    {
      name: 'concreteExistence',
      definition: 'Whatever is, exists concretely',
      type: 'determination',
    },
    {
      name: 'essence',
      definition: 'Essence that has come forth into immediacy',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'being',
    },
    {
      name: 'immediateSomething',
      definition: 'Not immediate something, but essence come forth',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'thg-1-inv-1',
      constraint: 'whateverIs.existsConcretely = true',
      predicate: 'existsConcretely(whateverIs)',
    },
    {
      id: 'thg-1-inv-2',
      constraint: 'truthOfBeing = essenceComeForthIntoImmediacy',
      predicate: 'truthOf(being, essenceComeForth(immediacy))',
    },
    {
      id: 'thg-1-inv-3',
      constraint: 'principle: whatever is exists concretely',
      predicate: 'principle(existsConcretely(whatever))',
    },
  ],

  forces: [
    {
      id: 'thg-1-force-1',
      description: 'Concrete existence as both grounded and unconditioned drives toward analysis',
      type: 'contradiction',
      trigger: 'concreteExistence.groundedAndUnconditioned = true',
      effect: 'analysis.required = true',
      targetState: 'thg-2',
    },
  ],

  transitions: [
    {
      id: 'thg-1-trans-1',
      from: 'thg-1',
      to: 'thg-2',
      mechanism: 'reflection',
      description: 'From principle to ground and unconditioned',
    },
  ],

  nextStates: ['thg-2'],
  previousStates: [],

  provenance: {
    topicMapId: 'thg-1',
    lineRange: { start: 6, end: 16 },
    section: 'A. THE THING AND ITS PROPERTIES',
    order: 1,
  },

  description: thingTopicMap[0].description,
  keyPoints: thingTopicMap[0].keyPoints,
};

const state2: DialecticState = {
  id: 'thg-7',
  title: 'Concrete existence — thing',
  concept: 'ThingDetermination',
  phase: 'appearance',

  moments: [
    {
      name: 'selfIdenticalMediation',
      definition: 'Concrete existence as self-identical mediation',
      type: 'mediation',
    },
    {
      name: 'negativeUnity',
      definition: 'Negative unity and being-within-itself',
      type: 'determination',
    },
    {
      name: 'thing',
      definition: 'Concretely existent something determined as thing',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'concreteExistence',
    },
  ],

  invariants: [
    {
      id: 'thg-7-inv-1',
      constraint: 'concreteExistence.hasDeterminationsOfMediation = true',
      predicate: 'hasDeterminationsOfMediation(concreteExistence)',
    },
    {
      id: 'thg-7-inv-2',
      constraint: 'determinations.reflectedIntoThemselves = true',
      predicate: 'reflectedIntoThemselves(determinations)',
    },
    {
      id: 'thg-7-inv-3',
      constraint: 'concreteExistence = negativeUnity ∧ beingWithinItself',
      predicate: 'is(concreteExistence, negativeUnity) ∧ is(concreteExistence, beingWithinItself)',
    },
    {
      id: 'thg-7-inv-4',
      constraint: 'concreteExistent.determinedAs = thing',
      predicate: 'determinedAs(concreteExistent, thing)',
    },
  ],

  forces: [
    {
      id: 'thg-7-force-1',
      description: 'Thing posited in form of negative unity drives toward distinction of in-itself and external',
      type: 'reflection',
      trigger: 'negativeUnity.immediate = true',
      effect: 'distinction.emerges = {thingInItself, externalExistence}',
      targetState: 'thg-9',
    },
  ],

  transitions: [
    {
      id: 'thg-7-trans-1',
      from: 'thg-7',
      to: 'thg-9',
      mechanism: 'reflection',
      description: 'From thing to thing-in-itself and external concrete existence',
    },
  ],

  nextStates: ['thg-9'],
  previousStates: ['thg-2'],

  provenance: {
    topicMapId: 'thg-7',
    lineRange: { start: 200, end: 209 },
    section: 'A. THE THING AND ITS PROPERTIES',
    order: 7,
  },

  description: thingTopicMap[6].description,
  keyPoints: thingTopicMap[6].keyPoints,
};

const state3: DialecticState = {
  id: 'thg-9',
  title: 'Thing in itself — essential and unessential',
  concept: 'ThingInItself',
  phase: 'appearance',

  moments: [
    {
      name: 'thingInItself',
      definition: 'Essential immediate from sublated mediation',
      type: 'determination',
    },
    {
      name: 'mediatedBeing',
      definition: 'Unessential concrete existence, positedness',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'thingInItself',
    },
    {
      name: 'substrate',
      definition: 'Unmoved, indeterminate unity',
      type: 'quality',
      relation: 'contains',
      relatedTo: 'thingInItself',
    },
  ],

  invariants: [
    {
      id: 'thg-9-inv-1',
      constraint: 'thingInItself = essentialImmediate(sublatedMediation)',
      predicate: 'is(thingInItself, essentialImmediate(sublatedMediation))',
    },
    {
      id: 'thg-9-inv-2',
      constraint: 'mediatedBeing = unessentialConcreteExistence',
      predicate: 'is(mediatedBeing, unessentialConcreteExistence)',
    },
    {
      id: 'thg-9-inv-3',
      constraint: 'thingInItself.essential ∧ mediatedBeing.unessential = true',
      predicate: 'essential(thingInItself) ∧ unessential(mediatedBeing)',
    },
    {
      id: 'thg-9-inv-4',
      constraint: 'substrate = unmovedIndeterminate',
      predicate: 'is(substrate, unmovedIndeterminate)',
    },
  ],

  forces: [
    {
      id: 'thg-9-force-1',
      description: 'Substrate exposed to external reflection drives toward manifold',
      type: 'externality',
      trigger: 'externalReflection.exposes = substrate',
      effect: 'manifold.emerges = true',
      targetState: 'thg-11',
    },
  ],

  transitions: [
    {
      id: 'thg-9-trans-1',
      from: 'thg-9',
      to: 'thg-11',
      mechanism: 'reflection',
      description: 'From thing-in-itself to external reflection',
    },
  ],

  nextStates: ['thg-11'],
  previousStates: ['thg-7'],

  provenance: {
    topicMapId: 'thg-9',
    lineRange: { start: 240, end: 275 },
    section: 'a. The thing in itself',
    order: 9,
  },

  description: thingTopicMap[8].description,
  keyPoints: thingTopicMap[8].keyPoints,
};

const state4: DialecticState = {
  id: 'thg-12',
  title: 'External reflection — collapses',
  concept: 'ReflectionCollapse',
  phase: 'appearance',

  moments: [
    {
      name: 'essencelessReflection',
      definition: 'Reflection external to thing-in-itself',
      type: 'negation',
    },
    {
      name: 'collapse',
      definition: 'Founders to ground, becomes essential identity',
      type: 'sublation',
      relation: 'transforms',
      relatedTo: 'essencelessReflection',
    },
    {
      name: 'identity',
      definition: 'Thing-in-itself identical with external concrete existence',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'thg-12-inv-1',
      constraint: 'essencelessReflection.collapses = true',
      predicate: 'collapses(essencelessReflection)',
    },
    {
      id: 'thg-12-inv-2',
      constraint: 'collapse.foundersToGround = true',
      predicate: 'foundersToGround(collapse)',
    },
    {
      id: 'thg-12-inv-3',
      constraint: 'collapse.becomes = essentialIdentity',
      predicate: 'becomes(collapse, essentialIdentity)',
    },
    {
      id: 'thg-12-inv-4',
      constraint: 'thingInItself = externalConcreteExistence',
      predicate: 'equals(thingInItself, externalConcreteExistence)',
    },
  ],

  forces: [
    {
      id: 'thg-12-force-1',
      description: 'Identity of thing-in-itself and external existence drives toward property',
      type: 'mediation',
      trigger: 'identity.established = true',
      effect: 'property.emerges = true',
      targetState: 'thg-15',
    },
  ],

  transitions: [
    {
      id: 'thg-12-trans-1',
      from: 'thg-12',
      to: 'thg-15',
      mechanism: 'mediation',
      description: 'From collapse of reflection to property as determinateness',
    },
  ],

  nextStates: ['thg-15'],
  previousStates: ['thg-11'],

  provenance: {
    topicMapId: 'thg-12',
    lineRange: { start: 329, end: 344 },
    section: 'a. The thing in itself',
    order: 12,
  },

  description: thingTopicMap[11].description,
  keyPoints: thingTopicMap[11].keyPoints,
};

const state5: DialecticState = {
  id: 'thg-15',
  title: 'Property — negativity of reflection',
  concept: 'Property',
  phase: 'appearance',

  moments: [
    {
      name: 'property',
      definition: 'Negativity of reflection, by virtue of which concrete existence is thing-in-itself',
      type: 'negation',
    },
    {
      name: 'referenceToItselfAsOther',
      definition: 'Mediation that is immediately self-identity',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'property',
    },
    {
      name: 'constitution',
      definition: 'Determinateness that is itself determination',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'thg-15-inv-1',
      constraint: 'property = negativityOfReflection',
      predicate: 'is(property, negativityOfReflection)',
    },
    {
      id: 'thg-15-inv-2',
      constraint: 'property.referenceToItself = asToOther',
      predicate: 'referenceToItself(property, asToOther)',
    },
    {
      id: 'thg-15-inv-3',
      constraint: 'property.mediation = selfIdentity',
      predicate: 'equals(property.mediation, selfIdentity)',
    },
    {
      id: 'thg-15-inv-4',
      constraint: 'property.excludedFrom = alteration',
      predicate: 'excludedFrom(property, alteration)',
    },
  ],

  forces: [
    {
      id: 'thg-15-force-1',
      description: 'Property as self-subsistent prepares reciprocal action',
      type: 'passover',
      trigger: 'property.selfSubsistent = true',
      effect: 'reciprocalAction.emerges = true',
      targetState: 'thg-18',
    },
  ],

  transitions: [
    {
      id: 'thg-15-trans-1',
      from: 'thg-15',
      to: 'thg-18',
      mechanism: 'passover',
      description: 'From property to reciprocal action of things',
    },
  ],

  nextStates: ['thg-18'],
  previousStates: ['thg-12'],

  provenance: {
    topicMapId: 'thg-15',
    lineRange: { start: 438, end: 458 },
    section: 'b. Property',
    order: 15,
  },

  description: thingTopicMap[14].description,
  keyPoints: thingTopicMap[14].keyPoints,
};

export const thingIR: DialecticIR = {
  id: 'thing-ir',
  title: 'Thing IR: Concrete Existence, Thing-in-Itself, Property',
  section: 'ESSENCE - B. APPEARANCE - A. THE THING',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'thing.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'thg-1': 'appearance',
      'thg-7': 'appearance',
      'thg-9': 'appearance',
      'thg-12': 'appearance',
      'thg-15': 'appearance',
    },
  },
};

export const thingStates = {
  'thg-1': state1,
  'thg-7': state2,
  'thg-9': state3,
  'thg-12': state4,
  'thg-15': state5,
};
