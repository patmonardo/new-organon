/**
 * Constitution IR: Dialectic Pseudo-Code for Determination, Constitution, and Limit
 *
 * This file demonstrates the conversion from TopicMap + Chunks → Dialectic IRP
 * as executable pseudo-code that advances the dialectic.
 *
 * Architecture: CPU (Quality/Being → Reflection → Subject)
 * Section: B. FINITUDE (b) Determination, constitution, and limit
 *
 * This IR restricts the source text to what immediately advances the dialectic,
 * describing the State and NextState of the dialectic movement.
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';

/**
 * State 1: Introduction - In-itself mediated through being-for-other
 *
 * Dialectic Position: The in-itself is no longer abstract but mediated.
 * This is the transition from abstract in-itself to determinateness existent-in-itself.
 */
const state1: DialecticState = {
  id: 'constitution-1',
  title: 'Introduction: in-itself mediated through being-for-other; determinateness existent-in-itself',
  concept: 'InItselfMediated',
  phase: 'quality', // CPU: Quality/Being

  moments: [
    {
      name: 'inItself',
      definition: 'The something reflected into itself from its being-for-other',
      type: 'determination',
      relation: 'mediates',
      relatedTo: 'beingForOther',
    },
    {
      name: 'beingForOther',
      definition: 'The moment through which in-itself is mediated',
      type: 'moment',
      relation: 'mediates',
      relatedTo: 'inItself',
    },
    {
      name: 'determinatenessExistentInItself',
      definition: 'Determinateness that is immanently reflected',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-1-1',
      constraint: 'inItself.mediated = true',
      predicate: 'mediated(inItself, beingForOther)',
    },
    {
      id: 'inv-1-2',
      constraint: 'inItself.abstract = false',
      predicate: 'not(abstract(inItself))',
    },
    {
      id: 'inv-1-3',
      constraint: 'something.hasPresent = whatIsInItself',
      predicate: 'hasPresent(something, whatIsInItself)',
    },
    {
      id: 'inv-1-4',
      constraint: 'inItself.affectedWith = {negation, beingForOther}',
      predicate: 'affectedWith(inItself, negation) ∧ affectedWith(inItself, beingForOther)',
    },
  ],

  forces: [
    {
      id: 'force-1-1',
      description: 'Mediation through being-for-other drives in-itself beyond abstract',
      type: 'mediation',
      trigger: 'inItself.abstract = true',
      effect: 'inItself.mediated = true',
      targetState: 'constitution-2',
    },
  ],

  transitions: [
    {
      id: 'trans-1-1',
      from: 'constitution-1',
      to: 'constitution-2',
      mechanism: 'mediation',
      middleTerm: 'beingForOther',
      description: 'Mediated in-itself becomes determination',
    },
  ],

  nextStates: ['constitution-2'],
  previousStates: [],

  provenance: {
    topicMapId: 'constitution-1',
    lineRange: { start: 4, end: 25 },
    section: 'b. Determination, constitution, and limit',
    order: 1,
  },

  description: 'The in-itself, in which the something is reflected into itself from its being-for-other, no longer is an abstract in-itself but, as the negation of its being-for-other, is mediated through this latter, which is thus its moment.',
  keyPoints: [
    'In-itself no longer abstract but mediated through being-for-other',
    'Being-for-other is its moment',
    'Identity by virtue of which something has present in it what it is in itself',
    'Determinateness existent-in-itself',
  ],
};

/**
 * State 2: Determination I - Definition
 *
 * Dialectic Position: Determination as affirmative determinateness.
 * The in-itself by which something abides in existence.
 */
const state2: DialecticState = {
  id: 'constitution-2',
  title: 'Determination I: definition (affirmative determinateness; in-itself by which something abides)',
  concept: 'Determination',
  phase: 'quality', // CPU: Quality/Being

  moments: [
    {
      name: 'determination',
      definition: 'Affirmative determinateness; in-itself by which something abides',
      type: 'determination',
    },
    {
      name: 'beingInIt',
      definition: 'The something\'s other moment, in unity with in-itself',
      type: 'moment',
      relation: 'unified',
      relatedTo: 'determination',
    },
    {
      name: 'filling',
      definition: 'Further determinateness that accrues to something',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-2-1',
      constraint: 'determination = affirmativeDeterminateness',
      predicate: 'is(determination, affirmativeDeterminateness)',
    },
    {
      id: 'inv-2-2',
      constraint: 'something.abides = true when involvedWith(other)',
      predicate: 'abides(something, other) → determination(something)',
    },
    {
      id: 'inv-2-3',
      constraint: 'something.preservesSelfEquality = true',
      predicate: 'preservesSelfEquality(something)',
    },
    {
      id: 'inv-2-4',
      constraint: 'whatIsInItself ⊆ existence',
      predicate: 'subset(whatIsInItself, existence)',
    },
  ],

  forces: [
    {
      id: 'force-2-1',
      description: 'External determinateness introduces otherness',
      type: 'externality',
      trigger: 'externalDeterminateness.enters',
      effect: 'determination.reducedTo = constitution',
      targetState: 'constitution-4',
    },
  ],

  transitions: [
    {
      id: 'trans-2-1',
      from: 'constitution-2',
      to: 'constitution-3',
      mechanism: 'passover',
      description: 'Determination exemplified (human being)',
    },
    {
      id: 'trans-2-2',
      from: 'constitution-2',
      to: 'constitution-4',
      mechanism: 'passover',
      middleTerm: 'determinateness',
      description: 'Determination passes over into constitution via determinateness',
    },
  ],

  nextStates: ['constitution-3', 'constitution-4'],
  previousStates: ['constitution-1'],

  provenance: {
    topicMapId: 'constitution-2',
    lineRange: { start: 27, end: 52 },
    section: '1. Determination',
    order: 2,
  },

  description: 'The quality which in the simple something is an in-itself essentially in unity with the something\'s other moment, its being-in-it, can be named its determination.',
  keyPoints: [
    'Determination = affirmative determinateness',
    'In-itself by which something abides in existence while involved with other',
    'Preserves itself in self-equality',
    'What something is in itself also present in it',
  ],
};

/**
 * State 3: Determination II - Human example
 *
 * Dialectic Position: Rational thought as determination of the human being,
 * but still only in-itself as an ought facing sensuous existence.
 */
const state3: DialecticState = {
  id: 'constitution-3',
  title: 'Determination II: example (human being; rational thought; still only in itself as ought)',
  concept: 'DeterminationExample',
  phase: 'quality',

  moments: [
    {
      name: 'rationalThought',
      definition: 'Simple determinateness distinguishing human from brute',
      type: 'determination',
    },
    {
      name: 'sensuousExistence',
      definition: 'Immediate natural being still confronting the determination',
      type: 'quality',
      relation: 'opposite',
      relatedTo: 'rationalThought',
    },
    {
      name: 'ought',
      definition: 'Determination still only in-itself, not yet actualized',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'rationalThought',
    },
  ],

  invariants: [
    {
      id: 'inv-3-1',
      constraint: 'humanBeing.determination = rationalThought',
      predicate: 'determination(humanBeing, rationalThought)',
    },
    {
      id: 'inv-3-2',
      constraint: 'rationalThought ∈ humanBeing.existence',
      predicate: 'in(rationalThought, humanExistence)',
    },
    {
      id: 'inv-3-3',
      constraint: 'rationalThought.status = ought',
      predicate: 'status(rationalThought, ought)',
    },
    {
      id: 'inv-3-4',
      constraint: 'sensuousExistence ⟂ rationalThought',
      predicate: 'opposes(sensuousExistence, rationalThought)',
    },
  ],

  forces: [
    {
      id: 'force-3-1',
      description: 'Failure to embody rational thought forces appeal to external constitution',
      type: 'externality',
      trigger: 'rationalThought.notEmbodying = true',
      effect: 'determination.turnsToward = constitution',
      targetState: 'constitution-4',
    },
  ],

  transitions: [
    {
      id: 'trans-3-1',
      from: 'constitution-3',
      to: 'constitution-4',
      mechanism: 'negation',
      description: 'Abstract ought exposes need for constitution as external determinateness',
    },
  ],

  nextStates: ['constitution-4'],
  previousStates: ['constitution-2'],

  provenance: {
    topicMapId: 'constitution-3',
    lineRange: { start: 54, end: 78 },
    section: '1. Determination',
    order: 3,
  },

  description: 'Rational thought is the determination of the human being, yet it still stands as an ought against immediate sensuous existence.',
  keyPoints: [
    'Human being\'s determination: rational thought (vocation)',
    'Thinking exists concretely yet remains in-itself as ought',
    'Thinking opposes immediate sensuous nature',
    'Determination still abstract relative to existence',
  ],
};

/**
 * State 4: Constitution I - External existence
 *
 * Dialectic Position: Constitution as external existence, not belonging to being-in-itself.
 * This is the moment where determinateness separates itself.
 */
const state4: DialecticState = {
  id: 'constitution-4',
  title: 'Constitution I: external existence (not belonging to being-in-itself)',
  concept: 'Constitution',
  phase: 'quality', // CPU: Quality/Being

  moments: [
    {
      name: 'constitution',
      definition: 'Determinateness as external existence, not belonging to being-in-itself',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'determination',
    },
    {
      name: 'externalExistence',
      definition: 'What something has in it, separated from being-in-itself',
      type: 'determination',
    },
    {
      name: 'beingInItself',
      definition: 'The something\'s inner being, distinct from constitution',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'constitution',
    },
  ],

  invariants: [
    {
      id: 'inv-4-1',
      constraint: 'constitution = determinateness',
      predicate: 'is(constitution, determinateness)',
    },
    {
      id: 'inv-4-2',
      constraint: 'constitution.belongsTo = externalExistence',
      predicate: 'belongsTo(constitution, externalExistence)',
    },
    {
      id: 'inv-4-3',
      constraint: 'constitution.belongsTo ≠ beingInItself',
      predicate: 'not(belongsTo(constitution, beingInItself))',
    },
    {
      id: 'inv-4-4',
      constraint: 'whatSomethingHasInIt.separates = true',
      predicate: 'separates(whatSomethingHasInIt)',
    },
  ],

  forces: [
    {
      id: 'force-4-1',
      description: 'Constitution caught up in external influences',
      type: 'externality',
      trigger: 'externalInfluences.enter',
      effect: 'constitution.affected = true',
      targetState: 'constitution-5',
    },
  ],

  transitions: [
    {
      id: 'trans-4-1',
      from: 'constitution-4',
      to: 'constitution-5',
      mechanism: 'passover',
      description: 'Constitution affected by external influences',
    },
    {
      id: 'trans-4-2',
      from: 'constitution-4',
      to: 'constitution-6',
      mechanism: 'mediation',
      middleTerm: 'determinateness',
      description: 'Constitution and determination connected via determinateness',
    },
  ],

  nextStates: ['constitution-5', 'constitution-6'],
  previousStates: ['constitution-2', 'constitution-3'],

  provenance: {
    topicMapId: 'constitution-4',
    lineRange: { start: 80, end: 91 },
    section: '2. Constitution',
    order: 4,
  },

  description: 'The filling of the being-in-itself with determinateness is also distinct from the determinateness which is only being-for-other and remains outside the determination.',
  keyPoints: [
    'Determinateness = constitution',
    'External existence of something, not belonging to being-in-itself',
    'What something has in it separates itself',
  ],
};

/**
 * State 5: Constitution II - External influences; alteration
 *
 * Dialectic Position: Constitution is the side exposed to external influences;
 * alteration falls on constitution while determination preserves itself.
 */
const state5: DialecticState = {
  id: 'constitution-5',
  title: 'Constitution II: external influences; alteration falls on constitution',
  concept: 'ConstitutionExternality',
  phase: 'quality',

  moments: [
    {
      name: 'constitution',
      definition: 'That in something which becomes an other',
      type: 'determination',
    },
    {
      name: 'externalInfluence',
      definition: 'Accidental relations that seize the constitution',
      type: 'externality',
      relation: 'mediates',
      relatedTo: 'constitution',
    },
    {
      name: 'determination',
      definition: 'Self-preserving identity indifferent to alteration',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'constitution',
    },
  ],

  invariants: [
    {
      id: 'inv-5-1',
      constraint: 'alteration.targets = constitution',
      predicate: 'targets(alteration, constitution)',
    },
    {
      id: 'inv-5-2',
      constraint: 'determination.preserveSelf = true',
      predicate: 'preservesSelf(determination)',
    },
    {
      id: 'inv-5-3',
      constraint: 'constitution.quality = beingGivenOverToExternality',
      predicate: 'quality(constitution, externality)',
    },
  ],

  forces: [
    {
      id: 'force-5-1',
      description: 'Accumulated alteration reveals need for mediation between determination and constitution',
      type: 'contradiction',
      trigger: 'alteration.intensifies',
      effect: 'seekMiddleTerm = determinateness',
      targetState: 'constitution-6',
    },
  ],

  transitions: [
    {
      id: 'trans-5-1',
      from: 'constitution-5',
      to: 'constitution-6',
      mechanism: 'mediation',
      middleTerm: 'determinateness',
      description: 'External alteration drives search for internal connection',
    },
  ],

  nextStates: ['constitution-6'],
  previousStates: ['constitution-4'],

  provenance: {
    topicMapId: 'constitution-5',
    lineRange: { start: 93, end: 108 },
    section: '2. Constitution',
    order: 5,
  },

  description: 'Constitution is the side of a something that is caught up in external influences; alteration falls on this side while determination remains stable.',
  keyPoints: [
    'Constitution depends on external influences and relationships',
    'Quality of something to be given over to externality',
    'Alteration affects constitution, not determination',
    'Constitution is the becoming-other in the something',
  ],
};

/**
 * State 6: Determination and Constitution - Pass over into each other
 *
 * Dialectic Position: The middle term (determinateness) connects determination and constitution.
 * They pass over into each other through this mediation.
 */
const state6: DialecticState = {
  id: 'constitution-6',
  title: 'Determination and constitution: distinct but connected; pass over into each other',
  concept: 'DeterminationConstitutionMediation',
  phase: 'quality', // CPU: Quality/Being

  moments: [
    {
      name: 'determination',
      definition: 'Affirmative determinateness; in-itself',
      type: 'determination',
      relation: 'passesOver',
      relatedTo: 'constitution',
    },
    {
      name: 'constitution',
      definition: 'External existence; being-for-other',
      type: 'determination',
      relation: 'passesOver',
      relatedTo: 'determination',
    },
    {
      name: 'middleTerm',
      definition: 'Determinateness as such; identity belongs to both',
      type: 'mediation',
      relation: 'mediates',
      relatedTo: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-6-1',
      constraint: 'determination ≠ constitution',
      predicate: 'not(equals(determination, constitution))',
    },
    {
      id: 'inv-6-2',
      constraint: 'middleTerm = determinateness',
      predicate: 'is(middleTerm, determinateness)',
    },
    {
      id: 'inv-6-3',
      constraint: 'middleTerm.identity ∈ {determination, constitution}',
      predicate: 'belongsTo(middleTerm.identity, determination) ∧ belongsTo(middleTerm.identity, constitution)',
    },
    {
      id: 'inv-6-4',
      constraint: 'determination.passesOver = constitution',
      predicate: 'passesOver(determination, constitution)',
    },
    {
      id: 'inv-6-5',
      constraint: 'constitution.passesOver = determination',
      predicate: 'passesOver(constitution, determination)',
    },
  ],

  forces: [
    {
      id: 'force-6-1',
      description: 'Otherness introduced in determination reduces it to constitution',
      type: 'negation',
      trigger: 'otherness.introducedIn(determination)',
      effect: 'determination.reducedTo = constitution',
      targetState: 'constitution-7',
    },
  ],

  transitions: [
    {
      id: 'trans-6-1',
      from: 'constitution-6',
      to: 'constitution-7',
      mechanism: 'passover',
      middleTerm: 'determinateness',
      description: 'Constitution passes over into determination',
    },
    {
      id: 'trans-6-2',
      from: 'constitution-6',
      to: 'constitution-8',
      mechanism: 'negation',
      description: 'Alteration posited; negation immanent',
    },
  ],

  nextStates: ['constitution-7', 'constitution-8'],
  previousStates: ['constitution-4', 'constitution-5'],

  provenance: {
    topicMapId: 'constitution-6',
    lineRange: { start: 110, end: 137 },
    section: '2. Constitution',
    order: 6,
  },

  description: 'Determination and constitution are thus distinct from each other; something, according to its determination, is indifferent to its constitution. But that which the something has in it is the middle term of this syllogism connecting the two.',
  keyPoints: [
    'What something has in it = middle term connecting determination and constitution',
    'Determination passes over into constitution; constitution into determination',
    'Determinateness holds other in itself',
    'Introduces otherness in determination → reduced to constitution',
  ],
};

/**
 * State 7: Constitution depends on determination
 *
 * Dialectic Position: Constitution, even when isolated as being-for-other,
 * shows itself as self-referring determination; external determining is conditioned by the in-itself.
 */
const state7: DialecticState = {
  id: 'constitution-7',
  title: 'Constitution passes over into determination; constitution depends on determination',
  concept: 'ConstitutionAsDetermination',
  phase: 'quality',

  moments: [
    {
      name: 'constitution',
      definition: 'Being-for-other isolated yet identical with the other in it',
      type: 'determination',
    },
    {
      name: 'inItselfDetermination',
      definition: 'Self-referring existence with determinateness',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'constitution',
    },
    {
      name: 'externalDetermining',
      definition: 'Determination that appears to come from outside',
      type: 'externality',
      relation: 'mediates',
      relatedTo: 'constitution',
    },
  ],

  invariants: [
    {
      id: 'inv-7-1',
      constraint: 'constitution.dependsOn = inItselfDetermination',
      predicate: 'dependsOn(constitution, inItselfDetermination)',
    },
    {
      id: 'inv-7-2',
      constraint: 'externalDetermining.conditionedBy = inItselfDetermination',
      predicate: 'conditionedBy(externalDetermining, inItselfDetermination)',
    },
    {
      id: 'inv-7-3',
      constraint: 'constitution ∈ whatSomethingIsInItself',
      predicate: 'belongsTo(constitution, whatSomethingIsInItself)',
    },
  ],

  forces: [
    {
      id: 'force-7-1',
      description: 'Recognition of immanent negation pushes constitution toward explicit self-negation',
      type: 'negation',
      trigger: 'constitution.altersAlongWithItself',
      effect: 'negation.immanent = true',
      targetState: 'constitution-8',
    },
  ],

  transitions: [
    {
      id: 'trans-7-1',
      from: 'constitution-7',
      to: 'constitution-8',
      mechanism: 'negation',
      description: 'Constitution internalizes alteration as immanent negation',
    },
  ],

  nextStates: ['constitution-8'],
  previousStates: ['constitution-6'],

  provenance: {
    topicMapId: 'constitution-7',
    lineRange: { start: 138, end: 151 },
    section: '2. Constitution',
    order: 7,
  },

  description: 'Constitution that seemed dependent on an external other proves to belong to what something is in itself; determining from outside is conditioned by the internal determination.',
  keyPoints: [
    'Being-for-other isolated as constitution equals determination',
    'External determining depends on immanent determination',
    'Constitution belongs to what something is in itself',
    'Something alters along with its constitution',
  ],
};

/**
 * State 8: Alteration posited in something; negation immanent
 *
 * Dialectic Position: Alteration is no longer merely implicit—negation
 * is posited as immanent, developing the being-in-itself.
 */
const state8: DialecticState = {
  id: 'constitution-8',
  title: 'Alteration posited in something; negation immanent',
  concept: 'ImmanentNegation',
  phase: 'quality',

  moments: [
    {
      name: 'alteration',
      definition: 'Process formerly confined to being-for-other, now posited in the something',
      type: 'process',
    },
    {
      name: 'negation',
      definition: 'Immanent determination of the something',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'alteration',
    },
    {
      name: 'beingInItself',
      definition: 'Developed in the something through immanent negation',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-8-1',
      constraint: 'alteration.positedIn = something',
      predicate: 'positedIn(alteration, something)',
    },
    {
      id: 'inv-8-2',
      constraint: 'negation.immanent = true',
      predicate: 'immanent(negation, something)',
    },
    {
      id: 'inv-8-3',
      constraint: 'beingInItself.developed = true',
      predicate: 'developed(beingInItself)',
    },
  ],

  forces: [
    {
      id: 'force-8-1',
      description: 'Immanent negation doubles the somethings, preparing transition to limit',
      type: 'sublation',
      trigger: 'negation.immanent = true',
      effect: 'distinction.sublated = true',
      targetState: 'constitution-9',
    },
  ],

  transitions: [
    {
      id: 'trans-8-1',
      from: 'constitution-8',
      to: 'constitution-9',
      mechanism: 'sublation',
      description: 'Immanent negation sublates distinction into two somethings',
    },
  ],

  nextStates: ['constitution-9'],
  previousStates: ['constitution-7'],

  provenance: {
    topicMapId: 'constitution-8',
    lineRange: { start: 153, end: 161 },
    section: '2. Constitution',
    order: 8,
  },

  description: 'Alteration is now posited in the something itself; negation becomes immanent and develops the being-in-itself.',
  keyPoints: [
    'Alteration no longer merely implicit',
    'Negation posited as immanent determination',
    'Being-in-itself is developed through this immanence',
  ],
};

/**
 * State 9: Transition I - Two somethings with immanent negation
 *
 * Dialectic Position: The sublation of distinction yields two somethings
 * whose negation is immanent; affirmation is mediated.
 */
const state9: DialecticState = {
  id: 'constitution-9',
  title: 'Transition I: sublation → two somethings; negation immanent',
  concept: 'TwoSomethings',
  phase: 'quality',

  moments: [
    {
      name: 'somethingA',
      definition: 'First something arising from sublated distinction',
      type: 'determination',
    },
    {
      name: 'somethingB',
      definition: 'Second something equally determined',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'somethingA',
    },
    {
      name: 'immanentNegation',
      definition: 'Negation residing within each something',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'inv-9-1',
      constraint: 'negation.immanentIn = {somethingA, somethingB}',
      predicate: 'immanentIn(negation, somethingA) ∧ immanentIn(negation, somethingB)',
    },
    {
      id: 'inv-9-2',
      constraint: 'affirmation.mediated = true',
      predicate: 'mediatedAffirmation(somethingA)',
    },
    {
      id: 'inv-9-3',
      constraint: 'distinction.sublated = true',
      predicate: 'sublated(distinction)',
    },
  ],

  forces: [
    {
      id: 'force-9-1',
      description: 'Immanent negation demands a determinate connection—limit',
      type: 'mediation',
      trigger: 'somethingA.behavesThroughItselfToOther',
      effect: 'determinateness.named = limit',
      targetState: 'constitution-10',
    },
  ],

  transitions: [
    {
      id: 'trans-9-1',
      from: 'constitution-9',
      to: 'constitution-10',
      mechanism: 'mediation',
      description: 'From two somethings to limit as common determinateness',
    },
  ],

  nextStates: ['constitution-10'],
  previousStates: ['constitution-8'],

  provenance: {
    topicMapId: 'constitution-9',
    lineRange: { start: 163, end: 179 },
    section: 'Transition',
    order: 9,
  },

  description: 'Transition of determination and constitution sublates their distinction, positing two somethings whose negation is immanent; affirmation is now mediated.',
  keyPoints: [
    'Sublation yields two somethings',
    'Negation is immanent, not merely comparative',
    'Affirmation is mediated through sublation of otherness',
  ],
};

/**
 * State 10: Transition II - Limit as determinateness joining and separating
 *
 * Dialectic Position: Something behaves to the other through itself; the
 * determinateness identical with in-itselfness is limit.
 */
const state10: DialecticState = {
  id: 'constitution-10',
  title: 'Transition II: something behaves to other through itself; negation of other = quality; limit',
  concept: 'LimitEmergence',
  phase: 'quality',

  moments: [
    {
      name: 'selfNegation',
      definition: 'Non-being of the other contained within the something',
      type: 'negation',
    },
    {
      name: 'otherNegation',
      definition: 'Negation of the external other',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'selfNegation',
    },
    {
      name: 'limit',
      definition: 'Determinateness joining and separating the two somethings',
      type: 'determination',
      relation: 'mediates',
      relatedTo: 'selfNegation',
    },
  ],

  invariants: [
    {
      id: 'inv-10-1',
      constraint: 'selfNegation = inItselfness',
      predicate: 'equals(selfNegation, inItselfness)',
    },
    {
      id: 'inv-10-2',
      constraint: 'limit.joinsAndSeparates = true',
      predicate: 'joinsAndSeparates(limit, somethingA, somethingB)',
    },
    {
      id: 'inv-10-3',
      constraint: 'limit = determinateness',
      predicate: 'is(limit, determinateness)',
    },
  ],

  forces: [
    {
      id: 'force-10-1',
      description: 'Limit now explicit as determinate boundary, leading to limit analysis',
      type: 'sublation',
      trigger: 'limit.named = true',
      effect: 'limit.investigated = true',
      targetState: 'constitution-11',
    },
  ],

  transitions: [
    {
      id: 'trans-10-1',
      from: 'constitution-10',
      to: 'constitution-11',
      mechanism: 'reflection',
      description: 'Move from emergent limit to explicit concept of limit',
    },
  ],

  nextStates: ['constitution-11'],
  previousStates: ['constitution-9'],

  provenance: {
    topicMapId: 'constitution-10',
    lineRange: { start: 181, end: 227 },
    section: 'Transition',
    order: 10,
  },

  description: 'Something behaves toward the other through itself; the negation of its other is its quality, and this determinateness—joining and separating two somethings—is limit.',
  keyPoints: [
    'Otherness posited as own moment',
    'Self-negation preserves something',
    'Determinateness identical with in-itselfness is limit',
  ],
};

/**
 * State 11: Limit I - Non-being of other and of something
 *
 * Dialectic Position: Limit first appears as non-being of the other,
 * marking boundaries for both somethings.
 */
const state11: DialecticState = {
  id: 'constitution-11',
  title: 'Limit I: non-being of other and of something; mediation',
  concept: 'LimitAsBoundary',
  phase: 'quality',

  moments: [
    {
      name: 'limit',
      definition: 'Non-being of the other, marking boundary',
      type: 'negation',
    },
    {
      name: 'something',
      definition: 'Immediate self-referring existence',
      type: 'determination',
    },
    {
      name: 'other',
      definition: 'Another something equally bounded',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'something',
    },
  ],

  invariants: [
    {
      id: 'inv-11-1',
      constraint: 'limit(something) = nonBeing(other)',
      predicate: 'limitNonBeing(something, other)',
    },
    {
      id: 'inv-11-2',
      constraint: 'limit(other) = nonBeing(something)',
      predicate: 'limitNonBeing(other, something)',
    },
    {
      id: 'inv-11-3',
      constraint: 'limit.general = non-being of something in general',
      predicate: 'generalLimit(limit)',
    },
  ],

  forces: [
    {
      id: 'force-11-1',
      description: 'Recognition that limit also constitutes being of the something',
      type: 'contradiction',
      trigger: 'limit.onlyNegation = true',
      effect: 'limit.revealedAsBeing = true',
      targetState: 'constitution-12',
    },
  ],

  transitions: [
    {
      id: 'trans-11-1',
      from: 'constitution-11',
      to: 'constitution-12',
      mechanism: 'reflection',
      description: 'From limit as non-being to limit as mediation/being',
    },
  ],

  nextStates: ['constitution-12'],
  previousStates: ['constitution-10'],

  provenance: {
    topicMapId: 'constitution-11',
    lineRange: { start: 245, end: 260 },
    section: '3. Limit (a)',
    order: 11,
  },

  description: 'Limit first presents itself as the non-being of the other, marking the boundary of both somethings and thus of something in general.',
  keyPoints: [
    'Limit is non-being of the other, not of the something itself',
    'Each limit is simultaneously the other’s limit',
    'Limit = non-being of something in general',
  ],
};

/**
 * State 12: Limit II - Through limit something is; mediation
 *
 * Dialectic Position: Limit is simultaneously non-being of the other
 * and the being of the something—mediation in which both are.
 */
const state12: DialecticState = {
  id: 'constitution-12',
  title: 'Limit II: through limit something is; limit = mediation',
  concept: 'LimitAsMediation',
  phase: 'quality',

  moments: [
    {
      name: 'limit',
      definition: 'Simple negation that is also being of the something',
      type: 'negation',
    },
    {
      name: 'other',
      definition: 'Negation of the negation (in-itselfness)',
      type: 'negation',
    },
    {
      name: 'mediation',
      definition: 'Process through which something and other both are and are not',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'inv-12-1',
      constraint: 'limit.nonBeing(other) ∧ limit.being(something)',
      predicate: 'limitDual(limit, other, something)',
    },
    {
      id: 'inv-12-2',
      constraint: 'something.throughLimit = true',
      predicate: 'throughLimit(something, limit)',
    },
    {
      id: 'inv-12-3',
      constraint: 'mediation(something, other) = limit',
      predicate: 'mediationVia(limit, something, other)',
    },
  ],

  forces: [
    {
      id: 'force-12-1',
      description: 'Dual status of limit leads to spatial representation of inside/outside',
      type: 'appearance',
      trigger: 'limit.viewedAsBoundary',
      effect: 'representation.turnsTo = outsideInside',
      targetState: 'constitution-13',
    },
  ],

  transitions: [
    {
      id: 'trans-12-1',
      from: 'constitution-12',
      to: 'constitution-13',
      mechanism: 'appearance',
      description: 'Limit appears as middle point between existence and non-existence',
    },
  ],

  nextStates: ['constitution-13'],
  previousStates: ['constitution-11'],

  provenance: {
    topicMapId: 'constitution-12',
    lineRange: { start: 262, end: 285 },
    section: '3. Limit (a)',
    order: 12,
  },

  description: 'Limit is both the non-being of the other and the being of the something; it is the mediation through which something and other each both is and is not.',
  keyPoints: [
    'Limit is simple negation (first negation)',
    'Other is negation of the negation (in-itselfness)',
    'Something is through its limit',
    'Limit mediates being and non-being of something and other',
  ],
};

/**
 * State 13: Limit III - Existence outside/inside limit; middle point
 *
 * Dialectic Position: Limit is the middle point where existence and non-existence
 * leave off; representation views it as outside/inside relation.
 */
const state13: DialecticState = {
  id: 'constitution-13',
  title: 'Limit III: existence outside/inside limit; limit as middle point',
  concept: 'LimitAsMiddlePoint',
  phase: 'quality',

  moments: [
    {
      name: 'outsideExistence',
      definition: 'Existence of something beyond its limit',
      type: 'quality',
    },
    {
      name: 'insideExistence',
      definition: 'Existence conceived as inside the limit',
      type: 'quality',
      relation: 'opposite',
      relatedTo: 'outsideExistence',
    },
    {
      name: 'limit',
      definition: 'Middle point, non-being of each—other of both',
      type: 'determination',
      relation: 'mediates',
      relatedTo: 'outsideExistence',
    },
  ],

  invariants: [
    {
      id: 'inv-13-1',
      constraint: 'limit = nonBeing(outsideExistence) = nonBeing(insideExistence)',
      predicate: 'middlePoint(limit, outsideExistence, insideExistence)',
    },
    {
      id: 'inv-13-2',
      constraint: 'existenceBeyondLimit = true',
      predicate: 'beyondLimit(something)',
    },
  ],

  forces: [
    {
      id: 'force-13-1',
      description: 'Representational view leads to recognition that outside existence is only existence in general',
      type: 'reflection',
      trigger: 'outsideExistence.indistinctWithOther',
      effect: 'outsideExistence.reinterpreted = existenceInGeneral',
      targetState: 'constitution-14',
    },
  ],

  transitions: [
    {
      id: 'trans-13-1',
      from: 'constitution-13',
      to: 'constitution-14',
      mechanism: 'reflection',
      description: 'From figurative outside/inside to limit as principle of existence',
    },
  ],

  nextStates: ['constitution-14'],
  previousStates: ['constitution-12'],

  provenance: {
    topicMapId: 'constitution-13',
    lineRange: { start: 287, end: 312 },
    section: '3. Limit (b)',
    order: 13,
  },

  description: 'Something has existence outside (or inside) its limit; the limit is the middle point where existence and non-existence leave off, the other of both.',
  keyPoints: [
    'Something in its limit both is and is not',
    'Existence and non-existence fall outside each other',
    'Limit is middle point/other of both',
    'Representation sees line outside point, plane outside line, etc.',
  ],
};

/**
 * State 14: Limit IV - Limit as principle; existence only in limit
 *
 * Dialectic Position: Outside the limit is only existence in general; limit
 * is the principle/element in which something truly is.
 */
const state14: DialecticState = {
  id: 'constitution-14',
  title: 'Limit IV: something outside limit = existence in general; limit as principle',
  concept: 'LimitAsPrinciple',
  phase: 'quality',

  moments: [
    {
      name: 'limit',
      definition: 'Common distinguishedness, unity of existence and distinguishedness',
      type: 'determination',
    },
    {
      name: 'existenceInGeneral',
      definition: 'Undifferentiated being outside the limit',
      type: 'quality',
    },
    {
      name: 'selfSeparation',
      definition: 'Something declaring its non-being to be its being, passing beyond',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'inv-14-1',
      constraint: 'something.hasExistenceOnlyIn = limit',
      predicate: 'existsOnlyIn(something, limit)',
    },
    {
      id: 'inv-14-2',
      constraint: 'limit = principle(element) of what it delimits',
      predicate: 'principle(limit, delimited)',
    },
    {
      id: 'inv-14-3',
      constraint: 'limit.opposedToImmediateExistence = each negative of other',
      predicate: 'mutualNegation(limit, immediateExistence)',
    },
  ],

  forces: [
    {
      id: 'force-14-1',
      description: 'Limit as principle reveals inherent contradiction, driving movement',
      type: 'contradiction',
      trigger: 'selfSeparation.active',
      effect: 'limit.revealsUnrest = true',
      targetState: 'constitution-15',
    },
  ],

  transitions: [
    {
      id: 'trans-14-1',
      from: 'constitution-14',
      to: 'constitution-15',
      mechanism: 'contradiction',
      description: 'Limit as principle turns into unrest pushing beyond',
    },
  ],

  nextStates: ['constitution-15'],
  previousStates: ['constitution-13'],

  provenance: {
    topicMapId: 'constitution-14',
    lineRange: { start: 314, end: 359 },
    section: '3. Limit (c)',
    order: 14,
  },

  description: 'Outside the limit, something is only existence in general; therefore it is what it is only in its limit, which is the principle/element of what it delimitates.',
  keyPoints: [
    'Something outside limit indistinguishable from other',
    'Limit is common distinguishedness and unity',
    'Limit and immediate existence negate each other',
    'Limits are principles/elements (point→line→plane→solid)',
  ],
};

/**
 * State 15: Limit V - Unrest/contradiction; dialectic of point/line/plane
 *
 * Dialectic Position: Limit is intrinsically contradictory; point, line, plane
 * repel themselves and pass over, showing no static limit exists.
 */
const state15: DialecticState = {
  id: 'constitution-15',
  title: 'Limit V: unrest/contradiction; dialectic of point/line/plane',
  concept: 'LimitContradiction',
  phase: 'quality',

  moments: [
    {
      name: 'point',
      definition: 'Totally abstract limit in determinate existence',
      type: 'determination',
    },
    {
      name: 'line',
      definition: 'Movement/transition of point',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'point',
    },
    {
      name: 'plane',
      definition: 'Movement/transition of line, leading to total space',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'line',
    },
  ],

  invariants: [
    {
      id: 'inv-15-1',
      constraint: 'point.selfContradictory = true',
      predicate: 'selfContradiction(point)',
    },
    {
      id: 'inv-15-2',
      constraint: 'line = movement(point)',
      predicate: 'movement(point, line)',
    },
    {
      id: 'inv-15-3',
      constraint: 'plane = movement(line)',
      predicate: 'movement(line, plane)',
    },
    {
      id: 'inv-15-4',
      constraint: 'noStaticLimit(point|line|plane)',
      predicate: 'notExistsStaticLimit(pointLinePlane)',
    },
  ],

  forces: [
    {
      id: 'force-15-1',
      description: 'Contradiction of limit propels something beyond itself, producing finitude',
      type: 'contradiction',
      trigger: 'limit.repelsItself = true',
      effect: 'something.drivenBeyond = true',
      targetState: 'constitution-16',
    },
  ],

  transitions: [
    {
      id: 'trans-15-1',
      from: 'constitution-15',
      to: 'constitution-16',
      mechanism: 'contradiction',
      description: 'Limit\'s unrest culminates in finite that drives beyond itself',
    },
  ],

  nextStates: ['constitution-16'],
  previousStates: ['constitution-14'],

  provenance: {
    topicMapId: 'constitution-15',
    lineRange: { start: 361, end: 405 },
    section: '3. Limit (c)',
    order: 15,
  },

  description: 'Point, line, plane are self-contradictory limits that repel themselves, demonstrating that limit is an inherent unrest rather than a static border.',
  keyPoints: [
    'Unrest of something in its limit is immanent',
    'Point becomes line; line becomes plane; plane becomes total space',
    'Coming-to-be is not accidental but grounded in limits as principles',
    'There is no static point/line/plane—they transition through their own concept',
  ],
};

/**
 * State 16: Conclusion - Finite as something with immanent limit
 *
 * Dialectic Position: Something, with its immanent limit as contradiction,
 * is driven beyond itself—this is the finite.
 */
const state16: DialecticState = {
  id: 'constitution-16',
  title: 'Conclusion: something with immanent limit = finite',
  concept: 'TheFinite',
  phase: 'quality',

  moments: [
    {
      name: 'something',
      definition: 'Being that carries its limit within',
      type: 'determination',
    },
    {
      name: 'immanentLimit',
      definition: 'Contradiction of the something with itself',
      type: 'negation',
    },
    {
      name: 'beyond',
      definition: 'Direction toward which the finite is driven',
      type: 'process',
      relation: 'transitions',
      relatedTo: 'something',
    },
  ],

  invariants: [
    {
      id: 'inv-16-1',
      constraint: 'something.contains(immanentLimit)',
      predicate: 'contains(something, immanentLimit)',
    },
    {
      id: 'inv-16-2',
      constraint: 'immanentLimit = contradictionOf(something)',
      predicate: 'contradictionOf(immanentLimit, something)',
    },
    {
      id: 'inv-16-3',
      constraint: 'finite = drivenBeyondItself',
      predicate: 'drivenBeyond(finite)',
    },
  ],

  forces: [
    {
      id: 'force-16-1',
      description: 'Finite\'s immanent contradiction propels it to its beyond',
      type: 'contradiction',
      trigger: 'finite.exists',
      effect: 'finite.transcends = true',
      targetState: 'constitution-1', // cyclic reference when continuing dialectic
    },
  ],

  transitions: [
    {
      id: 'trans-16-1',
      from: 'constitution-16',
      to: 'constitution-1',
      mechanism: 'sublation',
      description: 'Finite returns to new immediacy, restarting dialectic',
    },
  ],

  nextStates: ['constitution-1'],
  previousStates: ['constitution-15'],

  provenance: {
    topicMapId: 'constitution-16',
    lineRange: { start: 407, end: 410 },
    section: 'Conclusion',
    order: 16,
  },

  description: 'Something posited with its immanent limit as contradiction is driven beyond itself—this being is the finite.',
  keyPoints: [
    'Finite = something with immanent limit',
    'Limit is contradiction propelling it beyond',
    'Direction out of itself is essential to the finite',
  ],
};

/**
 * Complete Constitution IR Document
 *
 * This is the executable pseudo-code representation of the entire
 * "Determination, constitution, and limit" section.
 */
export const constitutionIR: DialecticIR = {
  id: 'constitution-ir',
  title: 'Constitution IR: Determination, Constitution, and Limit',
  section: 'B. FINITUDE (b) Determination, constitution, and limit',

  states: [
    state1,
    state2,
    state3,
    state4,
    state5,
    state6,
    state7,
    state8,
    state9,
    state10,
    state11,
    state12,
    state13,
    state14,
    state15,
    state16,
  ],

  metadata: {
    sourceFile: 'constitution.txt',
    totalStates: 16,
    cpuGpuMapping: {
      'constitution-1': 'quality',
      'constitution-2': 'quality',
      'constitution-3': 'quality',
      'constitution-4': 'quality',
      'constitution-5': 'quality',
      'constitution-6': 'quality',
      'constitution-7': 'quality',
      'constitution-8': 'quality',
      'constitution-9': 'quality',
      'constitution-10': 'quality',
      'constitution-11': 'quality',
      'constitution-12': 'quality',
      'constitution-13': 'quality',
      'constitution-14': 'quality',
      'constitution-15': 'quality',
      'constitution-16': 'quality',
    },
  },
};

/**
 * Export individual states for programmatic access
 */
export const constitutionStates = {
  'constitution-1': state1,
  'constitution-2': state2,
  'constitution-3': state3,
  'constitution-4': state4,
  'constitution-5': state5,
  'constitution-6': state6,
  'constitution-7': state7,
  'constitution-8': state8,
  'constitution-9': state9,
  'constitution-10': state10,
  'constitution-11': state11,
  'constitution-12': state12,
  'constitution-13': state13,
  'constitution-14': state14,
  'constitution-15': state15,
  'constitution-16': state16,
};

