/**
 * Logical Operations: The Constitution of the Thing Out of Matters
 *
 * Matter emerges from property as self-subsistent stuff. The thing consists of
 * self-subsistent matters, and is determined as "this" through quantitative combination.
 *
 * Dialectical Movement:
 * - Property to matter: self-subsistent stuff
 * - Negative moment: thing as "this"
 * - "Also": quantitative connection
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE CONSTITUTION OF THE THING OUT OF MATTERS
// ============================================================================

export const matOp1TransitionToMatter: LogicalOperation = {
  id: 'mat-op-1-transition-to-matter',
  chunkId: 'mat-1',
  label: 'Transition of property into matter — chemistry',
  clauses: [
    'transition = familiarInChemistry',
    'properties = representedAsMatters',
    'things = consistOfMatters',
    'components = concretes',
  ],
  predicates: [
    { name: 'familiarInChemistry', args: ['transition'] },
    { name: 'representedAsMatters', args: ['properties'] },
    { name: 'consistOfMatters', args: ['things'] },
    { name: 'concretes', args: ['components'] },
  ],
  relations: [
    { predicate: 'transitionsInto', from: 'property', to: 'matter' },
  ],
  candidateSummary: 'Transition of property into matter or self-subsistent stuff is familiar transition performed on sensible matter by chemistry. Seeks to represent properties of color, smell, etc. as luminous matter, coloring matter, odorific matter, sour, bitter matter. Assumes others like calorific matter, electrical, magnetic matter, in conviction that has gotten hold of properties as they truly are. Saying that things consist of various matters or stuffs. Careful about calling matters or stuffs \'things.\' Distinction made between things and components without exact statement whether components are things or half-things. They are at least concretes in general.',
  provenance: {
    sourceChunk: 'mat-1',
    sourceOp: 'mat-op-1-transition-to-matter',
  },
};

export const matOp2NecessityOfTransition: LogicalOperation = {
  id: 'mat-op-2-necessity-of-transition',
  chunkId: 'mat-2',
  label: 'Necessity of transition — property as essential',
  clauses: [
    'properties = essentialInThings',
    'properties = trueSelfSubsistence',
    'reflectionOfProperty = onlyOneSide',
    'thinghood = reducedToUnessential',
  ],
  predicates: [
    { name: 'essentialInThings', args: ['properties'] },
    { name: 'trueSelfSubsistence', args: ['properties'] },
    { name: 'onlyOneSide', args: ['reflectionOfProperty'] },
    { name: 'reducedToUnessential', args: ['thinghood'] },
  ],
  relations: [
    { predicate: 'are', from: 'properties', to: 'essentialInThings' },
  ],
  candidateSummary: 'Necessity of making transition from properties to matters, or assuming properties are truly matters, resulted from fact they are what is essential in things and consequently their true self-subsistence. At same time, reflection of property into itself constitutes only one side of whole reflection: sublation of distinction and continuity of property (concrete existence for other) with itself. Thinghood, as immanent negative reflection and distinguishing that repels itself from other, has been reduced to unessential moment. At same time, has further determined itself.',
  provenance: {
    sourceChunk: 'mat-2',
    sourceOp: 'mat-op-2-necessity-of-transition',
  },
};

export const matOp3NegativeMoment: LogicalOperation = {
  id: 'mat-op-3-negative-moment',
  chunkId: 'mat-3',
  label: 'First — negative moment preserved',
  clauses: [
    'negativeMoment = preserved',
    'property = becomesMatterContinuous',
    'continuity = containsMomentOfNegative',
    'negativeSelfSubsistence = versusPositive',
  ],
  predicates: [
    { name: 'preserved', args: ['negativeMoment'] },
    { name: 'becomesMatterContinuous', args: ['property'] },
    { name: 'containsMomentOfNegative', args: ['continuity'] },
    { name: 'versusPositive', args: ['negativeSelfSubsistence'] },
  ],
  relations: [
    { predicate: 'becomes', from: 'property', to: 'matter' },
  ],
  candidateSummary: 'Negative moment has preserved itself. Property has become matter continuous with itself and self-subsisting only inasmuch as difference of things has sublated itself. Continuity of property in otherness itself contains moment of negative. As negative unity, self-subsistence is at same time restored something of thinghood. Negative self-subsistence versus positive self-subsistence of stuff.',
  provenance: {
    sourceChunk: 'mat-3',
    sourceOp: 'mat-op-3-negative-moment',
  },
};

export const matOp4ThingProgressedToDeterminateness: LogicalOperation = {
  id: 'mat-op-4-thing-progressed-to-determinateness',
  chunkId: 'mat-4',
  label: 'Second — thing progressed to determinateness',
  clauses: [
    'thing = progressedToDeterminateness',
    'thing = continuousWithOthers',
    'imperfectDistinction = sublated',
    'thing = determinedInItself',
  ],
  predicates: [
    { name: 'progressedToDeterminateness', args: ['thing'] },
    { name: 'continuousWithOthers', args: ['thing'] },
    { name: 'sublated', args: ['imperfectDistinction'] },
    { name: 'determinedInItself', args: ['thing'] },
  ],
  relations: [
    { predicate: 'is', from: 'thing', to: 'thisThing' },
  ],
  candidateSummary: 'Thing has progressed from indeterminacy to full determinateness. As thing in itself, abstract identity, simple negative concrete existence, concrete existence determined as indeterminate. Then determined through properties, by virtue of which supposed to be distinguished from other things. But since through property thing rather continuous with other things, imperfect distinction sublated. Thing returned into itself. Now determined as determined, determined in itself or is this thing.',
  provenance: {
    sourceChunk: 'mat-4',
    sourceOp: 'mat-op-4-thing-progressed-to-determinateness',
  },
};

export const matOp5TurningBackUnessential: LogicalOperation = {
  id: 'mat-op-5-turning-back-unessential',
  chunkId: 'mat-5',
  label: 'Third — turning back unessential',
  clauses: [
    'turningBack = unessentialDetermination',
    'selfContinuousSubsistence = makesMatter',
    'difference = sublated',
    'determinateness = inElementOfInessentiality',
  ],
  predicates: [
    { name: 'unessentialDetermination', args: ['turningBack'] },
    { name: 'makesMatter', args: ['selfContinuousSubsistence'] },
    { name: 'sublated', args: ['difference'] },
    { name: 'inElementOfInessentiality', args: ['determinateness'] },
  ],
  relations: [
    { predicate: 'makesUp', from: 'selfContinuousSubsistence', to: 'selfSubsistentMatter' },
  ],
  candidateSummary: 'Turning back into itself, though self-referring determination, at same time unessential determination. Self-continuous subsistence makes up self-subsistent matter in which difference of things, determinateness existing in and for itself, is sublated and is something external. Therefore, although thing as this thing is complete determinateness, determinateness is such in element of inessentiality.',
  provenance: {
    sourceChunk: 'mat-5',
    sourceOp: 'mat-op-5-turning-back-unessential',
  },
};

export const matOp6MovementOfProperty: LogicalOperation = {
  id: 'mat-op-6-movement-of-property',
  chunkId: 'mat-6',
  label: 'Movement of property — two moments',
  clauses: [
    'unity = repelsItselfFromItself',
    'moments = mattersAndThisThing',
    'thing = freedItself',
    'thing = selfIdenticalNegationVersusPositive',
  ],
  predicates: [
    { name: 'repelsItselfFromItself', args: ['unity'] },
    { name: 'mattersAndThisThing', args: ['moments'] },
    { name: 'freedItself', args: ['thing'] },
    { name: 'selfIdenticalNegationVersusPositive', args: ['thing'] },
  ],
  relations: [
    { predicate: 'has', from: 'property', to: 'twoMoments' },
  ],
  candidateSummary: 'Property is not only external determination but concrete existence immediately existing in itself. Unity of externality and essentiality repels itself from itself, contains reflection-into-itself and reflection-into-other. On one hand, determination as simple, self-identical and self-referring self-subsistent in which negative unity, one of thing, is sublated. On other hand, determination over against other, but likewise as one reflected into itself and determined in itself. Therefore, matters and this thing. Two moments of self-identical externality, property reflected into itself. Property was that by which things supposed to be distinguished. Thing freed itself of negative side of inhering in other, become free from being determined by other things, returned into itself. At same time, only thing-in-itself now become other of itself. Manifold properties have become self-subsistent, negative connection in one of thing now only sublated connection. Thing is self-identical negation only as against positive continuity of material.',
  provenance: {
    sourceChunk: 'mat-6',
    sourceOp: 'mat-op-6-movement-of-property',
  },
};

export const matOp7This: LogicalOperation = {
  id: 'mat-op-7-this',
  chunkId: 'mat-7',
  label: '"This" — complete determinateness',
  clauses: [
    'this = completeDeterminateness',
    'thing = consistsOfSelfSubsistentMatters',
    'connection = unessentialLinking',
    'matters = overrunThing',
    'thing = merelyQuantitativeConnection',
  ],
  predicates: [
    { name: 'completeDeterminateness', args: ['this'] },
    { name: 'consistsOfSelfSubsistentMatters', args: ['thing'] },
    { name: 'unessentialLinking', args: ['connection'] },
    { name: 'overrunThing', args: ['matters'] },
    { name: 'merelyQuantitativeConnection', args: ['thing'] },
  ],
  relations: [
    { predicate: 'consistsOf', from: 'thing', to: 'matters' },
  ],
  candidateSummary: '"This" constitutes complete determinateness of thing, determinateness which is at same time external determinateness. Thing consists of self-subsistent matters indifferent to connection they have in thing. Connection only unessential linking. Difference of one thing from another depends on more or less of particular matters and in what amount. Matters overrun this thing, continue into others. That they belong to this thing is no restriction for them. Just as little restriction for one another. Negative connection only impotent "this." In being linked together, do not sublate themselves. Self-subsistent, impenetrable to each other. In determinateness refer only to themselves, mutually indifferent manifold of subsistence. Only limit quantitative one. Thing as this is merely quantitative connection, mere collection, their "also." Thing consists of some quantum of matter, also quantum of another, also yet another. Combination, of not having any combination, alone constitutes thing.',
  provenance: {
    sourceChunk: 'mat-7',
    sourceOp: 'mat-op-7-this',
  },
};

export const matterOperations: LogicalOperation[] = [
  matOp1TransitionToMatter,
  matOp2NecessityOfTransition,
  matOp3NegativeMoment,
  matOp4ThingProgressedToDeterminateness,
  matOp5TurningBackUnessential,
  matOp6MovementOfProperty,
  matOp7This,
];
