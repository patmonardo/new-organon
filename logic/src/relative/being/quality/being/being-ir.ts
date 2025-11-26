/**
 * Being IR: Dialectic Pseudo-Code for Pure Being
 *
 * Architecture: CPU (Quality / Being)
 * Section: A. BEING
 *
 * Covers the initial dialectical movement:
 * - Pure being as indeterminate immediacy
 * - Pure being as emptiness (coincidence with nothing)
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';
import { beingTopicMap } from './sources/being-topic-map';

const state1: DialecticState = {
  id: 'being-1',
  title: 'Pure being without determination — indeterminate immediacy',
  concept: 'PureBeing',
  phase: 'quality',

  moments: [
    {
      name: 'being',
      definition: 'Pure being, without further determination',
      type: 'determination',
    },
    {
      name: 'immediacy',
      definition: 'Indeterminate immediacy equal only to itself',
      type: 'quality',
    },
    {
      name: 'nonDifference',
      definition: 'No difference within or outwardly',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'being-1-inv-1',
      constraint: 'being.hasDetermination = false',
      predicate: 'not(hasDetermination(being))',
    },
    {
      id: 'being-1-inv-2',
      constraint: 'immediacy = indeterminate',
      predicate: 'equals(immediacy, indeterminate)',
    },
    {
      id: 'being-1-inv-3',
      constraint: 'nonDifference.within = true ∧ nonDifference.outward = true',
      predicate: 'nonDifference(nonDifference)',
    },
  ],

  forces: [
    {
      id: 'being-1-force-1',
      description: 'Pure indeterminateness drives toward emptiness (nothing to intuit/think)',
      type: 'passover',
      trigger: 'determination.absent = true',
      effect: 'emptiness.emerges = true',
      targetState: 'being-2',
    },
  ],

  transitions: [
    {
      id: 'being-1-trans-1',
      from: 'being-1',
      to: 'being-2',
      mechanism: 'passover',
      description: 'Pure being passes over into pure emptiness',
    },
  ],

  nextStates: ['being-2'],
  previousStates: [],

  provenance: {
    topicMapId: 'being-1',
    lineRange: { start: 32, end: 39 },
    section: 'A. BEING',
    order: 1,
  },

  description:
    'Being, pure being, without further determination. In its indeterminate immediacy it is equal only to itself and also not unequal with respect to another; it has no difference within it, nor any outwardly.',
  keyPoints: beingTopicMap[0].keyPoints,
};

const state2: DialecticState = {
  id: 'being-2',
  title: 'Pure being as emptiness — nothing to intuit or think',
  concept: 'PureEmptiness',
  phase: 'quality',

  moments: [
    {
      name: 'emptiness',
      definition: 'Pure indeterminateness and emptiness',
      type: 'negation',
    },
    {
      name: 'emptyIntuiting',
      definition: 'Nothing to be intuited except pure intuiting itself',
      type: 'quality',
    },
    {
      name: 'emptyThinking',
      definition: 'Nothing to be thought except empty thinking',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'being-2-inv-1',
      constraint: 'emptiness = indeterminateness',
      predicate: 'equals(emptiness, indeterminateness)',
    },
    {
      id: 'being-2-inv-2',
      constraint: 'emptyIntuiting.onlyIntuiting = true',
      predicate: 'onlyIntuiting(emptyIntuiting)',
    },
    {
      id: 'being-2-inv-3',
      constraint: 'emptyThinking.onlyThinking = true',
      predicate: 'onlyThinking(emptyThinking)',
    },
    {
      id: 'being-2-inv-4',
      constraint: 'pureBeing = nothing',
      predicate: 'equals(pureBeing, nothing)',
    },
  ],

  forces: [
    {
      id: 'being-2-force-1',
      description: 'Recognition that pure being is nothing drives toward nothing-section',
      type: 'sublation',
      trigger: 'pureBeing.equalsNothing = true',
      effect: 'nothing.sectionInitiated = true',
      targetState: 'nothing-1',
    },
  ],

  transitions: [
    {
      id: 'being-2-trans-1',
      from: 'being-2',
      to: 'nothing-1',
      mechanism: 'sublation',
      description: 'Pure being as nothing transitions into the Nothing chapter',
    },
  ],

  nextStates: ['nothing-1'],
  previousStates: ['being-1'],

  provenance: {
    topicMapId: 'being-2',
    lineRange: { start: 40, end: 47 },
    section: 'A. BEING',
    order: 2,
  },

  description:
    'It is pure indeterminateness and emptiness. There is nothing to be intuited in it; just as little is anything to be thought in it. Being, the indeterminate immediate is in fact nothing, and neither more nor less than nothing.',
  keyPoints: beingTopicMap[1].keyPoints,
};

export const beingIR: DialecticIR = {
  id: 'being-ir',
  title: 'Being IR: Pure Being',
  section: 'A. BEING',
  states: [state1, state2],
  metadata: {
    sourceFile: 'being.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'being-1': 'quality',
      'being-2': 'quality',
    },
  },
};

export const beingStates = {
  'being-1': state1,
  'being-2': state2,
};

