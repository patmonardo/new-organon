import { describe, expect, it } from 'vitest';

import {
	GdsGraphStoreCatalogCallSchema,
	gdsApplicationOperationId,
} from '../src/ir/gds.application';

describe('@organon/gdsl smoke', () => {
	it('parses a minimal call and computes stable operation id', () => {
		const call = GdsGraphStoreCatalogCallSchema.parse({
			facade: 'graph_store_catalog',
			op: 'list_graphs',
			user: { username: 'test' },
			databaseId: 'neo4j',
		});

		expect(gdsApplicationOperationId(call)).toBe(
			'gds.graph_store_catalog.list_graphs',
		);
	});
});
