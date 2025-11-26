/**
 * Essence IR: Dialectic Pseudo-Code for Essence
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: ESSENCE - A. ESSENCE AS REFLECTION WITHIN ITSELF
 *
 * Covers the dialectical movement:
 * - Truth of being is essence (mediated knowledge)
 * - Essence as sublated being (critique of external abstraction)
 * - Essence as absolute negativity (being-in-and-for-itself)
 * - Essence must differentiate into existence
 * - Essence as reflection (compared to quality/quantity)
 * - Essence as middle term between being and concept
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';
import { essenceTopicMap } from './sources/essence-topic-map';

const state1: DialecticState = {
  id: 'ess-1',
  title: 'Truth of being is essence — mediated knowledge',
  concept: 'EssenceAsTruth',
  phase: 'reflection', // CPU: Reflection / Essence

  moments: [
    {
      name: 'being',
      definition: 'The immediate from which knowledge seeks truth',
      type: 'determination',
    },
    {
      name: 'essence',
      definition: 'The truth of being, found through mediation',
      type: 'determination',
      relation: 'mediates',
      relatedTo: 'being',
    },
    {
      name: 'recollection',
      definition: 'Knowledge recollects itself from immediate being',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'ess-1-inv-1',
      constraint: 'being = immediate',
      predicate: 'is(being, immediate)',
    },
    {
      id: 'ess-1-inv-2',
      constraint: 'essence = truthOf(being)',
      predicate: 'truthOf(essence, being)',
    },
    {
      id: 'ess-1-inv-3',
      constraint: 'essence.foundThrough = mediation',
      predicate: 'foundThrough(essence, mediation)',
    },
    {
      id: 'ess-1-inv-4',
      constraint: 'movement.isBeing\'sOwnNature = true',
      predicate: 'isBeingsOwnNature(movement)',
    },
  ],

  forces: [
    {
      id: 'ess-1-force-1',
      description: 'Knowledge penetrates beyond immediacy, seeking essence',
      type: 'mediation',
      trigger: 'knowledge.seeksTruth = true',
      effect: 'essence.emerges = true',
      targetState: 'ess-2',
    },
  ],

  transitions: [
    {
      id: 'ess-1-trans-1',
      from: 'ess-1',
      to: 'ess-2',
      mechanism: 'mediation',
      middleTerm: 'recollection',
      description: 'From immediate being to essence as sublated being',
    },
  ],

  nextStates: ['ess-2'],
  previousStates: [],

  provenance: {
    topicMapId: 'ess-1',
    lineRange: { start: 7, end: 43 },
    section: 'ESSENCE',
    order: 1,
  },

  description: essenceTopicMap[0].description,
  keyPoints: essenceTopicMap[0].keyPoints,
};

const state2: DialecticState = {
  id: 'ess-2',
  title: 'Essence as sublated being — external abstraction critique',
  concept: 'EssenceAsSublatedBeing',
  phase: 'reflection',

  moments: [
    {
      name: 'sublatedBeing',
      definition: 'Being that has undergone negation',
      type: 'sublation',
    },
    {
      name: 'externalAbstraction',
      definition: 'External reflection producing empty artifact',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'recollection',
    },
    {
      name: 'recollection',
      definition: 'Immanent movement presupposed by pure being',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'ess-2-inv-1',
      constraint: 'absolute.firstDetermination = being',
      predicate: 'firstDetermination(absolute, being)',
    },
    {
      id: 'ess-2-inv-2',
      constraint: 'absolute.secondDetermination = essence',
      predicate: 'secondDetermination(absolute, essence)',
    },
    {
      id: 'ess-2-inv-3',
      constraint: 'externalAbstraction.produces = emptyArtifact',
      predicate: 'produces(externalAbstraction, emptyArtifact)',
    },
    {
      id: 'ess-2-inv-4',
      constraint: 'essenceThroughExternalReflection.isInItself = false',
      predicate: 'not(isInItself(essenceThroughExternalReflection))',
    },
  ],

  forces: [
    {
      id: 'ess-2-force-1',
      description: 'Dead emptiness of external abstraction drives toward absolute negativity',
      type: 'negation',
      trigger: 'externalAbstraction.fails = true',
      effect: 'absoluteNegativity.sought = true',
      targetState: 'ess-3',
    },
  ],

  transitions: [
    {
      id: 'ess-2-trans-1',
      from: 'ess-2',
      to: 'ess-3',
      mechanism: 'negation',
      description: 'From external abstraction to essence through its own negativity',
    },
  ],

  nextStates: ['ess-3'],
  previousStates: ['ess-1'],

  provenance: {
    topicMapId: 'ess-2',
    lineRange: { start: 45, end: 86 },
    section: 'ESSENCE',
    order: 2,
  },

  description: essenceTopicMap[1].description,
  keyPoints: essenceTopicMap[1].keyPoints,
};

const state3: DialecticState = {
  id: 'ess-3',
  title: 'Essence as absolute negativity — being-in-and-for-itself',
  concept: 'AbsoluteNegativity',
  phase: 'reflection',

  moments: [
    {
      name: 'absoluteNegativity',
      definition: 'Essence through its own negativity, infinite movement of being',
      type: 'negation',
    },
    {
      name: 'beingInAndForItself',
      definition: 'Absolute in-itselfness, also being-for-itself',
      type: 'determination',
    },
    {
      name: 'sublationOfOtherness',
      definition: 'Self-sublation of otherness and determinateness',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'absoluteNegativity',
    },
  ],

  invariants: [
    {
      id: 'ess-3-inv-1',
      constraint: 'essence.negativity = absolute',
      predicate: 'absolute(essence.negativity)',
    },
    {
      id: 'ess-3-inv-2',
      constraint: 'essence = beingInAndForItself',
      predicate: 'is(essence, beingInAndForItself)',
    },
    {
      id: 'ess-3-inv-3',
      constraint: 'essence.indifferentTo = everyDeterminatenessOfBeing',
      predicate: 'indifferentTo(essence, everyDeterminatenessOfBeing)',
    },
    {
      id: 'ess-3-inv-4',
      constraint: 'essence.inItself ∧ essence.forItself = true',
      predicate: 'inItself(essence) ∧ forItself(essence)',
    },
  ],

  forces: [
    {
      id: 'ess-3-force-1',
      description: 'Indeterminate essence must differentiate into existence',
      type: 'reflection',
      trigger: 'essence.indeterminate = true',
      effect: 'differentiationIntoExistence.required = true',
      targetState: 'ess-4',
    },
  ],

  transitions: [
    {
      id: 'ess-3-trans-1',
      from: 'ess-3',
      to: 'ess-4',
      mechanism: 'reflection',
      description: 'From absolute negativity to differentiation into existence',
    },
  ],

  nextStates: ['ess-4'],
  previousStates: ['ess-2'],

  provenance: {
    topicMapId: 'ess-3',
    lineRange: { start: 87, end: 103 },
    section: 'ESSENCE',
    order: 3,
  },

  description: essenceTopicMap[2].description,
  keyPoints: essenceTopicMap[2].keyPoints,
};

const state4: DialecticState = {
  id: 'ess-4',
  title: 'Indeterminate essence — must differentiate into existence',
  concept: 'DifferentiationIntoExistence',
  phase: 'reflection',

  moments: [
    {
      name: 'indeterminateEssence',
      definition: 'Essence as complete turning back of being into itself',
      type: 'determination',
    },
    {
      name: 'repellingOfItself',
      definition: 'Negative self-reference, differentiating itself',
      type: 'negation',
      relation: 'transforms',
      relatedTo: 'indeterminateEssence',
    },
    {
      name: 'existenceDetermination',
      definition: 'Determinations different from determinations of being',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'ess-4-inv-1',
      constraint: 'essence = completeTurningBackOfBeingIntoItself',
      predicate: 'is(essence, completeTurningBack(being))',
    },
    {
      id: 'ess-4-inv-2',
      constraint: 'indeterminateEssence.holdsWithoutPositing = determinations',
      predicate: 'holdsWithoutPositing(indeterminateEssence, determinations)',
    },
    {
      id: 'ess-4-inv-3',
      constraint: 'essence.mustPassOver = existence',
      predicate: 'mustPassOver(essence, existence)',
    },
    {
      id: 'ess-4-inv-4',
      constraint: 'essenceDeterminations ≠ beingDeterminations',
      predicate: 'not(equals(essenceDeterminations, beingDeterminations))',
    },
  ],

  forces: [
    {
      id: 'ess-4-force-1',
      description: 'Repelling of itself from itself drives comparison with quality/quantity',
      type: 'reflection',
      trigger: 'negatively.selfRefers = true',
      effect: 'comparisonEmerges = {quality, quantity}',
      targetState: 'ess-5',
    },
  ],

  transitions: [
    {
      id: 'ess-4-trans-1',
      from: 'ess-4',
      to: 'ess-5',
      mechanism: 'reflection',
      description: 'From indeterminate essence to reflection as negativity',
    },
  ],

  nextStates: ['ess-5'],
  previousStates: ['ess-3'],

  provenance: {
    topicMapId: 'ess-4',
    lineRange: { start: 105, end: 135 },
    section: 'ESSENCE',
    order: 4,
  },

  description: essenceTopicMap[3].description,
  keyPoints: essenceTopicMap[3].keyPoints,
};

const state5: DialecticState = {
  id: 'ess-5',
  title: 'Essence compared to quality and quantity — reflection',
  concept: 'EssenceAsReflection',
  phase: 'reflection',

  moments: [
    {
      name: 'essenceToWhole',
      definition: 'Relation of essence to whole, like quality to being',
      type: 'determination',
    },
    {
      name: 'reflection',
      definition: 'Negativity of essence; determinations posited by essence',
      type: 'mediation',
    },
    {
      name: 'reflectedDeterminations',
      definition: 'Posited by essence, remain as sublated',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'reflection',
    },
  ],

  invariants: [
    {
      id: 'ess-5-inv-1',
      constraint: 'essence:whole = quality:being',
      predicate: 'analogous(essence, whole, quality, being)',
    },
    {
      id: 'ess-5-inv-2',
      constraint: 'quantity.indifference = inImmediateDetermination',
      predicate: 'indifferenceIn(quantity, immediateDetermination)',
    },
    {
      id: 'ess-5-inv-3',
      constraint: 'essence.determinateness.exists = false',
      predicate: 'not(exists(essence.determinateness))',
    },
    {
      id: 'ess-5-inv-4',
      constraint: 'determinateness.positedBy = essence',
      predicate: 'positedBy(determinateness, essence)',
    },
    {
      id: 'ess-5-inv-5',
      constraint: 'essence.negativity = reflection',
      predicate: 'is(essence.negativity, reflection)',
    },
  ],

  forces: [
    {
      id: 'ess-5-force-1',
      description: 'Reflection as positing negativity drives toward middle-term role',
      type: 'mediation',
      trigger: 'reflection.posits = determinations',
      effect: 'middleTermEmerges = true',
      targetState: 'ess-6',
    },
  ],

  transitions: [
    {
      id: 'ess-5-trans-1',
      from: 'ess-5',
      to: 'ess-6',
      mechanism: 'mediation',
      description: 'From reflection to essence as middle term',
    },
  ],

  nextStates: ['ess-6'],
  previousStates: ['ess-4'],

  provenance: {
    topicMapId: 'ess-5',
    lineRange: { start: 137, end: 150 },
    section: 'ESSENCE',
    order: 5,
  },

  description: essenceTopicMap[4].description,
  keyPoints: essenceTopicMap[4].keyPoints,
};

const state6: DialecticState = {
  id: 'ess-6',
  title: 'Essence as middle term — transition to concept',
  concept: 'EssenceAsMiddleTerm',
  phase: 'reflection',

  moments: [
    {
      name: 'middleTerm',
      definition: 'Essence stands between being and concept',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'being',
    },
    {
      name: 'firstNegation',
      definition: 'Essence as first negation of being',
      type: 'negation',
    },
    {
      name: 'movement',
      definition: 'Positing negation/determination, giving itself existence',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'ess-6-inv-1',
      constraint: 'essence.standsBetween = {being, concept}',
      predicate: 'standsBetween(essence, being, concept)',
    },
    {
      id: 'ess-6-inv-2',
      constraint: 'essence = beingInAndForItself.inDeterminationOfBeingInItself',
      predicate: 'is(essence, beingInAndForItself(beingInItself))',
    },
    {
      id: 'ess-6-inv-3',
      constraint: 'essence = firstNegation(being)',
      predicate: 'equals(essence, firstNegation(being))',
    },
    {
      id: 'ess-6-inv-4',
      constraint: 'concept.emergesWhen = existence.equalsBeingInItself',
      predicate: 'emergesWhen(concept, equals(existence, beingInItself))',
    },
  ],

  forces: [
    {
      id: 'ess-6-force-1',
      description: 'Middle-term role prepares threefold structure: shine, appearance, actuality',
      type: 'passover',
      trigger: 'movement.complete = true',
      effect: 'threefoldStructure.emerges = true',
      targetState: 'ess-7',
    },
  ],

  transitions: [
    {
      id: 'ess-6-trans-1',
      from: 'ess-6',
      to: 'ess-7',
      mechanism: 'passover',
      description: 'From middle term to threefold structure',
    },
  ],

  nextStates: ['ess-7'],
  previousStates: ['ess-5'],

  provenance: {
    topicMapId: 'ess-6',
    lineRange: { start: 152, end: 175 },
    section: 'ESSENCE',
    order: 6,
  },

  description: essenceTopicMap[5].description,
  keyPoints: essenceTopicMap[5].keyPoints,
};

export const essenceIR: DialecticIR = {
  id: 'essence-ir',
  title: 'Essence IR: Truth of Being, Absolute Negativity, Reflection',
  section: 'ESSENCE - A. ESSENCE AS REFLECTION WITHIN ITSELF',
  states: [state1, state2, state3, state4, state5, state6],
  metadata: {
    sourceFile: 'essence.txt',
    totalStates: 6,
    cpuGpuMapping: {
      'ess-1': 'reflection',
      'ess-2': 'reflection',
      'ess-3': 'reflection',
      'ess-4': 'reflection',
      'ess-5': 'reflection',
      'ess-6': 'reflection',
    },
  },
};

export const essenceStates = {
  'ess-1': state1,
  'ess-2': state2,
  'ess-3': state3,
  'ess-4': state4,
  'ess-5': state5,
  'ess-6': state6,
};
