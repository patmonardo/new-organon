import { makeInMemoryRepository } from "./memory";
import {
  ContextSchema,
  MorphSchema,
  EntitySchema,
  PropertySchema,
  AspectSchema,
} from "../schema";

export const Repos = {
  // shape removed - use FormShapeRepository directly
  context: () => makeInMemoryRepository(ContextSchema),
  morph: () => makeInMemoryRepository(MorphSchema),
  entity: () => makeInMemoryRepository(EntitySchema),
  property: () => makeInMemoryRepository(PropertySchema),
  aspect: () => makeInMemoryRepository(AspectSchema),
};
