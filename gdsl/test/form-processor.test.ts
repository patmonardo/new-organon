import { describe, it, expect } from 'vitest';

import { FormProcessor } from '../src/sdk/form-processor';
import {
    GdsFormEvalCall,
    GdsFormEvalCallSchema,
    GdsFormEvalResultSchema,
} from '../src/schema';

describe('FormProcessor (TS stub)', () => {
    it('produces a Concept and reflective Syllogisms based on morph patterns', async () => {
        const call: GdsFormEvalCall = GdsFormEvalCallSchema.parse({
            facade: 'form_eval',
            op: 'evaluate',
            user: { username: 'u1' },
            databaseId: 'db-dev',
            graphName: 'g0',
            program: {
                morph: {
                    patterns: ['essence', 'reflection', 'analogy', 'induction'],
                },
            },
        });

        const fp = new FormProcessor();
        const res = await fp.evaluate(call);

        const parsed = GdsFormEvalResultSchema.parse(res);
        expect(parsed.concepts.length).toBe(1);
        expect(parsed.concepts[0].morphPatterns).toContain('analogy');

        // Should include at least analogy + induction syllogisms
        const modes = new Set(parsed.syllogisms.map((s) => s.meta && (s.meta as any).mode));
        expect(modes.has('analogy')).toBe(true);
        expect(modes.has('induction')).toBe(true);
    });
});
