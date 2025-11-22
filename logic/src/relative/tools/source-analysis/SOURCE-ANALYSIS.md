# Source Analysis Workflow

## Overview

**Source Analysis IS Cognitive Science** - or perhaps **the Principle of Cognitive Science**.

Source Analysis is the **primary cognitive work** of Knowledge Graph construction. The skill in producing good chunks and topics is where real comprehension happens. This alone helps us understand the source.

**Source Analysis should be readable** - a type of **structured meal** to help digest the Logic. When done well, it makes Hegel's Science of Logic accessible and comprehensible.

**We Are Already in the Idea** - Reading Hegel requires a fundamentally different approach than normal Western scientific reading. We're not outside observers analyzing an object - we're **already within the Idea** as we read. The Idea sees through us. See `HOW-TO-READ-HEGEL.md` for more on this hermeneutical stance.

HLOs (Hegelian Logical Operations) are "all jargon LOL" - but they become meaningful when built on good chunking/topic analysis. They serve as **pure Intermediate Representations for Hegelian Logic** that enable executable codegen (the backend).

**IR and Codegen are Speculative Art of the Highest Realms of Mind** - stop and think about what we are trying to encode here. We're encoding the Science of Logic itself, the systematic exposition of the Absolute Idea. This is not mere translation - this is **encoding the structure of thought itself**.

## Workflow

**Semantic Flow: Chunking → Mapping**

```
Source Text (.txt)
    ↓
[Chunking - Human Consumption]
    ↓
Chunks (TopicChunking - extracting Topics)
    ↓
[TopicMap - Technical Artifact]
    ↓
[Logical Operations - IR (to be reinvented)]
    ↓
LO Map → TS (Technical)
```

**Key Points:**
- **Chunking** = Human consumption (readable, digestible)
- **Chunking** = TopicChunking (extracting Topics)
- **TopicMap** = Technical artifact (from Chunking)
- **IR (LO) no longer needs to maintain chunks** - flows through TopicMap
- **LO system will be reinvented later** - for now, stick with Chunking
- **Chunking shows up in Technical parts**: TopicMap → LO → LO Map → TS

## Phase 1: Source Analysis

### Input
- Raw source text (e.g., `cognition.txt`)

### Output Artifacts

**IMPORTANT**: Chunks and Topics are **Source Analysis artifacts**, NOT in the IR KG.

1. **Chunks** (`*.ts` files)
   - Extracted text segments with metadata
   - `Chunk.id`, `Chunk.title`, `Chunk.text`, `Chunk.summary`
   - **Purpose**: Readable, digestible form (NOT in IR KG)
   - Example: `idea-cognition.ts`, `idea-true.ts`

2. **Topics** (`*-topic-map.ts` files)
   - Structured planning for chunking
   - Maps to Chunks and Logical Operations
   - `TopicMapEntry.id` → `Chunk.id`
   - `TopicMapEntry.title` → `Chunk.title` AND `LogicalOperation.label`
   - **Purpose**: Source Analysis planning (NOT in IR KG)
   - Example: `cognition-topic-map.ts`

3. **Planning Documents** (`*.md` files)
   - Human-readable chunking plans
   - Reference for TopicMap creation
   - Example: `cognition-chunks.md`

### Artifacts Location

All Source Analysis artifacts live in the `sources/` folder:

```
sources/
├── cognition.txt                    # Raw source
├── cognition-chunks.md              # Planning document
├── cognition-topic-map.ts           # Topics (TopicMap)
├── idea-cognition.ts                # Chunks (implementation)
└── ...
```

## Phase 2: Logical Operation Generation (IR)

### Input
- Chunks (from Phase 1 - Source Analysis artifacts)
- Topics (from Phase 1 - Source Analysis artifacts)

### Process
1. Extract clauses, predicates, relations from Chunks
2. Use Topic titles as LogicalOperation labels
3. Generate Logical Operations (IR artifacts)

### Output
- `LogicalOperation[]` (IR artifacts - IN IR KG) with:
  - `id`: Generated from chunk
  - `chunkId`: Links back to source chunk (Source Analysis artifact)
  - `label`: From TopicMapEntry.title (the "Title")
  - `clauses`, `predicates`, `relations`: Extracted from chunk text

**Note**: Chunks and Topics remain Source Analysis artifacts. Only Logical Operations go into the IR KG.

## Key Mapping

```
TopicMapEntry
    ├── id ────────────────→ Chunk.id
    ├── title ──────────────→ Chunk.title
    │                         └─→ LogicalOperation.label (the "Title")
    ├── lineRange ──────────→ Extract text → Chunk.text
    ├── description ────────→ Chunk.summary
    └── keyPoints ──────────→ Additional summary content
```

## Benefits

1. **Cognitive Science**: The real work of understanding happens here
2. **Systematic**: Topics plan chunks before extraction
3. **Trackable**: Status tracking shows progress
4. **Mappable**: Clear mapping to Chunks and Logical Operations
5. **Relational**: Topics track concept relationships
6. **Validatable**: Line ranges can be validated
7. **Meaningful HLOs**: Good chunking/topics make HLOs meaningful (not just jargon)
8. **Enables Codegen**: Structured HLOs enable executable codegen (backend)

## Example Workflow

1. **Plan** (Cognitive Work): Create `cognition-chunks.md` with chunking plan
   - Understand the logical structure
   - Identify conceptual boundaries
   - This IS Cognitive Science

2. **Structure** (Cognitive Work): Create `cognition-topic-map.ts` with TopicMap entries
   - Structure comprehension
   - Map relationships
   - Track progress

3. **Extract** (Cognitive Work): Generate Chunks from Topics using `topicMapToChunks()`
   - Extract logical units
   - Create meaningful chunks
   - The skill here is what matters

4. **Implement**: Create `idea-cognition.ts` with Chunks and Logical Operations
   - Chunks from Source Analysis
   - HLOs as IR (structured form)

5. **Generate**: Extract Logical Operations from Chunks
   - HLOs become meaningful because of good Source Analysis
   - Enable executable codegen (backend)

## File Naming Conventions

- **Source texts**: `{name}.txt`
- **Planning docs**: `{name}-chunks.md`
- **Topics**: `{name}-topic-map.ts`
- **Chunks**: `idea-{name}.ts` (in `cognition/` folder)

## Status Tracking

Topics have status:
- `pending`: Not yet started
- `in_progress`: Currently being worked on
- `completed`: Chunk extracted, LogicalOperation created
- `reviewed`: Final review passed

Use `generateTopicMapReport()` to track progress.

