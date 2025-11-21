import type { Chunk, LogicalOperation } from './index';

/*
  Finitude — B. FINITUDE: THE DETERMINATION OF BEING (Section c)

  This module covers section c: Finitude proper
  - Finitude as immanent negation (quality as limit)
  - Non-being as being: finite self-transcendence and perishing
  - (a) Immediacy of finitude: mournful, obstinate category
  - (b) Restriction and the ought: determination and limit as moments
  - (c) Transition to infinite: perishing perishes, contradiction resolves

  PHILOSOPHICAL NOTES:

  1. **Finitude as Immanent Negation**:
     Finitude is quality as limit, where the limit (negation) is immanent
     to existence as its in-itself. This renders the something a mere becoming—
     the immanent negation is finitude itself.

  2. **Non-Being as Being**:
     Finite things have non-being as their nature, their being. They refer
     to themselves negatively and propel themselves beyond themselves. Their
     truth is finis (end). Perishing is essential—the hour of birth is the
     hour of death.

  3. **The Mournful Category**:
     Finitude is "mournful" because it fixes qualitative negation: nothing/
     perishing opposed to being. Unlike constitution/limit, it won't reconcile—
     hence the understanding's most obstinate category.

  4. **Restriction and the Ought**:
     The finite connects determination and limit as Ought and Restriction.
     Both are moments of the finite, both themselves finite. The ought contains
     double determination: in-itselfness over against negation, and non-being
     as restriction.

  5. **The Transition to Infinite**:
     The finite's contradiction collapses internally, resolving itself. The
     perishing perishes—the nothing is not the last. This transition reveals
     that finitude is not absolute but passes over into the infinite.

  6. **Simple Yet Complex**:
     As the user noted, this is "simple yet complex!"—the determination of being
     reveals its deepest structure: how something maintains itself while being
     determined, how finitude emerges from limit, and how the finite transitions
     to the infinite.
*/

// ============================================================================
// c. FINITUDE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
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
  {
    id: 'finitude-5c-immediacy-mournful-obstinate',
    title: 'Immediacy I: mournful category; finitude as fixed negation',
    text: `(a) The immediacy of finitude

The thought of the finitude of things
brings this mournful note with it
because finitude is qualitative negation driven to the extreme,
and in the simplicity of such a determination
there is no longer left to things
an affirmative being distinct from their determination,
as things destined to ruin.
Because of this qualitative simplicity of negation
that has returned to the abstract opposition of
nothing and perishing to being,
finitude is the most obstinate of
the categories of the understanding;
negation in general, constitution, limit,
are compatible with their other, with existence;
even the abstract nothing, by itself,
is given up as an abstraction;
but finitude is negation fixed in itself
and, as such, stands in stark contrast to its affirmative.`,
    summary:
      'Finitude appears “mournful” because it fixes qualitative negation: nothing/perishing opposed to being. Unlike constitution/limit, it won’t reconcile—hence the understanding’s most obstinate category.'
  },
  {
    id: 'finitude-5d-refusal-of-infinite-eternal-finitude',
    title: 'Immediacy II: refusal of union with the infinite; “eternal” finitude',
    text: `The finite thus does indeed let itself be submitted to flux;
this is precisely what it is,
that it should come to an end,
and this end is its only determination.
Its refusal is rather to let itself be brought
affirmatively to its affirmative, the infinite,
to be associated with it;
it is therefore inseparably posited with its nothing,
and thereby cut off from any reconciliation
with its other, the affirmative.
The determination of finite things does not go past their end.
The understanding persists in this sorrow of finitude,
for it makes non-being the determination of things
and, at the same time, this non-being imperishable and absolute.
Their transitoriness would only pass away in their other,
in the affirmative;
their finitude would then be severed from them;
but this finitude is their unalterable quality, that is,
their quality which does not pass over into their other, that is,
not into the affirmative;
and so finitude is eternal.`,
    summary:
      'Understanding fixes finitude as refusal of the infinite: glued to its nothing, unreconciled. Thus it deems finitude “eternal” (non‑being as imperishable determination).'
  },
  {
    id: 'finitude-5e-stance-and-decision-on-perishing',
    title: 'Immediacy III: what is last—does perishing persist or perish?',
    text: `This is a very important consideration.
But that the finite is absolute is
certainly not a standpoint that any philosophy or outlook,
or the understanding, would want to endorse.
The opposite is rather expressly present
in the assertion of finitude:
the finite is the restricted, the perishable,
the finite is only the finite, not the imperishable;
all this is immediately part and parcel
of its determination and expression.
But all depends on whether
in one's view of finitude its being is insisted on,
and the transitoriness thus persists,
or whether the transitoriness and the perishing perish.`,
    summary:
      'Key fork: either insist on finite being so that transitoriness persists, or let transitoriness itself perish (perishing of the perishing).'
  },
  {
    id: 'finitude-5f-abstract-nothing-and-incompatibility',
    title: 'Immediacy IV: incompatibility thesis and relapse into abstract nothing',
    text: `The fact is that this perishing of the perishing does not happen
on precisely the view that would make
the perishing the final end of the finite.
The official claim is that the finite is
incompatible with the infinite
and cannot be united with it;
that the finite is absolutely opposed to the infinite.
Being, absolute being, is ascribed to the infinite.
The finite remains held fast over against it as its negative;
incapable of union with the infinite,
it remains absolute on its own side;
from the affirmative, from the infinite,
it would receive affirmation and thus it would perish;
but a union with the infinite is precisely
what is declared impossible.
If the finite were not to persist over against the infinite
but were to perish, its perishing, as just said,
would then be the last of it;
not its affirmative, which would be only
a perishing of the perishing.
However, if it is not to perish into the affirmative
but its end is rather to be grasped as a nothing,
then we are back at that first, abstract nothing
that itself has long since passed away.`,
    summary:
      'If finite/infinite are held absolutely incompatible, finite is fixed as negative; perishing is made the last word. Or, taking the end as “nothing” merely relapses into the abstract nothing already sublated.'
  },
  {
    id: 'finitude-5g-contradiction-to-consciousness',
    title: 'Immediacy V: contradiction surfaces; collapse and resolution—perishing perishes',
    text: `With this nothing, however, which is supposed to be only nothing
but to which a reflective existence is nevertheless granted
in thought, in representation or in speech,
the same contradiction occurs as we have
just indicated in connection with the finite,
except that in the nothing it just occurs
but in the finite it is instead expressed.
In the one case, the contradiction appears as subjective;
in the other, the finite is said to stand
in perpetual opposition to the infinite,
in itself to be null, and to be as null in itself.
This is now to be brought to consciousness.
The development of the finite will show that,
expressly as this contradiction, it collapses internally,
but that, in this collapse, it actually resolves the contradiction;
it will show that the finite is not just perishable, and that it perishes,
but that the perishing, the nothing, is rather not the last of it;
that the perishing rather perishes.`,
    summary:
      'Granting “nothing” a standing breeds the same contradiction. Brought to consciousness, the finite collapses and resolves the contradiction: the perishing itself perishes.'
  },
  {
    id: 'finitude-5h-finite-immanence-and-moments',
    title: 'Restriction/Ought I: finite as immanent limit; moments to be seen',
    text: `(b) Restriction and the ought

This contradiction is indeed abstractly present
by the very fact that the something is finite,
or that the finite is.
But something or being is no longer posited abstractly
but reflected into itself,
and developed as being-in-itself
that has determination and constitution in it,
or, more determinedly still, in such a way
that it has a limit within it;
and this limit, as constituting
what is immanent to the something
and the quality of its being-in-itself,
is finitude.
It is to be seen what moments are contained
in this concept of the finite something.`,
    summary:
      'Finite = something with immanent limit (as quality of its in‑itself). We now exhibit the internal moments of this finite.'
  },
  {
    id: 'finitude-5i-immanent-otherness-restriction',
    title: 'Restriction/Ought II: immanent otherness; self-reference negates limit → restriction',
    text: `Determination and constitution arose as
sides for external reflection,
but determination already contained otherness
as belonging to the in-itself of something.
On the one side, the externality of otherness is
within the something's own inwardness;
on the other side, it remains as otherness distinguished from it;
it is still externality as such, but in the something.
But further, since otherness is determined as limit,
itself as negation of the negation,
the otherness immanent to the something is posited as the
connection of the two sides,
and the unity of the something with itself
(to which both determination and constitution belong)
is its reference turned back upon itself,
the reference to it of its implicitly existing determination
that in it negates its immanent limit.
The self-identical in-itself thus refers
itself to itself as to its own non-being,
but as negation of the negation,
as negating that which at the same time retains existence in it,
for it is the quality of its in-itselfness.
Something's own limit, thus posited by it as a
negative which is at the same time essential,
is not only limit as such, but restriction.`,
    summary:
      'Immanent otherness (as limit, negation‑of‑negation) connects determination and constitution. The something’s self‑reference negates its own immanent limit; posited as essential negative, the limit is restriction.'
  },
  {
    id: 'finitude-5j-ought-definition',
    title: 'Restriction/Ought III: negation cuts two ways; definition of ought',
    text: `But restriction is not alone in being posited as negative;
the negation cuts two ways, for that which it posits as negated is limit,
and limit is in general what is common to something and other,
and is also the determinateness of the in-itself of determination as such.
This in-itself, consequently, as negative reference to its limit
(which is also distinguished from it),
as negative reference to itself as restriction,
is the ought.`,
    summary:
      'Because limit is common to something and other, the negative reference is twofold: to the limit (as distinct) and to itself (as restriction). This negative self‑reference is the ought.'
  },
  {
    id: 'finitude-5k-transcendence-internal-sublation',
    title: 'Restriction/Ought IV: for restriction, limit must be transcended in-itself',
    text: `In order for the limit that is in every something to be a restriction,
the something must at the same time transcend it in itself,
must refer to it from within as to a non-existent.
The existence of something lies quietly indifferent,
as it were, alongside its limit.
But the something transcends its limit only
in so far as it is the sublatedness of the limit,
the negative in-itselfness over against it.
And inasmuch as the limit is as restriction
in the determination itself,
the something thereby transcends itself.`,
    summary:
      'Restriction requires inner transcendence of the limit: the something sublates its limit as non‑existent relative to its negative in‑itself, thereby transcending itself.'
  },
  {
    id: 'finitude-5l-ought-double-determination',
    title: 'Restriction/Ought V: the double determination of the ought',
    text: `The ought therefore contains the double determination:
once, as a determination which has
an in-itselfness over against negation;
and again, as a non-being which, as restriction,
is distinguished from the determination
but is at the same time itself
a determination existing in itself.`,
    summary:
      'Ought has a double: (1) in‑itselfness over against negation; (2) non‑being as restriction, itself a determination.'
  },
  {
    id: 'finitude-5m-ought-restriction-as-moments-finitude',
    title: 'Restriction/Ought VI: finite connects determination and limit—ought and restriction',
    text: `The finite has thus determined itself
as connecting determination and limit;
in this connection, the determination is the ought
and the limit is the restriction.
Thus the two are both moments of the finite,
and therefore both themselves finite,
the ought as well as the restriction.
But only restriction is posited as the finite;
the ought is restricted only in itself,
and therefore only for us.
It is restricted by virtue of its reference
to the limit already immanent within it,
though this restriction in it is shrouded in in-itselfness,
for according to its determinate being,
that is, according to its determinateness
in contrast to restriction,
it is posited as being-in-itself.`,
    summary:
      'Finite = linkage of determination and limit: determination→ought; limit→restriction. Both are moments (finite), but only restriction is posited as finite; the ought’s restriction is “for us,” veiled in its in‑itselfness.'
  },
  {
    id: 'finitude-5n-ought-is-and-is-not-essential-restriction',
    title: 'Restriction/Ought VII: what ought-to-be is and is not—its essential restriction',
    text: `What ought to be is, and at the same time is not.
If it were, it would not be what merely ought to be.
The ought has therefore a restriction essentially.
This restriction is not anything alien;
that which only ought to be is determination
now posited as it is in fact,
namely as at the same time
only a determinateness.`,
    summary:
      'Ought both is and is not; hence it is essentially restricted. The “ought” is determination that is also merely a determinateness.'
  },
  {
    id: 'finitude-5o-identity-restriction-ought-internality',
    title: 'Restriction/Ought VIII: restriction internal; identity of restriction and ought',
    text: `The being-in-itself of the something is
thus reduced in its determination
to the ought because the very thing
that constitutes the something's in-itselfness is,
in one and the same respect, as non-being;
or again, because in the in-itselfness,
in the negation of the negation,
the said being-in-itself is as
one negation (what negates) a unity with the other,
and this other, as qualitatively other, is the limit
by virtue of which that unity is as reference to it.
The restriction of the finite is not anything external,
but the finite's own determination is rather also its restriction;
and this restriction is both itself and the ought;
it is that which is common to both,
or rather that in which the two are identical.`,
    summary:
      'Because in‑itselfness is unity with its other (limit), restriction is internal. The finite’s determination is also its restriction; restriction and ought are identical as the common moment.'
  },
  {
    id: 'finitude-5p-ought-transcends-restriction-indivisible-pair',
    title: 'Restriction/Ought IX: as ought, transcendence; indivisibility with restriction',
    text: `But further, as “ought” the finite transcends its restriction;
the same determinateness which is its negation is also sublated,
and is thus its in-itself;
its limit is also not its limit.

As ought something is thus elevated above its restriction,
but conversely it has its restriction only as ought.
The two are indivisible.
Something has a restriction in so far as
it has negation in its determination,
and the determination is also
the being sublated of the restriction.`,
    summary:
      'As ought, the finite sublates its negating determinateness: the limit is “not its limit.” Yet it has restriction only as ought. Ought and restriction are indivisible; determination is also the sublation of restriction.'
  },
  {
    id: 'finitude-5q-transition-setup-contradiction',
    title: 'Transition I: ought↔restriction mutually contain; finite as self-contradiction',
    text: `(c) Transition of the finite into the infinite

The ought contains restriction explicitly, for itself,
and restriction contains the ought.
Their mutual connection is the finite itself,
which contains them both in its in-itself.
These moments of its determination are qualitatively opposed;
restriction is determined as the negative of the ought,
and the ought equally as the negative of restriction.
The finite is thus in itself the contradiction of itself;
it sublates itself, it goes away and ceases to be.`,
    summary:
      'Ought and restriction mutually contain each other in the finite. As qualitative opposites, they make the finite a contradiction that sublates itself (goes away).'
  },
  {
    id: 'finitude-5r-result-a-bad-infinite-progression',
    title: 'Transition II (a): negative as determination; finite → other finite ad infinitum',
    text: `But this, its result, the negative as such,
is (a) its very determination;
for it is the negative of the negative.
So, in going away and ceasing to be,
the finite has not ceased;
it has only become momentarily
an other finite which equally is,
however, a going-away as a going-over
into another finite, and so forth to infinity.`,
    summary:
      'Result (a): determination = negation of negation. The finite “ceases” only as passing into another finite, producing an endless progression (bad infinite).'
  },
  {
    id: 'finitude-5s-result-b-rejoining-identity',
    title: 'Transition III (b): in ceasing, finite attains in-itself; moments rejoin themselves',
    text: `But, (b) if we consider this result more closely,
in its going-away and ceasing-to-be,
in this negation of itself,
the finite has attained its being-in-itself;
in it, it has rejoined itself.
Each of its moments contains precisely this result;
the ought transcends the restriction,
that is, it transcends itself;
but its beyond, or its other, is only restriction itself.
Restriction, for its part, immediately points
beyond itself to its other,
and this is the ought;
but this ought is the same
diremption of in-itselfness and determinateness
as is restriction;
it is the same thing;
in going beyond itself,
restriction thus equally rejoins itself.`,
    summary:
      'Result (b): the finite’s ceasing is its in‑itself (rejoining). Ought transcends to restriction (its own other), restriction to ought—the same diremption; each passes beyond and rejoins itself.'
  },
  {
    id: 'finitude-5t-conclusion-infinite',
    title: 'Transition IV: negation of negation = affirmative being → the infinite',
    text: `This identity with itself, the negation of negation,
is affirmative being, is thus the other of the finite
which is supposed to have the first negation
for its determinateness;
this other is the infinite.`,
    summary:
      'Identity through negation‑of‑negation is affirmative being—the other of the finite (first negation). This other is the infinite.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'finitude-op-39-finitude-definition',
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
    id: 'finitude-op-40-finite-perishing-essential',
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
  },
  {
    id: 'finitude-op-41-obstinate-finitude',
    chunkId: 'finitude-5c-immediacy-mournful-obstinate',
    label: 'Finitude fixed as negation; obstinate category of understanding',
    clauses: [
      'tag(Finitude,"fixed-negation")',
      'assert(standsAgainst(Finitude,"affirmative"))',
      'tag(Understanding,"obstinate-on-finitude")'
    ],
    predicates: [{ name: 'FinitudeObstinacy', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-42-eternal-finitude-stance',
    chunkId: 'finitude-5d-refusal-of-infinite-eternal-finitude',
    label: 'Understanding’s stance: finitude “eternal,” refuses union with infinite',
    clauses: [
      'assert(refusesUnion(Finite,Infinite))',
      'assert(gluedTo(Finite,"nothing"))',
      'tag(Finitude,"eternal-in-understanding")'
    ],
    predicates: [{ name: 'EternalFinitudeStance', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-43-decision-on-perishing',
    chunkId: 'finitude-5e-stance-and-decision-on-perishing',
    label: 'Decision point: does transitoriness persist or itself perish?',
    clauses: [
      'assert(option("persist-transitoriness"))',
      'assert(option("perishing-of-perishing"))'
    ],
    predicates: [{ name: 'PerishingDecision', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-44-incompatibility-relapse-nothing',
    chunkId: 'finitude-5f-abstract-nothing-and-incompatibility',
    label: 'Holding finite/infinite as incompatible fixes finite; relapse into abstract nothing',
    clauses: [
      'assert(incompatible(Finite,Infinite))',
      'assert(heldAsNegative(Finite,true))',
      'assert(lastWordIs(Perishing,true))',
      'assert(relapseInto(AbstractNothing,true))'
    ],
    predicates: [{ name: 'IncompatibilityTrap', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-45-contradiction-collapse-resolution',
    chunkId: 'finitude-5g-contradiction-to-consciousness',
    label: 'Contradiction made explicit; finite collapses and resolves—perishing perishes',
    clauses: [
      'tag(Contradiction,"explicit")',
      'assert(collapsesInternally(Finite,true))',
      'assert(resolvesContradiction(Finite,true))',
      'assert(perishingOfPerishing(true))'
    ],
    predicates: [{ name: 'PerishingPerishes', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-46-finite-immanent-limit-moments',
    chunkId: 'finitude-5h-finite-immanence-and-moments',
    label: 'Finite = immanent limit (quality of in-itself); enumerate moments',
    clauses: [
      'assert(hasImmanentLimit(Something,true))',
      'tag(Finitude,"immanent-limit-as-quality")'
    ],
    predicates: [{ name: 'FiniteImmanence', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-47-restriction-from-self-negating-limit',
    chunkId: 'finitude-5i-immanent-otherness-restriction',
    label: 'Immanent otherness as limit (negation-of-negation); self-reference negates limit → restriction',
    clauses: [
      'assert(othernessImmanentIn(Something,true))',
      'tag(Limit,"negation-of-negation-immanent")',
      'assert(negatesImmanentLimit(Something,true))',
      'tag(Restriction,"essential-negative-limit")'
    ],
    predicates: [{ name: 'RestrictionEmergence', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-48-ought-definition',
    chunkId: 'finitude-5j-ought-definition',
    label: 'Ought = negative reference to limit and to self as restriction',
    clauses: [
      'tag(Limit,"common-to-something-and-other")',
      'tag(Ought,"negative-reference-to-limit-and-self")',
      'assert(defines(Ought,["neg-to-limit","neg-to-self-as-restriction"]))'
    ],
    predicates: [{ name: 'OughtDef', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-49-internal-transcendence-sublation',
    chunkId: 'finitude-5k-transcendence-internal-sublation',
    label: 'Restriction requires inner transcendence; sublation of limit; self-transcendence',
    clauses: [
      'assert(transcendsInItself(Something,Limit))',
      'assert(sublates(Something,Limit))',
      'assert(transcendsItselfBy(Something,"restriction-in-determination"))'
    ],
    predicates: [{ name: 'RestrictionRequiresTranscendence', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-50-ought-double-determination',
    chunkId: 'finitude-5l-ought-double-determination',
    label: 'Ought has double determination (in-itself vs negation; restriction as determination)',
    clauses: [
      'assert(doubleDetermination(Ought,["in-itselfness-over-against-negation","restriction-as-determination"]))'
    ],
    predicates: [{ name: 'OughtDouble', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-51-ought-and-restriction-as-finite-moments',
    chunkId: 'finitude-5m-ought-restriction-as-moments-finitude',
    label: 'Finite connects determination/limit → ought/restriction; only restriction posited as finite',
    clauses: [
      'assert(connects(Finite,["Determination","Limit"]))',
      'assert(equals(Determination,Ought))',
      'assert(equals(Limit,Restriction))',
      'tag(Ought,"finite-moment")',
      'tag(Restriction,"finite-moment")',
      'assert(positedAsFinite(Restriction,true))',
      'assert(restrictedOnlyForUs(Ought,true))',
      'tag(Ought,"veiled-in-in-itself")'
    ],
    predicates: [{ name: 'FiniteMoments', args: [] }],
    relations: [
      { predicate: 'equals', from: 'Determination', to: 'Ought' },
      { predicate: 'equals', from: 'Limit', to: 'Restriction' }
    ]
  },
  {
    id: 'finitude-op-52-ought-essential-restriction',
    chunkId: 'finitude-5n-ought-is-and-is-not-essential-restriction',
    label: 'Ought is-and-is-not; hence essentially restricted; merely determinateness',
    clauses: [
      'tag(Ought,"is-and-is-not")',
      'assert(essentialRestriction(Ought,true))',
      'assert(onlyDeterminateness(Ought,true))'
    ],
    predicates: [{ name: 'OughtRestriction', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-53-restriction-internal-identity-with-ought',
    chunkId: 'finitude-5o-identity-restriction-ought-internality',
    label: 'Restriction internal; determination is restriction; identity of restriction and ought',
    clauses: [
      'assert(internal(Restriction,Finite,true))',
      'assert(equals(Determination,Restriction))',
      'assert(identityOf(["Restriction","Ought"]))'
    ],
    predicates: [{ name: 'RestrictionIdentity', args: [] }],
    relations: [
      { predicate: 'equals', from: 'Determination', to: 'Restriction' }
    ]
  },
  {
    id: 'finitude-op-54-ought-transcends-restriction',
    chunkId: 'finitude-5p-ought-transcends-restriction-indivisible-pair',
    label: 'As ought, finite sublates its limit; yet has restriction only as ought; indivisible pair',
    clauses: [
      'assert(sublates(Ought,Restriction))',
      'assert(limitNotLimit(Something,true))',
      'assert(indivisible(["Ought","Restriction"]))',
      'assert(hasRestrictionIffNegationInDetermination(Something,true))',
      'assert(sublationIs(Determination,Restriction))'
    ],
    predicates: [{ name: 'OughtRestrictionPair', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-55-transition-setup-contradiction',
    chunkId: 'finitude-5q-transition-setup-contradiction',
    label: 'Ought↔Restriction mutually contain; finite is self-contradictory and self-sublating',
    clauses: [
      'assert(contains(Ought,Restriction))',
      'assert(contains(Restriction,Ought))',
      'assert(connectionIs(Finite,["Ought","Restriction"]))',
      'assert(qualitativelyOpposed(["Ought","Restriction"]))',
      'tag(Finite,"self-contradiction")',
      'assert(sublatesItself(Finite,true))'
    ],
    predicates: [{ name: 'FiniteContradiction', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-56-bad-infinite-progression',
    chunkId: 'finitude-5r-result-a-bad-infinite-progression',
    label: 'Result (a): determination = negation-of-negation; finite passes over to finite ad infinitum',
    clauses: [
      'assert(determinationIs(Finite,"negation-of-negation"))',
      'assert(passesOver(Finite,Finite))',
      'tag(Progression,"to-infinity")'
    ],
    predicates: [{ name: 'BadInfinite', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-57-rejoining-identity',
    chunkId: 'finitude-5s-result-b-rejoining-identity',
    label: 'Result (b): in ceasing, finite attains in-itself; ought↔restriction pass beyond and rejoin',
    clauses: [
      'assert(attainsInItself(Finite,true))',
      'assert(transcends(Ought,Restriction))',
      'assert(beyondIs(Ought,Restriction))',
      'assert(pointsBeyond(Restriction,Ought))',
      'assert(sameDiremption(["Ought","Restriction"]))',
      'assert(rejoinsItself(Ought,true))',
      'assert(rejoinsItself(Restriction,true))',
      'tag(Identity,"negation-of-negation")'
    ],
    predicates: [{ name: 'RejoiningByNegNeg', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-58-define-infinite',
    chunkId: 'finitude-5t-conclusion-infinite',
    label: 'Negation-of-negation → affirmative being; other of the finite = Infinite',
    clauses: [
      'tag(Infinite,"affirmative-being")',
      'assert(isNegationOfNegation(Infinite,true))',
      'assert(otherOf(Infinite,Finite))',
      'assert(transition(Finite,Infinite))'
    ],
    predicates: [{ name: 'TransitionToInfinite', args: [] }],
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
