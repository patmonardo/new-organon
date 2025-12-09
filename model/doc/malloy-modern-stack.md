# Malloy: Modern TS-Based Stack

## Why Malloy is Exploitable

### Modern TypeScript Foundation

**Malloy is TypeScript-based:**
- ✅ Type-safe query definitions
- ✅ Modern tooling integration (VSCode extension)
- ✅ Strong typing for measures/dimensions
- ✅ Compile-time validation
- ✅ Excellent developer experience

**This aligns perfectly with our approach:**
- We're TypeScript-based (SDSL)
- We can leverage Malloy's type system
- We can integrate with Malloy tooling
- We can share type definitions

### Looker vs Malloy

**Looker (Old):**
- From the dawn of BI and Data Science
- Older architecture
- Less modern tooling

**Malloy (Modern):**
- Built for modern development
- TypeScript-based
- VSCode extension
- Modern query language
- Exploitable for our use case

## Our Malloy-Lite Approach

### TypeScript Integration

**We can leverage Malloy's type system:**
```typescript
// Malloy-Lite ViewSpec (TypeScript)
interface ViewSpec {
  id: string;
  model: string;
  group_by?: string[];
  aggregate?: string[];
  // Type-safe definitions
}
```

**Type-safe execution:**
```typescript
// Type-safe view execution
const view = customerModel.viewFromSpec(revenueView);
const result: QueryResult = await engine.execute(view);
```

### Modern Tooling

**VSCode Integration:**
- Malloy has VSCode extension
- We can provide similar tooling
- Type checking for ViewSpec
- Autocomplete for measures/dimensions

**Development Experience:**
- Type-safe query definitions
- Compile-time validation
- IntelliSense support
- Error checking

## Architecture: ui/react vs ui/malloy

### ui/react/ - Basic Dashboard Components

**Purpose:** Basic, reusable dashboard components

**Components:**
- Button, Card, List, Table, Search
- Framework-agnostic shapes
- Can be used anywhere

**Usage:**
```tsx
import { ButtonRenderer, CardRenderer } from '@organon/model/ui/react';

<ButtonRenderer shape={buttonShape} />
<CardRenderer shape={cardShape} />
```

### ui/malloy/ - Malloy-Specific Components

**Purpose:** Malloy-specific components for views/dashboards

**Components:**
- ViewRenderer - Renders Malloy views
- DashboardRenderer - Renders Malloy dashboards
- ChartRenderer - Renders charts from views

**Usage:**
```tsx
import { ViewRenderer, DashboardRenderer } from '@organon/model/ui/malloy';

<ViewRenderer view={viewSpec} result={queryResult} />
<DashboardRenderer dashboard={dashboardSpec} results={viewResults} />
```

## Why This Separation?

### Clear Boundaries

**ui/react/** = Basic building blocks
- Reusable across any use case
- Framework-agnostic
- Shape-based rendering

**ui/malloy/** = Malloy-specific
- Malloy query results
- Malloy ViewSpec integration
- Malloy dashboard layouts

### Benefits

1. **Separation of Concerns**: Basic components vs Malloy-specific
2. **Reusability**: Basic components can be used without Malloy
3. **Clarity**: Clear what's basic vs Malloy-specific
4. **Maintainability**: Easier to maintain and extend

## Implementation Plan

### Phase 1: Structure

**Created:**
- `ui/malloy/` directory
- `ui/malloy/README.md` - Documentation
- `ui/malloy/index.ts` - Exports (placeholder)

### Phase 2: Components

**To Build:**
- `view-renderer.tsx` - ViewRenderer component
- `dashboard-renderer.tsx` - DashboardRenderer component
- `chart-renderer.tsx` - ChartRenderer component

### Phase 3: Integration

**Integration Points:**
- Use with Malloy-Lite ViewSpec
- Execute via execution engines
- Render with React components

## Malloy's Modern Stack Benefits

### Type Safety

**Malloy provides:**
- Type-safe query definitions
- Compile-time validation
- Strong typing

**We leverage:**
- TypeScript for ViewSpec
- Type-safe execution
- Type-safe rendering

### Developer Experience

**Malloy provides:**
- VSCode extension
- Autocomplete
- Error checking

**We can provide:**
- Similar tooling for SDSL
- Type checking for ViewSpec
- IntelliSense support

### Modern Architecture

**Malloy:**
- Modern query language
- TypeScript-based
- Modern tooling

**Us:**
- TypeScript-based SDSL
- Modern React components
- Modern execution engines

## Conclusion

**Malloy is modern and exploitable:**
- ✅ TypeScript-based (aligns with us)
- ✅ Modern tooling (we can leverage)
- ✅ Type-safe (we can use)
- ✅ Modern architecture (we can learn from)

**Our approach:**
- Separate `ui/react/` (basic) from `ui/malloy/` (Malloy-specific)
- Leverage Malloy's type system
- Build modern TypeScript-based components
- Provide excellent developer experience

---

**Key Insight**: Malloy is modern and TypeScript-based, making it highly exploitable. We separate basic components (`ui/react/`) from Malloy-specific components (`ui/malloy/`) for clarity and maintainability.

