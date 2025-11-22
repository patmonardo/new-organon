# TopicMap - Formal Chunking Model

## Overview

A **TopicMap** is a structured plan for chunking source texts that ensures systematic, trackable chunking yielding meaningful Logical Operations. It serves as the first pass to capture logic properly before extraction.

## Purpose

The TopicMap addresses the problem that chunking is often done ad-hoc, leading to:
- Incomprehensible content that nobody reads
- Inconsistent chunking that doesn't map well to Logical Operations
- Lost relationships between concepts
- Difficulty tracking progress

## Structure

### TopicMapEntry

Each entry in a TopicMap represents a planned chunk:

```typescript
interface TopicMapEntry {
  id: string;              // → Chunk.id
  title: string;           // → Chunk.title AND LogicalOperation.label
  lineRange: [number, number];  // Source text location
  description: string;      // What this chunk contains
  keyPoints: string[];     // Key concepts covered
  status: 'pending' | 'in_progress' | 'completed' | 'reviewed';
  relatedChunks?: string[];  // Track relationships
  section?: string;        // Parent section
  order?: number;          // Order within section
}
```

### TopicMap

The overall structure:

```typescript
interface TopicMap {
  sourceFile: string;      // Path to source text
  workTitle: string;        // Title of work
  section: string;          // Section being mapped
  entries: TopicMapEntry[]; // All planned chunks
  metadata?: {...};        // Creation/version info
}
```

## Mapping to Implementation

### TopicMapEntry → Chunk

- `entry.id` → `Chunk.id`
- `entry.title` → `Chunk.title`
- `entry.lineRange` → Extract text from source file → `Chunk.text`
- `entry.description` + `entry.keyPoints` → `Chunk.summary`

### TopicMapEntry → LogicalOperation

- `entry.id` → `LogicalOperation.chunkId`
- `entry.title` → `LogicalOperation.label` (the "Title")
- `entry.id` → `LogicalOperation.id` (with `-op` suffix)

## Workflow

1. **Plan**: Create TopicMap entries for each logical unit
2. **Track**: Use `status` to track progress
3. **Extract**: Convert TopicMap entries to Chunks
4. **Process**: Extract Logical Operations from Chunks
5. **Validate**: Ensure relationships and line ranges are correct

## Example Usage

```typescript
import { COGNITION_TOPIC_MAP } from './cognition-topic-map';
import { topicMapToChunks, generateTopicMapReport } from './topic-map-utils';
import { readFileSync } from 'fs';

// Read source text
const sourceText = readFileSync(COGNITION_TOPIC_MAP.sourceFile, 'utf-8');

// Generate report
const report = generateTopicMapReport(COGNITION_TOPIC_MAP);
console.log(`Total entries: ${report.total}`);
console.log(`Completed: ${report.completed}`);

// Convert to Chunks
const chunks = topicMapToChunks(COGNITION_TOPIC_MAP, sourceText);

// Process each chunk to extract Logical Operations
chunks.forEach(chunk => {
  // Extract LogicalOperation from chunk
  // The chunk.title becomes LogicalOperation.label
});
```

## Benefits

1. **Systematic**: Every chunk is planned before extraction
2. **Trackable**: Status tracking shows progress
3. **Mappable**: Clear mapping to Chunks and LogicalOperations
4. **Relational**: `relatedChunks` tracks concept relationships
5. **Validatable**: Line ranges can be validated for overlaps
6. **Documented**: Description and keyPoints provide context

## Best Practices

1. **Start with TopicMap**: Don't extract chunks without planning
2. **Use Descriptive Titles**: Titles become LogicalOperation labels
3. **Track Relationships**: Use `relatedChunks` to show connections
4. **Validate Early**: Check line ranges don't overlap
5. **Update Status**: Keep status current as you work
6. **Group by Section**: Use `section` to organize related chunks

## Status Workflow

- `pending`: Not yet started
- `in_progress`: Currently being worked on
- `completed`: Chunk extracted, LogicalOperation created
- `reviewed`: Final review passed

## Integration with Existing Workflow

The TopicMap integrates seamlessly with existing chunking:

1. **cognition-chunks.md**: Human-readable planning document
2. **cognition-topic-map.ts**: Formal TopicMap structure
3. **idea-true.ts**: Implementation using TopicMap entries

The TopicMap ensures consistency between planning and implementation.

