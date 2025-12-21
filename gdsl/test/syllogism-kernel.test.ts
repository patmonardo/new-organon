import { describe, it, expect } from 'vitest';

import { StubSyllogismKernelPort } from '../src/syllogism-kernel';
import { SyllogismArtifactSchema } from '../src/schema/syllogism';

describe('StubSyllogismKernelPort', () => {
	it('produces a SyllogismArtifact from morph patterns (+ optional judgment)', async () => {
		const kernel = new StubSyllogismKernelPort();
		const result = await kernel.sylogize({
			morphPatterns: ['essence', 'shine', 'reflection'],
			judgment: {
				thesis: 'Contradiction-free foundation is presented',
				grounds: ['foundation.moment=infinite'],
			},
			proof: { kind: 'reflection', citta: true },
		});

		expect(result.ok).toBe(true);
		const artifact = SyllogismArtifactSchema.parse(result.artifact);
		expect(artifact.kind).toBe('syllogism');
		expect(artifact.morphPatterns).toEqual(['essence', 'shine', 'reflection']);
		expect(artifact.conclusion).toContain('Contradiction-free');
		expect(artifact.premises.length).toBeGreaterThan(0);
		// ground should be explicit in premise 1
		expect(artifact.premises[0]?.thesis).toContain('Ground');
	});
});
