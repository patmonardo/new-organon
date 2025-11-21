import type { Chunk, LogicalOperation } from '../../../types';

/**
 * CHAPTER 1: SPECIFIC QUANTITY — Complete Structure
 * 
 * NOTE: This is Chapter 1 of Section III Measure. Specific quantity is a Species
 * of the Genus Measure. Each Chapter (Species) has three sub-species that make up
 * its Concept:
 * 
 * Structure:
 * Introduction: Qualitative quantity as immediate specific quantum, quantitative specifying, relation of qualities
 * A. THE SPECIFIC QUANTUM (Species 1)
 * B. SPECIFYING MEASURE (Species 2)
 * C. THE BEING-FOR-ITSELF IN MEASURE (Species 3)
 * 
 * Transition: Specific quantity passes over to real measure
 */

// ============================================================================
// INTRODUCTION: CHAPTER 1 — SPECIFIC QUANTITY
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'sq-intro-1-qualitative-quantity',
    title: 'Qualitative quantity — three moments',
    text: `Qualitative quantity is,

first, an immediate, specific quantum; and this quantum,

second, in relating itself to another,
becomes a quantitative specifying,
a sublating of the indifferent quantum.
This measure is to this extent a rule
and contains the two moments of measure as different;
namely, the quantitative determinateness
and the external quantum as existing in themselves.
In this difference, however, the two sides become qualities,
and the rule becomes a relation of the two;
measure presents itself thereby,

third, as a relation of qualities that have one single measure at first;
a measure, however, which further specifies itself in itself
into a difference of measures.`,
    summary: 'Qualitative quantity: (1) immediate specific quantum, (2) quantitative specifying as rule, (3) relation of qualities with one measure specifying into difference of measures.'
  },
];

// ============================================================================
// INTRODUCTION OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'sq-op-intro-1-qualitative-quantity-structure',
    chunkId: 'sq-intro-1-qualitative-quantity',
    label: 'Define qualitative quantity as three moments: immediate quantum, quantitative specifying, relation of qualities',
    clauses: [
      'qualitativeQuantity.first = immediateSpecificQuantum',
      'qualitativeQuantity.second = quantitativeSpecifying',
      'quantitativeSpecifying = sublating(indifferentQuantum)',
      'measure.asRule = contains(twoMoments)',
      'twoMoments = {quantitativeDeterminateness, externalQuantum}',
      'twoMoments.exist = inThemselves',
      'difference.becomes = qualities',
      'rule.becomes = relationOfTwo',
      'qualitativeQuantity.third = relationOfQualities',
      'relationOfQualities.has = oneSingleMeasure',
      'oneSingleMeasure.specifies = intoDifferenceOfMeasures',
    ],
    predicates: [
      { name: 'IsQualitativeQuantity', args: [] },
      { name: 'HasThreeMoments', args: ['qualitativeQuantity'] },
    ],
    relations: [
      { predicate: 'has', from: 'qualitativeQuantity', to: 'immediateSpecificQuantum' },
      { predicate: 'has', from: 'qualitativeQuantity', to: 'quantitativeSpecifying' },
      { predicate: 'has', from: 'qualitativeQuantity', to: 'relationOfQualities' },
    ],
  },
];

// Note: For aggregated exports, use index.ts
// Individual species modules are available for granular access

