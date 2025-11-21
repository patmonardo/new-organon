/*
  FOUNDATION — The Determinations of Reflection

  This module aggregates the three Determinations of Reflection:
  - A. Identity: Essence as simple self-reference, pure identity
  - B. Difference: The hard machinery of reflection (absolute difference, diversity, opposition)
  - C. Contradiction: Self-exclusion resolving to ground

  REFLECTION AS ESSENCE → FOUNDATION → GROUND:
  Reflection is the shining of essence within itself. The Determinations of Reflection
  (Identity, Difference, Contradiction) form the Foundation — the systematic
  self-determination of essence through its own internal movement. This Foundation
  resolves to Ground, completing the structure: Essence → Foundation → Ground.

  THE LOGICALLY PROFOUND TRANSITION: IDENTITY-DIFFERENCE → FORM-MATTER
  This is the key transition from Foundation to Ground — one of the most profound
  logical moves in the System of Reflection:
  - Identity (Foundation) → Form (Ground)
  - Difference (Foundation) → Matter (Ground)

  This transition shows how the Determinations of Reflection (Identity, Difference)
  become the Categories of Ground (Form, Matter). It is not surprising that ML
  researchers struggle to codify "Reflection" — this structure is the systematic
  foundation that makes reflection explicit and computable.

  PHILOSOPHICAL STRUCTURE:
  - Essence: The Essential and the Unessential
  - Foundation: The Determinations of Reflection (Identity, Difference, Contradiction)
    → Identity-Difference resolves to Form-Matter in Ground
  - Ground: The resolved contradiction (Absolute Ground: Form/Matter/Content,
    Determinate Ground, Condition)

  NEXT LEVEL: Inner/Outer (Reflection on Appearances/Aspects) → The Absolute
  Hegel's Inner/Outer is the key to the next level of Reflection — Reflection on
  Appearances/Aspects — which reaches into the Absolute. This completes the
  systematic structure of Reflection as the foundation of Science.

  NOTE ON KANT'S CATEGORIES OF REFLECTION:
  Kant includes "whole and parts" in his Categories of Reflection (along with
  Identity/Difference, Agreement/Opposition, Inner/Outer, Matter/Form). However,
  these categories become ambiguous when used transcendentally — they lack the
  systematic development that Hegel provides. Hegel's Determinations of Reflection
  (Identity, Difference, Contradiction) are systematically developed and resolve
  into Ground (Form, Matter, Content), avoiding the transcendental ambiguity that
  plagues Kant's approach. This systematic structure is what makes Reflection
  explicit and computable — something Kant's transcendental use cannot achieve.
*/

// Module imports
import {
  CANONICAL_CHUNKS as IDENTITY_CHUNKS,
  LOGICAL_OPERATIONS as IDENTITY_OPS
} from './identity'

import {
  CANONICAL_CHUNKS as DIFFERENCE_CHUNKS,
  LOGICAL_OPERATIONS as DIFFERENCE_OPS
} from './difference'

import {
  CANONICAL_CHUNKS as CONTRADICTION_CHUNKS,
  LOGICAL_OPERATIONS as CONTRADICTION_OPS
} from './contradiction'

// Foundation module: re-export types and aggregate module data
import type { Chunk, LogicalOperation } from '../../../../types'

// Re-export types for convenience
export type { Chunk, LogicalOperation, Predicate, Relation } from '../../../../types'

// Central registry: all Determinations of Reflection
export const CANONICAL_CHUNKS: Chunk[] = [
  ...IDENTITY_CHUNKS,
  ...DIFFERENCE_CHUNKS,
  ...CONTRADICTION_CHUNKS
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...IDENTITY_OPS,
  ...DIFFERENCE_OPS,
  ...CONTRADICTION_OPS
]

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
