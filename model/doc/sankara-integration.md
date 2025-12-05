# Sankara Form Theory Integration

## Overview

We've salvaged key concepts from `/archive/sankara/app/form` that extended the dashboard into a general form theory. These concepts have been integrated into the MVC schema system.

## Key Concepts Salvaged

### 1. Dashboard Extends FormShape

**From sankara:** Dashboard is a type of FormShape with grid-based positioning.

**Integration:**
- `DashboardShapeSchema` extends `FormShapeSchema`
- Grid-based positioning system (x, y, w, h)
- Component-based architecture

**Location:** `model/src/schema/dashboard.ts`

### 2. DisplayShape - Presentation Model

**From sankara:** `DisplayShape` is the output of the form transformation pipeline, ready for rendering.

**Integration:**
- `DisplayShapeSchema` - Presentation model
- `FieldDisplayShapeSchema` - Single field display
- Standard components registry

**Location:** `model/src/schema/display.ts`

### 3. Morph Pipeline System

**From sankara:** Transformation pipeline system with pure/fusible/memoizable morphs.

**Integration:**
- `Morph` - Core transformation interface
- `MorphPipeline` - Sequence of transformations
- `MorphOptions` - Configuration (pure, fusible, memoizable, cost)
- `composeMorphs` - Composition helper

**Location:** `model/src/schema/morph.ts`

### 4. Grid-Based Positioning

**From sankara:** Position schema with grid coordinates (x, y, w, h).

**Integration:**
- `PositionSchema` - Grid positioning
- Used in `DashboardComponentBaseSchema`
- Supports responsive grid layouts

**Location:** `model/src/schema/dashboard.ts`

### 5. Component-Based Architecture

**From sankara:** Dashboard components as first-class citizens.

**Integration:**
- `DashboardComponentBaseSchema` - Base component
- `StatCardSchema` - Stat cards
- `ContainerSchema` - Containers
- `ConceptCloudSchema` - Concept clouds
- `ExplorationsListSchema` - Exploration lists

**Location:** `model/src/schema/dashboard.ts`

## Integration Points

### Application Schema

**Enhanced with:**
- `DashboardShapeSchema` instead of simple `DashboardSchema`
- `DisplayShape` in Views
- `DashboardComponent` support

**Location:** `model/src/schema/application.ts`

### Form Theory

**Everything is a Form:**
- Dashboard extends FormShape
- Components are forms
- DisplayShape is form output

**Maintained through:**
- `DashboardShapeSchema` extends `FormShapeSchema`
- Components reference FormShape
- DisplayShape references FormShape

## Architecture

```
FormShape (Base)
    ↓
DashboardShape (Extends FormShape)
    ├─→ Grid-based positioning
    ├─→ Component-based architecture
    └─→ DisplayShape output
    ↓
Morph Pipeline
    ├─→ Pure transformations
    ├─→ Fusible transformations
    └─→ Memoizable transformations
    ↓
DisplayShape
    ├─→ Presentation model
    └─→ Ready for rendering
```

## Key Files

### Schemas
- `model/src/schema/dashboard.ts` - Dashboard schema (already existed, enhanced)
- `model/src/schema/display.ts` - DisplayShape schema (new)
- `model/src/schema/morph.ts` - Morph pipeline system (new)
- `model/src/schema/application.ts` - Application schema (enhanced)

### Documentation
- `model/doc/sankara-integration.md` - This document

## Usage Examples

### Creating a Dashboard

```typescript
import { DashboardShapeSchema } from '@model/schema';

const dashboard = DashboardShapeSchema.parse({
  id: 'my-dashboard',
  name: 'My Dashboard',
  type: 'dashboard',
  fields: [],
  state: { status: 'idle' },
  layout: {
    title: 'My Dashboard',
    gridColumns: 12,
  },
  components: [
    {
      id: 'stat-1',
      type: 'stat-card',
      title: 'Total Users',
      position: { x: 0, y: 0, w: 3, h: 1 },
      value: 1000,
      label: 'users',
    },
  ],
});
```

### Creating a Morph Pipeline

```typescript
import { createMorph, composeMorphs } from '@model/schema';

const transformA = createMorph('transformA', (input) => {
  // Transform input
  return transformed;
}, { pure: true, memoizable: true });

const transformB = createMorph('transformB', (input) => {
  // Transform input
  return transformed;
}, { pure: true, memoizable: true });

const pipeline = composeMorphs(transformA, transformB);
const result = pipeline.transform(input);
```

### Creating a DisplayShape

```typescript
import { DisplayShapeSchema } from '@model/schema';

const display = DisplayShapeSchema.parse({
  type: 'form',
  component: 'FormView',
  props: {
    id: 'form-1',
    fields: [
      {
        id: 'field-1',
        component: 'TextField',
        props: { label: 'Name' },
      },
    ],
    mode: 'view',
  },
});
```

## Next Steps

- [ ] Integrate Morph pipeline into Form processing
- [ ] Create DisplayShape renderers
- [ ] Build Dashboard component renderers
- [ ] Document Morph pipeline usage patterns
- [ ] Create examples of Morph transformations

## Key Insights

1. **Dashboard extends FormShape** - Everything is a Form
2. **DisplayShape** - Presentation model ready for rendering
3. **Morph Pipeline** - Pure/fusible/memoizable transformations
4. **Grid-based positioning** - Responsive layouts
5. **Component-based** - First-class component architecture

