/**
 * Reflection IR: Dialectic Pseudo-Code for Reflection
 *
 * Architecture: CPU (Reflection / Essence)
 * Section: A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. Reflection
 *
 * Covers the dialectical movement:
 * - Essence as reflection (movement from nothing to nothing)
 * - Positing reflection (positing and presupposing)
 * - External reflection (doubled determination)
 * - Determining reflection (unity of positing and external)
 * - Determination of reflection (positedness and immanent reflection)
 */

import type {
  DialecticState,
  DialecticIR,
} from '@schema/dialectic';

const state1: DialecticState = {
  id: 'ref-2',
  title: 'Essence as reflection — movement from nothing to nothing',
  concept: 'EssenceAsReflection',
  phase: 'reflection',

  moments: [
    {
      name: 'reflection',
      definition: 'Movement of becoming that remains within itself',
      type: 'process',
    },
    {
      name: 'negationInItself',
      definition: 'Distinguished determined as negative in itself, as shine',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'reflection',
    },
    {
      name: 'movementFromNothingToNothing',
      definition: 'Movement from nothing to nothing, back to itself',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'ref-2-inv-1',
      constraint: 'essence = reflection',
      predicate: 'is(essence, reflection)',
    },
    {
      id: 'ref-2-inv-2',
      constraint: 'movement.remainsWithinItself = true',
      predicate: 'remainsWithinItself(movement)',
    },
    {
      id: 'ref-2-inv-3',
      constraint: 'immediacy = movementItself',
      predicate: 'equals(immediacy, movementItself)',
    },
    {
      id: 'ref-2-inv-4',
      constraint: 'being = movement(nothingness, nothingness)',
      predicate: 'equals(being, movement(nothingness, nothingness))',
    },
  ],

  forces: [
    {
      id: 'ref-2-force-1',
      description: 'Pure absolute reflection determines itself as positing reflection',
      type: 'reflection',
      trigger: 'reflection.absolute = true',
      effect: 'positingReflection.emerges = true',
      targetState: 'ref-7',
    },
  ],

  transitions: [
    {
      id: 'ref-2-trans-1',
      from: 'ref-2',
      to: 'ref-7',
      mechanism: 'reflection',
      description: 'From absolute reflection to positing reflection',
    },
  ],

  nextStates: ['ref-7'],
  previousStates: ['shn-9'],

  provenance: {
    topicMapId: 'ref-2',
    lineRange: { start: 10, end: 54 },
    section: '2. Reflection',
    order: 1,
  },

  description: 'Essence is reflection, movement of becoming that remains within itself. Distinguished determined as negative in itself, as shine. Reflective movement: other as negation in itself, self-referring. Movement from nothing to nothing, back to itself. Being only as movement of nothingness to nothingness.',
};

const state2: DialecticState = {
  id: 'ref-7',
  title: 'Positing reflection — positing and presupposing',
  concept: 'PositingReflection',
  phase: 'reflection',

  moments: [
    {
      name: 'positing',
      definition: 'Immediacy as turning back, no other beforehand',
      type: 'mediation',
    },
    {
      name: 'presupposing',
      definition: 'Determines turning back as negative of itself (presupposition)',
      type: 'mediation',
      relation: 'opposite',
      relatedTo: 'positing',
    },
    {
      name: 'selfRepulsion',
      definition: 'Self-repulsion, presupposing of that from which reflection turns back',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'ref-7-inv-1',
      constraint: 'positing = immediacy(turningBack)',
      predicate: 'equals(positing, immediacy(turningBack))',
    },
    {
      id: 'ref-7-inv-2',
      constraint: 'sublating(negative) = sublating(immediacy)',
      predicate: 'equals(sublating(negative), sublating(immediacy))',
    },
    {
      id: 'ref-7-inv-3',
      constraint: 'presupposing = negation(negative)',
      predicate: 'equals(presupposing, negation(negative))',
    },
    {
      id: 'ref-7-inv-4',
      constraint: 'positing ↔ presupposing',
      predicate: 'reciprocal(positing, presupposing)',
    },
  ],

  forces: [
    {
      id: 'ref-7-force-1',
      description: 'Presupposing drives transition to external reflection',
      type: 'reflection',
      trigger: 'presupposing.determinesImmediate = true',
      effect: 'externalReflection.emerges = true',
      targetState: 'ref-10',
    },
  ],

  transitions: [
    {
      id: 'ref-7-trans-1',
      from: 'ref-7',
      to: 'ref-10',
      mechanism: 'reflection',
      description: 'From positing reflection to external reflection',
    },
  ],

  nextStates: ['ref-10'],
  previousStates: ['ref-2'],

  provenance: {
    topicMapId: 'ref-7',
    lineRange: { start: 128, end: 165 },
    section: '2. Reflection',
    order: 2,
  },

  description: 'Positing: immediacy as turning back, no other beforehand. Only as turning back, negative of itself. Immediacy is sublated negation and sublated return. Equally negation of negative as negative: presupposing. Presupposing: determines turning back as negative of itself. Self-repulsion, presupposing of that from which reflection turns back.',
};

const state3: DialecticState = {
  id: 'ref-10',
  title: 'External reflection — doubled determination',
  concept: 'ExternalReflection',
  phase: 'reflection',

  moments: [
    {
      name: 'externalReflection',
      definition: 'Presupposes itself as sublated, negative of itself',
      type: 'mediation',
    },
    {
      name: 'doubledDetermination',
      definition: 'Presupposed (immediate) and reflection negatively referring to itself',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'externalReflection',
    },
    {
      name: 'syllogismStructure',
      definition: 'Two extremes: immediate and reflection into itself; middle term connects',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'ref-10-inv-1',
      constraint: 'externalReflection.presupposes = selfSublated',
      predicate: 'presupposes(externalReflection, selfSublated)',
    },
    {
      id: 'ref-10-inv-2',
      constraint: 'determination = doubled',
      predicate: 'is(determination, doubled)',
    },
    {
      id: 'ref-10-inv-3',
      constraint: 'structure = syllogism',
      predicate: 'is(structure, syllogism)',
    },
    {
      id: 'ref-10-inv-4',
      constraint: 'middleTerm = determinateImmediate',
      predicate: 'equals(middleTerm, determinateImmediate)',
    },
  ],

  forces: [
    {
      id: 'ref-10-force-1',
      description: 'Unity of positing and external reflection drives toward determining reflection',
      type: 'reflection',
      trigger: 'reflection.unifies = true',
      effect: 'determiningReflection.emerges = true',
      targetState: 'ref-14',
    },
  ],

  transitions: [
    {
      id: 'ref-10-trans-1',
      from: 'ref-10',
      to: 'ref-14',
      mechanism: 'reflection',
      description: 'From external reflection to determining reflection',
    },
  ],

  nextStates: ['ref-14'],
  previousStates: ['ref-7'],

  provenance: {
    topicMapId: 'ref-10',
    lineRange: { start: 232, end: 247 },
    section: '2. Reflection',
    order: 3,
  },

  description: 'Absolute reflection posits only shine, positedness, for presupposition. External reflection presupposes itself as sublated, negative of itself. Doubled: presupposed (reflection into itself as immediate) and reflection negatively referring to itself. External reflection is syllogism.',
};

const state4: DialecticState = {
  id: 'ref-14',
  title: 'Determining reflection — unity of positing and external',
  concept: 'DeterminingReflection',
  phase: 'reflection',

  moments: [
    {
      name: 'determiningReflection',
      definition: 'Unity of positing and external reflection',
      type: 'mediation',
    },
    {
      name: 'positedness',
      definition: 'Positedness is existence, but ground is essence as pure negativity',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'determiningReflection',
    },
    {
      name: 'absoluteConnection',
      definition: 'Connection with turning back into itself is absolute',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'ref-14-inv-1',
      constraint: 'determiningReflection = unity(positing, external)',
      predicate: 'equals(determiningReflection, unity(positing, external))',
    },
    {
      id: 'ref-14-inv-2',
      constraint: 'positedness = existence(essence)',
      predicate: 'equals(positedness, existence(essence))',
    },
    {
      id: 'ref-14-inv-3',
      constraint: 'ground = essence(pureNegativity)',
      predicate: 'equals(ground, essence(pureNegativity))',
    },
    {
      id: 'ref-14-inv-4',
      constraint: 'positedness.superiorTo = existence',
      predicate: 'superiorTo(positedness, existence)',
    },
  ],

  forces: [
    {
      id: 'ref-14-force-1',
      description: 'Determining reflection establishes determination of reflection',
      type: 'reflection',
      trigger: 'determining.complete = true',
      effect: 'determinationOfReflection.emerges = true',
      targetState: 'ref-19',
    },
  ],

  transitions: [
    {
      id: 'ref-14-trans-1',
      from: 'ref-14',
      to: 'ref-19',
      mechanism: 'reflection',
      description: 'From determining reflection to determination of reflection',
    },
  ],

  nextStates: ['ref-19'],
  previousStates: ['ref-10'],

  provenance: {
    topicMapId: 'ref-14',
    lineRange: { start: 317, end: 334 },
    section: '2. Reflection',
    order: 4,
  },

  description: 'Determining reflection is unity of positing and external reflection. External reflection begins from immediate being, positing from nothing. Positedness is existence, but ground is essence as pure negativity. Positedness conjoins existence with essence.',
};

const state5: DialecticState = {
  id: 'ref-19',
  title: 'Determining reflection — nature as both positedness and immanent reflection',
  concept: 'DeterminationOfReflection',
  phase: 'reflection',

  moments: [
    {
      name: 'determinationOfReflection',
      definition: 'Both immanently reflected reference and positedness',
      type: 'determination',
    },
    {
      name: 'immanentReflectedness',
      definition: 'Subsisting, takes otherness back into itself',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'determinationOfReflection',
    },
    {
      name: 'infiniteReference',
      definition: 'Infinite reference to itself',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'ref-19-inv-1',
      constraint: 'determination = positedness ∧ immanentReflection',
      predicate: 'and(is(determination, positedness), is(determination, immanentReflection))',
    },
    {
      id: 'ref-19-inv-2',
      constraint: 'positedness = sublatedness(determination)',
      predicate: 'equals(positedness, sublatedness(determination))',
    },
    {
      id: 'ref-19-inv-3',
      constraint: 'immanentReflectedness = subsisting',
      predicate: 'equals(immanentReflectedness, subsisting)',
    },
    {
      id: 'ref-19-inv-4',
      constraint: 'reference = infinite',
      predicate: 'is(reference, infinite)',
    },
  ],

  forces: [
    {
      id: 'ref-19-force-1',
      description: 'Infinite reference to itself completes Reflection phase',
      type: 'reflection',
      trigger: 'reference.infinite = true',
      effect: 'reflectionPhaseComplete = true',
      targetState: 'identity-1',
    },
  ],

  transitions: [
    {
      id: 'ref-19-trans-1',
      from: 'ref-19',
      to: 'identity-1',
      mechanism: 'reflection',
      description: 'From determination of reflection to identity (Foundation)',
    },
  ],

  nextStates: ['identity-1'],
  previousStates: ['ref-14'],

  provenance: {
    topicMapId: 'ref-19',
    lineRange: { start: 443, end: 482 },
    section: '2. Reflection',
    order: 5,
  },

  description: 'Determination of reflection is both immanently reflected reference and positedness. Positedness is sublatedness of determination; immanent reflectedness is subsisting. Determination is determinate side and reference to its negation. Positedness, negation deflected into itself, unity of itself and its other. Infinite reference to itself.',
};

export const reflectionIR: DialecticIR = {
  id: 'reflection-ir',
  title: 'Reflection IR: Essence as Reflection, Positing, External, Determining',
  section: 'A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. Reflection',
  states: [state1, state2, state3, state4, state5],
  metadata: {
    sourceFile: 'reflection.txt',
    totalStates: 5,
    cpuGpuMapping: {
      'ref-2': 'reflection',
      'ref-7': 'reflection',
      'ref-10': 'reflection',
      'ref-14': 'reflection',
      'ref-19': 'reflection',
    },
  },
};

export const reflectionStates = {
  'ref-2': state1,
  'ref-7': state2,
  'ref-10': state3,
  'ref-14': state4,
  'ref-19': state5,
};
