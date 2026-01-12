/**
 * Context Evaluator
 *
 * Recursively descends into Context (Active Determination-of-Reflection moment).
 * 
 * Following Fichte: Context is Active Determination-of-Reflection.
 * The Principle enters into Context, and we descend into its structure.
 * 
 * **Context = Judgment** (Subject/Predicate - Determinate Science)
 * 
 * **Science works through Judgment**: Not about completeness (all trig identities),
 * but about knowing what you need (half of them). This is how Science works.
 * 
 * **Context Engineering**: Modern LLM context engineering might be transmitting
 * Principles as Shape.Eval (Genus/Species) and Contexts as Subject/Predicate.
 * That would be what we want in our Context Engineering - Principles as Genus/Species,
 * Contexts as Subject/Predicate, the full Shape:Context:Morph structure.
 */

import type { FormContext } from '../context/context-form';
import type {
  FormPrinciple,
  ContextEvalResult,
  ReflectiveDetermination,
  ReflectionStructure,
} from './principle-provider';

/**
 * Default Context Evaluator
 *
 * Recursively descends into Context structure:
 * - Presuppositions (from state.meta.presuppositions)
 * - Scope (from state.meta.scope)
 * - Conditions (from state.meta.conditions)
 */
export class DefaultContextEvaluator {
  /**
   * Evaluate Context through recursive descent
   *
   * Receives Context as Principle and descends into:
   * - Presuppositions (determinations within Reflection)
   * - Scope (modal/domain/phase)
   * - Conditions (constraints)
   * 
   * **Context = Judgment**: Science works through Judgment - knowing what you need,
   * not knowing everything. The complete Whole of Cognition (all identities) is nice
   * to have, but Science works through judgment - knowing what's necessary.
   * 
   * **Context Engineering**: For us, Context Engineering *is* Context (Subject/Predicate).
   * Modern LLM context engineering might be transmitting Principles as Shape.Eval
   * (Genus/Species) and Contexts as Subject/Predicate. That would be what we want
   * in our Context Engineering - the full Shape:Context:Morph structure.
   */
  async evaluate(
    context: FormContext,
    principle: FormPrinciple,
  ): Promise<ContextEvalResult> {
    // Extract reflection structure from state
    const reflectionStructure = this.extractReflectionStructure(context, principle);

    // Descend into Presuppositions
    const presuppositionDeterminations = this.descendIntoPresuppositions(
      context,
      principle,
      reflectionStructure,
    );

    // Descend into Scope
    const scopeDeterminations = this.descendIntoScope(
      context,
      principle,
      reflectionStructure,
    );

    // Descend into Conditions
    const conditionDeterminations = this.descendIntoConditions(
      context,
      principle,
      reflectionStructure,
    );

    return {
      determinations: [
        ...presuppositionDeterminations,
        ...scopeDeterminations,
        ...conditionDeterminations,
      ],
      reflectionStructure,
    };
  }

  /**
   * Extract Reflection Structure from Context state
   */
  private extractReflectionStructure(
    context: FormContext,
    principle: FormPrinciple,
  ): ReflectionStructure {
    const state = context.state as any;
    const meta = state?.meta || {};

    return {
      presuppositions: meta.presuppositions || [],
      scope: {
        modal: meta.scope?.modal || 'actual',
        domain: meta.scope?.domain || [],
        phase: meta.scope?.phase || 'quality',
      },
      conditions: meta.conditions || [],
    };
  }

  /**
   * Recursively descend into Presuppositions
   *
   * Presuppositions are determinations within Reflection.
   */
  private descendIntoPresuppositions(
    context: FormContext,
    principle: FormPrinciple,
    reflectionStructure: ReflectionStructure,
  ): ReflectiveDetermination[] {
    return reflectionStructure.presuppositions.map((presupposition: any, index: number) => ({
      id: `${context.id}:presupposition:${index}`,
      reflection: 'presupposition',
      value: presupposition,
    }));
  }

  /**
   * Recursively descend into Scope
   *
   * Scope determines the modal/domain/phase of Reflection.
   */
  private descendIntoScope(
    context: FormContext,
    principle: FormPrinciple,
    reflectionStructure: ReflectionStructure,
  ): ReflectiveDetermination[] {
    const determinations: ReflectiveDetermination[] = [];

    // Modal determination
    determinations.push({
      id: `${context.id}:scope:modal`,
      reflection: 'scope.modal',
      value: reflectionStructure.scope.modal,
    });

    // Domain determinations
    for (const [index, domain] of reflectionStructure.scope.domain.entries()) {
      determinations.push({
        id: `${context.id}:scope:domain:${index}`,
        reflection: 'scope.domain',
        value: domain,
      });
    }

    // Phase determination
    determinations.push({
      id: `${context.id}:scope:phase`,
      reflection: 'scope.phase',
      value: reflectionStructure.scope.phase,
    });

    return determinations;
  }

  /**
   * Recursively descend into Conditions
   *
   * Conditions are constraints within Reflection.
   */
  private descendIntoConditions(
    context: FormContext,
    principle: FormPrinciple,
    reflectionStructure: ReflectionStructure,
  ): ReflectiveDetermination[] {
    return reflectionStructure.conditions.map((condition: any, index: number) => ({
      id: `${context.id}:condition:${index}`,
      reflection: 'condition',
      value: condition,
    }));
  }
}

