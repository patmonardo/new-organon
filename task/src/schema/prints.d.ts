import { z } from 'zod';
export declare const PrintKind: z.ZodEnum<["knowing", "conceiving", "taw", "facttrace", "ml", "graph", "proof"]>;
export declare const PrintRole: z.ZodEnum<["kernel", "user", "system"]>;
export declare const EpistemicLevel: z.ZodEnum<["tacit", "inferred", "proven", "conclusive"]>;
export declare const Ontology: z.ZodEnum<["monadic", "triadic"]>;
export type Ontology = z.infer<typeof Ontology>;
declare const KnowingPayloadSchema: z.ZodObject<{
    modality: z.ZodOptional<z.ZodString>;
    embedding: z.ZodOptional<z.ZodArray<z.ZodNumber, "many">>;
    trace: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    summary: z.ZodOptional<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    trace?: Record<string, any> | undefined;
    modality?: string | undefined;
    embedding?: number[] | undefined;
    summary?: string | undefined;
}, {
    trace?: Record<string, any> | undefined;
    modality?: string | undefined;
    embedding?: number[] | undefined;
    summary?: string | undefined;
}>;
declare const ConceivingPayloadSchema: z.ZodObject<{
    proposition: z.ZodString;
    proof: z.ZodOptional<z.ZodObject<{
        steps: z.ZodArray<z.ZodString, "many">;
        evidenceIds: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        rationale: z.ZodOptional<z.ZodString>;
    }, "strip", z.ZodTypeAny, {
        steps: string[];
        evidenceIds?: string[] | undefined;
        rationale?: string | undefined;
    }, {
        steps: string[];
        evidenceIds?: string[] | undefined;
        rationale?: string | undefined;
    }>>;
    narrative: z.ZodOptional<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    proposition: string;
    proof?: {
        steps: string[];
        evidenceIds?: string[] | undefined;
        rationale?: string | undefined;
    } | undefined;
    narrative?: string | undefined;
}, {
    proposition: string;
    proof?: {
        steps: string[];
        evidenceIds?: string[] | undefined;
        rationale?: string | undefined;
    } | undefined;
    narrative?: string | undefined;
}>;
export declare const PrintEnvelopeSchema: z.ZodDiscriminatedUnion<"kind", [z.ZodObject<{
    id: z.ZodString;
    role: z.ZodEnum<["kernel", "user", "system"]>;
    timestamp: z.ZodDefault<z.ZodEffects<z.ZodDate, Date, unknown>>;
    provenance: z.ZodOptional<z.ZodObject<{
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
    }>>;
    derivedFrom: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    schemaVersion: z.ZodDefault<z.ZodString>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    epistemicLevel: z.ZodOptional<z.ZodEnum<["tacit", "inferred", "proven", "conclusive"]>>;
    confidence: z.ZodOptional<z.ZodNumber>;
    ontology: z.ZodOptional<z.ZodEnum<["monadic", "triadic"]>>;
    phases: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
} & {
    kind: z.ZodLiteral<"knowing">;
    payload: z.ZodObject<{
        modality: z.ZodOptional<z.ZodString>;
        embedding: z.ZodOptional<z.ZodArray<z.ZodNumber, "many">>;
        trace: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        summary: z.ZodOptional<z.ZodString>;
    }, "strip", z.ZodTypeAny, {
        trace?: Record<string, any> | undefined;
        modality?: string | undefined;
        embedding?: number[] | undefined;
        summary?: string | undefined;
    }, {
        trace?: Record<string, any> | undefined;
        modality?: string | undefined;
        embedding?: number[] | undefined;
        summary?: string | undefined;
    }>;
}, "strip", z.ZodTypeAny, {
    id: string;
    timestamp: Date;
    kind: "knowing";
    role: "kernel" | "user" | "system";
    schemaVersion: string;
    payload: {
        trace?: Record<string, any> | undefined;
        modality?: string | undefined;
        embedding?: number[] | undefined;
        summary?: string | undefined;
    };
    derivedFrom?: string[] | undefined;
    provenance?: {
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
    } | undefined;
    confidence?: number | undefined;
    metadata?: Record<string, any> | undefined;
    epistemicLevel?: "inferred" | "tacit" | "proven" | "conclusive" | undefined;
    ontology?: "monadic" | "triadic" | undefined;
    phases?: string[] | undefined;
}, {
    id: string;
    kind: "knowing";
    role: "kernel" | "user" | "system";
    payload: {
        trace?: Record<string, any> | undefined;
        modality?: string | undefined;
        embedding?: number[] | undefined;
        summary?: string | undefined;
    };
    timestamp?: unknown;
    derivedFrom?: string[] | undefined;
    provenance?: {
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
    } | undefined;
    confidence?: number | undefined;
    metadata?: Record<string, any> | undefined;
    schemaVersion?: string | undefined;
    epistemicLevel?: "inferred" | "tacit" | "proven" | "conclusive" | undefined;
    ontology?: "monadic" | "triadic" | undefined;
    phases?: string[] | undefined;
}>, z.ZodObject<{
    id: z.ZodString;
    role: z.ZodEnum<["kernel", "user", "system"]>;
    timestamp: z.ZodDefault<z.ZodEffects<z.ZodDate, Date, unknown>>;
    provenance: z.ZodOptional<z.ZodObject<{
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
    }>>;
    derivedFrom: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    schemaVersion: z.ZodDefault<z.ZodString>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    epistemicLevel: z.ZodOptional<z.ZodEnum<["tacit", "inferred", "proven", "conclusive"]>>;
    confidence: z.ZodOptional<z.ZodNumber>;
    ontology: z.ZodOptional<z.ZodEnum<["monadic", "triadic"]>>;
    phases: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
} & {
    kind: z.ZodLiteral<"conceiving">;
    payload: z.ZodObject<{
        proposition: z.ZodString;
        proof: z.ZodOptional<z.ZodObject<{
            steps: z.ZodArray<z.ZodString, "many">;
            evidenceIds: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
            rationale: z.ZodOptional<z.ZodString>;
        }, "strip", z.ZodTypeAny, {
            steps: string[];
            evidenceIds?: string[] | undefined;
            rationale?: string | undefined;
        }, {
            steps: string[];
            evidenceIds?: string[] | undefined;
            rationale?: string | undefined;
        }>>;
        narrative: z.ZodOptional<z.ZodString>;
    }, "strip", z.ZodTypeAny, {
        proposition: string;
        proof?: {
            steps: string[];
            evidenceIds?: string[] | undefined;
            rationale?: string | undefined;
        } | undefined;
        narrative?: string | undefined;
    }, {
        proposition: string;
        proof?: {
            steps: string[];
            evidenceIds?: string[] | undefined;
            rationale?: string | undefined;
        } | undefined;
        narrative?: string | undefined;
    }>;
}, "strip", z.ZodTypeAny, {
    id: string;
    timestamp: Date;
    kind: "conceiving";
    role: "kernel" | "user" | "system";
    schemaVersion: string;
    payload: {
        proposition: string;
        proof?: {
            steps: string[];
            evidenceIds?: string[] | undefined;
            rationale?: string | undefined;
        } | undefined;
        narrative?: string | undefined;
    };
    derivedFrom?: string[] | undefined;
    provenance?: {
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
    } | undefined;
    confidence?: number | undefined;
    metadata?: Record<string, any> | undefined;
    epistemicLevel?: "inferred" | "tacit" | "proven" | "conclusive" | undefined;
    ontology?: "monadic" | "triadic" | undefined;
    phases?: string[] | undefined;
}, {
    id: string;
    kind: "conceiving";
    role: "kernel" | "user" | "system";
    payload: {
        proposition: string;
        proof?: {
            steps: string[];
            evidenceIds?: string[] | undefined;
            rationale?: string | undefined;
        } | undefined;
        narrative?: string | undefined;
    };
    timestamp?: unknown;
    derivedFrom?: string[] | undefined;
    provenance?: {
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
    } | undefined;
    confidence?: number | undefined;
    metadata?: Record<string, any> | undefined;
    schemaVersion?: string | undefined;
    epistemicLevel?: "inferred" | "tacit" | "proven" | "conclusive" | undefined;
    ontology?: "monadic" | "triadic" | undefined;
    phases?: string[] | undefined;
}>, z.ZodObject<{
    id: z.ZodString;
    role: z.ZodEnum<["kernel", "user", "system"]>;
    timestamp: z.ZodDefault<z.ZodEffects<z.ZodDate, Date, unknown>>;
    provenance: z.ZodOptional<z.ZodObject<{
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
    }>>;
    derivedFrom: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    schemaVersion: z.ZodDefault<z.ZodString>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    epistemicLevel: z.ZodOptional<z.ZodEnum<["tacit", "inferred", "proven", "conclusive"]>>;
    confidence: z.ZodOptional<z.ZodNumber>;
    ontology: z.ZodOptional<z.ZodEnum<["monadic", "triadic"]>>;
    phases: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
} & {
    kind: z.ZodEnum<["taw", "facttrace", "ml", "graph", "proof"]>;
    payload: z.ZodAny;
}, "strip", z.ZodTypeAny, {
    id: string;
    timestamp: Date;
    kind: "graph" | "taw" | "facttrace" | "ml" | "proof";
    role: "kernel" | "user" | "system";
    schemaVersion: string;
    derivedFrom?: string[] | undefined;
    provenance?: {
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
    } | undefined;
    confidence?: number | undefined;
    metadata?: Record<string, any> | undefined;
    epistemicLevel?: "inferred" | "tacit" | "proven" | "conclusive" | undefined;
    ontology?: "monadic" | "triadic" | undefined;
    phases?: string[] | undefined;
    payload?: any;
}, {
    id: string;
    kind: "graph" | "taw" | "facttrace" | "ml" | "proof";
    role: "kernel" | "user" | "system";
    timestamp?: unknown;
    derivedFrom?: string[] | undefined;
    provenance?: {
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
    } | undefined;
    confidence?: number | undefined;
    metadata?: Record<string, any> | undefined;
    schemaVersion?: string | undefined;
    epistemicLevel?: "inferred" | "tacit" | "proven" | "conclusive" | undefined;
    ontology?: "monadic" | "triadic" | undefined;
    phases?: string[] | undefined;
    payload?: any;
}>]>;
export type PrintEnvelope = z.infer<typeof PrintEnvelopeSchema>;
export { KnowingPayloadSchema, ConceivingPayloadSchema };
//# sourceMappingURL=prints.d.ts.map