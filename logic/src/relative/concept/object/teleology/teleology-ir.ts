/**
 * Teleology IR: Dialectic Pseudo-Code for Subjective Purpose
 *
 * Architecture: GPU Object (Self-Executing Reality)
 * Section: C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - A. Subjective Purpose
 *
 * Covers the dialectical movement:
 * - Purpose Rediscovered: Exempt from transition, rational in concrete existence
 * - Purpose as Syllogism: Self-equal universal, self-determining
 * - Purpose as Finite: Has objective world as presupposition
 * - Realization of Purpose: Sublating presupposition, unification with objective being
 *
 * The GPU's purposive object - self-determining, realizing itself
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'tele-1',
  title: 'Purpose rediscovered — exempt from transition, rational in concrete existence',
  concept: 'SubjectivePurpose',
  phase: 'object',

  moments: [
    {
      name: 'purposeRediscovered',
      definition: 'Purpose rediscovered in centrality and chemism, exempt from transition',
      type: 'determination',
    },
    {
      name: 'rationalInConcreteExistence',
      definition: 'Purpose is rational in concrete existence, concrete concept holding objective difference in absolute unity',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'purposeRediscovered',
    },
    {
      name: 'essentialStrivingAndImpulse',
      definition: 'Purpose is subjective concept as essential striving and impulse to posit itself externally',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'tele-1-inv-1',
      constraint: 'purpose.exemptFromTransition = true',
      predicate: 'isExemptFromTransition(purpose)',
    },
    {
      id: 'tele-1-inv-2',
      constraint: 'purpose = rationalInConcreteExistence',
      predicate: 'equals(purpose, rationalInConcreteExistence)',
    },
  ],

  forces: [
    {
      id: 'tele-1-force-1',
      description: 'Purpose drives toward realization',
      type: 'mediation',
      trigger: 'purpose.striving = true',
      effect: 'realization.emerges = true',
      targetState: 'tele-2',
    },
  ],

  transitions: [
    {
      id: 'tele-1-trans-1',
      from: 'tele-1',
      to: 'tele-2',
      mechanism: 'mediation',
      description: 'From subjective purpose to realization',
    },
  ],

  nextStates: ['tele-2'],
  previousStates: ['chemism-ir'],

  provenance: {
    topicMapId: 'subjective-1',
    lineRange: { start: 2, end: 73 },
    section: 'A. THE SUBJECTIVE PURPOSE',
    order: 1,
  },

  description: 'Purpose rediscovered in centrality and chemism. Exempt from transition. Rational in concrete existence. Purpose is subjective concept as essential striving and impulse to posit itself externally. Essentially syllogism within.',
};

const state2: DialecticState = {
  id: 'tele-2',
  title: 'Realization of purpose — sublating presupposition, unification with objective being',
  concept: 'RealizationOfPurpose',
  phase: 'object',

  moments: [
    {
      name: 'sublatingPresupposition',
      definition: 'Movement directed at sublating presupposition, immediacy of object',
      type: 'sublation',
    },
    {
      name: 'realizationOfPurpose',
      definition: 'Realization is unification of objective being with purpose',
      type: 'process',
      relation: 'contains',
      relatedTo: 'sublatingPresupposition',
    },
    {
      name: 'impulseToRealization',
      definition: 'Purpose is impulse to its realization, concept repels itself from itself',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'tele-2-inv-1',
      constraint: 'purpose.movement = sublatingPresupposition',
      predicate: 'equals(purpose.movement, sublatingPresupposition)',
    },
    {
      id: 'tele-2-inv-2',
      constraint: 'realization = unification(objectiveBeing, purpose)',
      predicate: 'equals(realization, unification(objectiveBeing, purpose))',
    },
  ],

  forces: [
    {
      id: 'tele-2-force-1',
      description: 'Realization of purpose transitions to Idea',
      type: 'passover',
      trigger: 'purpose.realized = true',
      effect: 'idea.emerges = true',
      targetState: 'idea-1',
    },
  ],

  transitions: [
    {
      id: 'tele-2-trans-1',
      from: 'tele-2',
      to: 'idea-1',
      mechanism: 'passover',
      description: 'From teleology to Idea',
    },
  ],

  nextStates: ['idea-1'],
  previousStates: ['tele-1'],

  provenance: {
    topicMapId: 'subjective-4',
    lineRange: { start: 121, end: 164 },
    section: 'A. THE SUBJECTIVE PURPOSE',
    order: 2,
  },

  description: 'Movement of purpose directed at sublating presupposition. Realization is unification of objective being with purpose. Purpose is impulse to realization. Concept repels itself from itself. Posited as means.',
};

export const teleologyIR: DialecticIR = {
  id: 'teleology-ir',
  title: 'Teleology IR: Subjective Purpose, Realization, Idea',
  section: 'C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - A. Subjective Purpose',
  states: [state1, state2],
  metadata: {
    sourceFile: 'subjective.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'tele-1': 'object',
      'tele-2': 'object',
    },
  },
};

export const teleologyStates = {
  'tele-1': state1,
  'tele-2': state2,
};
