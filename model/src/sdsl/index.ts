/**
 * MVC SDSL - Special Data Science Language for @model
 *
 * The MVC SDSL is a Client of the @logic Form Processor.
 * The FactStore is the First Speaker - this SDSL receives and transforms.
 *
 * Architecture:
 *
 *   @logic/FactStore (First Speaker)
 *         │
 *         │ speaks FormShape
 *         ▼
 *   Form Processor (Host)
 *         │
 *         │ runs
 *         ▼
 *   MVC SDSL (Client)
 *         │
 *         ├── FormModel (State:Structure)
 *         ├── FormView (Representation:Perspective)
 *         └── FormController (Action:Rule)
 *                   │
 *                   │ outputs
 *                   ▼
 *             Generic Display Language
 *                   │
 *                   │ adapted by
 *                   ▼
 *             Runtime Adapters (React, Nest, CLI, etc.)
 */

// Core types
export * from './types';

// Base classes
export * from './form-model';
export * from './form-view';
export * from './form-controller';

// Adapter interface
export * from './adapter';

// React adapter (Tailwind components)
export * from './react-adapter';

// React MVC dyad (View + Controller for React/Next.js)
export * from './react-view';
export * from './react-controller';

// =============================================================================
// AGENT MVC - FactStore Client SDK for Agents
// =============================================================================
// The Agent MVC is the agent-facing interface to FactStore.
// While Form MVC renders for humans (DisplayDocument → React),
// Agent MVC renders for agents (ContextDocument → Prompts/Functions/Graphs).
//
// Pipeline: GraphStore → FactStore → KnowledgeStore
//                            ↑
//                     Agent MVC (here)
// =============================================================================

// Agent Model (relevance, provenance, confidence overlays)
export * from './agent-model';

// Agent View (ContextDocument - agent's display language)
export * from './agent-view';

// Agent Controller (query, infer, assert, retract, hypothesize)
export * from './agent-controller';

// Agent Adapter (prompt, function call, GraphQL, RDF, JSON-LD)
export * from './agent-adapter';
