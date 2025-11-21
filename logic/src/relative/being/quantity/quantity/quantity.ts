import type { Chunk, LogicalOperation } from '../../../types';

/*
  Quantity — SECTION II: MAGNITUDE (QUANTITY)

  This module translates the complete Quantity section:
  - A. Pure Quantity: sublated being-for-itself, continuity and discreteness
  - B. Continuous and Discrete Magnitude: the two moments of quantity
  - C. The Limiting of Quantity: discrete magnitude as quantum

  PHILOSOPHICAL NOTES:

  1. **Quantity as Sublated Being-for-Itself**:
     Quantity is the determinateness that has become indifferent to being - a limit
     which is just as much no limit. Being-for-itself has passed over into attraction,
     where the repelling one behaves towards the other as identical to itself.

  2. **Continuity and Discreteness**:
     Quantity contains the two moments of continuity and discreteness. Continuity
     is the simple, self-same reference to itself unbroken by any limit or exclusion.
     Discreteness is the moment of repulsion, the outsideness-of-one-another. These
     are not separate but unified in quantity.

  3. **Transition from Quality to Quantity**:
     Quality is the first, immediate determinateness. Quantity is determinateness
     that has become indifferent to being - where existence surfaces again on continuity
     and determinateness is posited as repelling itself from itself, referring to
     itself in the determinateness of an other existence.

  4. **Laws of Mathematical Cognition**:
     Quantity → Quantum → Ratio → Measure
     This progression establishes the foundation for mathematical cognition:
     - Thing (Entity): Quantum as determinate quantity
     - World/Law (Property): Ratio as the relation of quanta
     - EssentialRelation (Aspect): Measure as the unity of quality and quantity

     This connects to the Form Processor structure:
     - Essence → Shape (Form)
     - Foundation → Context (Reflection)
     - Ground → Morph (Active Ground)
     - Appearance → Entity/Property/Aspect (Thing/World/Relation)

     + Kriya connects this to @reality IN @reality OUT nondual.

  5. **Theory of Measure**:
     The progression Quantity → Quantum → Ratio culminates in Measure, where
     quality and quantity are united. This is the foundation for the "Laws of
     Mathematical Cognition" - the systematic structure that makes mathematical
     thinking explicit and computable.
*/

// ============================================================================
// SECTION II: MAGNITUDE (QUANTITY)
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  // ============================================================================
  // Introduction: Quantity vs Quality
  // ============================================================================
  {
    id: 'quantity-intro-quality-difference',
    title: 'Quantity vs Quality: determinateness indifferent to being',
    text: `SECTION II

Magnitude (Quantity)

The difference between quantity and quality has been indicated.
Quality is the first, immediate determinateness.
Quantity is the determinateness that
has become indifferent to being;
a limit which is just as much no limit;
being-for-itself which is absolutely
identical with being-for-another:
the repulsion of the many ones
which is immediate non-repulsion,
their continuity.

Because that which exists for itself is now so posited
that it does not exclude its other
but rather affirmatively continues in it,
it is then otherness, inasmuch as
existence surfaces again on this continuity
and its determinateness is at the same time
no longer simple self-reference,
no longer the immediate determinateness
of the existent something,
but is posited as repelling itself from itself,
as referring to itself in the determinateness
rather of an other existence
(a being which exists for itself);
and since they are at the same time indifferent limits,
reflected into themselves and unconnected,
determinateness is as such outside itself,
an absolute externality and a something just as external;
such a limit, the indifference of the limit as limit
and the indifference of the something to the limit,
constitutes the quantitative determinateness of the something.

In the first place, we have to distinguish pure quantity
from quantity as determinate, from quantum.
First, pure quantity is real being-for-itself
turned back into itself, with as yet no determinateness in it:
a compact, infinite unity which continues itself into itself.

Second, this quantity proceeds to determinateness,
and this is posited in it as a determinateness
that at the same time is none, is only external.
Quantity becomes quantum.
Quantum is indifferent determinateness,
that is, one that transcends itself, negates itself;
as this otherness of otherness,
it lapses into infinite progress.
Infinite quantum, however, is
sublated indifferent determinateness:
it is the restoration of quality.

Third, quantum in qualitative form is quantitative ratio.
Quantum transcends itself only in general;
in the ratio, however, it transcends itself into its otherness,
in such a way that this otherness in which it has its determination
is at the same time posited, is another quantum.
With this we have quantum as turned back into itself
and referring to itself as into its otherness.

At the foundation of this relation there still
lies the externality of quantum;
it is indifferent quanta that
relate themselves to each other,
that is, they have the reference
that mutually connects them
in this being-outside-itself.
The ratio is, therefore, only
a formal unity of quality and quantity,
and dialectic is its transition into
their absolute unity, in measure.`,
    summary:
      'Quantity vs Quality: Quantity is determinateness indifferent to being, a limit which is just as much no limit. Progression: Pure Quantity → Quantum → Ratio → Measure. Ratio is formal unity of quality/quantity, transitioning to absolute unity in measure.'
  },

  // ============================================================================
  // A. PURE QUANTITY
  // ============================================================================
  {
    id: 'quantity-pure-intro',
    title: 'Pure Quantity: sublated being-for-itself, attraction',
    text: `CHAPTER 1

Quantity

A. PURE QUANTITY

Quantity is sublated being-for-itself.
The repelling one that behaved only
negatively towards the excluded one,
now that it has gone over in connection with it,
behaves towards the other as identical to itself
and has therefore lost its determination;
being-for-itself has passed over into attraction.
The absolute obduracy of the one has melted away
into this unity which, however, as containing the one, is
at the same time determined by the repulsion residing in it;
as unity of the self-externality, it is unity with itself.
Attraction is in this way the moment of continuity in quantity.`,
    summary:
      'Pure Quantity: Sublated being-for-itself. Repelling one now behaves as identical to itself; being-for-itself passes into attraction. The one\'s obduracy melts into unity (containing repulsion); attraction is continuity.'
  },

  {
    id: 'quantity-pure-continuity',
    title: 'Continuity: simple self-same reference, unity of ones',
    text: `Continuity is therefore simple, self-same reference to itself
unbroken by any limit or exclusion;
not, however, immediate unity but the unity of ones
which have existence for themselves.
Still contained in it is the outside-one-another of plurality,
though at the same time as something without distinctions, unbroken.
Plurality is posited in continuity as it implicitly is in itself;
the many are each what the others are,
each is like the other,
and the plurality is, consequently,
simple and undifferentiated equality.
Continuity is this moment of self-equality
of the outsideness-of-one-another,
the self-continuation of the different ones
into the ones from which they are distinguished.`,
    summary:
      'Continuity: Simple self-same reference unbroken by limit/exclusion. Unity of ones (not immediate unity). Contains outsideness-of-one-another without distinctions. Many are each what others are - simple undifferentiated equality. Self-continuation of different ones into ones from which distinguished.'
  },

  {
    id: 'quantity-pure-discreteness',
    title: 'Discreteness: repulsion as moment in quantity',
    text: `In continuity, therefore, magnitude immediately possesses
the moment of discreteness, repulsion as now a moment in quantity.
Steady continuity is self-equality,
but of many that do not become exclusive;
it is repulsion that first expands self-equality to continuity.
Hence discreteness is, for its part, a discreteness of confluents,
of ones that do not have the void to connect them,
not the negative, but their own steady advance
and, in the many, do not interrupt this self-equality.`,
    summary:
      'Discreteness: Magnitude immediately possesses discreteness (repulsion) as moment in quantity. Repulsion expands self-equality to continuity. Discreteness is of confluents (ones without void), steady advance that does not interrupt self-equality.'
  },

  {
    id: 'quantity-pure-unity',
    title: 'Quantity as unity of continuity and discreteness',
    text: `Quantity is the unity of these moments,
of continuity and discreteness.
At first, however, it is this continuity
in the form of one of them, of continuity,
as a result of the dialectic of the being-for-itself
which has collapsed into the form of self-equal immediacy.
Quantity is as such this simple result
in so far as the being-for-itself has
not yet developed its moments
and has not posited them within it.
Quantity contains these moments at first
as being-for-itself posited in its truth.
It was the determination of being-in-itself
to be self-sublating self-reference,
a perpetual coming-out-of-itself.
But what is repelled is itself;
repulsion is thus a creative flowing away from itself.
On account of the sameness of what is repelled,
this discerning is unbroken continuity;
and on account of the coming-out-of-itself,
this continuity is at the same time,
without being broken off, a plurality,
a plurality which persists just as
immediately in its equality with itself.`,
    summary:
      'Quantity: Unity of continuity and discreteness. At first continuity (being-for-itself collapsed to self-equal immediacy). Contains moments as being-for-itself in truth. Repulsion is creative flowing away; sameness yields unbroken continuity; coming-out-of-itself yields plurality persisting in equality.'
  },

  // ============================================================================
  // B. CONTINUOUS AND DISCRETE MAGNITUDE
  // ============================================================================
  {
    id: 'quantity-continuous-discrete-intro',
    title: 'Continuous and Discrete Magnitude: two moments of quantity',
    text: `B. CONTINUOUS AND DISCRETE MAGNITUDE

1. Quantity contains the two moments of continuity and discreteness.
It is to be posited in both, in each as its determination.
It is already from the start the immediate unity of the two,
that is, quantity is itself posited at first
only in one of the two determinations, that of continuity,
and as such is continuous magnitude.

Or continuity is indeed one of the moments of quantity
which is brought to completion only with the other, discreteness.
But quantity is concrete unity only in so far as
it is the unity of distinct moments.
These are to be taken, therefore, also as distinct,
without however resolving them again
into attraction and repulsion
but, rather, as they truly are,
each remaining in its unity with the other,
that is, remaining the whole.
Continuity is only the compact unity
holding together as unity of the discrete;
posited as such, it is no longer only moment
but the whole quantity: continuous magnitude.`,
    summary:
      'Continuous/Discrete Magnitude: Quantity contains both moments. Posited at first as continuity (continuous magnitude). Continuity is compact unity holding discrete together; posited as such, it is whole quantity, not just moment.'
  },

  {
    id: 'quantity-continuous-magnitude',
    title: 'Continuous Magnitude: immediate quantity',
    text: `2. Immediate quantity is continuous magnitude.
Quantity, however, is not as such an immediate;
immediacy is a determinateness,
the sublated being of which is precisely quantity.
Quantity is to be posited, therefore,
in the determinateness immanent to it,
and this is the one.
Quantity is discrete magnitude.`,
    summary:
      'Continuous Magnitude: Immediate quantity is continuous magnitude. But quantity is not immediate; immediacy is sublated. Quantity must be posited in immanent determinateness (the one) - discrete magnitude.'
  },

  {
    id: 'quantity-discrete-magnitude',
    title: 'Discrete Magnitude: outsideness-of-one-another as discontinuous',
    text: `Discreteness is, like continuity, a moment of quantity,
but is itself also the whole quantity
just because it is a moment in it, the whole,
and therefore as distinct moment does not
diverge from its unity with the other moment.
Quantity is the outsideness-of-one-another as such,
and continuous magnitude is this outsideness-of-one-another
onwardly positing itself without negation
as an internally self-same connectedness.
On the other hand, discrete magnitude is this
outsideness-of-one-another as discontinuous, as broken off.
With this aggregate of ones, however,
the aggregate of atom and void,
repulsion in general, is not thereby reinstated.
Because discrete magnitude is quantity,
its discreteness is itself continuous.
Such a continuity in the discrete consists
in the ones being the same as one another,
or in that they have the same unity.
Discrete magnitude is therefore
the one-outside-the-other of
the many ones as of a same;
not the many ones in general,
but posited rather as the many of a unity.`,
    summary:
      'Discrete Magnitude: Moment and whole of quantity. Outsideness-of-one-another as discontinuous/broken off. But discreteness is itself continuous (ones are same, have same unity). Discrete magnitude is many ones of a unity, not many ones in general.'
  },

  // ============================================================================
  // C. THE LIMITING OF QUANTITY
  // ============================================================================
  {
    id: 'quantity-limiting-intro',
    title: 'Limiting of Quantity: discrete magnitude as quantum',
    text: `C. THE LIMITING OF QUANTITY

Discrete magnitude has,
first, the one for its principle
and, second, is a plurality of ones;
third, it is essentially continuous,
it is the one as at the same time
sublated, as unity,
the self-continuing as such
in the discreteness of the ones.
Consequently, it is posited as one magnitude,
and the "one" is its determinateness;
a "one" which, in this posited and determinate existence,
excludes, is a limit to the unity.
Discrete magnitude as such is not supposed
to be immediately limited,
but, when distinguished from continuous magnitude,
it is an existence and a something,
the determinateness of which,
and in it also the first negation and limit,
is the "one."`,
    summary:
      'Limiting of Quantity: Discrete magnitude has one as principle, plurality of ones, essentially continuous (one sublated as unity). Posited as one magnitude; "one" is determinateness/limit. Distinguished from continuous magnitude, it is existence/something with "one" as limit.'
  },

  {
    id: 'quantity-limiting-encompassing',
    title: 'Encompassing limit: one as enclosing limit',
    text: `This limit, besides referring to the unity
and being the moment of negation in it,
is also, as one, self-referred;
thus it is enclosing, encompassing limit.
The limit here is not at first distinct
from the something of its existence,
but, as one, is essentially this negative point itself.
But the being which is here limited is essentially as continuity,
and in virtue of this continuity it transcends the limit,
transcends this one, and is indifferent to it.
Real, discrete quantity is thus one quantity, or quantum:
quantity as an existence and a something.`,
    summary:
      'Encompassing Limit: Limit refers to unity (negation moment) and is self-referred as one - enclosing/encompassing. Limit is negative point itself (not distinct from something). But limited being is continuity, transcending limit, indifferent to it. Discrete quantity is quantum: quantity as existence/something.'
  },

  {
    id: 'quantity-limiting-quantum',
    title: 'Quantum: limit encompassing many ones, passing over',
    text: `Since the one which is a limit encompasses
within it the many ones of discrete quantity,
it posits them equally as sublated in it;
it is a limit to continuity simply as such
and, consequently, the distinction between
continuous and discrete magnitude is here indifferent;
or, more precisely, it is a limit to
the continuity of the one
just as much as of the other;
in this, both pass over into being quanta.`,
    summary:
      'Quantum: One as limit encompasses many ones (sublated in it). Limit to continuity; distinction continuous/discrete becomes indifferent. Limit to continuity of one and other; both pass over into quanta.'
  }
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // Introduction: Quantity vs Quality
  // ============================================================================
  {
    id: 'op-quantity-intro-1',
    chunkId: 'quantity-intro-quality-difference',
    label: 'Quantity as determinateness indifferent to being',
    clauses: [
      'Quality is first immediate determinateness',
      'Quantity is determinateness indifferent to being',
      'Quantity is limit which is just as much no limit',
      'Quantity is being-for-itself identical with being-for-another',
      'Quantity is repulsion of many ones as immediate non-repulsion (continuity)'
    ],
    predicates: [
      { name: 'isDeterminateness', args: ['Quality', 'immediate'] },
      { name: 'isDeterminateness', args: ['Quantity', 'indifferentToBeing'] },
      { name: 'isLimit', args: ['Quantity', 'limitAndNoLimit'] },
      { name: 'isIdentical', args: ['beingForItself', 'beingForAnother'] },
      { name: 'isContinuity', args: ['repulsion', 'nonRepulsion'] }
    ],
    relations: [
      { predicate: 'transitions', from: 'Quality', to: 'Quantity' },
      { predicate: 'contains', from: 'Quantity', to: 'continuity' }
    ],
    candidateSummary:
      'Quantity differs from Quality: determinateness indifferent to being, limit which is no limit, continuity of repulsion.'
  },

  {
    id: 'op-quantity-intro-2',
    chunkId: 'quantity-intro-quality-difference',
    label: 'Progression: Pure Quantity → Quantum → Ratio → Measure',
    clauses: [
      'Pure quantity is being-for-itself turned back into itself',
      'Quantity proceeds to determinateness (quantum)',
      'Quantum transcends itself into infinite progress',
      'Infinite quantum is restoration of quality',
      'Quantum in qualitative form is quantitative ratio',
      'Ratio is formal unity of quality and quantity',
      'Ratio transitions to absolute unity in measure'
    ],
    predicates: [
      { name: 'isBeingForItself', args: ['pureQuantity'] },
      { name: 'proceedsTo', args: ['quantity', 'determinateness'] },
      { name: 'transcends', args: ['quantum', 'itself'] },
      { name: 'isRestoration', args: ['infiniteQuantum', 'quality'] },
      { name: 'isFormalUnity', args: ['ratio', 'qualityAndQuantity'] },
      { name: 'transitionsTo', args: ['ratio', 'measure'] }
    ],
    relations: [
      { predicate: 'proceeds', from: 'PureQuantity', to: 'Quantum' },
      { predicate: 'proceeds', from: 'Quantum', to: 'Ratio' },
      { predicate: 'transitions', from: 'Ratio', to: 'Measure' }
    ],
    candidateSummary:
      'Progression: Pure Quantity (being-for-itself) → Quantum (determinateness) → Ratio (formal unity) → Measure (absolute unity).'
  },

  // ============================================================================
  // A. PURE QUANTITY
  // ============================================================================
  {
    id: 'op-quantity-pure-1',
    chunkId: 'quantity-pure-intro',
    label: 'Quantity as sublated being-for-itself',
    clauses: [
      'Quantity is sublated being-for-itself',
      'Repelling one behaves as identical to itself',
      'Being-for-itself passes over into attraction',
      'Obduracy of one melts into unity',
      'Unity contains repulsion',
      'Attraction is moment of continuity'
    ],
    predicates: [
      { name: 'isSublated', args: ['Quantity', 'beingForItself'] },
      { name: 'behavesAs', args: ['repellingOne', 'identicalToItself'] },
      { name: 'passesOver', args: ['beingForItself', 'attraction'] },
      { name: 'meltsInto', args: ['obduracy', 'unity'] },
      { name: 'contains', args: ['unity', 'repulsion'] },
      { name: 'isMoment', args: ['attraction', 'continuity'] }
    ],
    relations: [
      { predicate: 'sublates', from: 'Quantity', to: 'beingForItself' },
      { predicate: 'passesOver', from: 'beingForItself', to: 'attraction' }
    ],
    candidateSummary:
      'Quantity sublates being-for-itself: repelling one becomes identical to itself, passes into attraction (continuity).'
  },

  {
    id: 'op-quantity-pure-2',
    chunkId: 'quantity-pure-continuity',
    label: 'Continuity as simple self-same reference',
    clauses: [
      'Continuity is simple self-same reference to itself',
      'Continuity is unbroken by limit or exclusion',
      'Continuity is unity of ones (not immediate unity)',
      'Continuity contains outsideness-of-one-another without distinctions',
      'Many are each what others are (simple equality)',
      'Continuity is self-continuation of different ones'
    ],
    predicates: [
      { name: 'isSelfSameReference', args: ['continuity'] },
      { name: 'isUnbroken', args: ['continuity'] },
      { name: 'isUnity', args: ['continuity', 'ones'] },
      { name: 'contains', args: ['continuity', 'outsideness'] },
      { name: 'areEqual', args: ['many'] },
      { name: 'isSelfContinuation', args: ['continuity'] }
    ],
    relations: [
      { predicate: 'contains', from: 'continuity', to: 'outsideness' },
      { predicate: 'unifies', from: 'continuity', to: 'ones' }
    ],
    candidateSummary:
      'Continuity: Simple self-same reference unbroken by limit; unity of ones containing outsideness; many are equal; self-continuation.'
  },

  {
    id: 'op-quantity-pure-3',
    chunkId: 'quantity-pure-discreteness',
    label: 'Discreteness as moment in quantity',
    clauses: [
      'Magnitude possesses discreteness as moment',
      'Discreteness is repulsion as moment in quantity',
      'Repulsion expands self-equality to continuity',
      'Discreteness is of confluents (ones without void)',
      'Discreteness is steady advance not interrupting self-equality'
    ],
    predicates: [
      { name: 'possesses', args: ['magnitude', 'discreteness'] },
      { name: 'isMoment', args: ['discreteness', 'quantity'] },
      { name: 'expands', args: ['repulsion', 'selfEquality'] },
      { name: 'isOf', args: ['discreteness', 'confluents'] },
      { name: 'doesNotInterrupt', args: ['discreteness', 'selfEquality'] }
    ],
    relations: [
      { predicate: 'possesses', from: 'magnitude', to: 'discreteness' },
      { predicate: 'expands', from: 'repulsion', to: 'continuity' }
    ],
    candidateSummary:
      'Discreteness: Repulsion as moment in quantity; expands self-equality to continuity; confluents without void; steady advance.'
  },

  {
    id: 'op-quantity-pure-4',
    chunkId: 'quantity-pure-unity',
    label: 'Quantity as unity of continuity and discreteness',
    clauses: [
      'Quantity is unity of continuity and discreteness',
      'Quantity is continuity at first (collapsed being-for-itself)',
      'Quantity contains moments as being-for-itself in truth',
      'Repulsion is creative flowing away from itself',
      'Sameness of repelled yields unbroken continuity',
      'Coming-out-of-itself yields plurality in equality'
    ],
    predicates: [
      { name: 'isUnity', args: ['Quantity', 'continuityAndDiscreteness'] },
      { name: 'isContinuity', args: ['Quantity', 'atFirst'] },
      { name: 'contains', args: ['Quantity', 'moments'] },
      { name: 'isCreative', args: ['repulsion'] },
      { name: 'yields', args: ['sameness', 'continuity'] },
      { name: 'yields', args: ['comingOut', 'plurality'] }
    ],
    relations: [
      { predicate: 'unifies', from: 'Quantity', to: 'continuity' },
      { predicate: 'unifies', from: 'Quantity', to: 'discreteness' }
    ],
    candidateSummary:
      'Quantity: Unity of continuity/discreteness. At first continuity (collapsed being-for-itself). Repulsion creative; sameness yields continuity; coming-out yields plurality.'
  },

  // ============================================================================
  // B. CONTINUOUS AND DISCRETE MAGNITUDE
  // ============================================================================
  {
    id: 'op-quantity-continuous-discrete-1',
    chunkId: 'quantity-continuous-discrete-intro',
    label: 'Quantity posited in both moments',
    clauses: [
      'Quantity contains continuity and discreteness',
      'Quantity is to be posited in both moments',
      'Quantity is immediate unity of the two',
      'Quantity is posited at first as continuity',
      'Continuity is compact unity holding discrete together',
      'Posited as such, continuity is whole quantity'
    ],
    predicates: [
      { name: 'contains', args: ['Quantity', 'continuity'] },
      { name: 'contains', args: ['Quantity', 'discreteness'] },
      { name: 'isUnity', args: ['Quantity', 'immediate'] },
      { name: 'isPositedAs', args: ['Quantity', 'continuity'] },
      { name: 'holdsTogether', args: ['continuity', 'discrete'] },
      { name: 'isWhole', args: ['continuity', 'quantity'] }
    ],
    relations: [
      { predicate: 'contains', from: 'Quantity', to: 'continuity' },
      { predicate: 'contains', from: 'Quantity', to: 'discreteness' }
    ],
    candidateSummary:
      'Quantity contains both moments; posited at first as continuity (compact unity holding discrete together).'
  },

  {
    id: 'op-quantity-continuous-discrete-2',
    chunkId: 'quantity-continuous-magnitude',
    label: 'Continuous magnitude as immediate quantity',
    clauses: [
      'Immediate quantity is continuous magnitude',
      'Quantity is not as such immediate',
      'Immediacy is determinateness sublated by quantity',
      'Quantity must be posited in immanent determinateness',
      'Immanent determinateness is the one',
      'Quantity is discrete magnitude'
    ],
    predicates: [
      { name: 'is', args: ['immediateQuantity', 'continuousMagnitude'] },
      { name: 'isNot', args: ['Quantity', 'immediate'] },
      { name: 'isSublated', args: ['immediacy', 'quantity'] },
      { name: 'mustBePositedIn', args: ['Quantity', 'immanentDeterminateness'] },
      { name: 'is', args: ['immanentDeterminateness', 'one'] },
      { name: 'is', args: ['Quantity', 'discreteMagnitude'] }
    ],
    relations: [
      { predicate: 'is', from: 'immediateQuantity', to: 'continuousMagnitude' },
      { predicate: 'mustBePositedIn', from: 'Quantity', to: 'one' }
    ],
    candidateSummary:
      'Immediate quantity is continuous magnitude, but quantity is not immediate; must be posited in one (discrete magnitude).'
  },

  {
    id: 'op-quantity-continuous-discrete-3',
    chunkId: 'quantity-discrete-magnitude',
    label: 'Discrete magnitude as many ones of unity',
    clauses: [
      'Discreteness is moment and whole of quantity',
      'Discrete magnitude is outsideness-of-one-another as discontinuous',
      'Discrete magnitude is not aggregate of atom and void',
      'Discreteness is itself continuous',
      'Continuity in discrete: ones are same, have same unity',
      'Discrete magnitude is many ones of a unity'
    ],
    predicates: [
      { name: 'isMoment', args: ['discreteness', 'quantity'] },
      { name: 'isWhole', args: ['discreteness', 'quantity'] },
      { name: 'is', args: ['discreteMagnitude', 'discontinuous'] },
      { name: 'isNot', args: ['discreteMagnitude', 'atomAndVoid'] },
      { name: 'isContinuous', args: ['discreteness'] },
      { name: 'areSame', args: ['ones'] },
      { name: 'is', args: ['discreteMagnitude', 'manyOfUnity'] }
    ],
    relations: [
      { predicate: 'is', from: 'discreteMagnitude', to: 'discontinuous' },
      { predicate: 'is', from: 'discreteness', to: 'continuous' }
    ],
    candidateSummary:
      'Discrete magnitude: Discontinuous outsideness, but discreteness is continuous (ones same, same unity); many ones of unity.'
  },

  // ============================================================================
  // C. THE LIMITING OF QUANTITY
  // ============================================================================
  {
    id: 'op-quantity-limiting-1',
    chunkId: 'quantity-limiting-intro',
    label: 'Discrete magnitude as one magnitude',
    clauses: [
      'Discrete magnitude has one as principle',
      'Discrete magnitude is plurality of ones',
      'Discrete magnitude is essentially continuous',
      'Discrete magnitude is one as sublated (unity)',
      'Discrete magnitude is posited as one magnitude',
      'One is determinateness/limit',
      'Distinguished from continuous, it is existence/something'
    ],
    predicates: [
      { name: 'has', args: ['discreteMagnitude', 'one'] },
      { name: 'is', args: ['discreteMagnitude', 'plurality'] },
      { name: 'isContinuous', args: ['discreteMagnitude'] },
      { name: 'isSublated', args: ['one', 'unity'] },
      { name: 'isPositedAs', args: ['discreteMagnitude', 'oneMagnitude'] },
      { name: 'is', args: ['one', 'determinateness'] },
      { name: 'is', args: ['discreteMagnitude', 'existence'] }
    ],
    relations: [
      { predicate: 'has', from: 'discreteMagnitude', to: 'one' },
      { predicate: 'isPositedAs', from: 'discreteMagnitude', to: 'oneMagnitude' }
    ],
    candidateSummary:
      'Discrete magnitude: One as principle, plurality, essentially continuous (one sublated as unity); posited as one magnitude with one as limit.'
  },

  {
    id: 'op-quantity-limiting-2',
    chunkId: 'quantity-limiting-encompassing',
    label: 'Encompassing limit transcended by continuity',
    clauses: [
      'Limit refers to unity (negation moment)',
      'Limit is self-referred as one (enclosing)',
      'Limit is negative point itself',
      'Limited being is continuity',
      'Continuity transcends limit',
      'Continuity is indifferent to limit',
      'Discrete quantity is quantum (existence/something)'
    ],
    predicates: [
      { name: 'refersTo', args: ['limit', 'unity'] },
      { name: 'isSelfReferred', args: ['limit'] },
      { name: 'is', args: ['limit', 'negativePoint'] },
      { name: 'is', args: ['limitedBeing', 'continuity'] },
      { name: 'transcends', args: ['continuity', 'limit'] },
      { name: 'isIndifferent', args: ['continuity', 'limit'] },
      { name: 'is', args: ['discreteQuantity', 'quantum'] }
    ],
    relations: [
      { predicate: 'refersTo', from: 'limit', to: 'unity' },
      { predicate: 'transcends', from: 'continuity', to: 'limit' }
    ],
    candidateSummary:
      'Limit: Enclosing negative point referring to unity. But limited being is continuity, transcending/indifferent to limit. Discrete quantity is quantum.'
  },

  {
    id: 'op-quantity-limiting-3',
    chunkId: 'quantity-limiting-quantum',
    label: 'Both continuous and discrete pass over into quanta',
    clauses: [
      'One as limit encompasses many ones',
      'Limit posits many ones as sublated',
      'Limit is limit to continuity',
      'Distinction continuous/discrete becomes indifferent',
      'Limit is to continuity of one and other',
      'Both pass over into quanta'
    ],
    predicates: [
      { name: 'encompasses', args: ['one', 'manyOnes'] },
      { name: 'positsAs', args: ['limit', 'manyOnes', 'sublated'] },
      { name: 'isLimitTo', args: ['limit', 'continuity'] },
      { name: 'becomesIndifferent', args: ['distinction'] },
      { name: 'isLimitTo', args: ['limit', 'continuityOfOneAndOther'] },
      { name: 'passOver', args: ['both', 'quanta'] }
    ],
    relations: [
      { predicate: 'encompasses', from: 'one', to: 'manyOnes' },
      { predicate: 'passOver', from: 'continuousMagnitude', to: 'quanta' },
      { predicate: 'passOver', from: 'discreteMagnitude', to: 'quanta' }
    ],
    candidateSummary:
      'One as limit encompasses many ones (sublated); limit to continuity makes distinction indifferent; both pass over into quanta.'
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

