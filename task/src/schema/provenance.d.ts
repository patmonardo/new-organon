import { z } from 'zod';
export declare const KnowledgeOriginSchema: z.ZodEnum<["empirical", "transcendental", "synthetic", "reflective"]>;
export declare const EvidenceSchema: z.ZodObject<{
    id: z.ZodString;
    kind: z.ZodDefault<z.ZodEnum<["observation", "experiment", "derivation", "citation", "model", "testimonial", "other"]>>;
    description: z.ZodOptional<z.ZodString>;
    dataRef: z.ZodOptional<z.ZodString>;
    strength: z.ZodOptional<z.ZodNumber>;
    timestamp: z.ZodOptional<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    kind: "model" | "observation" | "experiment" | "derivation" | "citation" | "testimonial" | "other";
    description?: string | undefined;
    timestamp?: Date | undefined;
    strength?: number | undefined;
    dataRef?: string | undefined;
    metadata?: Record<string, any> | undefined;
}, {
    id: string;
    description?: string | undefined;
    timestamp?: Date | undefined;
    strength?: number | undefined;
    kind?: "model" | "observation" | "experiment" | "derivation" | "citation" | "testimonial" | "other" | undefined;
    dataRef?: string | undefined;
    metadata?: Record<string, any> | undefined;
}>;
export declare const ProvenanceSchema: z.ZodObject<{
    id: z.ZodString;
    origin: z.ZodEnum<["empirical", "transcendental", "synthetic", "reflective"]>;
    sources: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    evidence: z.ZodOptional<z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        kind: z.ZodDefault<z.ZodEnum<["observation", "experiment", "derivation", "citation", "model", "testimonial", "other"]>>;
        description: z.ZodOptional<z.ZodString>;
        dataRef: z.ZodOptional<z.ZodString>;
        strength: z.ZodOptional<z.ZodNumber>;
        timestamp: z.ZodOptional<z.ZodDate>;
        metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        kind: "model" | "observation" | "experiment" | "derivation" | "citation" | "testimonial" | "other";
        description?: string | undefined;
        timestamp?: Date | undefined;
        strength?: number | undefined;
        dataRef?: string | undefined;
        metadata?: Record<string, any> | undefined;
    }, {
        id: string;
        description?: string | undefined;
        timestamp?: Date | undefined;
        strength?: number | undefined;
        kind?: "model" | "observation" | "experiment" | "derivation" | "citation" | "testimonial" | "other" | undefined;
        dataRef?: string | undefined;
        metadata?: Record<string, any> | undefined;
    }>, "many">>;
    agentId: z.ZodOptional<z.ZodString>;
    createdAt: z.ZodDefault<z.ZodEffects<z.ZodDate, Date, unknown>>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    origin: "transcendental" | "empirical" | "synthetic" | "reflective";
    createdAt: Date;
    metadata?: Record<string, any> | undefined;
    sources?: string[] | undefined;
    evidence?: {
        id: string;
        kind: "model" | "observation" | "experiment" | "derivation" | "citation" | "testimonial" | "other";
        description?: string | undefined;
        timestamp?: Date | undefined;
        strength?: number | undefined;
        dataRef?: string | undefined;
        metadata?: Record<string, any> | undefined;
    }[] | undefined;
    agentId?: string | undefined;
}, {
    id: string;
    origin: "transcendental" | "empirical" | "synthetic" | "reflective";
    metadata?: Record<string, any> | undefined;
    sources?: string[] | undefined;
    evidence?: {
        id: string;
        description?: string | undefined;
        timestamp?: Date | undefined;
        strength?: number | undefined;
        kind?: "model" | "observation" | "experiment" | "derivation" | "citation" | "testimonial" | "other" | undefined;
        dataRef?: string | undefined;
        metadata?: Record<string, any> | undefined;
    }[] | undefined;
    agentId?: string | undefined;
    createdAt?: unknown;
}>;
export type KnowledgeOrigin = z.infer<typeof KnowledgeOriginSchema>;
export type Evidence = z.infer<typeof EvidenceSchema>;
export type Provenance = z.infer<typeof ProvenanceSchema>;
//# sourceMappingURL=provenance.d.ts.map