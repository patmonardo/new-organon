# Malloy BI Enhancement Plan: Concrete Implementation

## Overview

This document provides concrete implementation steps to enhance the SDSL (Malloy-inspired) layer for full BI capabilities as a "Secondary Target for Logic" - using secondary empirical concepts within the larger AI Platform.

## Current State Summary

### What We Have
- ✅ Basic `DataModel` with measures, dimensions, joins
- ✅ `DataView` for query definitions
- ✅ `SemanticHydrator` bridging data to forms
- ✅ `PolarsExecutionEngine` for execution
- ✅ Dashboard schema with components
- ✅ Application schema with View references

### What We Need
- ❌ Full Malloy View structure (nested views, parameters, turtles)
- ❌ Dashboard-View integration (Dashboard as View collection)
- ❌ Enhanced SDSL with primary_key, foreign_key, where filters
- ❌ Malloy query compilation path
- ❌ View-to-Dashboard component mapping

## Implementation Plan

### Phase 1: Enhance SDSL Core (`model/src/data/sdsl.ts`)

#### 1.1 Add Primary/Foreign Key Support

```typescript
export interface DataModelConfig<T extends Record<string, any>> {
  name: string;
  source: string | SourceDefinition; // Support structured source
  primary_key?: string | string[]; // Single or composite key
  fields: { [K in keyof T]: z.ZodType<T[K]> };
  measures?: Record<string, MeasureDefinition>;
  dimensions?: Record<string, DimensionDefinition | string>;
  joins?: Record<string, JoinDefinition>;
  where?: FilterDefinition; // Model-level filters
  sql?: string; // Custom SQL blocks (Malloy feature)
}

export interface SourceDefinition {
  type: 'table' | 'sql' | 'connection';
  value: string;
  connection?: string; // For connection-based sources
}

export interface FilterDefinition {
  field: string;
  operator: '=' | '!=' | '>' | '<' | '>=' | '<=' | 'in' | 'not in' | 'like' | 'is null' | 'is not null';
  value: any;
  logic?: 'and' | 'or'; // For compound filters
}
```

#### 1.2 Enhance ViewQuery for Malloy Features

```typescript
export interface ViewQuery {
  // Basic query
  group_by?: string[];
  aggregate?: string[];
  filter?: Record<string, any> | FilterDefinition[];
  limit?: number;
  order_by?: OrderDefinition[];
  
  // Malloy-specific features
  turtle?: TurtleDefinition[]; // Nested aggregations
  reduce?: string[]; // Simplified aggregation
  nest?: NestDefinition[]; // Nested structures
  parameters?: ViewParameter[]; // View parameters
  where?: FilterDefinition[]; // View-level filters
}

export interface TurtleDefinition {
  name: string;
  group_by?: string[];
  aggregate?: string[];
  limit?: number;
}

export interface NestDefinition {
  name: string;
  group_by?: string[];
  aggregate?: string[];
  limit?: number;
}

export interface ViewParameter {
  name: string;
  type: 'string' | 'number' | 'date' | 'boolean';
  default?: any;
}

export interface OrderDefinition {
  field: string;
  direction: 'asc' | 'desc';
}
```

#### 1.3 Add Explore Concept (Malloy's Primary Query)

```typescript
export class DataModel<T extends Record<string, any>> {
  constructor(public config: DataModelConfig<T>) {}

  // Malloy's "explore" - primary query mechanism
  explore(query?: Partial<ViewQuery>): DataView {
    return new DataView(this, query || {});
  }

  // Existing view method (backward compatible)
  view(query: ViewQuery): DataView {
    return new DataView(this, query);
  }
}
```

### Phase 2: Create Malloy View Schema (`model/src/schema/malloy-view.ts`)

#### 2.1 Malloy View Schema

```typescript
import { z } from 'zod';

export const MalloyViewParameterSchema = z.object({
  name: z.string(),
  type: z.enum(['string', 'number', 'date', 'boolean']),
  default: z.any().optional(),
});

export const MalloyFilterSchema = z.object({
  field: z.string(),
  operator: z.enum(['=', '!=', '>', '<', '>=', '<=', 'in', 'not in', 'like', 'is null', 'is not null']),
  value: z.any(),
  logic: z.enum(['and', 'or']).optional(),
});

export const MalloyTurtleSchema = z.object({
  name: z.string(),
  group_by: z.array(z.string()).optional(),
  aggregate: z.array(z.string()).optional(),
  limit: z.number().optional(),
});

export const MalloyViewSchema = z.object({
  id: z.string(),
  name: z.string(),
  model: z.string(), // Reference to DataModel name
  
  // Query definition
  group_by: z.array(z.string()).optional(),
  aggregate: z.array(z.string()).optional(),
  where: z.array(MalloyFilterSchema).optional(),
  filter: z.record(z.string(), z.any()).optional(), // Simplified filter (backward compat)
  limit: z.number().optional(),
  order_by: z.array(z.object({
    field: z.string(),
    direction: z.enum(['asc', 'desc']).default('asc'),
  })).optional(),
  
  // Malloy-specific
  turtle: z.array(MalloyTurtleSchema).optional(),
  reduce: z.array(z.string()).optional(),
  nest: z.array(MalloyTurtleSchema).optional(),
  parameters: z.array(MalloyViewParameterSchema).optional(),
  
  // Metadata
  title: z.string().optional(),
  description: z.string().optional(),
  tags: z.array(z.string()).optional(),
});

export type MalloyView = z.infer<typeof MalloyViewSchema>;
```

### Phase 3: Dashboard-View Integration (`model/src/schema/dashboard.ts`)

#### 3.1 Add View Reference to Dashboard Components

```typescript
// Add to dashboard.ts

export const MalloyViewReferenceSchema = z.object({
  viewId: z.string(), // Reference to MalloyView.id
  parameters: z.record(z.string(), z.any()).optional(), // View parameter values
  filters: z.array(MalloyFilterSchema).optional(), // Additional filters
  limit: z.number().optional(), // Override view limit
});

export const ViewComponentSchema = DashboardComponentBaseSchema.extend({
  type: z.literal('view'),
  view: MalloyViewReferenceSchema,
  display: z.enum(['table', 'chart', 'card', 'list']).default('table'),
  chartConfig: z.any().optional(), // Chart configuration if display is 'chart'
});

// Add to DashboardComponentSchema union
export const DashboardComponentSchema = z.discriminatedUnion('type', [
  StatCardSchema,
  ContainerSchema,
  ConceptCloudSchema,
  ExplorationsListSchema,
  ViewComponentSchema, // NEW
]);
```

#### 3.2 Dashboard-Level Filters and Parameters

```typescript
export const DashboardFilterSchema = z.object({
  field: z.string(),
  operator: z.enum(['=', '!=', '>', '<', '>=', '<=', 'in', 'not in']),
  value: z.any(),
  applyTo: z.array(z.string()).optional(), // Which views to apply to (empty = all)
});

export const DashboardParameterSchema = z.object({
  name: z.string(),
  type: z.enum(['string', 'number', 'date', 'boolean']),
  default: z.any().optional(),
  label: z.string().optional(),
});

// Enhance DashboardShapeSchema
export const DashboardShapeSchema = FormShapeSchema.extend({
  type: z.literal('dashboard').default('dashboard'),
  layout: DashboardLayoutSchema,
  components: z.array(DashboardComponentSchema),
  
  // NEW: Dashboard-level filters and parameters
  filters: z.array(DashboardFilterSchema).optional(),
  parameters: z.array(DashboardParameterSchema).optional(),
  
  // NEW: View collection (alternative to components)
  views: z.array(MalloyViewReferenceSchema).optional(),
});
```

### Phase 4: Application Schema Integration (`model/src/schema/application.ts`)

#### 4.1 Enhance ViewSchema to Support Malloy

```typescript
// In application.ts, enhance ViewSchema

export const ViewSchema = z.object({
  id: z.string(),
  name: z.string(),
  title: z.string().optional(),
  description: z.string().optional(),
  type: z.enum(['malloy', 'chart', 'table', 'form', 'custom', 'dashboard']),
  
  // For Malloy views
  malloyView: MalloyViewSchema.optional(), // Full Malloy view definition
  viewId: z.string().optional(), // Reference to MalloyView.id (if defined elsewhere)
  
  // For other view types
  source: z.string().optional(), // Reference to DataModel
  query: z.any().optional(), // ViewQuery (backward compat)
  config: z.record(z.string(), z.any()).optional(),
  module: z.string().optional(),
  dashboardComponent: DashboardComponentSchema.optional(),
});
```

### Phase 5: View Execution Service

#### 5.1 Create MalloyViewService (`model/src/data/malloy-view-service.ts`)

```typescript
import type { DataModel } from './sdsl';
import type { MalloyView } from '../schema/malloy-view';
import type { SemanticDataService } from './semantic-hydrator';

export class MalloyViewService {
  constructor(
    private readonly models: Map<string, DataModel<any>>,
    private readonly dataService: SemanticDataService
  ) {}

  /**
   * Execute a Malloy view against its model
   */
  async executeView(
    view: MalloyView,
    parameters?: Record<string, any>,
    additionalFilters?: any[]
  ): Promise<SemanticResult> {
    const model = this.models.get(view.model);
    if (!model) {
      throw new Error(`Model ${view.model} not found`);
    }

    // Build ViewQuery from MalloyView
    const query: ViewQuery = {
      group_by: view.group_by,
      aggregate: view.aggregate,
      where: view.where || [],
      filter: view.filter,
      limit: view.limit,
      order_by: view.order_by,
      turtle: view.turtle,
      reduce: view.reduce,
      nest: view.nest,
      parameters: view.parameters,
    };

    // Apply parameters
    if (parameters && view.parameters) {
      // Replace parameter placeholders in filters/query
      query.filter = this.applyParameters(query.filter, parameters, view.parameters);
    }

    // Apply additional filters
    if (additionalFilters) {
      query.where = [...(query.where || []), ...additionalFilters];
    }

    // Create DataView and execute
    const dataView = model.view(query);
    return await this.dataService.execute(dataView);
  }

  private applyParameters(
    filter: any,
    parameters: Record<string, any>,
    viewParameters: ViewParameter[]
  ): any {
    // Implementation: replace parameter placeholders
    // e.g., { field: 'region', operator: '=', value: '$region' } -> { value: parameters.region }
    return filter; // Simplified
  }
}
```

### Phase 6: Dashboard Execution

#### 6.1 Create DashboardViewService (`model/src/data/dashboard-view-service.ts`)

```typescript
import type { DashboardShape } from '../schema/dashboard';
import type { MalloyView } from '../schema/malloy-view';
import type { MalloyViewService } from './malloy-view-service';

export class DashboardViewService {
  constructor(private readonly viewService: MalloyViewService) {}

  /**
   * Execute all views in a dashboard
   */
  async executeDashboard(
    dashboard: DashboardShape,
    views: Map<string, MalloyView>,
    parameters?: Record<string, any>
  ): Promise<Map<string, SemanticResult>> {
    const results = new Map<string, SemanticResult>();

    // Apply dashboard-level filters
    const dashboardFilters = dashboard.filters || [];

    // Execute each view component
    for (const component of dashboard.components) {
      if (component.type === 'view' && component.view) {
        const viewId = component.view.viewId;
        const view = views.get(viewId);
        
        if (view) {
          // Combine dashboard filters with view-specific filters
          const allFilters = [
            ...dashboardFilters.filter(f => 
              !f.applyTo || f.applyTo.length === 0 || f.applyTo.includes(viewId)
            ),
            ...(component.view.filters || [])
          ];

          const result = await this.viewService.executeView(
            view,
            { ...parameters, ...component.view.parameters },
            allFilters
          );

          results.set(viewId, result);
        }
      }
    }

    return results;
  }
}
```

## Integration Points

### 1. Rust Platform Integration (Post-Prisma)

**Current Flow:**
```
SDSL DataModel
  ↓
DataView (ViewQuery)
  ↓
PolarsExecutionEngine
  ↓
Arrow/JSON Result
```

**Enhanced Flow:**
```
MalloyView (SDSL)
  ↓
ViewQuery (compiled)
  ↓
Execution Plan (JSON)
  ↓
Rust Platform (GDS/GDSL) ← No Prisma
  ↓
Polars/Arrow/DuckDB Execution
  ↓
Arrow/JSON Result
  ↓
SemanticHydrator
  ↓
FormModel / Dashboard Component
```

### 2. MVC as SDSL

- **Model**: `DataModel` (Malloy Source) = State:Structure
- **View**: `MalloyView` (Query Definition) = Representation:Perspective  
- **Controller**: `MalloyViewService` + `DashboardViewService` = Action:Rule

### 3. Model-View-Dashboard Platform

```
DataModel (Semantic Model)
    ↓
MalloyView (Query Definition)
    ↓
Dashboard (View Collection + Layout)
    ↓
UI Rendering (React/Radix)
```

## Testing Strategy

### Unit Tests
- `sdsl.test.ts` - Test enhanced SDSL features
- `malloy-view.test.ts` - Test MalloyView schema and execution
- `dashboard-view.test.ts` - Test Dashboard-View integration

### Integration Tests
- End-to-end: DataModel → MalloyView → Dashboard → Rendering
- Parameter passing: Dashboard → View → Execution
- Filter application: Dashboard filters → View filters → Execution

## Migration Path

### Step 1: Enhance SDSL (Backward Compatible)
- Add new fields to `DataModelConfig` (all optional)
- Enhance `ViewQuery` (backward compatible)
- Existing code continues to work

### Step 2: Add Malloy View Schema
- New schema file, doesn't break existing code
- Can be adopted incrementally

### Step 3: Enhance Dashboard Schema
- Add view components (optional)
- Existing dashboard components still work

### Step 4: Add Services
- New services, don't affect existing code
- Can be used alongside existing SemanticHydrator

## Questions to Resolve

1. **Malloy Compatibility Level**
   - Full Malloy spec or Malloy-inspired?
   - Which features are essential?

2. **View Execution Location**
   - TypeScript compilation or Rust compilation?
   - Where does query optimization happen?

3. **Dashboard Architecture**
   - Dashboard as View collection or Dashboard as separate concept?
   - How do non-view components (StatCard, etc.) relate to Views?

4. **Rust Platform Interface**
   - Exact API between SDSL and Rust platform?
   - How do we pass execution plans to Rust?

## Next Steps

1. **Review and Approve Plan** - Confirm approach aligns with architecture
2. **Implement Phase 1** - Enhance SDSL core (backward compatible)
3. **Implement Phase 2** - Create Malloy View schema
4. **Implement Phase 3** - Dashboard-View integration
5. **Test Integration** - Verify end-to-end flow
6. **Document Usage** - Update getting-started guide

