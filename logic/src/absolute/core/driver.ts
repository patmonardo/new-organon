import type { ProcessorInputs } from './contracts';
import type { World } from '@schema';

/**
 * Proposal / Commit shapes used by drivers.
 */
export interface Proposal {
  id: string;
  actor: string;
  baseRevision?: number;
  patch?: unknown;
  meta?: Record<string, unknown>;
}

export interface CommitResult {
  success: boolean;
  revision?: number;
  conflicts?: unknown[];
  errors?: string[];
}

export type BackpropagationHook = (summary: unknown) => void | Promise<void>;

/**
 * Base abstract Driver class. Essence drivers should extend this and
 * implement the APIs used by engines.
 */
export default abstract class BaseDriver {
  private hooks: Map<string, BackpropagationHook[]> = new Map();

  constructor(public readonly name: string) {}

  /**
   * World assemblers override this. Non-world drivers can ignore it.
   * Default throws to make accidental calls explicit.
   */
  assemble(_input: ProcessorInputs): World {
    throw new Error(`${this.name} does not implement assemble()`);
  }

  // Optional content indexing
  indexContent?(input: ProcessorInputs): { subtleWorldTotal: number; grossByThing: Record<string, number> };

  // Proposal/commit lifecycle (optional for simple drivers)
  propose?(p: Proposal): Promise<CommitResult> | CommitResult;
  commit?(patch: unknown, meta?: unknown): Promise<CommitResult> | CommitResult;

  registerHook(name: string, fn: BackpropagationHook) {
    const prev = this.hooks.get(name) ?? [];
    prev.push(fn);
    this.hooks.set(name, prev);
  }

  async invokeHooks(name: string, payload: unknown) {
    const hs = this.hooks.get(name) ?? [];
    for (const h of hs) {
      try {
        // allow hooks to be async
        // eslint-disable-next-line no-await-in-loop
        await h(payload);
      } catch (err) {
        // swallow hook errors; drivers may override to surface
        // but avoid breaking the core flow
        // eslint-disable-next-line no-console
        console.warn(`${this.name} hook ${name} failed:`, err);
      }
    }
  }

  emitTrace?(event: string, payload?: unknown): void;

  // Conversion helpers may be overridden by drivers
  toActive?(input: unknown): unknown;
  fromActive?(input: unknown): unknown;

  /**
   * Extract a canonical id from a variety of payload shapes.
   * Prefer payload.shape.core.id, fall back to payload.id or payload.shape.id.
   */
  protected extractIdFromPayload(payload: unknown): string | undefined {
    if (!payload || typeof payload !== 'object') return undefined;
    const p = payload as Record<string, any>;
    const shape = p.shape as Record<string, any> | undefined;
    const core = shape?.core as Record<string, any> | undefined;
    if (core && core.id) return String(core.id);
    if (p.id) return String(p.id);
    if (shape && (shape.id || (shape.core && shape.core.id))) return String(shape.id ?? shape.core?.id ?? '');
    return undefined;
  }

  /**
   * Normalize an engine/service event so callers can reliably read payload.shape.core.id.
   * If only payload.id exists, copy it into payload.shape.core.id so consumers can use the canonical path.
   */
  protected normalizeEvent(evt: any) {
    if (!evt || typeof evt !== 'object') return evt;
    const payload = (evt.payload ?? {}) as Record<string, any>;
    const id = this.extractIdFromPayload(payload);
    if (!id) return evt;
    // Ensure payload.shape.core.id exists
    const shape = payload.shape ?? {};
    const core = shape.core ?? {};
    if (!core.id) {
      const normalizedPayload = {
        ...payload,
        shape: {
          ...shape,
          core: {
            ...core,
            id,
          },
        },
      };
      return { ...evt, payload: normalizedPayload };
    }
    return evt;
  }
}
