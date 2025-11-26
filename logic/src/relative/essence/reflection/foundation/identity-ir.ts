/**
 * Identity IR: Dialectic Pseudo-Code for Identity
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - A. Identity
 *
 * Covers the dialectical movement:
 * - Essence as simple self-identity (pure identity)
 * - Essential identity (immediacy of reflection)
 * - Identity as negativity (being's negativity is identity itself)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'idn-3',
  title: 'Essence as simple self-identity — pure identity',
  concept: 'PureIdentity',
  phase: 'reflection',

  moments: [
    {
      name: 'simpleSelfIdentity',
      definition: 'Essence is simple immediacy as sublated immediacy',
      type: 'determination',
    },
    {
      name: 'pureSelfEquality',
      definition: 'Otherness and reference to other have disappeared into pure self-equality',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'simpleSelfIdentity',
    },
    {
      name: 'absenceOfDetermination',
      definition: 'This determination is absence of determination',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'idn-3-inv-1',
      constraint: 'essence = simpleSelfIdentity',
      predicate: 'equals(essence, simpleSelfIdentity)',
    },
    {
      id: 'idn-3-inv-2',
      constraint: 'negativity = being',
      predicate: 'equals(negativity, being)',
    },
    {
      id: 'idn-3-inv-3',
      constraint: 'otherness.disappeared = true',
      predicate: 'disappeared(otherness)',
    },
  ],

  forces: [
    {
      id: 'idn-3-force-1',
      description: 'Pure identity drives toward essential identity as production',
      type: 'mediation',
      trigger: 'identity.pure = true',
      effect: 'essentialIdentity.emerges = true',
      targetState: 'idn-4',
    },
  ],

  transitions: [
    {
      id: 'idn-3-trans-1',
      from: 'idn-3',
      to: 'idn-4',
      mechanism: 'mediation',
      description: 'From pure identity to essential identity',
    },
  ],

  nextStates: ['idn-4'],
  previousStates: ['reflection-ir'], // Connects from Reflection IR

  provenance: {
    topicMapId: 'idn-3',
    lineRange: { start: 32, end: 39 },
    section: 'A. Identity',
    order: 1,
  },

  description: 'Essence is simple immediacy as sublated immediacy. Its negativity is its being. Equal to itself in absolute negativity. By virtue of which otherness and reference to other have disappeared into pure self-equality. Essence is therefore simple self-identity.',
};

const state2: DialecticState = {
  id: 'idn-4',
  title: 'Self-identity as immediacy of reflection — essential identity',
  concept: 'EssentialIdentity',
  phase: 'reflection',

  moments: [
    {
      name: 'essentialIdentity',
      definition: 'Self-identity is immediacy of reflection',
      type: 'determination',
    },
    {
      name: 'pureProduction',
      definition: 'Pure production, from itself and in itself',
      type: 'process',
      relation: 'contains',
      relatedTo: 'essentialIdentity',
    },
    {
      name: 'notAbstractIdentity',
      definition: 'Not abstract identity, not result of relative negation',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'idn-4-inv-1',
      constraint: 'identity = immediacy(reflection)',
      predicate: 'equals(identity, immediacy(reflection))',
    },
    {
      id: 'idn-4-inv-2',
      constraint: 'production.source = itself',
      predicate: 'equals(source(production), itself)',
    },
    {
      id: 'idn-4-inv-3',
      constraint: 'identity.abstract = false',
      predicate: 'not(is(identity, abstract))',
    },
  ],

  forces: [
    {
      id: 'idn-4-force-1',
      description: 'Essential identity reveals being\'s negativity as identity itself',
      type: 'negation',
      trigger: 'identity.essential = true',
      effect: 'negativityAsIdentity.emerges = true',
      targetState: 'idn-6',
    },
  ],

  transitions: [
    {
      id: 'idn-4-trans-1',
      from: 'idn-4',
      to: 'idn-6',
      mechanism: 'negation',
      description: 'From essential identity to identity as negativity',
    },
  ],

  nextStates: ['idn-6'],
  previousStates: ['idn-3'],

  provenance: {
    topicMapId: 'idn-4',
    lineRange: { start: 41, end: 46 },
    section: 'A. Identity',
    order: 2,
  },

  description: 'This self-identity is immediacy of reflection. Not self-equality which being is, or also nothing. Self-equality which, in producing itself as unity, does not produce itself over again, as from another. Pure production, from itself and in itself. Essential identity.',
};

const state3: DialecticState = {
  id: 'idn-6',
  title: 'Being\'s negativity is identity itself',
  concept: 'IdentityAsNegativity',
  phase: 'reflection',

  moments: [
    {
      name: 'negativityOfBeing',
      definition: 'Negativity of being in itself is the identity itself',
      type: 'negation',
    },
    {
      name: 'simpleNegativity',
      definition: 'Simple negativity, sublated not relatively but in itself',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'negativityOfBeing',
    },
    {
      name: 'sameAsEssence',
      definition: 'In general, therefore, it is still the same as essence',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'idn-6-inv-1',
      constraint: 'identity = negativity(beingInItself)',
      predicate: 'equals(identity, negativity(beingInItself))',
    },
    {
      id: 'idn-6-inv-2',
      constraint: 'sublation.relative = false',
      predicate: 'not(is(sublation, relative))',
    },
    {
      id: 'idn-6-inv-3',
      constraint: 'identity = essence',
      predicate: 'equals(identity, essence)',
    },
  ],

  forces: [
    {
      id: 'idn-6-force-1',
      description: 'Identity as negativity drives toward Difference',
      type: 'negation',
      trigger: 'identity.isNegativity = true',
      effect: 'difference.emerges = true',
      targetState: 'diff-1',
    },
  ],

  transitions: [
    {
      id: 'idn-6-trans-1',
      from: 'idn-6',
      to: 'diff-1',
      mechanism: 'negation',
      description: 'From identity to difference',
    },
  ],

  nextStates: ['diff-1'],
  previousStates: ['idn-4'],

  provenance: {
    topicMapId: 'idn-6',
    lineRange: { start: 56, end: 63 },
    section: 'A. Identity',
    order: 3,
  },

  description: 'Being, and every determinateness of being, has sublated itself not relatively, but in itself. This simple negativity, negativity of being in itself, is the identity itself. In general, therefore, it is still the same as essence.',
};

export const identityIR: DialecticIR = {
  id: 'identity-ir',
  title: 'Identity IR: Essence as Simple Self-Identity',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - A. Identity',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'identity.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'idn-3': 'reflection',
      'idn-4': 'reflection',
      'idn-6': 'reflection',
    },
  },
};

export const identityStates = {
  'idn-3': state1,
  'idn-4': state2,
  'idn-6': state3,
};
