/*
  KG SCHEMA: Knowledge Graph Structure Definitions

  This file defines the core schema/structure of the Knowledge Graph:
  - Chunk: Text segments with metadata
  - LogicalOperation: Structured logical operations (HLOs)
  - Predicate, Relation: Components of Logical Operations
  - TopicMap types: Source Analysis planning structure

  NOTE: This is the KG schema, not just "types" in the TypeScript sense.
  The actual KG data structures are defined here.

  For Source Analysis tools and utilities, see: tools/source-analysis/

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

// Action trait for executable logical operations (microps)
export interface Action {
  type: string; // e.g., 'morph.create', 'empowerment.grant'
  payload: Record<string, unknown>;
  conditions?: string[]; // optional preconditions
}

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
  clauses?: string[]
  predicates?: Predicate[]
  relations?: Relation[]
  candidateSummary?: string
  provenance?: Provenance
  evidence?: unknown[]
  action?: Action
  [k: string]: unknown
}

// Re-export TopicMap types for convenience
export type {
  TopicMap,
  TopicMapEntry,
  TopicMapStatus,
} from '@schema/topic';

export {
  createTopicMap,
  createTopicMapEntry,
} from '@schema/topic';

