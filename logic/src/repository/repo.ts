import type { z } from "zod";
import type { Base } from "../schema";
import { Label } from "../schema";

export type IdLike = string;

export interface FindOptions {
  limit?: number;
  offset?: number;
}

export interface Concurrency {
  expectedRevision?: number;
}

export interface Repository<TDoc extends Base> {
  get(id: IdLike): Promise<TDoc | null>;
  create(doc: TDoc): Promise<TDoc>;
  update(
    id: IdLike,
    patch: Partial<TDoc> | ((current: TDoc) => TDoc),
    opts?: Concurrency
  ): Promise<TDoc>;
  delete(id: IdLike): Promise<boolean>;
  list(opts?: FindOptions): Promise<TDoc[]>;
  byType(type: string, opts?: FindOptions): Promise<TDoc[]>;
  byTag(tag: z.infer<typeof Label>, opts?: FindOptions): Promise<TDoc[]>;
}

// Accessors for BaseSchema-shaped docs
export function getId<T extends Base>(doc: T): string {
  return doc.shape.core.id;
}
export function getType<T extends Base>(doc: T): string {
  return doc.shape.core.type as unknown as string;
}

// Optional tags accessor (absent in skeletal schemas)
// Indexer will treat missing/non-array as empty.
export function getTags<T extends Base>(doc: T): string[] {
  const tags = (doc.shape as any)?.state?.tags;
  return Array.isArray(tags) ? tags.filter((t) => typeof t === "string") : [];
}
