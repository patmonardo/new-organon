# Malloy's React Components Approach

## What Malloy Does

### Malloy Provides React Components (Not JSX Spec)

**Malloy Publisher SDK:**
- Collection of **reusable React components**
- Designed for building UIs that interact with Malloy queries
- Integrates with existing React/JSX workflows
- **No new JSX specification**

**Malloy Renderer:**
- Web component for rendering Malloy query results
- Can be embedded in applications
- Works with standard React/JSX

### Key Insight

**Malloy does NOT introduce its own JSX specification.**

Instead:
- ✅ Provides React components
- ✅ Integrates with existing React/JSX
- ✅ Uses standard React patterns
- ❌ Does NOT create new JSX spec

## Our Approach (Already Aligned)

### We Already Have React Components

**Current Structure:**
```
model/src/ui/react/
├── button/     # ButtonRenderer, LinkRenderer
├── card/       # CardRenderer, StatCardRenderer
├── list/       # ListRenderer, BreadcrumbsRenderer
├── search/     # SearchRenderer
└── table/      # TableRenderer
```

**React Adapters:**
```
model/src/sdsl/
├── react-adapter.tsx        # DisplayDocument → React
├── react-shape-adapter.tsx  # Shape → React (preferred)
└── react-view.tsx           # FormView → React
```

### Shape-Based Rendering (Our Innovation)

**We use Shape objects, not DisplayDocument:**
```typescript
// Shape → React (our approach)
const buttonShape: ButtonShape = { type: 'button', label: 'Click' };
const jsx = renderShape(buttonShape);
```

**This is similar to Malloy's approach:**
- Malloy: Query results → React components
- Us: Shape objects → React components

## Architecture Comparison

### Malloy's Approach

```
Malloy Query → Query Results → React Components → UI
```

**Components:**
- `@malloy-publisher/sdk` - React components for Malloy
- `@malloydata/render` - Web component for results

### Our Approach

```
SDSL Query → Query Results → Shapes → React Components → UI
```

**Components:**
- `model/src/ui/react/` - React components for Shapes
- `model/src/sdsl/react-shape-adapter.tsx` - Shape → React adapter

## Key Differences

### Malloy
- **Query-focused**: Components render query results
- **Publisher SDK**: React components for Malloy queries
- **Renderer**: Web component for results

### Us
- **Shape-focused**: Components render Shape objects
- **React Components**: Direct Shape renderers
- **Adapters**: Shape → React transformation

## What This Means for Malloy-Lite

### We Should Provide React Components

**For Malloy-Lite views:**
```typescript
// Malloy-Lite view spec
const revenueView: ViewSpec = {
  id: 'revenue-by-region',
  model: 'Customer',
  group_by: ['region'],
  aggregate: ['totalRevenue'],
};

// Execute and render
const view = customerModel.viewFromSpec(revenueView);
const result = await engine.execute(view);

// Render with React component
<MalloyViewRenderer view={view} result={result} />
```

### React Components for Malloy-Lite

**New Components:**
```typescript
// model/src/ui/react/malloy/
├── view-renderer.tsx      # Renders MalloyView results
├── dashboard-renderer.tsx # Renders MalloyDashboard
└── chart-renderer.tsx     # Renders charts from views
```

**Usage:**
```tsx
import { MalloyViewRenderer } from '@organon/model/ui/react/malloy';

function Dashboard() {
  const view = customerModel.viewFromSpec(revenueView);
  const result = await engine.execute(view);
  
  return <MalloyViewRenderer view={view} result={result} />;
}
```

## Recommendation

### Follow Malloy's Pattern: React Components, Not JSX Spec

**Why:**
1. ✅ **Standard React/JSX** - No new spec to learn
2. ✅ **Reusable components** - Can be used in any React app
3. ✅ **Flexible** - Can customize with standard React patterns
4. ✅ **Ecosystem** - Works with existing React tooling

**What to Build:**
1. **MalloyViewRenderer** - React component for rendering views
2. **MalloyDashboardRenderer** - React component for dashboards
3. **MalloyChartRenderer** - React component for charts
4. **Keep existing Shape renderers** - They work great

### Architecture

```
Malloy-Lite ViewSpec
    ↓
Execute (engine)
    ↓
Query Result
    ↓
MalloyViewRenderer (React Component)
    ↓
React UI
```

**Plus existing:**
```
Shape Objects
    ↓
React Shape Renderers (existing)
    ↓
React UI
```

## Implementation Plan

### Phase 1: Malloy-Lite React Components

**New components:**
- `MalloyViewRenderer` - Renders view results as table/chart
- `MalloyDashboardRenderer` - Renders dashboard with multiple views
- `MalloyChartRenderer` - Renders charts from view results

**Location:**
```
model/src/ui/react/malloy/
├── view-renderer.tsx
├── dashboard-renderer.tsx
└── chart-renderer.tsx
```

### Phase 2: Integration

**Use with existing:**
- React Dashboard can use MalloyViewRenderer
- Can mix Shape renderers with Malloy renderers
- Gradual migration path

## Conclusion

**Malloy provides React components, not a JSX spec.**

**We should do the same:**
- ✅ Provide React components for Malloy-Lite
- ✅ Use standard React/JSX
- ✅ Integrate with existing React components
- ❌ Don't create new JSX specification

**Our advantage:**
- We already have React components (Shape renderers)
- We can add Malloy-Lite renderers alongside
- Both use standard React/JSX

---

**Key Insight**: Malloy uses React components, not a JSX spec. We should follow the same pattern - provide React components for Malloy-Lite views, using standard React/JSX.

