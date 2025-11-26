/**
 * Nothing IR: Dialectic Pseudo-Code for Pure Nothing
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';
import { nothingTopicMap } from './sources/nothing-topic-map';

const nothingState1: DialecticState = {
  id: 'nothing-1',
  title: 'Pure nothingness — simple equality and complete emptiness',
  concept: 'PureNothing',
  phase: 'quality',
  moments: [
    {
      name: 'nothing',
      definition: 'Pure nothingness, simple equality with itself',
      type: 'negation',
    },
    {
      name: 'emptiness',
      definition: 'Complete absence of determination and content',
      type: 'negation',
    },
    {
      name: 'nonDistinction',
      definition: 'Lack of all distinction within',
      type: 'quality',
    },
  ],
  invariants: [
    {
      id: 'nothing-1-inv-1',
      constraint: 'nothing = equalityWithItself',
      predicate: 'equals(nothing, equalityWithItself)',
    },
    {
      id: 'nothing-1-inv-2',
      constraint: 'emptiness.complete = true',
      predicate: 'complete(emptiness)',
    },
    {
      id: 'nothing-1-inv-3',
      constraint: 'nonDistinction.within = true',
      predicate: 'nonDistinction(nonDistinction)',
    },
  ],
  forces: [
    {
      id: 'nothing-1-force-1',
      description: 'Recognition that nothing is empty intuiting/thinking drives to state2',
      type: 'passover',
      trigger: 'emptiness.recognized = true',
      effect: 'emptyIntuitingThinking.emerges = true',
      targetState: 'nothing-2',
    },
  ],
  transitions: [
    {
      id: 'nothing-1-trans-1',
      from: 'nothing-1',
      to: 'nothing-2',
      mechanism: 'passover',
      description: 'Nothing conceived as empty intuiting/thinking',
    },
  ],
  nextStates: ['nothing-2'],
  previousStates: ['being-2'],
  provenance: {
    topicMapId: 'nothing-1',
    lineRange: { start: 12, end: 16 },
    section: 'B. NOTHING',
    order: 1,
  },
  description: nothingTopicMap[0].description,
  keyPoints: nothingTopicMap[0].keyPoints,
};

const nothingState2: DialecticState = {
  id: 'nothing-2',
  title: 'Nothing in intuiting and thinking — same as pure being',
  concept: 'NothingAsPureBeing',
  phase: 'quality',
  moments: [
    {
      name: 'emptyIntuiting',
      definition: 'To intuit nothing is the pure empty intuiting itself',
      type: 'quality',
    },
    {
      name: 'emptyThinking',
      definition: 'To think nothing is empty thinking itself',
      type: 'quality',
    },
    {
      name: 'identityWithBeing',
      definition: 'Nothing is same determination/absence as pure being',
      type: 'mediation',
    },
  ],
  invariants: [
    {
      id: 'nothing-2-inv-1',
      constraint: 'emptyIntuiting = emptyThinking',
      predicate: 'equals(emptyIntuiting, emptyThinking)',
    },
    {
      id: 'nothing-2-inv-2',
      constraint: 'nothing = pureBeing',
      predicate: 'equals(nothing, pureBeing)',
    },
  ],
  forces: [
    {
      id: 'nothing-2-force-1',
      description: 'Identity of being and nothing propels transition to becoming',
      type: 'sublation',
      trigger: 'identityWithBeing.recognized = true',
      effect: 'becomingSection.initiated = true',
      targetState: 'becoming-1',
    },
  ],
  transitions: [
    {
      id: 'nothing-2-trans-1',
      from: 'nothing-2',
      to: 'becoming-1',
      mechanism: 'sublation',
      description: 'Being = Nothing collapses into Becoming',
    },
  ],
  nextStates: ['becoming-1'],
  previousStates: ['nothing-1'],
  provenance: {
    topicMapId: 'nothing-2',
    lineRange: { start: 17, end: 28 },
    section: 'B. NOTHING',
    order: 2,
  },
  description: nothingTopicMap[1].description,
  keyPoints: nothingTopicMap[1].keyPoints,
};

export const nothingIR: DialecticIR = {
  id: 'nothing-ir',
  title: 'Nothing IR: Pure Nothing',
  section: 'B. NOTHING',
  states: [nothingState1, nothingState2],
  metadata: {
    sourceFile: 'nothing.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'nothing-1': 'quality',
      'nothing-2': 'quality',
    },
  },
};

export const nothingStates = {
  'nothing-1': nothingState1,
  'nothing-2': nothingState2,
};

