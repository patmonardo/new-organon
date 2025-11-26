/**
 * Condition IR: Dialectic Pseudo-Code for Condition
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - c. Condition
 *
 * Covers the dialectical movement:
 * - Relatively unconditioned (condition as immediate, presupposition)
 * - Condition and ground (two sides, contradiction)
 * - Absolutely unconditioned (fact in itself, substrate)
 * - Procession (absolute ground, totality of determinations)
 * - Concrete existence (all conditions at hand, fact steps into existence)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'con-1',
  title: 'The relatively unconditioned — condition as immediate',
  concept: 'RelativelyUnconditioned',
  phase: 'reflection',

  moments: [
    {
      name: 'condition',
      definition: 'Immediate to which ground refers as to its essential presupposition',
      type: 'determination',
    },
    {
      name: 'presupposingReflection',
      definition: 'Ground is presupposing reflection, refers to immediate through which mediated',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'condition',
    },
    {
      name: 'unconditioned',
      definition: 'Condition constitutes in-itself of ground, is unconditioned',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'con-1-inv-1',
      constraint: 'ground = presupposingReflection',
      predicate: 'is(ground, presupposingReflection)',
    },
    {
      id: 'con-1-inv-2',
      constraint: 'condition = presupposition(ground)',
      predicate: 'equals(condition, presupposition(ground))',
    },
    {
      id: 'con-1-inv-3',
      constraint: 'realGround.conditioned = true',
      predicate: 'isConditioned(realGround)',
    },
  ],

  forces: [
    {
      id: 'con-1-force-1',
      description: 'Condition and ground drive toward their contradiction',
      type: 'mediation',
      trigger: 'condition.established = true',
      effect: 'contradiction.emerges = true',
      targetState: 'con-4',
    },
  ],

  transitions: [
    {
      id: 'con-1-trans-1',
      from: 'con-1',
      to: 'con-4',
      mechanism: 'mediation',
      description: 'From relatively unconditioned to contradiction of two sides',
    },
  ],

  nextStates: ['con-4'],
  previousStates: ['determinate-ir'],

  provenance: {
    topicMapId: 'con-1',
    lineRange: { start: 6, end: 14 },
    section: 'c. Condition',
    order: 1,
  },

  description: 'Ground is presupposing reflection, refers itself to itself as to something sublated, to immediate through which it is itself mediated. Immediate to which ground refers as to its essential presupposition is condition. Real ground is essentially conditioned.',
};

const state2: DialecticState = {
  id: 'con-4',
  title: 'Two sides — indifferent and mediated',
  concept: 'TwoSidesContradiction',
  phase: 'reflection',

  moments: [
    {
      name: 'indifferentSides',
      definition: 'Two sides indifferent and unconditioned with respect to each other',
      type: 'determination',
    },
    {
      name: 'mediatedSides',
      definition: 'Two sides also mediated',
      type: 'mediation',
      relation: 'opposite',
      relatedTo: 'indifferentSides',
    },
    {
      name: 'contradiction',
      definition: 'Each side is contradiction: indifferent immediacy and essential mediation',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'con-4-inv-1',
      constraint: 'condition.indifferent = true',
      predicate: 'isIndifferent(condition)',
    },
    {
      id: 'con-4-inv-2',
      constraint: 'condition.mediated = true',
      predicate: 'isMediated(condition)',
    },
    {
      id: 'con-4-inv-3',
      constraint: 'eachSide = contradiction',
      predicate: 'is(eachSide, contradiction)',
    },
  ],

  forces: [
    {
      id: 'con-4-force-1',
      description: 'Contradiction drives toward absolutely unconditioned',
      type: 'sublation',
      trigger: 'contradiction.established = true',
      effect: 'absolutelyUnconditioned.emerges = true',
      targetState: 'con-10',
    },
  ],

  transitions: [
    {
      id: 'con-4-trans-1',
      from: 'con-4',
      to: 'con-10',
      mechanism: 'sublation',
      description: 'From contradiction to absolutely unconditioned',
    },
  ],

  nextStates: ['con-10'],
  previousStates: ['con-1'],

  provenance: {
    topicMapId: 'con-4',
    lineRange: { start: 115, end: 146 },
    section: 'c. Condition',
    order: 2,
  },

  description: 'Two sides indifferent and unconditioned, also mediated. Condition is in-itself of ground. Ground-connection has its in-itself outside itself. Each side is contradiction: indifferent immediacy and essential mediation. Contradiction of independent subsistence and being determined as moments.',
};

const state3: DialecticState = {
  id: 'con-10',
  title: 'One essential unity — absolutely unconditioned',
  concept: 'AbsolutelyUnconditioned',
  phase: 'reflection',

  moments: [
    {
      name: 'absolutelyUnconditioned',
      definition: 'Substrate is truly unconditioned, fact in itself',
      type: 'determination',
    },
    {
      name: 'oneIdentity',
      definition: 'Both presuppose one identity for subsistence and substrate',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'absolutelyUnconditioned',
    },
    {
      name: 'factInItself',
      definition: 'Substrate, one content and unity of form of both',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'con-10-inv-1',
      constraint: 'twoSides = oneEssentialUnity',
      predicate: 'equals(twoSides, oneEssentialUnity)',
    },
    {
      id: 'con-10-inv-2',
      constraint: 'substrate = absolutelyUnconditioned',
      predicate: 'equals(substrate, absolutelyUnconditioned)',
    },
    {
      id: 'con-10-inv-3',
      constraint: 'condition.sublated = true',
      predicate: 'sublated(condition)',
    },
  ],

  forces: [
    {
      id: 'con-10-force-1',
      description: 'Absolutely unconditioned drives toward procession',
      type: 'mediation',
      trigger: 'absolutelyUnconditioned.established = true',
      effect: 'procession.emerges = true',
      targetState: 'con-13',
    },
  ],

  transitions: [
    {
      id: 'con-10-trans-1',
      from: 'con-10',
      to: 'con-13',
      mechanism: 'mediation',
      description: 'From absolutely unconditioned to procession',
    },
  ],

  nextStates: ['con-13'],
  previousStates: ['con-4'],

  provenance: {
    topicMapId: 'con-10',
    lineRange: { start: 244, end: 273 },
    section: 'c. Condition',
    order: 3,
  },

  description: 'Two sides one essential unity, as content and form. Both presuppose one identity for subsistence and substrate. Substrate is truly unconditioned, fact in itself. Condition only relatively unconditioned. Sublated in absolutely unconditioned.',
};

const state4: DialecticState = {
  id: 'con-13',
  title: 'Conditions — totality of determinations',
  concept: 'Procession',
  phase: 'reflection',

  moments: [
    {
      name: 'totalityOfDeterminations',
      definition: 'Conditions are totality of determinations, whole content of fact',
      type: 'determination',
    },
    {
      name: 'sphereOfBeing',
      definition: 'For absolute fact, sphere of being itself is condition',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'totalityOfDeterminations',
    },
    {
      name: 'formProliferates',
      definition: 'Form proliferates as determinateness of being',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'con-13-inv-1',
      constraint: 'conditions = totalityOfDeterminations',
      predicate: 'equals(conditions, totalityOfDeterminations)',
    },
    {
      id: 'con-13-inv-2',
      constraint: 'sphereOfBeing = condition',
      predicate: 'is(sphereOfBeing, condition)',
    },
    {
      id: 'con-13-inv-3',
      constraint: 'truthOfExistence = condition',
      predicate: 'equals(truthOfExistence, condition)',
    },
  ],

  forces: [
    {
      id: 'con-13-force-1',
      description: 'All conditions at hand drive fact into concrete existence',
      type: 'passover',
      trigger: 'allConditions.atHand = true',
      effect: 'concreteExistence.emerges = true',
      targetState: 'con-15',
    },
  ],

  transitions: [
    {
      id: 'con-13-trans-1',
      from: 'con-13',
      to: 'con-15',
      mechanism: 'passover',
      description: 'From procession to concrete existence',
    },
  ],

  nextStates: ['con-15'],
  previousStates: ['con-10'],

  provenance: {
    topicMapId: 'con-13',
    lineRange: { start: 320, end: 380 },
    section: 'c. Condition',
    order: 4,
  },

  description: 'Conditions are totality of determinations, whole content of fact. For absolute fact, sphere of being itself is condition. Existence itself makes itself into moment of other. Truth of existence is that it is condition. Immediacy essentially only moment of form.',
};

const state5: DialecticState = {
  id: 'con-15',
  title: 'All conditions at hand — concrete existence',
  concept: 'ConcreteExistence',
  phase: 'reflection',

  moments: [
    {
      name: 'concreteExistence',
      definition: 'When all conditions at hand, fact steps into concrete existence',
      type: 'determination',
    },
    {
      name: 'internalRecollection',
      definition: 'Scattered manifold internally recollects itself',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'concreteExistence',
    },
    {
      name: 'disappearingOfMediation',
      definition: 'Coming forth mediated only by disappearing of mediation',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'con-15-inv-1',
      constraint: 'allConditions.atHand → concreteExistence',
      predicate: 'implies(allConditionsAtHand, concreteExistence)',
    },
    {
      id: 'con-15-inv-2',
      constraint: 'ground.sublated = true',
      predicate: 'sublated(ground)',
    },
    {
      id: 'con-15-inv-3',
      constraint: 'mediation.disappears = true',
      predicate: 'disappears(mediation)',
    },
  ],

  forces: [
    {
      id: 'con-15-force-1',
      description: 'Concrete existence transitions to Existence (Appearance phase)',
      type: 'passover',
      trigger: 'concreteExistence.established = true',
      effect: 'existence.emerges = true',
      targetState: 'existence-1',
    },
  ],

  transitions: [
    {
      id: 'con-15-trans-1',
      from: 'con-15',
      to: 'existence-1',
      mechanism: 'passover',
      description: 'From concrete existence to Existence (Appearance)',
    },
  ],

  nextStates: ['existence-1'],
  previousStates: ['con-13'],

  provenance: {
    topicMapId: 'con-15',
    lineRange: { start: 440, end: 489 },
    section: 'c. Condition',
    order: 5,
  },

  description: 'When all conditions at hand, fact steps into concrete existence. Scattered manifold internally recollects itself. Ground sublated, proves only reflective shine. Coming forth mediated only by disappearing of mediation. Immediacy mediated by ground and condition, self-identical through sublating of mediation, is concrete existence.',
};

export const conditionIR: DialecticIR = {
  id: 'condition-ir',
  title: 'Condition IR: Unconditioned, Procession, Concrete Existence',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - c. Condition',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'condition.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'con-1': 'reflection',
      'con-4': 'reflection',
      'con-10': 'reflection',
      'con-13': 'reflection',
      'con-15': 'reflection',
    },
  },
};

export const conditionStates = {
  'con-1': state1,
  'con-4': state2,
  'con-10': state3,
  'con-13': state4,
  'con-15': state5,
};
