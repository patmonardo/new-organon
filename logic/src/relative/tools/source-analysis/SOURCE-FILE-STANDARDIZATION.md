# Source File Standardization Guide

## Compulsory Methodology

**One-to-One Pattern**: Each species must have its own source text file that generates its own artifacts.

```
{species}.txt → {species}-chunks.md + {species}-topic-map.ts
```

## Required Structure

### 1. Source Text Files (`{species}.txt`)

**Requirements**:
- One file per species (e.g., `universal.txt`, `particular.txt`, `singular.txt`)
- Contains ONLY the text for that specific species
- No top-level genera text mixed with species text
- Clean, consistent formatting
- Line numbers preserved for accurate `lineRange` references

**Naming Convention**:
- Use lowercase species name: `universal.txt`, `particular.txt`, `singular.txt`
- Match the species identifier exactly

### 2. Chunks Files (`{species}-chunks.md`)

**Requirements**:
- One file per species: `{species}-chunks.md`
- Contains chunking plan for that species only
- References the corresponding `{species}.txt` file
- References the corresponding `{species}-topic-map.ts` file
- Follows the standard chunking plan format

**Standard Header**:
```markdown
# Chunking Plan for {Species}.txt - The {Species} Concept

**SOURCE ANALYSIS PHASE 1: Planning Document**

**Purpose**: Systematic chunking methodology to yield meaningful Logical Operations.
The TopicMap helps check and improve understanding of Hegel through step-by-step analysis.

**Status**: Work in progress - refining methodology as we practice.

**Workflow Stage**: **SEED** → **UPGRADES**
- **SEED**: Initial chunking plan (current state) - good enough to form initial seed
- **UPGRADES**: Refined chunks through actual study and editing
- As you study each chunk, edit and upgrade the seed chunks
- This document evolves from planning → studied/refined chunks

**Structure**: The {Species} Concept ({line_count} lines)
- [Brief structure overview]

**Workflow**: 
```
Source Text → [Source Analysis] → Chunks + Topics → [Logical Op Generation] → Logical Operations
```

**References**: 
- `tools/source-analysis/SOURCE-ANALYSIS.md` - Source Analysis workflow documentation
- `tools/source-analysis/ARCHITECTURE.md` - Architecture overview
- `{species}-topic-map.ts` - Formal TopicMap structure (Topics)

**TopicMap**: See `{species}-topic-map.ts` for the formal TopicMap structure that maps to:
- `TopicMapEntry.id` → `Chunk.id`
- `TopicMapEntry.title` → `Chunk.title` AND `LogicalOperation.label` (the "Title")
- `TopicMapEntry.lineRange` → Extract text → `Chunk.text`

The TopicMap ensures systematic, trackable chunking that yields meaningful Logical Operations.
```

### 3. Topic Map Files (`{species}-topic-map.ts`)

**Requirements**:
- One file per species: `{species}-topic-map.ts`
- Exports a single `TopicMap` constant: `{SPECIES}_TOPIC_MAP`
- Uses consistent naming: `{SPECIES}_TOPIC_MAP` (uppercase with underscores)
- References the corresponding `{species}.txt` file
- References the corresponding `{species}-chunks.md` file

**Standard Header**:
```typescript
/**
 * TopicMap for {Species}.txt - The {Species} Concept
 *
 * SOURCE ANALYSIS PHASE 1: Topics
 *
 * COGNITIVE SCIENCE: This is where the real cognitive work happens.
 * The skill in producing good chunks and topics is what makes everything else meaningful.
 * The TopicMap helps check and improve understanding of Hegel through step-by-step analysis.
 *
 * Architecture:
 *    Source Text → [Source Analysis: Cognitive Science] → Chunks + Topics
 *                                                              ↓
 *                    [Logical Op Generation: IR Translation] → Logical Operations (IR)
 *                                                              ↓
 *                    [Codegen: Backend] → Executable Code
 *
 * This TopicMap provides the structured plan for chunking the source text
 * into meaningful chunks. Good chunking/topic analysis makes Logical Operations meaningful
 * (not just jargon) and enables executable codegen (the backend).
 *
 * Each entry maps to:
 * - TopicMapEntry.id → Chunk.id
 * - TopicMapEntry.title → Chunk.title AND LogicalOperation.label (the "Title")
 * - TopicMapEntry.lineRange → Extract text → Chunk.text
 *
 * Reference:
 * - {species}-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '../../../types/topic-map';
import { createTopicMap, createTopicMapEntry } from '../../../types/topic-map';

export const {SPECIES}_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/concept/sources/{species}.txt',
  'Hegel\'s Science of Logic - The Concept',
  'The {Species} Concept',
  [
    // TopicMapEntry entries...
  ],
  {
    sectionDescription: 'The {Species} Concept - [Brief description].',
  }
);
```

## Standardization Checklist

When standardizing source files, ensure:

- [ ] **One source text file per species** (no mixing genera with species)
- [ ] **Consistent naming**: `{species}.txt`, `{species}-chunks.md`, `{species}-topic-map.ts`
- [ ] **Consistent constant naming**: `{SPECIES}_TOPIC_MAP` (uppercase with underscores)
- [ ] **All references updated** to match the one-to-one pattern
- [ ] **Headers standardized** across all files
- [ ] **Line ranges accurate** and match the source text
- [ ] **No contradictory methodology** - all files follow the same pattern

## Common Issues to Fix

### Issue 1: Mixed Source Files
**Problem**: One source file contains multiple species or genera mixed together.

**Solution**: Split into separate files:
- `concept.txt` (genera) → Split into `universal.txt`, `particular.txt`, `singular.txt`

### Issue 2: Inconsistent Naming
**Problem**: Files use different naming conventions.

**Solution**: Standardize to:
- `{species}.txt` (lowercase, hyphenated if needed)
- `{species}-chunks.md`
- `{species}-topic-map.ts`

### Issue 3: Missing References
**Problem**: Files don't reference each other correctly.

**Solution**: Ensure all three files reference each other:
- `{species}-chunks.md` references `{species}.txt` and `{species}-topic-map.ts`
- `{species}-topic-map.ts` references `{species}.txt` and `{species}-chunks.md`

### Issue 4: Inconsistent Headers
**Problem**: Headers don't follow the standard format.

**Solution**: Use the standard headers provided above.

## Enforcement

**This methodology is compulsory**. All source files must follow this pattern:
- No exceptions
- No mixing of species in one file
- No inconsistent naming
- No contradictory methodology

**When in doubt**: Follow the pattern established by `universal.txt`, `universal-chunks.md`, `universal-topic-map.ts`.

