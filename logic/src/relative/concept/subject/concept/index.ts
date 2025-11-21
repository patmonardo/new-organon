// Concept module: re-export types and aggregate module data
import type { Chunk, LogicalOperation } from '../../../../types'
import {
  CANONICAL_CHUNKS as UNIVERSAL_CHUNKS,
  LOGICAL_OPERATIONS as UNIVERSAL_OPS,
} from './concept_universal'
import {
  CANONICAL_CHUNKS as PARTICULAR_CHUNKS,
  LOGICAL_OPERATIONS as PARTICULAR_OPS,
} from './concept_particular'
import {
  CANONICAL_CHUNKS as SINGULAR_CHUNKS,
  LOGICAL_OPERATIONS as SINGULAR_OPS,
} from './concept_singular'

// Re-export types for convenience
export type { Chunk, LogicalOperation } from '../../../../types'

// If you add an intro/pure-concept module later, append here:
// import { CANONICAL_CHUNKS as PURE_CONCEPT_CHUNKS, LOGICAL_OPERATIONS as PURE_CONCEPT_OPS } from './concept_pure'

// Central registry
export const CANONICAL_CHUNKS: Chunk[] = [
  // ...PURE_CONCEPT_CHUNKS, // uncomment when available
  ...UNIVERSAL_CHUNKS,
  ...PARTICULAR_CHUNKS,
  ...SINGULAR_CHUNKS,
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ...PURE_CONCEPT_OPS, // uncomment when available
  ...UNIVERSAL_OPS,
  ...PARTICULAR_OPS,
  ...SINGULAR_OPS,
]

// Re-export accessors (if they exist in any of the modules)
// Note: Only export what actually exists in the modules
