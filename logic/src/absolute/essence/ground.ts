import { InMemoryEventBus, type EventBus } from '../core/bus';
import { makeInMemoryRepository } from '../../repository/memory';
import { MorphSchema, type Morph } from '../../schema/morph';
import { MorphEngine } from '../../relative/form/morph/morph-engine';
import type { Repository } from '../../repository/repo';
import type { Event } from '../core/message';
import BaseDriver from '../core/driver';
import { ActiveMorph } from '../../schema/active';
import { createWorld, type World } from '../../schema/world';

export class GroundDriver extends BaseDriver {
  private readonly morphEngine: MorphEngine;
  private readonly repo: Repository<Morph>;
  private readonly eventBus: EventBus;

  constructor(repo?: Repository<Morph>, bus?: EventBus) {
    super('GroundDriver');

    this.eventBus = bus ?? new InMemoryEventBus();
    this.repo = repo ?? (makeInMemoryRepository(MorphSchema as any) as unknown as Repository<Morph>);
    this.morphEngine = new MorphEngine(this.repo, this.eventBus);
  }

  // BaseDriver requirement - simple world assembly
  assemble(_input: any): World {
    return createWorld({
      type: 'system.World',
      name: 'Ground',
      horizon: { stage: 'ground' },
    });
  }

  // Safe ID extraction helper
  private extractId(events: Event[], expectedKind: string): string {
    const event = events.find((e) => e?.kind === expectedKind);
    if (!event) {
      throw new Error(`No ${expectedKind} event found in response`);
    }

    if (!event.payload) {
      throw new Error(`${expectedKind} event has no payload`);
    }

    const rawId = (event.payload as any)?.id;
    if (typeof rawId !== 'string' && typeof rawId !== 'number') {
      throw new Error(`${expectedKind} event payload.id must be string or number. Got: ${typeof rawId}`);
    }

    return String(rawId);
  }

  // Process morphs using ActiveMorph schema
  async processMorphs(
    morphs: Array<ActiveMorph>,
    particulars: any[] = [],
    context?: any,
  ) {
    return this.morphEngine.process(morphs, particulars, context);
  }

  // Commit processed actions
  async commitMorphs(actions: any[], snapshot: { count: number }) {
    return this.morphEngine.commit(actions, snapshot);
  }

  // Process and commit in one step
  async processAndCommitMorphs(
    morphs: Array<ActiveMorph>,
    particulars: any[] = [],
    context?: any,
  ) {
    const { actions, snapshot } = await this.processMorphs(morphs, particulars, context);
    return this.commitMorphs(actions, snapshot);
  }

  // Direct morph access via engine
  async getMorph(id: string) {
    return this.morphEngine.getMorph(id);
  }

  // Create morph via engine
  async createMorph(input: {
    id?: string;
    type: string;
    name?: string;
    state?: any;
  }): Promise<string> {
    const events = await this.morphEngine.handle({
      kind: 'morph.create',
      payload: input,
    } as any);

    return this.extractId(events, 'morph.create');
  }

  // Delete morph via engine
  async deleteMorph(id: string) {
    await this.morphEngine.handle({
      kind: 'morph.delete',
      payload: { id },
    } as any);
  }
}

// Functional interfaces
export async function processMorphs(
  morphs: Array<ActiveMorph>,
  driver: GroundDriver = DefaultGroundDriver,
  particulars: any[] = [],
  context?: any,
) {
  return driver.processMorphs(morphs, particulars, context);
}

export async function commitMorphs(
  actions: any[],
  snapshot: { count: number },
  driver: GroundDriver = DefaultGroundDriver,
) {
  return driver.commitMorphs(actions, snapshot);
}

export const DefaultGroundDriver = new GroundDriver();
