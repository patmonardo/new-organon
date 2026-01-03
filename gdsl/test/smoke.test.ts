import { describe, expect, it } from 'vitest';

import { gdsApplicationOperationId } from '../src/schema/application';

describe('@organon/gdsl smoke', () => {
	it('operation id functions are defined', () => {
		expect(gdsApplicationOperationId).toBeDefined();
	});
});
