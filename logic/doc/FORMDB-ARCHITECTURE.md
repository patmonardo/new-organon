# FormDB Architecture - Tags, Facets, Signatures

## Overview

FormDB (Neo4j) stores **Rational structure** - the blueprint/schema. The core structure (FormField, FormLayout, FormSection, FormAction) is **protected** and kept separate from AI-generated metadata.

Critical clarification (layering):
- **Entity in FormDB is not an empirical domain model.** It must not store the “Customer/Invoice/Product record” itself.
- Entity is a **transcendental appearance handle**: it carries principled linkage (`formId`), dialectical state (facets), and an operational interface (signature).
- Empirical particulars (e.g. `email`, `phone`, `amount`) live in the Model layer (Prisma/Postgres). FormDB may store only **references** (IDs) and **non-authoritative derived metadata**.

## Three Layers of Metadata

### 1. **FormTags** (Separate Nodes)
- **Storage**: Separate `:FormTag` nodes with `HAS_TAG` relationships
- **Structure**: `{ value: any, label: string }`
- **Purpose**: Simple categorization (e.g., "billing", "financial", "crm")
- **Isolation**: ✅ **Completely separate** from FormField/FormLayout structure
- **Use Case**: Filtering, grouping, search

```cypher
// Tags are separate nodes
(FormShape)-[:HAS_TAG]->(FormTag {value: "billing", label: "Billing"})
```

### 2. **Facets** (JSON Blob)
- **Storage**: Stored as JSON string in Neo4j node property
- **Structure**: `Record<string, unknown>` - free-form object
- **Purpose**: Non-authoritative metadata, UI hints, cached projections, derived flags
- **Isolation**: ✅ **Stored as JSON string** - doesn't affect Neo4j schema
- **Use Case**: 
  - Dialectical state (for Entity)
  - UI hints (`ui.*`, `calc.*`, `cache.*`)
  - Annotations that can be dropped/rebuilt
  - **Organic Unity** via references to Model/Prisma (never authoritative empirical records)

```typescript
// Stored as JSON string in Neo4j
props = {
  facets: JSON.stringify({
    dialecticState: {...},
    ui: { hints: [...] },
    cache: { version: 1 },
    // Optional bridge: references only (IDs/keys), not the empirical record
    modelRef: { system: 'model', kind: 'prisma', table: 'Customer', id: 'customer-123' }
  })
}
```

### 3. **Signatures** (JSON Blob)
- **Storage**: Stored as JSON string in Neo4j node property  
- **Structure**: `Record<string, unknown>` - operational interface
- **Purpose**: Capabilities/claims - what this form/entity can do
- **Isolation**: ✅ **Stored as JSON string** - doesn't affect Neo4j schema
- **Use Case**: Operational interface, capabilities, claims

```typescript
// Stored as JSON string in Neo4j
props = {
  signature: JSON.stringify({
    canSubmit: true,
    canDelete: false,
    operations: [...]
  })
}
```

## Core Structure (Protected)

The **core structure** is stored as proper Neo4j nodes/relationships:

```
FormShape (Rational - Substance)
  ├─ HAS_FIELD → FormField
  │   └─ HAS_OPTION → FormOption
  ├─ HAS_LAYOUT → FormLayout
  │   ├─ HAS_SECTION → FormSection
  │   │   └─ CONTAINS_FIELD → FormField
  │   └─ HAS_ACTION → FormAction
  └─ HAS_TAG → FormTag (separate, not part of core)
  ❌ NO facets/signature - Pure Substance

Entity (Empirical - Appearance)
  └─ formId: string (reference to FormShape)
  └─ facets: JSON string (dialectical state, UI hints, Model bridges)
  └─ signature: JSON string (operational interface)

Entity note:
- “Empirical” here means **appearance/instance-ness**, not “store the empirical dataset.”
- The Entity node is a **protocol-layer object**: it should not leak Particular model fields into the Universal layer.

Context (Reflective - Scope)
  └─ facets: JSON string (predicates, provenance, references)
  └─ signature: JSON string (contextual operations)
  └─ state: JSON string (runtime state)
  └─ entities: JSON string (entity references)
  └─ relations: JSON string (relation references)

Property (Reflective - Law)
  └─ facets: JSON string (law structure, invariants, mediation)
  └─ signature: JSON string (property operations)
  └─ HAS_TAG → Tag (categorization)

Aspect (Spectral - Relation)
  └─ facets: JSON string (spectrum, essential relations, appearing)
  └─ signature: JSON string (relational operations)

Morph (Ground - Transformation)
  └─ facets: JSON string (container, transformation)
  └─ signature: JSON string (transform operations)
  └─ composition: JSON string (pipeline/composite structure)
```

**Key Point**: `FormShape` does NOT have facets or signature - it's pure Rational structure (Substance). All other pillars (Entity, Context, Property, Aspect, Morph) have facets/signature for their dialectical roles and Organic Unity.

## How They're Stored in Neo4j

### FormShape Node (Pure Rational Structure)
```cypher
CREATE (fs:FormShape {
  id: "...",
  name: "...",
  title: "...",
  description: "...",
  // Core structure properties only
  // NO facets/signature - FormShape is pure structure
})
// Then create separate nodes for Fields, Layout, Tags
```

### Entity Node (Appearance)
```cypher
CREATE (e:Entity {
  id: "...",
  formId: "...",
  // Facets and Signature stored as JSON strings
  facets: '{"dialecticState": {...}, "ui": {...}, "prismaModelId": "..."}',
  signature: '{"canSubmit": true, "canDelete": false, ...}'
})
```

### Context Node (Reflection)
```cypher
CREATE (c:Context {
  id: "...",
  type: "...",
  // Facets and Signature stored as JSON strings
  facets: '{"predicates": {...}, "provenance": {...}, "formRef": "..."}',
  signature: '{"operations": [...]}',
  state: '{"...": "..."}',
  entities: '[...]',
  relations: '[...]'
})
```

### Property Node (Law)
```cypher
CREATE (p:Property {
  id: "...",
  type: "...",
  // Facets and Signature stored as JSON strings
  facets: '{"law": {...}, "invariants": [...], "mediates": {...}}',
  signature: '{"operations": [...]}',
  status: "...",
  meta: '{"...": "..."}'
})
```

### Aspect Node (Spectral Relation)
```cypher
CREATE (a:Aspect {
  id: "...",
  type: "...",
  // Facets and Signature stored as JSON strings
  facets: '{"spectrum": {...}, "essentialRelation": {...}, "appearing": {...}}',
  signature: '{"relationalOps": [...]}',
  status: "...",
  meta: '{"...": "..."}'
})
```

### Morph Node (Ground)
```cypher
CREATE (m:Morph {
  id: "...",
  type: "...",
  // Facets and Signature stored as JSON strings
  facets: '{"container": {...}, "transformation": {...}}',
  signature: '{"transformOps": [...]}',
  composition: '{"kind": "pipeline", "steps": [...]}',
  config: '{"...": "..."}',
  meta: '{"...": "..."}'
})
```

## Key Points

1. **FormField/FormLayout are protected** - stored as proper Neo4j nodes, not JSON
2. **Tags are separate nodes** - don't interfere with core structure
3. **Facets/Signatures are JSON blobs** - stored as strings, don't affect Neo4j schema
4. **Organic Unity** - Facets/Signatures can bridge to Model/Prisma models without touching FormDB structure

## Usage in Organic Unity

You can use Facets/Signatures to:
- Store references to Prisma model IDs
- Cache computed values from Model layer
- Store UI state that syncs with Model
- Bridge FormDB (Rational) ↔ Model (Empirical)

**Example:**
```typescript
// In Entity facets - bridge to Prisma
facets: {
  prismaModelId: "customer-123",  // Reference to Prisma Customer
  ui: {
    lastViewed: "2024-01-15",
    computedFields: {...}
  },
  cache: {
    relatedInvoices: ["inv-1", "inv-2"]  // Cached from Model queries
  }
}
```

## Facets/Signatures Across All Pillars

### The Complete Picture

**FormShape** (Rational - Substance)
- ❌ **NO facets/signature** - Pure structure, the essential blueprint
- **Role**: The Substance - the essential structure that defines what a form IS
- **Philosophy**: Pure noumenal structure, no accidental properties

**Entity** (Empirical - Appearance)
- ✅ **HAS facets/signature** - The Appearance of FormShape
- **Role**: The Effect of a Particular Shape - empirical instantiation
- **Philosophy**: "Essence as Thing is Appearance" - Entity is the appearance of FormShape
- **Facets**: Dialectical state, UI hints, Model/Prisma bridges, runtime metadata
- **Signature**: Operational interface (what this entity can do)

**Context** (Reflective - Scope)
- ✅ **HAS facets/signature** - Reflection on Form/Entity
- **Role**: Defines scope and conditions for dialectical logic
- **Philosophy**: Reflection on the horizontal (Form ↔ Entity)
- **Facets**: Predicates, provenance, entity/relation references
- **Signature**: Contextual operations

**Property** (Reflective - Law)
- ✅ **HAS facets/signature** - Law/Invariant as Middle Term
- **Role**: Mediates Entity ↔ Aspect, grounds laws/invariants
- **Philosophy**: Reflection on Context - the conditional genesis
- **Facets**: Law structure, invariants, mediation rules
- **Signature**: Property operations

**Aspect** (Spectral - Relation)
- ✅ **HAS facets/signature** - Spectral Theory of Relation
- **Role**: How essences appear in existence (spectrum of polarities)
- **Philosophy**: Essential relation as spectrum of appearing
- **Facets**: Spectrum, essential relations, appearing modes
- **Signature**: Relational operations

**Morph** (Ground - Transformation)
- ✅ **HAS facets/signature** - Ground = Shape + Context
- **Role**: Active unity that holds moments, enables transformation
- **Philosophy**: The Ground that unifies Shape and Context
- **Facets**: Container structure, transformation principles
- **Signature**: Transform/pipeline operations

## Philosophical Mapping: Substance & Appearance

### Hegel's Dialectic: Essence as Thing is Appearance

**FormShape = Substance** (The Essential)
- Pure rational structure
- No accidental properties
- The "what IS" - the essential blueprint
- Protected from runtime mutations

**Entity = Appearance** (The Effect)
- Empirical instantiation of FormShape
- The "what APPEARS" - the effect of a particular shape
- **Facets = Accidental Properties** to Shape's Substance
- Runtime games of persistent principled design
- Particular Design artifacts

### The Dialectical Unity

```
FormShape (Substance - Essential)
    ↓ (causation)
Entity (Appearance - Effect)
    ↓ (reflection)
Context (Reflection on Form/Entity)
    ↓ (grounding)
Property (Law/Invariant)
    ↓ (spectral)
Aspect (Relation Appearance)
    ↓ (ground)
Morph (Transformation)
```

**Key Insight**: Facets handle **Accidental Properties** to Shape's **Substance**. Entity is the **Appearance** of FormShape as the **Effect** of a Particular Shape. This is "Essence as Thing is Appearance" - the noumenal structure (FormShape) manifests as phenomenal appearance (Entity) through facets.

## Summary

✅ **FormShape is pure Substance**: No facets/signature - just Rational structure (Fields, Layout, Tags)  
✅ **All other Pillars have facets/signature**: Entity, Context, Property, Aspect, Morph all store accidental properties  
✅ **Entity = Appearance**: Facets/Signatures enable "runtime games of persistent principled design"  
✅ **Tags are separate**: FormTag nodes don't interfere with core structure  
✅ **Clean separation**: 
   - FormShape (Rational/Substance) = Pure structure, no accidental properties
   - Entity (Empirical/Appearance) = Has facets/signature for Model integration + dialectical state
   - Context/Property/Aspect/Morph = All have facets/signature for their dialectical roles
   - Tags = Separate nodes for categorization

**Your FormField/FormLayout are completely protected from AI-generated metadata!**

**Facets enable "Particular Design artifacts" - runtime games of persistent principled design where accidental properties attach to essential structure without corrupting the Substance.**

