import type { Chunk, LogicalOperation } from './index'

/*
  Essence — B. SHINE
  
  This module covers the complete Shine section.
  
  Two-fold representation:
  - text: verbatim source segmented into readable chunks (preserve full passage)
  - summary: short IR summary to support HLO extraction (non-destructive)
  
  PHILOSOPHICAL NOTE: The Cit-Citi-Citta Triad
  
  **Cit** (Essence) = Pure Consciousness, the Principle
  **Citi** (Shine) = Consciousness as Activity/Operation  
  **Citta** (Synthesis) = Mind, the Dharma/Law of Citta
  
  Essence (Cit) + Shine (Citi) = Citta (the synthesis, the Law of Citta)
  
  This is the essence of Citta—the Dharma of Citta, the Law of Citta.
  Essence and Shine together synthesize into Citta, the complete structure of Mind.
  
  Shine (Citi) is the activity/operation of consciousness—the movement, the shining,
  the reflective activity that makes Essence (Cit) manifest as Citta (Mind).
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ess-sh-1-being-is-shine',
    title: 'Being is shine — negative posited as negative',
    text: `1. Being is shine.

The being of shine consists solely
in the sublatedness of being,
in being's nothingness;
this nothingness it has in essence,
apart from its nothingness,
apart from essence, it does not exist.
It is the negative posited as negative.`,
    summary: 'Shine = being’s being only as sublatedness (nothingness) in essence; the negative posited as negative.'
  },
  {
    id: 'ess-sh-2-otherness-and-nonexistence',
    title: 'Shine as remainder of being; otherness reduced to non-existence',
    text: `Shine is all that remains of the sphere of being.
But it still seems to have an immediate side
which is independent of essence
and to be, in general, an other of essence.
Other entails in general the two moments
of existence and non-existence.
Since the unessential no longer has a being,
what is left to it of otherness is
only the pure moment of non-existence;
shine is this immediate non-existence,
a non-existence in the determinateness of being,
so that it has existence only with reference to another,
in its non-existence;
it is the non-self-subsistent
which exists only in its negation.`,
    summary: 'As “other” of essence, shine retains only non-existence: it exists only relative to another, as non-self-subsistent, existing only in its negation.'
  },
  {
    id: 'ess-sh-3-reflected-immediacy',
    title: 'Reflected immediacy — empty immediacy of non-existence',
    text: `What is left over to it is thus only
the pure determinateness of immediacy;
it is as reflected immediacy, that is,
one which is only by virtue of
the mediation of its negation
and which, over against this mediation, is
nothing except the empty determination
of the immediacy of non-existence.`,
    summary: 'Shine is reflected immediacy: only through mediation of its negation; merely the empty immediacy of non-existence.'
  },
  {
    id: 'ess-sh-4-skepticism-idealism-intro',
    title: 'Phenomenon of skepticism; appearance of idealism',
    text: `Shine, the “phenomenon” of skepticism,
and also the “appearance” of idealism,
is thus this immediacy which is not
a something nor a thing in general,
not an indifferent being that would exist apart
from its determinateness and connection with the subject.
Skepticism did not permit itself to say “It is,”
and the more recent idealism did not permit itself
to regard cognitions as a knowledge of the thing-in-itself.`,
    summary: 'Skepticism’s “phenomenon” and idealism’s “appearance” = immediacy that is not a thing-in-itself; refusal of “It is”/thing-in-itself knowledge.'
  },
  {
    id: 'ess-sh-5-manifold-without-foundation',
    title: 'Manifold content without being as foundation',
    text: `The shine of the former was supposed absolutely
not to have the foundation of a being:
the thing-in-itself was not supposed
to enter into these cognitions.
But at the same time skepticism allowed
a manifold of determinations for its shine,
or rather the latter turned out to have
the full richness of the world for its content.
Likewise for the appearance of idealism:
it encompassed the full range of these manifold determinacies.
So, the shine of skepticism and the appearance of idealism
do immediately have a manifold of determination.`,
    summary: 'Even without foundation in being, shine/appearance carries a manifold — the richness of the world — immediately.'
  },
  {
    id: 'ess-sh-6-content-immediate-not-posited',
    title: 'Content transposed into shine — immediate, not posited by shine',
    text: `This content, therefore, might well have no being as foundation,
no thing or thing-in-itself;
for itself, it remains as it is;
it is simply transposed from being into shine,
so that the latter has within itself those manifold
determinacies that exist immediately,
each an other to the other.
The shine is thus itself something immediately determined.
It can have this or that content;
but whatever content it has, it has not posited it
but possesses it immediately.`,
    summary: 'Content is transposed from being into shine: immediately present, mutually other, not posited by shine but immediately possessed.'
  },
  {
    id: 'ess-sh-7-idealism-variants',
    title: 'Leibniz, Kant — immediacy of representations/affections',
    text: `Idealism, whether Leibnizian, Kantian, Fichtean, or in any
other form, has not gone further than skepticism in this:
it has not advanced beyond being as determinateness.
Skepticism lets the content of its shine to be given to it;
the shine exists for it immediately,
whatever content it might have.
The Leibnizian monad develops its representations from itself
but is not their generating and controlling force;
they rise up in it as a froth, indifferent,
immediately present to each other and to the monad as well.
Likewise Kant's appearance is a given content of perception
that presupposes affections, determinations of the subject
which are immediate to each other and to the subject.`,
    summary: 'Leibniz: representations arise immediately and indifferently; Kant: appearance as given affections — immediacies not generated/grounded.'
  },
  {
    id: 'ess-sh-8-fichte-obstacle',
    title: 'Fichte’s obstacle — immediate determinateness limiting the I',
    text: `As for the infinite obstacle of Fichte's Idealism,
it might well be that it has no thing-in-itself for foundation,
so that it becomes a determinateness purely within the “I.”
But this determinateness that the “I” makes its own,
sublating its externality,
is to the “I” at the same time an immediate determinateness,
a limitation of the “I” which the latter may transcend
but which contains a side of indifference,
and on account of this indifference,
although internal to the “I,”
it entails an immediate non-being of it.`,
    summary: 'Fichte: the obstacle internalized in the I remains immediate/indifferent — a limiting non-being within the I.'
  },
  {
    id: 'ess-sh-9-immediate-presupposition-and-task',
    title: 'Shine’s immediate presupposition; the task',
    text: `2. Shine thus contains an immediate presupposition,
an independent side vis-à-vis essence.
But the task, inasmuch as this shine is distinct from essence,
is not to demonstrate that it sublates itself
and returns into essence,
for being has returned into essence in its totality;
shine is the null as such.
The task is to demonstrate that the determinations which
distinguish it from essence are the determinations of essence itself;
further, that this determinateness of essence,
which shine is, is sublated in essence itself.`,
    summary: 'Do not show shine self-sublates (being already returned); show: shine’s distinguishing determinations are essence’s own, and this determinateness is sublated within essence.'
  },
  {
    id: 'ess-sh-10-immediacy-of-nonbeing-and-reflective-immediacy',
    title: 'Immediacy of non-being; negativity within essence; being as moment',
    text: `What constitutes the shine is
the immediacy of non-being;
this non-being, however, is nothing else than
the negativity of essence within essence itself.
In essence, being is non-being.
Its inherent nothingness is the
negative nature of essence itself.
But the immediacy or indifference
which this non-being contains is
essences's own absolute in-itself.
The negativity of essence is its self-equality
or its simple immediacy and indifference.
Being has preserved itself in essence inasmuch
as this latter, in its infinite negativity,
has this equality with itself;
it is through this that essence is itself being.
The immediacy that the determinateness has
in shine against essence is
thus none other than essence's own immediacy,
though not the immediacy of an existent
but rather the absolutely mediated
or reflective immediacy which is shine;
being, not as being, but only as
the determinateness of being as against mediation;
being as moment.`,
    summary: 'Shine = immediacy of non-being, i.e., essence’s own negativity. Its “immediacy” is essence’s reflective immediacy; being persists only as moment (determinateness against mediation).'
  },
  {
    id: 'ess-sh-11-two-moments-are-essences-own',
    title: 'Two moments of shine are essence’s moments',
    text: `These two moments [nothingness but as subsisting],
and being but as moment;
or again, negativity existing in itself and reflected immediacy,
these two moments that are the moments of shine,
are thus the moments of essence itself;
it is not that there is a shine of being in essence,
or a shine of essence in being:
the shine in the essence is not the shine of an other
but is rather shine as such, the shine of essence itself.`,
    summary: 'The two moments—negativity-in-itself and reflected immediacy—are essence’s own; shine is not of another but the shine of essence itself.'
  },
  {
    id: 'ess-sh-12-shine-as-essence-determinate-being',
    title: 'Shine as essence’s determinate being; unity of negativity and immediacy',
    text: `Shine is essence itself in the determinateness of being.
Essence has a shine because it is determined within itself
and is therefore distinguished from its absolute unity.
But this determinateness is as determinateness
just as absolutely sublated in it.
For essence is what stands on its own:
it exists as self-mediating through a negation which it itself is.
It is, therefore, the identical unit of absolute negativity and immediacy.
The negativity is negativity in itself;
it is its reference to itself and thus immediacy in itself.
But it is negative reference to itself,
a self-repelling negating;
thus the immediacy existing in itself is
the negative or the determinate over against the negativity.
But this determinateness is itself absolute negativity
and this determining, which as determining immediately sublates itself,
is a turning back into itself.`,
    summary: 'Shine = essence in determinate being. Essence self-mediates; unity of absolute negativity and immediacy. Determining immediately sublates itself—turning back into itself.'
  },
  {
    id: 'ess-sh-13-negation-of-negative-return',
    title: 'Negative returning-to-self; negation over against the negative (absolute sublation)',
    text: `Shine is the negative which has a being,
but in another, in its negation;
it is a non-self-subsisting-being
which is sublated within and null.
And so it is the negative which returns into itself,
the non-subsistent as such, internally non-subsistent.
This reference of the negative or
the non-subsistent to itself is
the immediacy of this non-subsistent;
it is an other than it;
it is its determinateness over against it,
or the negation over against the negative.
But this negation which stands over against the negative is
negativity as referring solely to itself,
the absolute sublation of the determinateness itself.`,
    summary: 'Shine: negative-in-another that returns to itself; negation over the negative = negativity self-referring (absolute sublation of determinateness).'
  },
  {
    id: 'ess-sh-14-infinite-determinateness-identity',
    title: 'Infinite determinateness; identity of negativity and immediacy; essence shines-in-itself',
    text: `The determinateness that shine is in essence is,
therefore, infinite determinateness;
it is only the negative which coincides with itself
and hence a determinateness that, as determinateness,
is self-subsistence and not determined.
Contrariwise, the self-subsistence, as self-referring immediacy,
equally is just determinateness and moment,
negativity solely referring to itself.
This negativity which is identical with immediacy,
and thus the immediacy which is identical with negativity, is essence.
Shine is, therefore, essence itself,
but essence in a determinateness, in such a way, however,
that the determinateness is only a moment,
and the essence is the shining of itself within itself.`,
    summary: 'Shine’s determinateness is infinite: negativity coinciding with itself. In essence, negativity ≡ immediacy; determinateness is only a moment—essence shines within itself.'
  },
  {
    id: 'ess-sh-15-essence-as-reflection',
    title: 'From being/becoming to essence/reflection; unessential is shine contained in essence',
    text: `In the sphere of being, non-being arises over against being,
each equally an immediate, and the truth of both is becoming.
In the sphere of essence, we have the contrast
first of essence and the non-essential,
then of essence and shine,
the non-essential and the shine
being both the leftover of being.
But these two, and no less the
distinction of essence from them,
consist solely in this:
that essence is taken at first as an immediate,
not as it is in itself,
namely as an immediacy which is immediacy
as pure mediacy or absolute negativity.
This first immediacy is thus only the determinateness of immediacy.
The sublating of this determinateness of essence consists, therefore,
in nothing further than showing that the unessential is only shine,
and that essence rather contains this shine within itself.
For essence is an infinite self-contained movement
which determines its immediacy as negativity
and its negativity as immediacy,
and is thus the shining of itself within itself.
In this, in its self-movement,
essence is reflection.`,
    summary: 'Against being/becoming, essence’s contrasts reduce to taking essence as immediate. Show: unessential is only shine, contained within essence. Essence is self-movement: immediacy↔negativity—shining in itself, i.e., reflection.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ess-sh-op-1-define-shine',
    chunkId: 'ess-sh-1-being-is-shine',
    label: 'Define shine as negative posited as negative (being’s nothingness in essence)',
    digest: 'Shine exists only as sublated being (nothingness) in essence; not apart from essence.',
    clauses: [
      'assert(Shine == negative(Being))',
      'assert(Shine.existsOnlyIn(Essence))',
      'tag(Shine,"negative-posited-as-negative")'
    ],
    predicates: [{ name: 'DefinesShine', args: [] }],
    relations: [
      { predicate: 'derivesFrom', from: 'Shine', to: 'Being' },
      { predicate: 'inheresIn', from: 'Shine', to: 'Essence' }
    ]
  },
  {
    id: 'ess-sh-op-2-nonselfsubsistence',
    chunkId: 'ess-sh-2-otherness-and-nonexistence',
    label: 'Shine as non-self-subsistent, existing only in its negation/relative-to-other',
    digest: 'Classify shine: immediate non-existence, exists only relative to another and in its negation.',
    clauses: [
      'if otherness(Shine,Essence) then set(Shine.mode,"non-existence")',
      'assert(existsOnlyRelativeTo(Shine,"another"))',
      'assert(existsOnlyInNegation(Shine) == true)',
      'tag(Shine,"non-self-subsistent")'
    ],
    predicates: [{ name: 'ClassifiesNonSubsistence', args: [] }],
    relations: [
      { predicate: 'relativeTo', from: 'Shine', to: 'Other' },
      { predicate: 'negatedBy', from: 'Shine', to: 'Essence' }
    ]
  },
  {
    id: 'ess-sh-op-3-reflected-immediacy-structure',
    chunkId: 'ess-sh-3-reflected-immediacy',
    label: 'Encode reflected immediacy (mediation through negation)',
    digest: 'Shine is only by mediation of its negation; an empty immediacy of non-existence.',
    clauses: [
      'assert(reflectedImmediacy(Shine) == by(mediationOf(negation(Shine))))',
      'tag(Shine,"empty-immediacy-of-non-existence")'
    ],
    predicates: [{ name: 'EncodesReflectedImmediacy', args: [] }],
    relations: [{ predicate: 'mediatedBy', from: 'Shine', to: 'Negation' }]
  },
  {
    id: 'ess-sh-op-4-skepticism-idealism-refusals',
    chunkId: 'ess-sh-4-skepticism-idealism-intro',
    label: 'Record skepticism/idealism refusals (no “It is”, no thing-in-itself knowledge)',
    digest: 'Skepticism refuses “It is”; idealism refuses knowledge of thing-in-itself; shine/appearance not a thing.',
    clauses: [
      'annotate(Skepticism,{refuses:"It is"})',
      'annotate(Idealism,{refuses:"thing-in-itself-knowledge"})',
      'tag(Shine,"not-thing-in-itself")'
    ],
    predicates: [{ name: 'RecordsRefusals', args: [] }],
    relations: [
      { predicate: 'contrasts', from: 'Shine', to: 'ThingInItself' },
      { predicate: 'relatesAs', from: 'Shine', to: 'Subject' }
    ]
  },
  {
    id: 'ess-sh-op-5-manifold-without-foundation',
    chunkId: 'ess-sh-5-manifold-without-foundation',
    label: 'Allow manifold determinations without being-foundation',
    digest: 'Shine/appearance carries manifold determinations immediately, without foundation in being.',
    clauses: [
      'if not hasFoundationIn(Shine,Being) then allow(Shine.content,"manifold")',
      'tag(Shine,"immediate-manifold")'
    ],
    predicates: [{ name: 'AllowsManifoldContent', args: [] }],
    relations: [{ predicate: 'carries', from: 'Shine', to: 'ManifoldDeterminacies' }]
  },
  {
    id: 'ess-sh-op-6-content-transposed-immediate',
    chunkId: 'ess-sh-6-content-immediate-not-posited',
    label: 'Content transposed from being; in shine immediately, not posited by shine',
    digest: 'Treat content as transposed from being into shine; shine possesses content immediately (not self-posited).',
    clauses: [
      'assert(transposed(content, from:Being, to:Shine))',
      'assert(immediateIn(content, Shine) == true)',
      'assert(selfPositedBy(content, Shine) == false)'
    ],
    predicates: [{ name: 'EncodesTransposition', args: [] }],
    relations: [
      { predicate: 'transposes', from: 'Being', to: 'Shine' },
      { predicate: 'possesses', from: 'Shine', to: 'Content' }
    ]
  },
  {
    id: 'ess-sh-op-7-idealism-variants-immediacy',
    chunkId: 'ess-sh-7-idealism-variants',
    label: 'Leibniz/Kant patterns: immediacy of representations/affections',
    digest: 'Leibniz monad: representations arise immediately, not generated; Kant: appearances as given affections.',
    clauses: [
      'annotate(LeibnizMonad,{representations:"arise-immediately", control:false, indifference:true})',
      'annotate(KantAppearance,{given:true, presupposes:"affections", immediacy:true})'
    ],
    predicates: [{ name: 'ProfilesIdealismVariants', args: [] }],
    relations: [
      { predicate: 'has', from: 'Monad', to: 'Representations' },
      { predicate: 'presupposes', from: 'Appearance', to: 'Affections' }
    ]
  },
  {
    id: 'ess-sh-op-8-fichte-obstacle-immediacy',
    chunkId: 'ess-sh-8-fichte-obstacle',
    label: 'Fichte’s obstacle: internal, immediate limitation (non-being within the I)',
    digest: 'Obstacle internalized by the I remains an immediate, indifferent limit — an immediate non-being of the I.',
    clauses: [
      'annotate(FichteObstacle,{internalTo:"I", immediate:true, indifferent:true})',
      'tag(FichteObstacle,"limit-of-I")',
      'assert(entails(FichteObstacle,"immediate-non-being-of-I"))'
    ],
    predicates: [{ name: 'ProfilesFichteObstacle', args: [] }],
    relations: [
      { predicate: 'internalTo', from: 'Obstacle', to: 'I' },
      { predicate: 'limits', from: 'Obstacle', to: 'I' }
    ]
  },
  {
    id: 'ess-sh-op-9-map-shine-determinations-to-essence',
    chunkId: 'ess-sh-9-immediate-presupposition-and-task',
    label: 'Map shine’s distinguishing determinations to determinations of essence',
    digest: 'Show that shine’s distinctness is composed of essence’s own determinations; its determinateness is sublated in essence.',
    clauses: [
      'for each d in determinations(Shine.distinctFrom(Essence)) map d -> Essence.d',
      'assert(sublatedIn(d, Essence) == true)',
      'tag(Shine,"null-as-such")'
    ],
    predicates: [{ name: 'MapsShineToEssenceDeterminations', args: [] }],
    relations: [
      { predicate: 'correspondsTo', from: 'ShineDetermination', to: 'EssenceDetermination' },
      { predicate: 'sublatedIn', from: 'Determination', to: 'Essence' }
    ]
  },
  {
    id: 'ess-sh-op-10-reflective-immediacy-and-being-as-moment',
    chunkId: 'ess-sh-10-immediacy-of-nonbeing-and-reflective-immediacy',
    label: 'Encode immediacy of non-being as essence’s negativity; mark being as moment',
    digest: 'Non-being = essence’s negativity; shine’s immediacy = reflective immediacy; being persists only as moment.',
    clauses: [
      'assert(nonBeing(Shine) == negativity(Essence))',
      'set(Shine.immediacy,"reflective")',
      'tag(Being,"moment-only")'
    ],
    predicates: [{ name: 'EncodesReflectiveImmediacy', args: [] }],
    relations: [
      { predicate: 'is', from: 'NonBeingOfShine', to: 'EssenceNegativity' },
      { predicate: 'downgradesTo', from: 'Being', to: 'Moment' }
    ]
  },
  {
    id: 'ess-sh-op-11-assert-two-moments-as-essences-own',
    chunkId: 'ess-sh-11-two-moments-are-essences-own',
    label: 'Assert negativity-in-itself and reflected immediacy as essence’s moments',
    digest: 'The two moments of shine are moments of essence; exclude “shine of other”.',
    clauses: [
      'tag(Essence,"has-moment:negativity-in-itself")',
      'tag(Essence,"has-moment:reflected-immediacy")',
      'assert(not(shineOfOther(Essence)))'
    ],
    predicates: [{ name: 'AssertsEssenceOwnMoments', args: [] }],
    relations: [
      { predicate: 'hasMoment', from: 'Essence', to: 'NegativityInItself' },
      { predicate: 'hasMoment', from: 'Essence', to: 'ReflectedImmediacy' }
    ]
  },
  {
    id: 'ess-sh-op-12-unity-and-self-sublating-determining',
    chunkId: 'ess-sh-12-shine-as-essence-determinate-being',
    label: 'Encode unity of negativity and immediacy; determining turns back into itself',
    digest: 'Essence self-mediates: unity(absolute negativity, immediacy); determining immediately sublates itself (return-to-self).',
    clauses: [
      'assert(unity(Essence,{absoluteNegativity, immediacy}))',
      'assert(selfMediates(Essence, by:"its own negation"))',
      'assert(turnsBackIntoItself(determining(Essence)) == true)'
    ],
    predicates: [{ name: 'EncodesUnityAndReturn', args: [] }],
    relations: [
      { predicate: 'unifies', from: 'Essence', to: 'NegativityAndImmediacy' },
      { predicate: 'returnsToSelf', from: 'Determining', to: 'Essence' }
    ]
  },
  {
    id: 'ess-sh-op-13-negation-of-negative',
    chunkId: 'ess-sh-13-negation-of-negative-return',
    label: 'Encode negative returning to itself; negation over the negative as absolute sublation',
    digest: 'Non-subsistent self-refers (yields immediacy); negation-against-negative is self-referential negativity (absolute sublation).',
    clauses: [
      'assert(returnsIntoItself(Negative) == true)',
      'assert(immediacy(NonSubsistent) == selfReference(NonSubsistent))',
      'assert(negationAgainst(Negative) == absoluteSublation(Determinateness))',
      'tag(Shine,"returning-negative")'
    ],
    predicates: [{ name: 'EncodesNegationOfNegative', args: [] }],
    relations: [
      { predicate: 'returnsToSelf', from: 'Negative', to: 'Negative' },
      { predicate: 'absolutelySublates', from: 'Negation', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-sh-op-14-infinite-determinateness',
    chunkId: 'ess-sh-14-infinite-determinateness-identity',
    label: 'Classify shine’s determinateness as infinite; identify negativity with immediacy in essence',
    digest: 'Infinite determinateness: negativity coincides with itself; in essence, negativity ≡ immediacy; determinateness only as moment; essence shines within itself.',
    clauses: [
      'tag(Shine,"infinite-determinateness")',
      'assert(identity(Negativity, Immediacy) in Essence)',
      'tag(Determinateness,"moment-only")',
      'tag(Essence,"shines-in-itself")'
    ],
    predicates: [{ name: 'ClassifiesInfiniteDeterminateness', args: [] }],
    relations: [
      { predicate: 'identifies', from: 'Essence', to: 'Negativity~Immediacy' },
      { predicate: 'hasMoment', from: 'Essence', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-sh-op-15-essence-as-reflection',
    chunkId: 'ess-sh-15-essence-as-reflection',
    label: 'Reduce unessential to shine contained in essence; assert essence as reflection (self-movement)',
    digest: 'Show unessential = shine; essence contains shine; essence = infinite self-movement mapping immediacy↔negativity (reflection).',
    clauses: [
      'assert(Unessential == Shine)',
      'assert(contains(Essence, Shine) == true)',
      'assert(selfMovement(Essence) && maps(Essence, Immediacy, Negativity) && maps(Essence, Negativity, Immediacy))',
      'tag(Essence,"reflection")'
    ],
    predicates: [{ name: 'PromotesEssenceAsReflection', args: [] }],
    relations: [
      { predicate: 'contains', from: 'Essence', to: 'Shine' },
      { predicate: 'selfMoves', from: 'Essence', to: 'Essence' },
      { predicate: 'maps', from: 'Essence', to: 'Immediacy↔Negativity' }
    ]
  }
]
