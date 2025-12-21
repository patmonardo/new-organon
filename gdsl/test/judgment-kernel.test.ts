import { describe, it, expect } from 'vitest';

import { StubJudgmentKernelPort } from '../src/judgment-kernel';
import { JudgmentArtifactSchema } from '../src/schema/judgment';

describe('StubJudgmentKernelPort', () => {
	it('produces a JudgmentArtifact from foundation thesis', async () => {
		const kernel = new StubJudgmentKernelPort();
		const result = await kernel.judge({
			moment: 'reflection',
			phenomenology: {
				contradictions: [{ id: 'c1', claim: 'A and not-A both asserted' }],
				foundation: {
					moment: 'infinite',
					thesis: 'Contradiction-free foundation is presented',
					resolves: ['c1'],
				},
			},
			proof: { kind: 'reflection', citta: true },
		});

		expect(result.ok).toBe(true);
		const artifact = JudgmentArtifactSchema.parse(result.artifact);
		expect(artifact.kind).toBe('judgment');
		expect(artifact.moment).toBe('reflection');
		expect(artifact.foundationMoment).toBe('infinite');
		expect(artifact.thesis).toContain('Contradiction-free');
		expect(artifact.tokens.length).toBeGreaterThan(0);
		expect(artifact.grounds?.some((g) => g.includes('foundation.resolves=c1'))).toBe(true);
	});
});
