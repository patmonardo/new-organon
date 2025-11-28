/**
 * Agent Controller - Agent's Operations on FactStore
 *
 * While FormController handles CRUD for humans,
 * AgentController handles Query/Infer/Assert/Retract for agents.
 *
 * This is the agent's interface for:
 * - Querying relevant facts
 * - Inferring new facts from existing ones
 * - Asserting new facts into the store
 * - Retracting facts from the store
 * - Hypothesizing tentative facts
 */

import { z } from "zod";
import type { FormShape, FormField, FormMode, OperationResult } from "./types";
import { AgentModel, SimpleAgentModel, type AgentGoal, type FactProvenance, type FactConfidence, type ProvenanceType, type ConfidenceLevel } from "./agent-model";
import { AgentView, SimpleAgentView, type ContextDocument } from "./agent-view";

// =============================================================================
// QUERY TYPES - How agents ask questions
// =============================================================================

/**
 * FactPredicate - A condition for filtering facts
 */
export const FactPredicateSchema = z.object({
  fieldId: z.string().optional(),        // Match specific field
  type: z.string().optional(),           // Match field type
  hasValue: z.boolean().optional(),      // Has non-null value
  relevance: z.enum(["critical", "important", "relevant", "peripheral", "irrelevant"]).optional(),
  confidence: z.enum(["certain", "high", "medium", "low", "speculative"]).optional(),
  provenance: z.enum(["asserted", "inferred", "observed", "hypothesized", "inherited"]).optional(),
  // custom function is defined as interface, not zod schema
});

export interface FactPredicate {
  fieldId?: string;
  type?: string;
  hasValue?: boolean;
  relevance?: "critical" | "important" | "relevant" | "peripheral" | "irrelevant";
  confidence?: "certain" | "high" | "medium" | "low" | "speculative";
  provenance?: "asserted" | "inferred" | "observed" | "hypothesized" | "inherited";
  custom?: (fact: { field: FormField; value: unknown }) => boolean;
}

/**
 * QueryResult - Result of a fact query
 */
export interface QueryResult {
  facts: Array<{
    field: FormField;
    value: unknown;
    relevance?: string;
    confidence?: string;
    provenance?: string;
  }>;
  count: number;
  query: FactPredicate;
}

// =============================================================================
// INFERENCE TYPES - How agents derive new facts
// =============================================================================

/**
 * InferenceRule - A rule for deriving new facts
 */
export interface InferenceRule {
  id: string;
  name: string;
  description?: string;

  // Input: which facts to consider
  premises: FactPredicate[];

  // Output: how to derive new fact
  derive: (facts: Array<{ field: FormField; value: unknown }>) => {
    fieldId: string;
    value: unknown;
  } | null;

  // Metadata for the derived fact
  confidence: ConfidenceLevel;
}

/**
 * InferenceResult - Result of applying an inference rule
 */
export interface InferenceResult {
  success: boolean;
  rule: InferenceRule;
  derivedFact?: {
    fieldId: string;
    value: unknown;
    confidence: ConfidenceLevel;
    derivedFrom: string[];
  };
  reason?: string;
}

// =============================================================================
// ASSERTION TYPES - How agents add facts
// =============================================================================

/**
 * FactAssertion - A new fact to assert
 */
export interface FactAssertion {
  fieldId: string;
  value: unknown;
  provenance: ProvenanceType;
  confidence: ConfidenceLevel;
  source?: string;
  derivedFrom?: string[];
}

/**
 * AssertionResult - Result of asserting a fact
 */
export interface AssertionResult {
  success: boolean;
  fieldId: string;
  previousValue?: unknown;
  newValue: unknown;
  reason?: string;
}

// =============================================================================
// HYPOTHESIS TYPES - How agents make tentative claims
// =============================================================================

/**
 * Hypothesis - A tentative fact that hasn't been committed
 */
export interface Hypothesis {
  id: string;
  fieldId: string;
  value: unknown;
  confidence: ConfidenceLevel;
  basis: string;
  status: "pending" | "confirmed" | "rejected";
  createdAt: string;
}

// =============================================================================
// AGENT CONTROLLER STATE
// =============================================================================

export interface AgentControllerState<T extends FormShape = FormShape> {
  model: AgentModel<T>;
  mode: FormMode;
  hypotheses: Map<string, Hypothesis>;
  inferenceHistory: InferenceResult[];
}

// =============================================================================
// AGENT CONTROLLER CLASS
// =============================================================================

/**
 * AgentController - Agent's operations on FactStore
 *
 * Parallel to FormController, but with agent-specific operations
 */
export class AgentController<T extends FormShape = FormShape> {
  protected state: AgentControllerState<T>;

  constructor(model: AgentModel<T>, mode: FormMode = "view") {
    this.state = {
      model,
      mode,
      hypotheses: new Map(),
      inferenceHistory: [],
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

  getView(): AgentView<T> {
    return new AgentView(this.state.model, this.state.mode);
  }

  // ---------------------------------------------------------------------------
  // QUERY - Retrieve relevant facts
  // ---------------------------------------------------------------------------

  /**
   * Query facts matching a predicate
   */
  query(predicate: FactPredicate): QueryResult {
    const shape = this.state.model.getShape();
    const values = this.state.model.getValues();

    const matchingFacts = shape.fields
      .filter((field) => this.matchesPredicate(field, predicate))
      .map((field) => ({
        field,
        value: values[field.id],
        relevance: this.state.model.getRelevance(field.id)?.level,
        confidence: this.state.model.getConfidence(field.id)?.level,
        provenance: this.state.model.getProvenance(field.id)?.type,
      }));

    return {
      facts: matchingFacts,
      count: matchingFacts.length,
      query: predicate,
    };
  }

  protected matchesPredicate(field: FormField, predicate: FactPredicate): boolean {
    const values = this.state.model.getValues();

    // Field ID match
    if (predicate.fieldId && field.id !== predicate.fieldId) return false;

    // Type match
    if (predicate.type && field.type !== predicate.type) return false;

    // Has value check
    if (predicate.hasValue !== undefined) {
      const hasValue = values[field.id] !== undefined && values[field.id] !== null;
      if (predicate.hasValue !== hasValue) return false;
    }

    // Relevance match
    if (predicate.relevance) {
      const relevance = this.state.model.getRelevance(field.id);
      if (relevance?.level !== predicate.relevance) return false;
    }

    // Confidence match
    if (predicate.confidence) {
      const confidence = this.state.model.getConfidence(field.id);
      if (confidence?.level !== predicate.confidence) return false;
    }

    // Provenance match
    if (predicate.provenance) {
      const provenance = this.state.model.getProvenance(field.id);
      if (provenance?.type !== predicate.provenance) return false;
    }

    // Custom predicate
    if (predicate.custom) {
      const fact = { field, value: values[field.id] };
      if (!predicate.custom(fact)) return false;
    }

    return true;
  }

  /**
   * Query all facts with values
   */
  queryAll(): QueryResult {
    return this.query({ hasValue: true });
  }

  /**
   * Query critical facts only
   */
  queryCritical(): QueryResult {
    return this.query({ relevance: "critical" });
  }

  // ---------------------------------------------------------------------------
  // INFER - Derive new facts
  // ---------------------------------------------------------------------------

  /**
   * Apply an inference rule to derive new facts
   */
  infer(rule: InferenceRule): InferenceResult {
    // Gather premise facts
    const premiseFacts: Array<{ field: FormField; value: unknown }> = [];
    const derivedFrom: string[] = [];

    for (const premise of rule.premises) {
      const result = this.query(premise);
      if (result.count === 0) {
        return {
          success: false,
          rule,
          reason: `Premise not satisfied: ${JSON.stringify(premise)}`,
        };
      }
      for (const fact of result.facts) {
        premiseFacts.push({ field: fact.field, value: fact.value });
        derivedFrom.push(fact.field.id);
      }
    }

    // Apply derivation
    const derived = rule.derive(premiseFacts);
    if (!derived) {
      return {
        success: false,
        rule,
        reason: "Derivation returned null",
      };
    }

    // Record the inference
    const result: InferenceResult = {
      success: true,
      rule,
      derivedFact: {
        fieldId: derived.fieldId,
        value: derived.value,
        confidence: rule.confidence,
        derivedFrom,
      },
    };

    this.state.inferenceHistory.push(result);
    return result;
  }

  /**
   * Apply inference and automatically assert the result
   */
  inferAndAssert(rule: InferenceRule): AssertionResult | InferenceResult {
    const inferResult = this.infer(rule);

    if (!inferResult.success || !inferResult.derivedFact) {
      return inferResult;
    }

    return this.assert({
      fieldId: inferResult.derivedFact.fieldId,
      value: inferResult.derivedFact.value,
      provenance: "inferred",
      confidence: inferResult.derivedFact.confidence,
      derivedFrom: inferResult.derivedFact.derivedFrom,
    });
  }

  // ---------------------------------------------------------------------------
  // ASSERT - Add/update facts
  // ---------------------------------------------------------------------------

  /**
   * Assert a new fact into the model
   */
  assert(assertion: FactAssertion): AssertionResult {
    const shape = this.state.model.getShape();
    const values = this.state.model.getValues();

    // Check if field exists
    const field = shape.fields.find((f) => f.id === assertion.fieldId);
    if (!field) {
      return {
        success: false,
        fieldId: assertion.fieldId,
        newValue: assertion.value,
        reason: `Field '${assertion.fieldId}' not found in shape`,
      };
    }

    // Store previous value
    const previousValue = values[assertion.fieldId];

    // Update value
    values[assertion.fieldId] = assertion.value;

    // Set provenance
    this.state.model.setProvenance(assertion.fieldId, {
      type: assertion.provenance,
      source: assertion.source,
      derivedFrom: assertion.derivedFrom,
      timestamp: new Date().toISOString(),
    });

    // Set confidence
    this.state.model.setConfidence(assertion.fieldId, {
      level: assertion.confidence,
    });

    return {
      success: true,
      fieldId: assertion.fieldId,
      previousValue,
      newValue: assertion.value,
    };
  }

  /**
   * Assert multiple facts at once
   */
  assertAll(assertions: FactAssertion[]): AssertionResult[] {
    return assertions.map((a) => this.assert(a));
  }

  // ---------------------------------------------------------------------------
  // RETRACT - Remove facts
  // ---------------------------------------------------------------------------

  /**
   * Retract a fact (set to undefined)
   */
  retract(fieldId: string): AssertionResult {
    const values = this.state.model.getValues();
    const previousValue = values[fieldId];

    delete values[fieldId];

    return {
      success: true,
      fieldId,
      previousValue,
      newValue: undefined,
    };
  }

  /**
   * Retract facts matching a predicate
   */
  retractMatching(predicate: FactPredicate): AssertionResult[] {
    const result = this.query(predicate);
    return result.facts.map((f) => this.retract(f.field.id));
  }

  // ---------------------------------------------------------------------------
  // HYPOTHESIZE - Tentative facts
  // ---------------------------------------------------------------------------

  /**
   * Create a hypothesis (not yet committed)
   */
  hypothesize(
    fieldId: string,
    value: unknown,
    confidence: ConfidenceLevel,
    basis: string
  ): Hypothesis {
    const hypothesis: Hypothesis = {
      id: `hyp-${fieldId}-${Date.now()}`,
      fieldId,
      value,
      confidence,
      basis,
      status: "pending",
      createdAt: new Date().toISOString(),
    };

    this.state.hypotheses.set(hypothesis.id, hypothesis);
    return hypothesis;
  }

  /**
   * Confirm a hypothesis (commit it as a fact)
   */
  confirmHypothesis(hypothesisId: string): AssertionResult | null {
    const hypothesis = this.state.hypotheses.get(hypothesisId);
    if (!hypothesis || hypothesis.status !== "pending") {
      return null;
    }

    hypothesis.status = "confirmed";

    return this.assert({
      fieldId: hypothesis.fieldId,
      value: hypothesis.value,
      provenance: "hypothesized",
      confidence: hypothesis.confidence,
    });
  }

  /**
   * Reject a hypothesis
   */
  rejectHypothesis(hypothesisId: string): boolean {
    const hypothesis = this.state.hypotheses.get(hypothesisId);
    if (!hypothesis || hypothesis.status !== "pending") {
      return false;
    }

    hypothesis.status = "rejected";
    return true;
  }

  /**
   * Get all pending hypotheses
   */
  getPendingHypotheses(): Hypothesis[] {
    return Array.from(this.state.hypotheses.values()).filter(
      (h) => h.status === "pending"
    );
  }

  // ---------------------------------------------------------------------------
  // CONTEXT GENERATION - For agent consumption
  // ---------------------------------------------------------------------------

  /**
   * Generate context document for agent
   */
  toContext(): ContextDocument {
    return this.getView().toContextDocument();
  }

  /**
   * Generate prompt for LLM
   */
  toPrompt(): string {
    return this.getView().toPrompt();
  }

  /**
   * Generate function arguments
   */
  toFunctionArgs(): Record<string, unknown> {
    return this.getView().toFunctionArgs();
  }

  // ---------------------------------------------------------------------------
  // HISTORY & STATE
  // ---------------------------------------------------------------------------

  /**
   * Get inference history
   */
  getInferenceHistory(): InferenceResult[] {
    return [...this.state.inferenceHistory];
  }

  /**
   * Clear inference history
   */
  clearHistory(): void {
    this.state.inferenceHistory = [];
  }

  /**
   * Export full state
   */
  toJSON(): object {
    return {
      model: this.state.model.toJSON(),
      mode: this.state.mode,
      hypotheses: Object.fromEntries(this.state.hypotheses),
      inferenceHistory: this.state.inferenceHistory,
    };
  }

  // ---------------------------------------------------------------------------
  // Static Factories
  // ---------------------------------------------------------------------------

  static fromModel<T extends FormShape>(
    model: AgentModel<T>,
    mode: FormMode = "view"
  ): AgentController<T> {
    return new AgentController(model, mode);
  }

  static fromShape<T extends FormShape>(
    shape: T,
    values: Record<string, unknown> = {},
    mode: FormMode = "view"
  ): AgentController<T> {
    const model = new AgentModel(shape, values);
    return new AgentController(model, mode);
  }
}

// =============================================================================
// SIMPLE AGENT CONTROLLER - Fluent API
// =============================================================================

/**
 * SimpleAgentController - Convenient wrapper with fluent API
 */
export class SimpleAgentController<T extends FormShape = FormShape> extends AgentController<T> {
  constructor(model: AgentModel<T> | SimpleAgentModel<T>, mode: FormMode = "view") {
    super(model, mode);
  }

  // Fluent query builders
  find(predicate: FactPredicate): QueryResult {
    return this.query(predicate);
  }

  findByType(type: string): QueryResult {
    return this.query({ type });
  }

  findWithValue(): QueryResult {
    return this.query({ hasValue: true });
  }

  // Fluent assertions
  set(fieldId: string, value: unknown): this {
    this.assert({
      fieldId,
      value,
      provenance: "asserted",
      confidence: "high",
    });
    return this;
  }

  inferred(fieldId: string, value: unknown, from: string[]): this {
    this.assert({
      fieldId,
      value,
      provenance: "inferred",
      confidence: "medium",
      derivedFrom: from,
    });
    return this;
  }

  // Static factory
  static create<T extends FormShape>(
    shape: T,
    values: Record<string, unknown> = {},
    mode?: FormMode
  ): SimpleAgentController<T> {
    const model = SimpleAgentModel.create(shape, values);
    return new SimpleAgentController(model, mode);
  }
}

// =============================================================================
// AGENT MVC FACTORY - Unified creation
// =============================================================================

/**
 * AgentMVC - Factory for creating Agent Model-View-Controller
 */
export class AgentMVC {
  /**
   * Create a complete Agent MVC stack
   */
  static create<T extends FormShape>(
    shape: T,
    values: Record<string, unknown> = {},
    options: {
      mode?: FormMode;
      goal?: AgentGoal;
    } = {}
  ): {
    model: SimpleAgentModel<T>;
    view: SimpleAgentView<T>;
    controller: SimpleAgentController<T>;
  } {
    const model = SimpleAgentModel.create(shape, values);

    if (options.goal) {
      model.withGoal(options.goal);
    }

    const view = new SimpleAgentView(model, options.mode);
    const controller = new SimpleAgentController(model, options.mode);

    return { model, view, controller };
  }

  /**
   * Create with a specific goal
   */
  static withGoal<T extends FormShape>(
    shape: T,
    values: Record<string, unknown>,
    goal: AgentGoal
  ): ReturnType<typeof AgentMVC.create<T>> {
    return AgentMVC.create(shape, values, { goal });
  }
}
