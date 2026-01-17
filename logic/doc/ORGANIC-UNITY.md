**Organic Unity — Entity & Sublated Particulars**

- **Thesis:** an `Entity` is not a flat record of values; it is the unity of a Form and its sublated particulars — particulars that have been preserved, negated, and elevated (sublated) into the entity's dialectical presence.

- **Practical encoding:** add a `particulars` link-set on the entity shape. Each particular is an `EntityRef` (id + type) with optional relation metadata describing the dialectical role (e.g. `preserved`, `negated`, `sublated`). This keeps the Entity compact while making its dialectical lineage explicit.

- **Why this matters:** teaching, correction, and transformation operate on particulars. Exposing these links allows examples, audits, and pedagogy experiments to trace how an Entity emerged from—and continues to relate to—its empirical instances.

- **Minimal JSON example:**

```
{
  "id": "entity:person:alice",
  "type": "Person",
  "formId": "form:person:v1",
  "particulars": [
    { "id": "datum:survey:123", "type": "SurveyResponse", "role": "preserved" },
    { "id": "datum:log:456", "type": "ActivityLog", "role": "sublated" }
  ],
  "values": { "name": "Alice", "age": 42 }
}
```

- **Zod sketch (refer to `logic/src/schema/entity.ts`):**

```
import { EntityRef } from './entity';

export const EntityShapeSchema = z.object({
  id: z.string(),
  type: Type,
  formId: z.string(),
  particulars: z.array(EntityRef).optional(),
  values: z.record(z.string(), z.any()).optional()
});
```

- **Next steps:**
  - Add optional `role` metadata to `EntityRef` when richer tracing is needed.
  - Provide example transformers in `logic/examples` that show how particulars are ingested and sublated into Entities over iterative teaching rounds.

This note orients the repository toward explicit, traceable dyadic relations so pedagogy, validation, and transformation workflows can be implemented cleanly.

# Organic Unity - Dialectical Architecture

## The Principle

**Dyads form organic unity with reciprocating dyads.**

The system fulfills two dimensions of dialectical linking:

1. **Horizontal (Empirical)**: FormDB ↔ Model/Prisma (Rational ↔ Empirical)
2. **Vertical (Reflective)**: Form/Content ↔ Context/Property (Reflection on Form/Content)

## The Dialectical Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    REFLECTIVE SYSTEM                        │
│              (Vertical Dialectic)                           │
│                                                             │
│  Context ──────── Property                                 │
│    │                 │                                      │
│    │ (reflects on)   │ (reflects on)                       │
│    │                 │                                      │
│    ▼                 ▼                                      │
│  Form ──────────── Entity                                   │
│  (Rational)      (Appearance / Linkage)                     │
│    │                 │                                      │
│    │ (horizontal)    │ (horizontal)                        │
│    │                 │                                      │
│    ▼                 ▼                                      │
│  FormDB ────────── Model/Prisma                             │
│  (Neo4j)         (Postgres)                                 │
└─────────────────────────────────────────────────────────────┘
```

## Two Dimensions of Linking

### 1. Horizontal Dialectic (Empirical Dimension)

**Form ↔ Entity ↔ Model**

- **Form (Rational)**: Pure structure in FormDB (Neo4j)
  - FormShape: Fields, Layout, Sections, Actions
  - No empirical data, no AI metadata
  - The "blueprint" or schema

- **Entity (Appearance / Linkage)**: Transcendental handle in FormDB (Neo4j)
  - References Form via `formId`
  - Has `facets` and `signature` (JSON strings) for Organic Unity
  - Bridges to Model/Prisma via **references** in facets
  - Stores no authoritative empirical values (those are in Model/Postgres)

- **Model (Empirical Data)**: Actual values in Prisma/Postgres
  - Customer, Invoice, Product records
  - The actual data that Entity references

**The Reciprocation:**

- Form (Rational structure) → Entity (Empirical instance) → Model (Empirical data)
- Entity.facets bridges FormDB ↔ Model
- Entity.formId links back to Form (explicit Thinking-work)

Layering note:

- “Entity instance” here means **instance of appearance in the protocol stack**, not “instance row of the business database.”

### 2. Vertical Dialectic (Reflective Dimension)

**Form/Entity ↔ Context/Property**

- **Context**: Reflects on Form/Entity
  - Vertical relationship: Context reflects on the Form-Entity dyad
  - Provides contextual predicates, provenance
  - Versioned: Context version bump invalidates prior Property variations

- **Property**: Reflects on Context
  - Grounded in Context
  - Contains law/invariant structure
  - Mediates Entity ↔ Aspect relationships

**The Reciprocation:**

- Form/Entity (content) → Context (reflection on content) → Property (reflection on context)
- Vertical progression: Content → Reflection on Content → Reflection on Reflection

## The Organic Unity

### Dyad 1: Form ↔ Entity (Horizontal)

- Form = Rational structure (what could be)
- Entity = Appearance handle (what is, as linked)
- Reciprocation: Form provides structure, Entity instantiates it

### Dyad 2: Context ↔ Property (Vertical)

- Context = Reflection on Form/Entity
- Property = Reflection on Context
- Reciprocation: Context provides context, Property grounds it

### The Unity: Fourfold Reciprocation

```
Form (Rational Structure)
  ↕ (horizontal dialectic)
Entity (Empirical Instance)
  ↕ (vertical dialectic)
Context (Reflection on Form/Entity)
  ↕ (vertical dialectic)
Property (Reflection on Context)
```

## How Facets/Signatures Enable Organic Unity

### In Entity (Horizontal Bridge)

```typescript
Entity.facets = {
  // Bridge to Model/Prisma (horizontal)
  modelRef: { system: 'model', kind: 'prisma', table: 'Customer', id: 'customer-123' },
  // Optional: cache must be derived/non-authoritative
  cache: { version: 1 },

  // Bridge to Context/Property (vertical)
  contextId: "ctx-456",
  propertyRefs: [...],

  // Dialectical state
  dialecticState: {...}
}
```

### In Context (Vertical Reflection)

```typescript
Context.facets = {
  // Reflects on Form/Entity
  formRef: "invoice-form",
  entityRefs: ["inv-1", "inv-2"],

  // Provides context for Properties
  predicates: {...},
  provenance: {...}
}
```

### In Property (Vertical Grounding)

```typescript
Property.facets = {
  // Grounded in Context
  contextId: "ctx-456",

  // Law/invariant structure
  law: {
    invariants: [...],
    universality: "necessary"
  },

  // Mediates Entity ↔ Aspect
  mediates: {
    fromEntities: [...],
    toAspects: [...]
  }
}
```

## The Complete Flow

### Horizontal (Empirical)

1. **Form** (FormDB) - Rational structure
2. **Entity** (FormDB) - Empirical instance, references Form
3. **Model** (Prisma) - Empirical data, referenced by Entity.facets

### Vertical (Reflective)

1. **Form/Entity** - Content to be reflected upon
2. **Context** - Reflection on Form/Entity, provides predicates
3. **Property** - Reflection on Context, grounds laws/invariants

### The Unity

- **Form ↔ Entity**: Horizontal dialectic (Rational ↔ Empirical)
- **Context ↔ Property**: Vertical dialectic (Reflection)
- **Entity.facets**: Bridges both dimensions
- **Explicit Thinking-work**: No "fake immediacy" - all links are explicit

## Key Principles

1. **No Fake Immediacy**: All relationships are explicit (formId, contextId, etc.)
2. **Separation of Concerns**:
   - FormDB (Neo4j) = Rational structure + transcendental linkage
   - Model (Postgres) = Empirical data
   - Facets/Signatures = Organic Unity bridges
3. **Dialectical Progression**:
   - Horizontal: Form → Entity → Model
   - Vertical: Form/Entity → Context → Property
4. **Reciprocation**: Each dyad reciprocates with its partner
   - Form reciprocates with Entity
   - Context reciprocates with Property
   - Together they form Organic Unity

## Architecture Summary

```
FormDB (Neo4j) - Rational Structure
├─ FormShape (pure structure, no facets/signature)
├─ Entity (has facets/signature for Organic Unity)
├─ Context (reflects on Form/Entity)
└─ Property (reflects on Context)

Model (Prisma/Postgres) - Empirical Data
└─ Customer, Invoice, Product (actual values)

Organic Unity Bridges
├─ Entity.facets → Model (horizontal)
└─ Entity.facets → Context/Property (vertical)
```

This structure ensures:

- ✅ Form/Content separation (Rational ↔ Empirical)
- ✅ Reflection on Form/Content (Context/Property)
- ✅ Organic Unity through reciprocating dyads
- ✅ Explicit Thinking-work (no fake immediacy)
