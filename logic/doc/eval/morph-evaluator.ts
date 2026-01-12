/**
 * Morph Evaluator
 *
 * Recursively descends into Morph (Active Ground moment).
 * 
 * Morph = Syllogism → Morph Eval = Syllogism evaluation.
 * 
 * Following Fichte: Morph is Active Ground.
 * The Principle enters into Morph, and we descend into its structure.
 * 
 * Key insight: Morph is itself a recursive structure.
 * Syllogism Eval can recursively invoke GDS Algos as Procedures.
 * 
 * **Morph as Identities**: Morphs are identities (like `sin^2 + cos^2 = 1`)
 * that need proof. They are syllogisms, not Shapes. The PrincipleProvider
 * gets us Genus/Species (Shape), but we have to build up the Set of Morphs
 * (all trig identities, etc.). **The Set of Morphs is what gets applied to
 * Appearances** - this is the bridge between the Principle/Shape and the
 * actual application to empirical content.
 * 
 * **Morph as ContextualizedShape**: Morph is the point at which we can
 * Apply our Processor to Appearances (Entity/Property/Aspect).
 * 
 * **Morph as Whole of Cognition**: Morph is a type of Whole of Cognition
 * (Shape + Context). It is a Presupposition Chain - we cannot enter into
 * Entity/Property/Aspect unless we have Morph as Whole of Cognition.
 */

import type { FormMorph } from '../morph/morph-form';
import type {
  FormPrinciple,
  MorphEvalResult,
  GroundingDetermination,
  GroundStructure,
} from './principle-provider';

/**
 * Syllogism-driven Morph Evaluator
 *
 * Processes Morph as Syllogism through recursive structure.
 * 
 * Morph = Syllogism
 * - Can recursively invoke GDS Algos as Procedures
 * - Patterns are Active Ground operator chains
 * - Transformations are syllogistic inferences
 * 
 * Recursively descends into Morph structure:
 * - Patterns (syllogistic premises/grounds)
 * - Transformations (syllogistic conclusions/inferences)
 */
export class DefaultMorphEvaluator {
  /**
   * Evaluate Morph through recursive descent
   *
   * Receives Morph as Principle and processes it as Syllogism:
   * - Patterns (syllogistic premises/grounds - can invoke GDS procedures)
   * - Transformations (syllogistic conclusions/inferences)
   * 
   * Morph is itself a recursive structure - syllogism eval can invoke
   * GDS Algos as Procedures recursively.
   * 
   * **Morph as Identities**: Morphs are identities (like `sin^2 + cos^2 = 1`)
   * that need proof. The PrincipleProvider gets us Genus/Species (Shape),
   * but we have to build up the Set of Morphs (all trig identities).
   * 
   * **Morph as ContextualizedShape**: This is the point at which we can
   * Apply our Processor to Appearances (Entity/Property/Aspect).
   * 
   * **Morph as Whole of Cognition**: Morph is a type of Whole of Cognition
   * (Shape + Context). It is the Presupposition for Entity/Property/Aspect.
   */
  async evaluate(
    morph: FormMorph,
    principle: FormPrinciple,
  ): Promise<MorphEvalResult> {
    // Extract ground structure (syllogistic structure)
    const groundStructure = this.extractGroundStructure(morph, principle);

    // Descend into Patterns (syllogistic premises - may invoke GDS procedures)
    const patternDeterminations = await this.descendIntoPatterns(
      morph,
      principle,
      groundStructure,
    );

    // Descend into Transformations (syllogistic conclusions)
    const transformationDeterminations = this.descendIntoTransformations(
      morph,
      principle,
      groundStructure,
    );

    return {
      determinations: [...patternDeterminations, ...transformationDeterminations],
      groundStructure,
    };
  }

  /**
   * Extract Ground Structure from Morph
   */
  private extractGroundStructure(
    morph: FormMorph,
    principle: FormPrinciple,
  ): GroundStructure {
    const signature = morph.signature as any;
    const state = morph.state as any;

    // Patterns come from signature.patterns or state.patterns
    const patterns =
      signature?.patterns || state?.patterns || signature?.morphPatterns || [];

    // Transformations come from facets or state
    const transformations = state?.transformations || morph.facets?.transformations || [];

    return {
      patterns: Array.isArray(patterns) ? patterns : [],
      transformations: Array.isArray(transformations) ? transformations : [],
    };
  }

  /**
   * Recursively descend into Patterns
   *
   * Patterns are syllogistic premises/grounds.
   * Can recursively invoke GDS Algos as Procedures.
   */
  private async descendIntoPatterns(
    morph: FormMorph,
    principle: FormPrinciple,
    groundStructure: GroundStructure,
  ): Promise<GroundingDetermination[]> {
    const determinations: GroundingDetermination[] = [];
    
    for (const [index, pattern] of groundStructure.patterns.entries()) {
      // Pattern may be a GDS procedure invocation
      const isGdsProcedure = this.isGdsProcedure(pattern);
      
      if (isGdsProcedure) {
        // Recursively invoke GDS procedure
        const procedureResult = await this.invokeGdsProcedure(pattern, principle);
        determinations.push({
          id: `${morph.id}:pattern:${index}`,
          ground: 'pattern',
          value: {
            pattern,
            procedureResult,
            type: 'gds-procedure',
          },
        });
      } else {
        // Regular pattern (syllogistic premise)
        determinations.push({
          id: `${morph.id}:pattern:${index}`,
          ground: 'pattern',
          value: pattern,
          type: 'syllogistic-premise',
        });
      }
    }
    
    return determinations;
  }

  /**
   * Check if pattern is a GDS procedure invocation
   */
  private isGdsProcedure(pattern: string): boolean {
    // Heuristic: GDS procedures typically have specific naming patterns
    // This is a placeholder - actual implementation would check against
    // GDS procedure registry or use a more sophisticated pattern matching
    return pattern.startsWith('gds.') || pattern.includes('::');
  }

  /**
   * Invoke GDS procedure recursively
   *
   * Morph eval can invoke GDS Algos as Procedures.
   * This is the recursive structure of Morph.
   */
  private async invokeGdsProcedure(
    pattern: string,
    principle: FormPrinciple,
  ): Promise<unknown> {
    // Placeholder: actual implementation would:
    // 1. Parse pattern to extract procedure name and parameters
    // 2. Look up procedure in GDS procedure registry
    // 3. Invoke procedure with principle context
    // 4. Return procedure result
    
    // For now, return a stub
    return {
      procedure: pattern,
      invoked: true,
      result: 'stub',
    };
  }

  /**
   * Recursively descend into Transformations
   *
   * Transformations are active ground movements.
   */
  private descendIntoTransformations(
    morph: FormMorph,
    principle: FormPrinciple,
    groundStructure: GroundStructure,
  ): GroundingDetermination[] {
    return groundStructure.transformations.map((transformation: any, index: number) => ({
      id: `${morph.id}:transformation:${index}`,
      ground: 'transformation',
      value: transformation,
    }));
  }
}

