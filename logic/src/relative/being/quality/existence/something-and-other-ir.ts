/**
 * Something and Other IR: Dialectic Pseudo-Code for Something and Other
 *
 * This file demonstrates the conversion from TopicMap + Chunks → Dialectic IR
 * as executable pseudo-code that advances the dialectic.
 *
 * Architecture: CPU (Quality/Being → Reflection → Subject)
 * Section: B. FINITUDE (a) Something and other
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
import { somethingAndOtherTopicMap } from './sources/something-and-other-topic-map';

/**
 * State 1: Outline - Something and other → Determination/constitution/limit → Finitude
 *
 * Dialectic Position: The structure of finitude begins with something and other.
 */
const state1: DialecticState = {
  id: 'something-and-other-1',
  title: 'Outline: (a) Something and other → (b) Determination/constitution/limit → (c) Finitude',
  concept: 'FinitudeStructure',
  phase: 'quality', // CPU: Quality/Being

  moments: [
    {
      name: 'something',
      definition: 'Immediate existent, in itself in contrast to being-for-other',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'other',
    },
    {
      name: 'other',
      definition: 'Also immediate existent, negation falls outside both',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'something',
    },
    {
      name: 'determinateness',
      definition: 'Belongs to in-itself, passes over into constitution and limit',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-1-1',
      constraint: 'something.other = indifferent',
      predicate: 'indifferent(something, other)',
    },
    {
      id: 'inv-1-2',
      constraint: 'negation.fallsOutside = {something, other}',
      predicate: 'fallsOutside(negation, something) ∧ fallsOutside(negation, other)',
    },
    {
      id: 'inv-1-3',
      constraint: 'determinateness.belongsTo = inItself',
      predicate: 'belongsTo(determinateness, inItself)',
    },
  ],

  forces: [
    {
      id: 'force-1-1',
      description: 'Determinateness in in-itself drives toward constitution and limit',
      type: 'passover',
      trigger: 'determinateness.inItself = true',
      effect: 'determinateness.passesOver = constitution',
      targetState: 'constitution-1',
    },
  ],

  transitions: [
    {
      id: 'trans-1-1',
      from: 'something-and-other-1',
      to: 'something-and-other-2',
      mechanism: 'negation',
      description: 'From outline to negative determination',
    },
  ],

  nextStates: ['something-and-other-2'],
  previousStates: ['existence-16'],

  provenance: {
    topicMapId: 'something-and-other-1',
    lineRange: { start: 4, end: 19 },
    section: 'B. FINITUDE',
    order: 1,
  },

  description: '(a) Something and other: at first they are indifferent to one another; an other is also an immediate existent, a something; the negation thus falls outside both.',
  keyPoints: [
    '(a) Something and other: indifferent at first; negation falls outside both',
    '(b) Determination passes over into constitution; limit of something',
    '(c) Limit is immanent determination; something is finite',
  ],
};

/**
 * State 2: Introduction - Negative determination
 *
 * Dialectic Position: This division develops the negative determination,
 * now determined to being-in-itself (negation of negation).
 */
const state2: DialecticState = {
  id: 'something-and-other-2',
  title: 'Introduction: negative determination (vs affirmative in first division)',
  concept: 'NegativeDetermination',
  phase: 'quality',

  moments: [
    {
      name: 'firstDivision',
      definition: 'Existence had determination of existent (affirmative)',
      type: 'determination',
    },
    {
      name: 'presentDivision',
      definition: 'Develops negative determination present in existence',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'firstDivision',
    },
    {
      name: 'negationOfNegation',
      definition: 'Being-in-itself, determined from first negation',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'inv-2-1',
      constraint: 'firstDivision.determination = affirmative',
      predicate: 'is(firstDivision.determination, affirmative)',
    },
    {
      id: 'inv-2-2',
      constraint: 'presentDivision.determination = negative',
      predicate: 'is(presentDivision.determination, negative)',
    },
    {
      id: 'inv-2-3',
      constraint: 'negationOfNegation = beingInItself',
      predicate: 'equals(negationOfNegation, beingInItself)',
    },
  ],

  forces: [
    {
      id: 'force-2-1',
      description: 'Negative determination drives toward something and other',
      type: 'negation',
      trigger: 'negativeDetermination.present = true',
      effect: 'somethingAndOther.emerges = true',
      targetState: 'something-and-other-3',
    },
  ],

  transitions: [
    {
      id: 'trans-2-1',
      from: 'something-and-other-2',
      to: 'something-and-other-3',
      mechanism: 'negation',
      description: 'Negative determination leads to something and other',
    },
  ],

  nextStates: ['something-and-other-3'],
  previousStates: ['something-and-other-1'],

  provenance: {
    topicMapId: 'something-and-other-2',
    lineRange: { start: 21, end: 30 },
    section: 'B. FINITUDE',
    order: 2,
  },

  description: 'The present division develops the negative determination which is present in existence and was there from the start only as negation in general.',
  keyPoints: [
    'First division: existence had determination of existent (affirmative)',
    'Present division: develops negative determination present in existence',
    'Was first negation; now determined to being-in-itself, negation of negation',
  ],
};

/**
 * State 3: Something and other I - Both existents; both others; indifference
 *
 * Dialectic Position: Something and other are both existents, both others,
 * indifferent which is named first.
 */
const state3: DialecticState = {
  id: 'something-and-other-3',
  title: 'Something and other I: both existents; both others; indifference',
  concept: 'SomethingAndOtherIndifference',
  phase: 'quality',

  moments: [
    {
      name: 'something',
      definition: 'First existent, equally an other',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'other',
    },
    {
      name: 'other',
      definition: 'Second existent, equally an other',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'something',
    },
    {
      name: 'indifference',
      definition: 'It is indifferent which is named first',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-3-1',
      constraint: 'something = existent',
      predicate: 'is(something, existent)',
    },
    {
      id: 'inv-3-2',
      constraint: 'other = existent',
      predicate: 'is(other, existent)',
    },
    {
      id: 'inv-3-3',
      constraint: 'something.other = true',
      predicate: 'isOther(something)',
    },
    {
      id: 'inv-3-4',
      constraint: 'other.other = true',
      predicate: 'isOther(other)',
    },
    {
      id: 'inv-3-5',
      constraint: 'namingOrder.indifferent = true',
      predicate: 'indifferent(namingOrder)',
    },
  ],

  forces: [
    {
      id: 'force-3-1',
      description: 'Indifference drives toward subjective designation',
      type: 'externality',
      trigger: 'naming.required = true',
      effect: 'subjectiveDesignation.emerges = true',
      targetState: 'something-and-other-4',
    },
  ],

  transitions: [
    {
      id: 'trans-3-1',
      from: 'something-and-other-3',
      to: 'something-and-other-4',
      mechanism: 'reflection',
      description: 'From indifference to subjective designation ("this")',
    },
  ],

  nextStates: ['something-and-other-4'],
  previousStates: ['something-and-other-2'],

  provenance: {
    topicMapId: 'something-and-other-3',
    lineRange: { start: 34, end: 46 },
    section: 'a. Something and other',
    order: 3,
  },

  description: 'Something and other are, first, both existents or something. Second, each is equally an other. It is indifferent which is named first.',
  keyPoints: [
    'First: both existents or something',
    'Second: each equally an other',
    'Indifferent which named first',
    'Both other in same way',
  ],
};

/**
 * State 4: "This" as subjective designation
 *
 * Dialectic Position: "This" fixes distinction but is subjective,
 * falling outside the something itself.
 */
const state4: DialecticState = {
  id: 'something-and-other-4',
  title: '"This" as subjective designation; language expresses universal',
  concept: 'SubjectiveDesignation',
  phase: 'quality',

  moments: [
    {
      name: 'this',
      definition: 'Subjective designation fixing distinction',
      type: 'determination',
    },
    {
      name: 'externalPointing',
      definition: 'Determinateness falls on side of external pointing',
      type: 'determination',
    },
    {
      name: 'language',
      definition: 'Work of understanding, expresses universal',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-4-1',
      constraint: 'this.fixesDistinction = true',
      predicate: 'fixesDistinction(this)',
    },
    {
      id: 'inv-4-2',
      constraint: 'this.subjective = true',
      predicate: 'subjective(this)',
    },
    {
      id: 'inv-4-3',
      constraint: 'this.fallsOutside = something',
      predicate: 'fallsOutside(this, something)',
    },
    {
      id: 'inv-4-4',
      constraint: 'language.expresses = universal',
      predicate: 'expresses(language, universal)',
    },
  ],

  forces: [
    {
      id: 'force-4-1',
      description: 'External pointing reveals otherness as external',
      type: 'externality',
      trigger: 'externalPointing.active',
      effect: 'otherness.revealedAsExternal = true',
      targetState: 'something-and-other-5',
    },
  ],

  transitions: [
    {
      id: 'trans-4-1',
      from: 'something-and-other-4',
      to: 'something-and-other-5',
      mechanism: 'reflection',
      description: 'From "this" to otherness as external',
    },
  ],

  nextStates: ['something-and-other-5'],
  previousStates: ['something-and-other-3'],

  provenance: {
    topicMapId: 'something-and-other-4',
    lineRange: { start: 47, end: 63 },
    section: 'a. Something and other',
    order: 4,
  },

  description: '"This" serves to fix the distinction and the something which is to be taken in the affirmative sense. But "this" also expresses the fact that the distinction is a subjective designation that falls outside the something itself.',
  keyPoints: [
    '"This" fixes distinction, privileges one something',
    'Subjective designation falling outside something itself',
    'Language expresses universal, naming as single object',
  ],
};

/**
 * State 5: Otherness as external
 *
 * Dialectic Position: Otherness appears as determination alien to existence,
 * determined by comparison by a Third.
 */
const state5: DialecticState = {
  id: 'something-and-other-5',
  title: 'Otherness as external; comparison by Third; every existence is other',
  concept: 'OthernessExternal',
  phase: 'quality',

  moments: [
    {
      name: 'otherness',
      definition: 'Determination alien to existence',
      type: 'determination',
    },
    {
      name: 'third',
      definition: 'That which compares, determining otherness',
      type: 'mediation',
    },
    {
      name: 'externalOther',
      definition: 'Other outside existence, not other for itself',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-5-1',
      constraint: 'otherness.alien = existence',
      predicate: 'alien(otherness, existence)',
    },
    {
      id: 'inv-5-2',
      constraint: 'otherness.determinedBy = third',
      predicate: 'determinedBy(otherness, third)',
    },
    {
      id: 'inv-5-3',
      constraint: 'externalOther.notOtherForItself = true',
      predicate: 'notOtherForItself(externalOther)',
    },
    {
      id: 'inv-5-4',
      constraint: 'everyExistence.other = true',
      predicate: 'forall(existence, isOther)',
    },
  ],

  forces: [
    {
      id: 'force-5-1',
      description: 'External otherness drives toward sameness in reflection',
      type: 'reflection',
      trigger: 'otherness.external = true',
      effect: 'sameness.inReflection = true',
      targetState: 'something-and-other-6',
    },
  ],

  transitions: [
    {
      id: 'trans-5-1',
      from: 'something-and-other-5',
      to: 'something-and-other-6',
      mechanism: 'reflection',
      description: 'From external otherness to sameness in reflection',
    },
  ],

  nextStates: ['something-and-other-6'],
  previousStates: ['something-and-other-4'],

  provenance: {
    topicMapId: 'something-and-other-5',
    lineRange: { start: 65, end: 79 },
    section: 'a. Something and other',
    order: 5,
  },

  description: 'Otherness thus appears as a determination alien to the existence thus pointed at, or the other existence as outside this one existence, partly because the one existence is determined as other only by being compared by a Third.',
  keyPoints: [
    'Otherness appears as determination alien to existence',
    'Determined as other by comparison by Third',
    'Every existence determines itself as other existence',
  ],
};

/**
 * State 6: Sameness in external reflection
 *
 * Dialectic Position: Both determined as something and other: same,
 * but sameness falls only in external reflection.
 */
const state6: DialecticState = {
  id: 'something-and-other-6',
  title: 'Sameness falls in external reflection; other for itself apart from something',
  concept: 'SamenessInReflection',
  phase: 'quality',

  moments: [
    {
      name: 'sameness',
      definition: 'Both determined as something and other: same',
      type: 'determination',
    },
    {
      name: 'externalReflection',
      definition: 'Sameness falls only in external reflection (comparison)',
      type: 'mediation',
    },
    {
      name: 'otherForItself',
      definition: 'Other for itself apart from something',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-6-1',
      constraint: 'something.other = true',
      predicate: 'isOther(something)',
    },
    {
      id: 'inv-6-2',
      constraint: 'other.something = true',
      predicate: 'isSomething(other)',
    },
    {
      id: 'inv-6-3',
      constraint: 'sameness.in = externalReflection',
      predicate: 'in(sameness, externalReflection)',
    },
    {
      id: 'inv-6-4',
      constraint: 'otherForItself.apartFrom = something',
      predicate: 'apartFrom(otherForItself, something)',
    },
  ],

  forces: [
    {
      id: 'force-6-1',
      description: 'Other for itself drives toward other-of-itself',
      type: 'reflection',
      trigger: 'otherForItself.emerges = true',
      effect: 'otherOfItself.emerges = true',
      targetState: 'something-and-other-7',
    },
  ],

  transitions: [
    {
      id: 'trans-6-1',
      from: 'something-and-other-6',
      to: 'something-and-other-7',
      mechanism: 'reflection',
      description: 'From other for itself to other-of-itself',
    },
  ],

  nextStates: ['something-and-other-7'],
  previousStates: ['something-and-other-5'],

  provenance: {
    topicMapId: 'something-and-other-6',
    lineRange: { start: 81, end: 89 },
    section: 'a. Something and other',
    order: 6,
  },

  description: 'Both are determined as something as well as other: thus they are the same and there is as yet no distinction present in them. But this sameness of determinations, too, falls only within external reflection.',
  keyPoints: [
    'Both determined as something and other: same, no distinction',
    'Sameness falls only in external reflection (comparison)',
    'Other, as posited, is other for itself apart from something',
  ],
};

/**
 * State 7: Other as 'to heteron' - Other-of-itself
 *
 * Dialectic Position: Other taken in isolation is other-of-itself,
 * as in nature as other of spirit.
 */
const state7: DialecticState = {
  id: 'something-and-other-7',
  title: "Other as 'to heteron' (Plato); other-of-itself; nature as other of spirit",
  concept: 'OtherOfItself',
  phase: 'quality',

  moments: [
    {
      name: 'toHeteron',
      definition: "Plato's other, opposed to one as moment of totality",
      type: 'determination',
    },
    {
      name: 'otherOfItself',
      definition: 'Other within, other of itself',
      type: 'determination',
    },
    {
      name: 'nature',
      definition: 'Other of spirit, exists-outside-itself (space, time, matter)',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'spirit',
    },
  ],

  invariants: [
    {
      id: 'inv-7-1',
      constraint: 'toHeteron.opposedTo = one',
      predicate: 'opposedTo(toHeteron, one)',
    },
    {
      id: 'inv-7-2',
      constraint: 'otherOfItself.otherWithin = true',
      predicate: 'otherWithin(otherOfItself)',
    },
    {
      id: 'inv-7-3',
      constraint: 'nature.otherOf = spirit',
      predicate: 'otherOf(nature, spirit)',
    },
    {
      id: 'inv-7-4',
      constraint: 'nature.existsOutsideItself = true',
      predicate: 'existsOutsideItself(nature)',
    },
  ],

  forces: [
    {
      id: 'force-7-1',
      description: 'Other-of-itself drives toward absolutely unequal',
      type: 'negation',
      trigger: 'otherOfItself.emerges = true',
      effect: 'absolutelyUnequal.emerges = true',
      targetState: 'something-and-other-8',
    },
  ],

  transitions: [
    {
      id: 'trans-7-1',
      from: 'something-and-other-7',
      to: 'something-and-other-8',
      mechanism: 'negation',
      description: 'From other-of-itself to absolutely unequal',
    },
  ],

  nextStates: ['something-and-other-8'],
  previousStates: ['something-and-other-6'],

  provenance: {
    topicMapId: 'something-and-other-7',
    lineRange: { start: 91, end: 108 },
    section: 'a. Something and other',
    order: 7,
  },

  description: 'The other, taken solely as such, is not the other of something, but is the other within, that is, the other of itself. Such an other, which is the other by its own determination, is physical nature.',
  keyPoints: [
    "Other taken in isolation, abstractly as the other",
    "'To heteron' of Plato, opposed to one as moment of totality",
    'Other, taken solely, is other within, other of itself',
    "Nature's quality: to be other within, that-which-exists-outside-itself",
  ],
};

/**
 * State 8: Other-of-itself - Absolutely unequal
 *
 * Dialectic Position: Other for itself is absolutely unequal in itself,
 * negates and alters itself, yet remains identical.
 */
const state8: DialecticState = {
  id: 'something-and-other-8',
  title: 'Other-of-itself: absolutely unequal, negates/alters itself, yet identical',
  concept: 'AbsolutelyUnequal',
  phase: 'quality',

  moments: [
    {
      name: 'absolutelyUnequal',
      definition: 'Other of other, absolutely unequal in itself',
      type: 'negation',
    },
    {
      name: 'selfNegation',
      definition: 'Negates itself, alters itself',
      type: 'negation',
    },
    {
      name: 'selfIdentity',
      definition: 'Remains identical, unites with itself',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-8-1',
      constraint: 'absolutelyUnequal.otherOfOther = true',
      predicate: 'otherOfOther(absolutelyUnequal)',
    },
    {
      id: 'inv-8-2',
      constraint: 'selfNegation.negatesItself = true',
      predicate: 'negatesItself(selfNegation)',
    },
    {
      id: 'inv-8-3',
      constraint: 'selfNegation.altersItself = true',
      predicate: 'altersItself(selfNegation)',
    },
    {
      id: 'inv-8-4',
      constraint: 'selfIdentity.remains = true',
      predicate: 'remains(selfIdentity)',
    },
  ],

  forces: [
    {
      id: 'force-8-1',
      description: 'Self-identity through alteration drives toward being-for-other',
      type: 'sublation',
      trigger: 'selfIdentity.throughAlteration = true',
      effect: 'beingForOther.emerges = true',
      targetState: 'something-and-other-9',
    },
  ],

  transitions: [
    {
      id: 'trans-8-1',
      from: 'something-and-other-8',
      to: 'something-and-other-9',
      mechanism: 'sublation',
      description: 'From absolutely unequal to being-for-other and being-in-itself',
    },
  ],

  nextStates: ['something-and-other-9'],
  previousStates: ['something-and-other-7'],

  provenance: {
    topicMapId: 'something-and-other-8',
    lineRange: { start: 110, end: 124 },
    section: 'a. Something and other',
    order: 8,
  },

  description: 'The other which is such for itself is the other within it, hence the other of itself and so the other of the other; therefore, the absolutely unequal in itself, that which negates itself, alters itself. But it equally remains identical with itself.',
  keyPoints: [
    'Other for itself is other within it, other of itself, other of other',
    'Absolutely unequal in itself',
    'Negates itself, alters itself',
    'Equally remains identical (alters to other with no additional determination)',
  ],
};

/**
 * State 9: Something preserves itself in non-being
 *
 * Dialectic Position: Something preserves itself in its non-being;
 * being-for-other and being-in-itself emerge.
 */
const state9: DialecticState = {
  id: 'something-and-other-9',
  title: 'Something preserves itself in non-being; being-for-other and being-in-itself',
  concept: 'BeingForOtherInItself',
  phase: 'quality',

  moments: [
    {
      name: 'something',
      definition: 'Preserves itself in non-being',
      type: 'determination',
    },
    {
      name: 'beingForOther',
      definition: 'Otherness at once contained and separated',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'beingInItself',
    },
    {
      name: 'beingInItself',
      definition: 'Being with reference to itself, self-equality',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'beingForOther',
    },
  ],

  invariants: [
    {
      id: 'inv-9-1',
      constraint: 'something.preservesInNonBeing = true',
      predicate: 'preservesInNonBeing(something)',
    },
    {
      id: 'inv-9-2',
      constraint: 'something.essentiallyOneWith = nonBeing',
      predicate: 'essentiallyOneWith(something, nonBeing)',
    },
    {
      id: 'inv-9-3',
      constraint: 'something.essentiallyNotOneWith = nonBeing',
      predicate: 'essentiallyNotOneWith(something, nonBeing)',
    },
    {
      id: 'inv-9-4',
      constraint: 'beingForOther.containedAndSeparated = true',
      predicate: 'containedAndSeparated(beingForOther)',
    },
    {
      id: 'inv-9-5',
      constraint: 'beingInItself.selfEquality = true',
      predicate: 'selfEquality(beingInItself)',
    },
  ],

  forces: [
    {
      id: 'force-9-1',
      description: 'Two moments drive toward their relationship',
      type: 'mediation',
      trigger: 'beingForOther.and.beingInItself.emerged',
      effect: 'relationship.established = true',
      targetState: 'something-and-other-10',
    },
  ],

  transitions: [
    {
      id: 'trans-9-1',
      from: 'something-and-other-9',
      to: 'something-and-other-10',
      mechanism: 'mediation',
      description: 'From two moments to their relationship',
    },
  ],

  nextStates: ['something-and-other-10'],
  previousStates: ['something-and-other-8'],

  provenance: {
    topicMapId: 'something-and-other-9',
    lineRange: { start: 126, end: 145 },
    section: 'a. Something and other',
    order: 9,
  },

  description: 'The something preserves itself in its non-being; it is essentially one with it, and essentially not one with it. It therefore stands in reference to an otherness without being just this otherness.',
  keyPoints: [
    'Something preserves itself in non-being',
    'Essentially one with it, essentially not one with it',
    'Otherness at once contained and separated: being-for-other',
    'Being with reference to itself: being-in-itself',
  ],
};

/**
 * State 10: Two pairs of determinations
 *
 * Dialectic Position: Being-for-other and being-in-itself are the two moments;
 * their truth is their connection.
 */
const state10: DialecticState = {
  id: 'something-and-other-10',
  title: 'Two pairs of determinations; truth is connection',
  concept: 'TwoPairsConnection',
  phase: 'quality',

  moments: [
    {
      name: 'pair1',
      definition: 'Something and other (non-connectedness)',
      type: 'determination',
    },
    {
      name: 'pair2',
      definition: 'Being-for-other and being-in-itself (connection)',
      type: 'determination',
    },
    {
      name: 'truth',
      definition: 'Connection of determinations',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'inv-10-1',
      constraint: 'pair1.nonConnectedness = true',
      predicate: 'nonConnectedness(pair1)',
    },
    {
      id: 'inv-10-2',
      constraint: 'pair2.connection = true',
      predicate: 'connection(pair2)',
    },
    {
      id: 'inv-10-3',
      constraint: 'truth = connection',
      predicate: 'equals(truth, connection)',
    },
    {
      id: 'inv-10-4',
      constraint: 'beingForOther.contains = beingInItself',
      predicate: 'contains(beingForOther, beingInItself)',
    },
  ],

  forces: [
    {
      id: 'force-10-1',
      description: 'Connection drives toward identity',
      type: 'mediation',
      trigger: 'connection.established = true',
      effect: 'identity.emerges = true',
      targetState: 'something-and-other-11',
    },
  ],

  transitions: [
    {
      id: 'trans-10-1',
      from: 'something-and-other-10',
      to: 'something-and-other-11',
      mechanism: 'mediation',
      description: 'From connection to identity of moments',
    },
  ],

  nextStates: ['something-and-other-11'],
  previousStates: ['something-and-other-9'],

  provenance: {
    topicMapId: 'something-and-other-10',
    lineRange: { start: 147, end: 179 },
    section: 'a. Something and other',
    order: 10,
  },

  description: 'Being-for-other and being-in-itself constitute the two moments of something. There are here two pairs of determinations: (1) something and other; (2) being-for-other and being-in-itself. The former contain the non-connectedness of their determinateness; something and other fall apart. But their truth is their connection.',
  keyPoints: [
    'Being-for-other and being-in-itself = two moments of something',
    'Two pairs: (1) something and other; (2) being-for-other and being-in-itself',
    'Truth is connection',
  ],
};

/**
 * State 11: Being-in-itself - Negative reference
 *
 * Dialectic Position: Being-in-itself has non-being in it;
 * being-for-other points to being-in-itself.
 */
const state11: DialecticState = {
  id: 'something-and-other-11',
  title: 'Being-in-itself: negative reference; has non-being in it',
  concept: 'BeingInItselfNegative',
  phase: 'quality',

  moments: [
    {
      name: 'beingInItself',
      definition: 'Negative reference to non-existence, has otherness outside',
      type: 'determination',
    },
    {
      name: 'nonBeingInIt',
      definition: 'Non-being right in it (non-being of being-for-other)',
      type: 'negation',
    },
    {
      name: 'beingForOther',
      definition: 'Points to being-in-itself as reflected',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-11-1',
      constraint: 'beingInItself.negativeReference = nonExistence',
      predicate: 'negativeReference(beingInItself, nonExistence)',
    },
    {
      id: 'inv-11-2',
      constraint: 'beingInItself.hasNonBeing = true',
      predicate: 'hasNonBeing(beingInItself)',
    },
    {
      id: 'inv-11-3',
      constraint: 'nonBeingInIt = nonBeingOf(beingForOther)',
      predicate: 'equals(nonBeingInIt, nonBeingOf(beingForOther))',
    },
    {
      id: 'inv-11-4',
      constraint: 'beingForOther.pointsTo = beingInItself',
      predicate: 'pointsTo(beingForOther, beingInItself)',
    },
  ],

  forces: [
    {
      id: 'force-11-1',
      description: 'Mutual pointing drives toward identity',
      type: 'mediation',
      trigger: 'beingInItself.pointsTo.beingForOther = true',
      effect: 'identity.established = true',
      targetState: 'something-and-other-12',
    },
  ],

  transitions: [
    {
      id: 'trans-11-1',
      from: 'something-and-other-11',
      to: 'something-and-other-12',
      mechanism: 'mediation',
      description: 'From mutual pointing to identity',
    },
  ],

  nextStates: ['something-and-other-12'],
  previousStates: ['something-and-other-10'],

  provenance: {
    topicMapId: 'something-and-other-11',
    lineRange: { start: 181, end: 197 },
    section: 'a. Something and other',
    order: 11,
  },

  description: 'Being-in-itself is, first, negative reference to non-existence; it has otherness outside it and is opposed to it. But, second, it has non-being also right in it; for it is itself the non-being of being-for-other.',
  keyPoints: [
    'Being-in-itself: first, negative reference to non-existence',
    'Second: has non-being in it (non-being of being-for-other)',
    'Being-for-other points to being-in-itself as reflected',
  ],
};

/**
 * State 12: Identity of being-in-itself and being-for-other
 *
 * Dialectic Position: Both moments are determinations of one and the same;
 * something has in it what it is in itself.
 */
const state12: DialecticState = {
  id: 'something-and-other-12',
  title: 'Both moments of something; identity of being-in-itself and being-for-other',
  concept: 'IdentityOfMoments',
  phase: 'quality',

  moments: [
    {
      name: 'something',
      definition: 'One and the same of both moments',
      type: 'determination',
    },
    {
      name: 'identity',
      definition: 'Something has in it what it is in itself; in itself what it is as being-for-other',
      type: 'mediation',
    },
    {
      name: 'undivided',
      definition: 'Both moments in it, undivided',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-12-1',
      constraint: 'bothMoments.of = something',
      predicate: 'of(bothMoments, something)',
    },
    {
      id: 'inv-12-2',
      constraint: 'something.hasInIt = whatIsInItself',
      predicate: 'hasInIt(something, whatIsInItself)',
    },
    {
      id: 'inv-12-3',
      constraint: 'something.inItself = whatIsAsBeingForOther',
      predicate: 'inItself(something, whatIsAsBeingForOther)',
    },
    {
      id: 'inv-12-4',
      constraint: 'moments.undivided = true',
      predicate: 'undivided(moments)',
    },
  ],

  forces: [
    {
      id: 'force-12-1',
      description: 'Identity drives toward critique of thing-in-itself',
      type: 'reflection',
      trigger: 'identity.established = true',
      effect: 'thingInItself.critique = activated',
      targetState: 'something-and-other-13',
    },
  ],

  transitions: [
    {
      id: 'trans-12-1',
      from: 'something-and-other-12',
      to: 'something-and-other-13',
      mechanism: 'reflection',
      description: 'From identity to thing-in-itself critique',
    },
  ],

  nextStates: ['something-and-other-13'],
  previousStates: ['something-and-other-11'],

  provenance: {
    topicMapId: 'something-and-other-12',
    lineRange: { start: 199, end: 221 },
    section: 'a. Something and other',
    order: 12,
  },

  description: 'Both moments are determinations of one and the same, namely of something. Something is in-itself in so far as it has returned from the being-for-other back to itself.',
  keyPoints: [
    'Both moments determinations of one and same: something',
    'Identity: something has in it what it is in itself',
    'Something is one and same of both moments; undivided',
  ],
};

/**
 * State 13: In-itself as abstract - Thing-in-itself critique I
 *
 * Dialectic Position: What is only in itself is also only in it;
 * in-itself is merely abstract.
 */
const state13: DialecticState = {
  id: 'something-and-other-13',
  title: 'In-itself as abstract; thing-in-itself critique I',
  concept: 'InItselfAbstract',
  phase: 'quality',

  moments: [
    {
      name: 'inItself',
      definition: 'Merely abstract, hence external determination',
      type: 'determination',
    },
    {
      name: 'opinion',
      definition: 'In-itself something lofty (like inner)',
      type: 'determination',
    },
    {
      name: 'truth',
      definition: 'What is only in itself is also only in it',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-13-1',
      constraint: 'inItself.abstract = true',
      predicate: 'abstract(inItself)',
    },
    {
      id: 'inv-13-2',
      constraint: 'inItself.external = true',
      predicate: 'external(inItself)',
    },
    {
      id: 'inv-13-3',
      constraint: 'onlyInItself = onlyInIt',
      predicate: 'equals(onlyInItself, onlyInIt)',
    },
  ],

  forces: [
    {
      id: 'force-13-1',
      description: 'Abstract in-itself drives toward thing-in-itself critique',
      type: 'reflection',
      trigger: 'inItself.abstract = true',
      effect: 'thingInItself.critique = deepened',
      targetState: 'something-and-other-14',
    },
  ],

  transitions: [
    {
      id: 'trans-13-1',
      from: 'something-and-other-13',
      to: 'something-and-other-14',
      mechanism: 'reflection',
      description: 'From abstract in-itself to thing-in-itself critique',
    },
  ],

  nextStates: ['something-and-other-14'],
  previousStates: ['something-and-other-12'],

  provenance: {
    topicMapId: 'something-and-other-13',
    lineRange: { start: 222, end: 232 },
    section: 'a. Something and other',
    order: 13,
  },

  description: 'Opinion has it that with the in-itself something lofty is being said, as with the inner; but what something is only in itself, is also only in it; in-itself is a merely abstract, and hence itself external determination.',
  keyPoints: [
    'Opinion: in-itself something lofty (like inner)',
    'But what is only in itself is also only in it',
    'In-itself = merely abstract, hence external determination',
  ],
};

/**
 * State 14: Thing-in-itself critique II - Empty abstraction
 *
 * Dialectic Position: Thing-in-itself is empty abstraction void of truth;
 * impossible to know what it is.
 */
const state14: DialecticState = {
  id: 'something-and-other-14',
  title: 'Thing-in-itself critique II: empty abstraction',
  concept: 'ThingInItselfEmpty',
  phase: 'quality',

  moments: [
    {
      name: 'thingInItself',
      definition: 'Very simple abstraction, empty void of truth',
      type: 'determination',
    },
    {
      name: 'abstraction',
      definition: 'Thought without all determination, as nothing',
      type: 'negation',
    },
    {
      name: 'impossibility',
      definition: 'Impossible to know what thing-in-itself is',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'inv-14-1',
      constraint: 'thingInItself = simpleAbstraction',
      predicate: 'equals(thingInItself, simpleAbstraction)',
    },
    {
      id: 'inv-14-2',
      constraint: 'thingInItself.voidOfTruth = true',
      predicate: 'voidOfTruth(thingInItself)',
    },
    {
      id: 'inv-14-3',
      constraint: 'abstraction.from = allBeingForOther',
      predicate: 'from(abstraction, allBeingForOther)',
    },
    {
      id: 'inv-14-4',
      constraint: 'abstraction.result = nothing',
      predicate: 'equals(abstraction.result, nothing)',
    },
  ],

  forces: [
    {
      id: 'force-14-1',
      description: 'Empty abstraction drives toward truth of thing-in-itself',
      type: 'reflection',
      trigger: 'thingInItself.empty = true',
      effect: 'truthOfThingInItself.emerges = true',
      targetState: 'something-and-other-15',
    },
  ],

  transitions: [
    {
      id: 'trans-14-1',
      from: 'something-and-other-14',
      to: 'something-and-other-15',
      mechanism: 'reflection',
      description: 'From empty abstraction to truth of thing-in-itself',
    },
  ],

  nextStates: ['something-and-other-15'],
  previousStates: ['something-and-other-13'],

  provenance: {
    topicMapId: 'something-and-other-14',
    lineRange: { start: 234, end: 255 },
    section: 'a. Something and other',
    order: 14,
  },

  description: 'It may be observed that here we have the meaning of the thing-in-itself. It is a very simple abstraction, though it was for a while a very important determination, something sophisticated, as it were.',
  keyPoints: [
    'Here meaning of thing-in-itself',
    'Very simple abstraction (was important, sophisticated)',
    'Things "in-themselves": abstraction from all being-for-other',
    'Things-in-themselves = empty abstractions void of truth',
  ],
};

/**
 * State 15: Thing-in-itself in truth - Concept
 *
 * Dialectic Position: Thing-in-itself in truth is what something is in its concept;
 * concept is concrete, cognizable.
 */
const state15: DialecticState = {
  id: 'something-and-other-15',
  title: 'Thing-in-itself in truth: concept (concrete, cognizable)',
  concept: 'ThingInItselfConcept',
  phase: 'quality',

  moments: [
    {
      name: 'thingInItself',
      definition: 'What something is in its concept',
      type: 'determination',
    },
    {
      name: 'concept',
      definition: 'In itself concrete, conceptually graspable',
      type: 'determination',
    },
    {
      name: 'cognizable',
      definition: 'As determined and connected whole, inherently cognizable',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-15-1',
      constraint: 'thingInItself.truth = whatInConcept',
      predicate: 'equals(thingInItself.truth, whatInConcept)',
    },
    {
      id: 'inv-15-2',
      constraint: 'concept.concrete = true',
      predicate: 'concrete(concept)',
    },
    {
      id: 'inv-15-3',
      constraint: 'concept.graspable = true',
      predicate: 'graspable(concept)',
    },
    {
      id: 'inv-15-4',
      constraint: 'concept.cognizable = true',
      predicate: 'cognizable(concept)',
    },
  ],

  forces: [
    {
      id: 'force-15-1',
      description: 'Concept drives toward positedness',
      type: 'reflection',
      trigger: 'concept.established = true',
      effect: 'positedness.emerges = true',
      targetState: 'something-and-other-16',
    },
  ],

  transitions: [
    {
      id: 'trans-15-1',
      from: 'something-and-other-15',
      to: 'something-and-other-16',
      mechanism: 'reflection',
      description: 'From concept to positedness',
    },
  ],

  nextStates: ['something-and-other-16'],
  previousStates: ['something-and-other-14'],

  provenance: {
    topicMapId: 'something-and-other-15',
    lineRange: { start: 256, end: 264 },
    section: 'a. Something and other',
    order: 15,
  },

  description: 'What, however, the thing-in-itself in truth is, what there basically is in it, of this the Logic is the exposition. But in this Logic something better is understood by the in-itself than an abstraction, namely, what something is in its concept.',
  keyPoints: [
    'Thing-in-itself in truth: Logic is exposition',
    'In Logic, in-itself = what something is in its concept',
    'Concept is in itself concrete',
    'As determined and connected whole: inherently cognizable',
  ],
};

/**
 * State 16: Positedness - Transition vs positing
 *
 * Dialectic Position: In being, self-determining is only in itself (implicit) = transition;
 * in essence, positing occurs.
 */
const state16: DialecticState = {
  id: 'something-and-other-16',
  title: 'Positedness; transition vs positing; in-itself vs posited',
  concept: 'Positedness',
  phase: 'quality',

  moments: [
    {
      name: 'positedness',
      definition: 'Bending back of what is not in itself into being-in-itself',
      type: 'determination',
    },
    {
      name: 'transition',
      definition: 'In being: self-determining only in itself (implicit)',
      type: 'process',
    },
    {
      name: 'positing',
      definition: 'In essence: ground posits grounded, cause produces effect',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'inv-16-1',
      constraint: 'inBeing.selfDetermining = implicit',
      predicate: 'equals(inBeing.selfDetermining, implicit)',
    },
    {
      id: 'inv-16-2',
      constraint: 'inBeing.mechanism = transition',
      predicate: 'equals(inBeing.mechanism, transition)',
    },
    {
      id: 'inv-16-3',
      constraint: 'inEssence.mechanism = positing',
      predicate: 'equals(inEssence.mechanism, positing)',
    },
    {
      id: 'inv-16-4',
      constraint: 'distinction.inItselfVsPosited = capital',
      predicate: 'capital(distinction.inItselfVsPosited)',
    },
  ],

  forces: [
    {
      id: 'force-16-1',
      description: 'Positedness drives toward determination as quality',
      type: 'passover',
      trigger: 'positedness.established = true',
      effect: 'determination.asQuality = true',
      targetState: 'something-and-other-17',
    },
  ],

  transitions: [
    {
      id: 'trans-16-1',
      from: 'something-and-other-16',
      to: 'something-and-other-17',
      mechanism: 'passover',
      description: 'From positedness to determination as quality',
    },
  ],

  nextStates: ['something-and-other-17'],
  previousStates: ['something-and-other-15'],

  provenance: {
    topicMapId: 'something-and-other-16',
    lineRange: { start: 266, end: 315 },
    section: 'a. Something and other',
    order: 16,
  },

  description: 'Being-in-itself is normally to be taken as an abstract way of expressing the concept; positing, strictly speaking, first occurs in the sphere of essence, of objective reflection.',
  keyPoints: [
    'Being-in-itself = abstract expression of concept',
    'Positing: strictly in essence (objective reflection)',
    'In being: self-determining only in itself (implicit) = transition',
    'Capital concern: distinguish in-itself (implicit) vs posited',
  ],
};

/**
 * State 17: Identity in something - Determination as quality
 *
 * Dialectic Position: In unity of something with itself, being-for-other
 * identical with in-itself; determinateness reflected = quality: determination.
 */
const state17: DialecticState = {
  id: 'something-and-other-17',
  title: 'Identity in something; determination as quality',
  concept: 'DeterminationAsQuality',
  phase: 'quality',

  moments: [
    {
      name: 'unity',
      definition: 'Unity of something with itself',
      type: 'mediation',
    },
    {
      name: 'identity',
      definition: 'Being-for-other identical with in-itself',
      type: 'mediation',
    },
    {
      name: 'determination',
      definition: 'Determinateness reflected into itself = quality',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'inv-17-1',
      constraint: 'unity.of = somethingWithItself',
      predicate: 'of(unity, somethingWithItself)',
    },
    {
      id: 'inv-17-2',
      constraint: 'beingForOther.identicalWith = beingInItself',
      predicate: 'identicalWith(beingForOther, beingInItself)',
    },
    {
      id: 'inv-17-3',
      constraint: 'determinateness.reflected = simpleExistent',
      predicate: 'reflected(determinateness, simpleExistent)',
    },
    {
      id: 'inv-17-4',
      constraint: 'determination = quality',
      predicate: 'equals(determination, quality)',
    },
  ],

  forces: [
    {
      id: 'force-17-1',
      description: 'Determination as quality completes something-and-other',
      type: 'sublation',
      trigger: 'determination.asQuality = true',
      effect: 'somethingAndOther.complete = true',
      targetState: 'constitution-1',
    },
  ],

  transitions: [
    {
      id: 'trans-17-1',
      from: 'something-and-other-17',
      to: 'constitution-1',
      mechanism: 'passover',
      description: 'From determination as quality to constitution',
    },
  ],

  nextStates: ['constitution-1'],
  previousStates: ['something-and-other-16'],

  provenance: {
    topicMapId: 'something-and-other-17',
    lineRange: { start: 324, end: 329 },
    section: 'a. Something and other',
    order: 17,
  },

  description: 'In the unity of the something with itself, being-for-other is identical with its in-itself; the being-for-other is thus in the something. The determinateness thus reflected into itself is therefore again a simple existent and hence again a quality: determination.',
  keyPoints: [
    'In unity of something with itself, being-for-other identical with in-itself',
    'Being-for-other thus in the something',
    'Determinateness reflected into itself = simple existent',
    'Hence again quality: determination',
  ],
};

/**
 * Complete Something and Other IR Document
 *
 * This is the executable pseudo-code representation of the entire
 * "Something and other" section.
 */
export const somethingAndOtherIR: DialecticIR = {
  id: 'something-and-other-ir',
  title: 'Something and Other IR: Something and Other',
  section: 'B. FINITUDE (a) Something and other',

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
    state17,
  ],

  metadata: {
    sourceFile: 'something-and-other.txt',
    totalStates: 17,
    cpuGpuMapping: {
      'something-and-other-1': 'quality',
      'something-and-other-2': 'quality',
      'something-and-other-3': 'quality',
      'something-and-other-4': 'quality',
      'something-and-other-5': 'quality',
      'something-and-other-6': 'quality',
      'something-and-other-7': 'quality',
      'something-and-other-8': 'quality',
      'something-and-other-9': 'quality',
      'something-and-other-10': 'quality',
      'something-and-other-11': 'quality',
      'something-and-other-12': 'quality',
      'something-and-other-13': 'quality',
      'something-and-other-14': 'quality',
      'something-and-other-15': 'quality',
      'something-and-other-16': 'quality',
      'something-and-other-17': 'quality',
    },
  },
};

/**
 * Export individual states for programmatic access
 */
export const somethingAndOtherStates = {
  'something-and-other-1': state1,
  'something-and-other-2': state2,
  'something-and-other-3': state3,
  'something-and-other-4': state4,
  'something-and-other-5': state5,
  'something-and-other-6': state6,
  'something-and-other-7': state7,
  'something-and-other-8': state8,
  'something-and-other-9': state9,
  'something-and-other-10': state10,
  'something-and-other-11': state11,
  'something-and-other-12': state12,
  'something-and-other-13': state13,
  'something-and-other-14': state14,
  'something-and-other-15': state15,
  'something-and-other-16': state16,
  'something-and-other-17': state17,
};

