// World module: re-export types and aggregate module data
import type { Chunk, LogicalOperation } from '../../../types'

// Re-export types for convenience
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../../types'

// Aggregate chunks and operations from world modules
import { CANONICAL_CHUNKS as LAW_CHUNKS, LOGICAL_OPERATIONS as LAW_OPS } from './law';
import { CANONICAL_CHUNKS as WORLD_CHUNKS, LOGICAL_OPERATIONS as WORLD_OPS } from './world';
import { CANONICAL_CHUNKS as DISAPPEARANCE_CHUNKS, LOGICAL_OPERATIONS as DISAPPEARANCE_OPS } from './disappearance';

export const CANONICAL_CHUNKS: Chunk[] = [
  ...LAW_CHUNKS,
  ...WORLD_CHUNKS,
  ...DISAPPEARANCE_CHUNKS
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...LAW_OPS,
  ...WORLD_OPS,
  ...DISAPPEARANCE_OPS
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

