import type { Chunk, LogicalOperation } from '../../../types';

/**
 * CHAPTER 3: THE BECOMING OF ESSENCE — Complete Structure
 * 
 * NOTE: This is Chapter 3 of Section III Measure. The becoming of essence
 * transitions from measure to essence. Each Chapter (Species) has three sub-species
 * that make up its Concept:
 * 
 * Structure:
 * A. ABSOLUTE INDIFFERENCE (Species 1)
 * B. INDIFFERENCE AS INVERSE RATIO OF ITS FACTORS (Species 2)
 * C. TRANSITION INTO ESSENCE (Species 3)
 * 
 * Transition: The becoming of essence passes over into essence
 */

// ============================================================================
// INTRODUCTION: CHAPTER 3 — THE BECOMING OF ESSENCE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'be-intro-1-becoming-of-essence',
    title: 'The becoming of essence — transition from measure',
    text: `CHAPTER 3

The becoming of essence`,
    summary: 'Chapter 3: The becoming of essence - transition from measure to essence.'
  },
];

// ============================================================================
// INTRODUCTION OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'be-op-intro-1-becoming-of-essence',
    chunkId: 'be-intro-1-becoming-of-essence',
    label: 'Define the becoming of essence as transition from measure',
    clauses: [
      'becomingOfEssence = transitionFromMeasure',
      'becomingOfEssence.to = essence',
    ],
    predicates: [
      { name: 'IsBecomingOfEssence', args: [] },
    ],
    relations: [
      { predicate: 'transitions', from: 'measure', to: 'essence' },
    ],
  },
];

// Note: For aggregated exports, use index.ts
// Individual species modules are available for granular access

