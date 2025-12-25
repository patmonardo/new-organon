/**
 * Dialectic artifact types (discursive / DECODE layer)
 *
 * These are *not* kernel boundary schemas. They live in Logic because they represent
 * the transition from Intuition (kernel outputs) into Concept (discursive structure).
 *
 * This is intentionally lightweight: plain TS structural types used by the dialectic stubs.
 */

export type JudgmentArtifact = {
	kind: 'judgment';
	id?: string;
	moment?: 'existence' | 'reflection' | 'necessity' | 'concept';
	foundationMoment?: 'positive' | 'negative' | 'infinite';
	thesis: string;
	tokens?: { text: string; kind?: string }[];
	grounds?: string[];
	contradictions?: string[];
	meta?: Record<string, unknown>;
} & Record<string, unknown>;

export type SyllogismArtifact = {
	kind: 'syllogism';
	id?: string;
	morphPatterns: string[];
	premises?: { id?: string; thesis: string; grounds?: string[] }[];
	conclusion: string;
	tokens?: { text: string; kind?: string }[];
	meta?: Record<string, unknown>;
} & Record<string, unknown>;

export type ObjectivityRef = { kind: 'entity' | 'property' | 'aspect'; id: string } & Record<string, unknown>;

export type ObjectivityEntity = {
	kind: 'entity';
	id?: string;
	label: string;
	meta?: Record<string, unknown>;
} & Record<string, unknown>;

export type ObjectivityProperty = {
	kind: 'property';
	id?: string;
	entityId?: string;
	name: string;
	value?: unknown;
	grounds?: string[];
	meta?: Record<string, unknown>;
} & Record<string, unknown>;

export type ObjectivityAspect = {
	kind: 'aspect';
	id?: string;
	entityId?: string;
	name: string;
	description?: string;
	grounds?: string[];
	meta?: Record<string, unknown>;
} & Record<string, unknown>;

export type EssentialRelation = {
	kind: 'essentialRelation';
	id?: string;
	from: ObjectivityRef;
	to: ObjectivityRef;
	relation: string;
	grounds?: string[];
	meta?: Record<string, unknown>;
} & Record<string, unknown>;

export type ObjectivityArtifact = {
	kind: 'objectivity';
	id?: string;
	entities: ObjectivityEntity[];
	properties: ObjectivityProperty[];
	aspects: ObjectivityAspect[];
	essentialRelations: EssentialRelation[];
	meta?: Record<string, unknown>;
} & Record<string, unknown>;

export type ConceptArtifact = {
	kind: 'concept';
	id?: string;
	morphPatterns: string[];
	judgment?: JudgmentArtifact;
	syllogism?: SyllogismArtifact;
	objectivity?: ObjectivityArtifact;
	meta?: Record<string, unknown>;
} & Record<string, unknown>;


