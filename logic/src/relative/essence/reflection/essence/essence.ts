import type { Chunk, LogicalOperation } from './index'

/*
  Essence — Complete Introduction and Section A: THE ESSENTIAL AND THE UNESSENTIAL

  This module covers the complete introduction to Essence and Section A.

  Two-fold representation:
  - text: full source chunk (verbatim segments, preserving the whole passage across chunks)
  - summary: short IR summary for HLO extraction (non-destructive)

  PHILOSOPHICAL NOTE: The Cit-Citi-Citta Triad

  **Cit** (Essence) = Pure Consciousness, the Principle
  **Citi** (Shine) = Consciousness as Activity/Operation
  **Citta** (Synthesis) = Mind, the Dharma/Law of Citta

  Essence (Cit) + Shine (Citi) = Citta (the synthesis, the Law of Citta)

  This is the essence of Citta—the Dharma of Citta, the Law of Citta.
  Essence and Shine together synthesize into Citta, the complete structure of Mind.
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  // Introduction: The Truth of Being is Essence
  {
    id: 'ess-intro-1-truth-of-being',
    title: 'The truth of being is essence',
    text: `The truth of being is essence.`,
    summary: 'Essence is the truth of being.'
  },
  {
    id: 'ess-intro-2-mediated-knowledge',
    title: 'Knowledge penetrates beyond immediate being to find essence',
    text: `Being is the immediate.
Since the goal of knowledge is the truth,
what being is in and for itself,
knowledge does not stop at
the immediate and its determinations,
but penetrates beyond it
on the presupposition that
behind this being there still is
something other than being itself,
and that this background
constitutes the truth of being.
This cognition is a mediated knowledge,
for it is not to be found
with and in essence immediately,
but starts off from an other, from being,
and has a prior way to make,
the way that leads over and beyond being
or that rather penetrates into it.
Only inasmuch as knowledge recollects
itself into itself out of immediate being,
does it find essence through this mediation.`,
    summary: 'Knowledge seeks truth beyond immediate being; finds essence through mediated recollection from being.'
  },
  {
    id: 'ess-intro-3-essence-as-past-being',
    title: 'Essence as past (timelessly past) being',
    text: `The German language has kept "essence" (Wesen)
in the past participle (gewesen) of the verb "to be" (sein),
for essence is past [but timelessly past] being.`,
    summary: 'Essence = past (timelessly past) being, preserved in German as gewesen (past participle of sein).'
  },
  {
    id: 'ess-intro-4-movement-of-being-itself',
    title: 'The movement to essence is being\'s own movement',
    text: `When this movement is represented as a pathway of knowledge,
this beginning with being and the subsequent advance
which sublates being and arrives at essence as a mediated term
appears to be an activity of cognition external to being
and indifferent to its nature.

But this course is the movement of being itself.
That it is being's nature to recollect itself,
and that it becomes essence by virtue of this interiorizing,
this has been displayed in being itself.`,
    summary: 'The movement to essence appears external but is being\'s own movement—being recollecting itself into essence.'
  },
  {
    id: 'ess-intro-5-absolute-as-essence',
    title: 'The absolute determined as essence',
    text: `If, therefore, the absolute was
at first determined as being,
now it is determined as essence.`,
    summary: 'The absolute transitions from being to essence.'
  },
  {
    id: 'ess-intro-6-pure-being-presupposes-recollection',
    title: 'Pure being presupposes recollection and movement',
    text: `Cognition cannot in general stop
at the manifold of existence;
but neither can it stop at being, pure being;
immediately one is forced to the reflection that
this pure being, this negation of everything finite,
presupposes a recollection and a movement
which has distilled immediate existence into pure being.
Being thus comes to be determined as essence,
as a being in which everything determined and finite is negated.`,
    summary: 'Pure being presupposes recollection/movement that distills existence; being determined as essence (negation of finite).'
  },
  {
    id: 'ess-intro-7-external-abstraction-problem',
    title: 'External abstraction: essence as artifact',
    text: `So it is simple unity, void of determination,
from which the determinate has been
removed in an external manner;
to this unity the determinate was
itself something external
and, after this removal,
it still remains opposite to it;
for it has not been sublated in itself but relatively,
only with reference to this unity.
We already noted above that if pure essence is defined
as the sum total of all realities,
these realities are equally subject to
the nature of determinateness and abstractive reflection
and their sum total is reduced to empty simplicity.
Thus defined, essence is only a product, an artifact.
External reflection, which is abstraction,
only lifts the determinacies of being
out of what is left over as essence
and only deposits them, as it were,
somewhere else, letting them exist as before.
In this way, however, essence is
neither in itself nor for itself;
it is by virtue of another,
through external abstractive reflection;
and it is for another, namely for abstraction
and in general for the existent
which still remains opposite to it.
In its determination, therefore,
it is a dead and empty absence of determinateness.`,
    summary: 'External abstraction makes essence an artifact: dead, empty, neither in-itself nor for-itself, dependent on external reflection.'
  },
  {
    id: 'ess-intro-8-infinite-movement-of-being',
    title: 'Essence through its own negativity: infinite movement of being',
    text: `As it has come to be here, however,
essence is what it is,
not through a negativity foreign to it,
but through one which is its own:
the infinite movement of being.
It is being-in-and-for-itself,
absolute in-itselfness;
since it is indifferent to
every determinateness of being,
otherness and reference to other have been sublated.
But neither is it only this in-itselfness;
as merely being-in-itself,
it would be only the abstraction of pure essence;
but it is being-for-itself just as essentially;
it is itself this negativity,
the self-sublation of otherness
and of determinateness.`,
    summary: 'Essence through its own negativity (infinite movement of being): being-in-and-for-itself, absolute in-itselfness, yet also being-for-itself—self-sublation of otherness.'
  },
  {
    id: 'ess-intro-9-indeterminate-essence',
    title: 'Essence as indeterminate essence: complete turning back',
    text: `Essence, as the complete turning back of being into itself,
is thus at first the indeterminate essence;
the determinacies of being are sublated in it;
it holds them in itself but without their being posited in it.
Absolute essence in this simple unity with itself has no existence.
But it must pass over into existence,
for it is being-in-and-for-itself;
that is to say, it differentiates
the determinations which it holds in itself,
and, since it is the repelling of itself from itself
or indifference towards itself, negative self-reference,
it thereby posits itself over against itself
and is infinite being-for-itself
only in so far as in thus
differentiating itself from itself
it is in unity with itself.`,
    summary: 'Essence = complete turning back: indeterminate essence holding determinations without positing them; must pass into existence through self-differentiation (negative self-reference).'
  },
  {
    id: 'ess-intro-10-determinations-of-essence',
    title: 'Determinations of essence: self-subsisting yet conjoined',
    text: `This determining is thus of another nature than
the determining in the sphere of being,
and the determinations of essence have
another character than the determinations of being.
Essence is absolute unity of being-in-itself and being-for-itself;
consequently, its determining remains inside this unity;
it is neither a becoming nor a passing over,
just as the determinations themselves are
neither an other as other nor references to some other;
they are self-subsisting but, as such,
at the same time conjoined in the unity of essence.`,
    summary: 'Essence\'s determinations differ from being\'s: self-subsisting yet conjoined in unity, neither becoming nor passing over.'
  },
  {
    id: 'ess-intro-11-essence-must-posit-determinateness',
    title: 'Essence must posit determinateness to give itself existence',
    text: `Since essence is at first simple negativity,
in order to give itself existence and then being-for-itself,
it must now posit in its sphere the determinateness
which it contains in principle only in itself.`,
    summary: 'Essence (simple negativity) must posit determinateness to give itself existence and being-for-itself.'
  },
  {
    id: 'ess-intro-12-essence-as-quality-reflection',
    title: 'Essence as quality: absolute indifference; negativity as reflection',
    text: `Essence is in the whole what quality was in the sphere of being:
absolute indifference with respect to limit.
Quantity is instead this indifference in immediate determination,
limit being in it an immediate external determinateness;
quantity passes over into quantum;
the external limit is necessary to it and exists in it.
In essence, by contrast, the determinateness does not exist;
it is posited only by the essence itself,
not free but only with reference to
the unity of the essence.
The negativity of essence is reflection,
and the determinations are reflected,
posited by the essence itself
in which they remain as sublated.`,
    summary: 'Essence = quality (absolute indifference); determinateness posited by essence itself; negativity = reflection; determinations remain as sublated.'
  },
  {
    id: 'ess-intro-13-essence-between-being-and-concept',
    title: 'Essence between being and concept: transition',
    text: `Essence stands between being and concept;
it makes up their middle,
its movement constituting the transition
of being into the concept.
Essence is being-in-and-for-itself,
but it is this in the determination of being-in-itself;
for its general determination is that it emerges from being
or that it is the first negation of being.
Its movement consists in positing negation
or determination in being,
thereby giving itself existence
and becoming as infinite being-for-itself
what it is in itself.
It thus gives itself its existence
which is equal to its being-in-itself
and becomes concept.`,
    summary: 'Essence = middle between being and concept; first negation of being; posits negation/determination to become concept.'
  },
  {
    id: 'ess-intro-14-concept-as-absolute',
    title: 'Concept as absolute in and for itself',
    text: `For the concept is the absolute
as it is absolutely,
or in and for itself,
in its existence.
But the existence which essence gives to itself is
not yet existence as it is in and for itself
but as essence gives it to itself or as posited,
and hence still distinct from the existence of the concept.`,
    summary: 'Concept = absolute in-and-for-itself; essence\'s existence is posited, distinct from concept\'s existence.'
  },
  {
    id: 'ess-intro-15-threefold-movement',
    title: 'Essence\'s threefold movement: shine/reflection, appearance, revelation',
    text: `First, essence shines within itself
or is reflection;
second, it appears;
third, it reveals itself.`,
    summary: 'Essence\'s threefold movement: (1) shine/reflection, (2) appearance, (3) revelation.'
  },
  {
    id: 'ess-intro-16-three-determinations',
    title: 'Three determinations of essence',
    text: `In the course of its movement,
it posits itself in the following determinations:

I. As simple essence existing in itself,
remaining in itself in its determinations;

II. As emerging into existence,
or according to its concrete existence and appearance;

III. As essence which is one with its appearance,
as actuality.`,
    summary: 'Essence\'s three determinations: (I) simple essence in itself, (II) emerging into existence/appearance, (III) one with appearance (actuality).'
  },
  {
    id: 'ess-intro-17-section-intro-reflection-within',
    title: 'Section I: Essence as Reflection Within',
    text: `SECTION I

Essence as Reflection Within

Essence issues from being;
hence it is not immediately in and for itself
but is a result of that movement.
Or, since essence is taken at first as something immediate,
it is a determinate existence to which another stands opposed;
it is only essential existence, as against the unessential.
But essence is being which has been sublated in and for itself;
what stands over against it is only shine.
The shine, however, is essence's own positing.`,
    summary: 'Section I: Essence as reflection within. Essence issues from being; result of movement; shine is essence\'s own positing.'
  },
  {
    id: 'ess-intro-18-reflection-determines-itself',
    title: 'Reflection determines itself; essentialities; foundation',
    text: `First, essence is reflection.
Reflection determines itself;
its determinations are a positedness
which is immanent reflection at the same time.
Second, these reflective determinations
or essentialities are to be considered.
Third, as the reflection of its immanent determining,
essence turns into foundation and passes over
into concrete existence and appearance.`,
    summary: 'Reflection determines itself (positedness = immanent reflection); essentialities; essence → foundation → existence/appearance.'
  },
  {
    id: 'ess-intro-19-chapter-shine-intro',
    title: 'Chapter 1: Shine — three moments',
    text: `CHAPTER 1

Shine

As it issues from being, essence seems to stand over against it;
this immediate being is, first, the unessential.

But, second, it is more than just the unessential;
it is being void of essence; it is shine.

Third, this shine is not something external,
something other than essence, but is essence's own shining.
This shining of essence within it is reflection.`,
    summary: 'Chapter 1: Shine. Three moments: (1) unessential, (2) shine (being void of essence), (3) essence\'s own shining (reflection).'
  },
  // Section A: THE ESSENTIAL AND THE UNESSENTIAL
  {
    id: 'ess-1-sublated-being',
    title: 'Essence is sublated being',
    text: `A. THE ESSENTIAL AND THE UNESSENTIAL

Essence is sublated being.`,
    summary: 'Define essence as being that has been sublated (negated-and-preserved).'
  },
  {
    id: 'ess-2-determined-negation-and-immediacy',
    title: 'Determined negation; immediacy preserved; equal value as immediacies',
    text: `It is simple equality with itself
but is such as the negation of
the sphere of being in general.
And so it has immediacy over against it,
as something from which it has come to be
but which has preserved and maintained itself in this sublating.
Essence itself is in this determination
an existent immediate essence,
and with reference to it
being is only something negative,
nothing in and for itself:
essence, therefore, is a determined negation.
Being and essence relate to each other in this fashion
as against others in general which are mutually indifferent,
for each has a being, an immediacy,
and according to this being they stand in equal value.`,
    summary: 'Essence = equality-with-self as negation of being; both retain immediacy, yet being is only negative; as immediacies they stand in equal value (indifference).'
  },
  {
    id: 'ess-3-being-unessential-essential-problem',
    title: 'Being as unessential; "essential" as mere other if opposed',
    text: `But as contrasted with essence,
being is at the same time the unessential;
as against essence, it has the determination of something sublated.
And in so far as it thus relates to essence
as an other only in general,
essence itself is not essence proper
but is just another existence, the essential.`,
    summary: 'Opposed to essence, being is unessential (sublated). If essence is only an "other" to being, it lapses into mere "essential" existence, not essence proper.'
  },
  {
    id: 'ess-4-external-positing-and-standpoint',
    title: 'Relapse into existence; external positing of essential vs unessential',
    text: `The distinction of essential and unessential has
made essence relapse into the sphere of existence,
for as essence is at first,
it is determined with respect to being
as an existent and therefore as an other.
The sphere of existence is thus laid out as foundation,
and that in this sphere being is being-in-and-for-itself,
is a further determination external to existence,
just as, contrariwise, essence is indeed being-in-and-for-itself,
but only over against an other, in a determinate respect.
Consequently, inasmuch as essential and unessential aspects are
distinguished in an existence from each other,
this distinguishing is an external positing,
a taking apart that leaves the existence itself untouched;
it is a separation which falls on the side of
a third and leaves undetermined
what belongs to the essential
and what belongs to the unessential.
It is dependent on some external standpoint or consideration
and the same content can therefore sometimes be considered
as essential, sometimes as unessential.`,
    summary: 'Treating essential/unessential as a mere separation in an existent is an external positing (by a third standpoint), leaving the content underdetermined and contingent.'
  },
  {
    id: 'ess-5-absolute-negativity-and-schein',
    title: 'From first negation to absolute negativity; the immediate as Schein (shine)',
    text: `On closer consideration, essence becomes something
only essential as contrasted with an unessential
because essence is only taken,
is as sublated being or existence.
In this fashion, essence is only the first negation,
or the negation, which is determinateness,
through which being becomes only existence,
or existence only an other.
But essence is the absolute negativity of being;
it is being itself, but not being determined only as an other:
it is being rather that has sublated itself
both as immediate being
and as immediate negation,
as the negation which is affected by an otherness.
Being or existence, therefore, does not persist
except as what essence is,
and the immediate which still differs from essence is not just
an unessential existence but an immediate
which is null in and for itself;
it only is a non-essence, shine.`,
    summary: 'Essence is not mere first negation but absolute negativity: being sublating itself as being and as negation. What remains as mere immediacy is null—non-essence (Schein).'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // Introduction Operations
  {
    id: 'ess-intro-op-1-truth-of-being',
    chunkId: 'ess-intro-1-truth-of-being',
    label: 'Assert essence as truth of being',
    digest: 'Essence is the truth of being.',
    clauses: [
      'assert(Essence == truthOf(Being))'
    ],
    predicates: [{ name: 'AssertsTruthOfBeing', args: [] }],
    relations: [{ predicate: 'isTruthOf', from: 'Essence', to: 'Being' }]
  },
  {
    id: 'ess-intro-op-2-mediated-knowledge',
    chunkId: 'ess-intro-2-mediated-knowledge',
    label: 'Encode mediated knowledge: recollection from being to essence',
    digest: 'Knowledge seeks truth beyond immediate being; finds essence through mediated recollection.',
    clauses: [
      'assert(Knowledge.seeks("truth") && beyond("immediate-being"))',
      'assert(finds(Essence) == via(recollection(Knowledge, from:Being)))',
      'tag(Knowledge,"mediated")'
    ],
    predicates: [{ name: 'EncodesMediatedKnowledge', args: [] }],
    relations: [
      { predicate: 'recollects', from: 'Knowledge', to: 'Essence' },
      { predicate: 'startsFrom', from: 'Knowledge', to: 'Being' }
    ]
  },
  {
    id: 'ess-intro-op-3-past-being',
    chunkId: 'ess-intro-3-essence-as-past-being',
    label: 'Tag essence as past (timelessly past) being',
    digest: 'Essence = past (timelessly past) being.',
    clauses: [
      'tag(Essence,"past-being")',
      'annotate(Essence,{temporal:"timelessly-past"})'
    ],
    predicates: [{ name: 'TagsPastBeing', args: [] }],
    relations: [{ predicate: 'derivesFrom', from: 'Essence', to: 'Being(Past)' }]
  },
  {
    id: 'ess-intro-op-4-being-movement',
    chunkId: 'ess-intro-4-movement-of-being-itself',
    label: 'Assert movement to essence is being\'s own movement',
    digest: 'The movement to essence is being\'s own movement—being recollecting itself.',
    clauses: [
      'assert(movement(Being -> Essence) == selfRecollection(Being))',
      'tag(Being,"self-recollecting")'
    ],
    predicates: [{ name: 'AssertsBeingMovement', args: [] }],
    relations: [
      { predicate: 'recollects', from: 'Being', to: 'Essence' },
      { predicate: 'becomes', from: 'Being', to: 'Essence' }
    ]
  },
  {
    id: 'ess-intro-op-5-absolute-transition',
    chunkId: 'ess-intro-5-absolute-as-essence',
    label: 'Mark absolute transition from being to essence',
    digest: 'The absolute transitions from being to essence.',
    clauses: [
      'assert(was(Absolute, "being") && now(Absolute, "essence"))'
    ],
    predicates: [{ name: 'MarksAbsoluteTransition', args: [] }],
    relations: [{ predicate: 'transitions', from: 'Absolute', to: 'Essence' }]
  },
  {
    id: 'ess-intro-op-6-pure-being-presupposition',
    chunkId: 'ess-intro-6-pure-being-presupposes-recollection',
    label: 'Encode pure being presupposes recollection',
    digest: 'Pure being presupposes recollection/movement that distills existence; being determined as essence.',
    clauses: [
      'assert(presupposes(PureBeing, recollection) && presupposes(PureBeing, movement))',
      'assert(distills(recollection, ImmediateExistence) -> PureBeing)',
      'assert(determinedAs(Being, Essence) == true)'
    ],
    predicates: [{ name: 'EncodesPureBeingPresupposition', args: [] }],
    relations: [
      { predicate: 'presupposes', from: 'PureBeing', to: 'Recollection' },
      { predicate: 'distills', from: 'Recollection', to: 'PureBeing' }
    ]
  },
  {
    id: 'ess-intro-op-7-external-abstraction',
    chunkId: 'ess-intro-7-external-abstraction-problem',
    label: 'Detect external abstraction making essence an artifact',
    digest: 'External abstraction makes essence an artifact: dead, empty, dependent on external reflection.',
    clauses: [
      'if abstraction(Essence).by == "external-reflection" then tag(Essence,"artifact")',
      'tag(Essence,"dead-empty")',
      'assert(dependentOn(Essence, ExternalReflection) == true)'
    ],
    predicates: [{ name: 'DetectsExternalAbstraction', args: [] }],
    relations: [
      { predicate: 'dependsOn', from: 'Essence', to: 'ExternalReflection' },
      { predicate: 'makes', from: 'ExternalAbstraction', to: 'Artifact' }
    ]
  },
  {
    id: 'ess-intro-op-8-infinite-movement',
    chunkId: 'ess-intro-8-infinite-movement-of-being',
    label: 'Encode essence through its own negativity: infinite movement',
    digest: 'Essence through its own negativity: being-in-and-for-itself, yet also being-for-itself—self-sublation.',
    clauses: [
      'assert(negativity(Essence) == "own")',
      'tag(Essence,"being-in-and-for-itself")',
      'tag(Essence,"absolute-in-itselfness")',
      'assert(selfSublates(Essence, Otherness) && selfSublates(Essence, Determinateness))'
    ],
    predicates: [{ name: 'EncodesInfiniteMovement', args: [] }],
    relations: [
      { predicate: 'selfSublates', from: 'Essence', to: 'Otherness' },
      { predicate: 'selfSublates', from: 'Essence', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-intro-op-9-indeterminate-essence',
    chunkId: 'ess-intro-9-indeterminate-essence',
    label: 'Encode indeterminate essence: complete turning back',
    digest: 'Essence = complete turning back: holds determinations without positing; must pass into existence.',
    clauses: [
      'assert(turningBack(Essence, from:Being) == "complete")',
      'tag(Essence,"indeterminate")',
      'assert(holds(Essence, Determinations) && not(posited(Essence, Determinations)))',
      'assert(mustPassInto(Essence, Existence) == true)'
    ],
    predicates: [{ name: 'EncodesIndeterminateEssence', args: [] }],
    relations: [
      { predicate: 'turnsBack', from: 'Essence', to: 'Being' },
      { predicate: 'holds', from: 'Essence', to: 'Determinations' }
    ]
  },
  {
    id: 'ess-intro-op-10-self-differentiation',
    chunkId: 'ess-intro-9-indeterminate-essence',
    label: 'Encode self-differentiation: negative self-reference',
    digest: 'Essence differentiates itself through negative self-reference (repelling from itself), positing itself over against itself.',
    clauses: [
      'assert(differentiates(Essence, from:Itself) == true)',
      'tag(Essence,"negative-self-reference")',
      'assert(positsOverAgainst(Essence, Essence) == true)',
      'assert(infiniteBeingForItself(Essence) == via(selfDifferentiation))'
    ],
    predicates: [{ name: 'EncodesSelfDifferentiation', args: [] }],
    relations: [
      { predicate: 'differentiates', from: 'Essence', to: 'Essence' },
      { predicate: 'positsOverAgainst', from: 'Essence', to: 'Essence' }
    ]
  },
  {
    id: 'ess-intro-op-11-determinations-self-subsisting',
    chunkId: 'ess-intro-10-determinations-of-essence',
    label: 'Encode determinations: self-subsisting yet conjoined',
    digest: 'Essence\'s determinations: self-subsisting yet conjoined in unity, neither becoming nor passing over.',
    clauses: [
      'tag(DeterminationsOfEssence,"self-subsisting")',
      'assert(conjoinedIn(DeterminationsOfEssence, UnityOfEssence) == true)',
      'assert(not(becoming(DeterminationsOfEssence)) && not(passingOver(DeterminationsOfEssence)))'
    ],
    predicates: [{ name: 'EncodesSelfSubsistingDeterminations', args: [] }],
    relations: [
      { predicate: 'conjoinedIn', from: 'DeterminationsOfEssence', to: 'UnityOfEssence' }
    ]
  },
  {
    id: 'ess-intro-op-12-must-posit-determinateness',
    chunkId: 'ess-intro-11-essence-must-posit-determinateness',
    label: 'Assert essence must posit determinateness',
    digest: 'Essence (simple negativity) must posit determinateness to give itself existence and being-for-itself.',
    clauses: [
      'tag(Essence,"simple-negativity")',
      'assert(mustPosit(Essence, Determinateness) == true)',
      'assert(toGive(Essence, Existence) && toGive(Essence, BeingForItself))'
    ],
    predicates: [{ name: 'AssertsMustPosit', args: [] }],
    relations: [
      { predicate: 'mustPosit', from: 'Essence', to: 'Determinateness' },
      { predicate: 'gives', from: 'Essence', to: 'Existence' }
    ]
  },
  {
    id: 'ess-intro-op-13-essence-as-quality-reflection',
    chunkId: 'ess-intro-12-essence-as-quality-reflection',
    label: 'Encode essence as quality: reflection',
    digest: 'Essence = quality (absolute indifference); determinateness posited by essence; negativity = reflection.',
    clauses: [
      'assert(corresponds(Essence, Quality) == true)',
      'tag(Essence,"absolute-indifference")',
      'assert(positedBy(Essence, Determinateness) == true)',
      'assert(negativity(Essence) == Reflection)'
    ],
    predicates: [{ name: 'EncodesEssenceAsQuality', args: [] }],
    relations: [
      { predicate: 'correspondsTo', from: 'Essence', to: 'Quality' },
      { predicate: 'posits', from: 'Essence', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-intro-op-14-between-being-and-concept',
    chunkId: 'ess-intro-13-essence-between-being-and-concept',
    label: 'Encode essence as middle: transition from being to concept',
    digest: 'Essence = middle between being and concept; first negation of being; becomes concept.',
    clauses: [
      'tag(Essence,"middle")',
      'assert(between(Essence, Being, Concept) == true)',
      'assert(negation(Essence, of:Being) == "first")',
      'assert(becomes(Essence, Concept) == true)'
    ],
    predicates: [{ name: 'EncodesMiddleTransition', args: [] }],
    relations: [
      { predicate: 'between', from: 'Essence', to: 'Being|Concept' },
      { predicate: 'becomes', from: 'Essence', to: 'Concept' }
    ]
  },
  {
    id: 'ess-intro-op-15-concept-absolute',
    chunkId: 'ess-intro-14-concept-as-absolute',
    label: 'Encode concept as absolute; essence\'s existence as posited',
    digest: 'Concept = absolute in-and-for-itself; essence\'s existence is posited, distinct from concept\'s.',
    clauses: [
      'tag(Concept,"absolute-in-and-for-itself")',
      'tag(ExistenceOfEssence,"posited")',
      'assert(distinct(ExistenceOfEssence, ExistenceOfConcept) == true)'
    ],
    predicates: [{ name: 'EncodesConceptAbsolute', args: [] }],
    relations: [
      { predicate: 'distinctFrom', from: 'ExistenceOfEssence', to: 'ExistenceOfConcept' }
    ]
  },
  {
    id: 'ess-intro-op-16-threefold-movement',
    chunkId: 'ess-intro-15-threefold-movement',
    label: 'Encode threefold movement: shine/reflection, appearance, revelation',
    digest: 'Essence\'s threefold movement: (1) shine/reflection, (2) appearance, (3) revelation.',
    clauses: [
      'assert(movement(Essence) == ["shine/reflection","appearance","revelation"])',
      'tag(Essence,"threefold-movement")'
    ],
    predicates: [{ name: 'EncodesThreefoldMovement', args: [] }],
    relations: [
      { predicate: 'moves', from: 'Essence', to: 'Shine|Reflection' },
      { predicate: 'moves', from: 'Essence', to: 'Appearance' },
      { predicate: 'moves', from: 'Essence', to: 'Revelation' }
    ]
  },
  {
    id: 'ess-intro-op-17-three-determinations',
    chunkId: 'ess-intro-16-three-determinations',
    label: 'Encode three determinations of essence',
    digest: 'Essence\'s three determinations: (I) simple essence in itself, (II) emerging into existence/appearance, (III) one with appearance (actuality).',
    clauses: [
      'assert(determinations(Essence) == ["simple-essence-in-itself","emerging-into-existence","one-with-appearance-actuality"])'
    ],
    predicates: [{ name: 'EncodesThreeDeterminations', args: [] }],
    relations: [
      { predicate: 'hasDetermination', from: 'Essence', to: 'SimpleEssenceInItself' },
      { predicate: 'hasDetermination', from: 'Essence', to: 'EmergingIntoExistence' },
      { predicate: 'hasDetermination', from: 'Essence', to: 'OneWithAppearanceActuality' }
    ]
  },
  {
    id: 'ess-intro-op-18-reflection-within',
    chunkId: 'ess-intro-17-section-intro-reflection-within',
    label: 'Encode Section I: Essence as Reflection Within',
    digest: 'Section I: Essence as reflection within. Essence issues from being; shine is essence\'s own positing.',
    clauses: [
      'tag(SectionI,"essence-as-reflection-within")',
      'assert(issuesFrom(Essence, Being) == true)',
      'assert(shine == positing(Essence))'
    ],
    predicates: [{ name: 'EncodesReflectionWithin', args: [] }],
    relations: [
      { predicate: 'issuesFrom', from: 'Essence', to: 'Being' },
      { predicate: 'posits', from: 'Essence', to: 'Shine' }
    ]
  },
  {
    id: 'ess-intro-op-19-reflection-determines',
    chunkId: 'ess-intro-18-reflection-determines-itself',
    label: 'Encode reflection determines itself; essentialities; foundation',
    digest: 'Reflection determines itself (positedness = immanent reflection); essentialities; essence → foundation → existence/appearance.',
    clauses: [
      'assert(determinesItself(Reflection) == true)',
      'assert(positedness(Reflection) == immanentReflection(Reflection))',
      'assert(turnsInto(Essence, Foundation) && passesOver(Essence, Existence|Appearance))'
    ],
    predicates: [{ name: 'EncodesReflectionDetermines', args: [] }],
    relations: [
      { predicate: 'determines', from: 'Reflection', to: 'Reflection' },
      { predicate: 'turnsInto', from: 'Essence', to: 'Foundation' }
    ]
  },
  {
    id: 'ess-intro-op-20-shine-intro',
    chunkId: 'ess-intro-19-chapter-shine-intro',
    label: 'Encode Chapter 1: Shine — three moments',
    digest: 'Chapter 1: Shine. Three moments: (1) unessential, (2) shine (being void of essence), (3) essence\'s own shining (reflection).',
    clauses: [
      'assert(moments(Shine) == ["unessential","shine-being-void-of-essence","essence-own-shining-reflection"])',
      'assert(shining(Essence, within:Itself) == Reflection)'
    ],
    predicates: [{ name: 'EncodesShineIntro', args: [] }],
    relations: [
      { predicate: 'hasMoment', from: 'Shine', to: 'Unessential' },
      { predicate: 'hasMoment', from: 'Shine', to: 'BeingVoidOfEssence' },
      { predicate: 'hasMoment', from: 'Shine', to: 'EssenceOwnShining' }
    ]
  },
  // Section A Operations
  {
    id: 'ess-op-1-define-essence-as-sublated-being',
    chunkId: 'ess-1-sublated-being',
    label: 'Define essence as sublated being',
    digest: 'Tag Essence as the result of sublation (being negated and preserved).',
    clauses: [
      'if x.kind == "Essence" then assert(x = sublate(Being))',
      'tag(x,"sublated-being")'
    ],
    predicates: [{ name: 'DefinesSublatedBeing', args: [] }],
    relations: [{ predicate: 'derivesFrom', from: 'Essence', to: 'Being' }]
  },
  {
    id: 'ess-op-2-characterize-determined-negation',
    chunkId: 'ess-2-determined-negation-and-immediacy',
    label: 'Characterize essence as determined negation with preserved immediacy; mark indifference',
    digest: 'Essence equals self-equality as negation of being; both retain immediacy; as immediacies, being and essence are indifferent (equal value).',
    clauses: [
      'tag(Essence,"determined-negation")',
      'assert(Essence.relationTo(Being) == "negates-and-preserves")',
      'tag(Essence,"immediate-as-essence"); tag(Being,"immediate-as-being")',
      'annotate(pair:{Essence,Being}, "equal-value-as-immediacies")',
      'record(meta:{indifference:true})'
    ],
    predicates: [{ name: 'CharacterizesDeterminedNegation', args: [] }],
    relations: [
      { predicate: 'negates', from: 'Essence', to: 'Being' },
      { predicate: 'preserves', from: 'Essence', to: 'Being' }
    ]
  },
  {
    id: 'ess-op-3-classify-unessential-vs-essential',
    chunkId: 'ess-3-being-unessential-essential-problem',
    label: 'Classify being as unessential; caution: "essential" vs essence proper',
    digest: 'Opposed to essence, being is unessential (sublated); if essence is only an other to being, it is merely "essential" (another existent), not essence proper.',
    clauses: [
      'if opposes(Being,Essence) then tag(Being,"unessential")',
      'if Essence.role == "mere-other-to-being" then tag(Essence,"essential-not-proper")'
    ],
    predicates: [{ name: 'ClassifiesUnessential', args: [] }],
    relations: [{ predicate: 'opposes', from: 'Being', to: 'Essence' }]
  },
  {
    id: 'ess-op-4-detect-external-positing',
    chunkId: 'ess-4-external-positing-and-standpoint',
    label: 'Detect external positing of essential/unessential within an existent',
    digest: 'Separation of essential vs unessential by a third standpoint is external positing; leaves content underdetermined.',
    clauses: [
      'if partition(existent, ["essential","unessential"]).by == "external-standpoint" then tag(existent,"externally-posited")',
      'annotate(existent,{underdetermined:true, standpoint:"third"})'
    ],
    predicates: [{ name: 'DetectsExternalPositing', args: [] }],
    relations: [{ predicate: 'partitionedBy', from: 'existent', to: 'third-standpoint' }]
  },
  {
    id: 'ess-op-5-assert-absolute-negativity-and-schein',
    chunkId: 'ess-5-absolute-negativity-and-schein',
    label: 'Assert essence as absolute negativity; classify mere immediacy as Schein',
    digest: 'Essence is absolute negativity (self-sublating being and negation). Pure immediacy remaining outside essence is non-essence (Schein).',
    clauses: [
      'tag(Essence,"absolute-negativity")',
      'assert(Essence == selfSublation(Being, Negation))',
      'if immediate(x) && differsFrom(x,Essence) then tag(x,"schein")'
    ],
    predicates: [{ name: 'AssertsAbsoluteNegativity', args: [] }],
    relations: [
      { predicate: 'selfSublates', from: 'Being', to: 'Essence' },
      { predicate: 'classifies', from: 'Essence', to: 'Schein' }
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
