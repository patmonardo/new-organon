/**
 * Shape Evaluator
 *
 * Recursively descends into Shape (Active Consciousness moment).
 * 
 * Shape emerges as Concept → Shape Eval = Genus/Species processing.
 * 
 * Following Fichte: Shape is Active Consciousness.
 * The Principle enters into Shape, and we descend into its structure.
 * 
 * Key insight: Shape evaluator must be driven/informed by the Concept itself.
 * 
 * **Without Principle**: Shape.eval is Empty (meaningless - Business Forms)
 * **With Principle**: Shape.Eval = Genus/Species (Determinate Science - meaningful)
 */

import type { FormShape } from '../shape/shape-form';
import type {
  FormPrinciple,
  ShapeEvalResult,
  AspectualDetermination,
  ConsciousnessMoment,
} from './principle-provider';

/**
 * Concept-driven Shape Evaluator
 *
 * Processes Shape as Concept through genus/species relationships.
 * 
 * Shape = Concept
 * - Genus: The general category/type
 * - Species: The specific determination within the genus
 * 
 * Recursively descends into Shape structure:
 * - Signature (moments as species within genus)
 * - Facets (dialectical state as genus/species determinations)
 * - State (active determinations as species)
 */
export class DefaultShapeEvaluator {
  /**
   * Evaluate Shape through recursive descent
   *
   * Receives Shape as Principle and processes it as Concept:
   * - Signature moments (species within genus)
   * - Facets (genus/species determinations)
   * - State (species determinations)
   * 
   * **With Principle**: Shape.Eval = Genus/Species (Determinate Science - meaningful)
   * **Without Principle**: Shape.Eval is Nil (empty, meaningless - Business Forms)
   * 
   * **Architecture**: Shape.Eval is Nil until Absolute Logic (PureFormProcessor) returns a Principle.
   * AbsoluteEssence = PureFormProcessor (Principle discovery).
   * Procedure/ML layers are Reflective - can participate in Principle discovery,
   * but Shape.Eval is still Nil until Principle is returned.
   */
  async evaluate(
    shape: FormShape,
    principle: FormPrinciple,
  ): Promise<ShapeEvalResult> {
    // Process Shape as Concept (genus/species)
    // With Principle: this becomes Determinate Science (meaningful)
    // Without Principle: Shape.Eval is Nil (meaningless)
    const concept = this.extractConcept(shape, principle);
    
    // Descend into Signature (moments as species within genus)
    const consciousnessMoments = this.descendIntoSignature(shape, principle, concept);

    // Descend into Facets (genus/species determinations)
    const facetDeterminations = this.descendIntoFacets(shape, principle, concept);

    // Descend into State (species determinations)
    const stateDeterminations = this.descendIntoState(shape, principle, concept);

    return {
      determinations: [...facetDeterminations, ...stateDeterminations],
      consciousnessMoments,
    };
  }

  /**
   * Extract Concept from Shape
   *
   * Shape emerges as Concept - extract genus/species structure.
   */
  private extractConcept(shape: FormShape, principle: FormPrinciple): {
    genus: string;
    species: string[];
  } {
    // Genus is the type/category
    const genus = shape.type || 'system.Form';
    
    // Species are the moments/specific determinations
    const moments = shape.getMoments();
    const species = moments.map(m => m.name);
    
    return { genus, species };
  }

  /**
   * Recursively descend into Signature
   *
   * Signature contains moments as species within genus.
   * Each moment is a species determination within the Concept.
   */
  private descendIntoSignature(
    shape: FormShape,
    principle: FormPrinciple,
    concept: { genus: string; species: string[] },
  ): ConsciousnessMoment[] {
    const moments = shape.getMoments();
    return moments.map((moment) => ({
      id: `${shape.id}:moment:${moment.name}`,
      moment: moment.name,
      active: true, // Active Consciousness is always active
      // Concept-driven: moment is a species within the genus
      genus: concept.genus,
      species: moment.name,
    }));
  }

  /**
   * Recursively descend into Facets
   *
   * Facets contain genus/species determinations.
   * These are aspectual determinations from Shape as Concept.
   */
  private descendIntoFacets(
    shape: FormShape,
    principle: FormPrinciple,
    concept: { genus: string; species: string[] },
  ): AspectualDetermination[] {
    const facets = shape.facets;
    const determinations: AspectualDetermination[] = [];

    // Descend into each facet (genus/species processing)
    for (const [key, value] of Object.entries(facets)) {
      determinations.push({
        id: `${shape.id}:facet:${key}`,
        aspect: key,
        value,
        // Concept-driven: facet is a genus/species determination
        genus: concept.genus,
      });
    }

    return determinations;
  }

  /**
   * Recursively descend into State
   *
   * State contains species determinations.
   * These are aspectual determinations from Shape as Concept.
   */
  private descendIntoState(
    shape: FormShape,
    principle: FormPrinciple,
    concept: { genus: string; species: string[] },
  ): AspectualDetermination[] {
    const state = shape.state;
    const determinations: AspectualDetermination[] = [];

    // Descend into each state entry (species determinations)
    for (const [key, value] of Object.entries(state)) {
      determinations.push({
        id: `${shape.id}:state:${key}`,
        aspect: key,
        value,
        // Concept-driven: state entry is a species determination
        genus: concept.genus,
        species: key,
      });
    }

    return determinations;
  }
}

