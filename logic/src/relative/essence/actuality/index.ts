// Re-export types for convenience
import type { Chunk, LogicalOperation } from '../../types';
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../types';

// Aggregate chunks and operations from all modules
import { CANONICAL_CHUNKS as ACTUALITY_CHUNKS, LOGICAL_OPERATIONS as ACTUALITY_OPS } from './actuality';
import { CANONICAL_CHUNKS as ABSOLUTE_INTRO_CHUNKS, LOGICAL_OPERATIONS as ABSOLUTE_INTRO_OPS } from './absolute';
import { CANONICAL_CHUNKS as EXPOSITION_CHUNKS, LOGICAL_OPERATIONS as EXPOSITION_OPS } from './exposition';
import { CANONICAL_CHUNKS as ATTRIBUTE_CHUNKS, LOGICAL_OPERATIONS as ATTRIBUTE_OPS } from './attribute';
import { CANONICAL_CHUNKS as MODE_CHUNKS, LOGICAL_OPERATIONS as MODE_OPS } from './mode';

export const CANONICAL_CHUNKS: Chunk[] = [
  ...ACTUALITY_CHUNKS,
  ...ABSOLUTE_INTRO_CHUNKS,
  ...EXPOSITION_CHUNKS,
  ...ATTRIBUTE_CHUNKS,
  ...MODE_CHUNKS,
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...ACTUALITY_OPS,
  ...ABSOLUTE_INTRO_OPS,
  ...EXPOSITION_OPS,
  ...ATTRIBUTE_OPS,
  ...MODE_OPS,
];

// Re-export individual modules for granular access
export { CANONICAL_CHUNKS as ACTUALITY_CHUNKS, LOGICAL_OPERATIONS as ACTUALITY_OPS } from './actuality';
export { CANONICAL_CHUNKS as ABSOLUTE_CHUNKS, LOGICAL_OPERATIONS as ABSOLUTE_OPS } from './absolute';
export { CANONICAL_CHUNKS as EXPOSITION_CHUNKS, LOGICAL_OPERATIONS as EXPOSITION_OPS } from './exposition';
export { CANONICAL_CHUNKS as ATTRIBUTE_CHUNKS, LOGICAL_OPERATIONS as ATTRIBUTE_OPS } from './attribute';
export { CANONICAL_CHUNKS as MODE_CHUNKS, LOGICAL_OPERATIONS as MODE_OPS } from './mode';

