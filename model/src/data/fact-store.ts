/**
 * FactStore - TypeScript interface mirroring GDS HyperFactStore
 *
 * Pipeline: GraphStore → FactStore → KnowledgeStore
 *
 * Root GDSL defines:
 * - GraphStore Interface (graph algorithms, procedures)
 * - FactStore Interface (facts, assertions) ← THIS FILE
 * - KnowledgeStore Interface (knowledge orchestration)
 *
 * FactStore represents Essential Being - Ground-Condition-Facticity
 * Being sublated into its Essence as Essential Being
 * Facticity "coming into Existence" as Property
 *
 * This is a MOCK for MVC development. Production connects to GDS.
 */

import { z } from "zod";

// =============================================================================
// CORE TYPES (Mirroring GDS fact_store.rs)
// =============================================================================

/**
 * Appearance - A raw mark
 *
 * The most primitive datum - something appeared.
 * No interpretation yet, just registration of a mark.
 */
export const AppearanceSchema = z.object({
  id: z.string(),
  groundHint: z.string().optional(),
  rawBlob: z.unknown(),
  recordedAtMs: z.number(),
  recordedBy: z.string().optional(),
});
export type Appearance = z.infer<typeof AppearanceSchema>;

/**
 * Fact - A dyadic structure
 *
 * Ground-Predicate-Value: what something IS.
 * Facts are the basic units of the FactStore.
 */
export const FactSchema = z.object({
  id: z.string(),
  ground: z.string(),
  predicate: z.string(),
  value: z.unknown(),
  originAppearance: z.string().optional(),
  createdAtMs: z.number(),
});
export type Fact = z.infer<typeof FactSchema>;

/**
 * Assertion - A triadic act
 *
 * A fact inhering in an interpretive act.
 * Who asserted it, with what confidence, in what context?
 */
export const AssertionSchema = z.object({
  id: z.string(),
  factId: z.string(),
  issuer: z.string(),
  context: z.string().optional(),
  confidence: z.number().min(0).max(1).optional(),
  tags: z.array(z.string()).default(() => []),
  provenanceBlob: z.unknown().optional(),
  validFromMs: z.number().optional(),
  validToMs: z.number().optional(),
  createdAtMs: z.number(),
});
export type Assertion = z.infer<typeof AssertionSchema>;

// =============================================================================
// FACT STORE INTERFACE
// =============================================================================

/**
 * FactStore Interface - The contract for FactStore implementations
 *
 * Both mock (in-memory) and real (GDS) implementations follow this.
 */
export interface FactStoreInterface {
  // Appearance operations
  insertAppearance(
    groundHint: string | undefined,
    rawBlob: unknown,
    recordedBy?: string
  ): Promise<string>;

  getAppearance(id: string): Promise<Appearance | undefined>;

  // Fact operations
  synthesizeFactFromAppearance(
    appearanceId: string,
    predicate: string,
    value: unknown
  ): Promise<string | undefined>;

  createFact(
    ground: string,
    predicate: string,
    value: unknown
  ): Promise<string>;

  getFact(id: string): Promise<Fact | undefined>;

  queryFacts(predicate?: string, ground?: string): Promise<Fact[]>;

  // Assertion operations
  assertFact(
    factId: string,
    issuer: string,
    context?: string,
    confidence?: number,
    tags?: string[]
  ): Promise<string | undefined>;

  getAssertion(id: string): Promise<Assertion | undefined>;

  getAssertionsForFact(factId: string): Promise<Assertion[]>;

  // Convenience: appearance → fact → assertion in one call
  interpretAppearance(
    appearanceId: string,
    predicate: string,
    value: unknown,
    issuer: string,
    context?: string,
    confidence?: number,
    tags?: string[]
  ): Promise<{ factId: string; assertionId: string } | undefined>;
}

// =============================================================================
// IN-MEMORY MOCK IMPLEMENTATION
// =============================================================================

/**
 * MockFactStore - In-memory implementation for development
 *
 * Mirrors GDS HyperFactStore but in TypeScript.
 * Use this when @logic connection is mocked.
 */
export class MockFactStore implements FactStoreInterface {
  private appearances: Map<string, Appearance> = new Map();
  private facts: Map<string, Fact> = new Map();
  private assertions: Map<string, Assertion> = new Map();
  private nextId: number = 1;

  private allocateId(): string {
    return `mock-${this.nextId++}`;
  }

  private nowMs(): number {
    return Date.now();
  }

  // ---------------------------------------------------------------------------
  // Appearance operations
  // ---------------------------------------------------------------------------

  async insertAppearance(
    groundHint: string | undefined,
    rawBlob: unknown,
    recordedBy?: string
  ): Promise<string> {
    const id = this.allocateId();
    const appearance: Appearance = {
      id,
      groundHint,
      rawBlob,
      recordedAtMs: this.nowMs(),
      recordedBy,
    };
    this.appearances.set(id, appearance);
    return id;
  }

  async getAppearance(id: string): Promise<Appearance | undefined> {
    return this.appearances.get(id);
  }

  // ---------------------------------------------------------------------------
  // Fact operations
  // ---------------------------------------------------------------------------

  async synthesizeFactFromAppearance(
    appearanceId: string,
    predicate: string,
    value: unknown
  ): Promise<string | undefined> {
    const appearance = this.appearances.get(appearanceId);
    if (!appearance) return undefined;

    const id = this.allocateId();
    const fact: Fact = {
      id,
      ground: appearance.groundHint ?? appearance.id,
      predicate,
      value,
      originAppearance: appearanceId,
      createdAtMs: this.nowMs(),
    };
    this.facts.set(id, fact);
    return id;
  }

  async createFact(
    ground: string,
    predicate: string,
    value: unknown
  ): Promise<string> {
    const id = this.allocateId();
    const fact: Fact = {
      id,
      ground,
      predicate,
      value,
      createdAtMs: this.nowMs(),
    };
    this.facts.set(id, fact);
    return id;
  }

  async getFact(id: string): Promise<Fact | undefined> {
    return this.facts.get(id);
  }

  async queryFacts(predicate?: string, ground?: string): Promise<Fact[]> {
    let results = Array.from(this.facts.values());

    if (predicate) {
      results = results.filter(f => f.predicate === predicate);
    }
    if (ground) {
      results = results.filter(f => f.ground === ground);
    }

    return results;
  }

  // ---------------------------------------------------------------------------
  // Assertion operations
  // ---------------------------------------------------------------------------

  async assertFact(
    factId: string,
    issuer: string,
    context?: string,
    confidence?: number,
    tags: string[] = []
  ): Promise<string | undefined> {
    if (!this.facts.has(factId)) return undefined;

    const id = this.allocateId();
    const now = this.nowMs();
    const assertion: Assertion = {
      id,
      factId,
      issuer,
      context,
      confidence,
      tags,
      validFromMs: now,
      createdAtMs: now,
    };
    this.assertions.set(id, assertion);
    return id;
  }

  async getAssertion(id: string): Promise<Assertion | undefined> {
    return this.assertions.get(id);
  }

  async getAssertionsForFact(factId: string): Promise<Assertion[]> {
    return Array.from(this.assertions.values())
      .filter(a => a.factId === factId);
  }

  // ---------------------------------------------------------------------------
  // Convenience operations
  // ---------------------------------------------------------------------------

  async interpretAppearance(
    appearanceId: string,
    predicate: string,
    value: unknown,
    issuer: string,
    context?: string,
    confidence?: number,
    tags: string[] = []
  ): Promise<{ factId: string; assertionId: string } | undefined> {
    const factId = await this.synthesizeFactFromAppearance(
      appearanceId,
      predicate,
      value
    );
    if (!factId) return undefined;

    const assertionId = await this.assertFact(
      factId,
      issuer,
      context,
      confidence,
      tags
    );
    if (!assertionId) return undefined;

    return { factId, assertionId };
  }

  // ---------------------------------------------------------------------------
  // Utility methods (for testing/debugging)
  // ---------------------------------------------------------------------------

  getAllFacts(): Fact[] {
    return Array.from(this.facts.values());
  }

  getAllAssertions(): Assertion[] {
    return Array.from(this.assertions.values());
  }

  clear(): void {
    this.appearances.clear();
    this.facts.clear();
    this.assertions.clear();
    this.nextId = 1;
  }
}

// =============================================================================
// SINGLETON MOCK INSTANCE
// =============================================================================

/**
 * Default mock FactStore instance
 *
 * Use this for development. Replace with real GDS connection in production.
 */
let defaultFactStore: FactStoreInterface | undefined;

export function getFactStore(): FactStoreInterface {
  if (!defaultFactStore) {
    defaultFactStore = new MockFactStore();
  }
  return defaultFactStore;
}

export function setFactStore(store: FactStoreInterface): void {
  defaultFactStore = store;
}

export function resetFactStore(): void {
  defaultFactStore = new MockFactStore();
}

