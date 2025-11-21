import type { Chunk, LogicalOperation } from '../../../types';

/*
  Constitution of the Thing out of Matters — B. THE CONSTITUTION OF THE THING OUT OF MATTERS

  This module translates the Matter section:
  - Transition from property to matter: property becomes self-subsistent stuff
  - Necessity: properties are essential, true self-subsistence
  - Three moments: negative preserved, thing progresses, turning back unessential
  - Property's movement: unity repels itself - matters and thing
  - Thing as "this": complete determinateness but external
  - Thing consists of matters: merely quantitative connection

  PHILOSOPHICAL NOTES:

  1. **Transition from Property to Matter**:
     Property becomes self-subsistent matter/stuff. This is familiar from chemistry
     (luminous matter, coloring matter, etc.). Properties are what is essential in
     things - their true self-subsistence. Property's reflection into itself is
     continuity of property with itself.

  2. **Thinghood Reduced to Unessential Moment**:
     Thinghood as immanent negative reflection, distinguishing that repels itself,
     is reduced to unessential moment. But negative moment preserved - property
     becomes matter continuous with itself only as difference of things sublated.
     Continuity contains negative moment - negative unity, restored thinghood.

  3. **Thing's Progress to Determinateness**:
     Thing progresses from indeterminacy to full determinateness. As thing-in-itself,
     abstract identity. Determined through properties. But property makes thing
     continuous with others - imperfect distinction sublated. Thing returns into
     itself - determined as determined, is this thing. But turning back is unessential.

  4. **Property's Movement: Matters and Thing**:
     Property is unity of externality and essentiality - repels itself from itself.
     Contains reflection-into-itself and reflection-into-other. On one hand: simple,
     self-identical, self-referring self-subsistent (negative unity of thing sublated).
     On other hand: determination over against other, reflected into itself, determined
     in itself - matters and this thing. Two moments of property reflected into itself.

  5. **Thing as Merely Quantitative Connection**:
     "This" constitutes complete determinateness but external. Thing consists of
     self-subsistent matters indifferent to connection. Connection is unessential
     linking - difference depends on more/less of matters. Matters overrun thing,
     continue into others. They are impenetrable, mutually indifferent - only
     quantitative limit. Thing is merely quantitative connection - mere collection,
     "also." Thing consists of quantum of matter, also quantum of another, etc.
     Combination of not having any combination constitutes thing.

  6. **Connection to Form Processor**:
     Matter maps to Property (in Form Processor) - self-subsistent stuff. Thing
     as quantitative connection maps to Entity as collection. This prepares for
     dissolution where thing becomes absolutely alterable, absolutely dissoluble.
*/

// ============================================================================
// B. THE CONSTITUTION OF THE THING OUT OF MATTERS
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'matter-transition-intro',
    title: 'Transition from property to matter',
    text: `B. THE CONSTITUTION OF THE THING OUT OF MATTERS

The transition of property into a matter
or into a self-subsistent stuff is the familiar
transition performed on sensible matter by chemistry
when it seeks to represent the properties of color, smell, etc.,
as luminous matter, coloring matter, odorific matter,
sour, bitter matter and so on;
or when it simply assumes others,
like calorific matter, electrical, magnetic matter,
in the conviction that it has thereby gotten hold
of properties as they truly are.
Equally current is the saying that
things consist of various matters or stuffs.
One is careful about calling these matters or stuffs "things,"
even though one will readily admit that,
for example, a pigment is a thing;
but I do not know whether luminous matter,
for instance, or calorific matter,
or electrical matter, etc., are called things.
The distinction is made between things and their components
without any exact statement as to whether these components also,
and to what extent, are things or perhaps just half-things;
but they are at least concretes in general.`,
    summary:
      'Transition: Property becomes self-subsistent matter/stuff. Familiar from chemistry (luminous matter, coloring matter, etc.). Things consist of various matters. Distinction between things and components - components are at least concretes.'
  },

  {
    id: 'matter-necessity',
    title: 'Necessity: properties are essential, true self-subsistence',
    text: `The necessity of making the transition
from properties to matters,
or of assuming that the properties are truly matters,
has resulted from the fact that they are
what is the essential in things
and consequently their true self-subsistence.
At the same time, however,
the reflection of the property into itself
constitutes only one side of the whole reflection,
namely the sublation of the distinction
and the continuity of the property
(which was supposed to be a concrete existence for an other)
with itself.
Thinghood, as immanent negative reflection
and as a distinguishing that repels itself from the other,
has consequently been reduced to an unessential moment;
at the same time, however, it has further determined itself.`,
    summary:
      'Necessity: Properties are essential in things - true self-subsistence. Property\'s reflection into itself is continuity with itself (sublation of distinction). Thinghood reduced to unessential moment but further determined itself.'
  },

  {
    id: 'matter-negative-moment',
    title: 'First: negative moment preserved',
    text: `First, this negative moment has preserved itself,
for property has become a matter continuous with itself
and self-subsisting only inasmuch as
the difference of things has sublated itself;
thus the continuity of the property in the otherness
itself contains the moment of the negative,
and, as this negative unity,
its self-subsistence is at the same time
the restored something of thinghood,
negative self-subsistence versus
the positive self-subsistence of the stuff.`,
    summary:
      'Negative moment preserved: Property becomes matter continuous with itself only as difference of things sublated. Continuity in otherness contains negative moment - negative unity. Self-subsistence is restored thinghood (negative) vs positive self-subsistence of stuff.'
  },

  {
    id: 'matter-determinateness',
    title: 'Second: thing progresses to determinateness',
    text: `Second, the thing has thereby progressed
from its indeterminacy to full determinateness.
As thing in itself, it is abstract identity,
simple negative concrete existence,
or this concrete existence
determined as the indeterminate;
it is then determined through its properties,
by virtue of which it is supposed to be
distinguished from other things;
but, since through the property the thing is
rather continuous with other things,
this imperfect distinction is sublated;
the thing has thereby returned into itself
and is now determined as determined;
it is determined in itself or is this thing.`,
    summary:
      'Thing progresses: From indeterminacy to full determinateness. As thing-in-itself, abstract identity. Determined through properties. But property makes thing continuous with others - imperfect distinction sublated. Thing returns into itself - determined as determined, is this thing.'
  },

  {
    id: 'matter-unessential',
    title: 'Third: turning back is unessential',
    text: `But, third, this turning back into itself,
though a self-referring determination,
is at the same time an unessential determination;
the self-continuous subsistence makes up
the self-subsistent matter
in which the difference of things,
their determinateness existing in and for itself,
is sublated and is something external.
Therefore, although the thing as this thing
is complete determinateness,
this determinateness is such
in the element of inessentiality.`,
    summary:
      'Turning back unessential: Self-referring determination but unessential. Self-continuous subsistence makes up self-subsistent matter. Difference of things sublated, is external. Thing is complete determinateness but in element of inessentiality.'
  },

  {
    id: 'matter-property-movement',
    title: 'Property\'s movement: matters and thing',
    text: `Considered from the side of the movement of the property,
this result follows in this way.
The property is not only external determination
but concrete existence immediately existing in itself.
This unity of externality and essentiality repels itself from itself,
for it contains reflection-into-itself and reflection-into-other,
and, on the one hand, it is determination as simple,
self-identical and self-referring self-subsistent
in which the negative unity,
the one of the thing, is sublated;
on the other hand, it is this determination over against an other,
but likewise as a one which is reflected into itself
and is determined in itself;
it is, therefore, the matters and this thing.
These are the two moments of self-identical externality,
or of property reflected into itself.
The property was that by which things
were supposed to be distinguished.
Since the thing has freed itself of its
negative side of inhering in an other,
it has thereby also become free
from its being determined by other things
and has returned into itself
from the reference connecting it to the other.
At the same time, however, it is only the thing-in-itself
now become the other of itself,
for the manifold properties on their part
have become self-subsistent
and their negative connection
in the one of the thing is
now only a sublated connection.
Consequently, the thing is self-identical negation
only as against the positive continuity of the material.`,
    summary:
      'Property\'s movement: Unity of externality/essentiality repels itself. Contains reflection-into-itself and reflection-into-other. On one hand: simple, self-identical, self-referring (negative unity of thing sublated). On other: determination over against other, reflected into itself - matters and this thing. Two moments of property reflected into itself. Thing freed from inhering in other, returned into itself. But thing-in-itself become other of itself - properties self-subsistent, negative connection sublated. Thing is self-identical negation vs positive continuity of material.'
  },

  {
    id: 'matter-thing-as-this',
    title: 'Thing as "this": complete determinateness but external',
    text: `The "this" thus constitutes the
complete determinateness of the thing,
a determinateness which is at the same time
an external determinateness.
The thing consists of self-subsistent matters
indifferent to the connection they have in the thing.
This connection is therefore only
an unessential linking of them,
the difference of one thing from another
depending on whether there is in it
a more or less of particular matters
and in what amount.`,
    summary:
      'Thing as "this": Complete determinateness but external. Thing consists of self-subsistent matters indifferent to connection. Connection is unessential linking - difference depends on more/less of matters.'
  },

  {
    id: 'matter-quantitative-connection',
    title: 'Thing as merely quantitative connection',
    text: `These matters overrun this thing,
continue into others,
and that they belong to this thing
is no restriction for them.
Just as little are they, moreover,
a restriction for one another,
for their negative connection is
only the impotent "this."
Hence, in being linked together in it,
they do not sublate themselves;
they are as self-subsistent,
impenetrable to each other;
in their determinateness they refer only to themselves
and are a mutually indifferent manifold of subsistence;
the only limit of which they are capable is a quantitative one.
The thing as this is just their merely quantitative connection,
a mere collection, their "also."
The thing consists of some quantum or other of a matter,
also of the quantum of another, and also of yet another;
this combination, of not having any combination
alone constitutes the thing.`,
    summary:
      'Merely quantitative connection: Matters overrun thing, continue into others. No restriction. They are impenetrable, mutually indifferent - only quantitative limit. Thing is merely quantitative connection - mere collection, "also." Thing consists of quantum of matter, also quantum of another, etc. Combination of not having any combination constitutes thing.'
  }
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'op-matter-1',
    chunkId: 'matter-transition-intro',
    label: 'Transition from property to matter',
    clauses: [
      'Property becomes self-subsistent matter/stuff',
      'Familiar from chemistry (luminous matter, coloring matter, etc.)',
      'Things consist of various matters',
      'Distinction between things and components',
      'Components are at least concretes'
    ],
    predicates: [
      { name: 'becomes', args: ['property', 'matter'] },
      { name: 'is', args: ['property', 'selfSubsistentStuff'] },
      { name: 'consistOf', args: ['things', 'matters'] },
      { name: 'are', args: ['components', 'concretes'] }
    ],
    relations: [
      { predicate: 'becomes', from: 'property', to: 'matter' },
      { predicate: 'consistOf', from: 'things', to: 'matters' }
    ],
    candidateSummary:
      'Transition: Property becomes self-subsistent matter/stuff. Familiar from chemistry. Things consist of various matters. Components are concretes.'
  },

  {
    id: 'op-matter-2',
    chunkId: 'matter-necessity',
    label: 'Necessity: properties are essential',
    clauses: [
      'Properties are essential in things - true self-subsistence',
      'Property\'s reflection into itself is continuity with itself',
      'Sublation of distinction',
      'Thinghood reduced to unessential moment',
      'But thinghood further determined itself'
    ],
    predicates: [
      { name: 'are', args: ['properties', 'essential'] },
      { name: 'are', args: ['properties', 'trueSelfSubsistence'] },
      { name: 'is', args: ['reflection', 'continuity'] },
      { name: 'is', args: ['thinghood', 'unessentialMoment'] },
      { name: 'is', args: ['thinghood', 'furtherDetermined'] }
    ],
    relations: [
      { predicate: 'are', from: 'properties', to: 'essential' },
      { predicate: 'is', from: 'thinghood', to: 'unessentialMoment' }
    ],
    candidateSummary:
      'Necessity: Properties are essential - true self-subsistence. Property\'s reflection is continuity with itself. Thinghood reduced to unessential moment but further determined.'
  },

  {
    id: 'op-matter-3',
    chunkId: 'matter-negative-moment',
    label: 'Negative moment preserved',
    clauses: [
      'Property becomes matter continuous with itself',
      'Only as difference of things sublated',
      'Continuity in otherness contains negative moment',
      'Negative unity',
      'Self-subsistence is restored thinghood (negative) vs positive self-subsistence of stuff'
    ],
    predicates: [
      { name: 'becomes', args: ['property', 'matter'] },
      { name: 'is', args: ['property', 'continuous'] },
      { name: 'isSublated', args: ['differenceOfThings'] },
      { name: 'contains', args: ['continuity', 'negativeMoment'] },
      { name: 'is', args: ['continuity', 'negativeUnity'] },
      { name: 'is', args: ['selfSubsistence', 'restoredThinghood'] },
      { name: 'is', args: ['selfSubsistence', 'negative'] },
      { name: 'is', args: ['stuffSelfSubsistence', 'positive'] }
    ],
    relations: [
      { predicate: 'becomes', from: 'property', to: 'matter' },
      { predicate: 'contains', from: 'continuity', to: 'negativeMoment' }
    ],
    candidateSummary:
      'Negative moment preserved: Property becomes matter continuous with itself only as difference sublated. Continuity contains negative moment - negative unity. Self-subsistence is restored thinghood (negative) vs positive stuff.'
  },

  {
    id: 'op-matter-4',
    chunkId: 'matter-determinateness',
    label: 'Thing progresses to determinateness',
    clauses: [
      'Thing progresses from indeterminacy to full determinateness',
      'As thing-in-itself, abstract identity',
      'Determined through properties',
      'But property makes thing continuous with others',
      'Imperfect distinction sublated',
      'Thing returns into itself - determined as determined, is this thing'
    ],
    predicates: [
      { name: 'progresses', args: ['thing', 'determinateness'] },
      { name: 'is', args: ['thingInItself', 'abstractIdentity'] },
      { name: 'isDetermined', args: ['thing', 'throughProperties'] },
      { name: 'makes', args: ['property', 'thingContinuous'] },
      { name: 'isSublated', args: ['imperfectDistinction'] },
      { name: 'returns', args: ['thing', 'itself'] },
      { name: 'is', args: ['thing', 'determined'] },
      { name: 'is', args: ['thing', 'thisThing'] }
    ],
    relations: [
      { predicate: 'progresses', from: 'thing', to: 'determinateness' },
      { predicate: 'returns', from: 'thing', to: 'itself' }
    ],
    candidateSummary:
      'Thing progresses: From indeterminacy to determinateness. As thing-in-itself, abstract identity. Determined through properties. But property makes thing continuous - imperfect distinction sublated. Thing returns into itself - is this thing.'
  },

  {
    id: 'op-matter-5',
    chunkId: 'matter-unessential',
    label: 'Turning back is unessential',
    clauses: [
      'Turning back is self-referring determination but unessential',
      'Self-continuous subsistence makes up self-subsistent matter',
      'Difference of things sublated, is external',
      'Thing is complete determinateness but in element of inessentiality'
    ],
    predicates: [
      { name: 'is', args: ['turningBack', 'selfReferring'] },
      { name: 'is', args: ['turningBack', 'unessential'] },
      { name: 'makesUp', args: ['selfContinuousSubsistence', 'matter'] },
      { name: 'is', args: ['matter', 'selfSubsistent'] },
      { name: 'isSublated', args: ['differenceOfThings'] },
      { name: 'is', args: ['difference', 'external'] },
      { name: 'is', args: ['thing', 'completeDeterminateness'] },
      { name: 'is', args: ['thing', 'inInessentiality'] }
    ],
    relations: [
      { predicate: 'makesUp', from: 'selfContinuousSubsistence', to: 'matter' },
      { predicate: 'is', from: 'thing', to: 'completeDeterminateness' }
    ],
    candidateSummary:
      'Turning back unessential: Self-referring but unessential. Self-continuous subsistence makes up self-subsistent matter. Difference sublated, external. Thing is complete determinateness but in inessentiality.'
  },

  {
    id: 'op-matter-6',
    chunkId: 'matter-property-movement',
    label: 'Property\'s movement: matters and thing',
    clauses: [
      'Property is unity of externality/essentiality - repels itself',
      'Contains reflection-into-itself and reflection-into-other',
      'On one hand: simple, self-identical, self-referring (negative unity sublated)',
      'On other: determination over against other, reflected into itself',
      'Matters and this thing - two moments of property reflected into itself',
      'Thing freed from inhering in other, returned into itself',
      'But thing-in-itself become other of itself',
      'Properties self-subsistent, negative connection sublated',
      'Thing is self-identical negation vs positive continuity of material'
    ],
    predicates: [
      { name: 'is', args: ['property', 'unity'] },
      { name: 'repels', args: ['property', 'itself'] },
      { name: 'contains', args: ['property', 'reflectionIntoItself'] },
      { name: 'contains', args: ['property', 'reflectionIntoOther'] },
      { name: 'is', args: ['property1', 'simple'] },
      { name: 'is', args: ['property1', 'selfIdentical'] },
      { name: 'isSublated', args: ['negativeUnity'] },
      { name: 'is', args: ['property2', 'determination'] },
      { name: 'is', args: ['property2', 'reflectedIntoItself'] },
      { name: 'are', args: ['moments', 'mattersAndThing'] },
      { name: 'isFreed', args: ['thing', 'fromInhering'] },
      { name: 'returns', args: ['thing', 'itself'] },
      { name: 'becomes', args: ['thingInItself', 'other'] },
      { name: 'are', args: ['properties', 'selfSubsistent'] },
      { name: 'isSublated', args: ['negativeConnection'] },
      { name: 'is', args: ['thing', 'selfIdenticalNegation'] }
    ],
    relations: [
      { predicate: 'repels', from: 'property', to: 'itself' },
      { predicate: 'are', from: 'moments', to: 'mattersAndThing' },
      { predicate: 'becomes', from: 'thingInItself', to: 'other' }
    ],
    candidateSummary:
      'Property\'s movement: Unity repels itself - contains reflection-into-itself and reflection-into-other. Two moments: matters and thing. Thing freed, returned into itself. But thing-in-itself become other - properties self-subsistent. Thing is self-identical negation vs positive continuity.'
  },

  {
    id: 'op-matter-7',
    chunkId: 'matter-thing-as-this',
    label: 'Thing as "this": external determinateness',
    clauses: [
      '"This" constitutes complete determinateness but external',
      'Thing consists of self-subsistent matters',
      'Matters indifferent to connection',
      'Connection is unessential linking',
      'Difference depends on more/less of matters'
    ],
    predicates: [
      { name: 'constitutes', args: ['this', 'completeDeterminateness'] },
      { name: 'is', args: ['determinateness', 'external'] },
      { name: 'consistsOf', args: ['thing', 'matters'] },
      { name: 'are', args: ['matters', 'selfSubsistent'] },
      { name: 'are', args: ['matters', 'indifferent'] },
      { name: 'is', args: ['connection', 'unessentialLinking'] },
      { name: 'depends', args: ['difference', 'moreLess'] }
    ],
    relations: [
      { predicate: 'consistsOf', from: 'thing', to: 'matters' },
      { predicate: 'is', from: 'connection', to: 'unessentialLinking' }
    ],
    candidateSummary:
      'Thing as "this": Complete determinateness but external. Thing consists of self-subsistent matters indifferent to connection. Connection is unessential linking - difference depends on more/less of matters.'
  },

  {
    id: 'op-matter-8',
    chunkId: 'matter-quantitative-connection',
    label: 'Thing as merely quantitative connection',
    clauses: [
      'Matters overrun thing, continue into others',
      'No restriction',
      'Matters are impenetrable, mutually indifferent',
      'Only quantitative limit',
      'Thing is merely quantitative connection',
      'Mere collection, "also"',
      'Thing consists of quantum of matter, also quantum of another, etc.',
      'Combination of not having any combination constitutes thing'
    ],
    predicates: [
      { name: 'overrun', args: ['matters', 'thing'] },
      { name: 'continue', args: ['matters', 'others'] },
      { name: 'are', args: ['matters', 'impenetrable'] },
      { name: 'are', args: ['matters', 'mutuallyIndifferent'] },
      { name: 'is', args: ['limit', 'quantitative'] },
      { name: 'is', args: ['thing', 'quantitativeConnection'] },
      { name: 'is', args: ['thing', 'collection'] },
      { name: 'consistsOf', args: ['thing', 'quantum'] },
      { name: 'constitutes', args: ['combination', 'thing'] }
    ],
    relations: [
      { predicate: 'overrun', from: 'matters', to: 'thing' },
      { predicate: 'is', from: 'thing', to: 'quantitativeConnection' }
    ],
    candidateSummary:
      'Merely quantitative connection: Matters overrun thing, continue into others. Impenetrable, mutually indifferent - only quantitative limit. Thing is merely quantitative connection - collection, "also." Combination of not having combination constitutes thing.'
  }
];

/* minimal, stable accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getChunkById(id: string): Chunk | null {
  return CANONICAL_CHUNKS.find((c) => c.id === id) ?? null;
}

export function getLogicalOpsForChunkId(chunkId: string): LogicalOperation[] {
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunkId);
}

export function getAllChunks(): Chunk[] {
  return CANONICAL_CHUNKS;
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS;
}

