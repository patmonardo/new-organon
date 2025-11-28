import { describe, it, expect, beforeEach } from "vitest";
import {
  MockEntityService,
  MockDashboardService,
  type EntityInput,
  type DashboardInput
} from '../src/data';

describe("EntityService", () => {
  let service: MockEntityService;

  beforeEach(() => {
    service = new MockEntityService();
  });

  it('creates an entity', async () => {
    const input: EntityInput = {
      type: 'concept',
      name: 'Brahman',
      data: { category: 'metaphysics' }
    };

    const entity = await service.create(input);

    expect(entity.id).toBeDefined();
    expect(entity.type).toBe('concept');
    expect(entity.name).toBe('Brahman');
    expect(entity.category).toBe('metaphysics');
  });

  it('finds entity by id', async () => {
    const created = await service.create({ type: 'concept', name: 'Test' });
    const found = await service.findById(created.id);

    expect(found).toBeDefined();
    expect(found?.id).toBe(created.id);
  });

  it('filters entities by type', async () => {
    await service.create({ type: 'concept', name: 'A' });
    await service.create({ type: 'concept', name: 'B' });
    await service.create({ type: 'text', name: 'C' });

    const concepts = await service.findMany({ type: 'concept' });

    expect(concepts.length).toBe(2);
  });

  it('updates an entity', async () => {
    const created = await service.create({ type: 'concept', name: 'Original' });
    const updated = await service.update(created.id, { name: 'Updated' });

    expect(updated.name).toBe('Updated');
    expect(updated.type).toBe('concept');
  });

  it('deletes an entity', async () => {
    const created = await service.create({ type: 'concept' });
    const deleted = await service.delete(created.id);
    const found = await service.findById(created.id);

    expect(deleted).toBe(true);
    expect(found).toBeNull();
  });
});

describe("DashboardService", () => {
  let service: MockDashboardService;

  beforeEach(() => {
    service = new MockDashboardService();
  });

  it('creates a dashboard', async () => {
    const input: DashboardInput = {
      name: 'Test Dashboard',
      title: 'Test Title',
      description: 'Test Description'
    };

    const dashboard = await service.create(input);

    expect(dashboard.id).toBeDefined();
    expect(dashboard.name).toBe('Test Dashboard');
    expect(dashboard.config).toBeDefined();
    expect(dashboard.config.type).toBe('dashboard');
  });

  it('finds dashboard by id', async () => {
    const created = await service.create({ name: 'Test' });
    const found = await service.findById(created.id);

    expect(found).toBeDefined();
    expect(found?.id).toBe(created.id);
  });

  it('sets default dashboard', async () => {
    const d1 = await service.create({ name: 'Dashboard 1', isDefault: true });
    const d2 = await service.create({ name: 'Dashboard 2' });

    // d1 is default
    let defaultDash = await service.findDefault();
    expect(defaultDash?.id).toBe(d1.id);

    // Set d2 as default
    await service.setDefault(d2.id);
    defaultDash = await service.findDefault();
    expect(defaultDash?.id).toBe(d2.id);

    // d1 should no longer be default
    const d1Updated = await service.findById(d1.id);
    expect(d1Updated?.isDefault).toBe(false);
  });

  it('updates a dashboard', async () => {
    const created = await service.create({ name: 'Original' });
    const updated = await service.update(created.id, {
      title: 'New Title',
      config: { components: [] } as any
    });

    expect(updated.title).toBe('New Title');
  });

  it('deletes a dashboard', async () => {
    const created = await service.create({ name: 'ToDelete' });
    const deleted = await service.delete(created.id);
    const found = await service.findById(created.id);

    expect(deleted).toBe(true);
    expect(found).toBeNull();
  });
});
