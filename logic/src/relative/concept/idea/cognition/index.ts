import type { Chunk, LogicalOperation } from '../../../types';

// Import introduction
import { CANONICAL_CHUNKS as INTRO_CHUNKS, LOGICAL_OPERATIONS as INTRO_OPS } from './cognition';

// Import species modules
import { CANONICAL_CHUNKS as IDEA_TRUE_CHUNKS, LOGICAL_OPERATIONS as IDEA_TRUE_OPS } from './idea-true';
import { CANONICAL_CHUNKS as IDEA_GOOD_CHUNKS, LOGICAL_OPERATIONS as IDEA_GOOD_OPS } from './idea-good';

// ============================================================================
// AGGREGATED EXPORTS
// ============================================================================

/**
 * All canonical chunks from Chapter 2: The idea of cognition
 * 
 * Structure:
 * - Introduction (cognition.ts)
 * - A. The Idea of the True (idea-true.ts)
 * - B. The Idea of the Good (idea-good.ts)
 */
export const CANONICAL_CHUNKS: Chunk[] = [
  ...INTRO_CHUNKS,
  ...IDEA_TRUE_CHUNKS,
  ...IDEA_GOOD_CHUNKS,
];

/**
 * All logical operations from Chapter 2: The idea of cognition
 * 
 * Structure:
 * - Introduction operations (cognition.ts)
 * - A. The Idea of the True operations (idea-true.ts)
 * - B. The Idea of the Good operations (idea-good.ts)
 */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...INTRO_OPS,
  ...IDEA_TRUE_OPS,
  ...IDEA_GOOD_OPS,
];

