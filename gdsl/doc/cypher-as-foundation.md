# Cypher as Foundation: Language-First Architecture

## The Neo4j Pattern

In Neo4j, **Cypher is foundational** - query execution and plans are built **on top of** Cypher, not the other way around.

### Traditional Database Pattern (Bottom-Up)

```
Query Execution Engine
    ↓
Query Planner/Optimizer
    ↓
Query Language (SQL, PL, etc.) ← Added on top
```

The language is an **afterthought** - a syntax layer over the execution engine.

### Neo4j Pattern (Top-Down)

```
Cypher (Language) ← Foundation
    ↓
Cypher AST (IR)
    ↓
Query Planner (works with Cypher patterns)
    ↓
Execution Engine (executes Cypher plans)
```

The language is **foundational** - execution is built to serve the language.

## Why This Matters

1. **Language as First-Class Citizen**: Cypher patterns are the primary abstraction
2. **Execution Follows Language**: The execution engine is designed to execute Cypher, not the reverse
3. **Pattern Matching is Core**: Graph patterns are the fundamental operation, not table scans
4. **Composability**: Cypher queries compose naturally because the language is the foundation

## GDSL's Approach

Looking at `gdsl/src/ast/cypher-ast.ts`:

```typescript
/**
 * OpenCypher AST: Intermediate Representation
 *
 * This module defines the parsed Cypher AST structures that serve as
 * the intermediate language between dialectical schema and Neo4j execution.
 *
 * GDSL uses these ASTs directly—no syntax parsing needed.
 * AI codegens dialectic states → Cypher AST patterns.
 */
```

### GDSL Architecture

```
Dialectic State (GDSL IR)
    ↓
dialecticStateToCypherPattern()
    ↓
Cypher AST (OpenCypher IR)
    ↓
Neo4j Execution
```

**Key Insight**: GDSL generates Cypher AST directly - no syntax parsing. The AST is the **intermediate representation**.

## Implications for GDSL

### 1. Cypher AST as IR

The Cypher AST is not just a query format - it's the **intermediate representation** between:
- Dialectical schema (GDSL's domain model)
- Neo4j execution (the runtime)

### 2. No Syntax Parsing Needed

From the comment: "GDSL uses these ASTs directly—no syntax parsing needed."

This means:
- ✅ AI codegen can produce Cypher AST directly
- ✅ No need for Cypher syntax parsing
- ✅ AST is the canonical form

### 3. Pattern Matching is Primary

Cypher patterns (NodePattern, RelationshipPattern, PathPattern) are the **primary abstraction**:

```typescript
export type CypherPattern =
  | NodePattern
  | RelationshipPattern
  | PathPattern;
```

Execution is built to **match patterns**, not scan tables.

## Comparison: Traditional vs Neo4j Pattern

| Aspect | Traditional (SQL) | Neo4j (Cypher) |
|--------|-------------------|----------------|
| **Foundation** | Execution engine | Query language |
| **IR** | Query plan tree | Cypher AST |
| **Primary Op** | Table scan | Pattern match |
| **Composition** | Query → Plan → Execute | Pattern → Match → Return |
| **Language Role** | Syntax layer | Foundation |

## GDSL's Position

GDSL follows the Neo4j pattern:

1. **Dialectic State** (domain model) → **Cypher AST** (language IR) → **Neo4j** (execution)
2. Cypher AST is the **canonical IR** - not just a query format
3. Pattern matching is the **primary operation**
4. Language-first design - execution serves the language

## Questions for Discussion

1. **Should GDSL have its own execution engine?** Or always translate to Cypher?
2. **Is Cypher AST the right IR?** Or should we have a GDSL-specific IR?
3. **How does this relate to Polars?** Polars is execution-first - how do we bridge?
4. **What about non-Neo4j backends?** Can we translate Cypher AST to other systems?

## Next Steps

- [ ] Document how dialectic states map to Cypher patterns
- [ ] Explore GDSL-specific IR vs Cypher AST
- [ ] Consider execution engine options (Neo4j-only vs multi-backend)
- [ ] Bridge to Polars (execution-first) from Cypher (language-first)

