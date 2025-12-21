import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

import {
	GdsGraphStoreCatalogCallSchema,
	gdsApplicationOperationId,
} from '../src/schema/gds.application';

import {
	GdsGraphStorePutCallSchema,
} from '../src/schema/gds.graph-store';

import {
	gdslAlgorithmOperationId,
} from '../src/schema/algorithm';

import { GdslRunRequestSchema } from '../src/schema/run';

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

	it('parses a graph_store put call and computes stable operation id', () => {
		const call = GdsGraphStorePutCallSchema.parse({
			facade: 'graph_store',
			op: 'put',
			user: { username: 'test' },
			databaseId: 'neo4j',
			graphName: 'stash1',
			snapshot: {
				nodes: [0, 1],
				relationships: [{ type: 'KNOWS', source: 0, target: 1 }],
			},
		});

		expect(gdsApplicationOperationId(call)).toBe('gds.graph_store.put');
	});

	it('parses a minimal run request and computes algorithm operation id', () => {
		const json = readFileSync(
			resolve(import.meta.dirname, '../../tools/fixtures/gdsl/run.pagerank.stream.json'),
			'utf8',
		);
		const run = GdslRunRequestSchema.parse(JSON.parse(json));

		expect(gdslAlgorithmOperationId(run.algorithm)).toBe('gds.algorithm.pagerank');
	});
});
