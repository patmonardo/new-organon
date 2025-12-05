import { z } from 'zod';
import { MalloyIRSchema } from './malloy-ir';

/**
 * Malloy Ontological IR - OWL/SHACL/SPIN/SPARQL Unified
 * 
 * This schema embeds a next generation Ontological pattern by unifying:
 * - OWL (Web Ontology Language) - Ontology definitions
 * - SHACL (Shapes Constraint Language) - Data validation
 * - SPIN (SPARQL Inferencing Notation) - Rules and inferencing
 * - SPARQL - Query and pattern matching
 * 
 * As a single coherent language embedded in Malloy, avoiding the
 * restrictive Java toolchains while getting the power of semantic ontologies.
 */

// ============================================================================
// OWL (Web Ontology Language)
// ============================================================================

// OWL Class Definition
export const OWLClassSchema = z.object({
  name: z.string(),
  subClassOf: z.array(z.string()).optional(), // Parent classes
  disjointWith: z.array(z.string()).optional(), // Disjoint classes
  equivalentTo: z.array(z.string()).optional(), // Equivalent classes
  // Properties
  properties: z.record(z.object({
    type: z.enum(['data', 'object']), // Data property or object property
    range: z.union([z.string(), z.array(z.string())]).optional(), // Value type or target class
    domain: z.string().optional(), // Source class
    functional: z.boolean().optional(), // Single value
    inverseFunctional: z.boolean().optional(),
    transitive: z.boolean().optional(),
    symmetric: z.boolean().optional(),
  })).optional(),
  // Axioms
  axioms: z.array(z.object({
    type: z.string(),
    description: z.string(),
    logical: z.any(), // Logical expression
  })).optional(),
});

// ============================================================================
// SHACL (Shapes Constraint Language)
// ============================================================================

// SHACL Shape
export const SHACLShapeSchema = z.object({
  name: z.string(),
  targetClass: z.string(), // Target OWL class
  // Property constraints
  properties: z.record(z.object({
    path: z.string(), // Property path
    datatype: z.string().optional(),
    minCount: z.number().optional(),
    maxCount: z.number().optional(),
    minLength: z.number().optional(),
    maxLength: z.number().optional(),
    pattern: z.string().optional(),
    minInclusive: z.any().optional(),
    maxInclusive: z.any().optional(),
    in: z.array(z.any()).optional(), // Enumeration
    unique: z.boolean().optional(),
    nodeKind: z.enum(['IRI', 'Literal', 'BlankNode']).optional(),
    class: z.string().optional(), // Target class for object properties
  })).optional(),
  // Node constraints
  closed: z.boolean().optional(), // Closed shape (only specified properties)
  ignoredProperties: z.array(z.string()).optional(),
});

// ============================================================================
// SPIN (SPARQL Inferencing Notation)
// ============================================================================

// SPIN Rule
export const SPINRuleSchema = z.object({
  name: z.string(),
  type: z.enum(['constraint', 'inference', 'magic-property']),
  // Condition (when to apply)
  condition: z.object({
    pattern: z.string().optional(), // SPARQL pattern
    expression: z.string().optional(), // Logical expression
  }).optional(),
  // Action (what to do)
  action: z.object({
    type: z.enum(['validate', 'infer', 'compute', 'error']),
    target: z.string().optional(), // Target property
    value: z.any().optional(), // Value to set/compute
    message: z.string().optional(), // Error message
  }),
  // Priority
  priority: z.number().optional().default(1),
});

// ============================================================================
// SPARQL (Query Language)
// ============================================================================

// SPARQL Pattern
export const SPARQLPatternSchema = z.object({
  type: z.enum(['triple', 'optional', 'union', 'filter', 'graph']),
  subject: z.string().optional(),
  predicate: z.string().optional(),
  object: z.string().optional(),
  patterns: z.array(z.lazy(() => SPARQLPatternSchema)).optional(), // Nested patterns
  filter: z.string().optional(), // Filter expression
  graph: z.string().optional(), // Named graph
});

// SPARQL Query
export const SPARQLQuerySchema = z.object({
  type: z.enum(['select', 'construct', 'ask', 'describe']),
  // Pattern matching
  where: z.array(SPARQLPatternSchema).optional(),
  // Aggregation
  groupBy: z.array(z.string()).optional(),
  aggregate: z.record(z.object({
    function: z.enum(['count', 'sum', 'avg', 'min', 'max']),
    variable: z.string(),
  })).optional(),
  // Filtering
  filter: z.string().optional(),
  // Ordering
  orderBy: z.array(z.object({
    variable: z.string(),
    direction: z.enum(['asc', 'desc']),
  })).optional(),
  // Limit
  limit: z.number().optional(),
});

// ============================================================================
// Unified Ontological IR
// ============================================================================

export const MalloyOntologicalIRSchema = z.object({
  // OWL - Ontology definitions
  ontology: z.object({
    classes: z.record(OWLClassSchema),
    namespace: z.string().optional(),
    imports: z.array(z.string()).optional(),
  }).optional(),
  
  // SHACL - Shape constraints
  shapes: z.record(SHACLShapeSchema).optional(),
  
  // SPIN - Rules and inferencing
  rules: z.record(SPINRuleSchema).optional(),
  
  // SPARQL - Queries
  sparqlQueries: z.record(SPARQLQuerySchema).optional(),
  
  // Traditional Malloy IR (embedded)
  malloyIR: MalloyIRSchema.optional(),
  
  // Metadata
  metadata: z.object({
    version: z.string().optional(),
    description: z.string().optional(),
    ontologyIRI: z.string().optional(),
  }).optional(),
});

// Export types
export type OWLClass = z.infer<typeof OWLClassSchema>;
export type SHACLShape = z.infer<typeof SHACLShapeSchema>;
export type SPINRule = z.infer<typeof SPINRuleSchema>;
export type SPARQLPattern = z.infer<typeof SPARQLPatternSchema>;
export type SPARQLQuery = z.infer<typeof SPARQLQuerySchema>;
export type MalloyOntologicalIR = z.infer<typeof MalloyOntologicalIRSchema>;

// Helper functions
export function createOntologicalIR(input: {
  ontology?: {
    classes: Record<string, z.infer<typeof OWLClassSchema>>;
  };
  shapes?: Record<string, z.infer<typeof SHACLShapeSchema>>;
  rules?: Record<string, z.infer<typeof SPINRuleSchema>>;
  sparqlQueries?: Record<string, z.infer<typeof SPARQLQuerySchema>>;
  malloyIR?: z.infer<typeof MalloyIRSchema>;
}): MalloyOntologicalIR {
  return MalloyOntologicalIRSchema.parse({
    ontology: input.ontology,
    shapes: input.shapes,
    rules: input.rules,
    sparqlQueries: input.sparqlQueries,
    malloyIR: input.malloyIR,
  });
}

