/**
 * Principle Provider Interface
 *
 * Pure Form (kernel) provides Principle to Relative Form Eval.
 * The Principle is Shape:Context:Morph as unity.
 */

import type { FormShape } from '../shape/shape-form';
import type { FormContext } from '../context/context-form';
import type { FormMorph } from '../morph/morph-form';

/**
 * Principle as Unity
 *
 * Following Fichte: Pure Form provides Principle.
 * The Principle is Relative Form Shape:Context:Morph as a unity.
 */
export interface FormPrinciple {
  /** Shape as Active Consciousness moment */
  shape: FormShape;
  /** Context as Active Determination-of-Reflection moment */
  context: FormContext;
  /** Morph as Active Ground moment */
  morph: FormMorph;
}

/**
 * Principle Provider
 *
 * Interface for receiving Principle from Pure Form (kernel).
 * The Principle enters into discursive evaluation.
 */
export interface PrincipleProvider {
  /**
   * Provide Principle
   *
   * Receives Principle from Pure Form (kernel).
   * The Principle is Shape:Context:Morph as unity.
   */
  providePrinciple(): FormPrinciple;
}

/**
 * Principle Receiver
 *
 * Receives Principle and initiates recursive descent.
 */
export interface PrincipleReceiver {
  /**
   * Receive Principle
   *
   * Receives Principle from Pure Form (kernel).
   * Initiates recursive descent into Shape:Context:Morph.
   */
  receivePrinciple(principle: FormPrinciple): Promise<PrincipleDescentResult>;
}

/**
 * Result of recursive descent into Principle
 */
export interface PrincipleDescentResult {
  /** Shape evaluation result */
  shapeResult: ShapeEvalResult;
  /** Context evaluation result */
  contextResult: ContextEvalResult;
  /** Morph evaluation result */
  morphResult: MorphEvalResult;
  /** Unity maintained through descent */
  unity: FormPrinciple;
}

/**
 * Shape evaluation result
 */
export interface ShapeEvalResult {
  /** Aspectual determinations from Shape */
  determinations: AspectualDetermination[];
  /** Active Consciousness moments */
  consciousnessMoments: ConsciousnessMoment[];
}

/**
 * Context evaluation result
 */
export interface ContextEvalResult {
  /** Reflective determinations from Context */
  determinations: ReflectiveDetermination[];
  /** Presuppositions, scope, conditions */
  reflectionStructure: ReflectionStructure;
}

/**
 * Morph evaluation result
 */
export interface MorphEvalResult {
  /** Grounding determinations from Morph */
  determinations: GroundingDetermination[];
  /** Patterns/transformations */
  groundStructure: GroundStructure;
}

/**
 * Aspectual determination (from Shape)
 * 
 * Concept-driven: Shape = Concept, so determinations have genus/species structure.
 */
export interface AspectualDetermination {
  id: string;
  aspect: string;
  value: unknown;
  /** Genus (from Concept) */
  genus?: string;
  /** Species (from Concept) */
  species?: string;
}

/**
 * Reflective determination (from Context)
 */
export interface ReflectiveDetermination {
  id: string;
  reflection: string;
  value: unknown;
}

/**
 * Grounding determination (from Morph)
 * 
 * Syllogism-driven: Morph = Syllogism, so determinations can invoke GDS procedures.
 */
export interface GroundingDetermination {
  id: string;
  ground: string;
  value: unknown;
  /** Type: 'syllogistic-premise' | 'gds-procedure' */
  type?: string;
  /** Procedure result (if GDS procedure was invoked) */
  procedureResult?: unknown;
}

/**
 * Active Consciousness moment
 * 
 * Concept-driven: moment is a species within the genus (Concept).
 */
export interface ConsciousnessMoment {
  id: string;
  moment: string;
  active: boolean;
  /** Genus (from Concept) */
  genus?: string;
  /** Species (from Concept) */
  species?: string;
}

/**
 * Reflection structure
 */
export interface ReflectionStructure {
  presuppositions: unknown[];
  scope: {
    modal: string;
    domain: string[];
    phase: string;
  };
  conditions: unknown[];
}

/**
 * Ground structure
 */
export interface GroundStructure {
  patterns: string[];
  transformations: unknown[];
}

