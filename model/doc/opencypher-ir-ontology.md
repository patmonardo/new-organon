# OpenCypher IR for Ontology Modeling

## What is OpenCypher Really?

**OpenCypher is:**
- **Property Graph Query Language** - Declarative graph querying
- **Pattern Matching Language** - Graph patterns as first-class
- **Open Specification** - Not tied to Neo4j
- **IR-based** - Rich intermediate representation (AST)

**Core concepts:**
- **Nodes** - Entities with labels and properties
- **Relationships** - Directed edges with types and properties
- **Patterns** - Graph pattern matching
- **Path** - Traversal patterns

## OpenCypher vs Malloy

### OpenCypher
- **Graph-focused** - Nodes, relationships, paths
- **Pattern matching** - Primary operation
- **Declarative** - What to find, not how
- **Property graph model** - Rich property support

### Malloy
- **Table-focused** - Sources, measures, dimensions
- **Aggregation** - Primary operation
- **Declarative** - What to compute, not how
- **Relational model** - With semantic layer

### Complementary
- **OpenCypher** - Graph structure, relationships
- **Malloy** - Analytics, aggregations, measures
- **Together** - Complete ontology + analytics

## OpenCypher IR Structure

### AST Components

```typescript
// OpenCypher IR (from our codebase)
interface OpenCypherIR {
  // Query structure
  query: CypherQuery {
    clauses: CypherClause[];
    parameters?: Record<string, unknown>;
  };
  
  // Patterns
  patterns: CypherPattern[];
  // - NodePattern (entities)
  // - RelationshipPattern (connections)
  // - PathPattern (traversals)
  
  // Expressions
  expressions: CypherExpression[];
}

// NodePattern - Entity representation
interface NodePattern {
  type: 'NodePattern';
  variable?: string;
  labels: string[]; // OWL classes
  properties?: Record<string, CypherExpression>; // Property values
}

// RelationshipPattern - Relationship representation
interface RelationshipPattern {
  type: 'RelationshipPattern';
  variable?: string;
  direction: 'left' | 'right' | 'both';
  types: string[]; // Relationship types
  properties?: Record<string, CypherExpression>;
}
```

## Using OpenCypher IR for Ontology

### Node Patterns as OWL Classes

```typescript
// OWL Class represented as OpenCypher NodePattern
const customerClass = {
  type: 'NodePattern',
  variable: 'customer',
  labels: ['Customer', 'LegalEntity'], // OWL class hierarchy
  properties: {
    name: { type: 'Literal', value: 'required' },
    age: { type: 'Literal', value: 'number' },
  },
};
```

### Relationship Patterns as OWL Properties

```typescript
// OWL Object Property as OpenCypher RelationshipPattern
const hasInvoiceProperty = {
  type: 'RelationshipPattern',
  variable: 'has_invoice',
  direction: 'right',
  types: ['HAS_INVOICE'], // OWL object property
  properties: {
    cardinality: { type: 'Literal', value: 'many' },
    required: { type: 'Literal', value: false },
  },
};
```

### Path Patterns as SHACL Shapes

```typescript
// SHACL Shape as OpenCypher PathPattern
const customerShape = {
  type: 'PathPattern',
  nodes: [
    { labels: ['Customer'], variable: 'c' },
    { labels: ['Invoice'], variable: 'i' },
  ],
  relationships: [
    {
      types: ['HAS_INVOICE'],
      direction: 'right',
      properties: {
        minCount: { type: 'Literal', value: 0 },
      },
    },
  ],
};
```

## Unified IR: OpenCypher + Malloy + Ontology

```typescript
interface UnifiedOntologyIR {
  // OpenCypher - Graph structure
  cypher: {
    nodes: NodePattern[]; // OWL classes
    relationships: RelationshipPattern[]; // OWL properties
    paths: PathPattern[]; // SHACL shapes
  };
  
  // Malloy - Analytics
  malloy: {
    sources: SourceIR[]; // Data sources
    measures: MeasureIR[]; // Aggregations
    dimensions: DimensionIR[]; // Groupings
  };
  
  // Ontology - Semantic structure
  ontology: {
    classes: OWLClass[]; // From OpenCypher nodes
    properties: OWLProperty[]; // From OpenCypher relationships
    shapes: SHACLShape[]; // From OpenCypher paths
    rules: SPINRule[]; // Inferencing rules
  };
  
  // ML - Machine learning
  ml: {
    features: FeatureIR[];
    embeddings: EmbeddingIR[];
    models: MLModelIR[];
  };
}
```

## OpenCypher Language Meaning in ML

### Graph Patterns for ML

```cypher
// OpenCypher pattern for ML training data
MATCH (customer:Customer)-[r:HAS_INVOICE]->(invoice:Invoice)
WHERE customer.total_revenue > 10000
WITH customer, 
     collect(invoice) as invoices,
     // ML feature extraction
     customer.age / 100.0 as age_normalized,
     size([i IN invoices WHERE i.status = 'PAID']) as paid_count
// Train ML model
CALL ml.train.classification({
  features: [age_normalized, paid_count],
  target: customer.churned,
  model: 'churn_prediction'
})
YIELD model, metrics
RETURN model, metrics
```

### Pattern Matching for Feature Engineering

```cypher
// Use OpenCypher patterns to extract ML features
MATCH (customer:Customer)-[:HAS_INVOICE]->(invoice:Invoice)
WITH customer,
     // Statistical features (Malloy-style)
     sum(invoice.amount) as total_revenue,
     avg(invoice.amount) as avg_invoice,
     // Graph features (OpenCypher-style)
     count(invoice) as invoice_count,
     collect(invoice.status) as invoice_statuses
// Create feature vector
CREATE (customer)-[:HAS_FEATURES]->(features:FeatureVector {
  total_revenue: total_revenue,
  avg_invoice: avg_invoice,
  invoice_count: invoice_count
})
```

## Adoption Strategy

### Don't Move Away from OpenCypher

**Instead, unify:**
- **OpenCypher** - Graph structure, pattern matching
- **Malloy** - Analytics, measures, dimensions
- **Together** - Complete language

### Use OpenCypher IR for Ontology

**OpenCypher IR provides:**
- **Node patterns** → OWL classes
- **Relationship patterns** → OWL properties
- **Path patterns** → SHACL shapes
- **Expressions** → SPIN rules

**This is natural:**
- Property graphs ARE ontologies
- Patterns ARE shape definitions
- Traversals ARE inferencing

### Malloy for Analytics + ML

**Malloy provides:**
- **Measures** → Statistical features
- **Dimensions** → Categorical features
- **Joins** → Feature combinations
- **Extended** → ML features, embeddings, models

## Unified Language Design

### Single Coherent Language

```typescript
// Unified syntax combining OpenCypher + Malloy + ML
source: Customer is Node {
  // OpenCypher - Graph structure
  labels: ['Customer', 'LegalEntity']
  
  // OWL - Ontology
  subClassOf: LegalEntity
  
  // Properties with SHACL constraints
  property: name {
    type: string
    required: true
    pattern: "^[A-Za-z ]+$"
  }
  
  // Malloy - Analytics
  measure: total_revenue is sum(invoices.amount)
  dimension: region is region
  
  // ML - Features
  feature: age_normalized is normalize(age)
  embedding: customer_embedding is embed([name, region])
  
  // OpenCypher - Relationships
  relationship: HAS_INVOICE {
    direction: outgoing
    targetClass: Invoice
    cardinality: { min: 0, max: unbounded }
  }
  
  // SPIN - Rules
  rule: must_be_adult {
    when: age < 18
    then: error("Customer must be 18 or older")
  }
}

// Unified query (OpenCypher + Malloy + ML)
query: comprehensive_analysis is {
  // OpenCypher pattern
  match: (customer:Customer)-[:HAS_INVOICE]->(invoice:Invoice)
  
  // Malloy aggregation
  group_by: customer.region
  aggregate: {
    total_revenue: sum(invoice.amount),
    customer_count: count(customer)
  }
  
  // ML prediction
  predict: churn using churn_prediction
  
  // SPARQL-style filtering
  filter: {
    total_revenue > 10000
    age >= 18
  }
  
  return: {
    region,
    total_revenue,
    customer_count,
    churn_prediction
  }
}
```

## OpenCypher IR as Ontology Foundation

### Why It Works

**Property graphs ARE ontologies:**
- **Nodes** = Entities/Classes (OWL)
- **Labels** = Class membership (OWL)
- **Properties** = Data properties (OWL)
- **Relationships** = Object properties (OWL)
- **Patterns** = Shape definitions (SHACL)
- **Traversals** = Inferencing (SPIN)

**OpenCypher IR provides:**
- **Rich structure** - Comprehensive graph representation
- **Pattern matching** - Natural for ontology queries
- **Path expressions** - Natural for inferencing
- **Property support** - Rich property model

### Mapping

| Semantic Web | OpenCypher IR | Malloy Extension |
|--------------|---------------|------------------|
| OWL Class | NodePattern (labels) | source with class |
| OWL Property | RelationshipPattern | join/relationship |
| SHACL Shape | PathPattern | shape constraints |
| SPIN Rule | WHERE clause + Action | rule definitions |
| SPARQL Query | MATCH...RETURN | query with match |

## Complete Malloy Extension

```
Malloy (Extended)
    ├─→ Analytics (original)
    │   ├─→ Sources, Measures, Dimensions
    │   └─→ Queries, Aggregations
    │
    ├─→ ML Extensions
    │   ├─→ Features, Embeddings, Models
    │   └─→ Predict, Train, Embed
    │
    ├─→ Ontological Extensions (OpenCypher IR)
    │   ├─→ OWL (Classes via NodePattern)
    │   ├─→ SHACL (Shapes via PathPattern)
    │   ├─→ SPIN (Rules via Expressions)
    │   └─→ SPARQL (Queries via MATCH)
    │
    └─→ IR Unification
        ├─→ OpenCypher IR (Graph/Ontology)
        ├─→ Malloy IR (Analytics)
        └─→ ML IR (Machine Learning)
```

## Benefits of Using OpenCypher IR

### 1. Natural Fit
- **Property graphs = Ontologies** - Direct mapping
- **Patterns = Shapes** - Natural constraint language
- **Traversals = Inferencing** - Natural rule language

### 2. Existing IR
- **Well-defined** - OpenCypher AST is comprehensive
- **Standard** - Open specification
- **Proven** - Used in production (Neo4j)

### 3. Unification
- **Graph structure** - OpenCypher
- **Analytics** - Malloy
- **ML** - Our extensions
- **Single IR** - All three unified

### 4. Don't Move Away
- **Keep OpenCypher** - For graph/ontology
- **Add Malloy** - For analytics
- **Extend to ML** - For machine learning
- **Unified** - Single coherent language

## Key Insights

1. **Don't move away from OpenCypher** - Unify with Malloy instead
2. **OpenCypher IR for Ontology** - Natural fit for OWL/SHACL/SPIN
3. **Malloy IR for Analytics** - Measures, dimensions, joins
4. **ML IR for ML** - Features, embeddings, models
5. **Single unified IR** - OpenCypher + Malloy + ML

## Next Steps

- [ ] Map OpenCypher IR to OWL/SHACL/SPIN
- [ ] Design unified IR (OpenCypher + Malloy + ML)
- [ ] Create converters between IRs
- [ ] Build unified language syntax
- [ ] Document IR unification strategy

