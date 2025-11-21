// Judgment module: re-export types and module data
import type { Chunk, LogicalOperation } from '../../../../types'

// Re-export types for convenience
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../../../types'

export const CANONICAL_CHUNKS: Chunk[] = []
export const LOGICAL_OPERATIONS: LogicalOperation[] = []

/* minimal, stable accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}

export function getChunkById(id: string): Chunk | null {
  return CANONICAL_CHUNKS.find((c) => c.id === id) ?? null
}

export function getLogicalOpsForChunkId(chunkId: string): LogicalOperation[] {
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunkId)
}

export function getAllChunks(): Chunk[] {
  return CANONICAL_CHUNKS
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS
}
