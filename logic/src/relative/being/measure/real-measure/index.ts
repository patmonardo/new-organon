// Re-export types for convenience
import type { Chunk, LogicalOperation } from '../../../types';
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../../types';

// Aggregate chunks and operations from all modules
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './real-measure';
import { CANONICAL_CHUNKS as RELATION_INDEPENDENT_CHUNKS, LOGICAL_OPERATIONS as RELATION_INDEPENDENT_OPS } from './relation-independent-measures';
import { CANONICAL_CHUNKS as NODAL_LINES_CHUNKS, LOGICAL_OPERATIONS as NODAL_LINES_OPS } from './nodal-lines';
import { CANONICAL_CHUNKS as MEASURELESS_CHUNKS, LOGICAL_OPERATIONS as MEASURELESS_OPS } from './measureless';

export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...RELATION_INDEPENDENT_CHUNKS,
  ...NODAL_LINES_CHUNKS,
  ...MEASURELESS_CHUNKS,
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...RELATION_INDEPENDENT_OPS,
  ...NODAL_LINES_OPS,
  ...MEASURELESS_OPS,
];

// Re-export individual modules for granular access
export { CANONICAL_CHUNKS as RELATION_INDEPENDENT_CHUNKS, LOGICAL_OPERATIONS as RELATION_INDEPENDENT_OPS } from './relation-independent-measures';
export { CANONICAL_CHUNKS as NODAL_LINES_CHUNKS, LOGICAL_OPERATIONS as NODAL_LINES_OPS } from './nodal-lines';
export { CANONICAL_CHUNKS as MEASURELESS_CHUNKS, LOGICAL_OPERATIONS as MEASURELESS_OPS } from './measureless';

