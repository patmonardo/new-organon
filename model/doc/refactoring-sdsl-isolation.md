# Refactoring: SDSL Isolation

## What Changed

### Moved React/Radix Modules from sdsl/ to ui/

**Before:**
```
sdsl/
├── react-adapter.tsx
├── react-controller.ts
├── react-shape-adapter.tsx
├── react-shape-view.tsx
├── react-view.tsx
├── radix-adapter.tsx
├── radix-controller.tsx
└── radix-primitives.tsx
```

**After:**
```
sdsl/                    # Isolated core
├── form-model.ts
├── form-view.ts
├── form-controller.ts
├── agent-model.ts
├── agent-view.ts
├── agent-controller.ts
├── agent-adapter.ts
├── adapter.ts
├── types.ts
└── index.ts

ui/react/sdsl/          # React adapters/controllers
├── react-adapter.tsx
├── react-controller.ts
├── react-shape-adapter.tsx
├── react-shape-view.tsx
├── react-view.tsx
└── index.ts

ui/radix/sdsl/          # Radix adapters/controllers
├── radix-adapter.tsx
├── radix-controller.tsx
├── radix-primitives.tsx
└── index.ts
```

## Benefits

### 1. SDSL is Now Isolated

**sdsl/ contains only:**
- Form MVC core (form-model, form-view, form-controller)
- Agent MVC core (agent-model, agent-view, agent-controller)
- Types and base adapters
- **No UI-specific code**

### 2. UI Code in UI Directories

**React adapters/controllers:**
- `ui/react/sdsl/` - React-specific SDSL adapters
- Clear separation from core

**Radix adapters/controllers:**
- `ui/radix/sdsl/` - Radix-specific SDSL adapters
- Clear separation from core

### 3. Better Organization

**Structure:**
```
sdsl/              # Core (isolated)
ui/react/sdsl/     # React adapters
ui/radix/sdsl/      # Radix adapters
ui/malloy/          # Malloy UI (speculative bubble)
```

## Updated Imports

### React Files

**Updated imports:**
- `./types` → `../../../sdsl/types`
- `./form-view` → `../../../sdsl/form-view`
- `./form-model` → `../../../sdsl/form-model`
- `./form-controller` → `../../../sdsl/form-controller`
- `./react-adapter` → `./react-adapter` (same directory)

### Radix Files

**Updated imports:**
- `./types` → `../../../sdsl/types`
- `../data/semantic-hydrator` → `../../../execution/semantic-hydrator`
- `../schema/radix` → `../../../schema/radix`

## Exports

### sdsl/index.ts

**Now exports:**
- Core SDSL (from data/sdsl)
- Form MVC core
- Agent MVC core
- Types and adapters
- **No React/Radix exports**

### ui/react/sdsl/index.ts

**Exports:**
- React adapters
- React controllers
- React views

### ui/radix/sdsl/index.ts

**Exports:**
- Radix adapters
- Radix controllers
- Radix primitives

## Result

**SDSL is properly isolated:**
- ✅ Core logic only
- ✅ No UI dependencies
- ✅ Clean separation
- ✅ UI code in UI directories

---

**Key Insight**: SDSL is now isolated - React/Radix adapters moved to ui/react/sdsl/ and ui/radix/sdsl/. This keeps sdsl/ clean and properly isolated.

