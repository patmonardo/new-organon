import { describe, it, expect } from 'vitest';

import { GdsFormProgramSchema } from '../src/schema/program';

describe('GdslMorphSchema integration', () => {
	it('accepts morph.patterns and optional morph.steps', () => {
		const program = GdsFormProgramSchema.parse({
			morph: {
				patterns: ['essence', 'shine', 'reflection'],
				steps: [
					{ kind: 'form', op: 'essence' },
					{ kind: 'form', op: 'shine' },
					{ kind: 'form', op: 'reflection' },
					{ kind: 'judge', moment: 'reflection' },
					{ kind: 'syllogize' },
				],
			},
		});

		expect(program.morph.patterns).toEqual(['essence', 'shine', 'reflection']);
		expect((program.morph as any).steps?.length).toBe(5);
	});
});
