# MVC SDK: The Data SDSL Manifesto

## Philosophy

**Data is not just storage; it is analysis.**

Traditional ORMs (like Prisma) focus on _persistence_: how to get data in and out of a database row-by-row.
The **Data SDSL** focuses on _semantics_: how to aggregate, group, and analyze data to derive meaning.

We draw inspiration from **Malloy** (Google) for the semantic layer and **Polars/Arrow** for the execution engine.

## Core Concepts

### 1. The Source

The raw data substrate. In our architecture, this is presumed to be **Apache Arrow** or **Polars DataFrames**. It is columnar, typed, and fast.

### 2. The Model (Semantic Layer)

A Model wraps a Source and adds meaning. It defines:

- **Dimensions**: Attributes you can group by (e.g., `status`, `date.month`).
- **Measures**: Aggregations you can calculate (e.g., `total_revenue`, `count`).
- **Joins**: Relationships to other Models.

### 3. The View (Analytical Query)

A View is a specific question asked of the Model. It is not just a `SELECT *`, but a structured request for analysis:

- "Show me `total_revenue` by `status`"
- "Show me `avg_invoice_amount` by `customer_region`"

### 4. The Controller (Runtime)

The Controller executes the View against the Model. It translates the semantic request into a high-performance query (e.g., Polars expression) and returns the result, often as an Arrow Table or a specialized View Component.

## Architecture

```mermaid
graph TD
    Source[Raw Data (Arrow/Polars)] --> Model[Data Model (SDSL)]
    Model --> |Defines| Dimensions
    Model --> |Defines| Measures

    UserRequest --> Controller
    Controller --> |Queries| Model
    Model --> |Executes| Source
    Source --> |Returns| Result[Arrow Table]
    Result --> View[UI Component]
```

## Example Syntax (Conceptual)

```typescript
export const Invoices = defineModel({
  source: 'db.invoices',
  fields: {
    amount: z.number(),
    status: z.enum(['PAID', 'PENDING']),
    date: z.date(),
  },
  measures: {
    totalRevenue: sum('amount'),
    count: count(),
    avgAmount: avg('amount'),
  },
  dimensions: {
    status: 'status',
    month: truncate('date', 'month'),
  },
});

// The "View" is a query against this model
export const RevenueByMonth = Invoices.view({
  group_by: ['month', 'status'],
  aggregate: ['totalRevenue', 'count'],
});
```
