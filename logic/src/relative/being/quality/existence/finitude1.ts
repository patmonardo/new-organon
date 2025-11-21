import type { Chunk, LogicalOperation } from './index';

/*
  Finitude — B. FINITUDE: THE DETERMINATION OF BEING (Section b)

  This module covers section b: Determination, Constitution, and Limit
  - The in-itself mediated by being-for-other → determinateness existent-in-itself
  - Determination (affirmative determinateness) vs Constitution (external determinateness)
  - Their passing over into each other
  - Limit as negation-of-negation that unites and separates
  - Limit as principle/element (geometric and numerical)
  - Dialectic: no absolute point/line/plane
  - Transition to finitude (something with immanent limit)

  PHILOSOPHICAL NOTES:

  1. **Determination, Constitution, and Limit as the Hard Machinery**:
     This is the "hard machinery of reflection" - where the in-itself becomes
     determinate through its mediation by being-for-other. Determination and
     Constitution pass over into each other, revealing the dynamic structure
     of something's self-determination.

  2. **Determination ↔ Constitution**:
     Determination (affirmative determinateness, preserving self-equality)
     and Constitution (external determinateness, for-other) are distinct yet
     pass over into each other. Determinateness-as-such is their middle term.

  3. **Limit as Negation-of-Negation**:
     Limit is the negation-of-negation that both unites and separates the
     two somethings. It is immanent, self-contradictory, and drives the
     something beyond itself—this is finitude.

  4. **Limit as Principle/Element**:
     Limits are not mere boundaries but principles and elements: point is
     the beginning and element of line; line of plane; plane of solid.
     Likewise "one" as both limit and element of hundred.

  5. **Geometric Dialectic**:
     Point → Line → Plane → Space by concept, not accidental motion.
     Limits are self-contradictory beginnings that pass over; hence no
     absolute point/line/plane as fixed existents.

  6. **The Hard Machinery**:
     This section reveals the "hard machinery" of determination—how something
     maintains itself while being determined, how determination and constitution
     interact, and how limit emerges as the immanent structure that drives
     toward finitude.
*/

// ============================================================================
// b. DETERMINATION, CONSTITUTION, AND LIMIT
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'finitude-4a-in-itself-mediated',
    title: 'Determination I: the in-itself mediated by being-for-other; determinateness-in-itself',
    text: `b. Determination, constitution, and limit

The in-itself, in which the something is
reflected into itself from its being-for-other,
no longer is an abstract in-itself
but, as the negation of its being-for-other,
is mediated through this latter,
which is thus its moment.
It is not only the immediate identity of
the something with itself,
but the identity by virtue of which
the something also has present in it
what it is in itself;
the being-for-other is present in it
because the in-itself is the sublation of it,
is in itself from it;
but, because it is still abstract,
and therefore essentially affected with negation,
it is equally affected with being-for-other.
We have here not only quality and reality,
existent determinateness,
but determinateness existent-in-itself;
and the development consists in positing
such determinateness as thus immanently reflected.`,
    summary:
      'The in‑itself is no longer abstract: it is the negation of being‑for‑other and thus mediated by it (as its moment). Hence determinateness existent‑in‑itself = immanently reflected determinateness.'
  },
  {
    id: 'finitude-4b-determination-definition',
    title: 'Determination II: definition—affirmative determinateness; preserving self-equality in being-for-other',
    text: `1. The quality which in the simple
something is an in-itself
essentially in unity with
the something's other moment,
its being-in-it,
can be named its determination,
provided that this word is distinguished,
in a more precise signification,
from determinateness in general.
Determination is affirmative determinateness;
it is the in-itself by which
a something abides in its existence
while involved with an other
that would determine it,
by which it preserves itself
in its self-equality,
holding on to it in its being-for-other.`,
    summary:
      'Definition: “determination” = affirmative determinateness (distinct from determinateness-in-general). It is the in‑itself that lets something abide and preserve self‑equality within its being‑for‑other.'
  },
  {
    id: 'finitude-4c-determination-filling-presence',
    title: 'Determination III: fulfillment (filling) and presence of the in-itself',
    text: `Something fulfills its determination
to the extent that the further determinateness,
which variously accrues to it
in the measure of its being-in-itself
as it relates to an other,
becomes its filling.
Determination implies that
what something is in itself
is also present in it.`,
    summary:
      'Fulfillment: further determinateness (through relation to other) becomes the “filling” of the something. Determination implies: what it is in‑itself is present in it.'
  },
  {
    id: 'finitude-4d-example-human-rational-thought-ought',
    title: 'Example: human determination (rational thought); actuality and the “ought”',
    text: `The determination of the human being,
its vocation, is rational thought:
thinking in general is his simple determinateness;
by it the human being is distinguished from the brute;
he is thinking in himself, in so far as this thinking is
distinguished also from his being-for-other,
from his own natural and sensuous being
that brings him in immediate association with the other.
But thinking is also in him;
the human being is himself thinking,
he exists as thinking,
thought is his concrete existence and actuality;
and, further, since thinking is in his existence
and his existence is in his thinking,
thinking is concrete,
must be taken as having content and filling;
it is rational thought and as such
the determination of the human being.
But even this determination is again
only in itself, as an ought, that is to say,
it is, together with the filling embodied in its in-itself,
in the form of an in-itself in general
as against the existence which is not embodied in it
but still lies outside confronting it,
immediate sensibility and nature.`,
    summary:
      'Human determination: rational thought—both in‑itself and in him as actuality (with content). Yet as mere in‑itself it appears as an ought, standing over against sensibility/nature.'
  },
  {
    id: 'finitude-4e-constitution-definition',
    title: 'Constitution I: external determinateness vs filling; determinateness as constitution',
    text: `2. The filling of the being-in-itself with determinateness
is also distinct from the determinateness
which is only being-for-other
and remains outside the determination.
For in the sphere of the qualitative,
the distinguished terms are left, in their sublated being,
also with an immediate, qualitative being contrasting them.
That which the something has in it thus separates itself
and is from this side the external existence of
the something and also its existence,
but not as belonging to its being-in-itself.
Determinateness is thus constitution.`,
    summary:
      'Distinguish: internal “filling” of in‑itself vs determinateness that is only for‑other (external). The latter is the something’s external existence: constitution.'
  },
  {
    id: 'finitude-4f-constitution-externality-accidental',
    title: 'Constitution II: external influences/relations; seeming accident yet qualitative surrender',
    text: `Constituted in this or that way,
the something is caught up in external influences
and in external relationships.
This external connection on which the constitution depends,
and the being determined through an other,
appear as something accidental.
But it is the quality of the something
to be given over to this externality
and to have a constitution.`,
    summary:
      'Constitution depends on external relations and seems accidental; yet it belongs to the something’s quality to be given over to such externality—to have a constitution.'
  },
  {
    id: 'finitude-4g-alteration-falls-on-constitution',
    title: 'Constitution III: alteration falls on constitution; determination preserved',
    text: `In so far as something alters,
the alteration falls on the side of its constitution;
the latter is that in the something which becomes an other.
The something itself preserves itself in the alteration;
the latter affects only this unstable surface
of the something's otherness, not its determination.`,
    summary:
      'Alteration affects the constitution (the “unstable surface”); the something preserves itself—its determination is not altered.'
  },
  {
    id: 'finitude-4h-middle-determinateness-passing-over',
    title: 'Interplay I: determinateness-as-such as middle; determination ↔ constitution pass over',
    text: `Determination and constitution are thus distinct from each other;
something, according to its determination,
is indifferent to its constitution.
But that which the something has in it is
the middle term of this syllogism connecting the two,
determination and constitution.
Or, rather, the being-in-the-something showed itself
to fall apart into these two extremes.
The simple middle term is determinateness as such;
its identity belongs to determination
just as well as to constitution.
But determination passes over into constitution on its own,
and constitution into determination.
This is implied in what has been said.`,
    summary:
      'Determinateness-as-such is the middle between determination and constitution. They are distinct yet pass over into each other.'
  },
  {
    id: 'finitude-4i-how-they-pass-open-to-other',
    title: 'Interplay II: why they pass—openness to other; otherness introduced',
    text: `The connection, upon closer consideration, is this:
in so far as that which something is in itself is also in it,
the something is affected with being-for-other;
determination is therefore open, as such,
to the relation with other.
Determinateness is at the same time moment,
but it contains at the same time the qualitative distinction
of being different from being-in-itself,
of being the negative of the something,
another existence.
This determinateness which thus holds the other in itself,
united with the being-in-itself,
introduces otherness in the latter or in determination,
and determination is thereby reduced to constitution.`,
    summary:
      'Because the in‑itself is present “in it,” determination is open to otherness; determinateness carries the other within it and thereby introduces otherness into the in‑itself—reducing determination to constitution.'
  },
  {
    id: 'finitude-4j-converse-dependence-and-belonging',
    title: 'Interplay III: converse—constitution → determination; dependence and belonging',
    text: `Conversely, the being-for-other, isolated as constitution
and posited on its own, is in it the same
as what the other as such is, the other in it,
that is, the other of itself;
but it consequently is self-referring existence,
thus being-in-itself with a determinateness, therefore determination.
Consequently, inasmuch as the two are also to be held apart,
constitution, which appears to be grounded in something external,
in an other in general, also depends on determination,
and the determining from outside is at the same time
determined by the something's own immanent determination.
And further, constitution belongs to that
which something is in itself:
something alters along with its constitution.`,
    summary:
      'Conversely, constitution (for‑other) reflects back into self‑reference—determination. Even held apart, constitution depends on immanent determination and belongs to what the something is in itself; something alters with its constitution.'
  },
  {
    id: 'finitude-4k-alteration-posited-immanent-negation',
    title: 'Alteration II: now posited; negation immanent (developed in-itself)',
    text: `This altering of something is no longer
the first alteration of something merely
in accordance with its being-for-other.
The first was an alteration only implicitly present,
one that belonged to the inner concept;
now the alteration is also posited in the something.
The something itself is further determined,
and negation is posited as immanent to it,
as its developed being-in-itself.`,
    summary:
      'Alteration is no longer merely implicit (for‑other); it is posited in the something. Negation is immanent as developed being‑in‑itself.'
  },
  {
    id: 'finitude-4l-two-somethings-immanent-negation',
    title: 'Toward limit I: sublating distinction yields two somethings with immanent negation',
    text: `The transition of determination and constitution
into each other is at first the sublation of their distinction,
and existence or something in general is thereby posited;
moreover, since this something in general results
from a distinction that also includes
qualitative otherness within it,
there are two somethings.
But these are, with respect to each other,
not just others in general,
so that this negation would still be abstract
and would occur only in the comparison of the two;
rather the negation now is immanent to the somethings.
As existing, they are indifferent to each other,
but this, their affirmation, is no longer immediate:
each refers itself to itself through the intermediary of
the sublation of the otherness which in determination is
reflected into the in-itselfness.`,
    summary:
      'Sublating the distinction yields “something in general,” but with immanent otherness → two somethings. Their negation is immanent; each affirms itself only through sublating the otherness within.'
  },
  {
    id: 'finitude-4m-limit-definition',
    title: 'Toward limit II: negative unity (negation of negation) that joins/separates—limit',
    text: `Something behaves in this way in relation
to the other through itself;
since otherness is posited in it as its own moment,
its in-itselfness holds negation in itself,
and it now has its affirmative existence
through its intermediary alone.
But the other is also qualitatively distinguished
from this affirmative existence
and is thus posited outside the something.
The negation of its other is only
the quality of the something,
for it is in this sublation of its other
that it is something.
The other, for its part,
truly confronts an existence
only with this sublation;
it confronts the first something only externally,
or, since the two are in fact inherently joined together,
that is, according to their concept,
their connectedness consists in this,
that existence has passed over into otherness,
something into other;
that something is just as much an other as the other is.
Now in so far as the in-itselfness is
the non-being of the otherness
that is contained in it
but is at the same time
also distinct as existent,
something is itself negation,
the ceasing to be of an other in it;
it is posited as behaving negatively in relation to
the other and in so doing preserving itself.
This other, the in-itselfness of
the something as negation of the negation,
is the something's being-in-itself,
and this sublation is as simple negation
at the same time in it, namely, as its negation
of the other something external to it.
It is one determinateness of the two somethings
that, on the one hand, as negation of the negation,
is identical with the in-itselfness of the somethings,
and also, on the other hand, since these negations are
to each other as other somethings,
joins them together of their own accord
and, since each negation negates the other,
equally separates them.
This determinateness is limit.`,
    summary:
      'Each something’s self‑affirmation is through sublating its other; yet the other also stands outside. The immanent negation (negation of negation) both unites and separates the two—this determinateness is limit.'
  },
  {
    id: 'finitude-4n-limit-intro-contradiction',
    title: 'Limit I: concept and immediate contradiction',
    text: `3. Being-for-other is indeterminate, affirmative
association of something with its other;
in limit the non-being-for-other is emphasized,
the qualitative negation of the other,
which is thereby kept out of the something
that is reflected into itself.
We must see the development of this concept,
a development that will rather look like
confusion and contradiction.
Contradiction immediately raises its head
because limit, as an internally reflected negation of something,
ideally holds in it the moments of something and other,
and these, as distinct moments, are at the same time
posited in the sphere of existence
as really, qualitatively, distinct.`,
    summary:
      'Limit stresses non‑being‑for‑other (keeping the other out). It internally contains something/other yet these are also posited as qualitatively distinct—hence immediate contradiction.'
  },
  {
    id: 'finitude-4o-limit-nonbeing-of-other-general',
    title: 'Limit II: with respect to other; non-being of both—of something in general',
    text: `(a) Something is therefore immediate, self-referring existence
and at first it has a limit with respect to an other;
limit is the non-being of the other,
not of the something itself;
in limit, something marks the boundary of its other.
But other is itself a something in general.
The limit that something has with respect to
an other is, therefore, also the limit of
the other as a something;
it is the limit of this something in virtue of which
the something holds the first something
as its other away from itself,
or is a non-being of that something.
The limit is thus not only the non-being of the other,
but of the one something just as of the other,
and consequently of the something in general.`,
    summary:
      'Initially: limit = non‑being of the other. But since the other is a something, the same limit negates each; thus limit is non‑being of “something in general.”'
  },
  {
    id: 'finitude-4p-limit-being-and-mediation',
    title: 'Limit III: being via limit; simple negation vs negation-of-negation; mediation',
    text: `But the limit is equally, essentially,
the non-being of the other;
thus, through its limit,
something at the same time is.
In limiting, something is of course thereby
reduced to being limited itself;
but, as the ceasing of the other in it,
its limit is at the same time itself
only the being of the something;
this something is what it is by virtue of it,
has its quality in it.
This relation is the external appearance of the fact
that limit is simple negation or the first negation,
whereas the other is, at the same time,
the negation of the negation,
the in-itselfness of the something.
Something, as an immediate existence,
is therefore the limit with respect
to another something;
but it has this limit in it
and is something through the mediation of that limit,
which is just as much its non-being.
The limit is the mediation in virtue of
which something and other each both is and is not.`,
    summary:
      'Limit is also the being of the something (as ceasing of the other). Limit = simple negation; the “other” = negation‑of‑negation (in‑itself). Limit mediates: each both is and is not.'
  },
  {
    id: 'finitude-4q-limit-outside-inside-middle',
    title: 'Limit IV: outside/inside; the middle and the other of both',
    text: `(b) Now in so far as something in its limit
both is and is not,
and these moments are an immediate, qualitative distinction,
the non-existence and the existence of the something
fall outside each other.
Something has its existence outside its limit
(or, as representation would also have it, inside it);
in the same way the other, too,
since it is something, has it outside it.
The limit is the middle point between the two
at which they leave off.
They have existence beyond each other,
beyond their limit;
the limit, as the non-being of each,
is the other of both.`,
    summary:
      'With the limit, existence and non‑existence fall apart. Each something has its existence “outside” its limit; the limit is the middle and the other of both.'
  },
  {
    id: 'finitude-4r-limit-spatial-illustrations',
    title: 'Limit V: spatial illustrations (point/line/plane/solid)',
    text: `It is in accordance with this difference
of the something from its limit
that the line appears as line outside its limit, the point;
the plane as plane outside the line;
the solid as solid only outside its limiting plane.
This is the aspect of limit that
first occurs to figurative representation
(the self-external-being of the concept)
and is also most commonly assumed
in the context of spatial objects.`,
    summary:
      'Figurative view: line outside its point; plane outside its line; solid outside its plane—limit seen as external boundary (self-external concept).'
  },
  {
    id: 'finitude-4s-limit-identity-with-existence',
    title: 'Limit VI: double identity—existence only in limit; passing-over',
    text: `(c) But further, something as it is outside the limit,
as the unlimited something, is only existence in general.
As such, it is not distinguished from its other;
it is only existence
and, therefore, it and its other have the same determination;
each is only something in general or each is other;
and so both are the same.
But this, their at first immediate existence,
is now posited in them as limit:
in it both are what they are, distinct from each other.
But it is also equally their common distinguishedness,
the unity and the distinguishedness of both,
just like existence.
This double identity of the two,
existence and limit, contains this:
that something has existence only in limit,
and that, since limit and immediate existence are each
at the same time the negative of each other,
the something, which is now only in its limit,
equally separates itself from itself,
points beyond itself to its non-being
and declares it to be its being,
and so it passes over into it.`,
    summary:
      'Unlimited → mere existence (undistinguished). Existence is posited as limit: identity of existence and limit. Something exists only in its limit; thus in its limit it points beyond itself and passes over.'
  },
  {
    id: 'finitude-4t-limit-as-principle-element',
    title: 'Limit VII: limits as principles/elements (geometry and number)',
    text: `To apply this to the preceding example,
the one determination is this:
that something is what it is only in its limit.
Therefore, the point is the limit of line,
not because the latter just ceases at the point
and has existence outside it;
the line is the limit of plane,
not because the plane just ceases at it;
and the same goes for the plane as the limit of solid.
Rather, at the point the line also begins;
the point is its absolute beginning,
and if the line is represented
as unlimited on both its two sides,
or, as is said, as extended to infinity,
the point still constitutes its element,
just as the line constitutes the element of the plane,
and the plane that of the solid.
These limits are the principle
of that which they delimit;
just as one, for instance,
is as hundredth the limit,
but also the element,
of the whole hundred.`,
    summary:
      'Limit is principle/element: point is the line’s beginning and element; line that of plane; plane that of solid; likewise “one” as both limit and element of hundred.'
  },
  {
    id: 'finitude-4u-limit-dialectic-no-absolute-figures',
    title: 'Limit VIII: dialectic of point→line→plane→space; no absolute point/line/plane',
    text: `The other determination is the unrest of
the something in its limit in which it is immanent,
the contradiction that propels it beyond itself.
Thus the point is this dialectic of itself becoming line;
the line, the dialectic of becoming plane;
the plane, of becoming total space.
A second definition is given of line, plane, and whole space
which has the line come to be through the movement of the point;
the plane through the movement of the line, and so forth.
This movement of the point, the line, and so forth, is
however viewed as something accidental,
or as movement only in figurative representation.
In fact, however, this view is taken back by supposing that
the determinations from which the line, and so forth,
originate are their elements and principles,
and these are, at the same time,
nothing else but their limits;
the coming to be is not considered as accidental
or only as represented.
That the point, the line, the plane, are per se
self-contradictory beginnings which on their own
repel themselves from themselves,
and consequently that the point passes over
from itself into the line through its concept,
moves in itself and makes the line come to be,
and so on all this lies in the concept of the limit
which is immanent in the something.
The application itself, however,
belongs to the treatment of space;
as an indication of it here, we can say that
the point is the totally abstract limit,
but in a determinate existence;
this existence is still taken in total abstraction,
it is the so-called absolute, that is, abstract space,
the absolutely continuous being-outside-one-another.
Inasmuch as the limit is not abstract negation,
but is rather in this existence,
inasmuch as it is spatial determinateness,
the point is spatial, is the contradiction
of abstract negation and continuity
and is, for that reason, the transition
as it occurs and has already occurred
into the line, and so forth.
And so there is no point,
just as there is no line or plane.`,
    summary:
      'Immanent unrest: point→line→plane→space by concept, not accidental motion. Limits are contradictory beginnings that pass over; hence no absolute point/line/plane as fixed existents.'
  },
  {
    id: 'finitude-4v-finite-definition',
    title: 'Limit IX: something with immanent limit → finite',
    text: `The something, posited with its immanent limit
as the contradiction of itself by virtue of which
it is directed and driven out and beyond itself,
the finite.`,
    summary:
      'Definition: something with immanent, self-contradictory limit—driven beyond itself—is the finite.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'finitude-op-17-in-itself-mediated',
    chunkId: 'finitude-4a-in-itself-mediated',
    label: 'In-itself mediated by being-for-other; determinateness existent-in-itself',
    clauses: [
      'assert(reflectedIntoItselfFrom(Something,BeingForOther))',
      'assert(negationOf(InItself,BeingForOther))',
      'assert(mediatedBy(InItself,BeingForOther))',
      'tag(Determinateness,"existent-in-itself")',
      'tag(Determinateness,"immanently-reflected")'
    ],
    predicates: [{ name: 'InItselfMediated', args: [] }],
    relations: [
      { predicate: 'mediatedBy', from: 'InItself', to: 'BeingForOther' }
    ]
  },
  {
    id: 'finitude-op-18-determination-definition',
    chunkId: 'finitude-4b-determination-definition',
    label: 'Determination = affirmative determinateness; preserves self-equality in being-for-other',
    clauses: [
      'tag(Determination,"affirmative-determinateness")',
      'assert(distinctFrom(Determination,"determinateness-in-general"))',
      'assert(letsAbideIn(Determination,Something,Existence))',
      'assert(preservesSelfEqualityIn(Determination,Something,BeingForOther))'
    ],
    predicates: [{ name: 'DeterminationDef', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-19-determination-filling',
    chunkId: 'finitude-4c-determination-filling-presence',
    label: 'Fulfillment: further determinateness as filling; in-itself present in it',
    clauses: [
      'assert(furtherDeterminatenessAccrues(Something))',
      'assert(becomesFilling(FurtherDeterminateness,Something))',
      'assert(presenceOfInItselfIn(Something))'
    ],
    predicates: [{ name: 'DeterminationFulfillment', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-20-human-determination-ought',
    chunkId: 'finitude-4d-example-human-rational-thought-ought',
    label: 'Human determination: rational thought (in-itself and as actuality); as ought vs sensibility',
    clauses: [
      'assert(determinationOf(Human,"rational-thought"))',
      'tag(Thinking,"in-itself")',
      'tag(Thinking,"actuality-in-human")',
      'assert(hasContent(Thinking))',
      'tag(Determination,"as-ought-when-merely-in-itself")',
      'assert(standsOverAgainst(Thinking,["sensibility","nature"]))'
    ],
    predicates: [{ name: 'HumanDetermination', args: [] }],
    relations: [
      { predicate: 'determinationOf', from: 'rational-thought', to: 'Human' }
    ]
  },
  {
    id: 'finitude-op-21-constitution-definition',
    chunkId: 'finitude-4e-constitution-definition',
    label: 'Constitution = external determinateness (for-other) vs internal filling',
    clauses: [
      'assert(distinguish(Determinateness,["internal-filling","for-other"]))',
      'tag(Constitution,"external-existence-of-something")',
      'assert(notBelongingTo(InItself,Constitution))'
    ],
    predicates: [{ name: 'ConstitutionDef', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-22-constitution-externality-accident',
    chunkId: 'finitude-4f-constitution-externality-accidental',
    label: 'Constitution depends on external relations (appears accidental) yet belongs to quality',
    clauses: [
      'assert(dependsOn(Constitution,ExternalRelations))',
      'tag(Constitution,"appears-accidental")',
      'assert(givenOverToExternality(Something,true))'
    ],
    predicates: [{ name: 'ConstitutionExternality', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-23-alteration-on-constitution',
    chunkId: 'finitude-4g-alteration-falls-on-constitution',
    label: 'Alteration falls on constitution; determination preserved',
    clauses: [
      'assert(alterationFallsOn(Something,Constitution))',
      'assert(preserves(Something,Determination))',
      'tag(Constitution,"unstable-surface")'
    ],
    predicates: [{ name: 'AlterationConstitution', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-24-middle-and-passage',
    chunkId: 'finitude-4h-middle-determinateness-passing-over',
    label: 'Determinateness-as-such is middle; determination ↔ constitution pass over',
    clauses: [
      'tag(DeterminatenessAsSuch,"middle-term")',
      'assert(passesOver(Determination,Constitution))',
      'assert(passesOver(Constitution,Determination))'
    ],
    predicates: [{ name: 'MiddlePassage', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-25-open-to-other-introduces-otherness',
    chunkId: 'finitude-4i-how-they-pass-open-to-other',
    label: 'Determination open to other; determinateness introduces otherness (reduces to constitution)',
    clauses: [
      'assert(openToOther(Determination,true))',
      'assert(containsOther(Determinateness,true))',
      'assert(reducesTo(Determination,Constitution))'
    ],
    predicates: [{ name: 'OpenAndIntroduceOther', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-26-converse-dependence-belonging',
    chunkId: 'finitude-4j-converse-dependence-and-belonging',
    label: 'Constitution reflects to determination; depends on and belongs to in-itself',
    clauses: [
      'assert(reflectsTo(Constitution,Determination))',
      'assert(dependsOn(Constitution,Determination))',
      'assert(belongsTo(Constitution,InItself))',
      'assert(altersWith(Something,Constitution))'
    ],
    predicates: [{ name: 'ConstitutionDependsBelongs', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-27-alteration-posited-immanent',
    chunkId: 'finitude-4k-alteration-posited-immanent-negation',
    label: 'Alteration now posited; negation immanent (developed in-itself)',
    clauses: [
      'tag(Alteration,"posited-in-something")',
      'assert(immanentNegation(Something,true))',
      'tag(InItself,"developed")'
    ],
    predicates: [{ name: 'AlterationImmanent', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-28-two-somethings-immanent-negation',
    chunkId: 'finitude-4l-two-somethings-immanent-negation',
    label: 'Two somethings; negation immanent; affirmation via sublation of otherness',
    clauses: [
      'assert(resultIs(TwoSomethings,true))',
      'assert(immanentNegation(TwoSomethings,true))',
      'assert(affirmsThroughSublationEach(TwoSomethings,true))'
    ],
    predicates: [{ name: 'TwoSomethingsSublation', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-29-limit-definition',
    chunkId: 'finitude-4m-limit-definition',
    label: 'Limit: negation-of-negation that both unites and separates the two',
    clauses: [
      'tag(NegationOfNegation,"immanent")',
      'assert(unites(NegationOfNegation,TwoSomethings))',
      'assert(separates(NegationOfNegation,TwoSomethings))',
      'tag(Limit,"determinateness-that-unites-and-separates")',
      'assert(equals(Limit,NegationOfNegation))'
    ],
    predicates: [{ name: 'LimitDef', args: [] }],
    relations: [
      { predicate: 'equals', from: 'Limit', to: 'NegationOfNegation' }
    ]
  },
  {
    id: 'finitude-op-30-limit-intro-contradiction',
    chunkId: 'finitude-4n-limit-intro-contradiction',
    label: 'Limit emphasizes non-being-for-other; internalizes something/other → contradiction',
    clauses: [
      'tag(Limit,"non-being-for-other")',
      'assert(containsMoments(Limit,["Something","Other"]))',
      'assert(qualitativelyDistinctInExistence(["Something","Other"]))',
      'tag(Contradiction,"immediate")'
    ],
    predicates: [{ name: 'LimitIntro', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-31-limit-nonbeing-general',
    chunkId: 'finitude-4o-limit-nonbeing-of-other-general',
    label: 'Limit as non-being of other → equally of each → of something-in-general',
    clauses: [
      'assert(limitOf(Something,Other))',
      'assert(nonBeingOf(Limit,Other))',
      'assert(nonBeingOf(Limit,Something))',
      'tag(Result,"non-being-of-something-in-general")'
    ],
    predicates: [{ name: 'LimitNonBeingGeneral', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-32-limit-mediation',
    chunkId: 'finitude-4p-limit-being-and-mediation',
    label: 'Limit gives being (quality); simple negation vs negation-of-negation; mediation',
    clauses: [
      'assert(givesBeingThroughLimit(Something))',
      'tag(Limit,"simple-negation")',
      'tag(Other,"negation-of-negation")',
      'assert(mediates(Limit,["Something","Other"]))',
      'assert(eachBothIsAndIsNot(["Something","Other"]))'
    ],
    predicates: [{ name: 'LimitMediation', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-33-limit-middle-outside-inside',
    chunkId: 'finitude-4q-limit-outside-inside-middle',
    label: 'Existence vs non-existence fall apart; limit as middle/other of both',
    clauses: [
      'assert(existenceFallsOutsideItself(Something,Limit))',
      'tag(Limit,"middle")',
      'assert(otherOf(Limit,["Something","Other"]))'
    ],
    predicates: [{ name: 'LimitMiddle', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-34-spatial-illustrations',
    chunkId: 'finitude-4r-limit-spatial-illustrations',
    label: 'Spatial view: line/point, plane/line, solid/plane (external boundary representation)',
    clauses: [
      'annotate(SpatialExamples,{pairs:[["Line","Point"],["Plane","Line"],["Solid","Plane"]]})',
      'tag(Representation,"self-external")'
    ],
    predicates: [{ name: 'SpatialLimitIllustrations', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-35-identity-existence-limit',
    chunkId: 'finitude-4s-limit-identity-with-existence',
    label: 'Double identity: existence only in limit; limit/existence negate each other; passing-over',
    clauses: [
      'assert(identityOf(["Existence","Limit"]))',
      'assert(onlyIn(Something,"Existence","Limit"))',
      'assert(pointsBeyondItself(Something,Limit))',
      'tag(Passage,"posited")'
    ],
    predicates: [{ name: 'ExistenceLimitIdentity', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-36-limits-as-principles-elements',
    chunkId: 'finitude-4t-limit-as-principle-element',
    label: 'Limits are principles/elements (point→line, line→plane, plane→solid; one→hundred)',
    clauses: [
      'assert(principleOf(Point,Line))',
      'assert(principleOf(Line,Plane))',
      'assert(principleOf(Plane,Solid))',
      'assert(elementOf(Point,Line))',
      'assert(elementOf(Line,Plane))',
      'assert(elementOf(Plane,Solid))',
      'assert(elementOf(One,Hundred))'
    ],
    predicates: [{ name: 'LimitAsPrincipleElement', args: [] }],
    relations: [
      { predicate: 'principleOf', from: 'Point', to: 'Line' },
      { predicate: 'principleOf', from: 'Line', to: 'Plane' },
      { predicate: 'principleOf', from: 'Plane', to: 'Solid' }
    ]
  },
  {
    id: 'finitude-op-37-dialectic-no-absolute-figures',
    chunkId: 'finitude-4u-limit-dialectic-no-absolute-figures',
    label: 'Dialectic by concept (not accidental): no absolute point/line/plane',
    clauses: [
      'assert(passesOverByConcept(Point,Line))',
      'assert(passesOverByConcept(Line,Plane))',
      'assert(passesOverByConcept(Plane,Space))',
      'tag(Point,"self-contradictory-beginning")',
      'tag(Line,"self-contradictory-beginning")',
      'tag(Plane,"self-contradictory-beginning")',
      'assert(noAbsolute(["Point","Line","Plane"]))'
    ],
    predicates: [{ name: 'GeometricDialectic', args: [] }],
    relations: [
      { predicate: 'passesTo', from: 'Point', to: 'Line' },
      { predicate: 'passesTo', from: 'Line', to: 'Plane' },
      { predicate: 'passesTo', from: 'Plane', to: 'Space' }
    ]
  },
  {
    id: 'finitude-op-38-finite-definition',
    chunkId: 'finitude-4v-finite-definition',
    label: 'Finite = something with immanent, self-contradictory limit (driven beyond itself)',
    clauses: [
      'tag(Finite,"something-with-immanent-limit")',
      'assert(drivenBeyondItselfBy(Something,Limit))'
    ],
    predicates: [{ name: 'FiniteDef', args: [] }],
    relations: []
  }
];

// Accessors
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
