# Engine-Repository Upgrade Plan

**Date**: Current  
**Focus**: Wire ShapeEngine & EntityEngine to FormShapeRepository & EntityShapeRepository

## Current State Analysis

### Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│ Engine Layer (Business Logic)                          │
│ - ShapeEngine (Form Engine)                            │
│ - EntityEngine                                          │
│ - Uses: Repository<Shape>, Repository<Entity>         │
└─────────────────────────────────────────────────────────┘
                         ↓ (needs wiring)
┌─────────────────────────────────────────────────────────┐
│ Repository Layer (Precise Transactions)                │
│ - FormShapeRepository (Neo4j CRUD for Forms)           │
│ - EntityShapeRepository (Neo4j CRUD for Entities)      │
│ - Uses: FormShape, EntityShape schemas                 │
└─────────────────────────────────────────────────────────┘
```

### Current Issues

#### 1. **Schema Mismatch**
- **Engines use**: `Shape` schema (from `@schema`)
- **Repositories use**: `FormShape` schema (from `@schema/form`)
- **Engines use**: `Entity` schema (from `@schema`)
- **Repositories use**: `EntityShape` schema (from `@schema/entity`)

#### 2. **Repository Interface Mismatch**
- **Engines expect**: Generic `Repository<T>` interface
  ```typescript
  interface Repository<T> {
    get(id: string): Promise<T | null>;
    create(doc: T): Promise<void>;
    update(id: string, doc: T | ((current: T) => T)): Promise<void>;
    delete(id: string): Promise<void>;
  }
  ```
- **Repositories provide**: Concrete Neo4j repositories
  ```typescript
  class FormShapeRepository {
    async getFormById(id: string): Promise<FormShape | null>;
    async saveForm(shape: FormShape): Promise<FormShape>;
    async deleteForm(id: string): Promise<boolean>;
  }
  ```

#### 3. **Missing Form Loading in EntityEngine**
- **EntityEngine** needs to load Form Shapes via `formId`
- **Current**: EntityEngine only has `Repository<Entity>`, no Form access
- **Needed**: EntityEngine should use ShapeEngine to load Forms

#### 4. **In-Memory vs Neo4j**
- **Current**: Engines use in-memory repositories (`makeInMemoryRepository`)
- **Target**: Engines should use Neo4j repositories (`FormShapeRepository`, `EntityShapeRepository`)

## Upgrade Strategy

### Phase 1: Create Repository Adapters

**Goal**: Bridge generic `Repository<T>` interface to concrete Neo4j repositories

#### 1.1 FormShapeRepositoryAdapter

```typescript
// logic/src/repository/adapters/form-shape-adapter.ts
import { Repository } from '@repository';
import { FormShapeRepository } from '../form';
import { ShapeSchema, type Shape } from '@schema';
import { FormShape } from '@schema/form';

export class FormShapeRepositoryAdapter implements Repository<Shape> {
  constructor(private readonly formRepo: FormShapeRepository) {}

  async get(id: string): Promise<Shape | null> {
    const formShape = await this.formRepo.getFormById(id);
    if (!formShape) return null;
    return this.formShapeToShape(formShape);
  }

  async create(doc: Shape): Promise<void> {
    const formShape = this.shapeToFormShape(doc);
    await this.formRepo.saveForm(formShape);
  }

  async update(id: string, doc: Shape | ((current: Shape) => Shape)): Promise<void> {
    const current = await this.get(id);
    if (!current) throw new Error(`Shape not found: ${id}`);
    
    const updated = typeof doc === 'function' ? doc(current) : doc;
    const formShape = this.shapeToFormShape(updated);
    await this.formRepo.saveForm(formShape);
  }

  async delete(id: string): Promise<void> {
    await this.formRepo.deleteForm(id);
  }

  private shapeToFormShape(shape: Shape): FormShape {
    // Convert Shape schema → FormShape schema
    // Map shape.shape.core → FormShape fields
    // Map shape.shape.facets → FormShape structure
  }

  private formShapeToShape(formShape: FormShape): Shape {
    // Convert FormShape schema → Shape schema
    // Map FormShape → shape.shape structure
  }
}
```

#### 1.2 EntityShapeRepositoryAdapter

```typescript
// logic/src/repository/adapters/entity-shape-adapter.ts
import { Repository } from '@repository';
import { EntityShapeRepository } from '../entity';
import { EntitySchema, type Entity } from '@schema';
import { EntityShape } from '@schema/entity';

export class EntityShapeRepositoryAdapter implements Repository<Entity> {
  constructor(private readonly entityRepo: EntityShapeRepository) {}

  async get(id: string): Promise<Entity | null> {
    const entityShape = await this.entityRepo.getEntityById(id);
    if (!entityShape) return null;
    return this.entityShapeToEntity(entityShape);
  }

  async create(doc: Entity): Promise<void> {
    const entityShape = this.entityToEntityShape(doc);
    await this.entityRepo.saveEntity(entityShape);
  }

  async update(id: string, doc: Entity | ((current: Entity) => Entity)): Promise<void> {
    const current = await this.get(id);
    if (!current) throw new Error(`Entity not found: ${id}`);
    
    const updated = typeof doc === 'function' ? doc(current) : doc;
    const entityShape = this.entityToEntityShape(updated);
    await this.entityRepo.saveEntity(entityShape);
  }

  async delete(id: string): Promise<void> {
    await this.entityRepo.deleteEntity(id);
  }

  private entityToEntityShape(entity: Entity): EntityShape {
    // Convert Entity schema → EntityShape schema
    // Extract formId from entity.shape.facets or entity.shape.core
    // Map entity.shape.core → EntityShape fields
    // Map entity.shape.state → EntityShape values/meta
  }

  private entityShapeToEntity(entityShape: EntityShape): Entity {
    // Convert EntityShape schema → Entity schema
    // Map EntityShape → entity.shape structure
    // Store formId in appropriate location
  }
}
```

### Phase 2: Wire ShapeEngine to FormShapeRepository

**Goal**: ShapeEngine uses FormShapeRepository via adapter

#### 2.1 Update ShapeEngine Constructor

```typescript
// logic/src/relative/form/shape/shape-engine.ts
import { FormShapeRepositoryAdapter } from '@repository/adapters/form-shape-adapter';
import { FormShapeRepository } from '@repository/form';

export class ShapeEngine {
  constructor(
    private readonly repo?: Repository<Shape>,
    // OR accept FormShapeRepository directly:
    private readonly formRepo?: FormShapeRepository,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'shape',
  ) {
    // If formRepo provided, create adapter
    if (formRepo && !repo) {
      this.repo = new FormShapeRepositoryAdapter(formRepo);
    }
  }
  
  // ... rest of implementation
}
```

#### 2.2 Update ShapeService

```typescript
// logic/src/relative/form/shape/shape-service.ts
import { FormShapeRepository } from '@repository/form';

export class ShapeService {
  constructor(
    private readonly repo?: Repository<Shape>,
    // OR accept FormShapeRepository:
    private readonly formRepo?: FormShapeRepository,
    bus?: EventBus
  ) {
    // Create adapter if formRepo provided
    const adaptedRepo = formRepo 
      ? new FormShapeRepositoryAdapter(formRepo)
      : repo;
    
    this.engine = new ShapeEngine(adaptedRepo, this.bus);
  }
}
```

### Phase 3: Wire EntityEngine to EntityShapeRepository + ShapeEngine

**Goal**: EntityEngine uses EntityShapeRepository AND ShapeEngine for Form loading

#### 3.1 Update EntityEngine Constructor

```typescript
// logic/src/relative/form/entity/entity-engine.ts
import { EntityShapeRepositoryAdapter } from '@repository/adapters/entity-shape-adapter';
import { EntityShapeRepository } from '@repository/entity';
import { ShapeEngine } from '../shape/shape-engine';

export class EntityEngine {
  constructor(
    private readonly repo: Repository<Entity>,
    // Add ShapeEngine dependency for Form loading
    private readonly shapeEngine?: ShapeEngine,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'entity',
  ) {}

  // Add method to load Form via ShapeEngine
  async getFormForEntity(entityId: string): Promise<FormShape | undefined> {
    const entity = await this.getEntity(entityId);
    if (!entity) return undefined;
    
    const formId = entity.formId; // Extract from entity
    if (!formId) return undefined;
    
    if (!this.shapeEngine) {
      throw new Error('ShapeEngine required to load Form for Entity');
    }
    
    const shape = await this.shapeEngine.getShape(formId);
    return shape; // Convert Shape → FormShape if needed
  }
}
```

#### 3.2 Update EntityService

```typescript
// logic/src/relative/form/entity/entity-service.ts
import { EntityShapeRepository } from '@repository/entity';
import { ShapeEngine } from '../shape/shape-engine';

export class EntityService {
  constructor(
    private readonly repo?: Repository<Entity>,
    // OR accept EntityShapeRepository + ShapeEngine:
    private readonly entityRepo?: EntityShapeRepository,
    private readonly shapeEngine?: ShapeEngine,
  ) {
    // Create adapter if entityRepo provided
    if (entityRepo && !repo) {
      this.repo = new EntityShapeRepositoryAdapter(entityRepo);
    }
  }
}
```

### Phase 4: Schema Conversion Functions

**Goal**: Implement Shape ↔ FormShape and Entity ↔ EntityShape conversions

#### 4.1 Shape ↔ FormShape Conversion

**Key Mappings**:
- `Shape.shape.core` → `FormShape` (id, name, type, etc.)
- `Shape.shape.facets` → `FormShape` (fields, layout, actions, tags)
- `Shape.shape.state` → **REMOVED** (Rational/Empirical split)
- `Shape.shape.signature` → `FormShape` (if needed)

#### 4.2 Entity ↔ EntityShape Conversion

**Key Mappings**:
- `Entity.shape.core` → `EntityShape` (id, type, name, description)
- `Entity.shape.core.formId` → `EntityShape.formId` (required)
- `Entity.shape.state` → `EntityShape.values` + `EntityShape.meta`
- `Entity.shape.facets` → `EntityShape.facets`
- `Entity.shape.signature` → `EntityShape.signature`

## Implementation Checklist

### Phase 1: Adapters
- [ ] Create `FormShapeRepositoryAdapter`
- [ ] Create `EntityShapeRepositoryAdapter`
- [ ] Implement schema conversion functions
- [ ] Add tests for adapters

### Phase 2: ShapeEngine Wiring
- [ ] Update `ShapeEngine` constructor to accept `FormShapeRepository`
- [ ] Update `ShapeService` to accept `FormShapeRepository`
- [ ] Wire adapter in constructor
- [ ] Test ShapeEngine → FormShapeRepository flow

### Phase 3: EntityEngine Wiring
- [ ] Update `EntityEngine` constructor to accept `EntityShapeRepository` + `ShapeEngine`
- [ ] Add `getFormForEntity()` method
- [ ] Update `EntityService` to accept repositories
- [ ] Test EntityEngine → EntityShapeRepository flow
- [ ] Test EntityEngine → ShapeEngine → Form loading

### Phase 4: Integration
- [ ] Update factory/initialization code
- [ ] Update tests to use Neo4j repositories
- [ ] Verify Form:Entity relationship works end-to-end
- [ ] Performance testing

## Key Decisions

1. **Adapter Pattern**: Use adapters to bridge generic Repository interface to concrete Neo4j repositories
2. **ShapeEngine Dependency**: EntityEngine needs ShapeEngine to load Forms (not direct repository access)
3. **Schema Conversion**: Convert between `Shape`/`Entity` (Engine schemas) and `FormShape`/`EntityShape` (Repository schemas)
4. **Backward Compatibility**: Keep existing `Repository<T>` interface, adapters implement it

## Testing Strategy

1. **Unit Tests**: Test adapters in isolation
2. **Integration Tests**: Test Engine → Adapter → Repository flow
3. **End-to-End**: Test Form:Entity relationship (Entity loads Form via ShapeEngine)

## Notes

- **Rational/Empirical Split**: FormShape has no state/meta, EntityShape has values/meta
- **Form Reference**: EntityShape.formId → FormShape.id (explicit Thinking)
- **Repository Focus**: Repositories handle precise transactions only
- **Engine Focus**: Engines handle business logic and orchestration

