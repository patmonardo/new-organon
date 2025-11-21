/*
  Shared Types for Logic Modules
  
  Single source of truth for Chunk, LogicalOperation, and related types.
  All modules should import from here: `import type { Chunk, LogicalOperation } from '../types'`
*/

// Provenance/evidence metadata (optional, for IR/extraction workflows)
export interface Provenance {
  sourceChunk?: string
  sourceOp?: string
  extractor?: string
  ts?: string | number
  deps?: string[]
  evidenceIds?: string[]
}

// Predicate for logical operations
export type Predicate = { name: string; args?: string[] }

// Relation for logical operations
export type Relation = { predicate: string; from: string; to: string }

// Canonical text chunk
export type Chunk = {
  id: string
  title?: string
  text: string
  summary?: string
  [k: string]: unknown
}

// Logical operation (HLO)
export type LogicalOperation = {
  id: string
  chunkId?: string
  label?: string
  clauses: string[]
  predicates?: Predicate[]
  relations?: Relation[]
  candidateSummary?: string
  provenance?: Provenance
  evidence?: unknown[]
  [k: string]: unknown
}

