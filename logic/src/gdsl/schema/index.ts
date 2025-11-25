/**
 * GDSL Schema: Dialectical Schema Language
 * 
 * Extends schema/dialectic.ts with OpenCypher AST integration
 * and Reflective System of Marks support.
 * 
 * This is the "syntax-free" dialectical schema that AI codegens
 * directly in IR—no parsing needed.
 */

export * from '../../schema/dialectic';

// Re-export dialectic types for GDSL
export type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
  CpuGpuPhase,
} from '../../schema/dialectic';

/**
 * GDSL extends dialectic schema with:
 * - Cypher pattern mapping
 * - Reflective marks
 * - PropertyState rights
 * - Daemon protocol
 */

