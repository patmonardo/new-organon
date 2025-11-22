# Tools

Tools and utilities for working with the Knowledge Graph.

## Structure

### `source-analysis/`

Source Analysis tools and documentation for the Source Analysis workflow.

**What's here:**
- **Documentation**: Methodology, architecture, and workflow guides
- **Utilities**: `topic-map-utils.ts` - Functions for converting Topics to Chunks and Logical Operations
- **Index**: Exports for easy importing

**Why separate from `types/`:**
- `types/` contains **KG schema definitions** (Chunk, LogicalOperation, TopicMap types)
- `tools/` contains **utilities and documentation** for working with the schema
- This separation clarifies: schema vs. tooling

## Usage

```typescript
// Import TopicMap utilities
import { topicMapToChunks, generateTopicMapReport } from '../tools/source-analysis';

// Import TopicMap types (from types/)
import type { TopicMap, TopicMapEntry } from '../types';
```

## Documentation

See `source-analysis/` for:
- `SOURCE-ANALYSIS.md` - Source Analysis workflow
- `ARCHITECTURE.md` - Architecture overview
- `TOPICMAP.md` - TopicMap documentation
- `CHUNKING-METHODOLOGY.md` - Chunking principles

