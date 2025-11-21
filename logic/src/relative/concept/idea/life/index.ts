import type { Chunk, LogicalOperation } from '../../../types';

// Import introduction
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './life';

// Import species modules
import { CANONICAL_CHUNKS as LIVING_INDIVIDUAL_CHUNKS, LOGICAL_OPERATIONS as LIVING_INDIVIDUAL_OPS } from './living-individual';
import { CANONICAL_CHUNKS as LIFE_PROCESS_CHUNKS, LOGICAL_OPERATIONS as LIFE_PROCESS_OPS } from './life-process';
import { CANONICAL_CHUNKS as GENUS_CHUNKS, LOGICAL_OPERATIONS as GENUS_OPS } from './genus';

// ============================================================================
// AGGREGATED EXPORTS
// ============================================================================

/**
 * All canonical chunks from Chapter 1: Life
 * 
 * Structure:
 * - Introduction (life.ts)
 * - A. The Living Individual (living-individual.ts)
 * - B. The Life-Process (life-process.ts)
 * - C. The Genus (genus.ts)
 */
export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...LIVING_INDIVIDUAL_CHUNKS,
  ...LIFE_PROCESS_CHUNKS,
  ...GENUS_CHUNKS,
];

/**
 * All logical operations from Chapter 1: Life
 * 
 * Structure:
 * - Introduction operations (life.ts)
 * - A. The Living Individual operations (living-individual.ts)
 * - B. The Life-Process operations (life-process.ts)
 * - C. The Genus operations (genus.ts)
 */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...LIVING_INDIVIDUAL_OPS,
  ...LIFE_PROCESS_OPS,
  ...GENUS_OPS,
];

