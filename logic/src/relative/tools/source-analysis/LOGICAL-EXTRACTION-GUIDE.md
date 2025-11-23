# Logical Extraction Guide: What is Logical?

## The Core Question

**What is Logical in Hegel's Science of Logic?**

We want to extract **what is Logical**—the logical structure, relationships, transitions, and operations—and crystallize it into executable form. Logical Operations are the **only entities we are interested in here**.

## What is Logical?

### 1. Logical Structure

**Logical structure** is the systematic organization of concepts:
- How concepts are organized
- How concepts relate to each other
- How concepts contain other concepts
- How concepts form systems

**Examples**:
- `concept.moments = {universality, particularity, singularity}`
- `judgment.structure = {subject, predicate, copula}`
- `syllogism.structure = {major, minor, conclusion}`

### 2. Logical Relationships

**Logical relationships** are how concepts relate to each other:
- Identity: `concept = definition`
- Containment: `concept.contains = {moments}`
- Grounding: `concept.grounds = otherConcept`
- Mediation: `concept.mediates = {A, B}`

**Examples**:
- `universality.contains = {particularity, singularity}`
- `being.grounds = essence`
- `concept.mediates = {subjectivity, objectivity}`

### 3. Logical Transitions

**Logical transitions** are how concepts move to other concepts:
- `being.transitions = essence`
- `essence.transitions = concept`
- `concept.transitions = judgment`

**Pattern**: `concept.transitions = target`

### 4. Logical Operations

**Logical operations** are the operations/transformations happening:
- Negation: `concept = negationOfNegation`
- Sublation: `concept.sublates = otherConcept`
- Determination: `concept.determines = otherConcept`
- Externalization: `concept.externalizes = objectivity`

**Pattern**: `concept.operation = result`

## What is NOT Logical?

### 1. Historical References

**Do NOT extract** historical references unless they show logical structure:
- ❌ "Kant said X" (unless it reveals logical structure)
- ❌ "Fichte argued Y" (unless it shows logical relationships)
- ❌ Historical context (unless it reveals logical operations)

**DO extract** if it reveals logical structure:
- ✅ "Kant's distinction shows logical structure X"
- ✅ "Fichte's argument reveals logical relationship Y"

### 2. Examples

**Do NOT extract** examples unless they illustrate logical operations:
- ❌ "Rose is red" (unless it shows logical structure)
- ❌ "All men are mortal" (unless it illustrates logical operations)
- ❌ Concrete examples (unless they reveal logical relationships)

**DO extract** if they illustrate logical operations:
- ✅ "Rose is red" shows judgment structure
- ✅ "All men are mortal" illustrates syllogistic structure

### 3. Academic Commentary

**Do NOT extract** academic commentary unless it reveals logical relationships:
- ❌ "This is important because..."
- ❌ "Hegel is responding to..."
- ❌ "This connects to..."

**DO extract** if it reveals logical relationships:
- ✅ "This shows logical relationship X"
- ✅ "This reveals logical structure Y"

### 4. Mere Descriptions

**Do NOT extract** mere descriptions unless they encode logical operations:
- ❌ "The concept is simple"
- ❌ "This is complex"
- ❌ "Hegel discusses X"

**DO extract** if they encode logical operations:
- ✅ "The concept is simple yet rich" → `concept.property = {simple, rich}`
- ✅ "This is complex" → `concept.complexity = high` (if it reveals logical structure)

## The Extraction Process

### Step 1: Identify What is Logical

**Ask**:
1. What **logical structure** is being described?
2. What **logical relationships** are being established?
3. What **logical transitions** are occurring?
4. What **logical operations** are happening?

**Ignore**:
- Historical references (unless they show logical structure)
- Examples (unless they illustrate logical operations)
- Academic commentary (unless it reveals logical relationships)
- Mere descriptions (unless they encode logical operations)

### Step 2: Extract Clauses

**Extract** logical structure into clauses:
- **Logical identities**: `concept = definition`
- **Logical properties**: `concept.property = value`
- **Logical operations**: `concept.operation = result`
- **Logical transitions**: `concept.transitions = target`
- **Logical structure**: `concept.contains = {moments}`

### Step 3: Extract Predicates

**Extract** logical properties into predicates:
- Properties that can be checked
- Operations that can be validated
- Relationships that can be verified

### Step 4: Extract Relations

**Extract** logical connections into relations:
- Transitions between concepts
- Connections between concepts
- Structure (containment, grounding, etc.)

## The Goal: Crystallize and Expand

**Crystallize** the logical structure:
- Extract the **logical core** from the chunk
- Make it **explicit and clear**
- Structure it as **clauses, predicates, relations**

**Expand** on the initial nugget:
- Build from the logical core outward
- Connect logical operations to each other
- Build the logical graph structure

## Example: What is Logical vs. What is Not

### Chunk Text
```
The concept is absolute self-identity, which is negation of negation. 
This is the infinite unity of negativity with itself. 
Kant said that concepts are functions of unity. 
Hegel is responding to Kant here. 
Universality is pure self-reference via negativity.
```

### What is Logical (Extract This)
- ✅ `concept = absoluteSelfIdentity`
- ✅ `concept = negationOfNegation`
- ✅ `concept = infiniteUnityOfNegativity`
- ✅ `universality = pureSelfReference`
- ✅ `universality.via = negativity`

### What is NOT Logical (Ignore This)
- ❌ "Kant said that concepts are functions of unity" (historical reference)
- ❌ "Hegel is responding to Kant here" (academic commentary)

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
  ],
  relations: [
    { predicate: 'is', from: 'concept', to: 'absoluteSelfIdentity' },
    { predicate: 'is', from: 'concept', to: 'negationOfNegation' },
    { predicate: 'is', from: 'universality', to: 'pureSelfReference' },
  ],
}
```

## Summary

**Extract what is Logical**:
- Logical structure
- Logical relationships
- Logical transitions
- Logical operations

**Ignore what is NOT Logical**:
- Historical references (unless they show logical structure)
- Examples (unless they illustrate logical operations)
- Academic commentary (unless it reveals logical relationships)
- Mere descriptions (unless they encode logical operations)

**Crystallize and Expand**:
- Extract the logical core
- Make it explicit
- Build from the core outward
- Connect operations to each other

