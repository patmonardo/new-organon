import type { Chunk, LogicalOperation } from './index';

/*
  Infinity — C. INFINITY: THE DETERMINATION OF BEING (Section a)

  This module covers section a: The infinite in general
  - Infinite as absolute (self-reference as being/becoming)
  - Distinction: true infinite vs bad infinite (finitized infinite)
  - Outline: (a) affirmative as negation-of-finite; (b) alternating one-sided; (c) self-sublation → true infinite
  - Infinite = negation of negation, true being, elevation above restriction
  - Spirit at home in the infinite (universality, freedom)
  - Finite's nature: transcend restriction, negate negation, become infinite
  - Not external beyond, nor merely subjective elevation
  - Finite's self-elevation via restriction/ought
  - Infinity as finite's vocation (affirmative determination)
  - Conclusion: finite vanishes; only the infinite is

  PHILOSOPHICAL NOTES:

  1. **The Infinite in General**:
     The infinite is negation of negation—the affirmative, true being that has
     reinstated itself out of restrictedness. It is elevation above restriction,
     more intense than first immediate being.

  2. **Spirit at Home**:
     At the mention of the infinite, soul and spirit light up. In the infinite,
     spirit is at home—rising to itself, to universality and freedom. This is
     not abstract but concrete self-recognition.

  3. **Finite's Nature to Become Infinite**:
     The finite's very nature is to transcend itself, negate its negation, and
     become infinite. This is not external power but the finite's own self-elevation
     through reference to restriction/ought.

  4. **Infinity as Vocation**:
     Infinity is the finite's affirmative determination, its vocation, its truth.
     The finite becomes itself the infinite through its nature—not by being sublated
     "in general" but by its own self-transcendence.

  5. **Not External, Not Merely Subjective**:
     The infinite is not a ready-made beyond with the finite fixed outside it, nor
     only our subjective reason elevating itself. The finite itself is elevated,
     and this elevation affects it—the finite's own nature accomplishes this.

  6. **True vs Bad Infinite**:
     Essential distinction: true infinite (of reason) vs bad infinite (of understanding).
     The bad infinite is finitized—wanting to keep the infinite pure from the finite
     makes it finite. The true infinite is the self-sublation of both finite and
     bad infinite in one process.
*/

// ============================================================================
// a. THE INFINITE IN GENERAL
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'infinity-6a-intro-absolute-and-caveat',
    title: 'Infinity: absolute, but not by mere negation of the finite',
    text: `C. INFINITY

The infinite in its simple concept
can be regarded, first of all,
as a fresh definition of the absolute;
as self-reference devoid of determination,
it is posited as being and becoming.
The forms of existence have no place in
the series of determinations that can be
regarded as definitions of the absolute,
since the forms of that sphere are
immediately posited for themselves
only as determinacies, as finite in general.
But the infinite is accepted
unqualifiedly as absolute,
since it is explicitly determined
as the negation of the finite;
the restrictedness
(to which being and becoming would somehow be susceptible
even if they do not have it or exhibit it)
is thereby both explicitly referred to and denied in it.
But, in fact, by just this negation the infinite is
not already free from restrictedness and finitude.
It is essential to distinguish
the true concept of infinity
from bad infinity,
the infinite of reason from
the infinite of the understanding.
The latter is in fact a finitized infinite,
and, as we shall now discover,
in wanting to maintain the infinite
pure and distant from the finite,
the infinite is by that very fact
only made finite.`,
    summary:
      'Infinite often taken as absolute (self-reference as being/becoming). Mere negation of the finite does not free it from finitude; distinguish true vs bad infinite. Bad infinite is finitized—wanting to keep it pure makes it finite.'
  },
  {
    id: 'infinity-6b-outline-a-b-c',
    title: 'Outline: (a) affirmative as negation-of-finite; (b) alternating one-sided; (c) self-sublation → true infinite',
    text: `The latter is in fact a finitized infinite ... In wanting to keep the infinite pure from the finite, it is thereby made finite.
The infinite (a) in simple determination is the affirmative as negation of the finite;
(b) thereby in alternating determination with the finite, abstract/one-sided;
(c) the self-sublation of this infinite and of the finite in one process. This is the true infinite.`,
    summary:
      'Schema: (a) infinite as negation of finite; (b) alternation with finite (bad infinite); (c) their self-sublation in one process = true infinite.'
  },
  {
    id: 'infinity-6c-def-negation-of-negation',
    title: 'a) Infinite in general: negation of negation, true being, elevation above restriction',
    text: `The infinite is the negation of negation, the affirmative,
being that has reinstated itself out of restrictedness.
The infinite is, in a more intense sense than first being, the true being—the elevation above restriction.`,
    summary:
      'Infinite = negation of negation (affirmative true being), reinstated beyond restriction—an elevation.'
  },
  {
    id: 'infinity-6d-spirit-at-home',
    title: 'Spirit at home in the infinite',
    text: `At the mention of the infinite, soul and spirit light up; in the infinite the spirit is at home—not abstractly but rising to itself, universality, freedom.`,
    summary:
      'Spirit recognizes itself in the infinite: universality and freedom.'
  },
  {
    id: 'infinity-6e-finite-nature-to-transcend',
    title: 'Finite’s nature: transcend restriction and become infinite',
    text: `What is first given with the concept of the infinite: in-itself, existence is finite and transcends restriction. It is the nature of the finite to transcend itself, negate its negation, and become infinite.`,
    summary:
      'Finite intrinsically transcends itself (negates its negation) and becomes infinite.'
  },
  {
    id: 'infinity-6f-not-external-not-merely-subjective',
    title: 'Not an external beyond, nor merely our subjective elevation',
    text: `The infinite is not a ready-made beyond with the finite fixed outside it, nor only our subjective reason elevating itself without affecting the finite.`,
    summary:
      'Infinite is not a fixed beyond, nor merely subjective; it implicates the finite itself.'
  },
  {
    id: 'infinity-6g-finite-self-elevation-via-restriction-ought',
    title: 'Finite elevates itself: refers to restriction/ought and sublates it',
    text: `Insofar as the finite itself is elevated, it is its nature to refer to itself as restriction (and as ought) and to transcend it—indeed to have already negated it in that self-reference.`,
    summary:
      'Finite’s self-reference to restriction/ought is already sublation—its own elevation.'
  },
  {
    id: 'infinity-6h-infinity-as-vocation',
    title: 'Infinity as the finite’s affirmative determination (vocation)',
    text: `Infinity does not come by sublating “the finite in general”; rather the finite is such that through its nature it becomes itself the infinite. Infinity is its affirmative determination, its vocation, its truth.`,
    summary:
      'Infinity is the finite’s own truth/vocation—its affirmative determination.'
  },
  {
    id: 'infinity-6i-only-infinite-is',
    title: 'Conclusion of (a): the finite vanishes; what is, is only the infinite',
    text: `The finite has thus vanished into the infinite and what is, is only the infinite.`,
    summary:
      'Finite vanishes into the infinite; only the infinite is.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'infinity-op-59-intro-absolute-caveat',
    chunkId: 'infinity-6a-intro-absolute-and-caveat',
    label: 'Infinite as absolute by self-reference; beware finitized (bad) infinite',
    clauses: [
      'tag(Infinite,"absolute")',
      'tag(Infinite,"self-reference-as-being/becoming")',
      'assert(distinguish(TrueInfinite,BadInfinite))',
      'tag(BadInfinite,"finitized")'
    ],
    predicates: [{ name: 'InfinityIntro', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-60-outline-abc',
    chunkId: 'infinity-6b-outline-a-b-c',
    label: '(a) negate finite; (b) alternation (one-sided); (c) self-sublation = true infinite',
    clauses: [
      'assert(stages(Infinite,["negate-finite","alternation","self-sublation"]))',
      'assert(defines(TrueInfinite,"self-sublation-of-finite-and-bad-infinite"))'
    ],
    predicates: [{ name: 'InfinityOutline', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-61-negation-of-negation',
    chunkId: 'infinity-6c-def-negation-of-negation',
    label: 'Infinite = negation-of-negation; true being; elevation above restriction',
    clauses: [
      'assert(isNegationOfNegation(Infinite,true))',
      'tag(Infinite,"true-being")',
      'assert(elevationAbove(Infinite,"Restriction"))'
    ],
    predicates: [{ name: 'NegNegDef', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-62-spirit-home',
    chunkId: 'infinity-6d-spirit-at-home',
    label: 'Spirit at home in the infinite (universality, freedom)',
    clauses: [
      'tag(Spirit,"at-home-in-infinite")',
      'assert(signifies(Infinite,["Universality","Freedom"]))'
    ],
    predicates: [{ name: 'SpiritHome', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-63-finite-transcend-become-infinite',
    chunkId: 'infinity-6e-finite-nature-to-transcend',
    label: 'Finite’s nature: negate negation, transcend restriction, become infinite',
    clauses: [
      'assert(natureToTranscend(Finite,true))',
      'assert(negatesNegation(Finite,true))',
      'assert(becomes(Finite,Infinite))'
    ],
    predicates: [{ name: 'FiniteBecomesInfinite', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-64-not-external-nor-merely-subjective',
    chunkId: 'infinity-6f-not-external-not-merely-subjective',
    label: 'Infinite not a fixed beyond nor only subjective elevation',
    clauses: [
      'assert(notExternalBeyond(Infinite,Finite))',
      'assert(notMerelySubjectiveElevation(Infinite,true))'
    ],
    predicates: [{ name: 'NotExternal', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-65-self-elevation-via-restriction-ought',
    chunkId: 'infinity-6g-finite-self-elevation-via-restriction-ought',
    label: 'Finite refers to restriction/ought and sublates it (self-elevation)',
    clauses: [
      'assert(refersTo(Finite,["Restriction","Ought"]))',
      'assert(sublates(Finite,"Restriction"))',
      'assert(sublates(Finite,"Ought"))'
    ],
    predicates: [{ name: 'SelfElevation', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-66-infinity-as-vocation',
    chunkId: 'infinity-6h-infinity-as-vocation',
    label: 'Infinity is the finite’s affirmative determination (vocation)',
    clauses: [
      'assert(affirmativeDeterminationOf(Infinite,Finite))',
      'tag(Infinite,"vocation-of-finite")'
    ],
    predicates: [{ name: 'Vocation', args: [] }],
    relations: []
  },
  {
    id: 'infinity-op-67-only-infinite-is',
    chunkId: 'infinity-6i-only-infinite-is',
    label: 'Finite vanishes into infinite; only the infinite is',
    clauses: [
      'assert(vanishesInto(Finite,Infinite))',
      'tag(Result,"only-infinite-is")'
    ],
    predicates: [{ name: 'OnlyInfiniteIs', args: [] }],
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
