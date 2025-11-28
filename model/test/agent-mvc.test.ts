/**
 * Agent MVC Tests - FactStore Client SDK for Agents
 *
 * These tests demonstrate the Agent MVC as the agent-facing
 * interface to FactStore, parallel to Form MVC for humans.
 */

import { describe, it, expect } from "vitest";
import {
  // Agent Model
  AgentModel,
  SimpleAgentModel,
  type AgentGoal,
  type RelevanceLevel,
  type ConfidenceLevel,
  type ProvenanceType,

  // Agent View
  AgentView,
  SimpleAgentView,
  type ContextDocument,

  // Agent Controller
  AgentController,
  SimpleAgentController,
  AgentMVC,
  type InferenceRule,

  // Agent Adapter
  AgentAdapter,
  toAgentPrompt,
  toAgentFunctionCall,
  toAgentJSONLD,

  // Form types (shared)
  type FormShape,
} from "../src/sdsl";

// =============================================================================
// TEST FIXTURES
// =============================================================================

const personShape: FormShape = {
  id: "person",
  name: "Person",
  description: "A person entity",
  fields: [
    { id: "name", type: "text", label: "Full Name", required: true, disabled: false },
    { id: "age", type: "number", label: "Age", required: false, disabled: false },
    { id: "email", type: "email", label: "Email Address", required: true, disabled: false },
    { id: "isActive", type: "boolean", label: "Active Status", required: false, disabled: false },
  ],
};

const personValues = {
  name: "Alice Smith",
  age: 30,
  email: "alice@example.com",
  isActive: true,
};

const researchGoal: AgentGoal = {
  id: "goal-1",
  type: "query",
  description: "Find contact information for the person",
  constraints: ["Only use verified data", "Respect privacy settings"],
  priority: 1,
};

// =============================================================================
// AGENT MODEL TESTS
// =============================================================================

describe("AgentModel", () => {
  it("should create from shape and values", () => {
    const model = new AgentModel(personShape, personValues);

    expect(model.getShape()).toBe(personShape);
    expect(model.getValues()).toEqual(personValues);
    expect(model.getValue("name")).toBe("Alice Smith");
  });

  it("should manage relevance overlays", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .markRelevant("email", "critical", "Primary contact method")
      .markRelevant("name", "important", "For addressing")
      .markRelevant("age", "peripheral", "Not needed for contact");

    expect(model.getRelevance("email")?.level).toBe("critical");
    expect(model.getRelevance("name")?.level).toBe("important");

    const relevant = model.getRelevantFields("important");
    expect(relevant.map(f => f.id)).toContain("email");
    expect(relevant.map(f => f.id)).toContain("name");
    expect(relevant.map(f => f.id)).not.toContain("age");
  });

  it("should manage provenance overlays", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .markProvenance("name", "asserted", "User input")
      .markProvenance("email", "observed", "From API");

    expect(model.getProvenance("name")?.type).toBe("asserted");
    expect(model.getProvenance("email")?.type).toBe("observed");

    const asserted = model.getFactsByProvenance("asserted");
    expect(asserted.map(f => f.id)).toContain("name");
  });

  it("should manage confidence overlays", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .markConfidence("email", "certain", "Verified")
      .markConfidence("age", "low", "Estimated");

    expect(model.getConfidence("email")?.level).toBe("certain");
    expect(model.getConfidence("age")?.level).toBe("low");

    const certain = model.getCertainFacts("high");
    expect(certain.map(f => f.id)).toContain("email");
    expect(certain.map(f => f.id)).not.toContain("age");
  });

  it("should manage dependencies", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .addLink("email", "name", "requires")
      .addLink("isActive", "email", "implies");

    const emailDeps = model.getDependencies("email");
    expect(emailDeps.length).toBe(2);

    expect(model.getDependentsOf("name")).toContain("email");
    expect(model.getDependenciesOf("email")).toContain("name");
  });

  it("should export to fact records", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .markRelevant("email", "critical")
      .markConfidence("email", "certain");

    const records = model.toFactRecords();
    const emailRecord = records.find(r => r.id === "email");

    expect(emailRecord?.value).toBe("alice@example.com");
    expect(emailRecord?.relevance).toBe("critical");
    expect(emailRecord?.confidence).toBe("certain");
  });
});

// =============================================================================
// AGENT VIEW TESTS
// =============================================================================

describe("AgentView", () => {
  it("should render ContextDocument", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .withGoal(researchGoal);
    const view = new AgentView(model);

    const ctx = view.toContextDocument();

    expect(ctx.id).toMatch(/^ctx-person-/);
    expect(ctx.schema.id).toBe("person");
    expect(ctx.schema.name).toBe("Person");
    expect(ctx.facts.length).toBe(4);
    expect(ctx.goal?.type).toBe("query");
  });

  it("should render prompt for LLM", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .withGoal(researchGoal);
    const view = new AgentView(model);

    const prompt = view.toPrompt();

    expect(prompt).toContain("## Goal");
    expect(prompt).toContain("Find contact information");
    expect(prompt).toContain("## Facts");
    expect(prompt).toContain("Alice Smith");
    expect(prompt).toContain("alice@example.com");
  });

  it("should render function arguments", () => {
    const model = SimpleAgentModel.create(personShape, personValues);
    const view = new AgentView(model);

    const args = view.toFunctionArgs();

    expect(args.name).toBe("Alice Smith");
    expect(args.email).toBe("alice@example.com");
    expect(args.age).toBe(30);
  });

  it("should render graph structure", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .addLink("email", "name", "requires");
    const view = new AgentView(model);

    const graph = view.toGraph();

    expect(graph.nodes.length).toBe(5); // 1 shape + 4 fields
    expect(graph.edges.length).toBe(5); // 4 hasField + 1 requires
    expect(graph.edges.some(e => e.type === "requires")).toBe(true);
  });

  it("should filter by relevance", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .markRelevant("email", "critical")
      .markRelevant("name", "important")
      .markRelevant("age", "irrelevant");
    const view = new SimpleAgentView(model);

    const relevant = view.relevantOnly("important");

    expect(relevant.map(f => f.id)).toContain("email");
    expect(relevant.map(f => f.id)).toContain("name");
    expect(relevant.map(f => f.id)).not.toContain("age");
  });
});

// =============================================================================
// AGENT CONTROLLER TESTS
// =============================================================================

describe("AgentController", () => {
  it("should query facts by predicate", () => {
    const controller = SimpleAgentController.create(personShape, personValues);

    const result = controller.query({ type: "text" });
    expect(result.count).toBe(1);
    expect(result.facts[0].field.id).toBe("name");

    const withValue = controller.query({ hasValue: true });
    expect(withValue.count).toBe(4);
  });

  it("should assert new facts", () => {
    const controller = SimpleAgentController.create(personShape, {});

    const result = controller.assert({
      fieldId: "name",
      value: "Bob Jones",
      provenance: "asserted",
      confidence: "high",
    });

    expect(result.success).toBe(true);
    expect(result.newValue).toBe("Bob Jones");
    expect(controller.getModel().getValue("name")).toBe("Bob Jones");
    expect(controller.getModel().getProvenance("name")?.type).toBe("asserted");
  });

  it("should retract facts", () => {
    const controller = SimpleAgentController.create(personShape, personValues);

    expect(controller.getModel().getValue("age")).toBe(30);

    const result = controller.retract("age");

    expect(result.success).toBe(true);
    expect(result.previousValue).toBe(30);
    expect(controller.getModel().getValue("age")).toBeUndefined();
  });

  it("should apply inference rules", () => {
    const controller = SimpleAgentController.create(personShape, personValues);

    // First verify the query works
    const ageQuery = controller.query({ fieldId: "age" });
    expect(ageQuery.count).toBe(1);
    expect(ageQuery.facts[0].value).toBe(30);

    const rule: InferenceRule = {
      id: "is-adult",
      name: "Adult Check",
      // Simpler premise - just match by fieldId
      premises: [{ fieldId: "age" }],
      derive: (facts) => {
        const ageFact = facts.find(f => f.field.id === "age");
        if (ageFact && typeof ageFact.value === "number" && ageFact.value >= 18) {
          return { fieldId: "isActive", value: true };
        }
        return null;
      },
      confidence: "high",
    };

    const result = controller.infer(rule);

    expect(result.success).toBe(true);
    expect(result.derivedFact?.fieldId).toBe("isActive");
    expect(result.derivedFact?.value).toBe(true);
    expect(result.derivedFact?.derivedFrom).toContain("age");
  });

  it("should manage hypotheses", () => {
    const controller = SimpleAgentController.create(personShape, personValues);

    // Create hypothesis
    const hypothesis = controller.hypothesize(
      "email",
      "bob@example.com",
      "medium",
      "Guessed from name pattern"
    );

    expect(hypothesis.status).toBe("pending");
    expect(controller.getPendingHypotheses().length).toBe(1);

    // Reject it
    controller.rejectHypothesis(hypothesis.id);
    expect(controller.getPendingHypotheses().length).toBe(0);

    // Create and confirm another
    const h2 = controller.hypothesize("age", 25, "low", "Estimated");
    const result = controller.confirmHypothesis(h2.id);

    expect(result?.success).toBe(true);
    expect(controller.getModel().getValue("age")).toBe(25);
  });

  it("should generate context for agent consumption", () => {
    const controller = SimpleAgentController.create(personShape, personValues);

    const context = controller.toContext();
    expect(context.facts.length).toBe(4);

    const prompt = controller.toPrompt();
    expect(prompt).toContain("Alice Smith");

    const args = controller.toFunctionArgs();
    expect(args.name).toBe("Alice Smith");
  });
});

// =============================================================================
// AGENT ADAPTER TESTS
// =============================================================================

describe("AgentAdapter", () => {
  const adapter = new AgentAdapter();

  it("should adapt model to ContextDocument", () => {
    const model = SimpleAgentModel.create(personShape, personValues);
    const context = adapter.adapt(model);

    expect(context.schema.id).toBe("person");
    expect(context.facts.length).toBe(4);
  });

  it("should format as prompt", () => {
    const model = SimpleAgentModel.create(personShape, personValues)
      .withGoal(researchGoal);
    const context = adapter.adapt(model);
    const prompt = adapter.toPrompt(context);

    expect(prompt.type).toBe("prompt");
    expect(prompt.content).toContain("## Goal");
    expect(prompt.content).toContain("## Schema");
    expect(prompt.content).toContain("## Facts");
    expect(prompt.sections.goal).toBeDefined();
    expect(prompt.tokenEstimate).toBeGreaterThan(0);
  });

  it("should format as function call", () => {
    const model = SimpleAgentModel.create(personShape, personValues);
    const context = adapter.adapt(model);
    const func = adapter.toFunctionCall(context, "updatePerson");

    expect(func.type).toBe("function");
    expect(func.name).toBe("updatePerson");
    expect(func.arguments.name).toBe("Alice Smith");
    expect(func.schema.properties.name.type).toBe("string");
    expect(func.schema.required).toContain("name");
  });

  it("should format as GraphQL", () => {
    const model = SimpleAgentModel.create(personShape, personValues);
    const context = adapter.adapt(model);
    const gql = adapter.toGraphQL(context, { operationType: "query" });

    expect(gql.type).toBe("graphql");
    expect(gql.operationType).toBe("query");
    expect(gql.query).toContain("query GetFacts");
    expect(gql.query).toContain("name");
    expect(gql.query).toContain("email");
  });

  it("should format as RDF triples", () => {
    const model = SimpleAgentModel.create(personShape, personValues);
    const context = adapter.adapt(model);
    const rdf = adapter.toRDF(context);

    expect(rdf.type).toBe("rdf");
    expect(rdf.triples.length).toBeGreaterThan(0);
    expect(rdf.triples.some(t => t.object === "Alice Smith")).toBe(true);
  });

  it("should format as JSON-LD", () => {
    const model = SimpleAgentModel.create(personShape, personValues);
    const context = adapter.adapt(model);
    const jsonld = adapter.toJSONLD(context);

    expect(jsonld.type).toBe("jsonld");
    expect(jsonld["@type"]).toBe("FormShape");
    expect(jsonld["@context"]).toBeDefined();
    expect(jsonld.name).toBe("Alice Smith");
  });

  it("should provide convenience functions", () => {
    const model = SimpleAgentModel.create(personShape, personValues);

    const prompt = toAgentPrompt(model);
    expect(prompt).toContain("Alice Smith");

    const func = toAgentFunctionCall(model, "test");
    expect(func.name).toBe("test");

    const jsonld = toAgentJSONLD(model);
    expect(jsonld["@type"]).toBe("FormShape");
  });
});

// =============================================================================
// AGENT MVC FACTORY TESTS
// =============================================================================

describe("AgentMVC", () => {
  it("should create complete MVC stack", () => {
    const { model, view, controller } = AgentMVC.create(
      personShape,
      personValues,
      { goal: researchGoal }
    );

    expect(model.getGoal()?.type).toBe("query");
    expect(view.toContextDocument().goal?.description).toBe(
      "Find contact information for the person"
    );
    expect(controller.query({ hasValue: true }).count).toBe(4);
  });

  it("should create with goal shorthand", () => {
    const { model } = AgentMVC.withGoal(personShape, personValues, researchGoal);

    expect(model.getGoal()).toEqual(researchGoal);
  });
});

// =============================================================================
// PARALLEL COMPARISON: FORM MVC vs AGENT MVC
// =============================================================================

describe("Form MVC vs Agent MVC (Parallel Structure)", () => {
  it("should have parallel Model structures", () => {
    // Form MVC: structure + values
    // Agent MVC: structure + values + overlays (relevance, confidence, provenance)

    const agentModel = SimpleAgentModel.create(personShape, personValues)
      .markRelevant("email", "critical")
      .markConfidence("email", "certain")
      .markProvenance("email", "observed");

    // Agent model adds semantic overlays
    expect(agentModel.getRelevance("email")?.level).toBe("critical");
    expect(agentModel.getConfidence("email")?.level).toBe("certain");
    expect(agentModel.getProvenance("email")?.type).toBe("observed");
  });

  it("should have parallel View outputs", () => {
    // Form MVC: DisplayDocument → React/HTML
    // Agent MVC: ContextDocument → Prompts/Functions/Graphs

    const agentView = SimpleAgentView.create(personShape, personValues);

    // Multiple output formats for different agent interfaces
    const context = agentView.asContext();  // Raw structured data
    const prompt = agentView.asPrompt();    // For LLM
    const args = agentView.asFunctionArgs(); // For tool calling
    const graph = agentView.asGraph();      // For knowledge operations

    expect(context.facts.length).toBe(4);
    expect(prompt).toContain("Alice Smith");
    expect(args.name).toBe("Alice Smith");
    expect(graph.nodes.length).toBe(5);
  });

  it("should have parallel Controller operations", () => {
    // Form MVC: submit, validate, setField (CRUD for humans)
    // Agent MVC: query, infer, assert, retract (reasoning for agents)

    const controller = SimpleAgentController.create(personShape, personValues);

    // Agent-specific operations
    const query = controller.query({ type: "email" });
    expect(query.facts[0].value).toBe("alice@example.com");

    controller.set("age", 31); // Fluent assertion
    expect(controller.getModel().getValue("age")).toBe(31);

    controller.retract("isActive");
    expect(controller.getModel().getValue("isActive")).toBeUndefined();
  });
});
