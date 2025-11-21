// Essence module: re-export types and aggregate module data
import type { Chunk, LogicalOperation } from '../../../../types'
import {
  CANONICAL_CHUNKS as SHINE_CHUNKS,
  LOGICAL_OPERATIONS as SHINE_OPS
} from './shine'
import {
  CANONICAL_CHUNKS as REFLECTION_CHUNKS,
  LOGICAL_OPERATIONS as REFLECTION_OPS
} from './reflection'

// Re-export types for convenience
export type { Chunk, LogicalOperation } from '../../../../types'

// Central registry
export const CANONICAL_CHUNKS: Chunk[] = [
  ...SHINE_CHUNKS,
  ...REFLECTION_CHUNKS
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...SHINE_OPS,
  ...REFLECTION_OPS
]

// Re-export accessors (if they exist in any of the modules)
// Note: Only export what actually exists in the modules
