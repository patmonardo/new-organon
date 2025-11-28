/**
 * Entity Service
 *
 * CRUD operations for Entity model.
 * Projects from @logic Entity (Thing in Appearance).
 */

/**
 * Entity: The basic unit of the Model
 */
export interface Entity {
  id: string;
  type: string;
  [key: string]: unknown;
}

/**
 * ModelEntity: Entity within a Model context
 */
export interface ModelEntity extends Entity {
  createdAt?: number;
  updatedAt?: number;
}

/**
 * EntityInput: Data for creating/updating an entity
 */
export interface EntityInput {
  type: string;
  name?: string;
  data?: Record<string, unknown>;
}

/**
 * EntityFilter: Criteria for querying entities
 */
export interface EntityFilter {
  type?: string;
  id?: string;
  ids?: string[];
}

/**
 * EntityService interface
 * Can be implemented with any persistence layer or mocked for testing
 */
export interface EntityService {
  create(input: EntityInput): Promise<ModelEntity>;
  findById(id: string): Promise<ModelEntity | null>;
  findMany(filter?: EntityFilter): Promise<ModelEntity[]>;
  update(id: string, input: Partial<EntityInput>): Promise<ModelEntity>;
  delete(id: string): Promise<boolean>;
}

/**
 * Mock EntityService for testing
 */
export class MockEntityService implements EntityService {
  private entities: Map<string, ModelEntity> = new Map();
  private idCounter = 1;

  async create(input: EntityInput): Promise<ModelEntity> {
    const entity: ModelEntity = {
      id: `entity-${this.idCounter++}`,
      type: input.type,
      name: input.name,
      ...input.data,
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };
    this.entities.set(entity.id, entity);
    return entity;
  }

  async findById(id: string): Promise<ModelEntity | null> {
    return this.entities.get(id) || null;
  }

  async findMany(filter?: EntityFilter): Promise<ModelEntity[]> {
    let results = Array.from(this.entities.values());

    if (filter?.type) {
      results = results.filter(e => e.type === filter.type);
    }
    if (filter?.id) {
      results = results.filter(e => e.id === filter.id);
    }
    if (filter?.ids) {
      results = results.filter(e => filter.ids!.includes(e.id));
    }

    return results;
  }

  async update(id: string, input: Partial<EntityInput>): Promise<ModelEntity> {
    const entity = this.entities.get(id);
    if (!entity) {
      throw new Error(`Entity not found: ${id}`);
    }

    const updated: ModelEntity = {
      ...entity,
      ...input,
      ...input.data,
      updatedAt: Date.now(),
    };
    this.entities.set(id, updated);
    return updated;
  }

  async delete(id: string): Promise<boolean> {
    return this.entities.delete(id);
  }

  // Test helpers
  clear(): void {
    this.entities.clear();
    this.idCounter = 1;
  }

  seed(entities: ModelEntity[]): void {
    for (const entity of entities) {
      this.entities.set(entity.id, entity);
    }
  }
}

/**
 * Default mock service instance for development/testing
 */
export const mockEntityService = new MockEntityService();
