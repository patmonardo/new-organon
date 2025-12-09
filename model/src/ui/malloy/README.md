# Malloy UI Sandbox (`ui/malloy`)

**Status**: Experimental / Laboratory  
**Owner**: AI (Antigravity)  
** Purpose**: Exploration of "Malloy-Like" UI patterns, Semantic Hydration ("The Aquifer"), and Next.js-style server/client separation using local mock engines.

## Overview

This directory serves as a sandbox to explore advanced BI and data application patterns that may eventually migrate to the production `ui/react` package. It focuses on:

1.  **Semantic Modeling**: Using declarative SDSL (Species DSL) to define data models (`domain/ecommerce.ts`).
2.  **The "Aquifer" Pattern**: Abstracting local vs. remote data execution behind a `SemanticDataService` interface.
3.  **Vice-Versa UI**: Building React components that drive the Malloy query engine, rather than just rendering outputs.

## Key Components

- **`CustomerInvoiceApp.tsx`**: The main entry point. Simulates a full Next.js application with "Server-Side" data fetching and Client-Side interactivity.
- **`components/MalloyViewBuilder`**: Visualizes and inspects the underlying query execution plans (DuckDB/Polars).
- **`components/MalloyDataGrid`**: A density-optimized, desktop-first data grid for viewing results.
- **`sdsl/useMalloyQuery`**: The primary React hook connecting UI state to the Execution Engine.

## Architecture

```mermaid
graph TD
    UI[React UI (CustomerInvoiceApp)] --> Hook[useMalloyQuery]
    Hook --> Service[SemanticDataService (Aquifer)]
    Service --> |Adapts| Engine[PolarsExecutionEngine]
    Engine --> |Queries| Data[Mock Arrow/Polars Data]
```

## Usage

This package is currently "mocked" and runs entirely in-memory using `nodejs-polars`. It does not require a running backend server.
