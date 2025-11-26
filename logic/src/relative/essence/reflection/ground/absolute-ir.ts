/**
 * Absolute Ground IR: Dialectic Pseudo-Code for Absolute Ground
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - a. Absolute Ground
 *
 * Covers the dialectical movement:
 * - Form and essence (ground and grounded)
 * - Form as absolute negativity
 * - Form and matter (reciprocal presupposition)
 * - Form determines matter (self-mediation)
 * - Form and content (unity, ground-connection)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'abs-1',
  title: 'Form and essence — ground and grounded',
  concept: 'FormAndEssence',
  phase: 'reflection',

  moments: [
    {
      name: 'ground',
      definition: 'Essence determined through itself, as indeterminate or sublated positedness',
      type: 'determination',
    },
    {
      name: 'grounded',
      definition: 'Immediate that is not anything in and for itself, positedness as positedness',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'ground',
    },
    {
      name: 'twofoldDeterminateness',
      definition: 'Determinateness of essence as ground is twofold: ground and grounded',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'abs-1-inv-1',
      constraint: 'ground = essence(determinedThroughItself)',
      predicate: 'equals(ground, essence(determinedThroughItself))',
    },
    {
      id: 'abs-1-inv-2',
      constraint: 'grounded = positedness(asPositedness)',
      predicate: 'equals(grounded, positedness(asPositedness))',
    },
    {
      id: 'abs-1-inv-3',
      constraint: 'selfIdenticalNegative = selfIdenticalPositive',
      predicate: 'equals(selfIdenticalNegative, selfIdenticalPositive)',
    },
  ],

  forces: [
    {
      id: 'abs-1-force-1',
      description: 'Twofold determinateness drives toward form as absolute negativity',
      type: 'mediation',
      trigger: 'determinateness.twofold = true',
      effect: 'absoluteNegativity.emerges = true',
      targetState: 'abs-5',
    },
  ],

  transitions: [
    {
      id: 'abs-1-trans-1',
      from: 'abs-1',
      to: 'abs-5',
      mechanism: 'mediation',
      description: 'From ground/grounded to form as absolute negativity',
    },
  ],

  nextStates: ['abs-5'],
  previousStates: ['contradiction-ir'],

  provenance: {
    topicMapId: 'abs-1',
    lineRange: { start: 5, end: 43 },
    section: 'a. Absolute Ground',
    order: 1,
  },

  description: 'Determination of reflection returns into ground. Ground is essence determined through itself, as indeterminate or sublated positedness. Determinateness of essence as ground is twofold: ground and grounded. Self-identical negative and self-identical positive are one and same identity.',
};

const state2: DialecticState = {
  id: 'abs-5',
  title: 'Form as completed whole — absolute negativity',
  concept: 'FormAsAbsoluteNegativity',
  phase: 'reflection',

  moments: [
    {
      name: 'completedWhole',
      definition: 'Form is completed whole of reflection',
      type: 'determination',
    },
    {
      name: 'absoluteNegativity',
      definition: 'Form is absolute negativity itself or negative absolute self-identity',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'completedWhole',
    },
    {
      name: 'internalReflectiveShining',
      definition: 'Form is only internal reflective shining of essence',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'abs-5-inv-1',
      constraint: 'form = completedWhole(reflection)',
      predicate: 'equals(form, completedWhole(reflection))',
    },
    {
      id: 'abs-5-inv-2',
      constraint: 'form = absoluteNegativity',
      predicate: 'equals(form, absoluteNegativity)',
    },
    {
      id: 'abs-5-inv-3',
      constraint: 'form = internalReflectiveShining(essence)',
      predicate: 'equals(form, internalReflectiveShining(essence))',
    },
  ],

  forces: [
    {
      id: 'abs-5-force-1',
      description: 'Form determines essence as matter',
      type: 'mediation',
      trigger: 'form.determinesEssence = true',
      effect: 'matter.emerges = true',
      targetState: 'abs-7',
    },
  ],

  transitions: [
    {
      id: 'abs-5-trans-1',
      from: 'abs-5',
      to: 'abs-7',
      mechanism: 'mediation',
      description: 'From form as absolute negativity to form and matter',
    },
  ],

  nextStates: ['abs-7'],
  previousStates: ['abs-1'],

  provenance: {
    topicMapId: 'abs-5',
    lineRange: { start: 139, end: 189 },
    section: 'a. Absolute Ground',
    order: 2,
  },

  description: 'Form is completed whole of reflection. Form is absolute negativity itself or negative absolute self-identity. Form is only internal reflective shining of essence. Form is itself ground of its sublating. Form is contradiction of being sublated in positedness and yet having subsistence in sublatedness.',
};

const state3: DialecticState = {
  id: 'abs-7',
  title: 'Form and matter — essence becomes matter',
  concept: 'FormAndMatter',
  phase: 'reflection',

  moments: [
    {
      name: 'matter',
      definition: 'Essence determined as formless identity, simple identity void of distinction',
      type: 'determination',
    },
    {
      name: 'reciprocalPresupposition',
      definition: 'Form and matter presuppose each other',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'matter',
    },
    {
      name: 'passiveActive',
      definition: 'Matter is passive, form is active',
      type: 'polarity',
    },
  ],

  invariants: [
    {
      id: 'abs-7-inv-1',
      constraint: 'matter = formlessIdentity',
      predicate: 'equals(matter, formlessIdentity)',
    },
    {
      id: 'abs-7-inv-2',
      constraint: 'form.presupposes = matter',
      predicate: 'presupposes(form, matter)',
    },
    {
      id: 'abs-7-inv-3',
      constraint: 'matter.presupposes = form',
      predicate: 'presupposes(matter, form)',
    },
  ],

  forces: [
    {
      id: 'abs-7-force-1',
      description: 'Form determines matter through self-mediation',
      type: 'mediation',
      trigger: 'reciprocalPresupposition.established = true',
      effect: 'selfMediation.emerges = true',
      targetState: 'abs-9',
    },
  ],

  transitions: [
    {
      id: 'abs-7-trans-1',
      from: 'abs-7',
      to: 'abs-9',
      mechanism: 'mediation',
      description: 'From form/matter presupposition to self-mediation',
    },
  ],

  nextStates: ['abs-9'],
  previousStates: ['abs-5'],

  provenance: {
    topicMapId: 'abs-7',
    lineRange: { start: 206, end: 228 },
    section: 'a. Absolute Ground',
    order: 3,
  },

  description: 'Essence becomes matter in that its reflection relates itself to essence as to formless indeterminate. Matter is simple identity, void of distinction, other of form. Form and matter presuppose each other. Matter is passive, form is active. Matter must be informed, form must materialize itself.',
};

const state4: DialecticState = {
  id: 'abs-9',
  title: 'Form determines matter — self-mediation',
  concept: 'SelfMediation',
  phase: 'reflection',

  moments: [
    {
      name: 'selfMediation',
      definition: 'Self-mediation of each through its own non-being',
      type: 'mediation',
    },
    {
      name: 'sublatingOfSemblance',
      definition: 'Sublating of semblance of their indifference and distinctness',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'selfMediation',
    },
    {
      name: 'restorationOfIdentity',
      definition: 'Restoration of original identity is inner recollection of exteriorization',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'abs-9-inv-1',
      constraint: 'form.contains = matter',
      predicate: 'contains(form, matter)',
    },
    {
      id: 'abs-9-inv-2',
      constraint: 'matter.contains = form',
      predicate: 'contains(matter, form)',
    },
    {
      id: 'abs-9-inv-3',
      constraint: 'twoMediations = oneMovement',
      predicate: 'equals(twoMediations, oneMovement)',
    },
  ],

  forces: [
    {
      id: 'abs-9-force-1',
      description: 'Self-mediation restores unity as content',
      type: 'mediation',
      trigger: 'selfMediation.complete = true',
      effect: 'content.emerges = true',
      targetState: 'abs-13',
    },
  ],

  transitions: [
    {
      id: 'abs-9-trans-1',
      from: 'abs-9',
      to: 'abs-13',
      mechanism: 'mediation',
      description: 'From self-mediation to form and content',
    },
  ],

  nextStates: ['abs-13'],
  previousStates: ['abs-7'],

  provenance: {
    topicMapId: 'abs-9',
    lineRange: { start: 298, end: 311 },
    section: 'a. Absolute Ground',
    order: 4,
  },

  description: 'Form determines matter, matter is determined by form. Activity of form on matter is only sublating of semblance of their indifference and distinctness. Self-mediation of each through its own non-being. Two mediations are one movement. Restoration of original identity is inner recollection of exteriorization.',
};

const state5: DialecticState = {
  id: 'abs-13',
  title: 'Original unity restored — content',
  concept: 'Content',
  phase: 'reflection',

  moments: [
    {
      name: 'content',
      definition: 'Informed matter or form that possesses subsistence, unity of form and matter',
      type: 'determination',
    },
    {
      name: 'groundConnection',
      definition: 'Content has ground-connection as its essential form',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'content',
    },
    {
      name: 'determinateGround',
      definition: 'Ground has made itself into determinate ground',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'abs-13-inv-1',
      constraint: 'content = unity(form, matter)',
      predicate: 'equals(content, unity(form, matter))',
    },
    {
      id: 'abs-13-inv-2',
      constraint: 'activityOfForm = movementOfMatter',
      predicate: 'equals(activityOfForm, movementOfMatter)',
    },
    {
      id: 'abs-13-inv-3',
      constraint: 'unity = ground',
      predicate: 'equals(unity, ground)',
    },
  ],

  forces: [
    {
      id: 'abs-13-force-1',
      description: 'Content as ground drives toward determinate ground',
      type: 'passover',
      trigger: 'content.established = true',
      effect: 'determinateGround.emerges = true',
      targetState: 'det-1',
    },
  ],

  transitions: [
    {
      id: 'abs-13-trans-1',
      from: 'abs-13',
      to: 'det-1',
      mechanism: 'passover',
      description: 'From content to determinate ground',
    },
  ],

  nextStates: ['det-1'],
  previousStates: ['abs-9'],

  provenance: {
    topicMapId: 'abs-13',
    lineRange: { start: 395, end: 457 },
    section: 'a. Absolute Ground',
    order: 5,
  },

  description: 'Original unity restored. Activity of form and movement of matter are one and same thing. Neither finite matter nor finite form have truth, only their unity is truth. Unity proves to be their ground. Informed matter or form that possesses subsistence is content.',
};

export const absoluteIR: DialecticIR = {
  id: 'absolute-ir',
  title: 'Absolute Ground IR: Form, Matter, Content',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - a. Absolute Ground',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'absolute.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'abs-1': 'reflection',
      'abs-5': 'reflection',
      'abs-7': 'reflection',
      'abs-9': 'reflection',
      'abs-13': 'reflection',
    },
  },
};

export const absoluteStates = {
  'abs-1': state1,
  'abs-5': state2,
  'abs-7': state3,
  'abs-9': state4,
  'abs-13': state5,
};
