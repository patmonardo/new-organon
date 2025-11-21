// Being-for-Self module: re-export types and module data
import type { Chunk, LogicalOperation } from '../../../types'
import {
  CANONICAL_CHUNKS as BFS_CHUNKS,
  LOGICAL_OPERATIONS as BFS_HLOS
} from './being-for-self'

// Re-export types for convenience
export type { Chunk, LogicalOperation } from '../../../types'

// Re-export module data
export const CANONICAL_CHUNKS = BFS_CHUNKS
export const LOGICAL_OPERATIONS = BFS_HLOS

// Re-export accessors from being-for-self.ts if they exist
// Note: Only export what actually exists in being-for-self.ts
