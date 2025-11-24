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

