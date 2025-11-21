import type { Chunk, LogicalOperation } from './index';

/*
  Finitude — B. FINITUDE: THE DETERMINATION OF BEING

  This module consolidates the complete Finitude section:
  - a. Something and Other: symmetric otherness, being-for-other, being-in-itself, thing-in-itself
  - b. Determination, Constitution, and Limit: immanent determinateness, limit as negation-of-negation
  - c. Finitude: restriction and ought, transition to infinite

  PHILOSOPHICAL NOTES:

  1. **Finitude as the Determination of Being**:
     This is the very important "Determination of Being" - where something becomes
     something-and-other. This is realistic dualism: the transition from simple
     existence to the structured relation of something → something and other.

  2. **Realistic Dualism**:
     This is where realistic dualism emerges - something → something and other.
     The something preserves itself in its non-being, giving rise to being-for-other
     and being-in-itself as the two moments of something.

  3. **Something and Other as Foundation**:
     The symmetric relation of something and other establishes the basic structure
     of determination. Each is equally other; "this" is external pointing; language
     expresses universals. The other-for-itself (Plato's to heteron) shows nature
     as other of spirit, existing-outside-itself.

  4. **Being-for-Other and Being-in-Itself**:
     These are the two moments of something, mutually containing each other.
     Being-in-itself is self-reference as non-being of otherness; being-for-other
     is reference to non-existence that points back to being-in-itself.

  5. **Thing-in-Itself**:
     The thing-in-itself is not an empty abstraction but what something is in its
     concept. The Logic shows that in-itself is concrete, conceptually graspable,
     and inherently cognizable as the connected whole of determinations.

  6. **Determination, Constitution, and Limit**:
     The in-itself mediated by being-for-other yields determinateness existent-in-itself.
     Determination (affirmative determinateness) and Constitution (external determinateness)
     pass over into each other, yielding Limit as the negation-of-negation that both
     unites and separates.

  7. **Finitude as Immanent Limit**:
     Finitude is something with immanent, self-contradictory limit - driven beyond itself.
     The finite connects determination and limit as Ought and Restriction, mutually containing
     each other in a self-contradiction that sublates itself, transitioning to the Infinite.
*/

// ============================================================================
// B. FINITUDE: THE DETERMINATION OF BEING
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  // ============================================================================
  // Introduction/Outline
  // ============================================================================
  {
    id: 'finitude-intro-outline',
    title: 'Finitude: outline (something/other → determination/constitution/limit → finitude)',
    text: `B. FINITUDE

(a) Something and other:
at first they are indifferent to one another;
an other is also an immediate existent, a something;
the negation thus falls outside both.
Something is in itself in contrast to its being-for-other.
But the determinateness belongs also to its in-itself, and

(b) the determination of this in-itself
in turn passes over into constitution,
and this latter, as identical with determination,
constitutes the immanent and at the same time
negated being-for-another,
the limit of something which

(c) is the immanent determination of the something itself,
and the something thus is the finite.

In the first division where existence in general was considered,
this existence had, as at first taken up, the determination of an existent.
The moments of its development, quality and something,
are therefore of equally affirmative determination.
The present division, on the contrary, develops
the negative determination which is present in existence
and was there from the start only as negation in general.
It was then the first negation but has now been determined to
the point of the being-in-itself of the something,
the point of the negation of negation.`,
    summary:
      'Outline: (a) Something/other indifferent; (b) determination→constitution→limit; (c) finitude as immanent determination. This division develops negative determination to negation-of-negation (being-in-itself).'
  },

  // ============================================================================
  // a. Something and Other
  // ============================================================================
  {
    id: 'finitude-1a-something-other-symmetric',
    title: 'Something↔Other I: symmetry; "this" as external pointing',
    text: `a. Something and other:

1. Something and other are,
first, both existents or something.
Second, each is equally an other.
It is indifferent which is named first,
and just for this reason it is named something
(in Latin, when they occur in a proposition,
both are aliud, or "the one, the other," alius alium;
in the case of an alternating relation,
the analogous expression is alter alterum).
If of two beings we call the one A and the other B,
the B is the one which is first determined as other.
But the A is just as much the other of the B.
Both are other in the same way.
"This" serves to fix the distinction
and the something which is to be taken in the affirmative sense.
But "this" also expresses the fact that the distinction,
and the privileging of one something,
is a subjective designation that falls outside the something itself.
The whole determinateness falls on the side of this external pointing;
also the expression "this" contains no distinctions;
each and every something is just as good a "this" as any other.
By "this" we mean to express something completely determinate,
overlooking the fact that language, as a work of the understanding,
only expresses the universal, albeit naming it as a single object.
But an individual name is something meaningless in the sense
that it does not express a universal.
It appears as something merely posited
and arbitrary for the same reason
that proper names can also be arbitrarily picked,
arbitrarily given as well as arbitrarily altered.`,
    summary:
      'Symmetry: both are existents; each is equally other. Any "this" that fixes one side is an external, subjective pointing; language names universals, so "this" and proper names lack intrinsic determination.'
  },
  {
    id: 'finitude-1b-otherness-as-external',
    title: 'Something↔Other II: otherness as alien/external (Third\'s comparison)',
    text: `Otherness thus appears as a determination
alien to the existence thus pointed at,
or the other existence as outside this one existence,
partly because the one existence is determined as other
only by being compared by a Third,
and partly because it is so determined
only on account of the other which is outside it,
but is not an other for itself.
At the same time, as has been remarked,
even for ordinary thinking every existence
equally determines itself as an other existence,
so that there is no existence
that remains determined simply as an existence,
none which is not outside an existence
and therefore is not itself an other.`,
    summary:
      'Externalization: at first, "other" seems alien—assigned by a Third or by comparison to an outside. Yet, for ordinary thinking, every existence also determines itself as other; none is simply "just existence."'
  },
  {
    id: 'finitude-1c-other-for-itself',
    title: 'Something↔Other III: sameness of determinations vs other-for-itself',
    text: `Both are determined as something as well as other:
thus they are the same and there is as yet
no distinction present in them.
But this sameness of determinations, too,
falls only within external reflection,
in the comparison of the two;
but the other, as posited at first,
though an other with reference to something,
is other also for itself apart from the something.`,
    summary:
      'Externally, both share "something/other," seeming the same. But beyond external reflection, the other is also other-for-itself, not merely relative to a given something.'
  },
  {
    id: 'finitude-1d-to-heteron-nature',
    title: 'Other-in-itself: Plato\'s to heteron; nature as other of spirit',
    text: `Third, the other is therefore to be taken in isolation,
with reference to itself, has to be taken abstractly as the other,
the 'to heteron' of Plato who opposes it to the one
as a moment of totality,
and in this way ascribes to the other a nature of its own.
Thus the other, taken solely as such,
is not the other of something,
but is the other within, that is, the other of itself.
Such an other, which is the other by its own determination,
is physical nature; nature is the other of spirit;
this, its determination, is at first
a mere relativity expressing not a quality of nature itself
but only a reference external to it.
But since spirit is the true something,
and hence nature is what it is within only in contrast to spirit,
taken for itself the quality of nature is just this,
to be the other within, that which-exists-outside-itself
(in the determinations of space, time, matter).`,
    summary:
      'Other-in-itself: as Plato\'s to heteron—moment of totality—other is "other within," other of itself. Nature, as other of spirit, exemplifies this: existing-outside-itself (space, time, matter).'
  },
  {
    id: 'finitude-1e-other-of-other-alteration-identity',
    title: 'Other-of-other: absolute inequality, alteration, and self-identity',
    text: `The other which is such for itself is the other within it,
hence the other of itself and so the other of the other;
therefore, the absolutely unequal in itself,
that which negates itself, alters itself.
But it equally remains identical with itself,
for that into which it alters is the other,
and this other has no additional determination;
but that which alters itself is not determined in
any other way than in this, to be an other;
in going over to this other, it only unites with itself.
It is thus posited as reflected into itself
with sublation of the otherness,
a self-identical something from which the otherness,
which is at the same time a moment of it, is therefore distinct,
itself not appertaining to it as something.`,
    summary:
      'Other-for-itself is other-of-itself (other-of-other): absolute inequality that negates/alters itself; yet the passage unites with itself (other adds nothing new). Hence reflected-into-itself: self-identical something with otherness as a distinct moment.'
  },
  {
    id: 'finitude-2a-preserves-in-nonbeing-bfo-bii',
    title: 'Something VI: preserves itself in non-being; being-for-other; being-in-itself',
    text: `2. The something preserves itself in its non-being;
it is essentially one with it, and essentially not one with it.
It therefore stands in reference to an otherness
without being just this otherness.
The otherness is at once contained in it
and yet separated from it;
it is being-for-other.
Existence as such is an immediate, bare of references;
or, it is in the determination of being.
However, as including non-being within itself,
existence is determinate being,
being negated within itself,
and then in the first instance an other;
but, since in being negated it preserves itself
at the same time, it is only being-for-other.
It preserves itself in its non-being and is being;
not, however, being in general but being with reference
to itself in contrast to its reference to the other,
as self-equality in contrast to its inequality.
Such a being is being-in-itself.`,
    summary:
      'Something preserves itself in its non-being: essentially one with it and not-one. Otherness is both contained and separated → being-for-other. As negated-within-itself, existence is determinate being and, at first, only being-for-other; as self-equality against the other, this is being-in-itself.'
  },
  {
    id: 'finitude-2b-two-pairs-moments-connection',
    title: 'Something VII: two pairs; truth as connection; mutual containment of moments',
    text: `Being-for-other and being-in-itself constitute
the two moments of something.
There are here two pairs of determinations:
(1) something and other;
(2) being-for-other and being-in-itself.
The former contain the non-connectedness of their determinateness;
something and other fall apart.
But their truth is their connection;
being-for-other and being-in-itself are
therefore the same determinations posited as
moments of one and the same unity,
as determinations which are connections
and which, in their unity,
remain in the unity of existence.
Each thus itself contains within it, at the same time,
also the moment diverse from it.
Being and nothing in their unity, which is existence,
are no longer being and nothing
(these they are only outside their unity);
so in their restless unity, in becoming,
they are coming-to-be and ceasing-to-be.
In the something, being is being-in-itself.
Now, as self-reference, self-equality,
being is no longer immediately,
but is self-reference only as the non-being of otherness
(as existence reflected into itself).
The same goes for non-being:
as the moment of something in this
unity of being and non-being:
it is not non-existence in general
but is the other, and more determinedly,
according as being is at the same time distinguished from it,
it is reference to its non-existence, being-for-other.`,
    summary:
      'Two pairs: (something|other) vs (being-for-other|being-in-itself). Truth = their connection: both are moments of one unity and mutually contain each other. In something: being = being-in-itself (self-reference as non-being of otherness); non-being = other, i.e., being-for-other (reference to non-existence).'
  },
  {
    id: 'finitude-2c-being-in-itself-dual',
    title: 'Something VIII: being-in-itself—dual character',
    text: `Hence being-in-itself is, first,
negative reference to non-existence;
it has otherness outside it and is opposed to it;
in so far as something is in itself,
it is withdrawn from being-other and being-for-other.
But, second, it has non-being also right in it;
for it is itself the non-being of being-for-other.`,
    summary:
      'Being-in-itself: (1) negative reference to non-existence—withdrawn from being-(for-)other; (2) equally has non-being in it—namely, as the non-being of being-for-other.'
  },
  {
    id: 'finitude-2d-being-for-other-dual',
    title: 'Something IX: being-for-other—dual character; reciprocal pointing',
    text: `But being-for-other is, first, the negation of
the simple reference of being to itself
which, in the first place, is supposed
to be existence and something;
in so far as something is in an other or for an other,
it lacks a being of its own.
But, second, it is not non-existence as pure nothing;
it is non-existence that points to being-in-itself
as its being reflected into itself,
just as conversely the being-in-itself points to being-for-other.`,
    summary:
      'Being-for-other: (1) negates simple self-reference (lack of its own being when "in/for" an other); (2) not pure nothing—its non-existence points to being-in-itself, and conversely being-in-itself points back to being-for-other.'
  },
  {
    id: 'finitude-3a-moments-of-something',
    title: 'Something X: both moments are determinations of one and the same something',
    text: `3. Both moments are determinations of one and the same,
namely of something.
Something is in-itself in so far as it has returned
from the being-for-other back to itself.
But something has also a determination or circumstance,
whether in itself (here the accent is on the in) or in it;
in so far as this circumstance is in it externally,
it is a being-for-other.
This leads to a further determination.
Being-in-itself and being-for-other
are different at first.
But that something also has in it what it is in itself and
conversely is in itself also what it is as being-for-other
this is the identity of being-in-itself and being-for-other,
in accordance with the determination
that the something is itself
one and the same something of both moments,
and these are in it, therefore, undivided.
This identity already occurs formally in the sphere of existence,
but more explicitly in the treatment of essence
and later of the relations of interiority and externality,
and in the most determinate form in the treatment of the idea,
as the unity of concept and actuality.`,
    summary:
      'Both moments are determinations of one something. Initially different, but something has in it what it is in itself and conversely—identity of being-in-itself and being-for-other. This identity appears formally in existence, more explicitly in essence, most determinately in the idea.'
  },
  {
    id: 'finitude-3b-in-itself-as-abstract',
    title: 'Something XI: in-itself as abstract vs concrete; "in it" implies inner worth',
    text: `Opinion has it that with the in-itself
something lofty is being said, as with the inner;
but what something is only in itself, is also only in it;
in-itself is a merely abstract,
and hence itself external determination.
The expressions:
there is nothing in it,
or there is something in it,
imply, though somewhat obscurely,
that what is in a thing also pertains
to its in-itselfness, to its inner, true worth.`,
    summary:
      'In-itself is abstract, hence external. But "in it" implies what pertains to in-itselfness, inner true worth—the concrete determination.'
  },
  {
    id: 'finitude-3c-thing-in-itself-meaning',
    title: 'Something XII: thing-in-itself as empty abstraction vs concrete concept',
    text: `It may be observed that here we have
the meaning of the thing-in-itself.
It is a very simple abstraction,
though it was for a while a very important determination,
something sophisticated, as it were,
just as the proposition that we know nothing of
what things are in themselves was a much valued piece of wisdom.
Things are called "in-themselves" in so far as abstraction
is made from all being-for-other, which really means,
in so far as they are thought without all determination, as nothing.
In this sense, of course, it is impossible to know
what the thing-in-itself is.
For the question "what?" calls for determinations to be produced;
but since the things of which the determinations are called for
are at the same time presumed to be things-in-themselves,
which means precisely without determination,
the impossibility of an answer is thoughtlessly implanted in the question,
or else a senseless answer is given.
The thing-in-itself is the same as that absolute
of which nothing is known except that in it all is one.
What there is in these things-in-themselves is therefore very well known;
they are as such nothing but empty abstractions void of truth.
What, however, the thing-in-itself in truth is,
what there basically is in it,
of this the Logic is the exposition.
But in this Logic something better is understood by the in-itself
than an abstraction, namely, what something is in its concept;
but this concept is in itself concrete:
as concept, in principle conceptually graspable;
and, as determined and as the connected whole
of its determinations, inherently cognizable.`,
    summary:
      'Thing-in-itself as empty abstraction (without determination = nothing) vs concrete concept. The Logic shows in-itself as what something is in its concept—concrete, conceptually graspable, inherently cognizable as connected whole of determinations.'
  },
  {
    id: 'finitude-3d-being-vs-essence-positing',
    title: 'Something XIII: being-in-itself vs positedness; sphere of being vs essence',
    text: `Being-in-itself has at first the being-for-other
as a moment standing over against it.
But positedness also comes to be positioned over against it,
and, although in this expression being-for-other is also included,
the expression still contains the determination of the bending back,
which has already occurred,
of that which is not in itself into that wherein it is
positive, and this is its being-in-itself.
Being-in-itself is normally to be taken
as an abstract way of expressing the concept;
positing, strictly speaking, first occurs
in the sphere of essence, of objective reflection;
the ground posits that which is grounded through it;
more strongly, the cause produces an effect,
an existence whose subsistence is immediately negated
and which carries the meaning that it has its substance,
its being, in an other.
In the sphere of being, existence only emerges out of becoming.
Or again, with the something an other is posited;
with the finite, an infinite;
but the finite does not bring forth the infinite,
does not posit it.
In the sphere of being, the self-determining of
the concept is at first only in itself or implicit,
and for that reason it is called a transition or passing over.`,
    summary:
      'Being-in-itself vs positedness. In being: transition/passing over (implicit self-determining). In essence: positing (ground posits grounded, cause produces effect). The finite does not posit the infinite—only transition.'
  },
  {
    id: 'finitude-3e-reflecting-determinations',
    title: 'Something XIV: reflecting determinations vs qualitative independence',
    text: `And the reflecting determinations of being,
such as something and other,
or finite and infinite,
although they essentially point to one another,
or are as being-for-other,
also stand on their own qualitatively; the other exists;
the finite, like the infinite, is equally to be regarded
as an immediate existent that stands firm on its own;
the meaning of each appears complete even without its other.
The positive and the negative, on the contrary, cause and effect,
however much they are taken in isolation,
have at the same time no meaning each without the other;
their reflective shining in each other,
the shine in each of its other,
is present right in them.
In the different cycles of determination
and especially in the progress of the exposition,
or, more precisely, in the progress of the concept
in the exposition of itself,
it is of capital concern always to clearly distinguish
what still is in itself or implicitly
and what is posited,
how determinations are in the concept
and how they are as posited
or as existing-for-other.
This is a distinction that belongs only to
the dialectical development and one unknown
to metaphysical philosophizing
(to which the critical also belongs);
the definitions of metaphysics,
like its presuppositions, distinctions, and conclusions,
are meant to assert and produce only the existent
and that, too, as existent-in-itself.`,
    summary:
      'Reflecting determinations (something/other, finite/infinite) stand qualitatively independent; each appears complete. Positive/negative, cause/effect: no meaning without the other—reflective shining present. Key distinction: in-itself/implicit vs posited/existing-for-other—dialectical, unknown to metaphysics.'
  },
  {
    id: 'finitude-3f-identity-reflected-determinateness',
    title: 'Something XV: identity of being-for-other and in-itself; reflected determinateness',
    text: `In the unity of the something with itself,
being-for-other is identical with its in-itself;
the being-for-other is thus in the something.
The determinateness thus reflected into itself is
therefore again a simple existent
and hence again a quality: determination.`,
    summary:
      'In unity: being-for-other identical with in-itself; being-for-other is in the something. Determinateness reflected into itself = simple existent = quality: determination.'
  },

  // ============================================================================
  // b. Determination, Constitution, and Limit
  // ============================================================================
  // Note: This section is extensive. Including key chunks here, with reference
  // to finitude1.ts for the complete translation of this section.
  
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
  
  // Note: The complete section b (Determination, Constitution, and Limit) 
  // contains 22 chunks covering: determination definition, filling, human example,
  // constitution definition, externality, alteration, middle term, passing over,
  // two somethings, limit definition, limit contradictions, limit as non-being,
  // limit mediation, limit outside/inside, spatial illustrations, limit identity,
  // limits as principles/elements, dialectic, finite definition.
  // For the complete upgraded translation of section b, see finitude1.ts as reference.
  // The key insight: determination ↔ constitution pass over; limit unites/separates.

  // ============================================================================
  // c. Finitude
  // ============================================================================
  {
    id: 'finitude-5a-finitude-intro-definition',
    title: 'Finitude I: quality as limit; immanent negation as finitude',
    text: `c. Finitude

Existence is determinate.
Something has a quality,
and in this quality it is
not only determined but delimited;
its quality is its limit and, affected by it,
something remains affirmative, quiescent existence.
But, so developed that the opposition of its existence
and of the negation as the limit immanent to this existence is
the very in-itselfness of the something,
and this is thus only becoming in it,
this negation constitutes the finitude of the something.`,
    summary:
      'Quality is limit. When the limit (negation) is immanent to existence as its in‑itself, it renders the something a mere becoming—this immanent negation is finitude.'
  },
  {
    id: 'finitude-5b-finite-nonbeing-as-being-perishing',
    title: 'Finitude II: non-being as being; finite self-transcendence and perishing',
    text: `When we say of things that they are finite,
we understand by this that they not only have a determinateness,
that their quality is not only reality
and existent determination,
that they are not merely limited
and as such still have existence outside their limit,
but rather that non-being constitutes their nature, their being.
Finite things are, but in their reference to themselves
they refer to themselves negatively
in this very self-reference
they propel themselves beyond themselves,
beyond their being.
They are, but the truth of this being is
(as in Latin) their finis, their end.
The finite does not just alter,
as the something in general does,
but perishes, and its perishing is
not just a mere possibility,
as if it might be without perishing.
Rather, the being as such of finite things is
to have the germ of this transgression
in their in-itselfness:
the hour of their birth is the hour of their death.`,
    summary:
      'Finite = non‑being constitutes its being. It self‑negates and propels beyond itself; its truth is finis (end). Perishing is essential (germ in its in‑itself): birth already entails death.'
  },
  
  // Note: Section c continues with (a) immediacy, (b) restriction/ought, (c) transition to infinite.
  // For complete upgraded translation, see finitude2.ts as reference.
  // Key insights: finitude as obstinate category; ought↔restriction mutually contain;
  // finite as self-contradiction; transition to infinite via negation-of-negation.
];

// ============================================================================
// LOGICAL OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'finitude-op-intro-outline',
    chunkId: 'finitude-intro-outline',
    label: 'Outline: something/other → determination/constitution/limit → finitude (negation-of-negation)',
    clauses: [
      'assert(division("Finitude","negative-determination"))',
      'assert(developsTo(Negation,"negation-of-negation"))',
      'assert(beingInItselfOf(Something,"negation-of-negation"))',
      'tag(Finitude,"immanent-determination-of-something")'
    ],
    predicates: [{ name: 'FinitudeOutline', args: [] }],
    relations: [
      { predicate: 'developsTo', from: 'Negation', to: 'NegationOfNegation' }
    ]
  },
  {
    id: 'finitude-op-1-symmetric-otherness-and-this',
    chunkId: 'finitude-1a-something-other-symmetric',
    label: 'Symmetric otherness; "this" as external subjective designation',
    clauses: [
      'assert(bothAre(["Something","Other"],"existents"))',
      'assert(eachEquallyOther("Something","Other"))',
      'tag(This,"external-pointing")',
      'tag(This,"subjective-designation")',
      'annotate(Language,{expresses:"universal"})',
      'assert(properNamesArbitrary(true))'
    ],
    predicates: [{ name: 'SymmetricOtherness', args: [] }],
    relations: [
      { predicate: 'equallyOtherThan', from: 'Something', to: 'Other' },
      { predicate: 'equallyOtherThan', from: 'Other', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-2-externalization-by-third',
    chunkId: 'finitude-1b-otherness-as-external',
    label: 'Otherness as alien/external (comparison by a Third); yet self-othering in ordinary thinking',
    clauses: [
      'assert(determinedAsOtherByThird(Existence))',
      'assert(outsideOf(Other,Something))',
      'tag(Otherness,"alien")',
      'assert(everyExistenceSelfOthers(true))'
    ],
    predicates: [{ name: 'ExternalOtherness', args: [] }],
    relations: [
      { predicate: 'comparedBy', from: 'Existence', to: 'Third' },
      { predicate: 'outsideOf', from: 'Other', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-3-other-for-itself',
    chunkId: 'finitude-1c-other-for-itself',
    label: 'Beyond external reflection: the other-for-itself',
    clauses: [
      'tag(Other,"for-itself")',
      'assert(notMerelyRelativeTo(Other,Something))',
      'annotate(Reflection,{external:true})'
    ],
    predicates: [{ name: 'OtherForItself', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-4-to-heteron-nature-of-spirit',
    chunkId: 'finitude-1d-to-heteron-nature',
    label: 'Plato\'s to heteron; nature as other of spirit (outside-itself)',
    clauses: [
      'tag(Other,"to-heteron")',
      'assert(momentOfTotality(Other))',
      'assert(otherOf(Nature,Spirit))',
      'tag(Nature,"exists-outside-itself")',
      'annotate(Nature,{determinations:["space","time","matter"]})'
    ],
    predicates: [{ name: 'OtherInItself', args: [] }],
    relations: [
      { predicate: 'otherOf', from: 'Nature', to: 'Spirit' }
    ]
  },
  {
    id: 'finitude-op-5-other-of-other-alteration-identity',
    chunkId: 'finitude-1e-other-of-other-alteration-identity',
    label: 'Other-of-other: absolute inequality; alteration that unites with itself',
    clauses: [
      'tag(Other,"other-of-itself")',
      'tag(Other,"other-of-other")',
      'assert(negatesItself(Other))',
      'assert(altersItself(Other))',
      'assert(remainsIdenticalThrough(Other,"passage-into-other"))',
      'assert(reflectedIntoItself(Something))',
      'assert(sublatesOtherness(Something))',
      'annotate(Something,{otherness:"distinct-moment"})'
    ],
    predicates: [{ name: 'AlterationAndIdentity', args: [] }],
    relations: [
      { predicate: 'reflectsIntoItself', from: 'Something', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-6-preserves-in-nonbeing',
    chunkId: 'finitude-2a-preserves-in-nonbeing-bfo-bii',
    label: 'Preserves itself in non-being; contained-and-separated otherness → being-for-other; toward being-in-itself',
    clauses: [
      'assert(preservesItselfIn(Something,NonBeing))',
      'assert(essentiallyOneWith(Something,NonBeing))',
      'assert(essentiallyNotOneWith(Something,NonBeing))',
      'tag(Otherness,"contained-and-separated")',
      'tag(BeingForOther,"defined")',
      'assert(determinateBeing(Existence))',
      'assert(negatedWithinItself(Existence))',
      'tag(Something,"at-first-being-for-other")',
      'tag(BeingInItself,"named-as-self-equality")'
    ],
    predicates: [{ name: 'PreservationInNonBeing', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-7-moments-and-connection',
    chunkId: 'finitude-2b-two-pairs-moments-connection',
    label: 'Two pairs; truth as connection; mutual containment; being↔non-being reinterpreted',
    clauses: [
      'assert(pairs([["Something","Other"],["BeingForOther","BeingInItself"]]))',
      'assert(truthIsConnection(["BeingForOther","BeingInItself"]))',
      'assert(momentOf(BeingForOther,Something))',
      'assert(momentOf(BeingInItself,Something))',
      'assert(containsMoment(BeingForOther,BeingInItself))',
      'assert(containsMoment(BeingInItself,BeingForOther))',
      'tag(Being,"as-being-in-itself")',
      'tag(NonBeing,"as-being-for-other")'
    ],
    predicates: [{ name: 'MomentsConnected', args: [] }],
    relations: [
      { predicate: 'momentOf', from: 'BeingForOther', to: 'Something' },
      { predicate: 'momentOf', from: 'BeingInItself', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-8-being-in-itself-dual',
    chunkId: 'finitude-2c-being-in-itself-dual',
    label: 'Being-in-itself: (1) negative reference to non-existence; (2) non-being of being-for-other',
    clauses: [
      'tag(BeingInItself,"negative-reference-to-non-existence")',
      'assert(hasOthernessOutside(BeingInItself))',
      'assert(withdrawnFrom(BeingInItself,["BeingOther","BeingForOther"]))',
      'assert(nonBeingOf(BeingInItself,BeingForOther))'
    ],
    predicates: [{ name: 'BeingInItselfDual', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-9-being-for-other-dual',
    chunkId: 'finitude-2d-being-for-other-dual',
    label: 'Being-for-other: (1) negation of simple self-reference; (2) points to being-in-itself (and back)',
    clauses: [
      'tag(BeingForOther,"negation-of-self-reference")',
      'assert(lacksOwnBeingWhenForOther(Something))',
      'tag(BeingForOther,"not-pure-nothing")',
      'assert(pointsTo(BeingForOther,BeingInItself))',
      'assert(pointsTo(BeingInItself,BeingForOther))'
    ],
    predicates: [{ name: 'BeingForOtherDual', args: [] }],
    relations: [
      { predicate: 'pointsTo', from: 'BeingForOther', to: 'BeingInItself' },
      { predicate: 'pointsTo', from: 'BeingInItself', to: 'BeingForOther' }
    ]
  },
  {
    id: 'finitude-op-10-moments-of-something',
    chunkId: 'finitude-3a-moments-of-something',
    label: 'Both moments are determinations of one something; identity of being-in-itself and being-for-other',
    clauses: [
      'assert(determinationsOf(Something,["BeingInItself","BeingForOther"]))',
      'assert(initiallyDifferent(["BeingInItself","BeingForOther"]))',
      'assert(hasInIt(Something,"what-it-is-in-itself"))',
      'assert(hasInItself(Something,"what-it-is-as-being-for-other"))',
      'assert(identityOf(["BeingInItself","BeingForOther"]))',
      'tag(Identity,"appears-formally-in-existence")',
      'tag(Identity,"more-explicitly-in-essence")',
      'tag(Identity,"most-determinately-in-idea")'
    ],
    predicates: [{ name: 'MomentsIdentity', args: [] }],
    relations: [
      { predicate: 'identityOf', from: 'BeingInItself', to: 'BeingForOther' }
    ]
  },
  {
    id: 'finitude-op-11-in-itself-abstract',
    chunkId: 'finitude-3b-in-itself-as-abstract',
    label: 'In-itself as abstract vs concrete; "in it" implies inner worth',
    clauses: [
      'tag(InItself,"abstract")',
      'tag(InItself,"external-determination")',
      'assert(implies("in-it","inner-true-worth"))',
      'assert(pertainsTo("in-it","in-itselfness"))'
    ],
    predicates: [{ name: 'InItselfAbstract', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-12-thing-in-itself',
    chunkId: 'finitude-3c-thing-in-itself-meaning',
    label: 'Thing-in-itself: empty abstraction (without determination = nothing) vs concrete concept',
    clauses: [
      'tag(ThingInItself,"empty-abstraction-when-without-determination")',
      'assert(equals(ThingInItselfWithoutDetermination,Nothing))',
      'assert(impossibleToKnow(ThingInItself,"when-empty-abstraction"))',
      'assert(LogicShows(InItself,"concrete-concept"))',
      'assert(conceptuallyGraspable(InItself,true))',
      'assert(inherentlyCognizable(InItself,"as-connected-whole-of-determinations"))'
    ],
    predicates: [{ name: 'ThingInItself', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-13-being-vs-essence',
    chunkId: 'finitude-3d-being-vs-essence-positing',
    label: 'Being: transition/passing over (implicit) vs Essence: positing (explicit)',
    clauses: [
      'tag(SphereOfBeing,"transition-passing-over")',
      'tag(SelfDetermining,"implicit-in-being")',
      'tag(SphereOfEssence,"positing")',
      'assert(groundPosits(Ground,Grounded))',
      'assert(causeProduces(Cause,Effect))',
      'assert(doesNotPosit(Finite,Infinite))',
      'assert(onlyTransition(Finite,Infinite))'
    ],
    predicates: [{ name: 'BeingVsEssence', args: [] }],
    relations: [
      { predicate: 'posits', from: 'Ground', to: 'Grounded' },
      { predicate: 'produces', from: 'Cause', to: 'Effect' }
    ]
  },
  {
    id: 'finitude-op-14-reflecting-determinations',
    chunkId: 'finitude-3e-reflecting-determinations',
    label: 'Reflecting determinations: qualitatively independent vs positive/negative: no meaning without other',
    clauses: [
      'assert(qualitativelyIndependent(["Something","Other"]))',
      'assert(qualitativelyIndependent(["Finite","Infinite"]))',
      'assert(completeWithoutOther(["Something","Other","Finite","Infinite"]))',
      'assert(noMeaningWithoutOther(["Positive","Negative"]))',
      'assert(noMeaningWithoutOther(["Cause","Effect"]))',
      'assert(reflectiveShiningPresent(["Positive","Negative","Cause","Effect"]))',
      'tag(Distinction,"in-itself-implicit-vs-posited-existing-for-other")',
      'tag(Distinction,"dialectical-unknown-to-metaphysics")'
    ],
    predicates: [{ name: 'ReflectingDeterminations', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-15-identity-reflected-determinateness',
    chunkId: 'finitude-3f-identity-reflected-determinateness',
    label: 'Identity: being-for-other = in-itself; reflected determinateness = quality: determination',
    clauses: [
      'assert(identityInUnity(BeingForOther,InItself))',
      'assert(inSomething(BeingForOther,true))',
      'assert(reflectedIntoItself(Determinateness))',
      'assert(equals(ReflectedDeterminateness,"simple-existent"))',
      'assert(equals(ReflectedDeterminateness,Quality))',
      'assert(equals(Quality,Determination))'
    ],
    predicates: [{ name: 'IdentityReflectedDeterminateness', args: [] }],
    relations: [
      { predicate: 'equals', from: 'ReflectedDeterminateness', to: 'Quality' },
      { predicate: 'equals', from: 'Quality', to: 'Determination' }
    ]
  },
  {
    id: 'finitude-op-16-in-itself-mediated',
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
    id: 'finitude-op-17-finitude-definition',
    chunkId: 'finitude-5a-finitude-intro-definition',
    label: 'Quality as limit; immanent negation constitutes finitude',
    clauses: [
      'assert(qualityAsLimit(Something))',
      'assert(immanentNegation(Something,true))',
      'tag(Finitude,"immanent-negation")',
      'assert(constitutes(Finitude,Something))'
    ],
    predicates: [{ name: 'FinitudeIntro', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-18-finite-perishing-essential',
    chunkId: 'finitude-5b-finite-nonbeing-as-being-perishing',
    label: 'Finite: non-being as being; self-transcendence; essential perishing',
    clauses: [
      'assert(nonBeingConstitutesBeing(Finite,true))',
      'assert(propelsBeyondItself(Finite,true))',
      'tag(Finis,"end-of-being")',
      'assert(essentialPerishing(Finite,true))',
      'assert(germOfTransgressionIn(Finite,"in-itself"))'
    ],
    predicates: [{ name: 'FiniteEssence', args: [] }],
    relations: []
  }
];

// Accessors
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS;
}

export function getLogicalOpsForChunk(oneBasedIndex: number): LogicalOperation[] {
  const chunk = getChunk(oneBasedIndex);
  if (!chunk) return [];
  return LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id);
}
