import type { EventBus } from '../../src/absolute/core/bus';
import { InMemoryEventBus } from '../../src/absolute/core/bus';

import type { Repository } from '../../src/repository/repo';

import type { Shape } from '../../src/schema/shape';
import type { Entity } from '../../src/schema/entity';

import { ShapeEngine } from '../../src/relative/form/shape';
import { EntityEngine } from '../../src/relative/form/entity';

import { Neo4jConnection, defaultConnection } from '../../src/repository/neo4j-client';
import { FormShapeRepository } from '../../src/repository/form';
import { EntityShapeRepository } from '../../src/repository/entity';
import { FormShapeRepositoryAdapter } from '../../src/repository/adapters/form-shape-adapter';
import { EntityShapeRepositoryAdapter } from '../../src/repository/adapters/entity-shape-adapter';

export type FormDbWiring = {
  connection: Neo4jConnection;

  // Precise Neo4j drivers (single-client: engines/adapters)
  formShapeRepo: FormShapeRepository;
  entityShapeRepo: EntityShapeRepository;

  // Engine-facing repositories (adapters)
  shapeRepo: Repository<Shape>;
  entityRepo: Repository<Entity>;

  // Engines (sole clients of the adapters)
  shapeEngine: ShapeEngine;
  entityEngine: EntityEngine;

  bus: EventBus;
};

export type CreateFormDbOptions = {
  connection?: Neo4jConnection;
  bus?: EventBus;
};

/**
 * Test-only wiring helper.
 *
 * Builds the full stack used by the Neo4j integration tests:
 * Engine -> Adapter Repository<T> -> Neo4j driver repo
 */
export function createFormDb(options: CreateFormDbOptions = {}): FormDbWiring {
  const connection = options.connection ?? defaultConnection;
  const bus = options.bus ?? new InMemoryEventBus();

  const formShapeRepo = new FormShapeRepository(connection);
  const entityShapeRepo = new EntityShapeRepository(connection);

  const shapeRepo = new FormShapeRepositoryAdapter(formShapeRepo);
  const entityRepo = new EntityShapeRepositoryAdapter(entityShapeRepo);

  const shapeEngine = new ShapeEngine(shapeRepo, bus);
  const entityEngine = new EntityEngine(entityRepo, bus);

  return {
    connection,
    formShapeRepo,
    entityShapeRepo,
    shapeRepo,
    entityRepo,
    shapeEngine,
    entityEngine,
    bus,
  };
}
