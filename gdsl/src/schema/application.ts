import { z } from 'zod';

import {
	GdsGraphStoreCatalogCallSchema,
	type GdsGraphStoreCatalogCall,
} from './graph-store-catalog';
import {
	GdsGraphStoreCallSchema,
	type GdsGraphStoreCall,
} from './graph-store';
import {
	GdsAlgorithmsCallSchema,
	type GdsAlgorithmsCall,
} from './algorithms';

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



