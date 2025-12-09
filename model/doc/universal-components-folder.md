# Universal Components Folder Structure

## New Structure

All UI directories now use a `components/` folder:

```
ui/react/
├── components/          # Basic dashboard components
│   ├── button/
│   ├── card/
│   ├── list/
│   ├── search/
│   ├── table/
│   ├── text/
│   ├── skeleton/
│   ├── theme/
│   └── index.ts
├── sdsl/                # React SDSL adapters
└── index.tsx

ui/radix/
├── components/          # Radix components (future)
│   └── index.ts
├── sdsl/                # Radix SDSL adapters
└── index.ts

ui/malloy/
├── components/          # Malloy React components (future)
│   └── index.ts
├── sdsl/                # Malloy SDSL adapters/controllers/views
└── index.ts
```

## Benefits

### 1. Consistent Structure

**All UI directories:**
- `components/` = UI components
- `sdsl/` = SDSL adapters/controllers

**Clear separation:**
- Components = UI rendering
- SDSL = Adapters/controllers

### 2. Scalability

**Easy to grow:**
- Add new components to `components/`
- Add new adapters to `sdsl/`
- Clear organization

### 3. Priority

**React and Malloy are priority:**
- `ui/react/components/` = Basic dashboard components
- `ui/malloy/components/` = Malloy-specific components
- Both can grow rapidly

## Migration

### React Components

**Moved:**
- `button/` → `components/button/`
- `card/` → `components/card/`
- `list/` → `components/list/`
- `search/` → `components/search/`
- `table/` → `components/table/`
- `text/` → `components/text/`
- `skeleton/` → `components/skeleton/`
- `theme/` → `components/theme/`
- `lib/` → `components/lib/`

**Exports:**
- `ui/react/index.tsx` → exports from `components/`
- `ui/react/components/index.ts` → exports all components

### Radix Components

**Created:**
- `ui/radix/components/` (placeholder for future)

### Malloy Components

**Already has:**
- `ui/malloy/components/` (ready for ViewRenderer, etc.)

## Result

✅ **Universal `components/` folder** across all UI directories
✅ **Consistent structure** - easy to navigate
✅ **Ready to grow** - React and Malloy are priority
✅ **Clear separation** - components vs sdsl adapters

---

**Key Insight**: Universal `components/` folder adopted across all UI directories. React and Malloy are priority for rapid growth.

