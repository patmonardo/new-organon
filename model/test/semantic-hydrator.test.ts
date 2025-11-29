import { describe, it, expect } from 'vitest';
import { z } from 'zod';
import {
  SemanticHydrator,
  type SemanticDataService,
  type HydratorSpec,
  type SemanticResult,
} from '../src/data/semantic-hydrator';
import { defineModel } from '../src/data/sdsl';
import { SimpleFormController } from '../src/sdsl/form-controller';
import type { FormShape } from '../src/sdsl/types';

const TestModel = defineModel<{ id: string; name: string; orders: unknown[]; metrics: { total: number } }>({
  name: 'TestModel',
  source: 'test_source',
  fields: {
    id: z.string(),
    name: z.string(),
    orders: z.array(z.any()),
    metrics: z.object({ total: z.number() }),
  },
});

const testShape: FormShape = {
  id: 'test-form',
  name: 'test',
  title: 'Test',
  fields: [
    { id: 'id', type: 'text', label: 'ID', required: false, disabled: false },
    { id: 'name', type: 'text', label: 'Name', required: false, disabled: false },
    { id: 'orderCount', type: 'number', label: 'Order Count', required: false, disabled: false },
    { id: 'orders', type: 'json', label: 'Orders', required: false, disabled: false },
    { id: 'plan', type: 'text', label: 'Plan', required: false, disabled: false },
  ],
};

class StubSemanticService implements SemanticDataService {
  async execute(): Promise<SemanticResult> {
    const row = {
      id: 'cust_1',
      name: 'Acme',
      orders: [
        { id: 'o-1', amount: 100 },
        { id: 'o-2', amount: 50 },
      ],
      metrics: {
        total: 150,
      },
    };

    return {
      plan: JSON.stringify({ source: 'test_source', limit: 1 }),
      rows: [row],
      meta: { engine: 'stub' },
    };
  }
}

describe('SemanticHydrator', () => {
  it('maps fields, collections, and metrics into the form model', async () => {
    const controller = new SimpleFormController(testShape);
    const hydrator = new SemanticHydrator(new StubSemanticService());

    const spec: HydratorSpec = {
      id: 'test-spec',
      view: () => TestModel.view({ limit: 1 }),
      fields: [
        { fieldId: 'id', source: 'id' },
        { fieldId: 'name', source: 'name' },
      ],
      collections: [
        { id: 'orders', source: 'orders', fieldId: 'orders', limit: 1 },
      ],
      metrics: [
        { name: 'orderCount', derive: rows => rows[0]?.orders ? (rows[0].orders as unknown[]).length : 0, fieldId: 'orderCount' },
        { name: 'totalRevenue', source: 'metrics.total' },
      ],
      metaFields: {
        '$plan': 'plan',
      },
    };

    const snapshot = await hydrator.hydrate(controller.model, spec);

    expect(snapshot.metrics.totalRevenue).toBe(150);
    expect(snapshot.collections.orders).toHaveLength(1);
    expect(snapshot.rows[0].name).toBe('Acme');
    expect(controller.model.getField('id')).toBe('cust_1');
    expect(controller.model.getField('orderCount')).toBe(2);
    expect(controller.model.getField('plan')).toContain('test_source');
    expect(snapshot.assignedFields).toEqual(expect.arrayContaining(['id', 'name', 'orders', 'orderCount', 'plan']));
  });
});
