# Logical Operation Extraction Template

## Quick Reference: Extraction Checklist

When extracting a Logical Operation from a Chunk, ask:

### 1. What Logical Operation is Happening?
- [ ] What is the **logical operation** being described?
- [ ] What **logical transformation** is occurring?
- [ ] What **logical relationship** is being established?

### 2. Extract Clauses
- [ ] **Logical identities**: `concept = definition`
- [ ] **Logical properties**: `concept.property = value`
- [ ] **Logical operations**: `concept.operation = result`
- [ ] **Logical transitions**: `concept.transitions = target`
- [ ] **Logical structure**: `concept.contains = {moments}`

### 3. Extract Predicates
- [ ] **Logical properties** that can be checked
- [ ] **Logical operations** that can be validated
- [ ] **Logical relationships** that can be verified

### 4. Extract Relations
- [ ] **Logical transitions**: `{ predicate: 'transitions', from: 'A', to: 'B' }`
- [ ] **Logical connections**: `{ predicate: 'contains', from: 'container', to: 'contained' }`
- [ ] **Logical grounding**: `{ predicate: 'grounds', from: 'ground', to: 'grounded' }`

## Template Structure

```typescript
{
  id: '{section}-op-{number}-{key-concept}',
  chunkId: '{section}-{number}-{key-concept}',
  label: '{Title from TopicMapEntry.title}',
  clauses: [
    // Extract logical structure here
    // Focus on: identities, properties, operations, transitions, structure
  ],
  predicates: [
    // Extract logical properties/operations here
    // Focus on: checkable, validatable logical properties
  ],
  relations: [
    // Extract logical connections here
    // Focus on: transitions, connections, structure
  ],
}
```

## Common Patterns

### Pattern 1: Concept Definition
```typescript
clauses: [
  'concept = definition',
  'concept.property = value',
]
predicates: [
  { name: 'IsX', args: ['concept'] },
]
relations: [
  { predicate: 'is', from: 'concept', to: 'definition' },
]
```

### Pattern 2: Transition
```typescript
clauses: [
  'concept.transitions = target',
  'transition.mechanism = mechanism',
]
predicates: [
  { name: 'TransitionsTo', args: ['concept', 'target'] },
]
relations: [
  { predicate: 'transitions', from: 'concept', to: 'target' },
]
```

### Pattern 3: Containment/Structure
```typescript
clauses: [
  'concept.contains = {moment1, moment2}',
  'concept.moments = {universality, particularity, singularity}',
]
predicates: [
  { name: 'HasMoments', args: ['concept', 'moments'] },
]
relations: [
  { predicate: 'contains', from: 'concept', to: 'moment1' },
  { predicate: 'contains', from: 'concept', to: 'moment2' },
]
```

### Pattern 4: Logical Operation
```typescript
clauses: [
  'concept.operation = result',
  'operation.input = input',
  'operation.output = output',
]
predicates: [
  { name: 'PerformsOperation', args: ['concept', 'operation'] },
]
relations: [
  { predicate: 'performs', from: 'concept', to: 'operation' },
  { predicate: 'produces', from: 'operation', to: 'result' },
]
```

## What NOT to Extract

**Do NOT extract**:
- ❌ Historical references (unless they show logical structure)
- ❌ Examples (unless they illustrate logical operations)
- ❌ Academic commentary (unless it reveals logical relationships)
- ❌ Mere descriptions (unless they encode logical operations)
- ❌ Non-logical content (unless it reveals logical structure)

**DO extract**:
- ✅ Logical structure
- ✅ Logical relationships
- ✅ Logical transitions
- ✅ Logical operations
- ✅ Logical properties

## Example: Complete Extraction

### Chunk
```
The concept is absolute self-identity, which is negation of negation. 
This is the infinite unity of negativity with itself. 
Universality is pure self-reference via negativity.
```

### Extracted Logical Operation

```typescript
{
  id: 'concept-op-1-self-identity',
  chunkId: 'concept-1-self-identity',
  label: 'Concept as Absolute Self-Identity: Negation of Negation',
  clauses: [
    'concept = absoluteSelfIdentity',
    'concept = negationOfNegation',
    'concept = infiniteUnityOfNegativity',
    'universality = pureSelfReference',
    'universality.via = negativity',
  ],
  predicates: [
    { name: 'IsAbsoluteSelfIdentity', args: ['concept'] },
    { name: 'IsNegationOfNegation', args: ['concept'] },
    { name: 'IsInfiniteUnityOfNegativity', args: ['concept'] },
    { name: 'IsPureSelfReference', args: ['universality'] },
  ],
  relations: [
    { predicate: 'is', from: 'concept', to: 'absoluteSelfIdentity' },
    { predicate: 'is', from: 'concept', to: 'negationOfNegation' },
    { predicate: 'is', from: 'universality', to: 'pureSelfReference' },
    { predicate: 'via', from: 'universality', to: 'negativity' },
  ],
}
```

## Next Steps

1. **Extract** the logical structure from the chunk
2. **Crystallize** it into clauses, predicates, relations
3. **Expand** on the initial nugget in subsequent operations
4. **Build** the logical graph structure through relations

