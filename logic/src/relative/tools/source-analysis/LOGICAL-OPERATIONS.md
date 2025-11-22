# Logical Operations: The IR Structure

## Terminology

**Current**: HLO (Hegelian Logical Operation)  
**Better**: **Logical Operation** (LO) - Already the type name, clearer

The abbreviation "HLO" was helpful initially, but **Logical Operation** is sufficient and clearer now that we understand the architecture.

## Structure

A **Logical Operation** is the Intermediate Representation (IR) structure:

```typescript
interface LogicalOperation {
  id: string;                    // Unique identifier
  chunkId?: string;              // Links back to source chunk
  label?: string;                // Title (from TopicMapEntry.title)
  clauses: string[];             // Structured logical clauses
  predicates?: Predicate[];      // Logical predicates
  relations?: Relation[];        // Logical relations
  candidateSummary?: string;     // Summary
  provenance?: Provenance;       // Source tracking
  evidence?: unknown[];          // Supporting evidence
}
```

## Example: From TopicMap → Chunk → Logical Operation

### 1. TopicMap Entry (Source Analysis)
```typescript
createTopicMapEntry(
  'definition-1-transformation',
  'Transformation to Concept Form',  // → LogicalOperation.label
  [1291, 1312],
  'Still given objectivity transformed into simple form...',
  ['objectivity transformed to concept form', ...]
)
```

### 2. Chunk (Source Analysis - NOT in IR KG)
```typescript
{
  id: 'definition-1-transformation',  // From TopicMapEntry.id
  title: 'Transformation to Concept Form',  // From TopicMapEntry.title
  text: 'Still given objectivity transformed...',  // Extracted from source
  summary: '...'
}
```

### 3. Logical Operation (IR - In KG)
```typescript
{
  id: 'definition-op-1-transformation',
  chunkId: 'definition-1-transformation',  // Links to chunk
  label: 'Transformation to Concept Form',  // From TopicMapEntry.title
  clauses: [
    'objectivity = transformedToConceptForm',
    'moments = {universality, particularity, singularity}',
    'singular = objectToBeDefined',
    'universal = proximateGenus',
    'particular = specificDifference',
  ],
  predicates: [
    { name: 'IsTransformed', args: ['objectivity', 'conceptForm'] },
    { name: 'HasMoments', args: ['concept', 'moments'] },
  ],
  relations: [
    { predicate: 'transformsTo', from: 'objectivity', to: 'conceptForm' },
    { predicate: 'has', from: 'concept', to: 'moments' },
  ],
}
```

## Key Points

1. **Chunks are NOT in IR KG**: They're Source Analysis artifacts for readability
2. **Topics are NOT in IR KG**: They're Source Analysis planning artifacts
3. **Logical Operations ARE in IR KG**: They're the structured IR form
4. **Label comes from Topic**: `TopicMapEntry.title` → `LogicalOperation.label`

## Mapping Flow

```
TopicMapEntry (Source Analysis)
    ├── id ────────────────→ Chunk.id
    ├── title ──────────────→ Chunk.title
    │                         └─→ LogicalOperation.label
    ├── lineRange ──────────→ Extract text → Chunk.text
    └── description ────────→ Chunk.summary
                                    ↓
                            Logical Operation (IR)
                                ├── chunkId: Links to Chunk
                                ├── label: From Topic title
                                ├── clauses: Extracted from Chunk.text
                                ├── predicates: Logical predicates
                                └── relations: Logical relations
```

## All Artifacts Are Important

All formats are **different artifacts** of the translation scheme:

- **TopicMap**: Planning structure
- **Chunks**: Readable comprehension (NOT in IR KG)
- **Logical Operations**: Structured IR (IN IR KG)
- **KG**: Graph representation
- **OWL**: Semantic web
- **TS**: Executable code
- **TS-JSON**: JSON with types

They all translate **Logic → Nondual Platonic Form Processor**.

