/**
 * Determinate Ground IR: Dialectic Pseudo-Code for Determinate Ground
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - b. Determinate Ground
 *
 * Covers the dialectical movement:
 * - Formal ground (determinate content, sufficient ground, tautology)
 * - Real ground (diverse content, realized ground)
 * - External ground (ground breaks down)
 * - Complete ground (unity of formal and real)
 * - Conditioning mediation (presupposing reflection)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'det-1',
  title: 'Formal ground — determinate content',
  concept: 'FormalGround',
  phase: 'reflection',

  moments: [
    {
      name: 'determinateContent',
      definition: 'Ground has determinate content, substrate as simple immediate',
      type: 'determination',
    },
    {
      name: 'sufficientGround',
      definition: 'Ground is sufficient, nothing in grounded which is not in ground',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'determinateContent',
    },
    {
      name: 'tautology',
      definition: 'Same determination doubled: as posited and as existence reflected into itself',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'det-1-inv-1',
      constraint: 'ground.content = grounded.content',
      predicate: 'equals(ground.content, grounded.content)',
    },
    {
      id: 'det-1-inv-2',
      constraint: 'ground.sufficient = true',
      predicate: 'isSufficient(ground)',
    },
    {
      id: 'det-1-inv-3',
      constraint: 'content.indifferentToForm = true',
      predicate: 'indifferentTo(content, form)',
    },
  ],

  forces: [
    {
      id: 'det-1-force-1',
      description: 'Formal ground drives toward real ground with diverse content',
      type: 'mediation',
      trigger: 'ground.formal = true',
      effect: 'realGround.emerges = true',
      targetState: 'det-5',
    },
  ],

  transitions: [
    {
      id: 'det-1-trans-1',
      from: 'det-1',
      to: 'det-5',
      mechanism: 'mediation',
      description: 'From formal ground to real ground',
    },
  ],

  nextStates: ['det-5'],
  previousStates: ['absolute-ir'],

  provenance: {
    topicMapId: 'det-1',
    lineRange: { start: 6, end: 17 },
    section: 'b. Determinate Ground',
    order: 1,
  },

  description: 'Ground has determinate content. Ground is negatively self-referring identity. This identity is substrate or content. Ground is sufficient, nothing in grounded which is not in ground. Determinate ground present only in pure form, as formal ground.',
};

const state2: DialecticState = {
  id: 'det-5',
  title: 'Real ground — diverse content',
  concept: 'RealGround',
  phase: 'reflection',

  moments: [
    {
      name: 'diverseContent',
      definition: 'Each side has diverse content as against other',
      type: 'determination',
    },
    {
      name: 'realizedGround',
      definition: 'Ground is realized, no longer tautology',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'diverseContent',
    },
    {
      name: 'twofoldContent',
      definition: 'Grounded is unity of twofold content',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'det-5-inv-1',
      constraint: 'ground.content ≠ grounded.content',
      predicate: 'not(equals(ground.content, grounded.content))',
    },
    {
      id: 'det-5-inv-2',
      constraint: 'groundConnection.formal = false',
      predicate: 'not(is(groundConnection, formal))',
    },
    {
      id: 'det-5-inv-3',
      constraint: 'content.possessesDifference = true',
      predicate: 'possesses(content, differenceOfForm)',
    },
  ],

  forces: [
    {
      id: 'det-5-force-1',
      description: 'Diverse content causes ground to break down into external determinations',
      type: 'negation',
      trigger: 'content.diverse = true',
      effect: 'externalGround.emerges = true',
      targetState: 'det-9',
    },
  ],

  transitions: [
    {
      id: 'det-5-trans-1',
      from: 'det-5',
      to: 'det-9',
      mechanism: 'negation',
      description: 'From real ground to external ground',
    },
  ],

  nextStates: ['det-9'],
  previousStates: ['det-1'],

  provenance: {
    topicMapId: 'det-5',
    lineRange: { start: 107, end: 133 },
    section: 'b. Determinate Ground',
    order: 2,
  },

  description: 'Each side has diverse content as against other. Content essentially possesses difference of form within. Ground-connection has ceased to be formal. Ground is realized. We demand another content determination for ground.',
};

const state3: DialecticState = {
  id: 'det-9',
  title: 'Ground breaks down — external ground',
  concept: 'ExternalGround',
  phase: 'reflection',

  moments: [
    {
      name: 'externalGround',
      definition: 'External ground holds together diversified content',
      type: 'mediation',
    },
    {
      name: 'twoSubstrates',
      definition: 'Two connections are two different substrates',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'externalGround',
    },
    {
      name: 'referenceToAnother',
      definition: 'Real ground is reference to another',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'det-9-inv-1',
      constraint: 'ground.breaksDown = true',
      predicate: 'breaksDown(ground)',
    },
    {
      id: 'det-9-inv-2',
      constraint: 'selfIdenticalForm.vanished = true',
      predicate: 'vanished(selfIdenticalForm)',
    },
    {
      id: 'det-9-inv-3',
      constraint: 'ground = referenceToAnother',
      predicate: 'is(ground, referenceToAnother)',
    },
  ],

  forces: [
    {
      id: 'det-9-force-1',
      description: 'External ground returns to its ground as complete ground',
      type: 'mediation',
      trigger: 'ground.external = true',
      effect: 'completeGround.emerges = true',
      targetState: 'det-10',
    },
  ],

  transitions: [
    {
      id: 'det-9-trans-1',
      from: 'det-9',
      to: 'det-10',
      mechanism: 'mediation',
      description: 'From external ground to complete ground',
    },
  ],

  nextStates: ['det-10'],
  previousStates: ['det-5'],

  provenance: {
    topicMapId: 'det-9',
    lineRange: { start: 207, end: 229 },
    section: 'b. Determinate Ground',
    order: 3,
  },

  description: 'Ground breaks down into external determinations. Two connections are two different substrates. Self-identical form of ground has vanished. External ground holds together diversified content. Real ground is reference to another.',
};

const state4: DialecticState = {
  id: 'det-10',
  title: 'Complete ground — real ground returns to ground',
  concept: 'CompleteGround',
  phase: 'reflection',

  moments: [
    {
      name: 'completeGround',
      definition: 'Contains formal and real ground in itself at same time',
      type: 'determination',
    },
    {
      name: 'immanentReflection',
      definition: 'New ground is immanent reflection of link',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'completeGround',
    },
    {
      name: 'inference',
      definition: 'Ground-connection is inference, mediation through connection',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'det-10-inv-1',
      constraint: 'ground.returned = true',
      predicate: 'returned(ground)',
    },
    {
      id: 'det-10-inv-2',
      constraint: 'completeGround = unity(formal, real)',
      predicate: 'equals(completeGround, unity(formal, real))',
    },
    {
      id: 'det-10-inv-3',
      constraint: 'groundConnection = inference',
      predicate: 'is(groundConnection, inference)',
    },
  ],

  forces: [
    {
      id: 'det-10-force-1',
      description: 'Complete ground drives toward conditioning mediation',
      type: 'mediation',
      trigger: 'ground.complete = true',
      effect: 'conditioningMediation.emerges = true',
      targetState: 'det-14',
    },
  ],

  transitions: [
    {
      id: 'det-10-trans-1',
      from: 'det-10',
      to: 'det-14',
      mechanism: 'mediation',
      description: 'From complete ground to conditioning mediation',
    },
  ],

  nextStates: ['det-14'],
  previousStates: ['det-9'],

  provenance: {
    topicMapId: 'det-10',
    lineRange: { start: 233, end: 254 },
    section: 'b. Determinate Ground',
    order: 4,
  },

  description: 'Ground itself has returned to its ground. New ground is immanent reflection of link. Newly arisen ground-connection is complete. Contains formal and real ground in itself at same time. Mediates content determinations.',
};

const state5: DialecticState = {
  id: 'det-14',
  title: 'Complete mediation — conditioning',
  concept: 'ConditioningMediation',
  phase: 'reflection',

  moments: [
    {
      name: 'conditioningMediation',
      definition: 'Total ground-connection has taken on determination of conditioning mediation',
      type: 'mediation',
    },
    {
      name: 'presupposingReflection',
      definition: 'Ground-connection is essentially presupposing reflection',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'conditioningMediation',
    },
    {
      name: 'selfExternalReflection',
      definition: 'Real ground is self-external reflection of ground',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'det-14-inv-1',
      constraint: 'groundConnection = presupposingReflection',
      predicate: 'is(groundConnection, presupposingReflection)',
    },
    {
      id: 'det-14-inv-2',
      constraint: 'ground.mediatesWithItself = true',
      predicate: 'mediatesWithItself(ground)',
    },
    {
      id: 'det-14-inv-3',
      constraint: 'groundConnection = conditioningMediation',
      predicate: 'equals(groundConnection, conditioningMediation)',
    },
  ],

  forces: [
    {
      id: 'det-14-force-1',
      description: 'Conditioning mediation drives toward condition',
      type: 'passover',
      trigger: 'conditioningMediation.established = true',
      effect: 'condition.emerges = true',
      targetState: 'con-1',
    },
  ],

  transitions: [
    {
      id: 'det-14-trans-1',
      from: 'det-14',
      to: 'con-1',
      mechanism: 'passover',
      description: 'From conditioning mediation to condition',
    },
  ],

  nextStates: ['con-1'],
  previousStates: ['det-10'],

  provenance: {
    topicMapId: 'det-14',
    lineRange: { start: 361, end: 396 },
    section: 'b. Determinate Ground',
    order: 5,
  },

  description: 'Real ground is self-external reflection of ground. Complete mediation is restoration of identity with itself. Ground-connection mediates itself with itself through its negation. Ground-connection is essentially presupposing reflection. Total ground-connection has taken on determination of conditioning mediation.',
};

export const determinateIR: DialecticIR = {
  id: 'determinate-ir',
  title: 'Determinate Ground IR: Formal, Real, Complete, Conditioning',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - b. Determinate Ground',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'determinate.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'det-1': 'reflection',
      'det-5': 'reflection',
      'det-9': 'reflection',
      'det-10': 'reflection',
      'det-14': 'reflection',
    },
  },
};

export const determinateStates = {
  'det-1': state1,
  'det-5': state2,
  'det-9': state3,
  'det-10': state4,
  'det-14': state5,
};
