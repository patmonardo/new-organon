/**
 * Logical Operations: Nothing
 *
 * Pure Nothing - the second moment of the Logic, the CPU input.
 * Nothing is simple equality with itself, complete emptiness.
 *
 * Dialectical Movement:
 * - Pure nothingness: simple equality and complete emptiness
 * - Nothing in intuiting and thinking: same as pure being
 */

import type { LogicalOperation } from '../../../../types';

// ============================================================================
// NOTHING
// ============================================================================

export const nothingOp1PureNothingness: LogicalOperation = {
  id: 'nothing-op-1-pure-nothingness',
  chunkId: 'nothing-1',
  label: 'Pure nothingness — simple equality and complete emptiness',
  clauses: [
    'nothing = pureNothingness',
    'it = isSimpleEquality',
    'it = withItself',
    'it = completeEmptiness',
    'it = completeAbsence',
    'it = ofDeterminationAndContent',
    'it = lackOfAllDistinction',
    'it = within',
  ],
  predicates: [
    { name: 'pureNothingness', args: ['nothing'] },
    { name: 'completeEmptiness', args: ['nothing'] },
  ],
  relations: [
    { predicate: 'is', from: 'nothing', to: 'pureNothingness' },
    { predicate: 'is', from: 'nothing', to: 'completeEmptiness' },
  ],
  candidateSummary: 'Nothing, pure nothingness; it is simple equality with itself, complete emptiness, complete absence of determination and content; lack of all distinction within.',
  provenance: {
    sourceChunk: 'nothing-1',
    sourceOp: 'nothing-op-1-pure-nothingness',
  },
};

export const nothingOp2NothingInIntuitingThinking: LogicalOperation = {
  id: 'nothing-op-2-nothing-in-intuiting-thinking',
  chunkId: 'nothing-2',
  label: 'Nothing in intuiting and thinking — same as pure being',
  clauses: [
    'inSoFar = asMentionCanBeMade',
    'inSoFar = hereOfIntuitingAndThinking',
    'it = makesADifference',
    'it = whetherSomethingOrNothing',
    'it = isBeingIntuitedOrThought',
    'toIntuit = orToThinkNothing',
    'toIntuit = hasThereforeAMeaning',
    'theTwo = areDistinguished',
    'theTwo = andSoNothing',
    'theTwo = isConcretelyExists',
    'theTwo = inOurIntuitingOrThinking',
    'orRather = itIsTheEmptyIntuiting',
    'orRather = andThinkingItself',
    'orRather = likePureBeing',
    'nothing = isThereforeTheSameDetermination',
    'nothing = orRatherAbsenceOfDetermination',
    'nothing = andThusAltogetherTheSame',
    'nothing = asWhatPureBeingIs',
  ],
  predicates: [
    { name: 'sameAsPureBeing', args: ['nothing'] },
  ],
  relations: [
    { predicate: 'is', from: 'nothing', to: 'sameAsPureBeing' },
  ],
  candidateSummary: 'In so far as mention can be made here of intuiting and thinking, it makes a difference whether something or nothing is being intuited or thought. To intuit or to think nothing has therefore a meaning; the two are distinguished and so nothing is (concretely exists) in our intuiting or thinking; or rather it is the empty intuiting and thinking itself, like pure being. Nothing is therefore the same determination or rather absence of determination, and thus altogether the same as what pure being is.',
  provenance: {
    sourceChunk: 'nothing-2',
    sourceOp: 'nothing-op-2-nothing-in-intuiting-thinking',
  },
};

export const nothingOperations: LogicalOperation[] = [
  nothingOp1PureNothingness,
  nothingOp2NothingInIntuitingThinking,
];

