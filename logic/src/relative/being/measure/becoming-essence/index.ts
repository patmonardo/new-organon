// Re-export types for convenience
import type { Chunk, LogicalOperation } from '../../../types';
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../../types';

// Aggregate chunks and operations from all modules
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './becoming-essence';
import { CANONICAL_CHUNKS as ABSOLUTE_INDIFFERENCE_CHUNKS, LOGICAL_OPERATIONS as ABSOLUTE_INDIFFERENCE_OPS } from './absolute-indifference';
import { CANONICAL_CHUNKS as INDIFFERENCE_INVERSE_RATIO_CHUNKS, LOGICAL_OPERATIONS as INDIFFERENCE_INVERSE_RATIO_OPS } from './indifference-inverse-ratio';
import { CANONICAL_CHUNKS as TRANSITION_ESSENCE_CHUNKS, LOGICAL_OPERATIONS as TRANSITION_ESSENCE_OPS } from './transition-essence';

export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...ABSOLUTE_INDIFFERENCE_CHUNKS,
  ...INDIFFERENCE_INVERSE_RATIO_CHUNKS,
  ...TRANSITION_ESSENCE_CHUNKS,
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...ABSOLUTE_INDIFFERENCE_OPS,
  ...INDIFFERENCE_INVERSE_RATIO_OPS,
  ...TRANSITION_ESSENCE_OPS,
];

// Re-export individual modules for granular access
export { CANONICAL_CHUNKS as ABSOLUTE_INDIFFERENCE_CHUNKS, LOGICAL_OPERATIONS as ABSOLUTE_INDIFFERENCE_OPS } from './absolute-indifference';
export { CANONICAL_CHUNKS as INDIFFERENCE_INVERSE_RATIO_CHUNKS, LOGICAL_OPERATIONS as INDIFFERENCE_INVERSE_RATIO_OPS } from './indifference-inverse-ratio';
export { CANONICAL_CHUNKS as TRANSITION_ESSENCE_CHUNKS, LOGICAL_OPERATIONS as TRANSITION_ESSENCE_OPS } from './transition-essence';

