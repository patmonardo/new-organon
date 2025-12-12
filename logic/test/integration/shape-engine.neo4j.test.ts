import { describe, it, expect } from 'vitest';

import { defaultConnection } from '../../src/repository/neo4j-client';
import { createFormDb } from './createFormDb';

const neo4jOk = await defaultConnection.verifyConnectivity();

describe.skipIf(!neo4jOk)('ShapeEngine × Neo4j (createFormDb)', () => {
  it('persists create/setCore/delete through FormShapeRepository', async () => {
    const { shapeEngine: engine, formShapeRepo: repo } = createFormDb({ connection: defaultConnection });

    const id = `shape:neo4j:${Date.now().toString(36)}`;

    try {
      await engine.handle({
        kind: 'shape.create',
        payload: { id, type: 'system.Form', name: 'Neo4j Form' },
      } as any);

      const saved1 = await repo.getFormById(id);
      expect(saved1).toBeTruthy();
      expect(saved1?.id).toBe(id);

      await engine.handle({
        kind: 'shape.setCore',
        payload: { id, name: 'Neo4j Form v2' },
      } as any);

      const saved2 = await repo.getFormById(id);
      expect(saved2?.name).toBe('Neo4j Form v2');

      await engine.handle({
        kind: 'shape.delete',
        payload: { id },
      } as any);

      const gone = await repo.getFormById(id);
      expect(gone).toBeNull();
    } finally {
      // Best-effort cleanup (in case the test fails mid-way)
      await repo.deleteForm(id);
    }
  });
});
