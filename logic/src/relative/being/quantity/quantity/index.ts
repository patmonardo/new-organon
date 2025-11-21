// Quantity module: re-export types and module data
import type { Chunk, LogicalOperation } from '../../../types'
import {
  CANONICAL_CHUNKS as QUANTITY_CHUNKS,
  LOGICAL_OPERATIONS as QUANTITY_OPS
} from './quantity'

// Re-export types for convenience
export type { Chunk, LogicalOperation } from '../../../types'

// Re-export module data
export const CANONICAL_CHUNKS = QUANTITY_CHUNKS
export const LOGICAL_OPERATIONS = QUANTITY_OPS

// Re-export accessors from quantity.ts
export {
  getChunk,
  getChunkById,
  getLogicalOpsForChunkId,
  getAllChunks,
  getLogicalOperations
} from './quantity'
