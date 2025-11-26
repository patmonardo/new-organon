/**
 * Shine IR: Dialectic Pseudo-Code for Shine
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 1. Shine
 *
 * Covers the dialectical movement:
 * - Being is shine (negative posited as negative)
 * - Shine as remainder of being (immediate non-existence)
 * - Shine in skepticism and idealism (manifold determinations)
 * - Shine as essence's negativity (immediacy of non-being)
 * - Shine as infinite determinateness (essence itself)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'shn-1',
  title: 'Being is shine — negative posited as negative',
  concept: 'BeingIsShine',
  phase: 'reflection',

  moments: [
    {
      name: 'shine',
      definition: 'Being of shine consists solely in sublatedness of being',
      type: 'sublation',
    },
    {
      name: 'nothingness',
      definition: 'Nothingness it has in essence; apart from essence, does not exist',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'shine',
    },
    {
      name: 'negativePosited',
      definition: 'The negative posited as negative',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'shn-1-inv-1',
      constraint: 'shine.being = sublatedness(being)',
      predicate: 'equals(shine.being, sublatedness(being))',
    },
    {
      id: 'shn-1-inv-2',
      constraint: 'shine.existsApartFromEssence = false',
      predicate: 'not(existsApartFrom(shine, essence))',
    },
    {
      id: 'shn-1-inv-3',
      constraint: 'shine = negativePositedAsNegative',
      predicate: 'is(shine, negativePositedAsNegative)',
    },
  ],

  forces: [
    {
      id: 'shn-1-force-1',
      description: 'Negative posited as negative drives toward immediate non-existence',
      type: 'negation',
      trigger: 'shine.isNegative = true',
      effect: 'immediateNonExistence.emerges = true',
      targetState: 'shn-2',
    },
  ],

  transitions: [
    {
      id: 'shn-1-trans-1',
      from: 'shn-1',
      to: 'shn-2',
      mechanism: 'negation',
      description: 'From shine as negative to shine as immediate non-existence',
    },
  ],

  nextStates: ['shn-2'],
  previousStates: [],

  provenance: {
    topicMapId: 'shn-1',
    lineRange: { start: 4, end: 12 },
    section: '1. Shine',
    order: 1,
  },

  description: 'Being of shine consists solely in sublatedness of being, in being\'s nothingness. This nothingness it has in essence. Apart from essence, it does not exist. It is the negative posited as negative.',
};

const state2: DialecticState = {
  id: 'shn-2',
  title: 'Shine as remainder of being — immediate non-existence',
  concept: 'ShineAsRemainder',
  phase: 'reflection',

  moments: [
    {
      name: 'remainderOfBeing',
      definition: 'Shine is all that remains of sphere of being',
      type: 'determination',
    },
    {
      name: 'immediateNonExistence',
      definition: 'Shine is immediate non-existence, non-existence in determinateness of being',
      type: 'negation',
    },
    {
      name: 'reflectedImmediacy',
      definition: 'Pure determinateness of immediacy, reflected immediacy',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'immediateNonExistence',
    },
  ],

  invariants: [
    {
      id: 'shn-2-inv-1',
      constraint: 'shine = remainder(sphereOfBeing)',
      predicate: 'equals(shine, remainder(sphereOfBeing))',
    },
    {
      id: 'shn-2-inv-2',
      constraint: 'shine.immediacy = reflected',
      predicate: 'is(shine.immediacy, reflected)',
    },
    {
      id: 'shn-2-inv-3',
      constraint: 'shine.existsOnlyInNegation = true',
      predicate: 'existsOnlyIn(shine, negation)',
    },
  ],

  forces: [
    {
      id: 'shn-2-force-1',
      description: 'Reflected immediacy allows manifold determinations (skepticism/idealism)',
      type: 'reflection',
      trigger: 'immediacy.reflected = true',
      effect: 'manifoldDeterminations.appear = true',
      targetState: 'shn-3',
    },
  ],

  transitions: [
    {
      id: 'shn-2-trans-1',
      from: 'shn-2',
      to: 'shn-3',
      mechanism: 'reflection',
      description: 'From immediate non-existence to manifold determinations',
    },
  ],

  nextStates: ['shn-3'],
  previousStates: ['shn-1'],

  provenance: {
    topicMapId: 'shn-2',
    lineRange: { start: 14, end: 36 },
    section: '1. Shine',
    order: 2,
  },

  description: 'Shine is all that remains of sphere of being. Seems to have immediate side independent of essence. Shine is immediate non-existence, non-existence in determinateness of being. Non-self-subsistent which exists only in its negation. Pure determinateness of immediacy, reflected immediacy.',
};

const state3: DialecticState = {
  id: 'shn-3',
  title: 'Shine in skepticism and idealism — manifold determinations',
  concept: 'ShineDeterminations',
  phase: 'reflection',

  moments: [
    {
      name: 'phenomenon',
      definition: 'Shine as phenomenon (skepticism) or appearance (idealism)',
      type: 'determination',
    },
    {
      name: 'manifoldDeterminations',
      definition: 'Full richness of world allowed as manifold of determinations',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'phenomenon',
    },
    {
      name: 'transposedContent',
      definition: 'Content simply transposed from being into shine',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'shn-3-inv-1',
      constraint: 'skepticism.foundationOfBeing = false',
      predicate: 'not(hasFoundation(skepticism, being))',
    },
    {
      id: 'shn-3-inv-2',
      constraint: 'idealism.thingInItself = false',
      predicate: 'not(entersCognition(thingInItself))',
    },
    {
      id: 'shn-3-inv-3',
      constraint: 'shine.content = transposedFrom(being)',
      predicate: 'equals(shine.content, transposedFrom(being))',
    },
  ],

  forces: [
    {
      id: 'shn-3-force-1',
      description: 'Transposed content drives task to show determinations are essence\'s own',
      type: 'reflection',
      trigger: 'content.transposed = true',
      effect: 'essenceDeterminations.reveal = true',
      targetState: 'shn-5',
    },
  ],

  transitions: [
    {
      id: 'shn-3-trans-1',
      from: 'shn-3',
      to: 'shn-5',
      mechanism: 'reflection',
      description: 'From manifold determinations to shine as essence\'s negativity',
    },
  ],

  nextStates: ['shn-5'],
  previousStates: ['shn-2'],

  provenance: {
    topicMapId: 'shn-3',
    lineRange: { start: 38, end: 93 },
    section: '1. Shine',
    order: 3,
  },

  description: 'Shine is \'phenomenon\' of skepticism, \'appearance\' of idealism. Both allow manifold of determinations, full richness of world. Content simply transposed from being into shine. Shine is immediately determined, can have this or that content. But whatever content, has not posited it but possesses it immediately.',
};

const state4: DialecticState = {
  id: 'shn-5',
  title: 'Shine as essence\'s negativity — immediacy of non-being',
  concept: 'ShineAsNegativity',
  phase: 'reflection',

  moments: [
    {
      name: 'immediacyOfNonBeing',
      definition: 'What constitutes shine is immediacy of non-being',
      type: 'negation',
    },
    {
      name: 'negativityOfEssence',
      definition: 'Non-being is negativity of essence within essence itself',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'immediacyOfNonBeing',
    },
    {
      name: 'absoluteInItself',
      definition: 'Immediacy which non-being contains is essence\'s own absolute in-itself',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'shn-5-inv-1',
      constraint: 'shine = immediacy(nonBeing)',
      predicate: 'equals(shine, immediacy(nonBeing))',
    },
    {
      id: 'shn-5-inv-2',
      constraint: 'nonBeing = negativityOfEssence',
      predicate: 'equals(nonBeing, negativityOfEssence)',
    },
    {
      id: 'shn-5-inv-3',
      constraint: 'essence.being = nonBeing',
      predicate: 'equals(essence.being, nonBeing)',
    },
    {
      id: 'shn-5-inv-4',
      constraint: 'immediacy(shine) = immediacy(essence)',
      predicate: 'equals(immediacy(shine), immediacy(essence))',
    },
  ],

  forces: [
    {
      id: 'shn-5-force-1',
      description: 'Negativity of essence drives toward infinite determinateness',
      type: 'reflection',
      trigger: 'negativity.internal = true',
      effect: 'infiniteDeterminateness.emerges = true',
      targetState: 'shn-9',
    },
  ],

  transitions: [
    {
      id: 'shn-5-trans-1',
      from: 'shn-5',
      to: 'shn-9',
      mechanism: 'reflection',
      description: 'From essence\'s negativity to infinite determinateness',
    },
  ],

  nextStates: ['shn-9'],
  previousStates: ['shn-3'],

  provenance: {
    topicMapId: 'shn-5',
    lineRange: { start: 107, end: 131 },
    section: '1. Shine',
    order: 4,
  },

  description: 'What constitutes shine is immediacy of non-being. This non-being is nothing else than negativity of essence within essence itself. In essence, being is non-being. Immediacy or indifference which non-being contains is essence\'s own absolute in-itself. Negativity of essence is its self-equality.',
};

const state5: DialecticState = {
  id: 'shn-9',
  title: 'Shine as infinite determinateness — essence',
  concept: 'InfiniteDeterminateness',
  phase: 'reflection',

  moments: [
    {
      name: 'infiniteDeterminateness',
      definition: 'Determinateness that shine is in essence is infinite determinateness',
      type: 'determination',
    },
    {
      name: 'selfSubsistence',
      definition: 'Determinateness that is self-subsistence and not determined',
      type: 'determination',
    },
    {
      name: 'shiningWithinItself',
      definition: 'Essence is shining of itself within itself',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'infiniteDeterminateness',
    },
  ],

  invariants: [
    {
      id: 'shn-9-inv-1',
      constraint: 'shine.determinateness = infinite',
      predicate: 'is(shine.determinateness, infinite)',
    },
    {
      id: 'shn-9-inv-2',
      constraint: 'negativity = immediacy',
      predicate: 'equals(negativity, immediacy)',
    },
    {
      id: 'shn-9-inv-3',
      constraint: 'shine = essenceInDeterminateness',
      predicate: 'equals(shine, essenceInDeterminateness)',
    },
    {
      id: 'shn-9-inv-4',
      constraint: 'essence = shiningOfItselfWithinItself',
      predicate: 'equals(essence, shiningOfItselfWithinItself)',
    },
  ],

  forces: [
    {
      id: 'shn-9-force-1',
      description: 'Shining within itself drives transition to Reflection',
      type: 'reflection',
      trigger: 'shining.internal = true',
      effect: 'reflection.emerges = true',
      targetState: 'ref-1',
    },
  ],

  transitions: [
    {
      id: 'shn-9-trans-1',
      from: 'shn-9',
      to: 'ref-1',
      mechanism: 'reflection',
      description: 'From shine as infinite determinateness to reflection',
    },
  ],

  nextStates: ['ref-1'],
  previousStates: ['shn-5'],

  provenance: {
    topicMapId: 'shn-9',
    lineRange: { start: 177, end: 190 },
    section: '1. Shine',
    order: 5,
  },

  description: 'Determinateness that shine is in essence is infinite determinateness. Only negative which coincides with itself. Negativity identical with immediacy, immediacy identical with negativity: essence. Shine is essence itself, but essence in determinateness. Essence is shining of itself within itself.',
};

export const shineIR: DialecticIR = {
  id: 'shine-ir',
  title: 'Shine IR: Being as Shine and Infinite Determinateness',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 1. Shine',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'shine.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'shn-1': 'reflection',
      'shn-2': 'reflection',
      'shn-3': 'reflection',
      'shn-5': 'reflection',
      'shn-9': 'reflection',
    },
  },
};

export const shineStates = {
  'shn-1': state1,
  'shn-2': state2,
  'shn-3': state3,
  'shn-5': state4,
  'shn-9': state5,
};
