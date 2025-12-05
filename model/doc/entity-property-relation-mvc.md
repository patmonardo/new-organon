# Entity:Property:Relation in MVC DSL

## Overview

**Entity:Property:Relation is part of the Form Processor API** (in @logic), but we **surface it as part of the MVC DSL** so that FormApps can work with it directly.

This maintains **"Everything is a Form"** while allowing MVC to work with the **concrete existence structure** (Entity:Property:Relation).

## The Problem

**Form Processor moved to @logic**, but we need to:
1. **Hold on to "Everything is a Form"** - FormShape is fundamental
2. **Not let it go easily** - How MVC uses Shape:Context:Morph Machinery
3. **Surface Entity:Property:Relation** - As part of MVC DSL and FormApp abstraction

## The Solution

**Surface Entity:Property:Relation in MVC DSL:**
- Entity:Property:Relation is part of Form Processor API
- But we expose it through MVC DSL
- FormApps can work with it directly
- Still maintains "Everything is a Form" via FormShape references

## Architecture

```
Logic Form Processor (@logic)
    ├─→ Entity:Property:Relation (Form Processor API)
    └─→ Shape:Context:Morph (Form Processing)
    ↓
MVC DSL (@model)
    ├─→ Entity:Property:Relation (Surfaced)
    ├─→ FormShape (Everything is a Form)
    └─→ FormApp Abstraction
```

## Entity:Property:Relation Structure

### Entity

**Entity represents:**
- Instances of Shapes
- Concrete existence
- FormShape reference (Everything is a Form)

**Schema:**
```typescript
Entity {
  id: string;
  type: string;
  name?: string;
  properties?: PropertyRef[];
  relations?: RelationRef[];
  formShape?: FormShape; // Everything is a Form
}
```

### Property

**Property represents:**
- Contextualized predicates/measures
- Qualitative and quantitative characteristics
- FormShape reference (Everything is a Form)

**Schema:**
```typescript
Property {
  id: string;
  type: string;
  name: string;
  entityId?: string;
  contextId?: string;
  qualitative?: {
    essential?: boolean;
    observable?: boolean;
    mutable?: boolean;
    inherent?: boolean;
  };
  quantitative?: {
    dataType?: string;
    unit?: string;
    precision?: number;
    range?: { min?: any; max?: any };
  };
  formShape?: FormShape; // Everything is a Form
}
```

### Relation

**Relation represents:**
- Essential/Absolute ties
- Entity connections
- FormShape reference (Everything is a Form)

**Schema:**
```typescript
Relation {
  id: string;
  type: string;
  sourceId: string;
  targetId: string;
  direction?: 'directed' | 'undirected' | 'bidirectional';
  essential?: boolean;
  absolute?: boolean;
  formShape?: FormShape; // Everything is a Form
}
```

## Integration with FormShape

**Everything is a Form:**
- Entity has `formShape?: FormShape`
- Property has `formShape?: FormShape`
- Relation has `formShape?: FormShape`

**This maintains the maxim** while allowing MVC to work with Entity:Property:Relation.

## Usage in FormApp

### Example: Customer Entity

```typescript
// Create Entity:Property:Relation
const customerEntity = createEntity({
  id: 'customer-1',
  type: 'Customer',
  name: 'Acme Corp',
  properties: [
    createProperty({
      id: 'prop-1',
      type: 'name',
      name: 'Company Name',
      entityId: 'customer-1',
    }),
  ],
  relations: [
    createRelation({
      id: 'rel-1',
      type: 'has_invoice',
      sourceId: 'customer-1',
      targetId: 'invoice-1',
    }),
  ],
});

// Use in FormApp
const app = createApplication({
  name: 'Customer App',
  dashboard: {
    id: 'dashboard-1',
    name: 'Customer Dashboard',
    sections: [
      {
        id: 'section-1',
        type: 'form',
        content: customerEntity.formShape, // Everything is a Form
      },
    ],
  },
});
```

## Shape:Context:Morph Machinery

**MVC still uses Shape:Context:Morph:**
- FormShape (Pure Form)
- Context (Transactional Environment)
- Morph (Organic Unity)

**But also works with Entity:Property:Relation:**
- Entity (Concrete Existence)
- Property (Contextualized Predicates)
- Relation (Essential Ties)

**Both are available** in MVC DSL.

## FormApp Abstraction

**FormApp can work with:**
1. **FormShape** - Pure form definitions
2. **Entity:Property:Relation** - Concrete existence
3. **Shape:Context:Morph** - Form processing machinery

**All three are available** through MVC DSL.

## Key Insights

1. **Entity:Property:Relation is part of Form Processor API** - But surfaced in MVC DSL
2. **Everything is a Form** - Maintained via FormShape references
3. **Shape:Context:Morph** - Still used by MVC
4. **FormApp abstraction** - Works with all three
5. **MVC DSL** - Provides unified interface

## Next Steps

- [ ] Integrate Entity:Property:Relation into FormApp examples
- [ ] Create adapters from Logic Form Processor to MVC DSL
- [ ] Document Shape:Context:Morph usage in MVC
- [ ] Build FormApp examples using Entity:Property:Relation

