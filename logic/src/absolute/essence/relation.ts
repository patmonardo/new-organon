import { InMemoryEventBus, type EventBus } from '../core/bus';
import { makeInMemoryRepository } from '../../repository/memory';
import { AspectSchema, type Aspect } from '../../schema/aspect';
import { AspectEngine } from '../../relative/form/aspect/aspect-engine';
import type { Repository } from '../../repository/repo';
import type { Event } from '../core/message';
import BaseDriver from '../core/driver';
import { ActiveAspect } from '../../schema/active';
import { createWorld, type World } from '../../schema/world';

export class RelationDriver extends BaseDriver {
  private readonly aspectEngine: AspectEngine;
  private readonly repo: Repository<Aspect>;
  private readonly eventBus: EventBus;

  constructor(repo?: Repository<Aspect>, bus?: EventBus) {
    super('RelationDriver');

    this.eventBus = bus ?? new InMemoryEventBus();
    this.repo = repo ?? (makeInMemoryRepository(AspectSchema as any) as unknown as Repository<Aspect>);
    this.aspectEngine = new AspectEngine(this.repo, this.eventBus);
  }

  // BaseDriver requirement - simple world assembly
  assemble(_input: any): World {
    return createWorld({
      type: 'system.World',
      name: 'Aspect',
      horizon: { stage: 'aspect' },
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

  // Process aspects using ActiveAspect schema
  async processAspects(
    aspects: Array<ActiveAspect>,
    particulars: any[] = [],
    context?: any,
  ) {
    return this.aspectEngine.process(aspects, particulars, context);
  }

  // Commit processed actions
  async commitAspects(actions: any[], snapshot: { count: number }) {
    return this.aspectEngine.commit(actions, snapshot);
  }

  // Process and commit in one step
  async processAndCommitAspects(
    aspects: Array<ActiveAspect>,
    particulars: any[] = [],
    context?: any,
  ) {
    const { actions, snapshot } = await this.processAspects(aspects, particulars, context);
    return this.commitAspects(actions, snapshot);
  }

  // Direct aspect access via engine
  async getAspect(id: string) {
    return this.aspectEngine.getAspect(id);
  }

  // Create aspect via engine
  async createAspect(input: {
    id?: string;
    type: string;
    name: string;
    state?: any;
  }): Promise<string> {
    const events = await this.aspectEngine.handle({
      kind: 'aspect.create',
      payload: input,
    } as any);

    return this.extractId(events, 'aspect.created');
  }

  // Delete aspect via engine
  async deleteAspect(id: string) {
    await this.aspectEngine.handle({
      kind: 'aspect.delete',
      payload: { id },
    } as any);
  }
}

// Functional interfaces
export async function processAspects(
  aspects: Array<ActiveAspect>,
  driver: RelationDriver = DefaultRelationDriver,
  particulars: any[] = [],
  context?: any,
) {
  return driver.processAspects(aspects, particulars, context);
}

export async function commitAspects(
  actions: any[],
  snapshot: { count: number },
  driver: RelationDriver = DefaultRelationDriver,
) {
  return driver.commitAspects(actions, snapshot);
}

export const DefaultRelationDriver = new RelationDriver();
