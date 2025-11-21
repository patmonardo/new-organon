// Re-export types for convenience
import type { Chunk, LogicalOperation } from '../../../types';
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../../types';

// Aggregate chunks and operations from all modules
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './specific-quantity';
import { CANONICAL_CHUNKS as SPECIFIC_QUANTUM_CHUNKS, LOGICAL_OPERATIONS as SPECIFIC_QUANTUM_OPS } from './specific-quantum';
import { CANONICAL_CHUNKS as SPECIFYING_MEASURE_CHUNKS, LOGICAL_OPERATIONS as SPECIFYING_MEASURE_OPS } from './specifying-measure';
import { CANONICAL_CHUNKS as BEING_FOR_ITSELF_CHUNKS, LOGICAL_OPERATIONS as BEING_FOR_ITSELF_OPS } from './being-for-itself';

export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...SPECIFIC_QUANTUM_CHUNKS,
  ...SPECIFYING_MEASURE_CHUNKS,
  ...BEING_FOR_ITSELF_CHUNKS,
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...SPECIFIC_QUANTUM_OPS,
  ...SPECIFYING_MEASURE_OPS,
  ...BEING_FOR_ITSELF_OPS,
];

// Re-export individual modules for granular access
export { CANONICAL_CHUNKS as SPECIFIC_QUANTUM_CHUNKS, LOGICAL_OPERATIONS as SPECIFIC_QUANTUM_OPS } from './specific-quantum';
export { CANONICAL_CHUNKS as SPECIFYING_MEASURE_CHUNKS, LOGICAL_OPERATIONS as SPECIFYING_MEASURE_OPS } from './specifying-measure';
export { CANONICAL_CHUNKS as BEING_FOR_ITSELF_CHUNKS, LOGICAL_OPERATIONS as BEING_FOR_ITSELF_OPS } from './being-for-itself';

