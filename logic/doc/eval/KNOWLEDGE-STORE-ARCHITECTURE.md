# KnowledgeStore Architecture

## The Insight

**The Agent that maintains the KnowledgeStore:**

- **Shape** = Genus/Species
- **Context** = Subject/Predicate
- **Morph** = Identities

**We would store not just all of the identities, but the Contexts that allowed the Shape:Context to support Morph discovery.**

## KnowledgeStore in the Architecture

**KnowledgeStore** (Concept layer):
- **Reason:Statement:Theorem** = Conceptual Being
- **Science/Universal-side**: Store of *promoted* determinations — relations/truth-steps worthy of inclusion in a Universal "encyclopedia" surface
- **Part of**: GraphStore → FactStore → KnowledgeStore (Being → Essence → Concept)

## The Architecture

### KnowledgeStore Structure

```
KnowledgeStore {
  // Shape:Context:Morph structure
  Shape: Genus/Species (e.g., Trigonometry)
  Context: Subject/Predicate (the conditions that enabled discovery)
  Morph: Identities (the discovered identities)
  
  // Key insight: Store the Contexts that enabled Morph discovery
  DiscoveryContexts: {
    [MorphId]: {
      Shape: Genus/Species,
      Context: Subject/Predicate,  // The Context that enabled this Morph discovery
      Morph: Identity
    }
  }
  
  // KnowledgeStore as Concept (Reason:Statement:Theorem)
  Reason: The systematic structure (evolved from Ground)
  Statement: The logical representation of State (evolved from Condition)
  Theorem: The proven fact (evolved from Facticity)
}
```

### What We Store

1. **Shape (Genus/Species)** = The concept (e.g., Trigonometry)
2. **Context (Subject/Predicate)** = The conditions that enabled discovery
3. **Morph (Identities)** = The discovered identities
4. **Discovery Contexts** = The Contexts that allowed Shape:Context to support Morph discovery

## The Flow

### Morph Discovery

```
Shape (Genus/Species - Trigonometry)
  ↓
Context (Subject/Predicate - the conditions)
  ↓
Shape:Context supports Morph discovery
  ↓
Morph (Identity discovered)
  ↓
Store in KnowledgeStore:
  - The Morph (Identity)
  - The Context that enabled its discovery
```

### KnowledgeStore Maintenance

```
Agent maintains KnowledgeStore:
  - Shape: Genus/Species
  - Context: Subject/Predicate (conditions that enabled discovery)
  - Morph: Identities
  
  Key: Store not just identities, but the Contexts that enabled their discovery
```

## The Point

**We store not just all of the identities (Morphs), but the Contexts that allowed the Shape:Context to support Morph discovery.**

- **Shape** = Genus/Species (the concept)
- **Context** = Subject/Predicate (the conditions that enabled discovery)
- **Morph** = Identities (the discovered identities)
- **Discovery Contexts** = The Contexts that allowed Shape:Context to support Morph discovery

This allows us to:
1. **Store the identities** (Morphs) that were discovered
2. **Store the Contexts** that enabled their discovery
3. **Understand the conditions** that allowed Shape:Context to support Morph discovery
4. **Reconstruct the discovery process** by understanding which Contexts enabled which Morphs

## The Architecture Connection

### Shape:Context:Morph in KnowledgeStore

- **Shape** = Genus/Species (stored in KnowledgeStore)
- **Context** = Subject/Predicate (stored as the conditions that enabled discovery)
- **Morph** = Identities (stored as the discovered identities)
- **Discovery Contexts** = The Contexts that allowed Shape:Context to support Morph discovery

### The Agent

**The Agent that maintains the KnowledgeStore** stores:
- The Shape (Genus/Species)
- The Context (Subject/Predicate) that enabled discovery
- The Morph (Identities) that were discovered
- The relationship: which Contexts enabled which Morph discoveries

This is how we maintain a KnowledgeStore that captures not just the results (Morphs/Identities), but the conditions (Contexts) that enabled their discovery.

## The Connection to Concept Layer

**KnowledgeStore is Concept** — Reason:Statement:Theorem as Conceptual Being.

- **Shape:Context:Morph** = The structure stored in KnowledgeStore
- **Reason:Statement:Theorem** = The Concept layer representation
- **Discovery Contexts** = The Contexts that allowed Shape:Context to support Morph discovery

**The Agent maintains the KnowledgeStore** by storing:
1. The Shape (Genus/Species) as the concept
2. The Context (Subject/Predicate) that enabled discovery
3. The Morph (Identities) that were discovered
4. The relationship: which Contexts enabled which Morph discoveries

This allows the KnowledgeStore to capture not just the identities (Morphs), but the discovery conditions (Contexts) that enabled them, maintaining the full Shape:Context:Morph structure that supports scientific cognition.

