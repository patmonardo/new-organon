/**
 * TopicMap Schema - Type definitions for Source Analysis TopicMaps
 *
 * This schema defines the structure for TopicMap entries used in Source Analysis.
 * TopicMaps provide structured plans for chunking source texts.
 */

export type TopicMapStatus = 'pending' | 'in_progress' | 'completed' | 'reviewed';

export interface TopicMapEntry {
  /** Unique identifier - becomes Chunk.id */
  id: string;

  /** Title - becomes Chunk.title and LogicalOperation.label */
  title: string;

  /** Line range in source text { start, end } (1-indexed) */
  lineRange: { start: number; end: number };

  /** Brief description of what this chunk contains */
  description: string;

  /** Key concepts/points covered in this chunk */
  keyPoints: string[];

  /** Status of chunking/processing */
  status?: TopicMapStatus;

  /** Related chunk IDs (for tracking relationships) */
  relatedChunks?: string[];

  /** Notes for implementation */
  notes?: string;

  /** Section/parent this belongs to */
  section?: string;

  /** Order within section */
  order?: number;
}

/**
 * TopicMap - Collection of TopicMapEntries for a source file
 */
export interface TopicMap {
  /** Source file path */
  sourceFile: string;

  /** Source title */
  sourceTitle: string;

  /** Section title */
  sectionTitle: string;

  /** All topic map entries */
  entries: TopicMapEntry[];

  /** Optional metadata */
  metadata?: {
    sectionDescription?: string;
    [key: string]: any;
  };
}

/**
 * Helper: Create a TopicMap
 */
export function createTopicMap(
  sourceFile: string,
  sourceTitle: string,
  sectionTitle: string,
  entries: TopicMapEntry[],
  metadata?: { sectionDescription?: string; [key: string]: any }
): TopicMap {
  return {
    sourceFile,
    sourceTitle,
    sectionTitle,
    entries,
    metadata,
  };
}

/**
 * Helper: Create a TopicMapEntry
 */
export function createTopicMapEntry(
  id: string,
  title: string,
  lineRange: [number, number],
  description: string,
  keyPoints: string[],
  options?: {
    section?: string;
    order?: number;
    status?: TopicMapStatus;
    relatedChunks?: string[];
    notes?: string;
  }
): TopicMapEntry {
  return {
    id,
    title,
    lineRange: { start: lineRange[0], end: lineRange[1] },
    description,
    keyPoints,
    ...options,
  };
}

