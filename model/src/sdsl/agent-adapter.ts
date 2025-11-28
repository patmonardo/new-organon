/**
 * Agent Adapter - Transform FactStore to Agent-Consumable Formats
 *
 * While ReactAdapter transforms to React/JSX for human UI,
 * AgentAdapter transforms to formats agents can consume:
 *
 * - Prompt: Markdown/text for LLM prompt injection
 * - FunctionCall: Structured args for tool/function calling
 * - GraphQL: Query format for structured data access
 * - RDF: Triples for knowledge graph systems
 * - JSON-LD: Linked data format
 *
 * This is the "last mile" adapter for agent consumption.
 */

import { z } from "zod";
import type { FormShape, FormMode } from "./types";
import { AgentModel, SimpleAgentModel, type AgentGoal } from "./agent-model";
import { AgentView, type ContextDocument, type StructuredFact } from "./agent-view";
import { AgentController } from "./agent-controller";

// =============================================================================
// AGENT ADAPTER INTERFACE
// =============================================================================

/**
 * AgentAdapterInterface - The agent equivalent of Adapter
 *
 * Unlike Form Adapter (which transforms DisplayDocument → UI),
 * Agent Adapter transforms ContextDocument → Agent-consumable formats
 */
export interface AgentAdapterInterface<T> {
  readonly name: string;
  adapt(model: AgentModel): ContextDocument;
  transform(context: ContextDocument): T;
}

// =============================================================================
// AGENT OUTPUT FORMATS
// =============================================================================

/**
 * Output format types for agent consumption
 */
export type AgentOutputFormat =
  | "prompt"        // Markdown/text for LLM
  | "function"      // Function call arguments
  | "graphql"       // GraphQL query/mutation
  | "rdf"           // RDF triples
  | "jsonld"        // JSON-LD linked data
  | "context";      // Raw ContextDocument

// =============================================================================
// PROMPT OUTPUT - For LLM Consumption
// =============================================================================

export interface PromptOutput {
  type: "prompt";
  content: string;
  sections: {
    goal?: string;
    schema?: string;
    facts?: string;
    constraints?: string;
  };
  tokenEstimate?: number;
}

// =============================================================================
// FUNCTION CALL OUTPUT - For Tool Calling
// =============================================================================

export interface FunctionCallOutput {
  type: "function";
  name: string;
  arguments: Record<string, unknown>;
  schema: {
    type: "object";
    properties: Record<string, { type: string; description?: string }>;
    required: string[];
  };
}

// =============================================================================
// GRAPHQL OUTPUT - For Structured Queries
// =============================================================================

export interface GraphQLOutput {
  type: "graphql";
  query: string;
  variables: Record<string, unknown>;
  operationType: "query" | "mutation";
}

// =============================================================================
// RDF OUTPUT - For Knowledge Graphs
// =============================================================================

export interface RDFTriple {
  subject: string;
  predicate: string;
  object: string | number | boolean;
  objectType?: "uri" | "literal" | "blank";
}

export interface RDFOutput {
  type: "rdf";
  triples: RDFTriple[];
  prefixes: Record<string, string>;
  format: "ntriples" | "turtle" | "jsonld";
}

// =============================================================================
// JSON-LD OUTPUT - For Linked Data
// =============================================================================

export interface JSONLDOutput {
  type: "jsonld";
  "@context": Record<string, string | object>;
  "@type": string;
  "@id": string;
  [key: string]: unknown;
}

// =============================================================================
// AGENT ADAPTER CLASS
// =============================================================================

/**
 * AgentAdapter - Transforms ContextDocument to agent-consumable formats
 *
 * This is the agent equivalent of ReactAdapter
 */
export class AgentAdapter implements AgentAdapterInterface<ContextDocument> {
  readonly name = "agent";
  private defaultNamespace: string;
  private prefixes: Record<string, string>;

  constructor(options: {
    namespace?: string;
    prefixes?: Record<string, string>;
  } = {}) {
    this.defaultNamespace = options.namespace || "http://example.org/fact/";
    this.prefixes = options.prefixes || {
      fact: this.defaultNamespace,
      rdf: "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
      rdfs: "http://www.w3.org/2000/01/rdf-schema#",
      xsd: "http://www.w3.org/2001/XMLSchema#",
    };
  }

  // ---------------------------------------------------------------------------
  // Core Adapter Interface
  // ---------------------------------------------------------------------------

  /**
   * Adapt model to ContextDocument (default format)
   */
  adapt(model: AgentModel): ContextDocument {
    const view = new AgentView(model);
    return view.toContextDocument();
  }

  /**
   * Transform ContextDocument (identity for this adapter)
   */
  transform(context: ContextDocument): ContextDocument {
    return context;
  }

  // ---------------------------------------------------------------------------
  // PROMPT FORMATTING - For LLM
  // ---------------------------------------------------------------------------

  /**
   * Format for LLM prompt injection
   */
  toPrompt(context: ContextDocument): PromptOutput {
    const sections: PromptOutput["sections"] = {};
    const lines: string[] = [];

    // Goal section
    if (context.goal) {
      sections.goal = `## Goal\n**${context.goal.type}**: ${context.goal.description}`;
      lines.push(sections.goal, "");
    }

    // Schema section
    sections.schema = this.formatSchemaSection(context);
    lines.push(sections.schema, "");

    // Facts section
    sections.facts = this.formatFactsSection(context);
    lines.push(sections.facts, "");

    // Constraints section
    if (context.constraints && context.constraints.length > 0) {
      sections.constraints = this.formatConstraintsSection(context);
      lines.push(sections.constraints);
    }

    const content = lines.join("\n");

    return {
      type: "prompt",
      content,
      sections,
      tokenEstimate: this.estimateTokens(content),
    };
  }

  protected formatSchemaSection(context: ContextDocument): string {
    const lines = ["## Schema"];
    lines.push(`**Name**: ${context.schema.name || context.schema.id}`);
    if (context.schema.description) {
      lines.push(`**Description**: ${context.schema.description}`);
    }
    lines.push(`**Fields**: ${context.schema.fieldCount}`);
    if (context.schema.requiredFields.length > 0) {
      lines.push(`**Required**: ${context.schema.requiredFields.join(", ")}`);
    }
    return lines.join("\n");
  }

  protected formatFactsSection(context: ContextDocument): string {
    const lines = ["## Facts"];

    for (const fact of context.facts) {
      const label = fact.label || fact.id;
      const value = this.formatFactValue(fact.value);
      const meta: string[] = [];

      if (fact.relevance && fact.relevance !== "relevant") {
        meta.push(`${fact.relevance}`);
      }
      if (fact.confidence && fact.confidence !== "high") {
        meta.push(`${fact.confidence} confidence`);
      }

      if (meta.length > 0) {
        lines.push(`- **${label}**: ${value} *(${meta.join(", ")})*`);
      } else {
        lines.push(`- **${label}**: ${value}`);
      }
    }

    return lines.join("\n");
  }

  protected formatConstraintsSection(context: ContextDocument): string {
    const lines = ["## Constraints"];
    for (const c of context.constraints || []) {
      const marker = c.type === "must" ? "🔴" :
                     c.type === "must_not" ? "⛔" :
                     c.type === "should" ? "🟡" : "⚪";
      lines.push(`${marker} [${c.type.toUpperCase()}] ${c.description}`);
    }
    return lines.join("\n");
  }

  protected formatFactValue(value: unknown): string {
    if (value === null || value === undefined) return "*(empty)*";
    if (typeof value === "string") return value || "*(empty string)*";
    if (typeof value === "number") return String(value);
    if (typeof value === "boolean") return value ? "Yes" : "No";
    if (Array.isArray(value)) return `[${value.length} items]`;
    if (typeof value === "object") return `{${Object.keys(value).length} properties}`;
    return String(value);
  }

  protected estimateTokens(text: string): number {
    // Rough estimate: ~4 chars per token
    return Math.ceil(text.length / 4);
  }

  // ---------------------------------------------------------------------------
  // FUNCTION CALL FORMATTING - For Tool Calling
  // ---------------------------------------------------------------------------

  /**
   * Format as function call for tool invocation
   */
  toFunctionCall(
    context: ContextDocument,
    functionName: string,
    options: { description?: string } = {}
  ): FunctionCallOutput {
    const properties: Record<string, { type: string; description?: string }> = {};
    const args: Record<string, unknown> = {};
    const required: string[] = [];

    for (const fact of context.facts) {
      const propType = this.inferJSONSchemaType(fact.value);
      properties[fact.id] = {
        type: propType,
        description: fact.label,
      };

      if (fact.value !== null && fact.value !== undefined) {
        args[fact.id] = fact.value;
      }

      if (context.schema.requiredFields.includes(fact.id)) {
        required.push(fact.id);
      }
    }

    return {
      type: "function",
      name: functionName,
      arguments: args,
      schema: {
        type: "object",
        properties,
        required,
      },
    };
  }

  protected inferJSONSchemaType(value: unknown): string {
    if (value === null || value === undefined) return "string";
    if (typeof value === "string") return "string";
    if (typeof value === "number") return Number.isInteger(value) ? "integer" : "number";
    if (typeof value === "boolean") return "boolean";
    if (Array.isArray(value)) return "array";
    if (typeof value === "object") return "object";
    return "string";
  }

  // ---------------------------------------------------------------------------
  // GRAPHQL FORMATTING - For Structured Queries
  // ---------------------------------------------------------------------------

  /**
   * Format as GraphQL query
   */
  toGraphQL(
    context: ContextDocument,
    options: {
      operationType?: "query" | "mutation";
      operationName?: string;
    } = {}
  ): GraphQLOutput {
    const opType = options.operationType || "query";
    const opName = options.operationName ||
      (opType === "query" ? "GetFacts" : "UpdateFacts");

    const variables: Record<string, unknown> = {};
    const variableTypes: string[] = [];
    const fieldSelections: string[] = [];

    for (const fact of context.facts) {
      const varName = fact.id;
      const gqlType = this.inferGraphQLType(fact.value);

      if (fact.value !== null && fact.value !== undefined) {
        variables[varName] = fact.value;
        variableTypes.push(`$${varName}: ${gqlType}`);
      }

      fieldSelections.push(`  ${fact.id}`);
    }

    const variableDecl = variableTypes.length > 0
      ? `(${variableTypes.join(", ")})`
      : "";

    const query = `${opType} ${opName}${variableDecl} {
  ${context.schema.id || "facts"} {
${fieldSelections.join("\n")}
  }
}`;

    return {
      type: "graphql",
      query,
      variables,
      operationType: opType,
    };
  }

  protected inferGraphQLType(value: unknown): string {
    if (value === null || value === undefined) return "String";
    if (typeof value === "string") return "String";
    if (typeof value === "number") return Number.isInteger(value) ? "Int" : "Float";
    if (typeof value === "boolean") return "Boolean";
    if (Array.isArray(value)) return "[String]";
    return "String";
  }

  // ---------------------------------------------------------------------------
  // RDF FORMATTING - For Knowledge Graphs
  // ---------------------------------------------------------------------------

  /**
   * Format as RDF triples
   */
  toRDF(
    context: ContextDocument,
    options: { format?: "ntriples" | "turtle" | "jsonld" } = {}
  ): RDFOutput {
    const triples: RDFTriple[] = [];
    const subjectUri = `${this.defaultNamespace}${context.id}`;

    // Type triple
    triples.push({
      subject: subjectUri,
      predicate: `${this.prefixes.rdf}type`,
      object: `${this.defaultNamespace}FormShape`,
      objectType: "uri",
    });

    // Schema triples
    if (context.schema.name) {
      triples.push({
        subject: subjectUri,
        predicate: `${this.prefixes.rdfs}label`,
        object: context.schema.name,
        objectType: "literal",
      });
    }

    // Fact triples
    for (const fact of context.facts) {
      if (fact.value !== null && fact.value !== undefined) {
        triples.push({
          subject: subjectUri,
          predicate: `${this.defaultNamespace}${fact.id}`,
          object: this.toRDFValue(fact.value),
          objectType: this.getRDFObjectType(fact.value),
        });
      }
    }

    return {
      type: "rdf",
      triples,
      prefixes: this.prefixes,
      format: options.format || "turtle",
    };
  }

  protected toRDFValue(value: unknown): string | number | boolean {
    if (typeof value === "string") return value;
    if (typeof value === "number") return value;
    if (typeof value === "boolean") return value;
    return JSON.stringify(value);
  }

  protected getRDFObjectType(value: unknown): "uri" | "literal" | "blank" {
    return "literal";
  }

  /**
   * Format RDF as Turtle string
   */
  toTurtle(context: ContextDocument): string {
    const rdf = this.toRDF(context, { format: "turtle" });
    const lines: string[] = [];

    // Prefixes
    for (const [prefix, uri] of Object.entries(rdf.prefixes)) {
      lines.push(`@prefix ${prefix}: <${uri}> .`);
    }
    lines.push("");

    // Group triples by subject
    const bySubject = new Map<string, RDFTriple[]>();
    for (const triple of rdf.triples) {
      const existing = bySubject.get(triple.subject) || [];
      existing.push(triple);
      bySubject.set(triple.subject, existing);
    }

    // Format each subject
    for (const [subject, subjectTriples] of bySubject) {
      lines.push(`<${subject}>`);
      for (let i = 0; i < subjectTriples.length; i++) {
        const t = subjectTriples[i];
        const predicate = this.abbreviateUri(t.predicate);
        const object = t.objectType === "uri"
          ? `<${t.object}>`
          : `"${t.object}"`;
        const terminator = i === subjectTriples.length - 1 ? " ." : " ;";
        lines.push(`  ${predicate} ${object}${terminator}`);
      }
      lines.push("");
    }

    return lines.join("\n");
  }

  protected abbreviateUri(uri: string): string {
    for (const [prefix, namespace] of Object.entries(this.prefixes)) {
      if (uri.startsWith(namespace)) {
        return `${prefix}:${uri.slice(namespace.length)}`;
      }
    }
    return `<${uri}>`;
  }

  // ---------------------------------------------------------------------------
  // JSON-LD FORMATTING - For Linked Data
  // ---------------------------------------------------------------------------

  /**
   * Format as JSON-LD
   */
  toJSONLD(context: ContextDocument): JSONLDOutput {
    const contextObj: Record<string, string | object> = {
      "@vocab": this.defaultNamespace,
      "xsd": this.prefixes.xsd,
      "rdfs": this.prefixes.rdfs,
    };

    const result: JSONLDOutput = {
      type: "jsonld",
      "@context": contextObj,
      "@type": "FormShape",
      "@id": `${this.defaultNamespace}${context.id}`,
    };

    // Add schema metadata
    if (context.schema.name) {
      result["rdfs:label"] = context.schema.name;
    }
    if (context.schema.description) {
      result["rdfs:comment"] = context.schema.description;
    }

    // Add facts
    for (const fact of context.facts) {
      if (fact.value !== null && fact.value !== undefined) {
        result[fact.id] = fact.value;
      }
    }

    return result;
  }

  // ---------------------------------------------------------------------------
  // UNIFIED OUTPUT - All formats
  // ---------------------------------------------------------------------------

  /**
   * Convert to any supported format
   */
  toFormat(
    context: ContextDocument,
    format: AgentOutputFormat,
    options: Record<string, unknown> = {}
  ): PromptOutput | FunctionCallOutput | GraphQLOutput | RDFOutput | JSONLDOutput | ContextDocument {
    switch (format) {
      case "prompt":
        return this.toPrompt(context);
      case "function":
        return this.toFunctionCall(context, options.functionName as string || "processData");
      case "graphql":
        return this.toGraphQL(context, options as { operationType?: "query" | "mutation" });
      case "rdf":
        return this.toRDF(context, options as { format?: "ntriples" | "turtle" | "jsonld" });
      case "jsonld":
        return this.toJSONLD(context);
      case "context":
      default:
        return context;
    }
  }

  // ---------------------------------------------------------------------------
  // Static Factories
  // ---------------------------------------------------------------------------

  /**
   * Create adapter with default options
   */
  static create(options?: { namespace?: string }): AgentAdapter {
    return new AgentAdapter(options);
  }

  /**
   * Quick convert: Model → Format
   */
  static convert<T extends FormShape>(
    model: AgentModel<T>,
    format: AgentOutputFormat,
    options?: Record<string, unknown>
  ): ReturnType<AgentAdapter["toFormat"]> {
    const adapter = new AgentAdapter();
    const context = adapter.adapt(model);
    return adapter.toFormat(context, format, options);
  }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/**
 * Quick conversion functions
 */
export function toAgentPrompt<T extends FormShape>(
  model: AgentModel<T>
): string {
  const adapter = new AgentAdapter();
  const context = adapter.adapt(model);
  return adapter.toPrompt(context).content;
}

export function toAgentFunctionCall<T extends FormShape>(
  model: AgentModel<T>,
  functionName: string
): FunctionCallOutput {
  const adapter = new AgentAdapter();
  const context = adapter.adapt(model);
  return adapter.toFunctionCall(context, functionName);
}

export function toAgentGraphQL<T extends FormShape>(
  model: AgentModel<T>,
  operationType: "query" | "mutation" = "query"
): GraphQLOutput {
  const adapter = new AgentAdapter();
  const context = adapter.adapt(model);
  return adapter.toGraphQL(context, { operationType });
}

export function toAgentJSONLD<T extends FormShape>(
  model: AgentModel<T>
): JSONLDOutput {
  const adapter = new AgentAdapter();
  const context = adapter.adapt(model);
  return adapter.toJSONLD(context);
}
