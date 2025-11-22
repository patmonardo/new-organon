/**
 * TopicMap - Formal Chunking Model
 *
 * KG SCHEMA: This defines the TopicMap type structure for Source Analysis.
 *
 * A TopicMap is a structured plan for chunking source texts that maps to:
 * - Chunk.id and Chunk.title
 * - LogicalOperation.label (the "Title" in Logical Operations)
 *
 * This ensures systematic, trackable chunking that yields meaningful Logical Operations.
 * The TopicMap serves as the first pass to capture logic properly before extraction.
 *
 * NOTE: TopicMap utilities and documentation are in tools/source-analysis/
 */

export type TopicMapStatus = 'pending' | 'in_progress' | 'completed' | 'reviewed';

export interface TopicMapEntry {
  /** Unique identifier - becomes Chunk.id */
  id: string;

  /** Title - becomes Chunk.title and LogicalOperation.label */
  title: string;

  /** Line range in source text [start, end] (1-indexed) */
  lineRange: [number, number];

  /** Brief description of what this chunk contains */
  description: string;

  /** Key concepts/points covered in this chunk */
  keyPoints: string[];

  /** Status of chunking/processing */
  status: TopicMapStatus;

  /** Related chunk IDs (for tracking relationships) */
  relatedChunks?: string[];

  /** Notes for implementation */
  notes?: string;

  /** Section/parent this belongs to */
  section?: string;

  /** Order within section */
  order?: number;
}

export interface TopicMap {
  /** Source file path */
  sourceFile: string;

  /** Title of the overall work being chunked */
  workTitle: string;

  /** Section/chapter being mapped */
  section: string;

  /** Description of the section */
  sectionDescription?: string;

  /** All topic map entries */
  entries: TopicMapEntry[];

  /** Metadata */
  metadata?: {
    createdAt?: string;
    updatedAt?: string;
    author?: string;
    version?: string;
  };
}

/**
 * Create a TopicMapEntry with defaults
 */
export function createTopicMapEntry(
  id: string,
  title: string,
  lineRange: [number, number],
  description: string,
  keyPoints: string[],
  options?: {
    status?: TopicMapStatus;
    relatedChunks?: string[];
    notes?: string;
    section?: string;
    order?: number;
  }
): TopicMapEntry {
  return {
    id,
    title,
    lineRange,
    description,
    keyPoints,
    status: options?.status ?? 'pending',
    relatedChunks: options?.relatedChunks,
    notes: options?.notes,
    section: options?.section,
    order: options?.order,
  };
}

/**
 * Create a TopicMap
 */
export function createTopicMap(
  sourceFile: string,
  workTitle: string,
  section: string,
  entries: TopicMapEntry[],
  options?: {
    sectionDescription?: string;
    metadata?: TopicMap['metadata'];
  }
): TopicMap {
  return {
    sourceFile,
    workTitle,
    section,
    sectionDescription: options?.sectionDescription,
    entries,
    metadata: options?.metadata,
  };
}

/**
 * Validate TopicMap structure
 */
export function validateTopicMap(topicMap: TopicMap): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  if (!topicMap.sourceFile) {
    errors.push('sourceFile is required');
  }

  if (!topicMap.workTitle) {
    errors.push('workTitle is required');
  }

  if (!topicMap.section) {
    errors.push('section is required');
  }

  if (!topicMap.entries || topicMap.entries.length === 0) {
    errors.push('entries array is required and must not be empty');
  }

  // Validate entries
  topicMap.entries.forEach((entry, index) => {
    if (!entry.id) {
      errors.push(`Entry ${index}: id is required`);
    }
    if (!entry.title) {
      errors.push(`Entry ${index}: title is required`);
    }
    if (!entry.lineRange || entry.lineRange.length !== 2) {
      errors.push(`Entry ${index}: lineRange must be [start, end]`);
    }
    if (entry.lineRange && entry.lineRange[0] > entry.lineRange[1]) {
      errors.push(`Entry ${index}: lineRange start must be <= end`);
    }
    if (!entry.description) {
      errors.push(`Entry ${index}: description is required`);
    }
    if (!entry.keyPoints || entry.keyPoints.length === 0) {
      errors.push(`Entry ${index}: keyPoints array is required and must not be empty`);
    }
  });

  return {
    valid: errors.length === 0,
    errors,
  };
}

/**
 * Get entry by ID
 */
export function getTopicMapEntry(topicMap: TopicMap, id: string): TopicMapEntry | undefined {
  return topicMap.entries.find(entry => entry.id === id);
}

/**
 * Get entries by status
 */
export function getTopicMapEntriesByStatus(
  topicMap: TopicMap,
  status: TopicMapStatus
): TopicMapEntry[] {
  return topicMap.entries.filter(entry => entry.status === status);
}

/**
 * Get entries in line range order
 */
export function getTopicMapEntriesSorted(topicMap: TopicMap): TopicMapEntry[] {
  return [...topicMap.entries].sort((a, b) => a.lineRange[0] - b.lineRange[0]);
}

/**
 * Update entry status
 */
export function updateTopicMapEntryStatus(
  topicMap: TopicMap,
  id: string,
  status: TopicMapStatus
): TopicMap {
  return {
    ...topicMap,
    entries: topicMap.entries.map(entry =>
      entry.id === id ? { ...entry, status } : entry
    ),
    metadata: {
      ...topicMap.metadata,
      updatedAt: new Date().toISOString(),
    },
  };
}

