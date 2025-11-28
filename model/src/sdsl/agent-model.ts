/**
 * Agent Model - Agent's interpretation of FactStore
 *
 * While FormModel structures facts for human display,
 * AgentModel structures facts for agent reasoning.
 *
 * The Agent is the primary consumer of FactStore,
 * feeding the KnowledgeStore downstream.
 *
 * Pipeline: GraphStore → FactStore → KnowledgeStore
 *                            ↑
 *                      AgentModel (here)
 */

import { z } from "zod";
import type { FormShape, FormField } from "./types";

// =============================================================================
// AGENT-SPECIFIC OVERLAYS ON FACTS
// =============================================================================

/**
 * Relevance - How important is this fact to the current goal?
 */
export const RelevanceLevel = z.enum([
  "critical",    // Must have for goal
  "important",   // Strongly supports goal
  "relevant",    // Somewhat useful
  "peripheral",  // Tangentially related
  "irrelevant",  // Not needed
]);
export type RelevanceLevel = z.infer<typeof RelevanceLevel>;

export const FactRelevance = z.object({
  fieldId: z.string(),
  level: RelevanceLevel,
  reason: z.string().optional(),
  goalId: z.string().optional(),
});
export type FactRelevance = z.infer<typeof FactRelevance>;

/**
 * Provenance - Where did this fact come from?
 */
export const ProvenanceType = z.enum([
  "asserted",    // Directly stated
  "inferred",    // Derived from other facts
  "observed",    // From external source
  "hypothesized", // Tentative/uncertain
  "inherited",   // From parent context
]);
export type ProvenanceType = z.infer<typeof ProvenanceType>;

export const FactProvenance = z.object({
  fieldId: z.string(),
  type: ProvenanceType,
  source: z.string().optional(),
  timestamp: z.string().optional(),
  derivedFrom: z.array(z.string()).optional(),
});
export type FactProvenance = z.infer<typeof FactProvenance>;

/**
 * Confidence - How certain are we about this fact?
 */
export const ConfidenceLevel = z.enum([
  "certain",     // 100% confident
  "high",        // Very likely true
  "medium",      // Probably true
  "low",         // Uncertain
  "speculative", // Guess/hypothesis
]);
export type ConfidenceLevel = z.infer<typeof ConfidenceLevel>;

export const FactConfidence = z.object({
  fieldId: z.string(),
  level: ConfidenceLevel,
  score: z.number().min(0).max(1).optional(),
  basis: z.string().optional(),
});
export type FactConfidence = z.infer<typeof FactConfidence>;

/**
 * Dependency - How do facts relate to each other?
 */
export const DependencyType = z.enum([
  "requires",    // A requires B
  "implies",     // A implies B
  "contradicts", // A contradicts B
  "supports",    // A supports B
  "derived",     // A derived from B
]);
export type DependencyType = z.infer<typeof DependencyType>;

export const FactDependency = z.object({
  fromField: z.string(),
  toField: z.string(),
  type: DependencyType,
  strength: z.number().min(0).max(1).optional(),
});
export type FactDependency = z.infer<typeof FactDependency>;

// =============================================================================
// AGENT CONTEXT - The Goal-Oriented Frame
// =============================================================================

/**
 * Goal - What the agent is trying to achieve
 */
export const GoalType = z.enum([
  "query",       // Find/retrieve information
  "transform",   // Change data structure
  "validate",    // Check correctness
  "infer",       // Derive new facts
  "summarize",   // Condense information
  "compare",     // Analyze differences
  "generate",    // Create new content
]);
export type GoalType = z.infer<typeof GoalType>;

export const AgentGoal = z.object({
  id: z.string(),
  type: GoalType,
  description: z.string(),
  constraints: z.array(z.string()).optional(),
  successCriteria: z.array(z.string()).optional(),
  priority: z.number().default(1),
});
export type AgentGoal = z.infer<typeof AgentGoal>;

/**
 * Focus - Which subset of facts matter for current goal
 */
export const AgentFocus = z.object({
  includedFields: z.array(z.string()).optional(),
  excludedFields: z.array(z.string()).optional(),
  depthLimit: z.number().optional(),
  maxFacts: z.number().optional(),
});
export type AgentFocus = z.infer<typeof AgentFocus>;

// =============================================================================
// AGENT MODEL STATE
// =============================================================================

/**
 * AgentModelState - The agent's enriched view of facts
 *
 * Parallel to FormModelState, but with agent-specific overlays
 */
export interface AgentModelState<T extends FormShape = FormShape> {
  // The underlying form shape (from FactStore)
  shape: T;
  values: Record<string, unknown>;

  // Agent-specific overlays
  relevance: Map<string, FactRelevance>;
  provenance: Map<string, FactProvenance>;
  confidence: Map<string, FactConfidence>;
  dependencies: FactDependency[];

  // Goal context
  goal?: AgentGoal;
  focus?: AgentFocus;
}

// =============================================================================
// AGENT MODEL CLASS
// =============================================================================

/**
 * AgentModel - Agent's interface to FactStore
 *
 * While FormModel asks "What is the structure?"
 * AgentModel asks "What is relevant to my goal?"
 */
export class AgentModel<T extends FormShape = FormShape> {
  private state: AgentModelState<T>;

  constructor(shape: T, values: Record<string, unknown> = {}) {
    this.state = {
      shape,
      values: { ...values }, // Copy to prevent mutation of original
      relevance: new Map(),
      provenance: new Map(),
      confidence: new Map(),
      dependencies: [],
    };
  }

  // ---------------------------------------------------------------------------
  // Core Accessors (parallel to FormModel)
  // ---------------------------------------------------------------------------

  getShape(): T {
    return this.state.shape;
  }

  getValues(): Record<string, unknown> {
    return this.state.values;
  }

  getValue(fieldId: string): unknown {
    return this.state.values[fieldId];
  }

  getField(fieldId: string): FormField | undefined {
    return this.state.shape.fields.find((f) => f.id === fieldId);
  }

  // ---------------------------------------------------------------------------
  // Goal Management
  // ---------------------------------------------------------------------------

  setGoal(goal: AgentGoal): void {
    this.state.goal = goal;
  }

  getGoal(): AgentGoal | undefined {
    return this.state.goal;
  }

  setFocus(focus: AgentFocus): void {
    this.state.focus = focus;
  }

  getFocus(): AgentFocus | undefined {
    return this.state.focus;
  }

  // ---------------------------------------------------------------------------
  // Relevance Management
  // ---------------------------------------------------------------------------

  setRelevance(fieldId: string, relevance: Omit<FactRelevance, "fieldId">): void {
    this.state.relevance.set(fieldId, { fieldId, ...relevance });
  }

  getRelevance(fieldId: string): FactRelevance | undefined {
    return this.state.relevance.get(fieldId);
  }

  getRelevantFields(minLevel: RelevanceLevel = "relevant"): FormField[] {
    const levelOrder: RelevanceLevel[] = [
      "critical",
      "important",
      "relevant",
      "peripheral",
      "irrelevant",
    ];
    const minIndex = levelOrder.indexOf(minLevel);

    return this.state.shape.fields.filter((field) => {
      const relevance = this.state.relevance.get(field.id);
      if (!relevance) return false;
      const fieldIndex = levelOrder.indexOf(relevance.level);
      return fieldIndex <= minIndex;
    });
  }

  // ---------------------------------------------------------------------------
  // Provenance Management
  // ---------------------------------------------------------------------------

  setProvenance(fieldId: string, provenance: Omit<FactProvenance, "fieldId">): void {
    this.state.provenance.set(fieldId, { fieldId, ...provenance });
  }

  getProvenance(fieldId: string): FactProvenance | undefined {
    return this.state.provenance.get(fieldId);
  }

  getFactsByProvenance(type: ProvenanceType): FormField[] {
    return this.state.shape.fields.filter((field) => {
      const provenance = this.state.provenance.get(field.id);
      return provenance?.type === type;
    });
  }

  // ---------------------------------------------------------------------------
  // Confidence Management
  // ---------------------------------------------------------------------------

  setConfidence(fieldId: string, confidence: Omit<FactConfidence, "fieldId">): void {
    this.state.confidence.set(fieldId, { fieldId, ...confidence });
  }

  getConfidence(fieldId: string): FactConfidence | undefined {
    return this.state.confidence.get(fieldId);
  }

  getCertainFacts(minLevel: ConfidenceLevel = "medium"): FormField[] {
    const levelOrder: ConfidenceLevel[] = [
      "certain",
      "high",
      "medium",
      "low",
      "speculative",
    ];
    const minIndex = levelOrder.indexOf(minLevel);

    return this.state.shape.fields.filter((field) => {
      const confidence = this.state.confidence.get(field.id);
      if (!confidence) return true; // Assume confident if not specified
      const fieldIndex = levelOrder.indexOf(confidence.level);
      return fieldIndex <= minIndex;
    });
  }

  // ---------------------------------------------------------------------------
  // Dependency Management
  // ---------------------------------------------------------------------------

  addDependency(dependency: FactDependency): void {
    this.state.dependencies.push(dependency);
  }

  getDependencies(fieldId: string): FactDependency[] {
    return this.state.dependencies.filter(
      (d) => d.fromField === fieldId || d.toField === fieldId
    );
  }

  getDependentsOf(fieldId: string): string[] {
    return this.state.dependencies
      .filter((d) => d.toField === fieldId)
      .map((d) => d.fromField);
  }

  getDependenciesOf(fieldId: string): string[] {
    return this.state.dependencies
      .filter((d) => d.fromField === fieldId)
      .map((d) => d.toField);
  }

  // ---------------------------------------------------------------------------
  // Fact Extraction (for Agent consumption)
  // ---------------------------------------------------------------------------

  /**
   * Get facts filtered by current focus
   */
  getFocusedFacts(): Array<{ field: FormField; value: unknown }> {
    const focus = this.state.focus;
    let fields = this.state.shape.fields;

    if (focus?.includedFields) {
      fields = fields.filter((f) => focus.includedFields!.includes(f.id));
    }
    if (focus?.excludedFields) {
      fields = fields.filter((f) => !focus.excludedFields!.includes(f.id));
    }
    if (focus?.maxFacts) {
      fields = fields.slice(0, focus.maxFacts);
    }

    return fields.map((field) => ({
      field,
      value: this.state.values[field.id],
    }));
  }

  /**
   * Get facts as structured records for agent
   */
  toFactRecords(): Array<{
    id: string;
    label?: string;
    type: string;
    value: unknown;
    relevance?: RelevanceLevel;
    confidence?: ConfidenceLevel;
    provenance?: ProvenanceType;
  }> {
    return this.state.shape.fields.map((field) => ({
      id: field.id,
      label: field.label,
      type: field.type,
      value: this.state.values[field.id],
      relevance: this.state.relevance.get(field.id)?.level,
      confidence: this.state.confidence.get(field.id)?.level,
      provenance: this.state.provenance.get(field.id)?.type,
    }));
  }

  // ---------------------------------------------------------------------------
  // State Export
  // ---------------------------------------------------------------------------

  getState(): AgentModelState<T> {
    return this.state;
  }

  toJSON(): object {
    return {
      shape: this.state.shape,
      values: this.state.values,
      goal: this.state.goal,
      focus: this.state.focus,
      relevance: Object.fromEntries(this.state.relevance),
      provenance: Object.fromEntries(this.state.provenance),
      confidence: Object.fromEntries(this.state.confidence),
      dependencies: this.state.dependencies,
    };
  }

  // ---------------------------------------------------------------------------
  // Factory Methods
  // ---------------------------------------------------------------------------

  static fromFormShape<T extends FormShape>(
    shape: T,
    values: Record<string, unknown> = {}
  ): AgentModel<T> {
    return new AgentModel(shape, values);
  }

  /**
   * Create AgentModel with automatic relevance based on goal
   */
  static withGoal<T extends FormShape>(
    shape: T,
    values: Record<string, unknown>,
    goal: AgentGoal
  ): AgentModel<T> {
    const model = new AgentModel(shape, values);
    model.setGoal(goal);
    return model;
  }
}

// =============================================================================
// SIMPLE AGENT MODEL (parallel to SimpleFormModel)
// =============================================================================

/**
 * SimpleAgentModel - Convenient wrapper with fluent API
 */
export class SimpleAgentModel<T extends FormShape = FormShape> extends AgentModel<T> {
  constructor(shape: T, values: Record<string, unknown> = {}) {
    super(shape, values);
  }

  // Fluent API for building agent context
  withGoal(goal: AgentGoal): this {
    this.setGoal(goal);
    return this;
  }

  withFocus(focus: AgentFocus): this {
    this.setFocus(focus);
    return this;
  }

  markRelevant(fieldId: string, level: RelevanceLevel, reason?: string): this {
    this.setRelevance(fieldId, { level, reason });
    return this;
  }

  markProvenance(fieldId: string, type: ProvenanceType, source?: string): this {
    this.setProvenance(fieldId, { type, source });
    return this;
  }

  markConfidence(fieldId: string, level: ConfidenceLevel, basis?: string): this {
    this.setConfidence(fieldId, { level, basis });
    return this;
  }

  addLink(from: string, to: string, type: DependencyType): this {
    this.addDependency({ fromField: from, toField: to, type });
    return this;
  }

  // Static factory
  static create<T extends FormShape>(
    shape: T,
    values: Record<string, unknown> = {}
  ): SimpleAgentModel<T> {
    return new SimpleAgentModel(shape, values);
  }
}
