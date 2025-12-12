import type { EventBus } from '../../src/absolute/core/bus';
import { InMemoryEventBus } from '../../src/absolute/core/bus';

import { ShapeEngine } from '../../src/relative/form/shape';
import { EntityEngine } from '../../src/relative/form/entity';

import { Neo4jConnection, defaultConnection } from '../../src/repository/neo4j-client';
import { FormShapeRepository } from '../../src/repository/form';
import { EntityShapeRepository } from '../../src/repository/entity';

export type FormDbWiring = {
  connection: Neo4jConnection;

  // Neo4j repositories
  formShapeRepo: FormShapeRepository;
  entityShapeRepo: EntityShapeRepository;

  // Engines (directly use repositories)
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
 * Engine -> Neo4j repository (direct, no adapters)
 */
export function createFormDb(options: CreateFormDbOptions = {}): FormDbWiring {
  const connection = options.connection ?? defaultConnection;
  const bus = options.bus ?? new InMemoryEventBus();

  const formShapeRepo = new FormShapeRepository(connection);
  const entityShapeRepo = new EntityShapeRepository(connection);

  // Engines now accept repositories directly (no adapters)
  const shapeEngine = new ShapeEngine(formShapeRepo, bus);
  const entityEngine = new EntityEngine(entityShapeRepo, bus);

  return {
    connection,
    formShapeRepo,
    entityShapeRepo,
    shapeEngine,
    entityEngine,
    bus,
  };
}
