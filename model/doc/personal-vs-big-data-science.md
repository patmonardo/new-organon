# Personal vs Big Data Science Architecture

## Overview

We have **two distinct data science domains**:

1. **GDSL (Big Data Science)** - GDS-oriented, ML/GNN, enterprise-scale
2. **SDSL (Personal Data Science)** - Malloy, Polars, Arrow, individual/team-scale

## GDSL: ML and Logical Forms

### Characteristics
- **ML-oriented** - Machine Learning operations
- **Logical Forms** - Advanced logical processing
- **Enterprise-scale** - Large datasets, distributed processing
- **Rust-based** - Performance-critical operations
- **OpenCypher** - Graph query language (GDSL itself)

### Use Cases
- ML model training and inference
- Logical form processing
- GNN operations
- Enterprise data processing
- Kernel-level operations
- Advanced graph analytics

### Architecture
```
GDSL (OpenCypher)
    ↓
GDS Kernel
    ↓
ML/GNN Algorithms
    ↓
Big Data Processing
```

## SDSL: General Data Models + Middleware

### Characteristics
- **General Data Models** - Any data modeling domain
- **Middleware** - Business logic, transformations
- **Special Domain Calculators** - Domain-specific calculations
- **Malloy-inspired** - Semantic data modeling
- **Polars/Arrow** - Columnar execution
- **TypeScript-based** - Business logic, MVC
- **Personal/Team-scale** - Individual analyst workflows

### Use Cases
- General data modeling
- Business logic and middleware
- Domain-specific calculators
- Form applications
- Team dashboards
- Business intelligence
- Interactive data exploration

### Architecture
```
Malloy Model (SDSL)
    ↓
Polars Execution
    ↓
Apache Arrow
    ↓
MVC Applications
```

## The Stack: General Data Models + Middleware

**Powerful stack for Data Modeling and Middleware:**

1. **Malloy** - Semantic modeling (measures, dimensions, joins)
2. **Polars** - DataFrame execution engine
3. **Apache Arrow** - Columnar memory format
4. **TypeScript** - Business logic, MVC, middleware
5. **React** - UI components
6. **Radix + Google Design Language** - Rich component library
7. **Domain Calculators** - Special-purpose calculation engines

## Office Capabilities

### Full Office Suite Features

We're building **full Office capabilities** - not just forms, but:
- ✅ **Forms** - Data entry, validation
- ✅ **Lists** - Data tables, collections
- ✅ **Navigation** - Breadcrumbs, navbars, menus
- ✅ **Dashboards** - Analytics, KPIs
- ✅ **Reports** - Data visualization
- ✅ **Documents** - Rich text, formatting
- ✅ **Spreadsheets** - Data manipulation
- ✅ **Presentations** - Slide decks

### Component Foundation

**Google Design Language + Radix Components** = Rich MVC Language

- **Google Design Language** - Material Design principles
- **Radix Components** - Accessible, composable primitives
- **Combined** - Rich, accessible, beautiful UI components

## Lists and Link Components

### Navigation Components

**Lists** - For structured navigation:
- Breadcrumbs
- Navigation menus
- Sidebars
- Table of contents

**Link** - For navigation:
- Internal links
- External links
- Action links
- Breadcrumb links

### Example: Breadcrumbs

```tsx
<List>
  <Link href="/">Home</Link>
  <Link href="/customers">Customers</Link>
  <Link href="/customers/123">John Doe</Link>
</List>
```

### Example: Navbar

```tsx
<List horizontal>
  <Link href="/dashboard">Dashboard</Link>
  <Link href="/customers">Customers</Link>
  <Link href="/orders">Orders</Link>
  <Link href="/reports">Reports</Link>
</List>
```

## React Dependency

**React is fine** - We're React-dependent and that's okay:
- ✅ Learning React is part of the journey
- ✅ GUI work can be enjoyable with the right tools
- ✅ EDA Data Science is "quite lovely"
- ✅ 2D Stylesheet design is accessible to everyone

**The stack makes it enjoyable:**
- Malloy for semantic modeling
- Polars for fast execution
- React for interactive UI
- Radix for accessible components
- Google Design Language for beautiful design

## Rich MVC Language

### Models
- **Malloy Models** - Semantic data models
- **Form Models** - Form state management
- **View Models** - Display logic

### Views
- **Radix Components** - Accessible primitives
- **Google Design Language** - Material Design
- **Lists & Links** - Navigation components
- **2D Stylesheets** - CSS/design system

### Controllers
- **React Controllers** - Server actions
- **Semantic Hydration** - Data loading
- **Business Logic** - Form processing

## Archive Reference: sankara/app/form

We have existing form patterns in `/archive/sankara/app/form` that we should reference:
- Form components
- Validation patterns
- Layout patterns
- Interaction patterns

## Vision: Full Office Suite

### Data Science Platform
- **Personal Data Science** - Malloy, Polars, Arrow
- **Big Data Science** - GDSL, ML/GNN (via RootAgent)
- **EDA Tools** - Interactive exploration
- **Visualization** - Charts, graphs, dashboards

### Office Applications
- **Forms** - Data entry, validation
- **Lists** - Tables, collections
- **Navigation** - Breadcrumbs, navbars
- **Documents** - Rich text editing
- **Spreadsheets** - Data manipulation
- **Presentations** - Slide decks

### Component Library
- **Radix Primitives** - Accessible base components
- **Google Design Language** - Material Design
- **Lists & Links** - Navigation components
- **2D Stylesheets** - Design system

## Next Steps

- [ ] Review archive/sankara/app/form patterns
- [ ] Document Lists and Link components
- [ ] Create navigation component library
- [ ] Integrate Google Design Language
- [ ] Build full Office capabilities
- [ ] Document Personal vs Big Data Science boundaries

