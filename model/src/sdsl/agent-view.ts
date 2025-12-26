/**
 * Agent View - Present FactStore for Agent Consumption
 *
 * While FormView renders facts for human eyes (DisplayDocument),
 * AgentView renders facts for agent consumption (ContextDocument).
 *
 * The Agent doesn't need pretty UI - it needs:
 * - Structured context for reasoning
 * - Prompt fragments for LLM consumption
 * - Function arguments for tool calls
 * - Graph representations for knowledge operations
 */

import { z } from "zod";
import type { FormShape, FormField, FormMode } from "./types";
import { AgentModel, SimpleAgentModel, type AgentGoal, type RelevanceLevel, type ConfidenceLevel, type ProvenanceType } from "./agent-model";

import {
  ContextDocumentSchema,
  ContextConstraintSchema,
  SchemaDescriptionSchema,
  StructuredFactSchema,
  type ContextConstraint,
  type ContextDocument,
  type SchemaDescription,
  type StructuredFact,
} from '@organon/task';

// =============================================================================
// CONTEXT DOCUMENT - The Agent's Display Language
// =============================================================================

/**
 * StructuredFact - A single fact in agent-consumable format
 *
 * This is the agent equivalent of DisplayElement
 */
export {
  ContextDocumentSchema,
  ContextConstraintSchema,
  SchemaDescriptionSchema,
  StructuredFactSchema,
  type ContextConstraint,
  type ContextDocument,
  type SchemaDescription,
  type StructuredFact,
};

// =============================================================================
// AGENT VIEW STATE
// =============================================================================

export interface AgentViewState<T extends FormShape = FormShape> {
  model: AgentModel<T>;
  mode: FormMode;
  format: AgentViewFormat;
}

export type AgentViewFormat = "context" | "prompt" | "function" | "graph";

// =============================================================================
// AGENT VIEW CLASS
// =============================================================================

/**
 * AgentView - Renders FactStore for agent consumption
 *
 * Parallel to FormView, but outputs ContextDocument instead of DisplayDocument
 */
export class AgentView<T extends FormShape = FormShape> {
  protected state: AgentViewState<T>;

  constructor(model: AgentModel<T>, mode: FormMode = "view") {
    this.state = {
      model,
      mode,
      format: "context",
    };
  }

  // ---------------------------------------------------------------------------
  // Core Accessors
  // ---------------------------------------------------------------------------

  getModel(): AgentModel<T> {
    return this.state.model;
  }

  getMode(): FormMode {
    return this.state.mode;
  }

  setFormat(format: AgentViewFormat): void {
    this.state.format = format;
  }

  // ---------------------------------------------------------------------------
  // Context Document Rendering
  // ---------------------------------------------------------------------------

  /**
   * Render to ContextDocument - the agent's display format
   */
  toContextDocument(): ContextDocument {
    const shape = this.state.model.getShape();
    const values = this.state.model.getValues();
    const goal = this.state.model.getGoal();

    return {
      id: `ctx-${shape.id}-${Date.now()}`,
      timestamp: new Date().toISOString(),
      facts: this.renderFacts(),
      schema: this.renderSchema(),
      constraints: this.renderConstraints(),
      goal: goal ? {
        id: goal.id,
        type: goal.type,
        description: goal.description,
      } : undefined,
      dependencies: this.renderDependencies(),
    };
  }

  protected renderFacts(): StructuredFact[] {
    const facts = this.state.model.toFactRecords();
    return facts.map((fact) => ({
      id: fact.id,
      label: fact.label,
      type: fact.type,
      value: fact.value,
      relevance: fact.relevance,
      confidence: fact.confidence,
      provenance: fact.provenance,
    }));
  }

  protected renderSchema(): SchemaDescription {
    const shape = this.state.model.getShape();
    return {
      id: shape.id,
      name: shape.name,
      description: shape.description,
      fieldCount: shape.fields.length,
      requiredFields: shape.fields.filter((f) => f.required).map((f) => f.id),
      optionalFields: shape.fields.filter((f) => !f.required).map((f) => f.id),
    };
  }

  protected renderConstraints(): ContextConstraint[] {
    const shape = this.state.model.getShape();
    const constraints: ContextConstraint[] = [];

    // Add required field constraints
    for (const field of shape.fields) {
      if (field.required) {
        constraints.push({
          type: "must",
          description: `Field '${field.label || field.id}' is required`,
          fieldId: field.id,
        });
      }
    }

    // Add goal constraints if present
    const goal = this.state.model.getGoal();
    if (goal?.constraints) {
      for (const constraint of goal.constraints) {
        constraints.push({
          type: "should",
          description: constraint,
        });
      }
    }

    return constraints;
  }

  protected renderDependencies(): Array<{ from: string; to: string; type: string }> {
    const shape = this.state.model.getShape();
    const deps: Array<{ from: string; to: string; type: string }> = [];

    for (const field of shape.fields) {
      const fieldDeps = this.state.model.getDependencies(field.id);
      for (const dep of fieldDeps) {
        deps.push({
          from: dep.fromField,
          to: dep.toField,
          type: dep.type,
        });
      }
    }

    // Deduplicate
    const seen = new Set<string>();
    return deps.filter((d) => {
      const key = `${d.from}-${d.to}-${d.type}`;
      if (seen.has(key)) return false;
      seen.add(key);
      return true;
    });
  }

  // ---------------------------------------------------------------------------
  // Prompt Rendering - For LLM consumption
  // ---------------------------------------------------------------------------

  /**
   * Render facts as a prompt fragment for LLM
   */
  toPrompt(): string {
    const ctx = this.toContextDocument();
    const lines: string[] = [];

    // Header
    if (ctx.goal) {
      lines.push(`## Goal: ${ctx.goal.description}`);
      lines.push("");
    }

    // Schema summary
    lines.push(`## Data Shape: ${ctx.schema.name || ctx.schema.id}`);
    if (ctx.schema.description) {
      lines.push(ctx.schema.description);
    }
    lines.push("");

    // Facts
    lines.push("## Facts:");
    for (const fact of ctx.facts) {
      const label = fact.label || fact.id;
      const value = this.formatValue(fact.value);
      const meta: string[] = [];

      if (fact.relevance) meta.push(`relevance: ${fact.relevance}`);
      if (fact.confidence) meta.push(`confidence: ${fact.confidence}`);

      if (meta.length > 0) {
        lines.push(`- **${label}**: ${value} (${meta.join(", ")})`);
      } else {
        lines.push(`- **${label}**: ${value}`);
      }
    }
    lines.push("");

    // Constraints
    if (ctx.constraints && ctx.constraints.length > 0) {
      lines.push("## Constraints:");
      for (const constraint of ctx.constraints) {
        lines.push(`- [${constraint.type}] ${constraint.description}`);
      }
    }

    return lines.join("\n");
  }

  protected formatValue(value: unknown): string {
    if (value === null || value === undefined) return "(empty)";
    if (typeof value === "string") return value;
    if (typeof value === "number" || typeof value === "boolean") return String(value);
    if (Array.isArray(value)) return `[${value.length} items]`;
    if (typeof value === "object") return JSON.stringify(value);
    return String(value);
  }

  // ---------------------------------------------------------------------------
  // Function Arguments - For tool/function calling
  // ---------------------------------------------------------------------------

  /**
   * Render facts as function arguments for tool calls
   */
  toFunctionArgs(): Record<string, unknown> {
    const shape = this.state.model.getShape();
    const values = this.state.model.getValues();

    const args: Record<string, unknown> = {};
    for (const field of shape.fields) {
      const value = values[field.id];
      if (value !== undefined && value !== null) {
        args[field.id] = value;
      }
    }

    return args;
  }

  /**
   * Render as typed function call with metadata
   */
  toFunctionCall(functionName: string): {
    name: string;
    arguments: Record<string, unknown>;
    schema: Record<string, { type: string; required: boolean }>;
  } {
    const shape = this.state.model.getShape();

    return {
      name: functionName,
      arguments: this.toFunctionArgs(),
      schema: Object.fromEntries(
        shape.fields.map((f) => [f.id, { type: f.type, required: f.required }])
      ),
    };
  }

  // ---------------------------------------------------------------------------
  // Graph Rendering - For knowledge graph operations
  // ---------------------------------------------------------------------------

  /**
   * Render as simple graph structure
   */
  toGraph(): {
    nodes: Array<{ id: string; type: string; properties: Record<string, unknown> }>;
    edges: Array<{ from: string; to: string; type: string }>;
  } {
    const shape = this.state.model.getShape();
    const values = this.state.model.getValues();

    // Shape node
    const nodes: Array<{ id: string; type: string; properties: Record<string, unknown> }> = [
      {
        id: shape.id,
        type: "FormShape",
        properties: {
          name: shape.name,
          description: shape.description,
        },
      },
    ];

    // Field nodes
    for (const field of shape.fields) {
      nodes.push({
        id: field.id,
        type: "FormField",
        properties: {
          label: field.label,
          type: field.type,
          value: values[field.id],
          required: field.required,
        },
      });
    }

    // Edges from shape to fields
    const edges: Array<{ from: string; to: string; type: string }> = shape.fields.map((f) => ({
      from: shape.id,
      to: f.id,
      type: "hasField",
    }));

    // Add dependency edges
    const deps = this.renderDependencies();
    edges.push(...deps);

    return { nodes, edges };
  }

  // ---------------------------------------------------------------------------
  // Static Factories
  // ---------------------------------------------------------------------------

  static fromModel<T extends FormShape>(
    model: AgentModel<T>,
    mode: FormMode = "view"
  ): AgentView<T> {
    return new AgentView(model, mode);
  }

  static fromShape<T extends FormShape>(
    shape: T,
    values: Record<string, unknown> = {},
    mode: FormMode = "view"
  ): AgentView<T> {
    const model = new AgentModel(shape, values);
    return new AgentView(model, mode);
  }
}

// =============================================================================
// SIMPLE AGENT VIEW - Convenient wrapper
// =============================================================================

/**
 * SimpleAgentView - Fluent API for agent context rendering
 */
export class SimpleAgentView<T extends FormShape = FormShape> extends AgentView<T> {
  constructor(model: AgentModel<T> | SimpleAgentModel<T>, mode: FormMode = "view") {
    super(model, mode);
  }

  // Fluent format selection
  asContext(): ContextDocument {
    return this.toContextDocument();
  }

  asPrompt(): string {
    return this.toPrompt();
  }

  asFunctionArgs(): Record<string, unknown> {
    return this.toFunctionArgs();
  }

  asGraph(): ReturnType<AgentView<T>["toGraph"]> {
    return this.toGraph();
  }

  // Filtered views
  relevantOnly(minLevel: RelevanceLevel = "relevant"): StructuredFact[] {
    const model = this.getModel();
    const relevantFields = model.getRelevantFields(minLevel);
    const values = model.getValues();

    return relevantFields.map((field) => ({
      id: field.id,
      label: field.label,
      type: field.type,
      value: values[field.id],
      relevance: model.getRelevance(field.id)?.level,
      confidence: model.getConfidence(field.id)?.level,
      provenance: model.getProvenance(field.id)?.type,
    }));
  }

  certainOnly(minLevel: ConfidenceLevel = "medium"): StructuredFact[] {
    const model = this.getModel();
    const certainFields = model.getCertainFacts(minLevel);
    const values = model.getValues();

    return certainFields.map((field) => ({
      id: field.id,
      label: field.label,
      type: field.type,
      value: values[field.id],
      relevance: model.getRelevance(field.id)?.level,
      confidence: model.getConfidence(field.id)?.level,
      provenance: model.getProvenance(field.id)?.type,
    }));
  }

  // Static factory
  static create<T extends FormShape>(
    shape: T,
    values: Record<string, unknown> = {},
    mode?: FormMode
  ): SimpleAgentView<T> {
    const model = SimpleAgentModel.create(shape, values);
    return new SimpleAgentView(model, mode);
  }
}
