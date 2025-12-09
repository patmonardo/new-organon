/**
 * @model/sdsl - SDSL Core (Species DSL)
 *
 * Root SDSL Substrate - The System Bus and FCI (Formal Concept Integration)
 *
 * This is the foundational layer where:
 * - Agent-MVC is the Root Synthesis (Universal Speaker of GDSL + Every SDSL)
 * - Form-MVC is the Client of the Form Processor
 * - All UI SDSLs (React, Radix, Malloy) are particular implementations
 *
 * The Agent alone speaks both GDSL (Genera DSL) and Every SDSL (Sarvadharma).
 * The root sdsl/ folder is the System Bus - the integration point for all SDSLs.
 *
 * Standalone BI package - zero dependencies on GDS/GDSL/Logic/Task.
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

// Types and adapters (Universal adapter interface)
export * from './types';
export * from './adapter';
