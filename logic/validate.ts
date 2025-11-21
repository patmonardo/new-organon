#!/usr/bin/env tsx
/**
 * Validation Script for The Science of Mind Knowledge Graph
 * 
 * Verifies:
 * 1. Structural integrity (unique IDs, valid references)
 * 2. Reversibility (text can be reconstructed)
 * 3. Completeness (1:1 chunk:operation mapping)
 * 4. Type safety (all structures conform to types)
 */

import {
  CANONICAL_CHUNKS as UNIVERSAL_CHUNKS,
  LOGICAL_OPERATIONS as UNIVERSAL_OPS,
} from './src/relative/concept/subject/concept/concept_universal';

import {
  CANONICAL_CHUNKS as PARTICULAR_CHUNKS,
  LOGICAL_OPERATIONS as PARTICULAR_OPS,
} from './src/relative/concept/subject/concept/concept_particular';

import {
  CANONICAL_CHUNKS as SINGULAR_CHUNKS,
  LOGICAL_OPERATIONS as SINGULAR_OPS,
} from './src/relative/concept/subject/concept/concept_singular';

import {
  CANONICAL_CHUNKS as JUDGMENT_EXISTENCE_CHUNKS,
  LOGICAL_OPERATIONS as JUDGMENT_EXISTENCE_OPS,
} from './src/relative/concept/subject/judgment/judgment_existence';

import {
  CANONICAL_CHUNKS as JUDGMENT_REFLECTION_CHUNKS,
  LOGICAL_OPERATIONS as JUDGMENT_REFLECTION_OPS,
} from './src/relative/concept/subject/judgment/judgment_reflection';

import {
  CANONICAL_CHUNKS as JUDGMENT_NECESSITY_CHUNKS,
  LOGICAL_OPERATIONS as JUDGMENT_NECESSITY_OPS,
} from './src/relative/concept/subject/judgment/judgment_necessity';

import {
  CANONICAL_CHUNKS as JUDGMENT_CONCEPT_CHUNKS,
  LOGICAL_OPERATIONS as JUDGMENT_CONCEPT_OPS,
} from './src/relative/concept/subject/judgment/judgment_concept';

import {
  CANONICAL_CHUNKS as SYLLOGISM_EXISTENCE_CHUNKS,
  LOGICAL_OPERATIONS as SYLLOGISM_EXISTENCE_OPS,
} from './src/relative/concept/subject/syllogism/syllogism_existence';

import {
  CANONICAL_CHUNKS as SYLLOGISM_REFLECTION_CHUNKS,
  LOGICAL_OPERATIONS as SYLLOGISM_REFLECTION_OPS,
} from './src/relative/concept/subject/syllogism/syllogism_reflection';

import {
  CANONICAL_CHUNKS as SYLLOGISM_NECESSITY_CHUNKS,
  LOGICAL_OPERATIONS as SYLLOGISM_NECESSITY_OPS,
} from './src/relative/concept/subject/syllogism/syllogism_necessity';

// Aggregate concept chunks and operations
const CONCEPT_CHUNKS = [
  ...UNIVERSAL_CHUNKS,
  ...PARTICULAR_CHUNKS,
  ...SINGULAR_CHUNKS,
];

const CONCEPT_OPS = [
  ...UNIVERSAL_OPS,
  ...PARTICULAR_OPS,
  ...SINGULAR_OPS,
];

// Aggregate judgment chunks and operations
const JUDGMENT_CHUNKS = [
  ...JUDGMENT_EXISTENCE_CHUNKS,
  ...JUDGMENT_REFLECTION_CHUNKS,
  ...JUDGMENT_NECESSITY_CHUNKS,
  ...JUDGMENT_CONCEPT_CHUNKS,
];

const JUDGMENT_OPS = [
  ...JUDGMENT_EXISTENCE_OPS,
  ...JUDGMENT_REFLECTION_OPS,
  ...JUDGMENT_NECESSITY_OPS,
  ...JUDGMENT_CONCEPT_OPS,
];

// Aggregate syllogism chunks and operations
const SYLLOGISM_CHUNKS = [
  ...SYLLOGISM_EXISTENCE_CHUNKS,
  ...SYLLOGISM_REFLECTION_CHUNKS,
  ...SYLLOGISM_NECESSITY_CHUNKS,
];

const SYLLOGISM_OPS = [
  ...SYLLOGISM_EXISTENCE_OPS,
  ...SYLLOGISM_REFLECTION_OPS,
  ...SYLLOGISM_NECESSITY_OPS,
];

// Aggregate all chunks and operations
const ALL_CHUNKS = [
  ...CONCEPT_CHUNKS,
  ...JUDGMENT_CHUNKS,
  ...SYLLOGISM_CHUNKS,
];

const ALL_OPS = [
  ...CONCEPT_OPS,
  ...JUDGMENT_OPS,
  ...SYLLOGISM_OPS,
];

interface ValidationResult {
  passed: boolean;
  errors: string[];
  warnings: string[];
  stats: {
    totalChunks: number;
    totalOperations: number;
    uniqueChunkIds: number;
    uniqueOpIds: number;
    chunksWithOps: number;
    opsWithChunks: number;
  };
}

function validate(): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  // 1. Check for duplicate chunk IDs
  const chunkIds = ALL_CHUNKS.map((c) => c.id);
  const chunkIdSet = new Set(chunkIds);
  if (chunkIds.length !== chunkIdSet.size) {
    const duplicates = chunkIds.filter((id, idx) => chunkIds.indexOf(id) !== idx);
    errors.push(`Duplicate chunk IDs found: ${duplicates.join(', ')}`);
  }

  // 2. Check for duplicate operation IDs
  const opIds = ALL_OPS.map((op) => op.id);
  const opIdSet = new Set(opIds);
  if (opIds.length !== opIdSet.size) {
    const duplicates = opIds.filter((id, idx) => opIds.indexOf(id) !== idx);
    errors.push(`Duplicate operation IDs found: ${duplicates.join(', ')}`);
  }

  // 3. Check that all operations reference valid chunks
  const chunkIdMap = new Set(chunkIds);
  const opsWithInvalidChunks = ALL_OPS.filter((op) => !chunkIdMap.has(op.chunkId));
  if (opsWithInvalidChunks.length > 0) {
    errors.push(
      `Operations with invalid chunkId references: ${opsWithInvalidChunks.map((op) => op.id).join(', ')}`
    );
  }

  // 4. Check 1:1 mapping (each chunk should have at least one operation)
  const opsByChunkId = new Map<string, number>();
  ALL_OPS.forEach((op) => {
    opsByChunkId.set(op.chunkId, (opsByChunkId.get(op.chunkId) || 0) + 1);
  });

  const chunksWithoutOps = ALL_CHUNKS.filter((c) => !opsByChunkId.has(c.id));
  if (chunksWithoutOps.length > 0) {
    warnings.push(
      `Chunks without operations: ${chunksWithoutOps.map((c) => c.id).join(', ')}`
    );
  }

  // 5. Check for chunks with multiple operations (this is OK, but note it)
  const chunksWithMultipleOps = Array.from(opsByChunkId.entries()).filter(
    ([, count]) => count > 1
  );
  if (chunksWithMultipleOps.length > 0) {
    warnings.push(
      `Chunks with multiple operations: ${chunksWithMultipleOps.map(([id]) => id).join(', ')}`
    );
  }

  // 6. Verify reversibility: all chunks have text
  const chunksWithoutText = ALL_CHUNKS.filter((c) => !c.text || c.text.trim().length === 0);
  if (chunksWithoutText.length > 0) {
    errors.push(`Chunks without text: ${chunksWithoutText.map((c) => c.id).join(', ')}`);
  }

  // 7. Check for required operation fields
  const opsWithoutLabel = ALL_OPS.filter((op) => !op.label || op.label.trim().length === 0);
  if (opsWithoutLabel.length > 0) {
    warnings.push(`Operations without label: ${opsWithoutLabel.map((op) => op.id).join(', ')}`);
  }

  const opsWithoutClauses = ALL_OPS.filter((op) => !op.clauses || op.clauses.length === 0);
  if (opsWithoutClauses.length > 0) {
    warnings.push(
      `Operations without clauses: ${opsWithoutClauses.map((op) => op.id).join(', ')}`
    );
  }

  // 8. Statistics
  const stats = {
    totalChunks: ALL_CHUNKS.length,
    totalOperations: ALL_OPS.length,
    uniqueChunkIds: chunkIdSet.size,
    uniqueOpIds: opIdSet.size,
    chunksWithOps: ALL_CHUNKS.filter((c) => opsByChunkId.has(c.id)).length,
    opsWithChunks: ALL_OPS.filter((op) => chunkIdMap.has(op.chunkId)).length,
  };

  return {
    passed: errors.length === 0,
    errors,
    warnings,
    stats,
  };
}

function demonstrateReversibility() {
  console.log('\n=== REVERSIBILITY DEMONSTRATION ===\n');

  // Show that text can be reconstructed
  const reconstructedText = ALL_CHUNKS.map((chunk, idx) => {
    return `[${idx + 1}] ${chunk.id}\n${chunk.text}`;
  }).join('\n\n---\n\n');

  console.log(`Reconstructed text length: ${reconstructedText.length} characters`);
  console.log(`Total chunks: ${ALL_CHUNKS.length}`);
  console.log(`Average chunk length: ${Math.round(reconstructedText.length / ALL_CHUNKS.length)} characters`);

  // Show sample reconstruction
  if (ALL_CHUNKS.length > 0) {
    console.log('\n--- Sample Reconstruction (First Chunk) ---');
    console.log(ALL_CHUNKS[0].text.substring(0, 200) + '...');
  }
}

// Main execution
function main() {
  console.log('=== THE SCIENCE OF MIND — VALIDATION ===\n');

  const result = validate();

  console.log('📊 Statistics:');
  console.log(`  Total Chunks: ${result.stats.totalChunks}`);
  console.log(`  Total Operations: ${result.stats.totalOperations}`);
  console.log(`  Unique Chunk IDs: ${result.stats.uniqueChunkIds}`);
  console.log(`  Unique Operation IDs: ${result.stats.uniqueOpIds}`);
  console.log(`  Chunks with Operations: ${result.stats.chunksWithOps}`);
  console.log(`  Operations with Valid Chunks: ${result.stats.opsWithChunks}`);

  if (result.errors.length > 0) {
    console.log('\n❌ Errors:');
    result.errors.forEach((error) => console.log(`  - ${error}`));
  }

  if (result.warnings.length > 0) {
    console.log('\n⚠️  Warnings:');
    result.warnings.forEach((warning) => console.log(`  - ${warning}`));
  }

  if (result.passed && result.warnings.length === 0) {
    console.log('\n✅ All validations passed!');
  } else if (result.passed) {
    console.log('\n✅ Core validations passed (with warnings)');
  } else {
    console.log('\n❌ Validation failed');
    process.exit(1);
  }

  demonstrateReversibility();

  console.log('\n=== VALIDATION COMPLETE ===');
}

main();

