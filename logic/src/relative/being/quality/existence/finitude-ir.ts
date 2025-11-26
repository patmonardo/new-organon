/**
 * Finitude IR: Dialectic Pseudo-Code for Finitude
 *
 * This file demonstrates the conversion from TopicMap + Chunks → Dialectic IR
 * as executable pseudo-code that advances the dialectic.
 *
 * Architecture: CPU (Quality/Being → Reflection → Subject)
 * Section: B. FINITUDE (c) Finitude
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
import { finitudeTopicMap } from './sources/finitude-topic-map';

/**
 * State 1: Introduction - Definition of finitude
 *
 * Dialectic Position: Non-being constitutes the nature of finite things;
 * hour of birth = hour of death.
 */
const state1: DialecticState = {
  id: 'finitude-1',
  title: 'Introduction: definition of finitude; non-being as nature; perishing; hour of birth = hour of death',
  concept: 'Finitude',
  phase: 'quality', // CPU: Quality/Being

  moments: [
    {
      name: 'existence',
      definition: 'Determinate; something has quality, delimited',
      type: 'determination',
    },
    {
      name: 'limit',
      definition: 'Quality is limit, immanent to existence',
      type: 'determination',
    },
    {
      name: 'finitude',
      definition: 'Negation constituting finitude; non-being as nature',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'inv-1-1',
      constraint: 'existence.determinate = true',
      predicate: 'determinate(existence)',
    },
    {
      id: 'inv-1-2',
      constraint: 'quality = limit',
      predicate: 'equals(quality, limit)',
    },
    {
      id: 'inv-1-3',
      constraint: 'opposition.existence.and.negation = inItselfness',
      predicate: 'equals(opposition(existence, negation), inItselfness)',
    },
    {
      id: 'inv-1-4',
      constraint: 'nonBeing.constitutes = nature',
      predicate: 'constitutes(nonBeing, nature)',
    },
    {
      id: 'inv-1-5',
      constraint: 'hourOfBirth = hourOfDeath',
      predicate: 'equals(hourOfBirth, hourOfDeath)',
    },
  ],

  forces: [
    {
      id: 'force-1-1',
      description: 'Finitude drives toward qualitative negation',
      type: 'negation',
      trigger: 'finitude.established = true',
      effect: 'qualitativeNegation.extreme = true',
      targetState: 'finitude-2',
    },
  ],

  transitions: [
    {
      id: 'trans-1-1',
      from: 'finitude-1',
      to: 'finitude-2',
      mechanism: 'negation',
      description: 'From definition to immediacy of finitude',
    },
  ],

  nextStates: ['finitude-2'],
  previousStates: ['constitution-16'],

  provenance: {
    topicMapId: 'finitude-1',
    lineRange: { start: 4, end: 37 },
    section: 'c. Finitude',
    order: 1,
  },

  description: 'Existence is determinate. Something has a quality, and in this quality it is not only determined but delimited; its quality is its limit and, affected by it, something remains affirmative, quiescent existence.',
  keyPoints: [
    'Existence determinate; something has quality; delimited; quality is limit',
    'Opposition of existence and negation (limit immanent) = in-itselfness',
    'This negation = finitude',
    'Finite things: non-being constitutes their nature',
    'Hour of birth = hour of death',
  ],
};

/**
 * State 2: Immediacy of finitude I - Mournful note
 *
 * Dialectic Position: Finitude is qualitative negation driven to extreme;
 * understanding persists in sorrow.
 */
const state2: DialecticState = {
  id: 'finitude-2',
  title: "Immediacy of finitude I: mournful note; qualitative negation driven to extreme; understanding's persistence",
  concept: 'FinitudeMournful',
  phase: 'quality',

  moments: [
    {
      name: 'qualitativeNegation',
      definition: 'Negation driven to extreme',
      type: 'negation',
    },
    {
      name: 'understanding',
      definition: 'Persists in sorrow, makes non-being imperishable',
      type: 'determination',
    },
    {
      name: 'finitude',
      definition: 'Most obstinate category; negation fixed in itself',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'inv-2-1',
      constraint: 'finitude = qualitativeNegation.extreme',
      predicate: 'equals(finitude, extreme(qualitativeNegation))',
    },
    {
      id: 'inv-2-2',
      constraint: 'noAffirmativeBeing.distinct = true',
      predicate: 'noAffirmativeBeing(distinct)',
    },
    {
      id: 'inv-2-3',
      constraint: 'finitude.obstinate = true',
      predicate: 'obstinate(finitude)',
    },
    {
      id: 'inv-2-4',
      constraint: 'understanding.makesNonBeing.imperishable = true',
      predicate: 'makesImperishable(understanding, nonBeing)',
    },
    {
      id: 'inv-2-5',
      constraint: 'finitude.eternal = true',
      predicate: 'eternal(finitude)',
    },
  ],

  forces: [
    {
      id: 'force-2-1',
      description: 'Understanding\'s persistence drives toward view of finite as absolute',
      type: 'reflection',
      trigger: 'understanding.persists = true',
      effect: 'finite.asAbsolute.questioned = true',
      targetState: 'finitude-3',
    },
  ],

  transitions: [
    {
      id: 'trans-2-1',
      from: 'finitude-2',
      to: 'finitude-3',
      mechanism: 'reflection',
      description: 'From mournful note to question of finite as absolute',
    },
  ],

  nextStates: ['finitude-3'],
  previousStates: ['finitude-1'],

  provenance: {
    topicMapId: 'finitude-2',
    lineRange: { start: 41, end: 79 },
    section: '(a) The immediacy of finitude',
    order: 2,
  },

  description: 'The thought of the finitude of things brings this mournful note with it because finitude is qualitative negation driven to the extreme, and in the simplicity of such a determination there is no longer left to things an affirmative being distinct from their determination.',
  keyPoints: [
    'Thought of finitude brings mournful note',
    'Finitude = qualitative negation driven to extreme',
    'Finitude = most obstinate category of understanding',
    'Understanding persists in sorrow: makes non-being imperishable/absolute',
    'Finitude does not pass over into affirmative; finitude is eternal',
  ],
};

/**
 * State 3: Immediacy of finitude II - Finite as absolute?
 *
 * Dialectic Position: Finite as absolute not endorsed; all depends on view.
 */
const state3: DialecticState = {
  id: 'finitude-3',
  title: 'Immediacy of finitude II: finite as absolute? No—depends on view',
  concept: 'FiniteAsAbsolute',
  phase: 'quality',

  moments: [
    {
      name: 'finite',
      definition: 'Restricted, perishable, only finite, not imperishable',
      type: 'determination',
    },
    {
      name: 'view',
      definition: 'Whether finitude\'s being insisted on OR transitoriness perishes',
      type: 'determination',
    },
    {
      name: 'perishing',
      definition: 'Perishing of perishing does not happen on view making perishing final end',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'inv-3-1',
      constraint: 'finite.absolute = notEndorsed',
      predicate: 'notEndorsed(finite.absolute)',
    },
    {
      id: 'inv-3-2',
      constraint: 'finite = restricted.perishable.onlyFinite',
      predicate: 'equals(finite, restricted.perishable.onlyFinite)',
    },
    {
      id: 'inv-3-3',
      constraint: 'allDependsOn = view',
      predicate: 'dependsOn(all, view)',
    },
  ],

  forces: [
    {
      id: 'force-3-1',
      description: 'View drives toward contradiction with infinite',
      type: 'contradiction',
      trigger: 'view.established = true',
      effect: 'contradictionWithInfinite.emerges = true',
      targetState: 'finitude-4',
    },
  ],

  transitions: [
    {
      id: 'trans-3-1',
      from: 'finitude-3',
      to: 'finitude-4',
      mechanism: 'contradiction',
      description: 'From view to contradiction with infinite',
    },
  ],

  nextStates: ['finitude-4'],
  previousStates: ['finitude-2'],

  provenance: {
    topicMapId: 'finitude-3',
    lineRange: { start: 81, end: 94 },
    section: '(a) The immediacy of finitude',
    order: 3,
  },

  description: 'But that the finite is absolute is certainly not a standpoint that any philosophy or outlook, or the understanding, would want to endorse. The opposite is rather expressly present in the assertion of finitude: the finite is the restricted, the perishable, the finite is only the finite, not the imperishable.',
  keyPoints: [
    'Important consideration',
    'Finite as absolute not standpoint any philosophy/understanding would endorse',
    'All depends on: whether finitude\'s being insisted on OR transitoriness perishes',
  ],
};

/**
 * State 4: Immediacy of finitude III - Contradiction with infinite
 *
 * Dialectic Position: Finite incompatible with infinite; perishing of perishing.
 */
const state4: DialecticState = {
  id: 'finitude-4',
  title: 'Immediacy of finitude III: contradiction with infinite; perishing of perishing',
  concept: 'FiniteInfiniteContradiction',
  phase: 'quality',

  moments: [
    {
      name: 'finite',
      definition: 'Incompatible with infinite, absolutely opposed',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'infinite',
    },
    {
      name: 'infinite',
      definition: 'Absolute being ascribed to it',
      type: 'determination',
      relation: 'opposite',
      relatedTo: 'finite',
    },
    {
      name: 'perishing',
      definition: 'Perishing of perishing; not the last of it',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'inv-4-1',
      constraint: 'finite.incompatibleWith = infinite',
      predicate: 'incompatibleWith(finite, infinite)',
    },
    {
      id: 'inv-4-2',
      constraint: 'finite.absoluteOpposition = infinite',
      predicate: 'absoluteOpposition(finite, infinite)',
    },
    {
      id: 'inv-4-3',
      constraint: 'finite.contradiction = itself',
      predicate: 'contradiction(finite, itself)',
    },
    {
      id: 'inv-4-4',
      constraint: 'perishing.perishes = true',
      predicate: 'perishes(perishing)',
    },
  ],

  forces: [
    {
      id: 'force-4-1',
      description: 'Contradiction drives toward restriction and ought',
      type: 'contradiction',
      trigger: 'contradiction.expressed = true',
      effect: 'restrictionAndOught.emerges = true',
      targetState: 'finitude-5',
    },
  ],

  transitions: [
    {
      id: 'trans-4-1',
      from: 'finitude-4',
      to: 'finitude-5',
      mechanism: 'contradiction',
      description: 'From contradiction to restriction and ought',
    },
  ],

  nextStates: ['finitude-5'],
  previousStates: ['finitude-3'],

  provenance: {
    topicMapId: 'finitude-4',
    lineRange: { start: 98, end: 137 },
    section: '(a) The immediacy of finitude',
    order: 4,
  },

  description: 'The official claim is that the finite is incompatible with the infinite and cannot be united with it; that the finite is absolutely opposed to the infinite. Being, absolute being, is ascribed to the infinite.',
  keyPoints: [
    'Official claim: finite incompatible with infinite, cannot be united',
    'Finite absolutely opposed to infinite',
    'Development will show: finite as contradiction collapses internally',
    'Perishing rather perishes',
  ],
};

/**
 * State 5: Restriction and ought I - Contradiction abstractly present
 *
 * Dialectic Position: Limit as constituting what is immanent = finitude.
 */
const state5: DialecticState = {
  id: 'finitude-5',
  title: 'Restriction and ought I: contradiction abstractly present; limit as finitude',
  concept: 'LimitAsFinitude',
  phase: 'quality',

  moments: [
    {
      name: 'contradiction',
      definition: 'Abstractly present by fact that something is finite',
      type: 'contradiction',
    },
    {
      name: 'beingInItself',
      definition: 'Reflected into itself, developed with determination and constitution',
      type: 'determination',
    },
    {
      name: 'limit',
      definition: 'Constituting what is immanent to something and quality of its being-in-itself = finitude',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-5-1',
      constraint: 'contradiction.abstractlyPresent = true',
      predicate: 'abstractlyPresent(contradiction)',
    },
    {
      id: 'inv-5-2',
      constraint: 'beingInItself.reflected = true',
      predicate: 'reflected(beingInItself)',
    },
    {
      id: 'inv-5-3',
      constraint: 'limit.within = something',
      predicate: 'within(limit, something)',
    },
    {
      id: 'inv-5-4',
      constraint: 'limit = finitude',
      predicate: 'equals(limit, finitude)',
    },
  ],

  forces: [
    {
      id: 'force-5-1',
      description: 'Limit as finitude drives toward determination and constitution',
      type: 'mediation',
      trigger: 'limit.asFinitude = true',
      effect: 'determinationAndConstitution.emerges = true',
      targetState: 'finitude-6',
    },
  ],

  transitions: [
    {
      id: 'trans-5-1',
      from: 'finitude-5',
      to: 'finitude-6',
      mechanism: 'mediation',
      description: 'From limit as finitude to determination and constitution',
    },
  ],

  nextStates: ['finitude-6'],
  previousStates: ['finitude-4'],

  provenance: {
    topicMapId: 'finitude-5',
    lineRange: { start: 141, end: 155 },
    section: '(b) Restriction and the ought',
    order: 5,
  },

  description: 'This contradiction is indeed abstractly present by the very fact that the something is finite, or that the finite is. But something or being is no longer posited abstractly but reflected into itself, and developed as being-in-itself that has determination and constitution in it.',
  keyPoints: [
    'Contradiction abstractly present by fact that something is finite',
    'Something/being no longer posited abstractly but reflected into itself',
    'Limit as constituting what is immanent to something and quality of its being-in-itself = finitude',
  ],
};

/**
 * State 6: Restriction and ought II - Determination and constitution
 *
 * Dialectic Position: Otherness immanent = connection of two sides;
 * in-itself as negative reference to limit = ought.
 */
const state6: DialecticState = {
  id: 'finitude-6',
  title: 'Restriction and ought II: determination and constitution; limit as connection; restriction and ought',
  concept: 'RestrictionAndOught',
  phase: 'quality',

  moments: [
    {
      name: 'determination',
      definition: 'Contains otherness as belonging to in-itself',
      type: 'determination',
    },
    {
      name: 'constitution',
      definition: 'Externality of otherness within something\'s inwardness',
      type: 'determination',
    },
    {
      name: 'limit',
      definition: 'Negation of negation, connection of two sides',
      type: 'mediation',
    },
    {
      name: 'restriction',
      definition: 'Something\'s own limit posited as negative which is essential',
      type: 'negation',
    },
    {
      name: 'ought',
      definition: 'In-itself as negative reference to limit (distinguished from it) = ought',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-6-1',
      constraint: 'otherness.immanent = connectionOfTwoSides',
      predicate: 'equals(otherness.immanent, connectionOfTwoSides)',
    },
    {
      id: 'inv-6-2',
      constraint: 'limit = negationOfNegation',
      predicate: 'equals(limit, negationOfNegation)',
    },
    {
      id: 'inv-6-3',
      constraint: 'restriction = limit.positedAsNegative.essential',
      predicate: 'equals(restriction, essential(negative(limit)))',
    },
    {
      id: 'inv-6-4',
      constraint: 'ought = inItself.negativeReferenceToLimit',
      predicate: 'equals(ought, negativeReference(inItself, limit))',
    },
  ],

  forces: [
    {
      id: 'force-6-1',
      description: 'Ought drives toward transcendence of restriction',
      type: 'sublation',
      trigger: 'ought.established = true',
      effect: 'ought.transcendsRestriction = true',
      targetState: 'finitude-7',
    },
  ],

  transitions: [
    {
      id: 'trans-6-1',
      from: 'finitude-6',
      to: 'finitude-7',
      mechanism: 'sublation',
      description: 'From ought to transcendence of restriction',
    },
  ],

  nextStates: ['finitude-7'],
  previousStates: ['finitude-5'],

  provenance: {
    topicMapId: 'finitude-6',
    lineRange: { start: 157, end: 189 },
    section: '(b) Restriction and the ought',
    order: 6,
  },

  description: 'Determination and constitution arose as sides for external reflection, but determination already contained otherness as belonging to the in-itself of something. On the one side, the externality of otherness is within the something\'s own inwardness.',
  keyPoints: [
    'Otherness determined as limit (negation of negation)',
    'Otherness immanent = connection of two sides',
    'Something\'s own limit posited as negative which is essential = restriction',
    'In-itself as negative reference to limit = ought',
  ],
};

/**
 * State 7: Restriction and ought III - Ought transcends restriction
 *
 * Dialectic Position: Something transcends limit only as sublatedness of limit;
 * something thereby transcends itself.
 */
const state7: DialecticState = {
  id: 'finitude-7',
  title: 'Restriction and ought III: ought transcends restriction; restriction only as ought',
  concept: 'OughtTranscends',
  phase: 'quality',

  moments: [
    {
      name: 'limit',
      definition: 'To be restriction: something must transcend it in itself',
      type: 'determination',
    },
    {
      name: 'transcendence',
      definition: 'Something transcends limit only as sublatedness of limit',
      type: 'sublation',
    },
    {
      name: 'selfTranscendence',
      definition: 'Since limit as restriction in determination: something transcends itself',
      type: 'sublation',
    },
  ],

  invariants: [
    {
      id: 'inv-7-1',
      constraint: 'forLimitToBeRestriction = something.mustTranscend',
      predicate: 'mustTranscend(something, limit)',
    },
    {
      id: 'inv-7-2',
      constraint: 'transcendence.onlyAs = sublatednessOfLimit',
      predicate: 'onlyAs(transcendence, sublatednessOfLimit)',
    },
    {
      id: 'inv-7-3',
      constraint: 'something.transcendsItself = true',
      predicate: 'transcendsItself(something)',
    },
  ],

  forces: [
    {
      id: 'force-7-1',
      description: 'Self-transcendence drives toward double determination',
      type: 'sublation',
      trigger: 'selfTranscendence.established = true',
      effect: 'doubleDetermination.emerges = true',
      targetState: 'finitude-8',
    },
  ],

  transitions: [
    {
      id: 'trans-7-1',
      from: 'finitude-7',
      to: 'finitude-8',
      mechanism: 'sublation',
      description: 'From transcendence to double determination',
    },
  ],

  nextStates: ['finitude-8'],
  previousStates: ['finitude-6'],

  provenance: {
    topicMapId: 'finitude-7',
    lineRange: { start: 191, end: 201 },
    section: '(b) Restriction and the ought',
    order: 7,
  },

  description: 'In order for the limit that is in every something to be a restriction, the something must at the same time transcend it in itself, must refer to it from within as to a non-existent.',
  keyPoints: [
    'For limit to be restriction: something must transcend it in itself',
    'Something transcends limit only as sublatedness of limit',
    'Since limit as restriction in determination: something transcends itself',
  ],
};

/**
 * State 8: Restriction and ought IV - Ought's double determination
 *
 * Dialectic Position: Ought contains double determination; finite = ought + restriction.
 */
const state8: DialecticState = {
  id: 'finitude-8',
  title: "Restriction and ought IV: ought's double determination; finite = ought + restriction",
  concept: 'OughtDoubleDetermination',
  phase: 'quality',

  moments: [
    {
      name: 'ought',
      definition: 'Contains double determination: (1) determination with in-itselfness; (2) non-being as restriction',
      type: 'determination',
    },
    {
      name: 'restriction',
      definition: 'Posited as finite',
      type: 'negation',
    },
    {
      name: 'finite',
      definition: 'Connecting determination and limit; determination = ought, limit = restriction',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-8-1',
      constraint: 'ought.doubleDetermination = true',
      predicate: 'doubleDetermination(ought)',
    },
    {
      id: 'inv-8-2',
      constraint: 'finite = connecting(determination, limit)',
      predicate: 'equals(finite, connecting(determination, limit))',
    },
    {
      id: 'inv-8-3',
      constraint: 'determination = ought',
      predicate: 'equals(determination, ought)',
    },
    {
      id: 'inv-8-4',
      constraint: 'limit = restriction',
      predicate: 'equals(limit, restriction)',
    },
    {
      id: 'inv-8-5',
      constraint: 'bothMoments.finite = true',
      predicate: 'finite(bothMoments)',
    },
  ],

  forces: [
    {
      id: 'force-8-1',
      description: 'Double determination drives toward what ought to be',
      type: 'mediation',
      trigger: 'doubleDetermination.established = true',
      effect: 'whatOughtToBe.emerges = true',
      targetState: 'finitude-9',
    },
  ],

  transitions: [
    {
      id: 'trans-8-1',
      from: 'finitude-8',
      to: 'finitude-9',
      mechanism: 'mediation',
      description: 'From double determination to what ought to be',
    },
  ],

  nextStates: ['finitude-9'],
  previousStates: ['finitude-7'],

  provenance: {
    topicMapId: 'finitude-8',
    lineRange: { start: 203, end: 227 },
    section: '(b) Restriction and the ought',
    order: 8,
  },

  description: 'The ought therefore contains the double determination: once, as a determination which has an in-itselfness over against negation; and again, as a non-being which, as restriction, is distinguished from the determination but is at the same time itself a determination existing in itself.',
  keyPoints: [
    'Ought contains double determination',
    'Finite determined as connecting determination and limit',
    'In connection: determination = ought, limit = restriction',
    'Both moments of finite, therefore both finite',
  ],
};

/**
 * State 9: Restriction and ought V - What ought to be is and is not
 *
 * Dialectic Position: What ought to be is, and at the same time is not;
 * restriction not alien.
 */
const state9: DialecticState = {
  id: 'finitude-9',
  title: 'Restriction and ought V: what ought to be is and is not; restriction not alien',
  concept: 'WhatOughtToBe',
  phase: 'quality',

  moments: [
    {
      name: 'whatOughtToBe',
      definition: 'Is, and at the same time is not',
      type: 'determination',
    },
    {
      name: 'restriction',
      definition: 'Not anything alien; finite\'s own determination also its restriction',
      type: 'negation',
    },
    {
      name: 'identity',
      definition: 'Restriction = both itself and ought; common to both',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'inv-9-1',
      constraint: 'whatOughtToBe.is = true',
      predicate: 'is(whatOughtToBe)',
    },
    {
      id: 'inv-9-2',
      constraint: 'whatOughtToBe.isNot = true',
      predicate: 'isNot(whatOughtToBe)',
    },
    {
      id: 'inv-9-3',
      constraint: 'restriction.notAlien = true',
      predicate: 'notAlien(restriction)',
    },
    {
      id: 'inv-9-4',
      constraint: 'restriction = both(itself, ought)',
      predicate: 'equals(restriction, both(itself, ought))',
    },
  ],

  forces: [
    {
      id: 'force-9-1',
      description: 'Identity drives toward transcendence',
      type: 'sublation',
      trigger: 'identity.established = true',
      effect: 'transcendence.emerges = true',
      targetState: 'finitude-10',
    },
  ],

  transitions: [
    {
      id: 'trans-9-1',
      from: 'finitude-9',
      to: 'finitude-10',
      mechanism: 'sublation',
      description: 'From identity to transcendence',
    },
  ],

  nextStates: ['finitude-10'],
  previousStates: ['finitude-8'],

  provenance: {
    topicMapId: 'finitude-9',
    lineRange: { start: 229, end: 253 },
    section: '(b) Restriction and the ought',
    order: 9,
  },

  description: 'What ought to be is, and at the same time is not. If it were, it would not be what merely ought to be. The ought has therefore a restriction essentially.',
  keyPoints: [
    'What ought to be is, and at same time is not',
    'Ought has restriction essentially',
    'Restriction not anything alien',
    'Restriction = both itself and ought; common to both',
  ],
};

/**
 * State 10: Restriction and ought VI - Ought transcends; indivisible
 *
 * Dialectic Position: As ought, finite transcends restriction;
 * but has restriction only as ought; two indivisible.
 */
const state10: DialecticState = {
  id: 'finitude-10',
  title: 'Restriction and ought VI: ought transcends restriction; restriction only as ought; indivisible',
  concept: 'OughtIndivisible',
  phase: 'quality',

  moments: [
    {
      name: 'ought',
      definition: 'Finite transcends restriction as ought',
      type: 'determination',
    },
    {
      name: 'restriction',
      definition: 'Finite has restriction only as ought',
      type: 'negation',
    },
    {
      name: 'indivisible',
      definition: 'The two are indivisible',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'inv-10-1',
      constraint: 'asOught.finite.transcends = restriction',
      predicate: 'transcends(asOught(finite), restriction)',
    },
    {
      id: 'inv-10-2',
      constraint: 'finite.hasRestriction.onlyAs = ought',
      predicate: 'onlyAs(hasRestriction(finite), ought)',
    },
    {
      id: 'inv-10-3',
      constraint: 'two.indivisible = true',
      predicate: 'indivisible(two)',
    },
    {
      id: 'inv-10-4',
      constraint: 'determination = beingSublatedOfRestriction',
      predicate: 'equals(determination, beingSublatedOf(restriction))',
    },
  ],

  forces: [
    {
      id: 'force-10-1',
      description: 'Indivisibility drives toward transition',
      type: 'mediation',
      trigger: 'indivisible.established = true',
      effect: 'transition.emerges = true',
      targetState: 'finitude-11',
    },
  ],

  transitions: [
    {
      id: 'trans-10-1',
      from: 'finitude-10',
      to: 'finitude-11',
      mechanism: 'mediation',
      description: 'From indivisibility to transition',
    },
  ],

  nextStates: ['finitude-11'],
  previousStates: ['finitude-9'],

  provenance: {
    topicMapId: 'finitude-10',
    lineRange: { start: 255, end: 266 },
    section: '(b) Restriction and the ought',
    order: 10,
  },

  description: 'But further, as "ought" the finite transcends its restriction; the same determinateness which is its negation is also sublated, and is thus its in-itself; its limit is also not its limit.',
  keyPoints: [
    'As "ought" finite transcends restriction',
    'But conversely has restriction only as ought',
    'Two indivisible',
  ],
};

/**
 * State 11: Transition I - Ought and restriction mutually connected
 *
 * Dialectic Position: Their mutual connection = finite itself;
 * finite is contradiction of itself.
 */
const state11: DialecticState = {
  id: 'finitude-11',
  title: 'Transition I: ought and restriction mutually connected; finite as contradiction',
  concept: 'FiniteAsContradiction',
  phase: 'quality',

  moments: [
    {
      name: 'ought',
      definition: 'Contains restriction explicitly, for itself',
      type: 'determination',
    },
    {
      name: 'restriction',
      definition: 'Contains ought',
      type: 'negation',
    },
    {
      name: 'finite',
      definition: 'Mutual connection; contains both in in-itself; contradiction of itself',
      type: 'contradiction',
    },
  ],

  invariants: [
    {
      id: 'inv-11-1',
      constraint: 'ought.contains = restriction',
      predicate: 'contains(ought, restriction)',
    },
    {
      id: 'inv-11-2',
      constraint: 'restriction.contains = ought',
      predicate: 'contains(restriction, ought)',
    },
    {
      id: 'inv-11-3',
      constraint: 'mutualConnection = finite',
      predicate: 'equals(mutualConnection, finite)',
    },
    {
      id: 'inv-11-4',
      constraint: 'finite.contradiction = itself',
      predicate: 'contradiction(finite, itself)',
    },
    {
      id: 'inv-11-5',
      constraint: 'finite.sublatesItself = true',
      predicate: 'sublatesItself(finite)',
    },
  ],

  forces: [
    {
      id: 'force-11-1',
      description: 'Contradiction drives toward result: other finite or infinite',
      type: 'contradiction',
      trigger: 'contradiction.expressed = true',
      effect: 'result.emerges = true',
      targetState: 'finitude-12',
    },
  ],

  transitions: [
    {
      id: 'trans-11-1',
      from: 'finitude-11',
      to: 'finitude-12',
      mechanism: 'contradiction',
      description: 'From contradiction to result (a): other finite',
    },
    {
      id: 'trans-11-2',
      from: 'finitude-11',
      to: 'finitude-13',
      mechanism: 'sublation',
      description: 'From contradiction to result (b): infinite',
    },
  ],

  nextStates: ['finitude-12', 'finitude-13'],
  previousStates: ['finitude-10'],

  provenance: {
    topicMapId: 'finitude-11',
    lineRange: { start: 270, end: 277 },
    section: '(c) Transition of the finite into the infinite',
    order: 11,
  },

  description: 'The ought contains restriction explicitly, for itself, and restriction contains the ought. Their mutual connection is the finite itself, which contains them both in its in-itself.',
  keyPoints: [
    'Ought contains restriction explicitly, for itself',
    'Restriction contains ought',
    'Mutual connection = finite itself',
    'Finite in itself = contradiction of itself',
    'Sublates itself, goes away, ceases to be',
  ],
};

/**
 * State 12: Transition II - Result (a): Other finite (infinite progression)
 *
 * Dialectic Position: In going away, finite has not ceased;
 * only become momentarily other finite → infinity.
 */
const state12: DialecticState = {
  id: 'finitude-12',
  title: 'Transition II: result (a) other finite (infinite progression)',
  concept: 'InfiniteProgression',
  phase: 'quality',

  moments: [
    {
      name: 'result',
      definition: 'Negative as such, its very determination (negative of negative)',
      type: 'negation',
    },
    {
      name: 'otherFinite',
      definition: 'Finite has only become momentarily other finite',
      type: 'determination',
    },
    {
      name: 'infinity',
      definition: 'Going-over into another finite, and so forth to infinity',
      type: 'process',
    },
  ],

  invariants: [
    {
      id: 'inv-12-1',
      constraint: 'result = negativeOfNegative',
      predicate: 'equals(result, negativeOfNegative)',
    },
    {
      id: 'inv-12-2',
      constraint: 'finite.hasNotCeased = true',
      predicate: 'hasNotCeased(finite)',
    },
    {
      id: 'inv-12-3',
      constraint: 'finite.become = otherFinite',
      predicate: 'become(finite, otherFinite)',
    },
    {
      id: 'inv-12-4',
      constraint: 'goingOver = infinity',
      predicate: 'equals(goingOver, infinity)',
    },
  ],

  forces: [
    {
      id: 'force-12-1',
      description: 'Infinite progression drives toward identity with itself',
      type: 'passover',
      trigger: 'infinity.progression = true',
      effect: 'identityWithItself.emerges = true',
      targetState: 'finitude-13',
    },
  ],

  transitions: [
    {
      id: 'trans-12-1',
      from: 'finitude-12',
      to: 'finitude-13',
      mechanism: 'sublation',
      description: 'From infinite progression to identity with itself (infinite)',
    },
  ],

  nextStates: ['finitude-13'],
  previousStates: ['finitude-11'],

  provenance: {
    topicMapId: 'finitude-12',
    lineRange: { start: 279, end: 286 },
    section: '(c) Transition of the finite into the infinite',
    order: 12,
  },

  description: 'But this, its result, the negative as such, is (a) its very determination; for it is the negative of the negative. So, in going away and ceasing to be, the finite has not ceased; it has only become momentarily an other finite which equally is, however, a going-away as a going-over into another finite, and so forth to infinity.',
  keyPoints: [
    'Result, negative as such, is (a) its very determination (negative of negative)',
    'In going away and ceasing to be, finite has not ceased',
    'Only become momentarily other finite',
    'And so forth to infinity',
  ],
};

/**
 * State 13: Transition III - Result (b): Identity with itself = infinite
 *
 * Dialectic Position: In negation of itself, finite attained its being-in-itself;
 * identity with itself = affirmative being = infinite.
 */
const state13: DialecticState = {
  id: 'finitude-13',
  title: 'Transition III: result (b) identity with itself = infinite',
  concept: 'Infinite',
  phase: 'quality',

  moments: [
    {
      name: 'beingInItself',
      definition: 'Finite attained in negation of itself',
      type: 'determination',
    },
    {
      name: 'identity',
      definition: 'Identity with itself, negation of negation',
      type: 'mediation',
    },
    {
      name: 'infinite',
      definition: 'Affirmative being, other of finite',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'inv-13-1',
      constraint: 'inNegationOfItself.finite.attained = beingInItself',
      predicate: 'attained(inNegationOfItself(finite), beingInItself)',
    },
    {
      id: 'inv-13-2',
      constraint: 'identity = negationOfNegation',
      predicate: 'equals(identity, negationOfNegation)',
    },
    {
      id: 'inv-13-3',
      constraint: 'identity = affirmativeBeing',
      predicate: 'equals(identity, affirmativeBeing)',
    },
    {
      id: 'inv-13-4',
      constraint: 'infinite = otherOfFinite',
      predicate: 'equals(infinite, otherOfFinite)',
    },
  ],

  forces: [
    {
      id: 'force-13-1',
      description: 'Infinite drives toward next dialectical moment',
      type: 'sublation',
      trigger: 'infinite.established = true',
      effect: 'nextDialecticalMoment.emerges = true',
      targetState: 'infinity-1', // Next section
    },
  ],

  transitions: [
    {
      id: 'trans-13-1',
      from: 'finitude-13',
      to: 'infinity-1',
      mechanism: 'sublation',
      description: 'From infinite to infinity section',
    },
  ],

  nextStates: ['infinity-1'],
  previousStates: ['finitude-11', 'finitude-12'],

  provenance: {
    topicMapId: 'finitude-13',
    lineRange: { start: 288, end: 310 },
    section: '(c) Transition of the finite into the infinite',
    order: 13,
  },

  description: 'But, (b) if we consider this result more closely, in its going-away and ceasing-to-be, in this negation of itself, the finite has attained its being-in-itself; in it, it has rejoined itself.',
  keyPoints: [
    '(b) If consider result more closely',
    'In negation of itself, finite attained its being-in-itself',
    'Identity with itself, negation of negation = affirmative being',
    'This other = infinite',
  ],
};

/**
 * Complete Finitude IR Document
 *
 * This is the executable pseudo-code representation of the entire
 * "Finitude" section.
 */
export const finitudeIR: DialecticIR = {
  id: 'finitude-ir',
  title: 'Finitude IR: Finitude',
  section: 'B. FINITUDE (c) Finitude',

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
  ],

  metadata: {
    sourceFile: 'finitude.txt',
    totalStates: 13,
    cpuGpuMapping: {
      'finitude-1': 'quality',
      'finitude-2': 'quality',
      'finitude-3': 'quality',
      'finitude-4': 'quality',
      'finitude-5': 'quality',
      'finitude-6': 'quality',
      'finitude-7': 'quality',
      'finitude-8': 'quality',
      'finitude-9': 'quality',
      'finitude-10': 'quality',
      'finitude-11': 'quality',
      'finitude-12': 'quality',
      'finitude-13': 'quality',
    },
  },
};

/**
 * Export individual states for programmatic access
 */
export const finitudeStates = {
  'finitude-1': state1,
  'finitude-2': state2,
  'finitude-3': state3,
  'finitude-4': state4,
  'finitude-5': state5,
  'finitude-6': state6,
  'finitude-7': state7,
  'finitude-8': state8,
  'finitude-9': state9,
  'finitude-10': state10,
  'finitude-11': state11,
  'finitude-12': state12,
  'finitude-13': state13,
};

