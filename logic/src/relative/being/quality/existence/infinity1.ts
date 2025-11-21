import type { Chunk, LogicalOperation } from './index';

/*
  Infinity — C. INFINITY: THE DETERMINATION OF BEING (Section b)

  This module covers section b: Alternating determination of finite and infinite
  - Bad infinite setup: immediacy falls back to Something-with-limit
  - Finite as restriction → passes over; Infinite as realized ought
  - Affirmative slips back to Being; Infinite burdened with opposition
  - Bad infinite (of understanding): unreconciled contradictions
  - Two-worlds contradiction: infinite as mere limit → finitized infinite
  - Separation: finite "here," infinite "beyond"
  - Common limit; each finite-in-itself; each posits the other
  - Mediation by negation; exclusion re-finitizes the infinite
  - Alternating transition (external connection)
  - Process: external pass-over → emptiness → limit → cycle to infinity
  - Alternating determination: relativity and inseparable-opposed unity
  - Infinite progress as unresolved contradiction
  - Bad infinite = perpetual ought; monotony
  - Hidden unity as impulse; beyond unattainable

  PHILOSOPHICAL NOTES:

  1. **The Bad Infinite**:
     As immediate, the infinite is "not-finite" and thus falls back into the category
     of Something-with-limit. It resurrects the finite over against it, creating the
     bad infinite—the understanding's "highest" but unreconciled contradictions.

  2. **Two Worlds**:
     Split into two worlds (finite and infinite) makes the infinite a mere limit of
     the finite—thus a finitized infinite. The infinite is separated as "beyond,"
     the finite remains "here"—separation persists.

  3. **Common Limit**:
     The negation connecting them is their common limit. Each has this limit in itself
     (as in-itselfness) and is thus finite-in-itself. Each posits its other: the finite
     posits its non-being as the infinite, and the infinite posits the finite.

  4. **The Alternating Process**:
     The process appears as external: finite passes over into infinite; in the empty
     beyond, limit arises; infinite vanishes, finite steps in; cycle repeats ad infinitum.
     This is the "progress to infinity"—unresolved contradiction.

  5. **Hidden Unity**:
     The unity of finite and infinite is hidden in their qualitative otherness—it is
     their inner unity, the impulse driving the progress. But as "beyond," the infinite
     is unattainable; the finite persists as "this-side" that perennially regenerates.

  6. **Perpetual Ought**:
     The bad infinite equals the perpetual ought: negation of the finite that cannot
     free itself from it. The transcending itself is not transcended—monotonous repetition.
*/

// ============================================================================
// b. ALTERNATING DETERMINATION OF FINITE AND INFINITE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'infinity-6j-immediacy-falls-back-to-something',
    title: 'Bad infinite setup: immediacy as negation of finite → falls back to Something-with-limit',
    text: `b. Alternating determination of finite and infinite

The infinite is; in this immediacy it is at the same time the negation of an other, the finite.
As existent and as non-being of an other it falls back into the category of the something, something with a limit.
Thus the finite stands over against the infinite; the immediacy of the infinite resurrects the being of its negation, the finite.`,
    summary:
      'As immediate, the infinite is “not-finite” and thus a Something with limit. It re-poses the finite over against it.'
  },
  {
    id: 'infinity-6k-finite-as-restriction-infinity-as-realized-ought',
    title: 'Finite as restriction→passes over; Infinity as realized ought (affirmative self-reference)',
    text: `Finite = restriction posited as restriction; as existence it passes over into its in‑itself and becomes infinite.
Infinity = the nothing of the finite, the in‑itself the finite ought to be—reflected-into-self, realized ought, affirmative self-referring being.
Satisfaction: determinateness, alteration, restriction and the ought vanish; the nothing of the finite is posited (as negation of negation, affirmative in itself).`,
    summary:
      'Finite tends beyond itself; Infinite as realized “ought” (affirmative self-reference) where restriction vanishes.'
  },
  {
    id: 'infinity-6l-affirmative-slips-back-to-being-burdened-opposition',
    title: 'Affirmative slips back to Being; Infinite burdened with opposition (beyond/emptiness)',
    text: `Yet this affirmation is qualitative immediacy (Being), hence again opposed by a finite other.
The infinite is burdened with opposition to the finite—“that which is not finite,” a being in the determinateness of negation.
Over against finite realities, the infinite is indeterminate emptiness, a beyond whose in‑itself is not in its determinate existence.`,
    summary:
      'Taking affirmation as simple Being reintroduces opposition: infinite as “not-finite,” empty beyond.'
  },
  {
    id: 'infinity-6m-name-bad-infinite',
    title: 'Name it: the bad infinite (of understanding)',
    text: `As so posed over against the finite, connected as mere others, this is the bad infinite—the understanding’s “highest,” yet unreconciled contradictions ensue on all sides.`,
    summary:
      'Bad infinite = infinite over against finite; understanding’s highest, but contradictory.'
  },
  {
    id: 'infinity-6n-two-worlds-finite-infinite',
    title: 'Two-worlds contradiction: infinite as mere limit of finite → finitized infinite',
    text: `There are now two determinacies, two worlds: finite and infinite.
In their connection the infinite is only the limit of the finite, a determinate—hence a finite infinite.`,
    summary:
      'Split into two worlds makes the infinite a mere limit—thus finitized.'
  },
  {
    id: 'infinity-6o-separation-beyond-vs-here',
    title: 'Separation: finite “here,” infinite “beyond” (yet finite persists)',
    text: `The finite persists as real even when “gone over” into its non‑being (infinite).
Understanding rises to the infinite, but the finite remains “here,” while the infinite, though the in‑itself of the finite, is posited “there” as a nebulous beyond.`,
    summary:
      'Understanding elevates to a beyond; finite remains here—separation persists.'
  },
  {
    id: 'infinity-6p-common-limit-each-is-finite',
    title: 'Common limit and inner finitude: each posits the other',
    text: `Separated, they are also essentially connected by the negation that divides them: the common limit.
Each has this limit in itself (as in‑itselfness) and is thus finite in itself.
Each repels its limit as non‑being and posits it as other: the finite posits its non‑being as the infinite, and the infinite posits the finite.`,
    summary:
      'Negation is their common limit; each is finite-in-itself and posits its other.'
  },
  {
    id: 'infinity-6q-mediation-by-negation-and-exclusion',
    title: 'Mediation by negation; exclusion re-finitizes the infinite',
    text: `Infinite attains affirmative being only via negation-of-negation.
Taken as simple Being, the contained negation is downgraded to immediate negation (determinateness/limit) and excluded as finite—hence the infinite is again finitized.
Since each is through its determination the positing of the other, they are inseparable; but this unity lies hidden in their qualitative otherness.`,
    summary:
      'Affirmation mediated by negation; taken as immediate, it excludes limit and so finitizes itself. Unity remains inner, hidden.'
  },
  {
    id: 'infinity-6r-appearance-of-unity-alternating-transition',
    title: 'Appearance of unity: alternating transition (external connection)',
    text: `Unity appears as the turning-over: finite → infinite and infinite → finite.
Each only emerges in the other; their arising is independent and immediate, the connection merely external—an alternation.`,
    summary:
      'Unity appears as external alternation of transitions: finite↔infinite.'
  },
  {
    id: 'infinity-6s-process-external-emptiness-limit-cycle',
    title: 'Process: external-appearing pass-over → emptiness → limit → cycle to infinity',
    text: `The process of their transition has this shape: the finite passes over into the infinite; this pass-over appears as an external doing. In the empty beyond, because the infinite that stands apart is itself restricted, the limit arises; the infinite vanishes and the finite steps in—again as if externally. We return to the prior determination; the new limit is to be sublated; emptiness returns—so on to infinity.`,
    summary:
      'Pass-over seems external; empty beyond yields a new limit; finite reappears; cycle repeats ad infinitum.'
  },
  {
    id: 'infinity-6t-alternating-determination-relativity-existence',
    title: 'Alternating determination: relativity and inseparable-opposed unity (existence)',
    text: `Finite is finite only with reference to the ought or the infinite; infinite only with reference to the finite. Inseparable yet absolutely other, each has the other in it; each is unity of itself and its other, and, in its determinateness, not to be either—this is existence.`,
    summary:
      'Finite/infinite are relative, inseparable, and opposed; each contains its other. As such determinateness = existence.'
  },
  {
    id: 'infinity-6u-infinite-progress-as-unresolved-contradiction',
    title: 'Infinite progress: self-negating/negating-negation as unresolved contradiction',
    text: `This alternating determination passes as the “progress to infinity.” It erupts when relatives harden into opposition: inseparable yet assigned independent subsistence. The contradiction is not resolved but merely asserted as present—“and so on to infinity.”`,
    summary:
      'Progress to infinity = asserted, unresolved contradiction when relatives are opposed.'
  },
  {
    id: 'infinity-6v-abstract-transcending-bad-infinite-perpetual-ought',
    title: 'Abstract transcending not transcended; bad infinite = perpetual ought; monotony',
    text: `The transcending itself is not transcended: positing a new limit “transcends” the infinite only to return to the finite. The bad infinite equals the perpetual ought: negation of the finite that cannot free itself from it; the finite resurfaces as its other. The progress is repetitive monotony.`,
    summary:
      'Without sublating the transcending, “beyond” returns to finite. Bad infinite ~ perpetual ought; tedious alternation.'
  },
  {
    id: 'infinity-6w-hidden-unity-impulse-beyond-unattainable',
    title: 'Hidden unity as impulse; empty unrest; beyond unattainable, finite this-side',
    text: `The “infinite progress” remains burdened by finitude and is itself finite; yet thereby it is posited as unity of finite and infinite—only not reflected. This hidden unity alone rouses each in the other and drives the progress. Representation fixates on the outside: the empty unrest of crossing limits, ever finding new limits, unable to halt at either limit or infinite. The infinite stands as an unattainable beyond (ought-not attained); thus it has the finite as a this-side that perennially regenerates as other.`,
    summary:
      'Progress presupposes unreflected unity (the true impulse). As beyond, the infinite is unattainable; the finite persists as this-side.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'infinity-op-68-immediacy-falls-back',
    chunkId: 'infinity-6j-immediacy-falls-back-to-something',
    label: 'Immediate infinite = “not-finite” → Something-with-limit; resurrects finite',
    clauses: [
      'assert(immediate(Infinite,true))',
      'assert(negationOf(Infinite,Finite))',
      'assert(fallsBackToCategory(Infinite,"Something"))',
      'assert(hasLimit(Infinite,true))',
      'assert(standsOverAgainst(Finite,Infinite))',
      'assert(resurrectsBeingOf(Infinite,Finite))'
    ],
    predicates: [{ name: 'BadInfiniteSetup', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-69-finite-restriction-infinite-realized-ought',
    chunkId: 'infinity-6k-finite-as-restriction-infinity-as-realized-ought',
    label: 'Finite as restriction passing over; Infinite as realized ought (affirmative self-reference)',
    clauses: [
      'assert(restrictionPositedAsRestriction(Finite,true))',
      'assert(passesOverIntoInItself(Finite,Infinite))',
      'assert(nothingOf(Infinite,Finite))',
      'assert(realizedOught(Infinite,true))',
      'assert(affirmativeSelfReference(Infinite,true))',
      'assert(vanishes(["Determinateness","Alteration","Restriction","Ought"]))'
    ],
    predicates: [{ name: 'RealizedOught', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-70-affirmative-slips-to-being-burdened',
    chunkId: 'infinity-6l-affirmative-slips-back-to-being-burdened-opposition',
    label: 'Affirmative as simple Being → opposition returns; “not-finite” beyond',
    clauses: [
      'assert(asSimpleBeing(Infinite,true))',
      'assert(opposedBy(Finite,Infinite))',
      'assert(isNotFinite(Infinite,true))',
      'assert(asBeyond(Infinite,true))',
      'tag(Infinite,"indeterminate-emptiness")'
    ],
    predicates: [{ name: 'OppositionalReturn', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-71-name-bad-infinite',
    chunkId: 'infinity-6m-name-bad-infinite',
    label: 'Bad infinite of understanding; contradictions unreconciled',
    clauses: [
      'tag(BadInfinite,"of-understanding")',
      'assert(heldAsHighest(Understanding,BadInfinite))',
      'assert(entangledInContradictions(Understanding,true))'
    ],
    predicates: [{ name: 'BadInfinite', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-72-two-worlds-finite-infinite',
    chunkId: 'infinity-6n-two-worlds-finite-infinite',
    label: 'Two worlds; infinite as mere limit → finite-infinite',
    clauses: [
      'assert(twoWorlds(["Finite","Infinite"]))',
      'assert(onlyLimitOf(Infinite,Finite))',
      'tag(Infinite,"finitized")'
    ],
    predicates: [{ name: 'TwoWorlds', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-73-separation-beyond-vs-here',
    chunkId: 'infinity-6o-separation-beyond-vs-here',
    label: 'Separation persists: finite “here”, infinite “beyond”',
    clauses: [
      'assert(persistsAsReal(Finite,true))',
      'assert(separatedAsBeyond(Infinite,true))',
      'assert(positedHere(Finite,true))',
      'assert(positedThere(Infinite,true))'
    ],
    predicates: [{ name: 'Separation', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-74-common-limit-each-finite',
    chunkId: 'infinity-6p-common-limit-each-is-finite',
    label: 'Common limit; each finite-in-itself; each posits the other',
    clauses: [
      'assert(commonLimitOf(["Finite","Infinite"]))',
      'assert(finiteInItself(Finite,true))',
      'assert(finiteInItself(Infinite,true))',
      'assert(positsOther(Finite,Infinite))',
      'assert(positsOther(Infinite,Finite))'
    ],
    predicates: [{ name: 'CommonLimit', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-75-mediation-and-exclusion',
    chunkId: 'infinity-6q-mediation-by-negation-and-exclusion',
    label: 'Affirmation via negation-of-negation; exclusion re-finitizes; unity hidden',
    clauses: [
      'assert(affirmedViaNegationOfNegation(Infinite,true))',
      'assert(excludesLimitAsFinite(Infinite,true))',
      'assert(inseparableThroughPositing(["Finite","Infinite"]))',
      'tag(Unity,"hidden-in-qualitative-otherness")'
    ],
    predicates: [{ name: 'HiddenUnity', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-76-alternating-transition',
    chunkId: 'infinity-6r-appearance-of-unity-alternating-transition',
    label: 'Unity appears as external alternation: finite↔infinite',
    clauses: [
      'assert(turnsOver(Finite,Infinite))',
      'assert(turnsOver(Infinite,Finite))',
      'tag(Connection,"external-alternation")'
    ],
    predicates: [{ name: 'Alternation', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-77-process-cycle-limit',
    chunkId: 'infinity-6s-process-external-emptiness-limit-cycle',
    label: 'Pass-over appears external; emptiness → limit; finite re-steps; cycle repeats',
    clauses: [
      'assert(passesOver(Finite,Infinite))',
      'tag(Appearance,"external-doing")',
      'assert(arises(Limit,true))',
      'assert(vanishes(Infinite,true))',
      'assert(stepsIn(Finite,true))',
      'assert(returnsTo("previous-determination",true))',
      'assert(toBeSublated(Limit,true))',
      'tag(Progression,"to-infinity-again")'
    ],
    predicates: [{ name: 'CycleToInfinity', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-78-relativity-and-existence',
    chunkId: 'infinity-6t-alternating-determination-relativity-existence',
    label: 'Relativity: finite↔(ought/infinite); inseparable-opposed; unity = existence',
    clauses: [
      'assert(onlyWithReference(Finite,["Ought","Infinite"]))',
      'assert(onlyWithReference(Infinite,Finite))',
      'assert(inseparableYetAbsolutelyOther(["Finite","Infinite"]))',
      'assert(contains(Finite,Infinite))',
      'assert(contains(Infinite,Finite))',
      'assert(unityOfItselfAndOther(Finite,true))',
      'assert(unityOfItselfAndOther(Infinite,true))',
      'tag(Existence,"as-such-determinateness")'
    ],
    predicates: [{ name: 'RelationalExistence', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-79-infinite-progress-contradiction',
    chunkId: 'infinity-6u-infinite-progress-as-unresolved-contradiction',
    label: 'Infinite progress as unresolved contradiction (“and so on to infinity”)',
    clauses: [
      'tag(Progress,"to-infinity")',
      'assert(arisesWhen(Relatives,"pressed-to-opposition"))',
      'assert(contradictionUnresolved(true))',
      'tag(Result,"asserted-as-present")'
    ],
    predicates: [{ name: 'UnresolvedProgress', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-80-bad-infinite-perpetual-ought',
    chunkId: 'infinity-6v-abstract-transcending-bad-infinite-perpetual-ought',
    label: 'Transcending not transcended; bad infinite ~ perpetual ought; monotony',
    clauses: [
      'assert(abstractTranscending(true))',
      'assert(notTranscended("transcending",true))',
      'assert(posits(NewLimit,true))',
      'assert(returnsTo(Finite,true))',
      'tag(BadInfinite,"perpetual-ought")',
      'assert(cannotFreeFrom(BadInfinite,Finite))',
      'tag(Progress,"repetitious-monotony")'
    ],
    predicates: [{ name: 'PerpetualOught', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-81-hidden-unity-impulse-beyond',
    chunkId: 'infinity-6w-hidden-unity-impulse-beyond-unattainable',
    label: 'Hidden unity drives progress; beyond unattainable; finite “this-side” regenerates',
    clauses: [
      'assert(positedAsUnity(["Finite","Infinite"]))',
      'assert(notReflected(Unity,true))',
      'assert(drivesProgress(Unity,true))',
      'tag(Progress,"outside-of-unity")',
      'tag(Progress,"empty-unrest")',
      'assert(findsNewLimit(Infinite,true))',
      'assert(cannotHaltAt(["Limit","Infinite"]))',
      'tag(Infinite,"beyond-unattainable")',
      'assert(oughtNotAttained(Infinite,true))',
      'assert(hasThisSideOverAgainst(Infinite,Finite))',
      'assert(perennialRegenerationAsOther(Finite,true))'
    ],
    predicates: [{ name: 'HiddenUnityImpulse', args: [] }],
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
