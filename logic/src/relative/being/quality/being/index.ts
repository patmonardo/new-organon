// Being module: re-export types and module data
import type { Chunk, LogicalOperation } from '../../../types'
import {
  CANONICAL_CHUNKS as BEING_CHUNKS,
  LOGICAL_OPERATIONS as BEING_HLOS
} from './being'

// Re-export types for convenience
export type { Chunk, LogicalOperation } from '../../../types'

// Re-export module data
export const CANONICAL_CHUNKS = BEING_CHUNKS
export const LOGICAL_OPERATIONS = BEING_HLOS

// Re-export accessors from being.ts if they exist
// Note: Only export what actually exists in being.ts
