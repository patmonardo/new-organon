import type { Chunk, LogicalOperation } from '../../../types';

// Import introduction
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './substance';

// Import species modules
import { CANONICAL_CHUNKS as SUBSTANTIALITY_CHUNKS, LOGICAL_OPERATIONS as SUBSTANTIALITY_OPS } from './relation-substantiality';
import { CANONICAL_CHUNKS as CAUSALITY_CHUNKS, LOGICAL_OPERATIONS as CAUSALITY_OPS } from './relation-causality';
import { CANONICAL_CHUNKS as RECIPROCITY_CHUNKS, LOGICAL_OPERATIONS as RECIPROCITY_OPS } from './reciprocity-action';

// ============================================================================
// AGGREGATED EXPORTS
// ============================================================================

/**
 * All canonical chunks from Chapter 3: The absolute relation
 * 
 * Structure:
 * - Introduction (substance.ts)
 * - A. Relation of Substantiality (relation-substantiality.ts)
 * - B. Relation of Causality (relation-causality.ts)
 * - C. Reciprocity of Action (reciprocity-action.ts)
 */
export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...SUBSTANTIALITY_CHUNKS,
  ...CAUSALITY_CHUNKS,
  ...RECIPROCITY_CHUNKS,
];

/**
 * All logical operations from Chapter 3: The absolute relation
 * 
 * Structure:
 * - Introduction operations (substance.ts)
 * - A. Relation of Substantiality operations (relation-substantiality.ts)
 * - B. Relation of Causality operations (relation-causality.ts)
 * - C. Reciprocity of Action operations (reciprocity-action.ts)
 */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...SUBSTANTIALITY_OPS,
  ...CAUSALITY_OPS,
  ...RECIPROCITY_OPS,
];

// ============================================================================
// RE-EXPORTS FOR GRANULAR ACCESS
// ============================================================================

// Re-export introduction
export { CANONICAL_CHUNKS as INTRODUCTION_CHUNKS, LOGICAL_OPERATIONS as INTRODUCTION_OPS } from './substance';

// Re-export species modules
export { CANONICAL_CHUNKS as SUBSTANTIALITY_CHUNKS, LOGICAL_OPERATIONS as SUBSTANTIALITY_OPS } from './relation-substantiality';
export { CANONICAL_CHUNKS as CAUSALITY_CHUNKS, LOGICAL_OPERATIONS as CAUSALITY_OPS } from './relation-causality';
export { CANONICAL_CHUNKS as RECIPROCITY_CHUNKS, LOGICAL_OPERATIONS as RECIPROCITY_OPS } from './reciprocity-action';

