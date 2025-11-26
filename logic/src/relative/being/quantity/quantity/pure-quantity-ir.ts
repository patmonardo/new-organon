/**
 * Pure Quantity IR: Dialectic Pseudo-Code for Pure Quantity
 *
 * Architecture: GPU (Quantity / Mathematical Coprocessor)
 * Section: A. PURE QUANTITY
 *
 * Covers the dialectical movement:
 * - Sublated being-for-itself (attraction, continuity)
 * - Continuity as simple self-same reference
 * - Discreteness as repulsion preserved in quantity
 * - Unity of continuity and discreteness
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'quantity-pure-intro',
  title: 'Pure Quantity: sublated being-for-itself, attraction',
  concept: 'PureQuantity',
  phase: 'quantity', // GPU: Quantity / Mathematical Coprocessor

  moments: [
    {
      name: 'sublatedBeingForItself',
      definition: 'Being-for-itself passed over into attraction',
      type: 'sublation',
    },
    {
      name: 'attraction',
      definition: 'Repelling one becomes identical to itself; absolute obduracy melts into unity',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'sublatedBeingForItself',
    },
    {
      name: 'continuity',
      definition: 'Unity containing repulsion, residing in self-externality',
      type: 'quality',
      relation: 'contains',
      relatedTo: 'attraction',
    },
  ],

  invariants: [
    {
      id: 'quantity-1-inv-1',
      constraint: 'beingForItself.sublated = true',
      predicate: 'sublated(beingForItself)',
    },
    {
      id: 'quantity-1-inv-2',
      constraint: 'repellingOne.identicalToItself = true',
      predicate: 'identicalToItself(repellingOne)',
    },
    {
      id: 'quantity-1-inv-3',
      constraint: 'attraction.contains = repulsion',
      predicate: 'contains(attraction, repulsion)',
    },
    {
      id: 'quantity-1-inv-4',
      constraint: 'continuity = moment(attraction)',
      predicate: 'moment(continuity, attraction)',
    },
  ],

  forces: [
    {
      id: 'quantity-1-force-1',
      description: 'Melting of absolute obduracy drives toward simple continuity',
      type: 'sublation',
      trigger: 'obduracy.meltsIntoUnity = true',
      effect: 'continuity.emerges = true',
      targetState: 'quantity-pure-continuity',
    },
  ],

  transitions: [
    {
      id: 'quantity-1-trans-1',
      from: 'quantity-pure-intro',
      to: 'quantity-pure-continuity',
      mechanism: 'sublation',
      description: 'From sublated being-for-itself to pure continuity',
    },
  ],

  nextStates: ['quantity-pure-continuity'],
  previousStates: [],

  provenance: {
    topicMapId: 'quantity-pure-intro',
    lineRange: { start: 3, end: 14 },
    section: 'A. PURE QUANTITY',
    order: 1,
  },

  description: 'Pure Quantity: Sublated being-for-itself. Repelling one now behaves as identical to itself; being-for-itself passes into attraction. The one\'s obduracy melts into unity (containing repulsion); attraction is continuity.',
  keyPoints: [
    'Being-for-itself sublated',
    'Repelling one behaves as identical to itself',
    'Absolute obduracy melts into unity',
    'Attraction is continuity containing repulsion',
  ],
};

const state2: DialecticState = {
  id: 'quantity-pure-continuity',
  title: 'Continuity: simple self-same reference, unity of ones',
  concept: 'Continuity',
  phase: 'quantity',

  moments: [
    {
      name: 'continuity',
      definition: 'Simple self-same reference, unbroken by limit or exclusion',
      type: 'quality',
    },
    {
      name: 'unityOfOnes',
      definition: 'Unity of ones (not immediate unity)',
      type: 'mediation',
    },
    {
      name: 'outsideOneAnother',
      definition: 'Outside-one-another without distinctions',
      type: 'moment',
      relation: 'contains',
      relatedTo: 'unityOfOnes',
    },
    {
      name: 'selfEquality',
      definition: 'Simple undifferentiated equality',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'quantity-2-inv-1',
      constraint: 'continuity.unbroken = true',
      predicate: 'unbroken(continuity)',
    },
    {
      id: 'quantity-2-inv-2',
      constraint: 'unityOfOnes ≠ immediateUnity',
      predicate: 'not(equals(unityOfOnes, immediateUnity))',
    },
    {
      id: 'quantity-2-inv-3',
      constraint: 'outsideOneAnother.hasDistinctions = false',
      predicate: 'not(hasDistinctions(outsideOneAnother))',
    },
    {
      id: 'quantity-2-inv-4',
      constraint: 'many.each = what(others.are)',
      predicate: 'equals(many.each, what(others.are))',
    },
  ],

  forces: [
    {
      id: 'quantity-2-force-1',
      description: 'Unbroken continuity contains implicit plurality, driving toward discreteness',
      type: 'immanence',
      trigger: 'plurality.implicit = true',
      effect: 'discreteness.emerges = true',
      targetState: 'quantity-pure-discreteness',
    },
  ],

  transitions: [
    {
      id: 'quantity-2-trans-1',
      from: 'quantity-pure-continuity',
      to: 'quantity-pure-discreteness',
      mechanism: 'reflection',
      description: 'Implicit plurality in continuity becomes explicit as discreteness',
    },
  ],

  nextStates: ['quantity-pure-discreteness'],
  previousStates: ['quantity-pure-intro'],

  provenance: {
    topicMapId: 'quantity-pure-continuity',
    lineRange: { start: 16, end: 30 },
    section: 'A. PURE QUANTITY',
    order: 2,
  },

  description: 'Continuity: Simple self-same reference unbroken by limit/exclusion. Unity of ones (not immediate unity). Contains outsideness-of-one-another without distinctions. Many are each what others are - simple undifferentiated equality.',
  keyPoints: [
    'Simple self-same reference',
    'Unbroken by limit or exclusion',
    'Unity of ones (not immediate)',
    'Outside-one-another without distinctions',
    'Self-continuation of different ones',
  ],
};

const state3: DialecticState = {
  id: 'quantity-pure-discreteness',
  title: 'Discreteness: repulsion as moment in quantity',
  concept: 'Discreteness',
  phase: 'quantity',

  moments: [
    {
      name: 'discreteness',
      definition: 'Repulsion as moment in quantity',
      type: 'moment',
    },
    {
      name: 'repulsion',
      definition: 'Creative force expanding self-equality to continuity',
      type: 'process',
      relation: 'mediates',
      relatedTo: 'discreteness',
    },
    {
      name: 'confluents',
      definition: 'Ones without void, connecting without negative of own',
      type: 'quality',
    },
    {
      name: 'steadyAdvance',
      definition: 'Advance that does not interrupt self-equality',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'quantity-3-inv-1',
      constraint: 'magnitude.possesses = discreteness',
      predicate: 'possesses(magnitude, discreteness)',
    },
    {
      id: 'quantity-3-inv-2',
      constraint: 'repulsion.expands = selfEqualityToContinuity',
      predicate: 'expands(repulsion, selfEquality, continuity)',
    },
    {
      id: 'quantity-3-inv-3',
      constraint: 'confluents.hasVoid = false',
      predicate: 'not(hasVoid(confluents))',
    },
    {
      id: 'quantity-3-inv-4',
      constraint: 'steadyAdvance.interrupts(selfEquality) = false',
      predicate: 'not(interrupts(steadyAdvance, selfEquality))',
    },
  ],

  forces: [
    {
      id: 'quantity-3-force-1',
      description: 'Discreteness without void, connected in steady advance, prepares unity',
      type: 'mediation',
      trigger: 'steadyAdvance.preservesSelfEquality = true',
      effect: 'unityOfMoments.emerges = true',
      targetState: 'quantity-pure-unity',
    },
  ],

  transitions: [
    {
      id: 'quantity-3-trans-1',
      from: 'quantity-pure-discreteness',
      to: 'quantity-pure-unity',
      mechanism: 'mediation',
      description: 'Discreteness mediates with continuity to form quantity as unity',
    },
  ],

  nextStates: ['quantity-pure-unity'],
  previousStates: ['quantity-pure-continuity'],

  provenance: {
    topicMapId: 'quantity-pure-discreteness',
    lineRange: { start: 32, end: 40 },
    section: 'A. PURE QUANTITY',
    order: 3,
  },

  description: 'Discreteness: Magnitude immediately possesses discreteness (repulsion) as moment in quantity. Repulsion expands self-equality to continuity. Discreteness is of confluents (ones without void), steady advance that does not interrupt self-equality.',
  keyPoints: [
    'Discreteness as moment in quantity',
    'Repulsion expands to continuity',
    'Confluents without void',
    'Steady advance preserving self-equality',
  ],
};

const state4: DialecticState = {
  id: 'quantity-pure-unity',
  title: 'Quantity as unity of continuity and discreteness',
  concept: 'QuantityUnity',
  phase: 'quantity',

  moments: [
    {
      name: 'quantity',
      definition: 'Unity of moments: continuity and discreteness',
      type: 'determination',
    },
    {
      name: 'sublatedBeingForItself',
      definition: 'Being-for-itself collapsed to self-equal immediacy',
      type: 'sublation',
    },
    {
      name: 'creativeFlowingAway',
      definition: 'Repulsion as creative flowing away yielding continuity',
      type: 'process',
    },
    {
      name: 'pluralityInEquality',
      definition: 'Plurality persisting in equality via coming-out-of-itself',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'quantity-4-inv-1',
      constraint: 'quantity = unity(continuity, discreteness)',
      predicate: 'unity(quantity, continuity, discreteness)',
    },
    {
      id: 'quantity-4-inv-2',
      constraint: 'beingForItself.collapsedTo = selfEqualImme diacy',
      predicate: 'collapsedTo(beingForItself, selfEqualImmediacy)',
    },
    {
      id: 'quantity-4-inv-3',
      constraint: 'repulsion.creative = flowingAway',
      predicate: 'equals(repulsion.creative, flowingAway)',
    },
    {
      id: 'quantity-4-inv-4',
      constraint: 'plurality.persistsIn = equality',
      predicate: 'persistsIn(plurality, equality)',
    },
  ],

  forces: [
    {
      id: 'quantity-4-force-1',
      description: 'Unified quantity prepares for further determination as continuous and discrete magnitudes',
      type: 'passover',
      trigger: 'unity.complete = true',
      effect: 'magnitudeDetermination.emerges = true',
      targetState: 'magnitude-1',
    },
  ],

  transitions: [
    {
      id: 'quantity-4-trans-1',
      from: 'quantity-pure-unity',
      to: 'magnitude-1',
      mechanism: 'passover',
      description: 'Pure quantity passes over into determinate magnitude',
    },
  ],

  nextStates: ['magnitude-1'],
  previousStates: ['quantity-pure-discreteness'],

  provenance: {
    topicMapId: 'quantity-pure-unity',
    lineRange: { start: 42, end: 65 },
    section: 'A. PURE QUANTITY',
    order: 4,
  },

  description: 'Quantity: Unity of continuity and discreteness. Contains moments as being-for-itself in truth. Repulsion is creative flowing away; sameness yields unbroken continuity; coming-out-of-itself yields plurality persisting in equality.',
  keyPoints: [
    'Unity of continuity and discreteness',
    'Being-for-itself posited in truth',
    'Repulsion as creative flowing away',
    'Plurality persisting in equality',
    'Self-sublating self-reference',
  ],
};

export const pureQuantityIR: DialecticIR = {
  id: 'pure-quantity-ir',
  title: 'Pure Quantity IR: Sublated Being-for-Itself',
  section: 'A. PURE QUANTITY',
  states: [state1, state2, state3, state4],
  metadata: {
    sourceFile: 'pure-quantity.txt',
    totalStates: 4,
    cpuGpuMapping: {
      'quantity-pure-intro': 'quantity',
      'quantity-pure-continuity': 'quantity',
      'quantity-pure-discreteness': 'quantity',
      'quantity-pure-unity': 'quantity',
    },
  },
};

export const pureQuantityStates = {
  'quantity-pure-intro': state1,
  'quantity-pure-continuity': state2,
  'quantity-pure-discreteness': state3,
  'quantity-pure-unity': state4,
};
