import type { Chunk, LogicalOperation } from '../../../types';

/**
 * CHAPTER 2: REAL MEASURE — Complete Structure
 * 
 * NOTE: This is Chapter 2 of Section III Measure. Real measure is a Species
 * of the Genus Measure. Each Chapter (Species) has three sub-species that make up
 * its Concept:
 * 
 * Structure:
 * A. THE RELATION OF INDEPENDENT MEASURES (Species 1)
 * B. NODAL LINES OF MEASURE-RELATIONS (Species 2)
 * C. THE MEASURELESS (Species 3)
 * 
 * Transition: Real measure passes over to becoming-essence
 */

// ============================================================================
// INTRODUCTION: CHAPTER 2 — REAL MEASURE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'rm-intro-1-independent-measures',
    title: 'Independent measures — self-subsistent, physical, material',
    text: `By measures we no longer mean now
merely immediate measures,
but measures that are self-subsistent
because they become within themselves
relations which are specified,
and in this being-for-itself
they are thus a something,
things that are physical
and at first material.`,
    summary: 'Measures now = self-subsistent (not merely immediate), become within themselves specified relations, in being-for-itself = something, physical, material.'
  },
];

// ============================================================================
// INTRODUCTION OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'rm-op-intro-1-independent-measures',
    chunkId: 'rm-intro-1-independent-measures',
    label: 'Define independent measures as self-subsistent, physical, material',
    clauses: [
      'measures.not = merelyImmediateMeasures',
      'measures = selfSubsistent',
      'measures.becomeWithinThemselves = specifiedRelations',
      'measures.inBeingForItself = something',
      'measures.as = {physical, material}',
    ],
    predicates: [
      { name: 'IsIndependentMeasure', args: [] },
      { name: 'IsSelfSubsistent', args: ['measure'] },
    ],
    relations: [
      { predicate: 'is', from: 'measure', to: 'selfSubsistent' },
      { predicate: 'becomes', from: 'measure', to: 'specifiedRelations' },
    ],
  },
];

// Note: For aggregated exports, use index.ts
// Individual species modules are available for granular access

