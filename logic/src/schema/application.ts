import { z } from 'zod';

import {
  GdsApplicationFormKindSchema,
  GdsDatabaseIdSchema,
  GdsUserSchema,
} from './common';

/**
 * Shared base shape for all GDS application calls.
 *
 * Rust expects `{ facade, op, ... }` payloads for TS-JSON routing.
 */
export function gdsApplicationBase<F extends z.ZodTypeAny>(facade: F) {
  return z.object({
    kind: GdsApplicationFormKindSchema.optional(),
    facade,
    user: GdsUserSchema,
    databaseId: GdsDatabaseIdSchema,
  });
}

import {
  GdsGraphStoreCatalogCallSchema,
  type GdsGraphStoreCatalogCall,
} from './graph-store-catalog';
import { GdsGraphStoreCallSchema, type GdsGraphStoreCall } from './graph-store';
import { GdsAlgorithmsCallSchema, type GdsAlgorithmsCall } from './algorithms';

export const GdsApplicationCallSchema = z.union([
  GdsGraphStoreCatalogCallSchema,
  GdsGraphStoreCallSchema,
  GdsAlgorithmsCallSchema,
]);

export type GdsApplicationCall =
  | GdsGraphStoreCatalogCall
  | GdsGraphStoreCall
  | GdsAlgorithmsCall;

export function gdsApplicationOperationId(call: GdsApplicationCall): string {
  return `gds.${call.facade}.${call.op}`;
}
