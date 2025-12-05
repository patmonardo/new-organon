# Full Office Capabilities: Rich MVC Language

## Vision

Build **full Office capabilities** - not just forms, but a complete suite of office applications powered by:
- **Malloy** - Semantic data modeling
- **Polars/Arrow** - Fast execution
- **React** - Interactive UI
- **Radix + Google Design Language** - Rich, accessible components
- **2D Stylesheets** - Beautiful design

## Personal Data Science Stack

**Powerful stack for Personal Data Modeling:**

1. **Malloy** - Semantic modeling (measures, dimensions, joins)
2. **Polars** - DataFrame execution engine
3. **Apache Arrow** - Columnar memory format
4. **TypeScript** - Business logic, MVC
5. **React** - UI components (we're React-dependent, and that's fine!)
6. **Radix + Google Design Language** - Rich component library

**Why React is fine:**
- ✅ Learning React is part of the journey
- ✅ GUI work can be enjoyable with the right tools
- ✅ EDA Data Science is "quite lovely"
- ✅ 2D Stylesheet design is accessible to everyone

## Component Foundation

### Google Design Language + Radix Components

**Combination = Rich MVC Language**

- **Google Design Language** - Material Design principles
  - Material Design 3 (Material You)
  - Design tokens
  - Component patterns
  - Accessibility

- **Radix Components** - Accessible primitives
  - Unstyled, composable
  - Accessible by default
  - Keyboard navigation
  - Screen reader support

- **Combined** - Rich, accessible, beautiful UI components

## Lists and Link Components

### Navigation Components

**Lists** - For structured navigation:
- Breadcrumbs
- Navigation menus
- Sidebars
- Table of contents
- Pagination

**Link** - For navigation:
- Internal links
- External links
- Action links
- Breadcrumb links

### Example: Breadcrumbs

```tsx
<List type="breadcrumb">
  <Link href="/">Home</Link>
  <Link href="/customers">Customers</Link>
  <Link href="/customers/123" current>John Doe</Link>
</List>
```

### Example: Navbar

```tsx
<List type="navbar" horizontal>
  <Link href="/dashboard">Dashboard</Link>
  <Link href="/customers">Customers</Link>
  <Link href="/orders">Orders</Link>
  <Link href="/reports">Reports</Link>
</List>
```

### Example: Sidebar Navigation

```tsx
<List type="sidebar">
  <Link href="/dashboard" icon="dashboard">Dashboard</Link>
  <Link href="/customers" icon="users">Customers</Link>
  <Link href="/orders" icon="shopping-cart">Orders</Link>
  <Link href="/reports" icon="chart">Reports</Link>
</List>
```

## Full Office Capabilities

### 1. Forms
- ✅ Data entry
- ✅ Validation
- ✅ Semantic hydration
- ✅ Form state management

### 2. Lists
- ✅ Data tables
- ✅ Collections
- ✅ Navigation menus
- ✅ Breadcrumbs
- ✅ Pagination

### 3. Navigation
- ✅ Breadcrumbs
- ✅ Navbars
- ✅ Sidebars
- ✅ Menus
- ✅ Links

### 4. Dashboards
- ✅ Analytics
- ✅ KPIs
- ✅ Metrics
- ✅ Charts
- ✅ Data visualization

### 5. Reports
- ✅ Data visualization
- ✅ Charts
- ✅ Tables
- ✅ Export capabilities

### 6. Documents (Future)
- ✅ Rich text editing
- ✅ Formatting
- ✅ Collaboration

### 7. Spreadsheets (Future)
- ✅ Data manipulation
- ✅ Formulas
- ✅ Charts

### 8. Presentations (Future)
- ✅ Slide decks
- ✅ Animations
- ✅ Media

## Rich MVC Language

### Models
- **Malloy Models** - Semantic data models
  ```typescript
  export const CustomerModel = defineModel({
    measures: { totalRevenue: sum('invoices.amount') },
    dimensions: { region: 'region' },
    joins: { invoices: { model: InvoiceModel } }
  });
  ```

- **Form Models** - Form state management
  ```typescript
  const formModel = new FormModel(CustomerShape);
  ```

- **View Models** - Display logic
  ```typescript
  const view = new CustomerView(formModel, 'view');
  ```

### Views
- **Radix Components** - Accessible primitives
  - Card, Table, Metric, Button, Badge
  - Dialog, Dropdown, Tabs, Accordion

- **Google Design Language** - Material Design
  - Material You design tokens
  - Component patterns
  - Motion design

- **Lists & Links** - Navigation components
  - Breadcrumbs, Navbars, Sidebars
  - Internal/External links

- **2D Stylesheets** - CSS/design system
  - Tailwind CSS
  - Design tokens
  - Responsive design

### Controllers
- **React Controllers** - Server actions
  ```typescript
  const controller = new CustomerController('view');
  await controller.loadCustomerProfile(id);
  ```

- **Semantic Hydration** - Data loading
  ```typescript
  const snapshot = await controller.loadCustomerProfile(id);
  ```

- **Business Logic** - Form processing
  ```typescript
  await controller.executeAction('submit', data);
  ```

## Archive Reference: sankara/app/form

We have existing form patterns in `/archive/sankara/app/form`:

- **Form components** - Form engine, validation
- **List components** - Breadcrumbs, navlinks, sidenav, pagination
- **Link components** - Link morph, link tests
- **Card components** - Card morph, primitives, stat
- **Table components** - Table adapter, renderer
- **Schema components** - Form shapes, validation

**These patterns inform our component library.**

## EDA Data Science Experience

**"EDA Data Science can be quite lovely"**

The combination of:
- **Malloy** - Declarative semantic modeling
- **Polars** - Fast columnar execution
- **React** - Interactive visualizations
- **Radix** - Accessible components
- **Google Design Language** - Beautiful design

Creates a **delightful** data science experience:
- ✅ Interactive exploration
- ✅ Fast queries
- ✅ Beautiful visualizations
- ✅ Accessible interface
- ✅ Enjoyable workflow

## 2D Stylesheet Design

**"We should all be good with 2D Stylesheet design"**

- **CSS** - Standard web styling
- **Tailwind CSS** - Utility-first CSS
- **Design Tokens** - Consistent design system
- **Responsive Design** - Mobile-first approach
- **Accessibility** - WCAG compliance

**Everyone can learn 2D design:**
- ✅ Visual, intuitive
- ✅ Immediate feedback
- ✅ Iterative refinement
- ✅ Design system support

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Rich MVC Language                          │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  Models (Malloy) → Views (Radix + Material) → Controllers│
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Forms      │  │   Lists      │  │ Navigation   │ │
│  │   Tables     │  │   Links      │  │ Breadcrumbs  │ │
│  │   Dashboards │  │   Reports    │  │ Menus        │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
│                                                           │
│  Powered by:                                              │
│  - Malloy (Semantic Modeling)                            │
│  - Polars/Arrow (Execution)                              │
│  - React (UI)                                            │
│  - Radix + Material (Components)                         │
│  - 2D Stylesheets (Design)                               │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Next Steps

- [ ] Review sankara archive patterns
- [ ] Document Lists and Link components
- [ ] Create navigation component library
- [ ] Integrate Google Design Language
- [ ] Build breadcrumb component
- [ ] Build navbar component
- [ ] Build sidebar component
- [ ] Document full Office capabilities roadmap

