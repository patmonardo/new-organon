import type { Chunk, LogicalOperation } from '../essence'

/*
  Being — CHAPTER 1: BEING

  This module consolidates the complete Being section:
  - A. Being: Pure being, indeterminate immediacy
  - B. Nothing: Pure nothingness, complete emptiness
  - C. Becoming: Unity of being and nothing, movement, transition to existence

  PHILOSOPHICAL NOTES:

  1. **Being as Core of Pure Algorithm**:
     Being appears simple, but deceptively so—it is really the core of the Pure Algorithm.
     Being is the first moment, the immediate, indeterminate ground from which all
     determination arises. It is the foundation of the Pure Algorithm—the structure
     that processes Being (input) through Essence (Form Processor) to Concept (output).

  2. **Being as Dialectic**:
     Being is dialectic—it immediately passes into Nothing, and Nothing passes into Being.
     This mutual passage is Becoming, the movement where Being and Nothing are absolutely
     distinct yet inseparable, each vanishing into its opposite. Becoming is the truth
     of Being and Nothing—the dialectical movement that collapses into quiescent unity
     and transitions to Existence.

  3. **Reciprocity: No Objectivity without Subjectivity**:
     As Schelling mentions, the idea of reciprocity: No objectivity without subjectivity.
     This is the Logic of Experience. There is no quantity without quality.
     Determinateness:Magnitude—the qualitative structure (determinateness) is presupposed
     in the quantitative structure (magnitude). Being is the qualitative foundation.

  4. **The Structure**:
     Being (Pure, Indeterminate) → Nothing (Pure Emptiness) → Becoming (Dialectical Movement) → Existence
     This is the first moment of the Logic—simple but foundational, the core of the Pure Algorithm.
*/

// ============================================================================
// A. BEING
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'being-1-pure-indeterminate-immediate',
    title: 'Pure being: indeterminate immediacy, empty; equal only to itself; in fact nothing',
    text: `A. BEING

Being, pure being, without further determination.
In its indeterminate immediacy it is equal only to itself
and also not unequal with respect to another;
it has no difference within it, nor any outwardly.
If any determination or content were posited in it as distinct,
or if it were posited by this determination or content
as distinct from an other,
it would thereby fail to hold fast to its purity.
It is pure indeterminateness and emptiness.
There is nothing to be intuited in it,
if one can speak here of intuiting;
or, it is only this pure empty intuiting itself.
Just as little is anything to be thought in it,
or, it is equally only this empty thinking.
Being, the indeterminate immediate is in fact nothing,
and neither more nor less than nothing.`,
    summary:
      'Pure being = indeterminate immediacy, without further determination; equal only to itself, no inner or outer difference; any determination destroys purity. It is pure indeterminateness and emptiness; only empty intuiting/empty thinking. The indeterminate immediate is in fact nothing.'
  },
  {
    id: 'being-2-nothing-pure-emptiness',
    title: 'Nothing: pure nothingness; complete emptiness; absence of determination; same as pure being',
    text: `B. NOTHING

Nothing, pure nothingness;
it is simple equality with itself,
complete emptiness,
complete absence of determination and content;
lack of all distinction within.
In so far as mention can be made here of
intuiting and thinking,
it makes a difference whether something or nothing is
being intuited or thought.
To intuit or to think nothing has therefore a meaning;
the two are distinguished and so nothing is (concretely exists)
in our intuiting or thinking;
or rather it is the empty intuiting and thinking itself,
like pure being.
Nothing is therefore the same determination
or rather absence of determination,
and thus altogether the same as what pure being is.`,
    summary:
      'Pure nothingness = complete emptiness, absence of determination/content, no internal distinction; thinking/intuition of nothing is empty thinking/empty intuiting; nothing is altogether the same as pure being.'
  },
  {
    id: 'being-3-becoming-unity',
    title: 'Becoming: unity of being and nothing; distinct yet inseparable; mutual vanishing',
    text: `C. BECOMING

1. Unity of being and nothing

Pure being and pure nothing are therefore the same.
The truth is neither being nor nothing,
but rather that being has passed over into nothing
and nothing into being;
"has passed over," not passes over.

But the truth is just as much that
they are not without distinction;
it is rather that they are not the same,
that they are absolutely distinct
yet equally unseparated and inseparable,
and that each immediately vanishes in its opposite.

Their truth is therefore this movement of
the immediate vanishing of the one into the other:
becoming, a movement in which the two are distinguished,
but by a distinction which has just as immediately dissolved itself.`,
    summary:
      'Becoming = the truth of being and nothing: each has passed over into the other; they are absolutely distinct yet inseparable; each immediately vanishes in its opposite. Becoming is this movement where distinction is, and immediately dissolves itself.'
  },
  {
    id: 'being-4a-becoming-moments-determinate-unity',
    title: 'Moments I: determinate unity; equally are; vanishing as sublated',
    text: `Becoming is the unseparatedness of being and nothing,
not the unity that abstracts from being and nothing;
as the unity of being and nothing
it is rather this determinate unity,
or one in which being and nothing equally are.
However, inasmuch as being and nothing are
each unseparated from its other, each is not.
In this unity, therefore, they are,
but as vanishing, only as sublated.
They sink from their initially represented self-subsistence
into moments which are still distinguished
but at the same time sublated.`,
    summary:
      'Determinate-unity: being∧nothing unseparated; both equally are yet each is-not (in-unity). Presence only-as-vanishing (sublated). Self-subsistence sinks into distinguished-yet-sublated moments.'
  },
  {
    id: 'being-4b-becoming-moments-two-unities',
    title: 'Moments II: two unities (unequal value)',
    text: `Grasped as thus distinguished,
each is in their distinguishedness
a unity with the other.
Becoming thus contains being and nothing as two such unities,
each of which is itself unity of being and nothing;
the one is being as immediate and as reference to nothing;
the other is nothing as immediate and as reference to being;
in these unities the determinations are of unequal value.`,
    summary:
      'Two-unities: (1) being-immediate→ref-nothing; (2) nothing-immediate→ref-being. Each is unity-of-both; determinations of unequal value.'
  },
  {
    id: 'being-4c-becoming-moments-double-determination',
    title: 'Moments III: double determination—directions',
    text: `Becoming is in this way doubly determined.
In one determination, nothing is the immediate,
that is, the determination begins with nothing
and this refers to being;
that is to say, it passes over into it.
In the other determination, being is the immediate,
that is, the determination begins with being
and this passes over into nothing:
coming-to-be and ceasing-to-be.`,
    summary:
      'Double-determination: nothing→being (coming-to-be); being→nothing (ceasing-to-be).'
  },
  {
    id: 'being-4d-becoming-moments-interpenetration',
    title: 'Moments IV: interpenetration; paralysis; self-sublation',
    text: `Both are the same, becoming,
and even as directions that are so different
they interpenetrate and paralyze each other.
The one is ceasing-to-be;
being passes over into nothing,
but nothing is just as much the opposite of itself,
the passing-over into being, coming-to-be.
This coming-to-be is the other direction;
nothing goes over into being,
but being equally sublates itself
and is rather the passing-over into nothing;
it is ceasing-to-be.
They do not sublate themselves reciprocally
[the one sublating the other externally]
but each rather sublates itself in itself
and is within it the opposite of itself.`,
    summary:
      'Interpenetration: the two directions paralyze each other. Each is opposite-within-itself and self‑sublates (not reciprocal external sublation).'
  },
  {
    id: 'being-5a-becoming-sublation-equilibrium',
    title: 'Becoming: equilibrium and quiescent unity; vanishing of vanishing',
    text: `C. BECOMING

3. Sublation of becoming

The equilibrium in which coming-to-be and ceasing-to-be are poised
is in the first place becoming itself.
But this becoming equally collects itself in quiescent unity.
Being and nothing are in it only as vanishing;
becoming itself, however, is only by virtue of their being distinguished.
Their vanishing is therefore the vanishing of becoming,
or the vanishing of the vanishing itself.
Becoming is a ceaseless unrest that collapses into a quiescent result.`,
    summary:
      'Becoming as equilibrium of directions gathers into quiescent unity; being and nothing are only as vanishing; vanishing of vanishing; unrest collapses into a quiescent result.'
  },
  {
    id: 'being-5b-becoming-sublation-contradiction',
    title: 'Becoming: rests on distinctness; self-contradiction; union destroys itself',
    text: `This can also be expressed thus:
becoming is the vanishing of being into nothing,
and of nothing into being,
and the vanishing of being and nothing in general;
but at the same time it rests on their being distinct.
It therefore contradicts itself in itself,
because what it unites within itself is self-opposed;
but such a union destroys itself.`,
    summary:
      'Becoming vanishes being↔nothing and rests on distinctness; it is self-contradictory and its union destroys itself.'
  },
  {
    id: 'being-5c-becoming-sublation-result',
    title: 'Result: vanishedness (not nothing); quiescent simplicity as being (not-for-itself)',
    text: `This result is a vanishedness, but it is not nothing;
as such, it would be only a relapse into one of
the already sublated determinations
and not the result of nothing and of being.
It is the unity of being and nothing
that has become quiescent simplicity.
But this quiescent simplicity is being,
yet no longer for itself but as determination of the whole.`,
    summary:
      'The result is vanishedness (not nothing): unity of being and nothing as quiescent simplicity—being, not-for-itself, as determination of the whole.'
  },
  {
    id: 'being-5d-becoming-sublation-transition',
    title: 'Transition: becoming → existence (immediate unity as existent)',
    text: `Becoming, as transition into
the unity of being and nothing,
a unity which is as existent
or has the shape of the one-sided
immediate unity of these moments,
is existence.`,
    summary:
      'Becoming transitions into existence: the immediate, one-sided unity of being and nothing as existent.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // A. BEING OPERATIONS
  // ============================================================================
  {
    id: 'being-op-1-pure-without-determination',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Pure being: without further determination; indeterminate immediacy',
    digest:
      'Marks Being as pure, without further determination; indeterminate immediate.',
    clauses: [
      'tag(Being,"pure")',
      'tag(Being,"without-further-determination")',
      'tag(Being,"indeterminate-immediacy")'
    ],
    predicates: [{ name: 'DefinesPureBeing', args: [] }],
    relations: [
      { predicate: 'hasMode', from: 'Being', to: 'IndeterminateImmediate' }
    ]
  },
  {
    id: 'being-op-2-self-equality-no-unequal',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Equal only to itself; not unequal to another',
    digest:
      'Being equals only itself; not unequal with respect to another.',
    clauses: [
      'assert(equalOnlyToItself(Being))',
      'assert(not(unequalToAnother(Being)))'
    ],
    predicates: [{ name: 'SelfEquality', args: [] }],
    relations: []
  },
  {
    id: 'being-op-3-no-difference-in-or-out',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'No inner or outer difference',
    digest:
      'Being has no internal difference and none outwardly.',
    clauses: [
      'annotate(Being,{noDifference:"internal|external"})',
      'tag(Being,"no-difference")'
    ],
    predicates: [{ name: 'NoDifference', args: [] }],
    relations: [
      { predicate: 'lacks', from: 'Being', to: 'Difference' }
    ]
  },
  {
    id: 'being-op-4-determination-destroys-purity',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Any determination destroys purity',
    digest:
      'If any determination/content is posited as distinct, Being loses purity.',
    clauses: [
      'tag(Determination,"excludes-pure-being")',
      'annotate(Being,{constraint:"no-determination"})'
    ],
    predicates: [{ name: 'PurityConstraint', args: [] }],
    relations: [
      { predicate: 'excludes', from: 'PureBeing', to: 'Determination' }
    ]
  },
  {
    id: 'being-op-5-empty-intuiting-thinking',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Empty intuiting and empty thinking',
    digest:
      'Nothing to intuit or think in pure being; only empty intuiting/empty thinking.',
    clauses: [
      'tag(Intuiting,"empty")',
      'tag(Thinking,"empty")'
    ],
    predicates: [{ name: 'EmptyIntuitionThinking', args: [] }],
    relations: []
  },
  {
    id: 'being-op-6-being-equals-nothing',
    chunkId: 'being-1-pure-indeterminate-immediate',
    label: 'Indeterminate immediate is in fact nothing',
    digest:
      'Pure being (indeterminate immediate) is in fact nothing; neither more nor less than nothing.',
    clauses: [
      'assert(equalsTruth(Being, Nothing))',
      'tag(Identity,"being≡nothing")'
    ],
    predicates: [{ name: 'BeingEqualsNothing', args: [] }],
    relations: [
      { predicate: 'equates', from: 'Being', to: 'Nothing' }
    ]
  },
  // ============================================================================
  // B. NOTHING OPERATIONS
  // ============================================================================
  {
    id: 'nothing-op-1-pure-emptiness',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Pure nothingness: complete emptiness; absence of determination/content',
    digest:
      'Marks Nothing as pure nothingness: complete emptiness, absence of determination and content.',
    clauses: [
      'tag(Nothing,"pure")',
      'tag(Nothing,"nothingness")',
      'tag(Nothing,"complete-emptiness")',
      'tag(Nothing,"absence-of-determination")',
      'tag(Nothing,"absence-of-content")'
    ],
    predicates: [{ name: 'DefinesPureNothing', args: [] }],
    relations: []
  },
  {
    id: 'nothing-op-2-self-equality',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Simple equality with itself',
    digest:
      'Nothing has simple equality with itself; not unequal to another.',
    clauses: [
      'assert(equalOnlyToItself(Nothing))',
      'assert(not(unequalToAnother(Nothing)))'
    ],
    predicates: [{ name: 'SelfEquality', args: [] }],
    relations: []
  },
  {
    id: 'nothing-op-3-no-internal-distinction',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Lack of all distinction within',
    digest:
      'No internal distinction is found in Nothing.',
    clauses: [
      'annotate(Nothing,{noDifference:"internal"})',
      'tag(Nothing,"no-internal-distinction")'
    ],
    predicates: [{ name: 'NoInternalDifference', args: [] }],
    relations: [
      { predicate: 'lacks', from: 'Nothing', to: 'Difference' }
    ]
  },
  {
    id: 'nothing-op-4-empty-intuiting-thinking',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Empty intuiting and empty thinking (of nothing)',
    digest:
      'To intuit/think nothing is meaningful as empty intuiting/empty thinking (distinguished from something).',
    clauses: [
      'assert(distinguished(Intuiting,Something,Nothing))',
      'assert(distinguished(Thinking,Something,Nothing))',
      'tag(Intuiting,"empty")',
      'tag(Thinking,"empty")',
      'annotate(Nothing,{positedIn:["intuiting","thinking"]})'
    ],
    predicates: [{ name: 'EmptyIntuitionThinking', args: [] }],
    relations: [
      { predicate: 'positedIn', from: 'Nothing', to: 'Intuiting' },
      { predicate: 'positedIn', from: 'Nothing', to: 'Thinking' }
    ]
  },
  {
    id: 'nothing-op-5-same-as-pure-being',
    chunkId: 'being-2-nothing-pure-emptiness',
    label: 'Nothing is altogether the same as pure being',
    digest:
      'Nothing is the same (absence of determination) as pure being; identity of nothing and pure being.',
    clauses: [
      'assert(equalsTruth(Nothing, Being))',
      'tag(Identity,"nothing≡being")',
      'tag(Nothing,"same-absence-of-determination-as-being")'
    ],
    predicates: [{ name: 'NothingEqualsPureBeing', args: [] }],
    relations: [
      { predicate: 'equates', from: 'Nothing', to: 'Being' }
    ]
  },
  // ============================================================================
  // C. BECOMING OPERATIONS
  // ============================================================================
  {
    id: 'becoming-op-1-truth-as-mutual-passage',
    chunkId: 'being-3-becoming-unity',
    label: 'Truth: neither being nor nothing, but mutual passage (has passed over)',
    digest:
      'Marks the truth as the completed passage of being→nothing and nothing→being (not an ongoing process).',
    clauses: [
      'tag(Truth,"neither-being-nor-nothing")',
      'tag(Truth,"mutual-passage")',
      'annotate(Truth,{aspect:"completed-passage",tense:"perfect"})',
      'assert(passedOver(Being,Nothing))',
      'assert(passedOver(Nothing,Being))'
    ],
    predicates: [{ name: 'MutualPassage', args: [] }],
    relations: [
      { predicate: 'passesOver', from: 'Being', to: 'Nothing' },
      { predicate: 'passesOver', from: 'Nothing', to: 'Being' }
    ]
  },
  {
    id: 'becoming-op-2-distinct-yet-inseparable',
    chunkId: 'being-3-becoming-unity',
    label: 'Absolutely distinct yet inseparable',
    digest:
      'Being and nothing are absolutely distinct, yet unseparated and inseparable.',
    clauses: [
      'assert(distinct(Being,Nothing))',
      'tag(Being,"inseparable-from-Nothing")',
      'tag(Nothing,"inseparable-from-Being")',
      'assert(inseparable(Being,Nothing))'
    ],
    predicates: [{ name: 'DistinctYetInseparable', args: [] }],
    relations: [
      { predicate: 'distinctFrom', from: 'Being', to: 'Nothing' },
      { predicate: 'inseparableWith', from: 'Being', to: 'Nothing' }
    ]
  },
  {
    id: 'becoming-op-3-immediate-mutual-vanishing',
    chunkId: 'being-3-becoming-unity',
    label: 'Each immediately vanishes in its opposite',
    digest:
      'Immediate vanishing of being in nothing and of nothing in being.',
    clauses: [
      'tag(Vanishing,"immediate")',
      'assert(vanishesInto(Being,Nothing,"immediate"))',
      'assert(vanishesInto(Nothing,Being,"immediate"))'
    ],
    predicates: [{ name: 'ImmediateMutualVanishing', args: [] }],
    relations: [
      { predicate: 'vanishesInto', from: 'Being', to: 'Nothing' },
      { predicate: 'vanishesInto', from: 'Nothing', to: 'Being' }
    ]
  },
  {
    id: 'becoming-op-4-becoming-as-movement',
    chunkId: 'being-3-becoming-unity',
    label: 'Becoming: movement where distinction dissolves itself',
    digest:
      'Defines becoming as the movement of immediate vanishing; distinction is present and self-dissolving.',
    clauses: [
      'tag(Becoming,"movement")',
      'tag(Becoming,"unity-of-opposites")',
      'assert(movementOfMutualVanishing(Becoming,Being,Nothing))',
      'assert(distinctionPresent(Becoming,Being,Nothing))',
      'assert(distinctionSelfDissolves(Becoming))'
    ],
    predicates: [{ name: 'BecomingDefined', args: [] }],
    relations: [
      { predicate: 'isTruthOf', from: 'Becoming', to: 'Being' },
      { predicate: 'isTruthOf', from: 'Becoming', to: 'Nothing' }
    ]
  },
  {
    id: 'becoming-op-5-determinate-unity-unseparatedness',
    chunkId: 'being-4a-becoming-moments-determinate-unity',
    label: 'Determinate unity: unseparatedness; equally are; vanishing as sublated',
    digest:
      'Becoming is unseparatedness of being and nothing; a determinate unity in which both equally are, yet each is not; presence only as vanishing (sublated).',
    clauses: [
      'tag(Becoming,"unseparatedness-of-being-and-nothing")',
      'tag(Becoming,"determinate-unity")',
      'assert(unseparated(Being,Nothing))',
      'assert(equallyPresent(Becoming,Being,Nothing))',
      'annotate(Becoming,{eachIsNot:"each-unseparated-from-its-other"})',
      'tag(Becoming,"vanishing")',
      'assert(sublatedIn(Being,Becoming))',
      'assert(sublatedIn(Nothing,Becoming))'
    ],
    predicates: [{ name: 'DeterminateUnity', args: [] }],
    relations: [
      { predicate: 'unseparated', from: 'Being', to: 'Nothing' },
      { predicate: 'sublatedIn', from: 'Being', to: 'Becoming' },
      { predicate: 'sublatedIn', from: 'Nothing', to: 'Becoming' }
    ]
  },
  {
    id: 'becoming-op-6-moments-distinguished-and-sublated',
    chunkId: 'being-4a-becoming-moments-determinate-unity',
    label: 'Moments: still distinguished yet sublated',
    digest:
      'Being and nothing sink from self‑subsistence into moments that remain distinguished but are sublated.',
    clauses: [
      'assert(distinguished(Being,Nothing))',
      'assert(momentOf(Being,Becoming))',
      'assert(momentOf(Nothing,Becoming))',
      'tag(Moments,"distinguished-and-sublated")'
    ],
    predicates: [{ name: 'MomentsSublated', args: [] }],
    relations: [
      { predicate: 'momentOf', from: 'Being', to: 'Becoming' },
      { predicate: 'momentOf', from: 'Nothing', to: 'Becoming' }
    ]
  },
  {
    id: 'becoming-op-7-two-unities-unequal-value',
    chunkId: 'being-4b-becoming-moments-two-unities',
    label: 'Two unities: being-immediate→ref-nothing; nothing-immediate→ref-being (unequal value)',
    digest:
      'Becoming contains two unities: (a) being as immediate, referencing nothing; (b) nothing as immediate, referencing being; their determinations are of unequal value.',
    clauses: [
      'tag(Becoming,"two-unities")',
      'assert(immediateWithReference(Being,Nothing))',
      'assert(immediateWithReference(Nothing,Being))',
      'annotate(Becoming,{unequalValue:true})'
    ],
    predicates: [{ name: 'TwoUnities', args: [] }],
    relations: [
      { predicate: 'immediateTo', from: 'Being', to: 'Nothing' },
      { predicate: 'immediateTo', from: 'Nothing', to: 'Being' }
    ]
  },
  {
    id: 'becoming-op-8-double-determination-directions',
    chunkId: 'being-4c-becoming-moments-double-determination',
    label: 'Double determination: directions (coming-to-be, ceasing-to-be)',
    digest:
      'Two determinations: nothing is immediate and passes over into being (coming-to-be); being is immediate and passes over into nothing (ceasing-to-be).',
    clauses: [
      'tag(Becoming,"doubly-determined")',
      'tag(Direction,"coming-to-be")',
      'tag(Direction,"ceasing-to-be")',
      'assert(direction(Nothing,Being,"coming-to-be"))',
      'assert(direction(Being,Nothing,"ceasing-to-be"))'
    ],
    predicates: [{ name: 'DoubleDetermination', args: [] }],
    relations: [
      { predicate: 'direction', from: 'Nothing', to: 'Being' },
      { predicate: 'direction', from: 'Being', to: 'Nothing' }
    ]
  },
  {
    id: 'becoming-op-9-interpenetrate-and-paralyze',
    chunkId: 'being-4d-becoming-moments-interpenetration',
    label: 'Directions interpenetrate and paralyze each other',
    digest:
      'Even as different directions, coming-to-be and ceasing-to-be interpenetrate and paralyze each other.',
    clauses: [
      'assert(interpenetrate("coming-to-be","ceasing-to-be"))',
      'assert(mutuallyParalyze("coming-to-be","ceasing-to-be"))',
      'annotate(Becoming,{directions:["coming-to-be","ceasing-to-be"]})'
    ],
    predicates: [{ name: 'InterpenetrationParalysis', args: [] }],
    relations: [
      { predicate: 'interpenetrates', from: 'coming-to-be', to: 'ceasing-to-be' },
      { predicate: 'paralyzes', from: 'coming-to-be', to: 'ceasing-to-be' }
    ]
  },
  {
    id: 'becoming-op-10-self-sublation-internal-opposite',
    chunkId: 'being-4d-becoming-moments-interpenetration',
    label: 'Each self-sublates; opposite within itself (not reciprocal external sublation)',
    digest:
      'They do not reciprocally sublate one another; each self‑sublates and is the opposite within itself.',
    clauses: [
      'assert(selfSublates(Being))',
      'assert(selfSublates(Nothing))',
      'assert(oppositeWithinItself(Being))',
      'assert(oppositeWithinItself(Nothing))',
      'tag(Sublation,"internal")'
    ],
    predicates: [{ name: 'SelfSublationInternalOpposite', args: [] }],
    relations: [
      { predicate: 'selfSublates', from: 'Being', to: 'Being' },
      { predicate: 'selfSublates', from: 'Nothing', to: 'Nothing' },
      { predicate: 'oppositeWithin', from: 'Being', to: 'Being' },
      { predicate: 'oppositeWithin', from: 'Nothing', to: 'Nothing' }
    ]
  },
  {
    id: 'becoming-op-11-equilibrium-and-quiescent-unity',
    chunkId: 'being-5a-becoming-sublation-equilibrium',
    label: 'Equilibrium of directions; collects into quiescent unity; vanishing of vanishing',
    digest:
      'Becoming is the equilibrium of coming-to-be and ceasing-to-be; it gathers into quiescent unity. Being and nothing are only as vanishing; their vanishing is the vanishing of becoming (vanishing of vanishing). Ceaseless unrest collapses into a quiescent result.',
    clauses: [
      'assert(equilibriumOf("coming-to-be","ceasing-to-be"))',
      'tag(Becoming,"equilibrium")',
      'assert(collectsInto(Becoming,"quiescent-unity"))',
      'tag(Becoming,"quiescent-unity")',
      'assert(onlyAsVanishing(Being,Becoming))',
      'assert(onlyAsVanishing(Nothing,Becoming))',
      'assert(vanishingOfVanishing(Becoming))',
      'tag(Becoming,"ceaseless-unrest")',
      'assert(collapsesInto(Becoming,"quiescent-result"))'
    ],
    predicates: [{ name: 'EquilibriumQuiescence', args: [] }],
    relations: [
      { predicate: 'inEquilibriumWith', from: 'coming-to-be', to: 'ceasing-to-be' },
      { predicate: 'collectsInto', from: 'Becoming', to: 'QuiescentUnity' },
      { predicate: 'collapsesInto', from: 'Becoming', to: 'QuiescentResult' }
    ]
  },
  {
    id: 'becoming-op-12-contradiction-and-self-destruction',
    chunkId: 'being-5b-becoming-sublation-contradiction',
    label: 'Rests on distinctness; self-contradiction; union destroys itself',
    digest:
      'Becoming rests on the distinctness of being and nothing, yet unites self-opposed terms; it contradicts itself in itself and the union destroys itself.',
    clauses: [
      'assert(distinct(Being,Nothing))',
      'annotate(Becoming,{restsOn:"distinctness"})',
      'tag(Becoming,"self-contradictory")',
      'assert(unitesWithin(Becoming,Being,Nothing))',
      'assert(selfOpposed(Becoming))',
      'assert(destroysItself(Becoming))'
    ],
    predicates: [{ name: 'SelfContradiction', args: [] }],
    relations: [
      { predicate: 'distinctFrom', from: 'Being', to: 'Nothing' }
    ]
  },
  {
    id: 'becoming-op-13-result-vanishedness-not-nothing',
    chunkId: 'being-5c-becoming-sublation-result',
    label: 'Result: vanishedness (not nothing); no relapse',
    digest:
      'The result is vanishedness, not nothing; otherwise it would relapse into a sublated determination.',
    clauses: [
      'tag(Result,"vanishedness")',
      'assert(notEqual(Result,Nothing))',
      'annotate(Result,{noRelapse:true})',
      'assert(resultOf(Result,Becoming))'
    ],
    predicates: [{ name: 'ResultVanishedness', args: [] }],
    relations: [
      { predicate: 'resultOf', from: 'Result', to: 'Becoming' }
    ]
  },
  {
    id: 'becoming-op-14-quiescent-simplicity-as-being-not-for-self',
    chunkId: 'being-5c-becoming-sublation-result',
    label: 'Unity becomes quiescent simplicity: being, not-for-itself, as determination of the whole',
    digest:
      'The unity of being and nothing becomes quiescent simplicity. This quiescent simplicity is being—not for itself—but as determination of the whole.',
    clauses: [
      'assert(unityOf(Being,Nothing,"quiescent-simplicity"))',
      'tag(QuiescentSimplicity,"being")',
      'tag(QuiescentSimplicity,"not-for-itself")',
      'tag(QuiescentSimplicity,"determination-of-whole")'
    ],
    predicates: [{ name: 'QuiescentSimplicity', args: [] }],
    relations: [
      { predicate: 'isUnityOf', from: 'QuiescentSimplicity', to: 'Being' },
      { predicate: 'isUnityOf', from: 'QuiescentSimplicity', to: 'Nothing' }
    ]
  },
  {
    id: 'becoming-op-15-transition-to-existence',
    chunkId: 'being-5d-becoming-sublation-transition',
    label: 'Transition: becoming → existence (immediate unity as existent)',
    digest:
      'Becoming, as transition into the unity of being and nothing, which is as existent—one-sided immediate unity of the moments—is existence.',
    clauses: [
      'assert(transitionTo(Becoming,Existence))',
      'tag(Existence,"immediate-unity-of-being-and-nothing")',
      'tag(Existence,"one-sided-immediacy")'
    ],
    predicates: [{ name: 'TransitionToExistence', args: [] }],
    relations: [
      { predicate: 'transitionsTo', from: 'Becoming', to: 'Existence' }
    ]
  }
]

// Accessors
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS
}

export function getLogicalOpsForChunk(oneBasedIndex: number): LogicalOperation[] {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return []
  return LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id)
}
