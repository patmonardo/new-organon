import type { Chunk, LogicalOperation } from '../../../types';

// Import introduction
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './actuality';

// Import species modules
import { CANONICAL_CHUNKS as CONTINGENCY_CHUNKS, LOGICAL_OPERATIONS as CONTINGENCY_OPS } from './contingency';
import { CANONICAL_CHUNKS as RELATIVE_NECESSITY_CHUNKS, LOGICAL_OPERATIONS as RELATIVE_NECESSITY_OPS } from './relative-necessity';
import { CANONICAL_CHUNKS as ABSOLUTE_NECESSITY_CHUNKS, LOGICAL_OPERATIONS as ABSOLUTE_NECESSITY_OPS } from './absolute-necessity';

// ============================================================================
// AGGREGATED EXPORTS
// ============================================================================

/**
 * All canonical chunks from Chapter 2: Actuality
 * 
 * Structure:
 * - Introduction (actuality.ts)
 * - A. Contingency (contingency.ts)
 * - B. Relative Necessity (relative-necessity.ts)
 * - C. Absolute Necessity (absolute-necessity.ts)
 */
export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...CONTINGENCY_CHUNKS,
  ...RELATIVE_NECESSITY_CHUNKS,
  ...ABSOLUTE_NECESSITY_CHUNKS,
];

/**
 * All logical operations from Chapter 2: Actuality
 * 
 * Structure:
 * - Introduction operations (actuality.ts)
 * - A. Contingency operations (contingency.ts)
 * - B. Relative Necessity operations (relative-necessity.ts)
 * - C. Absolute Necessity operations (absolute-necessity.ts)
 */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...CONTINGENCY_OPS,
  ...RELATIVE_NECESSITY_OPS,
  ...ABSOLUTE_NECESSITY_OPS,
];

// ============================================================================
// RE-EXPORTS FOR GRANULAR ACCESS
// ============================================================================

// Re-export introduction
export { CANONICAL_CHUNKS as INTRODUCTION_CHUNKS, LOGICAL_OPERATIONS as INTRODUCTION_OPS } from './actuality';

// Re-export species modules
export { CANONICAL_CHUNKS as CONTINGENCY_CHUNKS, LOGICAL_OPERATIONS as CONTINGENCY_OPS } from './contingency';
export { CANONICAL_CHUNKS as RELATIVE_NECESSITY_CHUNKS, LOGICAL_OPERATIONS as RELATIVE_NECESSITY_OPS } from './relative-necessity';
export { CANONICAL_CHUNKS as ABSOLUTE_NECESSITY_CHUNKS, LOGICAL_OPERATIONS as ABSOLUTE_NECESSITY_OPS } from './absolute-necessity';

