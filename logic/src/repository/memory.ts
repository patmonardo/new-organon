import type { z } from "zod";
import type { Base } from "../schema";
import {
  Repository,
  FindOptions,
  Concurrency,
  getId,
  getType,
  getTags,
} from "./repo";

type AnySchema = z.ZodTypeAny;
type DocOf<S extends AnySchema> = z.infer<S> & Base;

type Index = {
  byType: Map<string, Set<string>>;
  byTag: Map<string, Set<string>>;
};

export class InMemoryRepository<S extends AnySchema>
  implements Repository<DocOf<S>>
{
  private store = new Map<string, DocOf<S>>();
  private index: Index = { byType: new Map(), byTag: new Map() };

  constructor(private readonly schema: S) {}

  async get(id: string): Promise<DocOf<S> | null> {
    return this.store.get(id) ?? null;
  }

  async create(doc: DocOf<S>): Promise<DocOf<S>> {
    const parsed = this.schema.parse(doc) as DocOf<S>;
    const id = getId(parsed);
    if (this.store.has(id)) throw new Error(`conflict: id exists (${id})`);
    this.store.set(id, parsed);
    this.addToIndex(parsed);
    return parsed;
  }

  async update(
    id: string,
    patch: Partial<DocOf<S>> | ((current: DocOf<S>) => DocOf<S>),
    opts?: Concurrency
  ): Promise<DocOf<S>> {
    const current = this.store.get(id);
    if (!current) throw new Error(`not found: ${id}`);
    if (
      opts?.expectedRevision !== undefined &&
      current.revision !== opts.expectedRevision
    ) {
      throw new Error(
        `conflict: revision ${current.revision} !== expected ${opts.expectedRevision}`
      );
    }
    const next =
      typeof patch === "function"
        ? patch(current)
        : ({ ...current, ...patch } as DocOf<S>);
    const candidate = { ...next, revision: current.revision + 1 } as DocOf<S>;
    const parsed = this.schema.parse(candidate) as DocOf<S>;

    this.removeFromIndex(current);
    this.store.set(id, parsed);
    this.addToIndex(parsed);
    return parsed;
  }

  async delete(id: string): Promise<boolean> {
    const curr = this.store.get(id);
    if (!curr) return false;
    this.removeFromIndex(curr);
    return this.store.delete(id);
  }

  async list(opts?: FindOptions): Promise<DocOf<S>[]> {
    return this.slice(Array.from(this.store.values()), opts);
  }

  async byType(type: string, opts?: FindOptions): Promise<DocOf<S>[]> {
    const ids = this.index.byType.get(type);
    if (!ids) return [];
    const docs = Array.from(ids)
      .map((i) => this.store.get(i)!)
      .filter(Boolean) as DocOf<S>[];
    return this.slice(docs, opts);
  }

  async byTag(tag: string, opts?: FindOptions): Promise<DocOf<S>[]> {
    const ids = this.index.byTag.get(tag);
    if (!ids) return [];
    const docs = Array.from(ids)
      .map((i) => this.store.get(i)!)
      .filter(Boolean) as DocOf<S>[];
    return this.slice(docs, opts);
  }

  private slice(arr: DocOf<S>[], opts?: FindOptions) {
    const o = opts?.offset ?? 0;
    const l = opts?.limit ?? arr.length;
    return arr.slice(o, o + l);
  }

  private addToIndex(doc: DocOf<S>) {
    const id = getId(doc);
    const ty = getType(doc);
    const tags = getTags(doc);
    if (!this.index.byType.has(ty)) this.index.byType.set(ty, new Set());
    this.index.byType.get(ty)!.add(id);
    for (const t of tags) {
      if (!this.index.byTag.has(t)) this.index.byTag.set(t, new Set());
      this.index.byTag.get(t)!.add(id);
    }
  }
  private removeFromIndex(doc: DocOf<S>) {
    const id = getId(doc);
    const ty = getType(doc);
    this.index.byType.get(ty)?.delete(id);
    for (const t of getTags(doc)) this.index.byTag.get(t)?.delete(id);
  }
}

// Factory (schema-driven inference)
export function makeInMemoryRepository<S extends AnySchema>(schema: S) {
  return new InMemoryRepository(schema);
}
