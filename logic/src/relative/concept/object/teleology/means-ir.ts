/**
 * Means IR: Dialectic Pseudo-Code for The Means
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - B. The Means
 *
 * Covers the dialectical movement:
 * - Means as Middle Term: Finitude of purpose, external existence
 * - Means as Mechanical Object: Concept and objectivity externally linked
 * - Means as Totality: Penetrable by purpose, powerless and serving
 *
 * The GPU's purposive means - middle term, mechanical, penetrable
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'means-1',
  title: 'Means as middle term — finitude of purpose',
  concept: 'MeansAsMiddleTerm',
  phase: 'object',

  moments: [
    {
      name: 'meansAsMiddleTerm',
      definition: 'Means is middle term of syllogism, purpose unites with objectivity through means',
      type: 'mediation',
    },
    {
      name: 'finitudeOfPurpose',
      definition: 'Purpose in need of means because finite, external existence indifferent to purpose',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'meansAsMiddleTerm',
    },
  ],

  invariants: [
    {
      id: 'means-1-inv-1',
      constraint: 'means = middleTerm',
      predicate: 'equals(means, middleTerm)',
    },
    {
      id: 'means-1-inv-2',
      constraint: 'purpose.finite = true',
      predicate: 'isFinite(purpose)',
    },
  ],

  forces: [
    {
      id: 'means-1-force-1',
      description: 'Means drives toward mechanical object',
      type: 'mediation',
      trigger: 'means.formalMiddleTerm = true',
      effect: 'mechanicalObject.emerges = true',
      targetState: 'means-4',
    },
  ],

  transitions: [
    {
      id: 'means-1-trans-1',
      from: 'means-1',
      to: 'means-4',
      mechanism: 'mediation',
      description: 'From middle term to mechanical object',
    },
  ],

  nextStates: ['means-4'],
  previousStates: ['teleology-ir'],

  provenance: {
    topicMapId: 'means-2',
    lineRange: { start: 27, end: 78 },
    section: 'B. THE MEANS',
    order: 1,
  },

  description: 'Means is middle term of syllogism. Purpose unites with objectivity through means. Purpose finite, needs means. Means is formal middle term, external to extremes.',
};

const state2: DialecticState = {
  id: 'means-4',
  title: 'Means as totality — penetrable by purpose, powerless and serving',
  concept: 'MeansAsTotality',
  phase: 'object',

  moments: [
    {
      name: 'mechanicalObject',
      definition: 'Concept and objectivity externally linked, means is mechanical object',
      type: 'determination',
    },
    {
      name: 'penetrableByPurpose',
      definition: 'Means is totality of concept, utterly penetrable by purpose, no resistance',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'mechanicalObject',
    },
    {
      name: 'powerlessAndServing',
      definition: 'Object has character of being powerless and serving purpose',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'means-4-inv-1',
      constraint: 'means = mechanicalObject',
      predicate: 'equals(means, mechanicalObject)',
    },
    {
      id: 'means-4-inv-2',
      constraint: 'means.penetrableByPurpose = true',
      predicate: 'isPenetrableByPurpose(means)',
    },
  ],

  forces: [
    {
      id: 'means-4-force-1',
      description: 'Means drives toward realized purpose',
      type: 'mediation',
      trigger: 'activity.throughMeans = true',
      effect: 'realizedPurpose.emerges = true',
      targetState: 'realized-1',
    },
  ],

  transitions: [
    {
      id: 'means-4-trans-1',
      from: 'means-4',
      to: 'realized-1',
      mechanism: 'mediation',
      description: 'From means to realized purpose',
    },
  ],

  nextStates: ['realized-1'],
  previousStates: ['means-1'],

  provenance: {
    topicMapId: 'means-5',
    lineRange: { start: 125, end: 181 },
    section: 'B. THE MEANS',
    order: 2,
  },

  description: 'Means is mechanical object, concept and objectivity externally linked. Means is totality of concept, penetrable by purpose. Object powerless and serving. Activity through means directed against presupposition.',
};

export const meansIR: DialecticIR = {
  id: 'means-ir',
  title: 'Means IR: Middle Term, Mechanical Object, Totality',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - B. The Means',
  states: [state1, state2],
  metadata: {
    sourceFile: 'means.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'means-1': 'object',
      'means-4': 'object',
    },
  },
};

export const meansStates = {
  'means-1': state1,
  'means-4': state2,
};
