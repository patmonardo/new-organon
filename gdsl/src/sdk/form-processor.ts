import {
    ConceptArtifact,
    ConceptArtifactSchema,
    GdsFormEvalCall,
    GdsFormEvalResult,
    GdsFormEvalResultSchema,
    SyllogismArtifact,
    SyllogismArtifactSchema,
} from '../schema';

/**
 * Minimal TS-first FormProcessor stub.
 *
 * - Treats `program.morph.patterns` as the operative ground.
 * - Emits a placeholder Concept and optional Syllogism for reflective modes
 *   (allness, induction, analogy) without invoking the Rust kernel.
 * - Schema-first: validates inputs/outputs with zod.
 */
export class FormProcessor {
    async evaluate(call: Extract<GdsFormEvalCall, { op: 'evaluate' }>): Promise<GdsFormEvalResult> {
        const patterns = call.program.morph.patterns ?? [];

        const concepts: ConceptArtifact[] = [];
        const syllogisms: SyllogismArtifact[] = [];

        // Produce a Concept artifact from morph patterns
        const conceptCandidate: ConceptArtifact = {
            kind: 'concept',
            morphPatterns: patterns,
            meta: {
                source: 'gdsl.form-processor.ts',
                note: 'Speculative placeholder — subject to redesign',
            },
        };
        concepts.push(ConceptArtifactSchema.parse(conceptCandidate));

        // Reflective syllogisms supported (speculative placeholders)
        const has = (name: string) => patterns.some((p) => p.toLowerCase().includes(name));

        if (has('allness')) {
            syllogisms.push(
                SyllogismArtifactSchema.parse({
                    kind: 'syllogism',
                    morphPatterns: patterns,
                    premises: [
                        { thesis: 'All A are B' },
                        { thesis: 'x is A' },
                    ],
                    conclusion: 'Therefore x is B',
                    meta: { mode: 'allness' },
                }),
            );
        }

        if (has('induction')) {
            syllogisms.push(
                SyllogismArtifactSchema.parse({
                    kind: 'syllogism',
                    morphPatterns: patterns,
                    premises: [
                        { thesis: 'Observed instances x1..xn have property P' },
                    ],
                    conclusion: 'Likely all X have property P (induction)',
                    meta: { mode: 'induction' },
                }),
            );
        }

        if (has('analogy')) {
            syllogisms.push(
                SyllogismArtifactSchema.parse({
                    kind: 'syllogism',
                    morphPatterns: patterns,
                    premises: [
                        { thesis: 'Structure S is similar between domains D1 and D2' },
                    ],
                    conclusion: 'Therefore property P may transfer by analogy',
                    meta: { mode: 'analogy' },
                }),
            );
        }

        const result: GdsFormEvalResult = GdsFormEvalResultSchema.parse({
            concepts,
            syllogisms,
            artifacts: call.artifacts ?? {},
            meta: {
                user: call.user,
                databaseId: call.databaseId,
                graphName: call.graphName,
                op: 'evaluate',
            },
        });

        return result;
    }
}
