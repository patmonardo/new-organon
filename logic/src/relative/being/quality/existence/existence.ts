import type { Chunk, LogicalOperation } from './index';

/*
  Existence — A. EXISTENCE AS SUCH

  This module consolidates the complete Existence section:
  - a. Existence in general: From becoming, da-sein, reflection vs posited
  - b. Quality: Immediate unity, reality and negation, limit/restriction
  - c. Something: Negation of negation, beginning of subject, mediation-with-itself, alteration

  PHILOSOPHICAL NOTES:

  1. **Existence Completes Qualitative Logic**:
     Existence is the first moment of Qualitative Logic—the determinate being that emerges
     from Becoming. It is Being with Non-Being, the simple oneness of Being and Nothing
     that has become quiescent. This completes the foundational structure of Qualitative Logic.

  2. **From Becoming to Existence**:
     Existence proceeds from Becoming—it is the simple oneness of Being and Nothing that
     has sublated its mediation. The Becoming lies behind it, and Existence appears as
     immediate, as a first from which the forward move is made.

  3. **Da-sein: Being with Non-Being**:
     Existence is not mere Being but Dasein—Being with Non-Being, where Non-Being is taken
     up into simple unity with Being. This constitutes determinateness as such—the qualitative
     structure that grounds all further determination.

  4. **Quality: Reality and Negation**:
     Quality is existent determinateness—simple, immediate. It appears as Reality (accent on
     being) and Negation (determined with non-being). Both are existences, but Reality conceals
     its negation, while Negation is existence determined with non-being.

  5. **Something: Negation of Negation**:
     Something is the first negation of negation—the beginning of the subject. It is simple
     existent self-reference, mediation-with-itself. As becoming, its moments are existents:
     itself and its negative, the Other. This is alteration—a becoming that has become concrete.

  6. **Qualitative Logic Complete**:
     With Existence, we complete the foundational structure of Qualitative Logic:
     Being → Nothing → Becoming → Existence (Quality → Something)
     This is the qualitative foundation that will be presupposed in Quantitative Logic.
*/

// ============================================================================
// A. EXISTENCE AS SUCH
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'existence-1-outline-as-such',
    title:
      'Existence as such: outline (determinateness → quality → reality/negation → something)',
    text: `In existence
(a) as such, its determinateness is first
(b) to be distinguished as quality.
The latter, however, is to be taken in
both the two determinations of
existence as reality and negation.
In these determinacies, however,
existence is equally reflected into itself,
and, as so reflected, it is posited as
(c) something, an existent.`,
    summary:
      'Outline: determinateness first; becomes quality as reality|negation; in these, existence reflects-into-itself and is posited as "something" (an existent).'
  },
  {
    id: 'existence-2-from-becoming-immediacy',
    title:
      'Existence from becoming: simple oneness; immediate (becoming behind it)',
    text: `Existence proceeds from becoming.
It is the simple oneness of being and nothing.
On account of this simplicity,
it has the form of an immediate.
Its mediation, the becoming, lies behind it;
it has sublated itself,
and existence therefore appears as a first
from which the forward move is made.
It is at first in the one-sided determination of being;
the other determination which it contains, nothing,
will likewise come up in it,
in contrast to the first.`,
    summary:
      'From Becoming: existence = simple oneness of being|nothing; now as immediacy (mediation behind it). Initially one-sided as being; its nothing will appear in contrast.'
  },
  {
    id: 'existence-3-da-being-with-non-being',
    title:
      '"Da"-sein: being with non-being; determinateness as such (in form of being)',
    text: `It is not mere being but existence,
or Dasein [in German];
according to its [German] etymology,
it is being (Sein) in a certain place (da).
But the representation of space does not belong here.
As it follows upon becoming,
existence is in general
being with a non-being,
so that this non-being is taken up
into simple unity with being.
Non-being thus taken up into being
with the result that the concrete whole is
in the form of being, of immediacy,
constitutes determinateness as such.`,
    summary:
      'Da‑sein: not mere being but being‑with‑non‑being; non‑being is taken up into unity, so the whole stands in the form of being (immediacy). This is determinateness as such.'
  },
  {
    id: 'existence-4-reflection-vs-posited-I',
    title:
      'Reflection vs posited I: being-as-moment (for us) vs determinateness-posited (in it)',
    text: `The whole is likewise in the form
or determinateness of being,
since in becoming being has likewise
shown itself to be only a moment:
something sublated, negatively determined.
It is such, however, for us, in our reflection;
not yet as posited in it.
What is posited, however, is
the determinateness as such of existence,
as is also expressed by the da (or "there") of the Dasein.
The two are always to be clearly distinguished.`,
    summary:
      'Method: being is only-a-moment (for us, by reflection); what is posited in existence itself is determinateness‑as‑such ("da"). Keep reflection vs posited strictly distinct.'
  },
  {
    id: 'existence-5-reflection-vs-posited-II',
    title:
      'Reflection vs posited II: scope of commentary vs moments of the fact itself',
    text: `Only that which is posited in a concept
belongs in the course of the elaboration
of the latter to its content.
Any determinateness not yet posited
in the concept itself
belongs instead to our reflection,
whether this reflection is directed to
the nature of the concept itself
or is a matter of external comparison.
To remark on a determinateness
of this last kind can only be
for the clarification or anticipation of the whole
that will transpire in the course of the development itself.
That the whole, the unity of being and nothing,
is in the one-sided determinateness of being
is an external reflection;
but in negation, in something and other,
and so forth, it will become posited.
It was necessary here to call attention to
the distinction just given;
but to comment on all
that reflection can allow itself,
to give an account of it,
would lead to a long-winded anticipation
of what must transpire in the fact itself.
Although such reflections may
serve to facilitate a general overview
and thus facilitate understanding,
they also bring the disadvantage of
being seen as unjustified assertions,
unjustified grounds and foundations,
of what is to follow.
They should be taken for no more than
what they are supposed to be
and should be distinguished from
what constitutes a moment in
the advance of the fact itself.`,
    summary:
      'Discipline: only what is posited belongs to the concept's content; unposited belongs to reflection (ancillary, at most clarifying). Negation/something/other will be posited in due course.'
  },
  {
    id: 'existence-6-correspondence-to-being',
    title:
      'Correspondence: being (indeterminate) vs existence (determinate being, concrete)',
    text: `Existence corresponds to being in the preceding sphere.
But being is the indeterminate;
there are no determinations that therefore transpire in it.
But existence is determinate being, something concrete;
consequently, several determinations,
several distinct relations of its moments,
immediately emerge in it.`,
    summary:
      'Correspondence: being = indeterminate (no inner articulation); existence = determinate being (concrete), where multiple determinations and relations emerge at once.'
  },
  {
    id: 'existence-7-quality-immediate-unity',
    title:
      'Quality I: immediacy of unity (existent = non-being to that extent)',
    text: `b. Quality

On account of the immediacy
with which being and nothing are
one in existence, neither oversteps the other;
to the extent that existence is existent,
to that extent it is non-being;
it is determined.
Being is not the universal,
determinateness not the particular.
Determinateness has yet to detach itself from being;
nor will it ever detach itself from it,
since the now underlying truth is
the unity of non-being with being;
all further determinations will transpire on this basis.
But the connection which determinateness now has
with being is one of the immediate unity of the two,
so that as yet no differentiation between the two is posited.`,
    summary:
      'Immediacy: being≡nothing in existence; "to the extent existent, to that extent non‑being." Determinateness inseparable from being (unity basis); no posited differentiation yet.'
  },
  {
    id: 'existence-8-quality-as-existent-determinateness',
    title:
      'Quality II: definition (existent determinateness; simple, immediate)',
    text: `Determinateness thus isolated by itself,
as existent determinateness, is quality:
something totally simple, immediate.
Determinateness in general is the more universal
which, further determined, can be
something quantitative as well.
On account of this simplicity,
there is nothing further to say
about quality as such.`,
    summary:
      'Definition: "quality" = existent determinateness—simple, immediate. Determinateness (more universal) can further determine as quantity; qua quality, nothing more to add.'
  },
  {
    id: 'existence-9-quality-reality-negation-limit',
    title: 'Quality III: reality and negation; reflection; limit/restriction',
    text: `Existence, however, in which
nothing and being are equally contained,
is itself the measure of the one-sidedness of
quality as an only immediate or existent determinateness.
Quality is equally to be posited in the determination of nothing,
and the result is that the immediate or existent determinateness is
posited as distinct, reflected, and the nothing,
as thus the determinate element of determinateness,
will equally be something reflected, a negation.
Quality, in the distinct value of existent, is reality;
when affected by a negating, it is negation in general,
still a quality but one that counts as a lack
and is further determined as limit, restriction.`,
    summary:
      'Measure: existence measures the one‑sidedness of "quality as mere immediacy." Posit quality also as nothing→ reflection: existent‑determinateness becomes distinct/for‑it, and "nothing" appears as negation. Thus: reality = quality accented as existent; negation = quality under negating (lack), further as limit/restriction.'
  },
  {
    id: 'existence-10-reality-vs-negation-valuation',
    title: 'Quality IV: valuation of reality vs negation (both are existence)',
    text: `Both are an existence, but in reality,
as quality with the accent on being an existent,
that it is determinateness
and hence also negation is concealed;
reality only has, therefore,
the value of something positive
from which negating, restriction, lack, are excluded.
Negation, for its part, taken as mere lack,
would be what nothing is;
but it is an existence, a quality,
only determined with a non-being.`,
    summary:
      'Both are existences. "Reality" (accent on being) conceals its own negation, thus appears as pure positive. "Negation," if taken as mere lack, collapses to nothing; yet as quality it is an existence—determined with non‑being.'
  },
  {
    id: 'existence-11-something-quality-unseparated',
    title:
      'Something I: reality ∧ negation as existences; quality unseparated from existence',
    text: `c. Something

In existence its determinateness has been distinguished as quality;
in this quality as something existing, the distinction exists:
the distinction of reality and negation.
Now though these distinctions are present in existence,
they are just as much null and sublated.
Reality itself contains negation;
it is existence, not indeterminate or abstract being.
Negation is for its part equally existence,
not the supposed abstract nothing
but posited here as it is in itself,
as existent, as belonging to existence.
Thus quality is in general unseparated from existence,
and the latter is only determinate, qualitative being.`,
    summary:
      'Reality and negation are present as existences in quality, yet sublated. Reality contains negation; negation is existent (not abstract nothing). Hence quality is unseparated from existence; existence = determinate, qualitative being.'
  },
  {
    id: 'existence-12-sublation-yields-being-in-itself',
    title:
      'Something II: sublation (not omission) → simplicity mediated → being-in-itself',
    text: `This sublating of the distinction is more than
the mere retraction and external re-omission of it,
or a simple return to the simple beginning,
to existence as such.
The distinction cannot be left out, for it is.
Therefore, what de facto is at hand is this:
existence in general, distinction in it,
and the sublation of this distinction;
the existence, not void of distinctions as at the beginning,
but as again self-equal through the sublation of the distinction;
the simplicity of existence mediated through this sublation.
This state of sublation of the distinction is
existence's own determinateness;
existence is thus being-in-itself;
it is existent, something.`,
    summary:
      'Sublation is not erasure: existence + distinction + its sublation. Existence regains self-equality as simplicity mediated by sublation; this is its own determinateness—being-in-itself, i.e., something.'
  },
  {
    id: 'existence-13-something-negation-of-negation-beginning-of-subject',
    title: 'Something III: negation of negation; beginning of the subject',
    text: `Something is the first negation of negation,
as simple existent self-reference.
Existence, life, thought, and so forth,
essentially take on the determination of an existent being,
a living thing, a thinking mind ("I"), and so forth.
This determination is of the highest importance
if we do not wish to halt at existence, life, thought,
and so forth, as generalities, also not at Godhood (instead of God).
In common representation, something rightly carries
the connotation of a real thing.
Yet it still is a very superficial determination,
just as reality and negation, existence and its determinateness,
though no longer the empty being and nothing,
still are quite abstract determinations.
For this reason they also are the most common expressions,
and a reflection that is still philosophically
unschooled uses them the most;
it casts its distinctions in them,
fancying that in them it has something
really well and firmly determined.
As something, the negative of the negative is
only the beginning of the subject;
its in-itselfness is still quite indeterminate.
It determines itself further on,
at first as existent-for-itself and so on,
until it finally obtains in the concept
the intensity of the subject.
At the base of all these determinations
there lies the negative unity with itself.
In all this, however, care must be taken
to distinguish the first negation, negation as negation in general,
from the second negation, the negation of negation
which is concrete, absolute negativity,
just as the first is on the contrary only abstract negativity.`,
    summary:
      '"Something" = first negation of negation: simple existent self-reference. It is the beginning of the subject (still indeterminate), later developing to for-itself and concept. Distinguish abstract negation from concrete negation-of-negation (absolute negativity).'
  },
  {
    id: 'existence-14-something-mediation-with-itself',
    title: 'Something IV: mediation-with-itself (vs alleged bare immediacy)',
    text: `Something is an existent as the negation of negation,
for such a negation is the restoration of
the simple reference to itself;
but the something is thereby equally
the mediation of itself with itself.
Present in the simplicity of something,
and then with greater determinateness
in being-for-itself,
in the subject, and so forth,
this mediation of itself with itself is
also already present in becoming,
but only as totally abstract mediation;
mediation with itself is posited in the something
in so far as the latter is determined as a simple identity.
Attention can be drawn to the presence of mediation in general,
as against the principle of the alleged bare immediacy of a knowledge
from which mediation should be excluded.
But there is no further need to draw
particular attention to the moment of mediation,
since it is to be found everywhere and on all sides,
in every concept.`,
    summary:
      'As negation-of-negation, something restores self-reference and is mediation-with-itself (already implicit in becoming, there abstract; here posited). This rebuts doctrines of "bare immediacy"; mediation pervades concepts.'
  },
  {
    id: 'existence-15-something-becoming-other',
    title:
      'Something V: mediation-only collapses to unity; becoming with moments as existents (other)',
    text: `This mediation with itself which something is in itself,
when taken only as the negation of negation,
has no concrete determinations for its sides;
thus it collapses into the simple unity which is being.
Something is, and is therefore also an existent.
Further, it is in itself also becoming,
but a becoming that no longer has only
being and nothing for its moments.
One of these moments, being, is now
existence and further an existent.
The other moment is equally an existent,
but determined as the negative of something; an other.`,
    summary:
      'If grasped only as negation-of-negation, mediation lacks sides and collapses to being. Yet something is an existent and in itself becoming whose moments are now existents: itself and its negative, the other.'
  },
  {
    id: 'existence-16-something-alteration-and-other-in-general',
    title:
      'Something VI: alteration; initially only in concept; other as qualitative',
    text: `As becoming, something is a transition,
the moments of which are themselves something,
and for that reason it is an alteration,
a becoming that has already become concrete.
At first, however, something alters only in its concept;
it is not yet posited in this way, as mediated and mediating,
but at first only as maintaining itself
simply in its reference to itself;
and its negative is posited as equally qualitative,
as only an other in general.`,
    summary:
      'Something's becoming is alteration (moments are something). Initially, alteration is only conceptual; something maintains simple self-reference, and its negative is posited merely as qualitative "other in general."'
  }
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // A. EXISTENCE AS SUCH OPERATIONS
  // ============================================================================
  {
    id: 'existence-op-1-outline-as-such',
    chunkId: 'existence-1-outline-as-such',
    label:
      'Outline: determinateness → quality (reality|negation) → something (existent)',
    clauses: [
      'tag(Existence,"as-such")',
      'assert(distinguished(Determinateness,Quality))',
      'assert(determinationsOf(Existence,["reality","negation"]))',
      'assert(reflectedIntoItself(Existence))',
      'tag(Something,"existent")',
    ],
    predicates: [{ name: 'ExistenceOutline', args: [] }],
    relations: [
      { predicate: 'hasDetermination', from: 'Existence', to: 'Reality' },
      { predicate: 'hasDetermination', from: 'Existence', to: 'Negation' },
      { predicate: 'yields', from: 'Existence', to: 'Something' },
    ],
  },
  {
    id: 'existence-op-2-from-becoming-immediate',
    chunkId: 'existence-2-from-becoming-immediacy',
    label: 'From becoming: simple oneness; immediate (mediation behind it)',
    clauses: [
      'assert(proceedsFrom(Existence,Becoming))',
      'tag(Existence,"simple-oneness-of-being-and-nothing")',
      'tag(Existence,"immediate")',
      'assert(mediationBehind(Existence,Becoming))',
      'tag(Existence,"appears-as-first")',
      'assert(contains(Existence,Being))',
      'assert(contains(Existence,Nothing))',
    ],
    predicates: [{ name: 'ExistenceFromBecoming', args: [] }],
    relations: [
      { predicate: 'proceedsFrom', from: 'Existence', to: 'Becoming' },
    ],
  },
  {
    id: 'existence-op-3-being-with-non-being',
    chunkId: 'existence-3-da-being-with-non-being',
    label: 'Being with non-being: determinateness in form of being (immediacy)',
    clauses: [
      'tag(Existence,"being-with-non-being")',
      'assert(takenUpIntoUnity(NonBeing,Being))',
      'assert(inFormOf(Whole,Being))',
      'tag(Determinateness,"as-such")',
      'annotate(Existence,{etymology:"da-sein",spatialRepresentation:"excluded"})',
    ],
    predicates: [{ name: 'BeingWithNonBeing', args: [] }],
    relations: [
      { predicate: 'includes', from: 'Existence', to: 'NonBeing' },
      { predicate: 'includes', from: 'Existence', to: 'Being' },
    ],
  },
  {
    id: 'existence-op-4-reflection-vs-posited-I',
    chunkId: 'existence-4-reflection-vs-posited-I',
    label: 'Being-as-moment (for us) vs determinateness-posited (in it)',
    clauses: [
      'assert(momentOnly(Being,Becoming))',
      'tag(Being,"sublated")',
      'assert(forUs(Reflection,BeingAsMoment))',
      'assert(positedIn(Existence,Determinateness))',
      'tag(Existence,"there-ness-da")',
    ],
    predicates: [{ name: 'ReflectionVsPositedI', args: [] }],
    relations: [
      { predicate: 'posits', from: 'Existence', to: 'Determinateness' },
    ],
  },
  {
    id: 'existence-op-5-reflection-vs-posited-II',
    chunkId: 'existence-5-reflection-vs-posited-II',
    label: 'Scope: only posited belongs to concept; reflection is ancillary',
    clauses: [
      'assert(onlyPositedBelongsToConcept(Content))',
      'assert(unpositedBelongsTo(Reflection))',
      'tag(Reflection,"ancillary")',
      'assert(willBePositedLater(["Negation","Something","Other"]))',
    ],
    predicates: [{ name: 'ReflectionDiscipline', args: [] }],
    relations: [
      {
        predicate: 'belongsTo',
        from: 'UnpositedDeterminateness',
        to: 'Reflection',
      },
    ],
  },
  {
    id: 'existence-op-6-correspondence-to-being',
    chunkId: 'existence-6-correspondence-to-being',
    label:
      'Existence vs being: indeterminate vs determinate (multiple relations emerge)',
    clauses: [
      'assert(correspondsTo(Existence,Being))',
      'tag(Being,"indeterminate")',
      'tag(Existence,"determinate-being")',
      'tag(Existence,"concrete")',
      'assert(multipleDeterminationsEmerge(Existence))',
    ],
    predicates: [{ name: 'ExistenceVsBeing', args: [] }],
    relations: [{ predicate: 'correspondsTo', from: 'Existence', to: 'Being' }],
  },
  // ============================================================================
  // B. QUALITY OPERATIONS
  // ============================================================================
  {
    id: 'existence-op-7-immediate-unity-no-differentiation',
    chunkId: 'existence-7-quality-immediate-unity',
    label: 'Immediate unity: existent ↔ non-being; no posited differentiation',
    clauses: [
      'tag(Existence,"immediate-unity-of-being-and-nothing")',
      'assert(toTheExtent(Existence,"existent","non-being"))',
      'annotate(Determinateness,{inseparableFrom:"Being"})',
      'assert(basisIsUnity(Being,NonBeing))',
      'tag(Existence,"no-differentiation-posited")',
    ],
    predicates: [{ name: 'ImmediateUnity', args: [] }],
    relations: [
      { predicate: 'inseparableFrom', from: 'Determinateness', to: 'Being' },
    ],
  },
  {
    id: 'existence-op-8-quality-definition',
    chunkId: 'existence-8-quality-as-existent-determinateness',
    label: 'Quality = existent determinateness (simple, immediate)',
    clauses: [
      'tag(Quality,"existent-determinateness")',
      'tag(Quality,"simple")',
      'tag(Quality,"immediate")',
      'assert(canFurtherDetermine(Determinateness,"quantity"))',
    ],
    predicates: [{ name: 'QualityDefinition', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-9-reality-and-negation',
    chunkId: 'existence-9-quality-reality-negation-limit',
    label: 'Reality vs negation; reflection; limit/restriction',
    clauses: [
      'assert(measuresOneSidedness(Existence,Quality))',
      'assert(positIn(Quality,"nothing"))',
      'assert(reflected(ExistentDeterminateness))',
      'tag(Negation,"reflected-nothing")',
      'assert(realityIs(Quality,"accent-on-existent"))',
      'assert(negationIs(Quality,"under-negating"))',
      'tag(Negation,"lack")',
      'tag(Negation,"limit")',
      'tag(Negation,"restriction")',
    ],
    predicates: [{ name: 'RealityNegationLimit', args: [] }],
    relations: [
      { predicate: 'determinesAs', from: 'Quality', to: 'Reality' },
      { predicate: 'determinesAs', from: 'Quality', to: 'Negation' },
    ],
  },
  {
    id: 'existence-op-10-valuation-reality-vs-negation',
    chunkId: 'existence-10-reality-vs-negation-valuation',
    label:
      'Both are existence; reality as positive; negation as existent quality',
    clauses: [
      'assert(bothAre(Reality,Negation,"existence"))',
      'tag(Reality,"positive-appearance")',
      'assert(conceals(Reality,["determinateness","negation"]))',
      'assert(wouldBeNothingIfMereLack(Negation))',
      'assert(isExistenceAsQuality(Negation))',
      'annotate(Negation,{determinedWith:"non-being"})',
    ],
    predicates: [{ name: 'RealityNegationValuation', args: [] }],
    relations: [],
  },
  // ============================================================================
  // C. SOMETHING OPERATIONS
  // ============================================================================
  {
    id: 'existence-op-11-something-unity-of-reality-negation',
    chunkId: 'existence-11-something-quality-unseparated',
    label:
      'Reality ∧ negation present and sublated; quality unseparated; existence qualitative',
    clauses: [
      'assert(distinctionPresent(Quality,["reality","negation"]))',
      'assert(sublatedIn(Existence,["reality","negation"]))',
      'assert(contains(Reality,Negation))',
      'assert(isExistence(Negation))',
      'tag(Quality,"unseparated-from-existence")',
      'tag(Existence,"determinate-qualitative-being")',
    ],
    predicates: [{ name: 'RealityNegationInExistence', args: [] }],
    relations: [{ predicate: 'contains', from: 'Reality', to: 'Negation' }],
  },
  {
    id: 'existence-op-12-sublation-being-in-itself',
    chunkId: 'existence-12-sublation-yields-being-in-itself',
    label:
      'Sublation (not omission) → simplicity mediated → being-in-itself (something)',
    clauses: [
      'assert(notMereOmission(Sublation))',
      'assert(triad(Existence,Distinction,Sublation))',
      'tag(Existence,"self-equal-through-sublation")',
      'tag(Existence,"simplicity-mediated")',
      'tag(Existence,"being-in-itself")',
      'tag(Something,"posited")',
    ],
    predicates: [{ name: 'SublationYieldsBeingInItself', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-13-something-negation-of-negation',
    chunkId: 'existence-13-something-negation-of-negation-beginning-of-subject',
    label:
      'Something = negation of negation; beginning of subject; abstract vs concrete negation',
    clauses: [
      'tag(Something,"negation-of-negation")',
      'tag(Something,"self-reference")',
      'tag(Something,"existent")',
      'assert(takesOnDetermination(["Existence","Life","Thought"],"existent"))',
      'assert(distinguish(AbstractNegation,NegationOfNegation))',
      'tag(NegationOfNegation,"concrete-absolute-negativity")',
      'tag(AbstractNegation,"abstract-negativity")',
    ],
    predicates: [{ name: 'SomethingAsBeginning', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-14-mediation-with-itself',
    chunkId: 'existence-14-something-mediation-with-itself',
    label: 'Mediation-with-itself posited (against "bare immediacy")',
    clauses: [
      'assert(mediatesWithItself(Something))',
      'annotate(Becoming,{mediation:"abstract"})',
      'assert(positsMediationAsIdentity(Something))',
      'tag(Mediation,"pervasive-in-concepts")',
      'assert(againstBareImmediacy(Knowledge))',
    ],
    predicates: [{ name: 'MediationPosited', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-15-becoming-with-other',
    chunkId: 'existence-15-something-becoming-other',
    label: 'Something as becoming: moments are existents—self and other',
    clauses: [
      'assert(collapsesTo(Something,Being))',
      'tag(Something,"existent")',
      'assert(inItselfBecoming(Something))',
      'assert(momentIs(Being,Existence))',
      'tag(Other,"negative-of-something")',
      'tag(Other,"existent")',
    ],
    predicates: [{ name: 'SomethingBecomingOther', args: [] }],
    relations: [{ predicate: 'otherOf', from: 'Other', to: 'Something' }],
  },
  {
    id: 'existence-op-16-alteration-and-other-general',
    chunkId: 'existence-16-something-alteration-and-other-in-general',
    label:
      'Alteration: at first only conceptual; negative as qualitative other-in-general',
    clauses: [
      'assert(transition(Something,Alteration))',
      'tag(Alteration,"concrete-becoming")',
      'assert(maintainsSelfReference(Something))',
      'assert(negativePositedAs(Other,"qualitative-other-in-general"))',
    ],
    predicates: [{ name: 'AlterationInitial', args: [] }],
    relations: [],
  },
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
