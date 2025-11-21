// Ground module: re-export types and aggregate module data
import type { Chunk, LogicalOperation } from '../../../../types'

// Re-export types for convenience
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../../../types'

// Aggregate chunks and operations from consolidated modules
import { CANONICAL_CHUNKS as ABSOLUTE_CHUNKS, LOGICAL_OPERATIONS as ABSOLUTE_OPS } from './ground_absolute';
import { CANONICAL_CHUNKS as DETERMINATE_CHUNKS, LOGICAL_OPERATIONS as DETERMINATE_OPS } from './ground_determinate';
import { CANONICAL_CHUNKS as CONDITION_CHUNKS, LOGICAL_OPERATIONS as CONDITION_OPS } from './ground_condition';

export const CANONICAL_CHUNKS: Chunk[] = [
  ...ABSOLUTE_CHUNKS,
  ...DETERMINATE_CHUNKS,
  ...CONDITION_CHUNKS
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...ABSOLUTE_OPS,
  ...DETERMINATE_OPS,
  ...CONDITION_OPS
];

/* minimal, stable accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getChunkById(id: string): Chunk | null {
  return CANONICAL_CHUNKS.find((c) => c.id === id) ?? null;
}

export function getLogicalOpsForChunkId(chunkId: string): LogicalOperation[] {
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunkId);
}

export function getAllChunks(): Chunk[] {
  return CANONICAL_CHUNKS;
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS;
}
