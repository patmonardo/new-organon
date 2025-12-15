import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

import {
	GdsGraphStoreCatalogCallSchema,
	gdsApplicationOperationId,
} from '../src/ir/gds.application';

import {
	gdslAlgorithmOperationId,
} from '../src/ir/algorithm';

import { GdslRunRequestSchema } from '../src/ir/run';

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

	it('parses a minimal run request and computes algorithm operation id', () => {
		const json = readFileSync(
			resolve(import.meta.dirname, '../../fixtures/gdsl/run.pagerank.stream.json'),
			'utf8',
		);
		const run = GdslRunRequestSchema.parse(JSON.parse(json));

		expect(gdslAlgorithmOperationId(run.algorithm)).toBe('gds.algorithm.pagerank');
	});
});
