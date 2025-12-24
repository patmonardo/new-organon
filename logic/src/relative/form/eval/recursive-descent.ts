/**
 * Recursive Descent Engine
 *
 * Implements Fichte's Principle Provision with Recursive Descent.
 * 
 * The Principle is Shape:Context:Morph as unity.
 * Relative form/eval recursively descends into all three.
 */

import type {
  FormPrinciple,
  PrincipleReceiver,
  PrincipleDescentResult,
  ShapeEvalResult,
  ContextEvalResult,
  MorphEvalResult,
} from './principle-provider';
import type { FormShape } from '../shape/shape-form';
import type { FormContext } from '../context/context-form';
import type { FormMorph } from '../morph/morph-form';
import { DefaultShapeEvaluator } from './shape-evaluator';
import { DefaultContextEvaluator } from './context-evaluator';
import { DefaultMorphEvaluator } from './morph-evaluator';

/**
 * Recursive Descent Engine
 *
 * Receives Principle from Pure Form (kernel) and recursively descends
 * into Shape → Context → Morph.
 */
export class RecursiveDescentEngine implements PrincipleReceiver {
  private readonly shapeEvaluator: ShapeEvaluator;
  private readonly contextEvaluator: ContextEvaluator;
  private readonly morphEvaluator: MorphEvaluator;

  constructor(
    shapeEvaluator?: ShapeEvaluator,
    contextEvaluator?: ContextEvaluator,
    morphEvaluator?: MorphEvaluator,
  ) {
    this.shapeEvaluator = shapeEvaluator || new DefaultShapeEvaluator();
    this.contextEvaluator = contextEvaluator || new DefaultContextEvaluator();
    this.morphEvaluator = morphEvaluator || new DefaultMorphEvaluator();
  }

  /**
   * Receive Principle and initiate recursive descent
   *
   * Following Fichte: Pure Form provides Principle.
   * The Principle enters into discursive evaluation.
   * Relative form/eval recursively descends into Shape:Context:Morph.
   */
  async receivePrinciple(principle: FormPrinciple): Promise<PrincipleDescentResult> {
    // The Principle is Shape:Context:Morph as unity
    // We must descend into all three while maintaining unity

    // Step 1: Descend into Shape (Active Consciousness)
    const shapeResult = await this.descendIntoShape(principle.shape, principle);

    // Step 2: Descend into Context (Active Determination-of-Reflection)
    const contextResult = await this.descendIntoContext(principle.context, principle);

    // Step 3: Descend into Morph (Active Ground)
    const morphResult = await this.descendIntoMorph(principle.morph, principle);

    return {
      shapeResult,
      contextResult,
      morphResult,
      unity: principle, // Maintain unity throughout descent
    };
  }

  /**
   * Recursively descend into Shape
   *
   * Shape is Active Consciousness moment.
   * Descends into Shape's structure while maintaining Principle unity.
   */
  private async descendIntoShape(
    shape: FormShape,
    principle: FormPrinciple,
  ): Promise<ShapeEvalResult> {
    return this.shapeEvaluator.evaluate(shape, principle);
  }

  /**
   * Recursively descend into Context
   *
   * Context is Active Determination-of-Reflection moment.
   * Descends into Context's presuppositions, scope, conditions while maintaining Principle unity.
   */
  private async descendIntoContext(
    context: FormContext,
    principle: FormPrinciple,
  ): Promise<ContextEvalResult> {
    return this.contextEvaluator.evaluate(context, principle);
  }

  /**
   * Recursively descend into Morph
   *
   * Morph is Active Ground moment.
   * Descends into Morph's patterns/transformations while maintaining Principle unity.
   */
  private async descendIntoMorph(
    morph: FormMorph,
    principle: FormPrinciple,
  ): Promise<MorphEvalResult> {
    return this.morphEvaluator.evaluate(morph, principle);
  }
}

/**
 * Shape Evaluator
 *
 * Evaluates Shape (Active Consciousness) through recursive descent.
 */
export interface ShapeEvaluator {
  evaluate(shape: FormShape, principle: FormPrinciple): Promise<ShapeEvalResult>;
}

/**
 * Context Evaluator
 *
 * Evaluates Context (Active Determination-of-Reflection) through recursive descent.
 */
export interface ContextEvaluator {
  evaluate(context: FormContext, principle: FormPrinciple): Promise<ContextEvalResult>;
}

/**
 * Morph Evaluator
 *
 * Evaluates Morph (Active Ground) through recursive descent.
 */
export interface MorphEvaluator {
  evaluate(morph: FormMorph, principle: FormPrinciple): Promise<MorphEvalResult>;
}

