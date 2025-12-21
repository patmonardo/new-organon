import { describe, it, expect } from 'vitest';

import { seedReflectionFormProgram } from '../../src/relative/form/dialectic/index.js';
import { seedSyllogismInputFromProgram } from '../../src/relative/form/dialectic/index.js';

describe('active ground → syllogism seed', () => {
	it('extracts morph patterns as active ground', () => {
		const program = seedReflectionFormProgram();
		const input = seedSyllogismInputFromProgram({ program });
		expect(input.morphPatterns).toEqual(['essence', 'shine', 'reflection']);
	});
});
