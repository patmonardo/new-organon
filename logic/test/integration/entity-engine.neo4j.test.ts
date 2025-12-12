import { describe, it, expect } from 'vitest';

import { defaultConnection } from '../../src/repository/neo4j-client';
import { createFormDb } from './createFormDb';

const neo4jOk = await defaultConnection.verifyConnectivity();

describe.skipIf(!neo4jOk)('EntityEngine × Neo4j (createFormDb)', () => {
  it('persists create/setCore/delete through EntityShapeRepository', async () => {
    const { entityEngine: engine, entityShapeRepo: repo } = createFormDb({ connection: defaultConnection });

    const id = `entity:neo4j:${Date.now().toString(36)}`;
    const formId = `form:neo4j:${Date.now().toString(36)}`;

    try {
      await engine.handle({
        kind: 'entity.create',
        payload: {
          id,
          type: 'system.Entity',
          name: 'Neo4j Entity',
          formId,
          values: {},
        },
      } as any);

      const saved1 = await repo.getEntityById(id);
      expect(saved1).toBeTruthy();
      expect(saved1?.formId).toBe(formId);
      expect(saved1?.values).toEqual({});

      await engine.handle({
        kind: 'entity.setCore',
        payload: { id, name: 'Neo4j Entity v2' },
      } as any);

      const saved2 = await repo.getEntityById(id);
      expect(saved2?.name).toBe('Neo4j Entity v2');

      await engine.handle({
        kind: 'entity.delete',
        payload: { id },
      } as any);

      const gone = await repo.getEntityById(id);
      expect(gone).toBeNull();
    } finally {
      await repo.deleteEntity(id);
    }
  });
});
