import type { Chunk, LogicalOperation } from '../../../types';

/*
  Quantum — B. EXTENSIVE AND INTENSIVE QUANTUM

  This module translates the Extensive and Intensive Quantum section:
  - a. Their difference: extensive magnitude vs intensive magnitude (degree)
  - b. Identity of extensive and intensive magnitude: same determinateness
  - c. Alteration of quantum: quantum must alter, sends itself beyond itself

  PHILOSOPHICAL NOTES:

  1. **Extensive vs Intensive Magnitude**:
     Extensive magnitude has limit as plurality (amount). Intensive magnitude
     (degree) has limit as simple determinateness. Degree is not aggregate but
     plurality gathered into simple determination - existence returned into being-for-itself.

  2. **The Transition from Extensive to Intensive**:
     Extensive quantum's many collapses into oneness and steps outside it, becoming
     intensive. But intensive magnitude has its determinateness in amount (its amount),
     so it is essentially extensive. They are one and the same determinateness.

  3. **Degree's Quality**:
     Degree has determinateness external to itself. It is unitary quantitative
     determinateness among plurality of intensities, each in essential connection
     with others. The scale of degrees is continuous progress - each has determinateness
     only in others. In this externality it possesses its quality.

  4. **Alteration of Quantum**:
     Quantum is determinateness posited as sublated - indifferent limit that is
     negation of itself. Quantum must alter - it has being only in continuity with
     other. It sends itself beyond itself (increases/decreases), impelling itself
     to further limits, and so on to infinity.

  5. **Connection to Laws of Mathematical Cognition**:
     Extensive/Intensive quantum shows the dual nature of determinate quantity -
     the Thing (Entity) as both discrete plurality and unified intensity. This
     prepares for Ratio where quanta relate to each other, establishing the
     foundation for mathematical relations and measures.
*/

// ============================================================================
// B. EXTENSIVE AND INTENSIVE QUANTUM
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  // ============================================================================
  // a. Their difference
  // ============================================================================
  {
    id: 'quantum-extensive-magnitude',
    title: 'Extensive magnitude: limit as plurality',
    text: `B. EXTENSIVE AND INTENSIVE QUANTUM

a. Their difference

1. We have seen that quantum has
its determinateness as limit in amount.
In itself quantum is discrete, a plurality
which does not have a being different
from its limit or its limit outside it.
Quantum, thus with its limit
which as limit is a plurality,
is extensive magnitude.

Extensive magnitude is to be distinguished from continuous magnitude;
its direct opposite is not the discrete, but the intensive magnitude.
Extensive and intensive magnitudes are determinacies of
the quantitative limit itself,
whereas quantum is identical with its limit;
continuous and discrete magnitudes are, on the contrary,
determinations of magnitude in itself,
that is, of quantity as such, in so far as
in quantum abstraction is made from the limit.
Extensive magnitude has the moment of continuity in it
in its limit, for its many is everywhere continuous;
the limit as negation appears, therefore,
in this equality of the many as a limiting of the unity.
Continuous magnitude is quantity that continues
without regard to any limit,
and in so far as it is represented with one such limit,
the latter is a limitation in general,
without discreteness being posited in it.
Determined as only continuous magnitude,
quantum is not yet determined for itself
because the magnitude lacks the one
(in which the determinateness-for-itself lies)
and number.
Similarly, a discrete magnitude is
immediately only a differentiated many in general
which, if it were to have a limit as a many,
would be only an aggregate,
that is, would be only indeterminately limited;
in order for quantum to be determinate,
the many must be concentrated
into one and thereby be posited as identical with the limit.
Continuous and discrete magnitude, taken as quantum in general,
have each posited in it only one of the two sides
by virtue of which quantum is fully determined and a number.
Taken immediately, this latter is extensive quantum
the simple determinateness which is amount essentially,
but the amount of one and the same unit;
extensive quantum is distinguished from number
only because in the latter the determinateness is
explicitly posited as plurality.`,
    summary:
      'Extensive magnitude: Quantum has determinateness as limit in amount (plurality). Extensive/intensive are determinacies of quantitative limit itself. Extensive quantum is amount essentially (amount of one unit), distinguished from number only by explicit plurality.'
  },

  {
    id: 'quantum-number-determinateness',
    title: 'Number\'s determinateness: self-referring limit',
    text: `2. However, the determinateness through number
[how much there is of something]
does not require being distinguished from
how much there is of something else,
as if to the determinateness of one thing belonged
how much there is of it
and how much there is of an other,
for the determinateness of magnitude as such
is a limit determined for itself,
indifferent and simply self-referring;
and in number this limit is posited
as enclosed within the one existing for itself:
the externality that it has,
the reference to other, is inside it.
Further, like the many in general,
this many of the limit is
not internally unequal but continuous;
each many is what any other many is;
consequently, the many as a many of
existents outside one another,
or as discrete, does not constitute
the determinateness as such.
Thus this many collapses for itself
into its continuity and becomes simple unity.
Amount is but a moment of number;
but, as an aggregate of numerical ones,
it does not constitute the determinateness of number;
on the contrary, these ones as indifferent and self-external are
sublated in number whose being has turned back into itself;
the externality that constituted the ones of plurality vanishes
in the one as the self-reference of number.`,
    summary:
      'Number\'s determinateness: Limit is determined for itself, indifferent and self-referring. In number, limit is enclosed within one - externality/reference to other is inside it. Many collapses into continuity, becomes simple unity. Amount is moment; ones are sublated in number\'s self-reference.'
  },

  {
    id: 'quantum-intensive-magnitude-degree',
    title: 'Intensive magnitude: degree as simple determinateness',
    text: `The limit of quantum which, as extensive,
had its existent determinateness as self-external number,
thus passes over into simple determinateness.
In this simple determination of limit,
quantum is intensive magnitude;
and the limit or the determinateness
which is identical with quantum is now
also posited as simple: it is degree.
Degree is thus a determinate magnitude, a quantum,
but at the same time it is not an aggregate
or several within itself;
it is only a plurality;
plurality is a severality that has gathered
together into simple determination,
it is existence that has returned into being-for-itself.
It is true that its determinateness must be expressed by a number,
which is the being of the quantum as completely determined,
but the number is not an amount or a how many times
but is rather a onefold, only a degree.
When we speak of 10 or 20 degrees,
the quantum which has that many degrees
[the tenth, the twentieth degree]
is not the amount and sum of the degrees;
if that were the case, it would be an extensive quantum;
it is rather only that one degree, the tenth, the twentieth.
It does contain the determinateness found in
the number ten or twenty,
but not as several ones:
the number is there as a sublated amount,
as a simple determinateness.`,
    summary:
      'Intensive magnitude (degree): Limit passes over into simple determinateness. Degree is quantum but not aggregate - plurality gathered into simple determination, existence returned into being-for-itself. Number expresses determinateness but as sublated amount, simple determinateness (onefold degree).'
  },

  {
    id: 'quantum-degree-externality-quality',
    title: 'Degree\'s externality and quality',
    text: `3. In number, quantum is posited in its complete determinateness;
but as intensive quantum, in the being-for-itself of number,
it is posited as it is according to its concept or implicitly in itself.
For the form of self-reference which it has in degree is
at the same time the externality of degree to itself.
As extensive quantum, number is a numerical plurality
and thus has externality inside it.
This externality, as plurality in general,
collapses into a state of undifferentiatedness
and is sublated into the numerical one,
the self-reference of number.
But quantum has its determinateness as number;
it contains this determinateness, as we have said,
whether the latter is posited in it or not.
Degree, therefore, which as internally simple no
longer has this external otherness in it, has it outside it,
and refers to it as to its determinateness.
A plurality external to it constitutes the determinateness
of the simple limit which the degree is for itself.
In so far as in the extensive quantum
amount was supposed to be found within number,
it was sublated there;
now, as thus sublated, amount is posited outside number.
Number, in being posited as a one,
as self-reference reflected into itself,
excludes from itself the indifference and the externality of amount
and is self-reference as reference to an external through itself.

In this, quantum has the reality which is adequate to its concept.
The indifference of the determinateness constitutes its quality,
that is, a determinateness which is in it
as a determinateness external to itself.
Accordingly, degree is a unitary quantitative determinateness
among a plurality of such intensities
which, though diverse, each being only a simple reference to itself,
are at the same time in essential connection with each other,
so that each has its determinateness in this continuity with the others.
This reference connecting a degree through itself to its other
makes ascent and descent on the scale of degrees
a continuous progress, a flow,
which is an uninterrupted and indivisible alteration;
none of the "more or less" differentiated within it
is separate from the others
but each has its determinateness only in these others.
As a self-referring quantitative determination,
each degree is indifferent towards the others;
but, in itself, it equally refers to this externality;
it is what it is only through
the intermediary of this externality;
in short, its reference to itself is not
an indifferent reference to externality
but in this externality it possesses its quality.`,
    summary:
      'Degree\'s externality/quality: Self-reference is externality to itself. Degree has determinateness outside it (plurality external). Amount sublated in extensive is posited outside intensive. Degree is unitary determinateness among plurality of intensities, each in essential connection - scale is continuous progress. In externality it possesses its quality.'
  },

  // ============================================================================
  // b. Identity of extensive and intensive magnitude
  // ============================================================================
  {
    id: 'quantum-degree-contains-amount',
    title: 'Degree contains amount: identity of extensive and intensive',
    text: `b. Identity of extensive and intensive magnitude

Degree is not inherently external to itself.
It is not, however, the indeterminate one,
is not the principle of number as such
which is not amount except negatively,
that is, only in the sense of not being an amount.
Intensive magnitude is at first a simple one
of many "more or less";
there are several degrees;
but they are not determined either as a unitary one or as a more or
less but only as referring to each other as outside each other,
or in the identity of the one and the "more or less than."
Thus, although the several "more
or less" are as such indeed outside the unitary degree,
the determinateness of the latter lies nonetheless
in its connection with them;
the degree thus contains amount.
Just as twenty contains as extensive magnitude
twenty ones as discrete,
the specific degree contains them as continuity,
a continuity which simply is this determinate plurality;
it is the twentieth degree,
and it is this twentieth degree
only through the intermediate of this amount
which, as such, is outside it.`,
    summary:
      'Degree contains amount: Degree is not indeterminate one. Several degrees refer to each other as outside each other. Determinateness lies in connection with them - degree contains amount. Twentieth degree contains twenty as continuity (determinate plurality) through intermediate of amount.'
  },

  {
    id: 'quantum-identity-two-sides',
    title: 'Identity: extensive and intensive are same determinateness',
    text: `The determinateness of the intensive magnitude is
to be considered, therefore, from two sides.
It is determined through other intensive quanta
and is continuous with its otherness,
so that its determinateness consists
in this connection with it.
Now, in so far as this determinateness is,
first, a simple determinateness,
it is determined as against the other degrees;
it excludes them from itself
and has its determinateness in this exclusion.
But, second, it is determined within;
this it is in the amount as its amount,
not in the amount as excluded,
or not in the amount of the other degrees.
The twentieth degree contains the twenty within itself;
it is not only determined as distinguished from the nineteenth,
the twenty-first, etc.,
but its determinateness is rather its amount.
But, inasmuch as the amount is its own,
and the determinateness is at the same time
essentially as amount, the degree is extensive quantum.

Extensive and intensive magnitude are, therefore,
one and the same determinateness of quantum;
they are distinguished only inasmuch as
the one has the amount within
and the other has the same without.
Extensive magnitude passes over into intensive magnitude
because its many collapses in and for itself
into oneness and steps outside it.
But, conversely, this simple one
has its determinateness only in the amount, its amount;
indifferent to the otherwise determined intensities,
it has the externality of amount in it;
thus intensive magnitude is just as essentially extensive magnitude.

With this identity, the qualitative something comes on the scene;
for the identity is the unity that refers back to itself
through the negation of its distinct terms;
these terms, however, make up the determinateness of
the existent magnitude.
The something is a quantum, but its qualitative existence is
now posited as indifferent to it as it is in itself.
One can speak of quantum, number as such, etc.,
without any mention of a something as their substrate.
But the something, self-mediated by virtue of the negation
of its determinations, now confronts these as existing for itself,
and, since it has a quantum, it confronts them as something
which has an extensive and intensive quantum.
Its one determinateness which it has as quantum
is posited in the distinct moments of unity and amount;
it is in itself not only one and the same determinateness,
but the positing of it in these differences as
extensive and intensive quantum is the return into this unity
which, as negative, is the something posited as indifferent to them.`,
    summary:
      'Identity: Extensive/intensive are same determinateness. Distinguished only by amount within vs without. Extensive passes into intensive (many collapses into oneness). But intensive has determinateness in amount - essentially extensive. With this identity, qualitative something appears - unity through negation of distinct terms.'
  },

  // ============================================================================
  // c. Alteration of quantum
  // ============================================================================
  {
    id: 'quantum-alteration-necessity',
    title: 'Alteration of quantum: quantum must alter',
    text: `c. Alteration of quantum

The difference between extensive and intensive quantum is
indifferent to the determinateness of quantum as quantum.
But quantum is in general
determinateness posited as sublated:
the indifferent limit, the determinateness
which is just as much the negation of itself.
In extensive quantum, this difference is developed;
but intensive magnitude is the existence of
this externality which quantum is in itself.
The difference is posited as the contradiction which it is in itself,
of being a simple self-referring determinateness
which is the negation of itself,
of having its determinateness not in it
but in another quantum.

A quantum, according to its quality, is therefore
in absolute continuity with its externality, with its otherness.
Consequently, not only can every determinateness of magnitude
be transcended, not only can it be altered:
that it must alter is now posited.
The determination of magnitude continues
into its otherness in such a way that
it has its being only in this continuity with an other;
it is not just a limit that exists but one that becomes.`,
    summary:
      'Alteration: Difference extensive/intensive is indifferent to quantum as quantum. Quantum is determinateness posited as sublated - indifferent limit that is negation of itself. Quantum is in absolute continuity with externality/otherness. It must alter - has being only in continuity with other. Limit becomes, not just exists.'
  },

  {
    id: 'quantum-sends-beyond-infinity',
    title: 'Quantum sends itself beyond itself: to infinity',
    text: `The one is infinite or self-referring negation,
and hence the repulsion of itself from itself.
Quantum is equally infinite,
posited as the self-referring negation;
it repels itself from itself.
But it is a determinate "one,"
the one which has passed over into existence and limit,
thus the repulsion of determinateness from itself,
not the generation of something that is like itself
(as the repulsion of the one) but of its otherness;
quantum is now posited in it as sending itself beyond itself.
It consists in this, that it increases or decreases;
it is within it the externality of determinateness.

Thus quantum sends itself beyond itself;
this other which it becomes is at first itself a quantum,
but a quantum which is not a static limit
but one that impels itself beyond itself.
The limit which arises in this beyond is
therefore only one that again sublates itself
and sends itself to a further limit,
and so on to infinity.`,
    summary:
      'Quantum sends beyond: Quantum is infinite (self-referring negation), repels itself from itself. Repulsion of determinateness from itself (not like itself but otherness). Quantum sends itself beyond itself (increases/decreases). Other it becomes is quantum impelling itself beyond - limit sublates and sends to further limit, to infinity.'
  }
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // a. Their difference
  // ============================================================================
  {
    id: 'op-quantum-extensive-1',
    chunkId: 'quantum-extensive-magnitude',
    label: 'Extensive magnitude: limit as plurality',
    clauses: [
      'Quantum has determinateness as limit in amount',
      'Quantum is discrete plurality',
      'Quantum is identical with its limit',
      'Limit as plurality is extensive magnitude',
      'Extensive/intensive are determinacies of quantitative limit',
      'Continuous/discrete are determinations of magnitude in itself',
      'Extensive magnitude has continuity in limit',
      'Extensive quantum is amount essentially (amount of one unit)',
      'Extensive quantum distinguished from number by explicit plurality'
    ],
    predicates: [
      { name: 'has', args: ['quantum', 'determinateness'] },
      { name: 'is', args: ['quantum', 'discretePlurality'] },
      { name: 'isIdentical', args: ['quantum', 'limit'] },
      { name: 'is', args: ['limitAsPlurality', 'extensiveMagnitude'] },
      { name: 'areDeterminacies', args: ['extensiveIntensive', 'quantitativeLimit'] },
      { name: 'areDeterminations', args: ['continuousDiscrete', 'magnitudeInItself'] },
      { name: 'has', args: ['extensiveMagnitude', 'continuity'] },
      { name: 'is', args: ['extensiveQuantum', 'amount'] },
      { name: 'isDistinguished', args: ['extensiveQuantum', 'number'] }
    ],
    relations: [
      { predicate: 'has', from: 'quantum', to: 'determinateness' },
      { predicate: 'isIdentical', from: 'quantum', to: 'limit' },
      { predicate: 'is', from: 'limitAsPlurality', to: 'extensiveMagnitude' }
    ],
    candidateSummary:
      'Extensive magnitude: Quantum has determinateness as limit in amount (plurality). Extensive/intensive are determinacies of quantitative limit. Extensive quantum is amount essentially, distinguished from number by explicit plurality.'
  },

  {
    id: 'op-quantum-number-1',
    chunkId: 'quantum-number-determinateness',
    label: 'Number\'s determinateness: self-referring limit',
    clauses: [
      'Determinateness of magnitude is limit determined for itself',
      'Limit is indifferent and self-referring',
      'In number, limit is enclosed within one',
      'Externality/reference to other is inside number',
      'Many of limit is continuous (each many is what any other is)',
      'Many collapses into continuity, becomes simple unity',
      'Amount is moment of number',
      'Ones are sublated in number\'s self-reference',
      'Externality of plurality vanishes in self-reference'
    ],
    predicates: [
      { name: 'is', args: ['determinateness', 'limit'] },
      { name: 'is', args: ['limit', 'indifferent'] },
      { name: 'isSelfReferring', args: ['limit'] },
      { name: 'isEnclosed', args: ['limit', 'one'] },
      { name: 'isInside', args: ['externality', 'number'] },
      { name: 'isContinuous', args: ['many'] },
      { name: 'collapsesInto', args: ['many', 'continuity'] },
      { name: 'becomes', args: ['many', 'simpleUnity'] },
      { name: 'isMoment', args: ['amount', 'number'] },
      { name: 'areSublated', args: ['ones', 'number'] }
    ],
    relations: [
      { predicate: 'isEnclosed', from: 'limit', to: 'one' },
      { predicate: 'isInside', from: 'externality', to: 'number' },
      { predicate: 'collapsesInto', from: 'many', to: 'continuity' }
    ],
    candidateSummary:
      'Number\'s determinateness: Limit is determined for itself, indifferent and self-referring. In number, limit enclosed within one - externality is inside. Many collapses into continuity, becomes unity. Amount is moment; ones sublated in self-reference.'
  },

  {
    id: 'op-quantum-intensive-1',
    chunkId: 'quantum-intensive-magnitude-degree',
    label: 'Intensive magnitude: degree as simple determinateness',
    clauses: [
      'Extensive limit passes over into simple determinateness',
      'Quantum is intensive magnitude',
      'Limit posited as simple is degree',
      'Degree is quantum but not aggregate',
      'Degree is plurality gathered into simple determination',
      'Degree is existence returned into being-for-itself',
      'Number expresses determinateness but as sublated amount',
      'Number is simple determinateness (onefold degree)',
      'Tenth/twentieth degree is not sum but one degree'
    ],
    predicates: [
      { name: 'passesOver', args: ['extensiveLimit', 'simpleDeterminateness'] },
      { name: 'is', args: ['quantum', 'intensiveMagnitude'] },
      { name: 'isPositedAs', args: ['limit', 'simple'] },
      { name: 'is', args: ['limit', 'degree'] },
      { name: 'is', args: ['degree', 'quantum'] },
      { name: 'isNot', args: ['degree', 'aggregate'] },
      { name: 'is', args: ['degree', 'pluralityGathered'] },
      { name: 'is', args: ['degree', 'existenceReturned'] },
      { name: 'is', args: ['number', 'sublatedAmount'] },
      { name: 'is', args: ['number', 'simpleDeterminateness'] }
    ],
    relations: [
      { predicate: 'passesOver', from: 'extensiveLimit', to: 'simpleDeterminateness' },
      { predicate: 'is', from: 'quantum', to: 'intensiveMagnitude' },
      { predicate: 'is', from: 'limit', to: 'degree' }
    ],
    candidateSummary:
      'Intensive magnitude (degree): Limit passes into simple determinateness. Degree is quantum but not aggregate - plurality gathered into simple determination, existence returned into being-for-itself. Number is sublated amount, simple determinateness.'
  },

  {
    id: 'op-quantum-degree-1',
    chunkId: 'quantum-degree-externality-quality',
    label: 'Degree\'s externality and quality',
    clauses: [
      'In intensive quantum, quantum posited according to concept',
      'Self-reference in degree is externality to itself',
      'Extensive has externality inside (numerical plurality)',
      'Externality collapses into numerical one',
      'Degree has determinateness outside it',
      'Plurality external constitutes determinateness of degree',
      'Amount sublated in extensive is posited outside intensive',
      'Degree is unitary determinateness among plurality of intensities',
      'Each degree in essential connection with others',
      'Scale of degrees is continuous progress',
      'Each has determinateness only in others',
      'In externality degree possesses its quality'
    ],
    predicates: [
      { name: 'isPosited', args: ['quantum', 'accordingToConcept'] },
      { name: 'is', args: ['selfReference', 'externality'] },
      { name: 'has', args: ['extensive', 'externalityInside'] },
      { name: 'collapsesInto', args: ['externality', 'numericalOne'] },
      { name: 'has', args: ['degree', 'determinatenessOutside'] },
      { name: 'constitutes', args: ['pluralityExternal', 'determinateness'] },
      { name: 'isPosited', args: ['amount', 'outside'] },
      { name: 'is', args: ['degree', 'unitaryDeterminateness'] },
      { name: 'isInConnection', args: ['degree', 'others'] },
      { name: 'is', args: ['scale', 'continuousProgress'] },
      { name: 'hasDeterminateness', args: ['degree', 'inOthers'] },
      { name: 'possesses', args: ['degree', 'quality'] }
    ],
    relations: [
      { predicate: 'is', from: 'selfReference', to: 'externality' },
      { predicate: 'has', from: 'degree', to: 'determinatenessOutside' },
      { predicate: 'isInConnection', from: 'degree', to: 'others' }
    ],
    candidateSummary:
      'Degree\'s externality/quality: Self-reference is externality to itself. Degree has determinateness outside (plurality external). Amount sublated in extensive is posited outside intensive. Degree is unitary among plurality, each in essential connection - scale is continuous progress. In externality it possesses quality.'
  },

  // ============================================================================
  // b. Identity of extensive and intensive magnitude
  // ============================================================================
  {
    id: 'op-quantum-identity-1',
    chunkId: 'quantum-degree-contains-amount',
    label: 'Degree contains amount: identity',
    clauses: [
      'Degree is not indeterminate one',
      'Intensive magnitude is simple one of many "more or less"',
      'Degrees refer to each other as outside each other',
      'Determinateness lies in connection with others',
      'Degree contains amount',
      'Twentieth degree contains twenty as continuity',
      'Continuity is determinate plurality',
      'Degree is twentieth through intermediate of amount'
    ],
    predicates: [
      { name: 'isNot', args: ['degree', 'indeterminateOne'] },
      { name: 'is', args: ['intensiveMagnitude', 'simpleOne'] },
      { name: 'referTo', args: ['degrees', 'eachOther'] },
      { name: 'liesIn', args: ['determinateness', 'connection'] },
      { name: 'contains', args: ['degree', 'amount'] },
      { name: 'contains', args: ['twentiethDegree', 'twenty'] },
      { name: 'is', args: ['continuity', 'determinatePlurality'] },
      { name: 'isThrough', args: ['degree', 'amount'] }
    ],
    relations: [
      { predicate: 'contains', from: 'degree', to: 'amount' },
      { predicate: 'contains', from: 'twentiethDegree', to: 'twenty' }
    ],
    candidateSummary:
      'Degree contains amount: Degree is not indeterminate one. Degrees refer to each other - determinateness lies in connection. Degree contains amount. Twentieth degree contains twenty as continuity (determinate plurality) through intermediate of amount.'
  },

  {
    id: 'op-quantum-identity-2',
    chunkId: 'quantum-identity-two-sides',
    label: 'Extensive and intensive are same determinateness',
    clauses: [
      'Intensive determinateness has two sides',
      'Determined through other intensive quanta (continuous with otherness)',
      'Simple determinateness excludes other degrees',
      'Determined within: in amount as its amount',
      'Twentieth degree contains twenty within itself',
      'Determinateness is its amount',
      'Since amount is its own, degree is extensive quantum',
      'Extensive/intensive are same determinateness',
      'Distinguished only by amount within vs without',
      'Extensive passes into intensive (many collapses into oneness)',
      'But intensive has determinateness in amount - essentially extensive',
      'With identity, qualitative something appears',
      'Unity through negation of distinct terms'
    ],
    predicates: [
      { name: 'has', args: ['intensiveDeterminateness', 'twoSides'] },
      { name: 'isDetermined', args: ['intensive', 'throughOthers'] },
      { name: 'isContinuous', args: ['intensive', 'otherness'] },
      { name: 'excludes', args: ['simpleDeterminateness', 'otherDegrees'] },
      { name: 'isDetermined', args: ['intensive', 'within'] },
      { name: 'contains', args: ['twentiethDegree', 'twenty'] },
      { name: 'is', args: ['determinateness', 'amount'] },
      { name: 'is', args: ['degree', 'extensiveQuantum'] },
      { name: 'are', args: ['extensiveIntensive', 'sameDeterminateness'] },
      { name: 'areDistinguished', args: ['extensiveIntensive', 'byAmount'] },
      { name: 'passesOver', args: ['extensive', 'intensive'] },
      { name: 'has', args: ['intensive', 'determinatenessInAmount'] },
      { name: 'is', args: ['intensive', 'essentiallyExtensive'] },
      { name: 'appears', args: ['qualitativeSomething'] },
      { name: 'is', args: ['identity', 'unity'] }
    ],
    relations: [
      { predicate: 'are', from: 'extensive', to: 'intensive' },
      { predicate: 'passesOver', from: 'extensive', to: 'intensive' },
      { predicate: 'is', from: 'intensive', to: 'essentiallyExtensive' }
    ],
    candidateSummary:
      'Identity: Extensive/intensive are same determinateness. Distinguished only by amount within vs without. Extensive passes into intensive (many collapses). But intensive has determinateness in amount - essentially extensive. With identity, qualitative something appears.'
  },

  // ============================================================================
  // c. Alteration of quantum
  // ============================================================================
  {
    id: 'op-quantum-alteration-1',
    chunkId: 'quantum-alteration-necessity',
    label: 'Quantum must alter',
    clauses: [
      'Difference extensive/intensive is indifferent to quantum as quantum',
      'Quantum is determinateness posited as sublated',
      'Quantum is indifferent limit that is negation of itself',
      'Intensive magnitude is existence of externality',
      'Difference is contradiction: simple self-referring determinateness',
      'Determinateness is negation of itself',
      'Determinateness not in it but in another quantum',
      'Quantum is in absolute continuity with externality/otherness',
      'Quantum must alter',
      'Magnitude has being only in continuity with other',
      'Limit becomes, not just exists'
    ],
    predicates: [
      { name: 'isIndifferent', args: ['difference', 'quantum'] },
      { name: 'is', args: ['quantum', 'determinateness'] },
      { name: 'isPositedAs', args: ['determinateness', 'sublated'] },
      { name: 'is', args: ['quantum', 'indifferentLimit'] },
      { name: 'is', args: ['limit', 'negation'] },
      { name: 'is', args: ['intensiveMagnitude', 'externality'] },
      { name: 'is', args: ['difference', 'contradiction'] },
      { name: 'is', args: ['determinateness', 'negation'] },
      { name: 'isNotIn', args: ['determinateness', 'it'] },
      { name: 'isIn', args: ['determinateness', 'anotherQuantum'] },
      { name: 'isIn', args: ['quantum', 'continuity'] },
      { name: 'mustAlter', args: ['quantum'] },
      { name: 'hasBeing', args: ['magnitude', 'inContinuity'] },
      { name: 'becomes', args: ['limit'] }
    ],
    relations: [
      { predicate: 'isIn', from: 'quantum', to: 'continuity' },
      { predicate: 'mustAlter', from: 'quantum', to: 'itself' }
    ],
    candidateSummary:
      'Alteration: Quantum is determinateness posited as sublated - indifferent limit that is negation of itself. Quantum is in absolute continuity with externality. It must alter - has being only in continuity with other. Limit becomes.'
  },

  {
    id: 'op-quantum-alteration-2',
    chunkId: 'quantum-sends-beyond-infinity',
    label: 'Quantum sends itself beyond itself to infinity',
    clauses: [
      'Quantum is infinite (self-referring negation)',
      'Quantum repels itself from itself',
      'Repulsion of determinateness from itself (otherness)',
      'Quantum sends itself beyond itself',
      'Quantum increases or decreases',
      'Externality of determinateness is within it',
      'Other it becomes is quantum impelling itself beyond',
      'Limit sublates and sends to further limit',
      'And so on to infinity'
    ],
    predicates: [
      { name: 'is', args: ['quantum', 'infinite'] },
      { name: 'is', args: ['quantum', 'selfReferringNegation'] },
      { name: 'repels', args: ['quantum', 'itself'] },
      { name: 'is', args: ['repulsion', 'determinateness'] },
      { name: 'sends', args: ['quantum', 'beyondItself'] },
      { name: 'increases', args: ['quantum'] },
      { name: 'decreases', args: ['quantum'] },
      { name: 'isWithin', args: ['externality', 'quantum'] },
      { name: 'is', args: ['other', 'quantum'] },
      { name: 'impels', args: ['quantum', 'beyond'] },
      { name: 'sublates', args: ['limit'] },
      { name: 'sends', args: ['limit', 'furtherLimit'] },
      { name: 'goesTo', args: ['process', 'infinity'] }
    ],
    relations: [
      { predicate: 'repels', from: 'quantum', to: 'itself' },
      { predicate: 'sends', from: 'quantum', to: 'beyondItself' },
      { predicate: 'goesTo', from: 'process', to: 'infinity' }
    ],
    candidateSummary:
      'Quantum sends beyond: Quantum is infinite (self-referring negation), repels itself. Sends itself beyond itself (increases/decreases). Other it becomes is quantum impelling itself beyond - limit sublates and sends to further limit, to infinity.'
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

