# Unified IR Design: OpenCypher + SDSL + ML + Ontology

## Overview

**Don't move away from OpenCypher** - Instead, unify OpenCypher IR with SDSL IR and ML IR to create a single coherent ontological language.

## The Three IRs

### 1. OpenCypher IR (Graph/Ontology)
- **NodePattern** - Entities/Classes
- **RelationshipPattern** - Connections/Properties
- **PathPattern** - Traversals/Shapes
- **Expressions** - Conditions/Rules

### 2. SDSL IR (Analytics - Our Own)
- **DataModel** - Data sources
- **MeasureIR** - Aggregations
- **DimensionIR** - Groupings
- **JoinIR** - Relationships

### 3. ML IR (Machine Learning)
- **FeatureIR** - ML features
- **EmbeddingIR** - Vector representations
- **MLModelIR** - Prediction models
- **PipelineIR** - Training/inference pipelines

## Unified IR Structure

```typescript
interface UnifiedOntologyIR {
  // Core structure (OpenCypher IR)
  graph: {
    nodes: NodePattern[]; // Entities/OWL classes
    relationships: RelationshipPattern[]; // OWL properties
    paths: PathPattern[]; // SHACL shapes
    expressions: CypherExpression[]; // SPIN rules
  };
  
  // Analytics layer (SDSL IR)
  analytics: {
    sources: DataModel[]; // Data sources
    measures: MeasureIR[]; // Aggregations
    dimensions: DimensionIR[]; // Groupings
    joins: JoinIR[]; // Analytical relationships
  };
  
  // ML layer (ML IR)
  ml: {
    features: FeatureIR[]; // ML features
    embeddings: EmbeddingIR[]; // Vector embeddings
    models: MLModelIR[]; // ML models
    pipelines: PipelineIR[]; // ML pipelines
  };
  
  // Ontology layer (Unified from above)
  ontology: {
    classes: OWLClass[]; // From NodePattern
    properties: OWLProperty[]; // From RelationshipPattern
    shapes: SHACLShape[]; // From PathPattern
    rules: SPINRule[]; // From Expressions
  };
}
```

## Mapping Strategy

### OpenCypher → Ontology

```typescript
// NodePattern → OWL Class
NodePattern {
  labels: ['Customer', 'LegalEntity']
  properties: { ... }
}
↓
OWL Class {
  name: 'Customer'
  subClassOf: ['LegalEntity']
  properties: { ... }
}

// RelationshipPattern → OWL Property
RelationshipPattern {
  types: ['HAS_INVOICE']
  direction: 'right'
  properties: { cardinality: 'many' }
}
↓
OWL ObjectProperty {
  name: 'has_invoice'
  range: 'Invoice'
  cardinality: { min: 0, max: unbounded }
}

// PathPattern → SHACL Shape
PathPattern {
  nodes: [Customer, Invoice]
  relationships: [HAS_INVOICE]
}
↓
SHACL Shape {
  targetClass: 'Customer'
  property: {
    path: 'has_invoice'
    class: 'Invoice'
    minCount: 0
  }
}
```

### SDSL → Analytics Layer

```typescript
// SourceIR → Analytical Source
SourceIR {
  type: 'table'
  measures: { totalRevenue: sum(...) }
  dimensions: { region: 'region' }
}
↓
Analytics Source {
  name: 'customers'
  aggregations: [totalRevenue]
  groupings: [region]
}
```

### ML → Machine Learning Layer

```typescript
// FeatureIR → ML Feature
FeatureIR {
  field: 'age'
  transform: 'normalize'
}
↓
ML Feature {
  name: 'age_normalized'
  type: 'numeric'
  transform: normalize(age)
}
```

## Adopting OpenCypher Language Meaning

### In ML Extension

**OpenCypher patterns for feature engineering:**
```cypher
// Graph pattern for feature extraction
MATCH (customer:Customer)-[r:HAS_INVOICE]->(invoice:Invoice)
// Extract features from pattern
WITH customer,
     count(r) as invoice_count,
     sum(invoice.amount) as total_revenue,
     collect(invoice.status) as statuses
// Create ML features
RETURN customer.id,
       invoice_count as feature_invoice_count,
       total_revenue as feature_total_revenue,
       size([s IN statuses WHERE s = 'PAID']) as feature_paid_count
```

**SDSL syntax with OpenCypher meaning:**
```typescript
source: Customer {
  // OpenCypher pattern embedded in Malloy
  pattern: (customer:Customer)-[:HAS_INVOICE]->(invoice:Invoice)
  
  // SDSL measures (using pattern)
  measure: invoice_count is count(pattern.relationship)
  measure: total_revenue is sum(pattern.invoice.amount)
  
  // ML features (using pattern + measures)
  feature: invoice_count_feature is normalize(invoice_count)
  feature: revenue_feature is standardize(total_revenue)
}
```

## IR Level Integration

### OpenCypher IR for Ontology Modeling

**Use OpenCypher AST to represent:**

```typescript
// Ontology as OpenCypher IR
const customerOntology = {
  // Classes as NodePatterns
  classes: [
    {
      type: 'NodePattern',
      labels: ['Customer', 'LegalEntity'],
      properties: {
        name: { datatype: 'string', required: true },
        age: { datatype: 'number', min: 0, max: 150 },
      },
    },
  ],
  
  // Properties as RelationshipPatterns
  objectProperties: [
    {
      type: 'RelationshipPattern',
      types: ['HAS_INVOICE'],
      direction: 'right',
      domain: 'Customer',
      range: 'Invoice',
      cardinality: { min: 0, max: unbounded },
    },
  ],
  
  // Shapes as PathPatterns
  shapes: [
    {
      type: 'PathPattern',
      targetClass: 'Customer',
      constraints: [
        { property: 'name', minLength: 2 },
        { property: 'age', min: 18 },
      ],
    },
  ],
};
```

## Benefits of Unified IR

### 1. Complete Semantics
- **Graph structure** - OpenCypher
- **Analytics** - SDSL
- **ML** - Our extensions
- **Ontology** - OWL/SHACL/SPIN mapped to OpenCypher

### 2. Single IR
- **No translation loss** - Direct mapping
- **Unified** - One IR structure
- **Comprehensive** - Covers all domains

### 3. Natural Fit
- **Property graphs = Ontologies** - Direct correspondence
- **Patterns = Shapes** - Natural constraint language
- **Expressions = Rules** - Natural inferencing

### 4. Modern Tooling
- **TypeScript/Rust** - Not Java-bound
- **VSCode** - Modern IDE
- **Composer** - Visual composition
- **No restrictive toolchains** - Free to extend

## Key Insights

1. **Don't move away from OpenCypher** - Unify with SDSL
2. **OpenCypher IR for ontology** - Natural fit
3. **SDSL IR for analytics** - Our own semantic layer
4. **ML IR for machine learning** - Our extensions
5. **Single unified IR** - Complete semantics
6. **Modern tooling** - Avoid Java restrictions

## Next Steps

- [ ] Map OpenCypher IR to OWL/SHACL/SPIN
- [ ] Design unified IR structure
- [ ] Create IR converters
- [ ] Build unified language syntax
- [ ] Implement IR-level integration
- [ ] Document ontology modeling with OpenCypher IR

