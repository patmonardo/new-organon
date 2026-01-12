/**
 * @model/sdsl - SDSL Core (Species DSL)
 *
 * Root SDSL Substrate - RealityPipe/RealityFabric (reality-facing substrate)
 *
 * This is the foundational layer where:
 * - Agent-MVC is the Root Synthesis (Universal Speaker of GDSL + Every SDSL)
 * - Form-MVC is the Client of the Form Processor
 * - All UI SDSLs (React, Radix, Malloy) are particular implementations
 *
 * The Agent alone speaks both GDSL (Genera DSL) and Every SDSL (Sarvadharma).
 * The root sdsl/ folder is the integration point for all SDSLs.
 *
 * Note: kernel representational contracts are sourced from @organon/gdsl.
 */

// Core SDSL (semantic modeling)
export {
  defineModel,
  DataModel,
  DataView,
  type DataModelConfig,
  type MeasureDefinition,
  type DimensionDefinition,
  type JoinDefinition,
  type ViewQuery,
  type ViewSpec,
  type FilterDefinition,
  type ViewParameter,
  sum,
  count,
  avg,
  dimension,
} from '../data/sdsl';

// Form MVC Core (Client of Form Processor)
export * from './form-model';
export * from './form-view';
export * from './form-controller';

// Agent MVC Core (Root Synthesis - Universal Speaker)
// The Agent is the Root Meaning of MVC - it speaks GDSL and Every SDSL
export * from './agent-model';
export * from './agent-view';
export * from './agent-controller';
export * from './agent-adapter';

// Fact trace bridge (Logic → Agent Context)
export * from './fact-trace';

// Logical model/view trace (SDSL Model/View → FactTrace)
export * from './logical-model-trace';

// Essence → View → Agent helpers
export * from './essence-to-agent';

// Middleware primitives (event stream → agent outputs)
export * from './agent-bus';
export * from './agent-runtime';

// RealityPipe primitives (internal middleware)
export * from './reality-pipe';

// Terminology (transcendentally located)
export * from './terminology';

// TAW surface (moment of concept)
export * from './taw';
export * from './taw-schema';

// Algorithm-first core
export * from './runtime-algorithms';

// Types and adapters (Universal adapter interface)
export * from './types';
export * from './adapter';
