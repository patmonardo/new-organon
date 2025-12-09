# Malloy SDSL Structure

## Consistent Pattern Across UI

All UI adapters follow the same `sdsl/` folder pattern:

```
ui/react/sdsl/     # React SDSL adapters/controllers
ui/radix/sdsl/     # Radix SDSL adapters/controllers
ui/malloy/sdsl/    # Malloy SDSL adapters/controllers/views
```

## Malloy Structure

```
ui/malloy/
├── sdsl/                    # Malloy SDSL (adapters/controllers/views)
│   ├── model/
│   │   └── malloy-model.ts  # Data Provider (not a viewer)
│   ├── controller/
│   │   └── malloy-controller.ts # Orchestrator
│   ├── view/
│   │   └── malloy-view.ts   # Display Translator
│   └── index.ts
│
└── components/              # Malloy React components (future)
    ├── view-renderer.tsx    # ViewRenderer
    ├── dashboard-renderer.tsx
    ├── chart-renderer.tsx
    └── index.ts
```

## Pattern Consistency

### React Pattern
```
ui/react/
├── sdsl/          # React adapters/controllers
└── button/        # React components
```

### Radix Pattern
```
ui/radix/
└── sdsl/          # Radix adapters/controllers
```

### Malloy Pattern
```
ui/malloy/
├── sdsl/          # Malloy adapters/controllers/views
└── components/    # Malloy React components
```

## Benefits

1. **Consistent Structure**: All UI adapters use `sdsl/` folder
2. **Clear Separation**: SDSL adapters vs UI components
3. **Isolated Core**: `sdsl/` core remains isolated
4. **Future Components**: `components/` ready for ViewRenderer, etc.

---

**Key Insight**: Malloy follows the same pattern as React/Radix - `sdsl/` for adapters/controllers/views, `components/` for actual UI components.

