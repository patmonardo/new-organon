import { makeInMemoryRepository } from './memory';
import {
  MorphSchema,
  EntitySchema,
  PropertyShapeSchema,
  AspectSchema,
} from '../schema';

export const Repos = {
  morph: () => makeInMemoryRepository(MorphSchema),
  entity: () => makeInMemoryRepository(EntitySchema),
  property: () => makeInMemoryRepository(PropertyShapeSchema),
  aspect: () => makeInMemoryRepository(AspectSchema),
};
