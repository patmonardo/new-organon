import type { Base } from '../../src/schema';
import type {
  Repository,
  Concurrency,
  FindOptions,
} from '../../src/repository/repo';
import { getId } from '../../src/repository/repo';

export function makeInMemoryRepository<T extends Base>(): Repository<T> {
  const store = new Map<string, T>();

  function applyFindOptions(items: T[], opts?: FindOptions): T[] {
    const offset = opts?.offset ?? 0;
    const limit = opts?.limit ?? items.length;
    return items.slice(offset, offset + limit);
  }

  return {
    async create(doc: T): Promise<T> {
      const id = getId(doc);
      store.set(id, doc);
      return doc;
    },

    async get(id: string): Promise<T | null> {
      return store.get(id) ?? null; // Repository.get -> null when missing
    },

    async update(
      id: string,
      patch: Partial<T> | ((current: T) => T),
      opts?: Concurrency,
    ): Promise<T> {
      const current = store.get(id);
      if (!current) throw new Error(`not found: ${id}`);

      // Enforce concurrency only if provided (handle undefined vs null safely)
      const exp = (opts as Concurrency | undefined)?.expectedRevision;
      if (exp !== undefined && exp !== null) {
        const rev = (current as any).revision ?? 0;
        if (rev !== exp) {
          throw new Error(`revision conflict: expected ${exp}, got ${rev}`);
        }
      }

      const next =
        typeof patch === 'function'
          ? (patch as (c: T) => T)(current)
          : ({ ...(current as any), ...(patch as any) } as T);

      store.set(id, next);
      return next;
    },

    async delete(id: string): Promise<boolean> {
      return store.delete(id);
    },

    async list(opts?: FindOptions): Promise<T[]> {
      const items = Array.from(store.values());
      return applyFindOptions(items, opts);
    },

    async byType(type: string, opts?: FindOptions): Promise<T[]> {
      const items = Array.from(store.values()).filter(
        (d) => (d as any).shape?.core?.type === type,
      );
      return applyFindOptions(items, opts);
    },

    async byTag(tag: string, opts?: FindOptions): Promise<T[]> {
      const items = Array.from(store.values()).filter((d) =>
        ((d as any).shape?.state?.tags ?? []).includes(tag),
      );
      return applyFindOptions(items, opts);
    },
  };
}
