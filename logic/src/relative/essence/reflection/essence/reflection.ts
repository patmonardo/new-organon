import type { Chunk, LogicalOperation } from './index'

/*
  Essence — C. REFLECTION

  This module consolidates the complete Reflection section:
  - Part 1: Positing reflection
  - Part 2: External reflection
  - Part 3: Determining reflection

  Two-fold representation:
  - text: verbatim source segmented into readable chunks (preserve full passage)
  - summary: short IR summary to support HLO extraction (non-destructive)

  PHILOSOPHICAL NOTE:
  "CIT as Truth of SAT" = Citta (Mind) as Truth of Being (Sat) = Purusha (Consciousness)
  Essence is Citta as the Truth of Being—the witnessing principle (Purusha) that reflects Being into itself.
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  // Part 1: Positing Reflection
  {
    id: 'ess-ref-1-positing-reflection-intro',
    title: 'Positing reflection — from shine to absolute reflection',
    text: `1. Positing reflection

Shine is a nothingness or a lack of essence.
But a nothingness or that which is void of essence
does not have its being in an other in which it shines,
but its being is its own equality with itself;
this conversion of the negative with itself has been determined
as the absolute reflection of essence.`,
    summary: 'From shine (lack of essence) to equality-with-self: conversion of the negative with itself = absolute reflection of essence.'
  },
  {
    id: 'ess-ref-2-self-referring-negativity',
    title: 'Self-referring negativity: negation of itself; unity of being and non-being',
    text: `This self-referring negativity is
therefore the negating of itself.
It is thus just as much
sublated negativity as it is negativity.
Or again, it is itself the negative
and the simple equality with itself or immediacy.
It consists, therefore, in being itself
and not being itself,
and the two in one unity.`,
    summary: 'Self-referring negativity negates itself; it is both negativity and sublated negativity: unity of being-itself and not-being-itself.'
  },
  {
    id: 'ess-ref-3-nothing-to-nothing',
    title: 'Reflection as movement of nothing to nothing; coincidence of negation with itself',
    text: `Reflection is at first the movement of
the nothing to the nothing,
and thus negation coinciding with itself.
This self-coinciding is in general
simple equality with itself, immediacy.
But this falling together is not
the transition of negation into equality
as into a being other than it;
reflection is transition rather
as the sublating of transition,
for it is the immediate falling together
of the negative with itself.
And so this coinciding is, first,
self-equality or immediacy;
but, second, this immediacy is
the self-equality of the negative,
and hence self-negating equality,
immediacy which is in itself the negative,
the negative of itself:
its being is to be what it is not.`,
    summary: 'Reflection: nothing→nothing; negation coincides with itself. Immediacy = self-equality of the negative: being that is to be what it is not.'
  },
  {
    id: 'ess-ref-4-positedness',
    title: 'Immediacy as positedness (immediacy as determinateness/self-reflecting)',
    text: `The self-reference of the negative is
therefore its turning back into itself;
it is immediacy as the sublating of the negative,
but immediacy simply and solely as this reference
or as turning back from a one,
and hence as self-sublating immediacy.
This is positedness,
immediacy purely as determinateness
or as self-reflecting.`,
    summary: 'Turning-back of the negative gives immediacy as positedness: immediacy purely as determinateness/self-reflecting.'
  },
  {
    id: 'ess-ref-5-origin-in-reflection',
    title: 'Immediacy arises from reflection; reflection as turning (start/return)',
    text: `This immediacy, which is only as
the turning back of the negative into itself,
is the immediacy which constitutes the determinateness of shine,
and from which the previous reflective movement seemed to begin.
But, far from being able to begin with this immediacy,
the latter first is rather as the turning back
or as the reflection itself.
Reflection is therefore the movement which,
since it is the turning back,
only in this turning is that
which starts out or returns.`,
    summary: 'We cannot begin with immediacy; it first is as turning-back (reflection). Reflection is the movement where only in turning is the start/return.'
  },
  {
    id: 'ess-ref-6-positing-to-presupposing',
    title: 'From positing (turning-back immediacy) to presupposing',
    text: `It is a positing, inasmuch as it is
immediacy as a turning back;
that is to say, there is not an other beforehand,
one either from which or to which it would turn back;
it is, therefore, only as a turning back
or as the negative of itself.
But further, this immediacy is sublated negation
and sublated return into itself.
Reflection, as the sublating of the negative, is
the sublating of its other, of the immediacy.
Because it is thus immediacy as a turning back,
the coinciding of the negative with itself,
it is equally the negation of the negative as negative.
And so it is presupposing.
Or immediacy is as a turning back
only the negative of itself,
just this, not to be immediacy;
but reflection is the sublating
of the negative of itself,
coincidence with itself;
it therefore sublates its positing,
and inasmuch as it is in its positing
the sublating of positing, it is presupposing.`,
    summary: 'Immediacy-as-turning-back is "positing," but reflection sublates this positing; thus reflection is presupposing (negation of the negative as negative).'
  },
  {
    id: 'ess-ref-7-structure-of-presupposing',
    title: 'Structure of presupposing: turning-back of the negative of itself',
    text: `In presupposing, reflection determines the turning back
into itself as the negative of itself,
as that of which essence is the sublating.
It is its relating to itself,
but to itself as to the negative of itself;
only so is it negativity which abides with itself,
self-referring negativity.
Immediacy comes on the scene simply and solely
as a turning back and is that negative
which is the semblance of a beginning,
the beginning which the return negates.
The turning back of essence is therefore its self-repulsion.
Or inner directed reflection is essentially
the presupposing of that from which
the reflection is the turning back.`,
    summary: 'Presupposing = relating to self as negative-of-self (abiding negativity). Immediacy is mere semblance of a beginning; turning-back is self-repulsion.'
  },
  {
    id: 'ess-ref-8-essence-presupposes-itself',
    title: 'Essence presupposes itself; sublating presupposition',
    text: `It is only by virtue of the sublating of
its equality with itself
that essence is equality with itself.
Essence presupposes itself,
and the sublating of this presupposing is essence itself;
contrariwise, this sublating of its presupposition is
the presupposition itself.
Reflection thus finds an immediate before it
which it transcends and from which it is the turning back.`,
    summary: 'Essence equals itself only by sublating its equality; it presupposes itself, and sublating this presupposition is itself presupposition. Reflection finds an immediate to transcend.'
  },
  {
    id: 'ess-ref-9-antecedent-by-being-left-behind',
    title: 'Antecedent arises by being left behind; arriving-at-itself',
    text: `But this turning back is only the presupposing of
what was antecedently found.
This antecedent comes to be only by being left behind;
its immediacy is sublated immediacy.
The sublated immediacy is, contrariwise, the turning
back into itself,
essence that arrives at itself,
simple being equal to itself.
This arriving at itself is thus
the sublating of itself
and self-repelling, presupposing reflection,
and its repelling of itself from itself is
the arriving at itself.`,
    summary: 'The "antecedent" exists only as left-behind; its immediacy is sublated. Repelling-itself-from-itself is arriving-at-itself (presupposing reflection).'
  },
  {
    id: 'ess-ref-10-absolute-internal-counter-repelling',
    title: 'Reflection as absolute internal counter-repelling; presupposition only in turning-back',
    text: `It follows from these considerations that
the movement of reflection is to be taken as
an absolute internal counter-repelling.
For the presupposition of
the turning back into itself
[that from which essence arises,
essence being only as this coming back]
is only in the turning back itself.
Transcending the immediate from which reflection begins
occurs rather only through this transcending;
and the transcending of the immediate is
the arriving at the immediate.
The movement, as forward movement, turns immediately
around into itself and so is only self-movement:
a movement which comes from itself in so far as
positing reflection is presupposing reflection, yet,
as presupposing reflection, is simply positing reflection.`,
    summary: 'Reflection = absolute internal counter-repelling; its presupposition exists only in turning-back. Transcending immediate = arriving at immediate. Positing ↔ presupposing reflection (self-movement).'
  },
  {
    id: 'ess-ref-11-reflection-self-and-nonbeing',
    title: 'Reflection is itself and its non-being; is itself by being negative of itself',
    text: `Thus is reflection itself and its non-being,
and only is itself by being the negative of itself,
for only in this way is the sublating of the negative
at the same time a coinciding with itself.`,
    summary: 'Reflection is itself-and-its-non-being: it is itself by being the negative of itself; sublating the negative coincides with itself.'
  },
  {
    id: 'ess-ref-12-external-reflection',
    title: 'Presupposed immediacy as positedness; determination as negative → external reflection',
    text: `The immediacy which reflection,
as a process of sublating,
presupposes for itself is
simply and solely a positedness,
something in itself sublated
which is not diverse from
reflection's turning back into itself
but is itself only this turning back.
But it is at the same time determined as a negative,
as immediately in opposition to something,
and hence to an other.
And so is reflection determined.
According to this determinateness,
because reflection has a presupposition
and takes its start from the immediate as its other,
it is external reflection.`,
    summary: 'The presupposed immediacy is positedness (already sublated, identical with turning-back), yet determined as negative against an other; thus reflection that starts from the immediate-as-other is external reflection.'
  },
  // Part 2: External Reflection
  {
    id: 'ess-ref1-1-definition-and-doubling',
    title: 'External reflection — definition and doubling',
    text: `2. External reflection

Reflection, as absolute reflection,
is essence shining within,
essence that posits only shine,
only positedness, for its presupposition;
and as presupposing reflection,
it is immediately only positing reflection.
But external or real reflection
presupposes itself as sublated,
as the negative of itself.
In this determination, it is doubled.
At one time it is as what is presupposed,
or the reflection into itself which is the immediate.
At another time, it is as the reflection
negatively referring to itself;
it refers itself to itself as
to that its non-being.`,
    summary: 'External reflection presupposes itself as sublated (negative of itself) and is doubled: (a) immediate-as-presupposed, (b) negative self-reference (to its non-being).'
  },
  {
    id: 'ess-ref1-2-presupposes-a-being',
    title: 'Presupposing a being; immediacy/self-reference; moment-only',
    text: `External reflection thus presupposes a being,
at first not in the sense that
its immediacy is only positedness or moment,
but in the sense rather that
this immediacy refers to itself
and the determinateness is only as moment.`,
    summary: 'It presupposes a being where immediacy self-refers; determinateness only as moment.'
  },
  {
    id: 'ess-ref1-3-posits-and-finds-presupposition',
    title: 'Posits then sublates its positing; finds an immediate presupposition',
    text: `Reflection refers to its presupposition in such a way
that the latter is its negative,
but this negative is thereby sublated as negative.
Reflection, in positing, immediately sublates its positing,
and so it has an immediate presupposition.
It therefore finds this presupposition before it
as something from which it starts,
and from which it only makes its way back into itself,
negating it as its negative.
But that this presupposition is a negative
or a positedness is not its concern;
this determinateness belongs only to positing reflection,
whereas in the presupposing positedness
it is only as sublated.`,
    summary: 'Reflection posits and instantly sublates (thus "finds" an immediate presupposition) and starts from it back to itself; whether it is negative/positedness is not its concern here (only as sublated).'
  },
  {
    id: 'ess-ref1-4-external-determinations-and-infinite',
    title: 'External determinations; finite/infinite pattern from being',
    text: `What external reflection determines and posits in the immediate
are determinations which to that extent are external to it.
In the sphere of being, external reflection was the infinite;
the finite stands as the first,
as the real from which the beginning is made
as from a foundation that abides,
whereas the infinite is the reflection into itself
standing over against it.`,
    summary: 'External reflection\'s posited determinations are external to it. In being: begin from finite as abiding foundation; infinite = reflection-into-itself over against it.'
  },
  {
    id: 'ess-ref1-5-syllogism-form',
    title: 'Syllogism of external reflection',
    text: `This external reflection is the syllogism
in which the two extremes are
the immediate and the reflection into itself;
the middle term is the reference connecting the two,
the determinate immediate, so that one part of this connecting reference,
the immediate, falls to one extreme alone, and the other,
the determinateness or the negation, only to the other extreme.`,
    summary: 'Extremes: immediate vs reflection-into-itself; middle: determinate immediate linking them (immediacy to one extreme, determinateness/negation to the other).'
  },
  {
    id: 'ess-ref1-6-positing-and-negating-its-negating',
    title: 'Positing the immediate → negative; negating its negating; beginning-only-in-beginning',
    text: `But if one takes a closer look at what the external reflection does,
it turns out that it is, secondly, the positing of the immediate,
an immediate which thus becomes the negative or the determined;
but it is immediately also the sublating of this positing,
for it presupposes the immediate;
in negating, it is the negating of its negating.
But thereby it immediately is equally a positing,
the sublating of the immediate which is its negative;
and this negative, from which it seemed to begin
as from something alien,
only is in this its beginning.`,
    summary: 'It posits the immediate (as negative/determined) and sublates that positing; negating-of-its-negating. The "alien" negative exists only in and as this beginning.'
  },
  {
    id: 'ess-ref1-7-externality-sublated-determining',
    title: 'Externality sublated; coincidence with immediate; determining reflection',
    text: `In this way, the immediate is not only implicitly in itself
(that is, for us or in external reflection)
the same as what reflection is,
but is posited as being the same.
For the immediate is determined by reflection as
the negative of the latter or as the other of it,
but it is reflection itself which negates this determining.
The externality of reflection vis-à-vis
the immediate is consequently sublated;
its self-negating positing is its coinciding
with its negative, with the immediate,
and this coinciding is the immediacy of essence itself.
It thus transpires that external reflection is not external
but is just as much the immanent reflection of immediacy itself;
or that the result of positing reflection is
essence existing in and for itself.
External reflection is thus determining reflection.`,
    summary: 'Immediate is posited as same as reflection; reflection negates its own determining. Externality is sublated; coincidence with immediate = immediacy of essence. Hence external reflection = immanent/determining reflection; result: essence in-and-for-itself.'
  },
  // Part 3: Determining Reflection
  {
    id: 'ess-ref2-0-determining-unity',
    title: 'Determining reflection — unity of positing and external reflection',
    text: `3. Determining reflection

Determining reflection is in general
the unity of positing and external reflection.
This is now to be examined more closely.`,
    summary: 'Determining reflection = unity of positing reflection and external reflection.'
  },
  {
    id: 'ess-ref2-1-origins-and-incompletion',
    title: 'Origins of the two reflections; incompletion and the merely posited',
    text: `1. External reflection begins from immediate being,
positing reflection from nothing.
In its determining, external reflection posits another in the
place of the sublated being, but this other is essence;
the positing does not posit its determination in the place of an other;
it has no presupposition.
But, precisely for this reason,
it is not complete as determining reflection;
the determination which it posits is consequently only a posited;
this is an immediate, not however as equal to itself
but as self-negating;
its connection with the turning back into itself is absolute;
it is only in the reflection-into-itself
but is not this reflection itself.

The posited is therefore an other,
but in such a manner that the self-equality
of reflection is retained;
for the posited is only as sublated,
as reference to the turning back into itself.`,
    summary: 'External begins from being; positing from nothing. External, in determining, posits "other" that is essence; positing has no presupposition, hence its determination is only a posited—immediate yet self-negating, only in reflection-into-itself.'
  },
  {
    id: 'ess-ref2-2-positedness-corresponds-existence',
    title: 'Positedness corresponds to existence; existence is positedness',
    text: `In the sphere of being, existence was the being
that had negation in it, and being was the immediate ground
and element of this negation which was,
therefore, itself immediate negation.
In the sphere of essence,
positedness is what corresponds to existence.
Positedness is equally an existence,
but its ground is being as essence
or as pure negativity;
it is a determinateness or a negation,
not as existent but immediately as sublated.
Existence is only positedness;
this is the principle of the essence of existence.`,
    summary: 'In essence, positedness corresponds to existence. Existence = positedness; its ground is essence (pure negativity); determinateness is immediately as sublated.'
  },
  {
    id: 'ess-ref2-3-positedness-mediator-and-superiority',
    title: 'Positedness mediates essence and existence; its "superiority" and scope',
    text: `Positedness stands on the one side over against existence,
and over against essence on the other:
it is to be regarded as the means which conjoins
existence with essence and essence with existence.
If it is said, a determination is only a positedness,
the claim can thus have a twofold meaning,
according to whether the determination is such
in opposition to existence or in opposition to essence.
In either meaning, existence is taken for
something superior to positedness,
which is attributed to external reflection, to the subjective.
In fact, however, positedness is the superior, because, as posited,
existence is what it is in itself something negative,
something that refers simply and solely to the turning back into itself.
For this reason positedness is only a positedness
with respect to essence:
it is the negation of this turning back
as achieved return into itself.`,
    summary: 'Positedness mediates between existence and essence. Though taken as "subjective," it is superior: it shows existence as negative referring to return-into-itself. Yet it is "only positedness" with respect to essence: negation of achieved return.'
  },
  {
    id: 'ess-ref2-4-positedness-becomes-determination',
    title: 'Positedness united with external reflection → determination of reflection',
    text: `2. Positedness is not yet a determination of reflection;
it is only determinateness as negation in general.
But the positing is now united with external reflection;
in this unity, the latter is absolute presupposing, that is,
the repelling of reflection from itself
or the positing of determinateness as its own.
As posited, therefore, positedness is negation;
but as presupposed, it is reflected into itself.
And in this way positedness is a determination of reflection.`,
    summary: 'Uniting positing with external reflection (absolute presupposing) makes positedness a determination: as posited → negation; as presupposed → reflected-into-itself.'
  },
  {
    id: 'ess-ref2-5-determination-vs-quality',
    title: 'Determination of reflection vs determinateness of being (quality)',
    text: `The determination of reflection is distinct
from the determinateness of being, of quality;
the latter is immediate reference to other in general;
positedness also is reference to other,
but to immanently reflected being.
Negation as quality is existent negation;
being constitutes its ground and element.
The determination of reflection, on the contrary,
has for this ground immanent reflectedness.
Positedness gets fixed in determination precisely
because reflection is self-equality in its negatedness;
the latter is therefore itself reflection into itself.
Determination persists here, not by virtue of being
but because of its self-equality.
Since the being which sustains quality is
unequal to the negation, quality is
consequently unequal within itself,
and hence a transient moment which disappears in the other.
The determination of reflection is
on the contrary positedness as negation,
negation which has negatedness for its ground,
is therefore not unequal to itself within itself,
and hence essential rather than transient determinateness.
What gives subsistence to it is the self-equality of reflection
which has the negative only as negative,
as something sublated or posited.`,
    summary: 'Quality: existent negation grounded in being → transient and unequal. Determination of reflection: positedness-as-negation grounded in immanent reflectedness → persists by self-equality, essential, not unequal within itself.'
  },
  {
    id: 'ess-ref2-6-essential-shine-negation-predominates',
    title: 'Free essentialities; essential shine; negation predominates',
    text: `Because of this reflection into themselves,
the determinations of reflection appear as
free essentialities, sublated in the void
without reciprocal attraction or repulsion.
In them the determinateness has become entranced
and infinitely fixed by virtue of the reference to itself.
It is the determinate which has subjugated its transitoriness
and its mere positedness to itself, that is to say,
has deflected its reflection-into-other into reflection-into- itself.
These determinations hereby constitute the determinate shine
as it is in essence, the essential shine.
Determining reflection is for this reason
reflection that has exited from itself;
the equality of essence with itself is
lost in the negation, and negation predominates.`,
    summary: 'Determinations appear as free essentialities (in the void); reflection-into-other is deflected into reflection-into-itself: essential shine. Determining reflection exits from itself; negation predominates.'
  },
  {
    id: 'ess-ref2-7-two-sides-of-determination',
    title: 'Two sides: positedness (negation-as-negation) and immanent reflection',
    text: `Thus there are two distinct sides to the determination of reflection.
First, reflection is positedness, negation as such;
second, it is immanent reflection.
According to the side of positedness,
it is negation as negation,
and this already is its unity with itself.
But it is this unity at first only implicitly or in itself,
an immediate which sublates itself within, is the other of itself.
To this extent, reflection is a determining that abides in itself.
In it essence does not exit from itself;
the distinctions are solely posited,
taken back into essence.
But, from the other side, they are not posited
but are rather reflected into themselves;
negation as negation is equality with itself,
not in its other, not reflected into its non-being.`,
    summary: 'Side 1: positedness (negation-as-negation), unity implicit; determining abides in itself, distinctions taken back into essence. Side 2: immanent reflection—determinations reflected into themselves; equality not in an other.'
  },
  {
    id: 'ess-ref2-8-dual-nature-of-determination',
    title: 'Dual nature: positedness (negation vs essence) and immanent self-reference',
    text: `3. Now keeping in mind that the determination of reflection is
both immanently reflected reference and positedness as well,
its nature immediately becomes more transparent.
For, as positedness, the determination is negation as such,
a non-being as against another, namely,
as against the absolute immanent reflection or as against essence.
But as self-reference, it is reflected within itself.
This, the reflection of the determination,
and that positedness are distinct;
its positedness is rather the sublatedness of the determination
whereas its immanent reflectedness is its subsisting.`,
    summary: 'Determination of reflection = positedness (negation vs essence) + immanent self-reference. Positedness = sublatedness; immanent reflectedness = subsistence.'
  },
  {
    id: 'ess-ref2-9-reference-to-its-otherness',
    title: 'Reference-to-otherness inside the determination (non-quiescent reference)',
    text: `In so far as now the positedness is
at the same time immanent reflection,
the determinateness of the reflection is
the reference in it to its otherness.
It is not a determinateness that exists quiescent,
one which would be referred to an other
in such a way that the referred term
and its reference would be different,
each something existing in itself,
each a something that excludes its other
and its reference to this other from itself.
Rather, the determination of reflection is
within it the determinate side
and the reference of this determinate side as determinate,
that is, the reference to its negation.`,
    summary: 'Reference to otherness is internal: the determination includes both determinate side and its reference (to its negation) within itself.'
  },
  {
    id: 'ess-ref2-10-otherness-taken-back-essentiality',
    title: 'Otherness taken back; deflection into self; unity with its other (essentiality)',
    text: `Quality, through its reference, passes over into another;
its alteration begins in its reference.
The determination of reflection, on the contrary,
has taken its otherness back into itself.
It is positedness, negation which has however deflected
the reference to another into itself,
and negation which, equal to itself,
is the unity of itself and its other,
and only through this is an essentiality.`,
    summary: 'Unlike quality, determination of reflection internalizes otherness: deflects reference-to-another into itself; equals itself as unity of itself and its other → essentiality.'
  },
  {
    id: 'ess-ref2-11-infinite-reference-to-self',
    title: 'Positedness sublated in self-reflection → infinite reference to itself',
    text: `It is, therefore, positedness, negation,
but as reflection into itself it is at the same time
the sublatedness of this positedness,
infinite reference to itself.`,
    summary: 'As self-reflection, positedness is sublated; determination = infinite self-reference.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // Part 1: Positing Reflection Operations
  {
    id: 'ess-ref-op-1-define-positing-reflection',
    chunkId: 'ess-ref-1-positing-reflection-intro',
    label: 'Define positing reflection: conversion of the negative with itself',
    digest: 'From shine (lack) to equality-with-self; absolute reflection = negative converting with itself.',
    clauses: [
      'assert(Shine == lackOf(Essence))',
      'assert(AbsoluteReflection == convert(Negative, into: selfEquality))',
      'tag(Essence,"absolute-reflection")'
    ],
    predicates: [{ name: 'DefinesPositingReflection', args: [] }],
    relations: [
      { predicate: 'derivesFrom', from: 'Reflection', to: 'Shine' },
      { predicate: 'yields', from: 'Negative', to: 'SelfEquality' }
    ]
  },
  {
    id: 'ess-ref-op-2-self-negation-unity',
    chunkId: 'ess-ref-2-self-referring-negativity',
    label: 'Encode self-referring negativity: self-negation and unity of being/not-being',
    digest: 'Self-referring negativity negates itself; is both negativity and sublated negativity — being and not-being in one.',
    clauses: [
      'assert(selfRefers(Negativity) && negatesItself(Negativity))',
      'tag(Negativity,"sublated-and-as-such")',
      'assert(unity(BeingItself, NotBeingItself))'
    ],
    predicates: [{ name: 'EncodesSelfRefNegativity', args: [] }],
    relations: [{ predicate: 'unifies', from: 'Negativity', to: 'Being∧NotBeing' }]
  },
  {
    id: 'ess-ref-op-3-coinciding-negation',
    chunkId: 'ess-ref-3-nothing-to-nothing',
    label: 'Model reflection as nothing→nothing; immediacy as self-negating equality',
    digest: 'Reflection is negation coinciding with itself; immediacy is "being to be what it is not".',
    clauses: [
      'assert(path(Reflection) == from(Nothing) -> to(Nothing))',
      'assert(coincidesWithItself(Negation) == true)',
      'tag(Immediacy,"self-equality-of-negative")',
      'annotate(Immediacy,{paradox:"its being is to be what it is not"})'
    ],
    predicates: [{ name: 'ModelsNothingToNothing', args: [] }],
    relations: [
      { predicate: 'coincides', from: 'Negation', to: 'Negation' },
      { predicate: 'constitutes', from: 'Reflection', to: 'Immediacy' }
    ]
  },
  {
    id: 'ess-ref-op-4-define-positedness',
    chunkId: 'ess-ref-4-positedness',
    label: 'Define positedness: immediacy as determinateness via turning-back',
    digest: 'Positedness = immediacy purely as determinateness (self-reflecting) produced by turning-back.',
    clauses: [
      'assert(Positedness == immediacy(as: Determinateness))',
      'assert(turnedBackFrom(Negative, into: Immediacy))',
      'tag(Immediacy,"self-reflecting-determinate")'
    ],
    predicates: [{ name: 'DefinesPositedness', args: [] }],
    relations: [
      { predicate: 'resultsFrom', from: 'Immediacy', to: 'TurningBack' },
      { predicate: 'determinesAs', from: 'Immediacy', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-ref-op-5-origin-in-reflection',
    chunkId: 'ess-ref-5-origin-in-reflection',
    label: 'Assert that immediacy cannot be the starting point; origin is reflection',
    digest: 'Immediacy first is as turning-back; reflection is the movement in which the start/return are only in the turning.',
    clauses: [
      'assert(cannotBeginWith(Immediacy) == true)',
      'assert(origin(Immediacy) == Reflection)',
      'annotate(Reflection,{structure:"turning-back", startEqualsReturn:true})'
    ],
    predicates: [{ name: 'AssertsOriginInReflection', args: [] }],
    relations: [
      { predicate: 'originates', from: 'Immediacy', to: 'Reflection' },
      { predicate: 'equates', from: 'Reflection', to: 'Start~Return' }
    ]
  },
  {
    id: 'ess-ref-op-6-positing-to-presupposing',
    chunkId: 'ess-ref-6-positing-to-presupposing',
    label: 'From positing (turning-back immediacy) to presupposing',
    digest: 'Reflection sublates its own positing (negation of negative as negative) and thereby is presupposing.',
    clauses: [
      'assert(Immediacy == turningBack(Negative, into: self))',
      'assert(Reflection == sublate(Negative))',
      'assert(Reflection == sublate(Immediacy))',
      'assert(negates(Negative, as:"negative") && hence(Reflection,"presupposing"))',
      'assert(sublates(Reflection,"its-positing"))'
    ],
    predicates: [{ name: 'DerivesPresupposingFromPositing', args: [] }],
    relations: [
      { predicate: 'sublates', from: 'Reflection', to: 'Positing' },
      { predicate: 'presupposes', from: 'Reflection', to: 'Immediacy' }
    ]
  },
  {
    id: 'ess-ref-op-7-define-presupposing-structure',
    chunkId: 'ess-ref-7-structure-of-presupposing',
    label: 'Define presupposing as self-relation to the negative-of-self',
    digest: 'Presupposing: relate-to-self as negative-of-self; immediacy is semblance of a beginning; turning-back = self-repulsion.',
    clauses: [
      'assert(presupposing(Reflection) == relatesTo(Self, as: negativeOf(Self)))',
      'tag(Negativity,"abides-with-itself")',
      'annotate(Immediacy,{role:"semblance-of-beginning"})',
      'assert(selfRepulsion(Essence) == turningBack(Essence))'
    ],
    predicates: [{ name: 'DefinesPresupposing', args: [] }],
    relations: [
      { predicate: 'relatesAs', from: 'Reflection', to: 'Self~NegativeOfSelf' },
      { predicate: 'negates', from: 'Return', to: 'Beginning(Semblance)' }
    ]
  },
  {
    id: 'ess-ref-op-8-essence-presupposes-itself',
    chunkId: 'ess-ref-8-essence-presupposes-itself',
    label: 'Essence presupposes itself; sublating its presupposition',
    digest: 'Essence equals itself by sublating its equality; it presupposes itself, and sublating this presupposition is itself presupposition.',
    clauses: [
      'assert(equalsItself(Essence) == sublate(equalsItself(Essence)))',
      'tag(Essence,"self-presupposing")',
      'assert(sublate(presupposition(Essence)) == presupposition(Essence))',
      'annotate(Reflection,{finds:"immediate", action:"transcend-and-turn-back"})'
    ],
    predicates: [{ name: 'EncodesSelfPresupposition', args: [] }],
    relations: [
      { predicate: 'presupposes', from: 'Essence', to: 'Essence' },
      { predicate: 'transcends', from: 'Reflection', to: 'Immediate' }
    ]
  },
  {
    id: 'ess-ref-op-9-antecedent-left-behind-arrival',
    chunkId: 'ess-ref-9-antecedent-by-being-left-behind',
    label: 'Antecedent by being-left-behind; repulsion equals arrival-at-itself',
    digest: 'Antecedent arises by being left behind; sublated immediacy = turning-back; repelling-from-itself equals arriving-at-itself.',
    clauses: [
      'assert(Antecedent == produceBy(leftBehind(Immediate)))',
      'assert(sublated(Immediate) == turningBackIntoItself(Essence))',
      'tag(Essence,"arrives-at-itself")',
      'assert(repelsFromItself(Essence) == arrivesAtItself(Essence))'
    ],
    predicates: [{ name: 'RelatesAntecedentAndArrival', args: [] }],
    relations: [
      { predicate: 'arisesAs', from: 'Antecedent', to: 'LeftBehind' },
      { predicate: 'arrivesAt', from: 'Essence', to: 'Essence' }
    ]
  },
  {
    id: 'ess-ref-op-10-absolute-counter-repelling',
    chunkId: 'ess-ref-10-absolute-internal-counter-repelling',
    label: 'Encode absolute internal counter-repelling; presupposition only in turning-back',
    digest: 'Reflection self-movement: presupposition exists only in turning-back; transcending immediate = arriving at immediate; positing ↔ presupposing.',
    clauses: [
      'assert(movement(Reflection) == "absolute-internal-counter-repelling")',
      'assert(presupposition(Reflection) == existsOnlyIn(turningBack(Reflection)))',
      'assert(transcend(Immediate) == arriveAt(Immediate))',
      'assert(forward(Reflection) == turnsIntoSelf(Reflection))',
      'annotate(Reflection,{duality:"positing↔presupposing", equivalence:true})'
    ],
    predicates: [{ name: 'EncodesCounterRepelling', args: [] }],
    relations: [
      { predicate: 'presupposesIn', from: 'Reflection', to: 'TurningBack' },
      { predicate: 'equates', from: 'TranscendImmediate', to: 'ArriveImmediate' }
    ]
  },
  {
    id: 'ess-ref-op-11-self-and-nonbeing',
    chunkId: 'ess-ref-11-reflection-self-and-nonbeing',
    label: 'Reflection is itself by being the negative of itself',
    digest: 'Only by being negative-of-itself does reflection coincide with itself (sublation as coincidence).',
    clauses: [
      'tag(Reflection,"itself-and-its-non-being")',
      'assert(isItselfByBeingNegativeOfItself(Reflection) == true)',
      'assert(coincidesWithItself(sublate(Negative)) == true)'
    ],
    predicates: [{ name: 'AssertsSelfViaNegativity', args: [] }],
    relations: [
      { predicate: 'coincides', from: 'Reflection', to: 'Reflection' },
      { predicate: 'sublates', from: 'Reflection', to: 'Negative' }
    ]
  },
  {
    id: 'ess-ref-op-12-define-external-reflection',
    chunkId: 'ess-ref-12-external-reflection',
    label: 'Define external reflection via positedness and opposition to other',
    digest: 'Presupposed immediacy = positedness (already sublated, identical with turning-back) but determined as negative vs other → external reflection.',
    clauses: [
      'assert(presupposedImmediacy(Reflection) == Positedness && sublated(Positedness) == true)',
      'assert(identical(Positedness, turningBack(Reflection)) == true)',
      'tag(Immediacy,"opposed-to-other")',
      'if startsFrom(Reflection, ImmediateAsOther) then tag(Reflection,"external")'
    ],
    predicates: [{ name: 'DefinesExternalReflection', args: [] }],
    relations: [
      { predicate: 'identicalTo', from: 'PresupposedImmediacy', to: 'TurningBack' },
      { predicate: 'startsFrom', from: 'Reflection', to: 'ImmediateAsOther' }
    ]
  },
  // Part 2: External Reflection Operations
  {
    id: 'ess-ref1-op-1-define-external-reflection',
    chunkId: 'ess-ref1-1-definition-and-doubling',
    label: 'Define external reflection: presupposed-as-sublated and doubled',
    digest: 'External reflection presupposes itself as sublated and splits into immediate-as-presupposed and negative self-reference.',
    clauses: [
      'assert(presupposed(Reflection) == sublated(Reflection))',
      'tag(Reflection,"doubled")',
      'annotate(Reflection,{modes:["immediate-as-presupposed","negative-self-reference"]})',
      'relate(Reflection,"refers-to-non-being-of-self")'
    ],
    predicates: [{ name: 'DefinesExternalReflection', args: [] }],
    relations: [
      { predicate: 'presupposes', from: 'Reflection', to: 'Reflection' },
      { predicate: 'splitsInto', from: 'Reflection', to: 'Immediate|NegativeSelfRef' }
    ]
  },
  {
    id: 'ess-ref1-op-2-presupposes-being-moment',
    chunkId: 'ess-ref1-2-presupposes-a-being',
    label: 'Presuppose a being; immediacy self-refers; determinateness as moment',
    digest: 'External reflection presupposes being with self-referring immediacy; determinateness only as moment.',
    clauses: [
      'assert(presupposes(Reflection, Being) == true)',
      'assert(selfRefers(Immediacy) == true)',
      'tag(Determinateness,"moment-only")'
    ],
    predicates: [{ name: 'PresupposesBeingWithMoment', args: [] }],
    relations: [
      { predicate: 'presupposes', from: 'Reflection', to: 'Being' },
      { predicate: 'hasMoment', from: 'Being', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-ref1-op-3-posits-then-finds-presupposition',
    chunkId: 'ess-ref1-3-posits-and-finds-presupposition',
    label: 'Posit → sublate → find presupposition and return',
    digest: 'Reflection posits and immediately sublates; hence finds an immediate presupposition to start from and returns, without concern for its "positedness."',
    clauses: [
      'assert(posit(Reflection, Presupposition))',
      'assert(sublate(Presupposition) == true)',
      'tag(Presupposition,"immediate-before-it")',
      'assert(startsFrom(Reflection, Presupposition) && returnsToSelf(Reflection))',
      'annotate(Presupposition,{status:"sublated", note:"not Reflection\'s concern here"})'
    ],
    predicates: [{ name: 'PositSublateFind', args: [] }],
    relations: [
      { predicate: 'startsFrom', from: 'Reflection', to: 'Presupposition' },
      { predicate: 'returnsTo', from: 'Reflection', to: 'Reflection' }
    ]
  },
  {
    id: 'ess-ref1-op-4-external-determinations-infinite',
    chunkId: 'ess-ref1-4-external-determinations-and-infinite',
    label: 'Mark determinations as external; finite/infinite schema',
    digest: 'What is posited in the immediate is external; in being: begin from finite (abiding), infinite stands as reflection-into-itself against it.',
    clauses: [
      'tag(Determinations,"external-to-reflection")',
      'annotate(Being,{finite:"abiding-foundation", infinite:"reflection-into-itself-over-against"})'
    ],
    predicates: [{ name: 'MarksExternalityAndSchema', args: [] }],
    relations: [
      { predicate: 'overAgainst', from: 'Infinite', to: 'Finite' },
      { predicate: 'beginsFrom', from: 'Analysis', to: 'Finite' }
    ]
  },
  {
    id: 'ess-ref1-op-5-syllogism-structure',
    chunkId: 'ess-ref1-5-syllogism-form',
    label: 'Encode syllogism of external reflection',
    digest: 'Extremes: Immediate vs Reflection-into-itself; Middle: Determinate-immediate connecting them.',
    clauses: [
      'assert(syllogism(ExternalReflection).extremes == ["Immediate","ReflectionIntoItself"])',
      'assert(syllogism(ExternalReflection).middle == "DeterminateImmediate")',
      'partition(MiddleReference, {immediacy:"toImmediate", determinateness:"toReflection"})'
    ],
    predicates: [{ name: 'EncodesSyllogism', args: [] }],
    relations: [
      { predicate: 'connects', from: 'DeterminateImmediate', to: 'Immediate|ReflectionIntoItself' }
    ]
  },
  {
    id: 'ess-ref1-op-6-negating-its-negating',
    chunkId: 'ess-ref1-6-positing-and-negating-its-negating',
    label: 'Posit immediate → negative; negate its negating; alien negative only-in-beginning',
    digest: 'Posits immediate as negative/determined; sublates this positing; negates its negating; the "alien" negative exists only as this beginning.',
    clauses: [
      'assert(posit(Immediate) && determineAs(Immediate,"negative"))',
      'assert(sublate(positing(Immediate)) == true)',
      'assert(negate(negate(Immediate)) == positing(Immediate))',
      'annotate(Negative,{onlyIn:"its-beginning"})'
    ],
    predicates: [{ name: 'DoubleNegationPattern', args: [] }],
    relations: [
      { predicate: 'determinesAs', from: 'Reflection', to: 'Immediate~Negative' }
    ]
  },
  {
    id: 'ess-ref1-op-7-externality-sublated-determining',
    chunkId: 'ess-ref1-7-externality-sublated-determining',
    label: 'Externality sublated → coincidence with immediate → determining reflection',
    digest: 'Immediate is posited as same as reflection; reflection negates its own determining; externality sublated; coincidence with immediate = essence\'s immediacy; hence determining reflection (essence in-and-for-itself).',
    clauses: [
      'assert(positedSame(Immediate, Reflection) == true)',
      'assert(negatesOwnDetermining(Reflection) == true)',
      'tag(Reflection,"externality-sublated")',
      'assert(coincides(Reflection, Immediate) == true)',
      'tag(Essence,"in-and-for-itself")',
      'tag(Reflection,"determining")'
    ],
    predicates: [{ name: 'SublatesExternality', args: [] }],
    relations: [
      { predicate: 'coincides', from: 'Reflection', to: 'Immediate' },
      { predicate: 'yields', from: 'DeterminingReflection', to: 'EssenceInAndForItself' }
    ]
  },
  // Part 3: Determining Reflection Operations
  {
    id: 'ess-ref2-op-0-unify-determining',
    chunkId: 'ess-ref2-0-determining-unity',
    label: 'Unify positing and external reflection as determining reflection',
    digest: 'Determining reflection is the unity/coordination of positing and external reflection.',
    clauses: [
      'assert(DeterminingReflection == unify(PositingReflection, ExternalReflection))'
    ],
    predicates: [{ name: 'UnifiesDeterminingReflection', args: [] }],
    relations: [{ predicate: 'unifies', from: 'DeterminingReflection', to: 'Positing|External' }]
  },
  {
    id: 'ess-ref2-op-1-origins-and-posited',
    chunkId: 'ess-ref2-1-origins-and-incompletion',
    label: 'Origins (being vs nothing); incomplete determining → merely posited (self-negating)',
    digest: 'External begins from being; positing from nothing; determination as merely posited—immediate yet self-negating, only in reflection-into-itself.',
    clauses: [
      'annotate(ExternalReflection,{origin:"immediate-being"})',
      'annotate(PositingReflection,{origin:"nothing"})',
      'assert(externalDetermining.posits(other:=Essence))',
      'assert(PositingReflection.hasPresupposition == false)',
      'tag(Determination,"posited")',
      'tag(Determination,"immediate-self-negating")',
      'assert(onlyIn(Determination, ReflectionIntoItself) && notEquals(Determination, ReflectionIntoItself))'
    ],
    predicates: [{ name: 'ClassifiesOriginsAndPosited', args: [] }],
    relations: [
      { predicate: 'posits', from: 'ExternalReflection', to: 'Essence' },
      { predicate: 'in', from: 'Determination', to: 'ReflectionIntoItself' }
    ]
  },
  {
    id: 'ess-ref2-op-2-correspondence-positedness-existence',
    chunkId: 'ess-ref2-2-positedness-corresponds-existence',
    label: 'Correspondence: positedness ↔ existence; existence is positedness',
    digest: 'In essence, positedness corresponds to existence; ground = pure negativity; determinateness is immediately sublated.',
    clauses: [
      'assert(corresponds(Positedness, Existence))',
      'assert(ground(Positedness) == Essence.asPureNegativity)',
      'tag(Determinateness,"immediately-sublated")',
      'assert(Existence == Positedness)',
      'annotate(Existence,{principle:"essence-of-existence-is-positedness"})'
    ],
    predicates: [{ name: 'RelatesPositednessAndExistence', args: [] }],
    relations: [
      { predicate: 'correspondsTo', from: 'Positedness', to: 'Existence' },
      { predicate: 'groundedIn', from: 'Positedness', to: 'Essence(PureNegativity)' }
    ]
  },
  {
    id: 'ess-ref2-op-3-mediator-and-superiority',
    chunkId: 'ess-ref2-3-positedness-mediator-and-superiority',
    label: 'Positedness mediates essence/existence; twofold sense; "superiority" scoped to essence',
    digest: 'Positedness is the means conjoining existence and essence; "only positedness" has two senses; positedness is superior since existence refers to return-into-itself; yet only with respect to essence.',
    clauses: [
      'assert(means(Positedness, between: [Existence, Essence]))',
      'annotate(Positedness,{twofoldSense:["opposed-to-existence","opposed-to-essence"]})',
      'annotate(Existence,{oftenTakenAs:"superior-to-positedness"})',
      'assert(superior(Positedness, Existence) == true because existenceRefersToReturnIntoItself)',
      'annotate(Positedness,{scope:"with-respect-to-essence", sense:"negation-of-achieved-return"})'
    ],
    predicates: [{ name: 'DefinesMediatorAndScope', args: [] }],
    relations: [
      { predicate: 'conjoins', from: 'Positedness', to: 'Essence|Existence' },
      { predicate: 'refersTo', from: 'Existence', to: 'TurningBackIntoItself' }
    ]
  },
  {
    id: 'ess-ref2-op-4-positedness-to-determination',
    chunkId: 'ess-ref2-4-positedness-becomes-determination',
    label: 'Uniting positing with external reflection makes positedness a determination',
    digest: 'Absolute presupposing (repelling from itself) yields: as posited → negation; as presupposed → reflected-into-itself.',
    clauses: [
      'assert(unite(PositingReflection, ExternalReflection) == AbsolutePresupposing)',
      'tag(Positedness,"as-posited:negation")',
      'tag(Positedness,"as-presupposed:reflected-into-itself")',
      'tag(Positedness,"determination-of-reflection")'
    ],
    predicates: [{ name: 'PromotesPositedness', args: [] }],
    relations: [
      { predicate: 'unites', from: 'PositingReflection', to: 'ExternalReflection' },
      { predicate: 'yields', from: 'AbsolutePresupposing', to: 'DeterminationOfReflection' }
    ]
  },
  {
    id: 'ess-ref2-op-5-contrast-with-quality',
    chunkId: 'ess-ref2-5-determination-vs-quality',
    label: 'Contrast determination of reflection with quality; essential persistence',
    digest: 'Quality = existent negation grounded in being (transient, unequal). Determination of reflection = positedness-as-negation grounded in immanent reflectedness (essential, self-equal).',
    clauses: [
      'annotate(Quality,{reference:"immediate-to-other", ground:"Being", status:"transient", unequalWithin:true})',
      'annotate(DeterminationOfReflection,{reference:"to-immanently-reflected-being", ground:"ImmanentReflectedness", persistsBy:"self-equality", unequalWithin:false, essential:true})',
      'tag(Negative,"only-as-negative (sublated/posited)")'
    ],
    predicates: [{ name: 'ContrastsQualityVsDetermination', args: [] }],
    relations: [
      { predicate: 'groundedIn', from: 'Quality', to: 'Being' },
      { predicate: 'groundedIn', from: 'DeterminationOfReflection', to: 'ImmanentReflectedness' }
    ]
  },
  {
    id: 'ess-ref2-op-6-essential-shine',
    chunkId: 'ess-ref2-6-essential-shine-negation-predominates',
    label: 'Free essentialities; deflect into reflection-into-itself; essential shine; negation predominates',
    digest: 'Determinations-as-free-essentialities (void, no attraction/repulsion); reflection-into-other deflected into reflection-into-itself; essential shine; determining reflection exits from itself, negation predominates.',
    clauses: [
      'tag(DeterminationsOfReflection,"free-essentialities")',
      'annotate(Context,{void:true, reciprocalAttraction:false, reciprocalRepulsion:false})',
      'assert(deflect(ReflectionIntoOther) == ReflectionIntoItself)',
      'tag(Shine,"essential")',
      'assert(exitsFromItself(DeterminingReflection) == true)',
      'annotate(Essence,{equalityWithItself:"lost-in-negation", negation:"predominates"})'
    ],
    predicates: [{ name: 'EstablishesEssentialShine', args: [] }],
    relations: [
      { predicate: 'deflectsTo', from: 'ReflectionIntoOther', to: 'ReflectionIntoItself' },
      { predicate: 'constitutes', from: 'Determinations', to: 'EssentialShine' }
    ]
  },
  {
    id: 'ess-ref2-op-7-two-sides',
    chunkId: 'ess-ref2-7-two-sides-of-determination',
    label: 'Two sides: positedness (implicit unity) and immanent reflection',
    digest: 'Side 1: positedness as negation-as-negation (implicit unity), immediate that self-sublates; determining abides in itself; distinctions taken back into essence. Side 2: reflected-into-themselves; equality not in an other.',
    clauses: [
      'tag(Side1,"positedness/negation-as-negation")',
      'annotate(Side1,{unity:"implicit(in-itself)", immediateSelfSublates:true, determiningAbides:true})',
      'assert(exitsFromItself(Essence) == false)',
      'annotate(Distinctions,{status:"posited-and-taken-back-into-essence"})',
      'tag(Side2,"immanent-reflection")',
      'assert(reflectedIntoItself(Determinations) == true)',
      'assert(equalityWithItself(NegationAsNegation) && not reflectedInto(NegationAsNegation,"non-being"))'
    ],
    predicates: [{ name: 'EncodesTwoSidedStructure', args: [] }],
    relations: [
      { predicate: 'takesBackInto', from: 'Essence', to: 'Distinctions' },
      { predicate: 'reflectsInto', from: 'Determinations', to: 'Themselves' }
    ]
  },
  {
    id: 'ess-ref2-op-8-dual-nature',
    chunkId: 'ess-ref2-8-dual-nature-of-determination',
    label: 'Dual nature: positedness vs essence, and immanent self-reference',
    digest: 'Determination = positedness (negation vs essence) + immanent reflectedness (subsistence).',
    clauses: [
      'tag(DeterminationOfReflection,"positedness:negation-vs-essence")',
      'tag(DeterminationOfReflection,"immanent-reflection:subsistence")',
      'assert(positedness(DeterminationOfReflection) == sublated(DeterminationOfReflection))'
    ],
    predicates: [{ name: 'EncodesDualNature', args: [] }],
    relations: [
      { predicate: 'opposes', from: 'Positedness', to: 'Essence' },
      { predicate: 'subsistsAs', from: 'Determination', to: 'ImmanentReflectedness' }
    ]
  },
  {
    id: 'ess-ref2-op-9-internal-reference-to-negation',
    chunkId: 'ess-ref2-9-reference-to-its-otherness',
    label: 'Reference to otherness internal to the determination (reference to its negation)',
    digest: 'Determination contains both determinate side and its reference, i.e., reference to its negation, not two externals.',
    clauses: [
      'assert(internalizeReference(Determination, to: negation(Determination)))',
      'annotate(Determination,{quiescent:false, excludesExternalDualism:true})'
    ],
    predicates: [{ name: 'InternalizesOtherness', args: [] }],
    relations: [
      { predicate: 'refersTo', from: 'Determination', to: 'ItsNegation' }
    ]
  },
  {
    id: 'ess-ref2-op-10-essential-unity',
    chunkId: 'ess-ref2-10-otherness-taken-back-essentiality',
    label: 'Deflect reference-to-another into itself; unity with its other → essentiality',
    digest: 'Otherness taken back; negation equal-to-itself unifies itself and its other; thereby becomes essential.',
    clauses: [
      'assert(deflectsToSelf(Determination, reference:"to-another"))',
      'assert(equalsToItself(NegationOf(Determination)))',
      'assert(unity(Determination, otherOf(Determination)) && tag(Determination,"essential"))'
    ],
    predicates: [{ name: 'EstablishesEssentiality', args: [] }],
    relations: [
      { predicate: 'unifiesWith', from: 'Determination', to: 'ItsOther' }
    ]
  },
  {
    id: 'ess-ref2-op-11-infinite-self-reference',
    chunkId: 'ess-ref2-11-infinite-reference-to-self',
    label: 'Positedness sublated in self-reflection → infinite self-reference',
    digest: 'As reflection-into-itself, positedness is sublated, yielding infinite reference to itself.',
    clauses: [
      'assert(sublated(Positedness) && via(ImmanentReflection))',
      'tag(DeterminationOfReflection,"infinite-self-reference")'
    ],
    predicates: [{ name: 'EncodesInfiniteSelfReference', args: [] }],
    relations: [
      { predicate: 'returnsInto', from: 'Determination', to: 'Itself(Infinite)' }
    ]
  }
]

/* accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}
export function getLogicalOpsForChunk(oneBasedIndex: number): LogicalOperation[] {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return []
  return LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id)
}
