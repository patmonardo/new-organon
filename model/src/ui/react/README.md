# React UI Components - Basic Dashboard Components

This directory contains **basic, reusable dashboard components** that render Shape objects directly to React elements.

**Structure:**
- `components/` = Basic dashboard components (Button, Card, List, etc.)
- `sdsl/` = React SDSL adapters/controllers

**Note:** This is separate from Malloy-specific components (`ui/malloy/`):
- `ui/react/components/` = Basic dashboard components
- `ui/malloy/components/` = Malloy-specific components (ViewRenderer, DashboardRenderer, etc.)

## Architecture

```
Shape → Renderer Component → React Elements
```

These components are used by the `react-shape-adapter` to transform Shape objects (ButtonShape, ListShape, CardShape, etc.) into rendered React components.

## Components

### Button

- **`button.tsx`**: `ButtonRenderer` - renders ButtonShape to button/link elements
- **`link.tsx`**: `LinkRenderer` - renders LinkShape to Next.js Link elements with icon support

### List

- **`list.tsx`**: `ListRenderer` - renders ListShape with search and pagination
- **`breadcrumbs.tsx`**: `BreadcrumbsRenderer` - renders navigation breadcrumbs
- **`pagination.tsx`**: `PaginationRenderer` - renders pagination controls with URL params

### Card

- **`card.tsx`**: Multiple card renderers:
  - `CardRenderer` - basic card with title/subtitle/content
  - `StatCardRenderer` - statistics card with icon and value
  - `ContainerCardRenderer` - flexible container card

### Search

- **`search.tsx`**: `SearchRenderer` - search input with URL integration and debouncing

## Usage

### Via Shape Adapter (Recommended)

```tsx
import { reactShapeAdapter } from '@/model/sdsl/react-shape-adapter';

const buttonShape: ButtonShape = {
  type: 'button',
  label: 'Click me',
  variant: 'primary',
};

const jsx = reactShapeAdapter.render(buttonShape);
```

### Direct Component Usage

```tsx
import { ButtonRenderer } from '@/model/ui/react';

function MyComponent() {
  const shape: ButtonShape = {
    type: 'button',
    label: 'Click me',
    variant: 'primary',
  };
  
  return <ButtonRenderer shape={shape} />;
}
```

## Design Philosophy

- **Shape-First**: All components consume Shape objects, not DisplayElements
- **React Server Components**: Most components are server components by default
- **Client Components**: Use `"use client"` directive for interactive components (search, lists with state)
- **Tailwind CSS**: All styling via Tailwind utility classes
- **Next.js Integration**: Leverages Next.js routing, Link component, and URL params

## Future Enhancements

- **VS Code-style Breadcrumbs**: Add dropdown segments for advanced navigation
- **More Morphology**: Consider reintroducing morphology pipeline from Sankara for advanced transforms
- **Radix Primitives**: Add Radix UI-based renderers for more sophisticated interactions

