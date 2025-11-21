import type { Chunk, LogicalOperation } from './index'

/*
  Foundation — C. CONTRADICTION

  This module consolidates the complete Contradiction section:
  - Part 1: Contradiction as self-exclusion (positive/negative)
  - Part 2: Contradiction resolves itself (null, sublation, self-withdrawal)
  - Part 3: Return to ground (essence as ground, unity of positive/negative)

  STRUCTURE: Essence → Foundation → Ground
  - Essence: The Essential and the Unessential
  - Foundation: The Determinations of Reflection (Identity, Difference, Contradiction)
  - Ground: The resolved contradiction (Absolute Ground, Determinate Ground, Condition)

  PHILOSOPHICAL NOTES:

  1. **Contradiction as the Completion of Opposition**:
     Contradiction emerges when opposition's self-subsisting moments exclude their
     own self-subsistence. The positive and negative, each containing and excluding
     the other, become self-contradictory — negation turned back upon itself.

  2. **Contradiction Resolves Itself**:
     Contradiction is not static; it resolves itself through sublation. The self-excluding
     reflection causes opposites to translate into one another, producing first the null,
     then through sublation a positive self-unity — essence identical with itself through
     self-negation.

  3. **Return to Ground**:
     The resolved contradiction returns to ground — essence as unity of positive and negative.
     Ground preserves and removes opposition simultaneously. This is the foundation that
     makes possible all further determinations.

  4. **Reflection as the Idea:Concept**:
     Contradiction completes the Determinations of Reflection. Through contradiction's
     resolution to ground, reflection shows itself as the idea:concept — the systematic
     self-determination of essence through its own internal movement.
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ctr-1-difference-moments',
    title: 'Difference contains its sides; self-subsisting moments',
    text: `Difference contains its moments (positive/negative). In opposition they become self-subsisting, each containing and excluding the other; this self-subsistence is the basis for contradiction.`
  },
  {
    id: 'ctr-2-self-exclusion-and-contradiction',
    title: 'Self-exclusion as contradiction',
    text: `A self-subsisting determination excludes its own self-subsistence by containing the other and simultaneously excluding it. That internal exclusion is contradiction: negation turned back upon itself.`
  },
  {
    id: 'ctr-3-positive-contradiction',
    title: 'Positive as implicit contradiction',
    text: `The positive posits self‑identity by excluding the negative; in positing itself it posits its other and so becomes implicitly contradictory — the positive is both identity and the positing of its negation.`
  },
  {
    id: 'ctr-4-negative-contradiction',
    title: 'Negative as posited contradiction',
    text: `The negative, as posited unlikeness, is the explicit contradiction: it is the non-being of another and in self-reference excludes itself, thus realizing contradiction as explicit negation.`
  },
  {
    id: 'ctr-5-contradiction-unity',
    title: 'Contradiction as unity of implicit and explicit negation',
    text: `Difference is implicitly contradictory; opposition makes contradiction explicit. Positive and negative are two faces of the same posited contradiction which demands mediation (sublation).`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ctr-op-1-identify-moments',
    chunkId: 'ctr-1-difference-moments',
    label: 'Identify self-subsisting moments that ground contradiction',
    clauses: [
      'difference.includes = {positive, negative}',
      'inOpposition: moments.selfSubsist = true',
      'selfSubsistence => containsAndExcludes(other)'
    ],
    predicates: [{ name: 'HasSelfSubsistingMoments', args: [] }],
    relations: [{ predicate: 'grounds', from: 'selfSubsistence', to: 'contradictionPotential' }]
  },
  {
    id: 'ctr-op-2-formalize-self-exclusion',
    chunkId: 'ctr-2-self-exclusion-and-contradiction',
    label: 'Formalize internal self-exclusion → contradiction',
    clauses: [
      'if side.contains(other) and side.excludes(other) then side.excludesSelf',
      'excludesSelf => contradiction'
    ],
    predicates: [{ name: 'IsSelfExcluding', args: [] }],
    relations: [{ predicate: 'produces', from: 'selfExclusion', to: 'contradiction' }]
  },
  {
    id: 'ctr-op-3-positive-implicit',
    chunkId: 'ctr-3-positive-contradiction',
    label: 'Encode positive as implicit contradiction (posited identity → other posited)',
    clauses: [
      'positive.posits(selfIdentity) => positive.posits(other)',
      'positsOther => implicitContradiction(positive)'
    ],
    predicates: [{ name: 'IsImplicitContradiction', args: ['positive'] }],
    relations: [{ predicate: 'implies', from: 'positivePositing', to: 'implicitContradiction' }]
  },
  {
    id: 'ctr-op-4-negative-explicit',
    chunkId: 'ctr-4-negative-contradiction',
    label: 'Encode negative as explicit contradiction (self‑reference of unlikeness)',
    clauses: [
      'negative = positedUnlikeness',
      'negative.selfReference => explicitContradiction',
      'explicitContradiction.requires = mediation'
    ],
    predicates: [{ name: 'IsExplicitContradiction', args: ['negative'] }],
    relations: [{ predicate: 'requires', from: 'explicitContradiction', to: 'mediationProcess' }]
  },
  {
    id: 'ctr-op-5-unify-and-trigger-sublation',
    chunkId: 'ctr-5-contradiction-unity',
    label: 'Treat contradiction as trigger for sublation/mediation',
    clauses: [
      'difference.implicitContradiction ∧ opposition.explicitContradiction => contradictionDetected',
      'on(contradictionDetected) => propose(sublationCandidate)'
    ],
    predicates: [{ name: 'TriggersSublation', args: [] }],
    relations: [{ predicate: 'proposes', from: 'contradictionNode', to: 'sublationCandidate' }]
  }
]

/* appended: Contradiction — part 2 (resolution, null, sublation, self-withdrawal, unity) */
CANONICAL_CHUNKS.push(
  {
    id: 'ctr-6-resolution-overview',
    title: 'Contradiction resolves itself — overview',
    text: `The self-excluding reflection causes opposites to translate into one another; this vanishing produces a first unity (the null). Contradiction contains both negation and positing, and through sublation the positedness founders and a higher self-unity emerges.`
  },
  {
    id: 'ctr-7-null-and-passing',
    title: 'Null: ceaseless passing of opposites',
    text: `The internal, ceaseless vanishing of opposites is the first unity — the null. Opposites pass into each other and nullify their isolated self-subsistence, revealing the transient character of posited moments.`
  },
  {
    id: 'ctr-8-positedness-founders',
    title: 'Positedness foundering in contradiction',
    text: `Posited self-subsistence (the sides taken as simply self-identical) founders in contradiction: because their self-identity is also a reference-to-other, their positedness collapses under self-negation.`
  },
  {
    id: 'ctr-9-excluding-reflection-sublates',
    title: 'Excluding reflection as sublation of positedness',
    text: `The excluding reflection both posits and sublates the positedness: by negating its positedness it turns back upon itself and effects a sublation that transforms posited immediacy into a self-withdrawn unity.`
  },
  {
    id: 'ctr-10-self-withdrawal-unity',
    title: 'Self-withdrawal and positive self-unity',
    text: `Sublation yields positive self-unity: self-subsistence becomes unity that turns back into itself by negating its own positedness. The unity is identity through self-negation — essence identical with itself.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'ctr-op-6-declare-resolution',
    chunkId: 'ctr-6-resolution-overview',
    label: 'Declare contradiction resolution and emergence of null',
    clauses: [
      'on(selfExclusion) => opposites.translateIntoEachOther',
      'translation.ceaseless -> nullUnity',
      'nullUnity = firstResultOfContradiction'
    ],
    predicates: [{ name: 'ProducesNullUnity', args: [] }],
    relations: [{ predicate: 'emergesFrom', from: 'nullUnity', to: 'selfExclusion' }]
  },
  {
    id: 'ctr-op-7-formalize-null',
    chunkId: 'ctr-7-null-and-passing',
    label: 'Formalize null as transient unity from passing',
    clauses: [
      'if opposites.passIntoEachOther then transientUnity = true',
      'transientUnity.name = null',
      'null.nullifies = isolatedSelfSubsistence'
    ],
    predicates: [{ name: 'IsNullUnity', args: [] }],
    relations: [{ predicate: 'nullifies', from: 'null', to: 'positedMoments' }]
  },
  {
    id: 'ctr-op-8-foundering-positedness',
    chunkId: 'ctr-8-positedness-founders',
    label: 'Encode positedness collapse under self-negation',
    clauses: [
      'positedness.selfIdentity ∧ referenceToOther => unstablePositedness',
      'unstablePositedness -> founderInContradiction'
    ],
    predicates: [{ name: 'PositednessFounders', args: [] }],
    relations: [{ predicate: 'resultsIn', from: 'unstablePositedness', to: 'foundering' }]
  },
  {
    id: 'ctr-op-9-sublation-process',
    chunkId: 'ctr-9-excluding-reflection-sublates',
    label: 'Model excluding reflection performing sublation (posits+negates)',
    clauses: [
      'excludingReflection.posits(and) && excludingReflection.negates(positedness)',
      'on(excludingReflection) => propose(sublationTransform)',
      'sublationTransform => preserveMoments + elevateUnity'
    ],
    predicates: [{ name: 'PerformsSublation', args: [] }],
    relations: [{ predicate: 'transforms', from: 'excludingReflection', to: 'sublationTransform' }]
  },
  {
    id: 'ctr-op-10-self-withdrawal-unity-formalize',
    chunkId: 'ctr-10-self-withdrawal-unity',
    label: 'Formalize self-withdrawal yielding positive self-unity',
    clauses: [
      'sublation => selfWithdrawal',
      'selfWithdrawal -> positiveSelfUnity',
      'positiveSelfUnity = identityThroughSelfNegation'
    ],
    predicates: [{ name: 'IsPositiveSelfUnity', args: [] }],
    relations: [{ predicate: 'yields', from: 'sublation', to: 'positiveSelfUnity' }]
  }
)

/* appended: Contradiction — part 3 (return to ground / essence; sublation -> ground; unity of pos/neg) */
CANONICAL_CHUNKS.push(
  {
    id: 'ctr-11-return-to-ground',
    title: 'Return to ground through foundering of opposition',
    text: `The self‑subsisting opposition, in foundering through its contradiction, returns to its ground: the excluding reflection reduces positedness to determinations that are only determinations, restoring essence as ground — a reflective unity withdrawn into itself.`
  },
  {
    id: 'ctr-12-essence-as-excluding-reflection',
    title: 'Essence as ground and excluding reflection',
    text: `Essence as ground is itself an excluding reflection: the opposition that was immediate is now posited and sublated, making essence a posited unity that nevertheless contains negation and preserves the self‑contradictory moments within its ground.`
  },
  {
    id: 'ctr-13-ground-unity-of-pos-neg',
    title: 'Ground = unity of positive and negative; preservation + removal',
    text: `The resolved contradiction is ground: opposition and contradiction are both removed and preserved. Ground is self‑identical in negation — the positive and negative are present as self‑subsistent determinations within a completed reflective unity.`
  },
  {
    id: 'ctr-14-sublation-completes-essence',
    title: 'Sublation completes essence and reflects self‑unity',
    text: `Through sublation each self‑subsisting opposite makes itself into its other and founders to ground, where it reunites with itself. In this foundering the essence appears as the first self‑reflected unity — identity through self‑negation.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'ctr-op-11-foundering-to-ground',
    chunkId: 'ctr-11-return-to-ground',
    label: 'Formalize foundering of opposition -> return to ground',
    clauses: [
      'on(foundering(opposition)) => reduce(positedness) -> determinationsAsDeterminations',
      'reduction => restore(essenceAsGround)'
    ],
    predicates: [{ name: 'ReturnsToGround', args: [] }],
    relations: [{ predicate: 'restores', from: 'foundering', to: 'essenceAsGround' }]
  },
  {
    id: 'ctr-op-12-essence-excluding-reflection',
    chunkId: 'ctr-12-essence-as-excluding-reflection',
    label: 'Encode essence as excluding reflection that preserves sublated moments',
    clauses: [
      'essence.asGround = excludingReflection',
      'essence.positedness.contains = {positive, negative} (sublated)',
      'essence.preserves && essence.removes = true'
    ],
    predicates: [{ name: 'IsEssenceAsGround', args: [] }],
    relations: [{ predicate: 'contains', from: 'essence', to: 'sublatedMoments' }]
  },
  {
    id: 'ctr-op-13-ground-unity-formalize',
    chunkId: 'ctr-13-ground-unity-of-pos-neg',
    label: 'Formalize ground as unity of positive & negative (preserve+remove)',
    clauses: [
      'ground = unity(positive,negative)',
      'ground.preserves(opposition) && ground.removes(opposition) = true',
      'ground.selfIdentity = identityThroughNegation'
    ],
    predicates: [{ name: 'IsGroundUnity', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'ground', to: 'identityThroughNegation' }]
  },
  {
    id: 'ctr-op-14-sublation-completion',
    chunkId: 'ctr-14-sublation-completes-essence',
    label: 'Model sublation completing essence: opposites foundering and reunification',
    clauses: [
      'on(sublation(opposition)) => eachSide.transformsInto(other) -> founderToGround',
      'founderToGround => reuniteWithSelf(inGround)',
      'result = essenceAsSelfReflectedUnity'
    ],
    predicates: [{ name: 'CompletesSublationToEssence', args: [] }],
    relations: [{ predicate: 'yields', from: 'sublation', to: 'essenceAsUnity' }]
  }
)
