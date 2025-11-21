import { makeInMemoryRepository } from "./memory";
import {
  ShapeSchema,
  ContextSchema,
  MorphSchema,
  EntitySchema,
  PropertySchema,
  AspectSchema,

} from "../schema";

export const Repos = {
  shape: () => makeInMemoryRepository(ShapeSchema),
  context: () => makeInMemoryRepository(ContextSchema),
  morph: () => makeInMemoryRepository(MorphSchema),
  entity: () => makeInMemoryRepository(EntitySchema),
  property: () => makeInMemoryRepository(PropertySchema),
  aspect: () => makeInMemoryRepository(AspectSchema),
};
