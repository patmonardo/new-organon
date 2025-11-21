import type { Chunk, LogicalOperation } from '../../../types';

// Import introduction
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './speculation';

// Import method sections
import { CANONICAL_CHUNKS as METHOD_BEGINNING_CHUNKS, LOGICAL_OPERATIONS as METHOD_BEGINNING_OPS } from './method-beginning';
import { CANONICAL_CHUNKS as METHOD_ADVANCE_CHUNKS, LOGICAL_OPERATIONS as METHOD_ADVANCE_OPS } from './method-advance';

// ============================================================================
// AGGREGATED EXPORTS
// ============================================================================

/**
 * All canonical chunks from Chapter 3: The absolute idea
 * 
 * Structure:
 * - Introduction (speculation.ts)
 * - 1. The Beginning (method-beginning.ts)
 * - 2. The Advance (method-advance.ts)
 */
export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...METHOD_BEGINNING_CHUNKS,
  ...METHOD_ADVANCE_CHUNKS,
];

/**
 * All logical operations from Chapter 3: The absolute idea
 * 
 * Structure:
 * - Introduction operations (speculation.ts)
 * - 1. The Beginning operations (method-beginning.ts)
 * - 2. The Advance operations (method-advance.ts)
 */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...METHOD_BEGINNING_OPS,
  ...METHOD_ADVANCE_OPS,
];

