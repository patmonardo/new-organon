/**
 * Becoming IR: Dialectic Pseudo-Code for Becoming
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';
import { becomingTopicMap } from './sources/becoming-topic-map';

const becomingStates: DialecticState[] = [
  {
    id: 'becoming-1',
    title: 'C.1. Unity of being and nothing — same yet distinct',
    concept: 'UnityBeingNothing',
    phase: 'quality',
    moments: [
      { name: 'being', definition: 'Pure being passing over into nothing', type: 'determination' },
      { name: 'nothing', definition: 'Pure nothing passing over into being', type: 'negation' },
      { name: 'movement', definition: 'Immediate vanishing of one into the other', type: 'mediation' },
    ],
    invariants: [
      { id: 'becoming-1-inv-1', constraint: 'being = nothing', predicate: 'equals(being, nothing)' },
      {
        id: 'becoming-1-inv-2',
        constraint: 'being ≠ nothing (distinct yet inseparable)',
        predicate: 'distinctYetInseparable(being, nothing)',
      },
      {
        id: 'becoming-1-inv-3',
        constraint: 'movement = immediateVanishing',
        predicate: 'equals(movement, immediateVanishing)',
      },
    ],
    forces: [
      {
        id: 'becoming-1-force-1',
        description: 'Movement of vanishing establishes determinate unity',
        type: 'passover',
        trigger: 'movement.present = true',
        effect: 'determinateUnity.emerges = true',
        targetState: 'becoming-2',
      },
    ],
    transitions: [
      {
        id: 'becoming-1-trans-1',
        from: 'becoming-1',
        to: 'becoming-2',
        mechanism: 'passover',
        description: 'From abstract unity to determinate unity',
      },
    ],
    nextStates: ['becoming-2'],
    previousStates: ['nothing-2'],
    provenance: {
      topicMapId: 'becoming-1',
      lineRange: { start: 11, end: 30 },
      section: 'C.1. Unity of being and nothing',
      order: 1,
    },
    description: becomingTopicMap[0].description,
    keyPoints: becomingTopicMap[0].keyPoints,
  },
  {
    id: 'becoming-2',
    title: 'C.2. Becoming as unseparatedness — determinate unity',
    concept: 'Unseparatedness',
    phase: 'quality',
    moments: [
      { name: 'determinateUnity', definition: 'Being and nothing equally are in unity', type: 'mediation' },
      { name: 'vanishing', definition: 'Each only as vanishing moment', type: 'process' },
      { name: 'moments', definition: 'Still distinguished yet sublated', type: 'moment' },
    ],
    invariants: [
      {
        id: 'becoming-2-inv-1',
        constraint: 'determinateUnity.includes = {being, nothing}',
        predicate: 'includes(determinateUnity, {being, nothing})',
      },
      {
        id: 'becoming-2-inv-2',
        constraint: 'vanishing = sublatedBeingNothing',
        predicate: 'equals(vanishing, sublatedBeingNothing)',
      },
    ],
    forces: [
      {
        id: 'becoming-2-force-1',
        description: 'Distinction of moments drives to double determination',
        type: 'mediation',
        trigger: 'moments.distinguished = true',
        effect: 'doubleUnity.emerges = true',
        targetState: 'becoming-3',
      },
    ],
    transitions: [
      {
        id: 'becoming-2-trans-1',
        from: 'becoming-2',
        to: 'becoming-3',
        mechanism: 'mediation',
        description: 'From simple unseparatedness to two unities',
      },
    ],
    nextStates: ['becoming-3'],
    previousStates: ['becoming-1'],
    provenance: {
      topicMapId: 'becoming-2',
      lineRange: { start: 33, end: 44 },
      section: 'C.2. The moments of becoming',
      order: 1,
    },
    description: becomingTopicMap[1].description,
    keyPoints: becomingTopicMap[1].keyPoints,
  },
  {
    id: 'becoming-3',
    title: 'C.2. Two unities — being and nothing as moments',
    concept: 'DoubleUnity',
    phase: 'quality',
    moments: [
      { name: 'unityBeing', definition: 'Being immediate referencing nothing', type: 'determination' },
      { name: 'unityNothing', definition: 'Nothing immediate referencing being', type: 'negation' },
      { name: 'directions', definition: 'Coming-to-be and ceasing-to-be', type: 'process' },
    ],
    invariants: [
      {
        id: 'becoming-3-inv-1',
        constraint: 'directions = {comingToBe, ceasingToBe}',
        predicate: 'equals(directions, {comingToBe, ceasingToBe})',
      },
    ],
    forces: [
      {
        id: 'becoming-3-force-1',
        description: 'Unequal determinations produce interpenetration',
        type: 'contradiction',
        trigger: 'directions.unequal = true',
        effect: 'interpenetration.emerges = true',
        targetState: 'becoming-4',
      },
    ],
    transitions: [
      {
        id: 'becoming-3-trans-1',
        from: 'becoming-3',
        to: 'becoming-4',
        mechanism: 'contradiction',
        description: 'From double unity to interpenetration',
      },
    ],
    nextStates: ['becoming-4'],
    previousStates: ['becoming-2'],
    provenance: {
      topicMapId: 'becoming-3',
      lineRange: { start: 46, end: 60 },
      section: 'C.2. The moments of becoming',
      order: 2,
    },
    description: becomingTopicMap[2].description,
    keyPoints: becomingTopicMap[2].keyPoints,
  },
  {
    id: 'becoming-4',
    title: 'C.2. Coming-to-be and ceasing-to-be — interpenetration',
    concept: 'Interpenetration',
    phase: 'quality',
    moments: [
      { name: 'comingToBe', definition: 'Nothing passes over into being', type: 'process' },
      { name: 'ceasingToBe', definition: 'Being passes over into nothing', type: 'process' },
      { name: 'selfSublation', definition: 'Each sublates itself within itself', type: 'sublation' },
    ],
    invariants: [
      {
        id: 'becoming-4-inv-1',
        constraint: 'comingToBe = ceasingToBe (same becoming)',
        predicate: 'equals(comingToBe, ceasingToBe)',
      },
      {
        id: 'becoming-4-inv-2',
        constraint: 'selfSublation.internal = true',
        predicate: 'internal(selfSublation)',
      },
    ],
    forces: [
      {
        id: 'becoming-4-force-1',
        description: 'Mutual sublation drives toward equilibrium',
        type: 'mediation',
        trigger: 'selfSublation.active = true',
        effect: 'equilibrium.emerges = true',
        targetState: 'becoming-5',
      },
    ],
    transitions: [
      {
        id: 'becoming-4-trans-1',
        from: 'becoming-4',
        to: 'becoming-5',
        mechanism: 'mediation',
        description: 'From interpenetration to equilibrium',
      },
    ],
    nextStates: ['becoming-5'],
    previousStates: ['becoming-3'],
    provenance: {
      topicMapId: 'becoming-4',
      lineRange: { start: 62, end: 82 },
      section: 'C.2. The moments of becoming',
      order: 3,
    },
    description: becomingTopicMap[3].description,
    keyPoints: becomingTopicMap[3].keyPoints,
  },
  {
    id: 'becoming-5',
    title: 'C.3. Equilibrium and quiescent unity — vanishing of becoming',
    concept: 'Equilibrium',
    phase: 'quality',
    moments: [
      { name: 'equilibrium', definition: 'Balance of coming-to-be and ceasing-to-be', type: 'mediation' },
      { name: 'quiescentUnity', definition: 'Becoming collects into quiescent unity', type: 'quality' },
      { name: 'vanishing', definition: 'Vanishing of vanishing itself', type: 'process' },
    ],
    invariants: [
      {
        id: 'becoming-5-inv-1',
        constraint: 'equilibrium = becoming',
        predicate: 'equals(equilibrium, becoming)',
      },
      {
        id: 'becoming-5-inv-2',
        constraint: 'vanishing = vanishingOfVanishing',
        predicate: 'equals(vanishing, vanishingOfVanishing)',
      },
    ],
    forces: [
      {
        id: 'becoming-5-force-1',
        description: 'Quiescent unity hints at contradiction of becoming',
        type: 'reflection',
        trigger: 'quiescentUnity.present = true',
        effect: 'contradiction.emerges = true',
        targetState: 'becoming-6',
      },
    ],
    transitions: [
      {
        id: 'becoming-5-trans-1',
        from: 'becoming-5',
        to: 'becoming-6',
        mechanism: 'reflection',
        description: 'From equilibrium to contradiction of becoming',
      },
    ],
    nextStates: ['becoming-6'],
    previousStates: ['becoming-4'],
    provenance: {
      topicMapId: 'becoming-5',
      lineRange: { start: 85, end: 91 },
      section: 'C.3. Sublation of becoming',
      order: 1,
    },
    description: becomingTopicMap[4].description,
    keyPoints: becomingTopicMap[4].keyPoints,
  },
  {
    id: 'becoming-6',
    title: 'C.3. Contradiction and vanishedness — not nothing',
    concept: 'Vanishedness',
    phase: 'quality',
    moments: [
      { name: 'contradiction', definition: 'Union of distinct being & nothing destroys itself', type: 'negation' },
      { name: 'vanishedness', definition: 'Result of becoming, but not nothing', type: 'determination' },
      { name: 'distinction', definition: 'Becoming rests on being distinct', type: 'determination' },
    ],
    invariants: [
      {
        id: 'becoming-6-inv-1',
        constraint: 'vanishedness ≠ nothing',
        predicate: 'notEquals(vanishedness, nothing)',
      },
    ],
    forces: [
      {
        id: 'becoming-6-force-1',
        description: 'Vanishedness prepares quiescent simplicity',
        type: 'sublation',
        trigger: 'vanishedness.established = true',
        effect: 'quiescentSimplicity.emerges = true',
        targetState: 'becoming-7',
      },
    ],
    transitions: [
      {
        id: 'becoming-6-trans-1',
        from: 'becoming-6',
        to: 'becoming-7',
        mechanism: 'sublation',
        description: 'From contradiction to quiescent simplicity',
      },
    ],
    nextStates: ['becoming-7'],
    previousStates: ['becoming-5'],
    provenance: {
      topicMapId: 'becoming-6',
      lineRange: { start: 93, end: 99 },
      section: 'C.3. Sublation of becoming',
      order: 2,
    },
    description: becomingTopicMap[5].description,
    keyPoints: becomingTopicMap[5].keyPoints,
  },
  {
    id: 'becoming-7',
    title: 'C.3. Transition to existence — quiescent simplicity',
    concept: 'TransitionToExistence',
    phase: 'quality',
    moments: [
      { name: 'quiescentSimplicity', definition: 'Unity of being and nothing become simple', type: 'quality' },
      { name: 'beingAsDetermination', definition: 'Being as determination of whole', type: 'determination' },
      { name: 'existence', definition: 'Unity as existent immediate result', type: 'determination' },
    ],
    invariants: [
      {
        id: 'becoming-7-inv-1',
        constraint: 'quiescentSimplicity = being (determination of whole)',
        predicate: 'equals(quiescentSimplicity, beingDetermination)',
      },
      {
        id: 'becoming-7-inv-2',
        constraint: 'unity.asExistent = existence',
        predicate: 'equals(unityAsExistent, existence)',
      },
    ],
    forces: [
      {
        id: 'becoming-7-force-1',
        description: 'Quiescent unity transitions to existence chapter',
        type: 'passover',
        trigger: 'existence.result = true',
        effect: 'existenceSection.initiated = true',
        targetState: 'existence-1',
      },
    ],
    transitions: [
      {
        id: 'becoming-7-trans-1',
        from: 'becoming-7',
        to: 'existence-1',
        mechanism: 'passover',
        description: 'Becoming collapses into Existence',
      },
    ],
    nextStates: ['existence-1'],
    previousStates: ['becoming-6'],
    provenance: {
      topicMapId: 'becoming-7',
      lineRange: { start: 101, end: 108 },
      section: 'C.3. Sublation of becoming',
      order: 3,
    },
    description: becomingTopicMap[6].description,
    keyPoints: becomingTopicMap[6].keyPoints,
  },
];

export const becomingIR: DialecticIR = {
  id: 'becoming-ir',
  title: 'Becoming IR: Unity of Being and Nothing',
  section: 'C. BECOMING',
  states: becomingStates,
  metadata: {
    sourceFile: 'becoming.txt',
    totalStates: becomingStates.length,
    cpuGpuMapping: Object.fromEntries(becomingStates.map((s) => [s.id, 'quality'])),
  },
};

export const becomingStateMap = Object.fromEntries(becomingStates.map((s) => [s.id, s]));

