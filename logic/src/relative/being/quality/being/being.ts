/**
 * Logical Operations: Being
 *
 * Pure Being - the first moment of the Logic, the CPU input.
 * Being is pure indeterminateness, emptiness, equal only to itself.
 *
 * Dialectical Movement:
 * - Pure being without determination: indeterminate immediacy
 * - Pure being as emptiness: nothing to intuit or think
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// BEING
// ============================================================================

export const beingOp1PureBeingWithoutDetermination: LogicalOperation = {
  id: 'being-op-1-pure-being-without-determination',
  chunkId: 'being-1',
  label: 'Pure being without determination — indeterminate immediacy',
  clauses: [
    'being = pureBeing',
    'being = withoutFurtherDetermination',
    'inIndeterminateImmediacy = equalOnlyToItself',
    'inIndeterminateImmediacy = alsoNotUnequal',
    'inIndeterminateImmediacy = withRespectToAnother',
    'has = noDifferenceWithinIt',
    'has = norAnyOutwardly',
    'if = anyDeterminationOrContent',
    'if = werePositedInIt',
    'if = asDistinct',
    'if = orIfItWerePosited',
    'if = byThisDeterminationOrContent',
    'if = asDistinctFromAnOther',
    'would = therebyFail',
    'would = toHoldFastToItsPurity',
    'it = isPureIndeterminateness',
  ],
  predicates: [
    { name: 'pureBeing', args: ['being'] },
    { name: 'indeterminateImmediacy', args: ['being'] },
  ],
  relations: [
    { predicate: 'is', from: 'being', to: 'pureBeing' },
    { predicate: 'is', from: 'being', to: 'indeterminateImmediacy' },
  ],
  candidateSummary: 'Being, pure being, without further determination. In its indeterminate immediacy it is equal only to itself and also not unequal with respect to another; it has no difference within it, nor any outwardly. If any determination or content were posited in it as distinct, or if it were posited by this determination or content as distinct from an other, it would thereby fail to hold fast to its purity. It is pure indeterminateness.',
  provenance: {
    sourceChunk: 'being-1',
    sourceOp: 'being-op-1-pure-being-without-determination',
  },
};

export const beingOp2PureBeingAsEmptiness: LogicalOperation = {
  id: 'being-op-2-pure-being-as-emptiness',
  chunkId: 'being-2',
  label: 'Pure being as emptiness — nothing to intuit or think',
  clauses: [
    'it = isPureIndeterminateness',
    'it = andEmptiness',
    'there = isNothingToBeIntuited',
    'there = inIt',
    'if = oneCanSpeakHere',
    'if = ofIntuiting',
    'or = itIsOnlyThis',
    'or = pureEmptyIntuitingItself',
    'justAsLittle = isAnythingToBeThought',
    'justAsLittle = inIt',
    'or = itIsEquallyOnlyThis',
    'or = emptyThinking',
    'being = theIndeterminateImmediate',
    'being = isInFactNothing',
    'being = andNeitherMoreNorLess',
    'being = thanNothing',
  ],
  predicates: [
    { name: 'emptiness', args: ['being'] },
  ],
  relations: [
    { predicate: 'is', from: 'being', to: 'emptiness' },
    { predicate: 'is', from: 'being', to: 'nothing' },
  ],
  candidateSummary: 'It is pure indeterminateness and emptiness. There is nothing to be intuited in it, if one can speak here of intuiting; or, it is only this pure empty intuiting itself. Just as little is anything to be thought in it, or, it is equally only this empty thinking. Being, the indeterminate immediate is in fact nothing, and neither more nor less than nothing.',
  provenance: {
    sourceChunk: 'being-2',
    sourceOp: 'being-op-2-pure-being-as-emptiness',
  },
};

export const beingOperations: LogicalOperation[] = [
  beingOp1PureBeingWithoutDetermination,
  beingOp2PureBeingAsEmptiness,
];

