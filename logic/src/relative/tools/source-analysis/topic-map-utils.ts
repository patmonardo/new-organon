/**
 * Utilities for working with TopicMaps
 * 
 * These utilities help convert TopicMap entries into Chunks and LogicalOperations,
 * ensuring the systematic mapping from planning to implementation.
 */

import type { TopicMap, TopicMapEntry } from '../../types/topic-map';
import type { Chunk, LogicalOperation } from '../../types';

/**
 * Convert a TopicMapEntry to a Chunk structure
 * 
 * This extracts the text from the source file based on lineRange
 * and creates a Chunk with the appropriate id and title.
 */
export function topicMapEntryToChunk(
  entry: TopicMapEntry,
  sourceText: string
): Chunk {
  const lines = sourceText.split('\n');
  const [startLine, endLine] = entry.lineRange;
  
  // Extract text (lineRange is 1-indexed, array is 0-indexed)
  const textLines = lines.slice(startLine - 1, endLine);
  const text = textLines.join('\n').trim();
  
  // Create summary from description and keyPoints
  const summary = [
    entry.description,
    ...entry.keyPoints.map(point => `- ${point}`),
  ].join('\n');
  
  return {
    id: entry.id,
    title: entry.title,
    text,
    summary,
    // Store metadata
    topicMapEntry: entry,
  };
}

/**
 * Convert all TopicMap entries to Chunks
 */
export function topicMapToChunks(
  topicMap: TopicMap,
  sourceText: string
): Chunk[] {
  return topicMap.entries
    .filter(entry => entry.status !== 'pending')
    .map(entry => topicMapEntryToChunk(entry, sourceText));
}

/**
 * Create a LogicalOperation label from TopicMapEntry title
 * 
 * The title becomes the label (the "Title" in Logical Operations)
 */
export function topicMapEntryToLogicalOperationLabel(
  entry: TopicMapEntry
): string {
  return entry.title;
}

/**
 * Create a LogicalOperation structure from a TopicMapEntry
 * 
 * This creates the basic structure - clauses, predicates, and relations
 * would be extracted separately through the Logical Operation extraction process.
 */
export function topicMapEntryToLogicalOperation(
  entry: TopicMapEntry
): Omit<LogicalOperation, 'clauses' | 'predicates' | 'relations'> {
  return {
    id: `${entry.id}-op`,
    chunkId: entry.id,
    label: entry.title,
    candidateSummary: entry.description,
    provenance: {
      sourceChunk: entry.id,
      extractor: 'topic-map',
    },
  };
}

/**
 * Get TopicMap entries ready for processing
 * (status: in_progress or completed)
 */
export function getReadyTopicMapEntries(topicMap: TopicMap): TopicMapEntry[] {
  return topicMap.entries.filter(
    entry => entry.status === 'in_progress' || entry.status === 'completed'
  );
}

/**
 * Get TopicMap entries that need work
 * (status: pending)
 */
export function getPendingTopicMapEntries(topicMap: TopicMap): TopicMapEntry[] {
  return topicMap.entries.filter(entry => entry.status === 'pending');
}

/**
 * Generate a report of TopicMap status
 */
export function generateTopicMapReport(topicMap: TopicMap): {
  total: number;
  pending: number;
  inProgress: number;
  completed: number;
  reviewed: number;
  bySection: Record<string, { total: number; completed: number }>;
} {
  const bySection: Record<string, { total: number; completed: number }> = {};
  
  topicMap.entries.forEach(entry => {
    const section = entry.section || 'unknown';
    if (!bySection[section]) {
      bySection[section] = { total: 0, completed: 0 };
    }
    bySection[section].total++;
    if (entry.status === 'completed' || entry.status === 'reviewed') {
      bySection[section].completed++;
    }
  });
  
  return {
    total: topicMap.entries.length,
    pending: topicMap.entries.filter(e => e.status === 'pending').length,
    inProgress: topicMap.entries.filter(e => e.status === 'in_progress').length,
    completed: topicMap.entries.filter(e => e.status === 'completed').length,
    reviewed: topicMap.entries.filter(e => e.status === 'reviewed').length,
    bySection,
  };
}

/**
 * Validate that line ranges don't overlap
 */
export function validateTopicMapLineRanges(topicMap: TopicMap): {
  valid: boolean;
  overlaps: Array<{ entry1: string; entry2: string; lines: [number, number] }>;
} {
  const overlaps: Array<{ entry1: string; entry2: string; lines: [number, number] }> = [];
  const sorted = [...topicMap.entries].sort((a, b) => a.lineRange[0] - b.lineRange[0]);
  
  for (let i = 0; i < sorted.length - 1; i++) {
    const current = sorted[i];
    const next = sorted[i + 1];
    
    // Check if current's end overlaps with next's start
    if (current.lineRange[1] >= next.lineRange[0]) {
      overlaps.push({
        entry1: current.id,
        entry2: next.id,
        lines: [current.lineRange[1], next.lineRange[0]],
      });
    }
  }
  
  return {
    valid: overlaps.length === 0,
    overlaps,
  };
}

