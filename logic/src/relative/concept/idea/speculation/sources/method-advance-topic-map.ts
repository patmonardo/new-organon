/**
 * TopicMap for method-advance.txt - The Advance
 *
 * SOURCE ANALYSIS PHASE 1: Topics
 *
 * COGNITIVE SCIENCE: This is where the real cognitive work happens.
 * The skill in producing good chunks and topics is what makes everything else meaningful.
 * The TopicMap helps check and improve understanding of Hegel through step-by-step analysis.
 *
 * Architecture:
 *    Source Text → [Source Analysis: Cognitive Science] → Chunks + Topics
 *                                                              ↓
 *                    [Logical Op Generation: IR Translation] → Logical Operations (IR)
 *                                                              ↓
 *                    [Codegen: Backend] → Executable Code
 *
 * This TopicMap provides the structured plan for chunking the source text
 * into meaningful chunks. Good chunking/topic analysis makes Logical Operations meaningful
 * (not just jargon) and enables executable codegen (the backend).
 *
 * Each entry maps to:
 * - TopicMapEntry.id → Chunk.id
 * - TopicMapEntry.title → Chunk.title AND LogicalOperation.label (the "Title")
 * - TopicMapEntry.lineRange → Extract text → Chunk.text
 *
 * Reference:
 * - method-advance-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const METHOD_ADVANCE_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/speculation/sources/method-advance.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Advance',
  [
    createTopicMapEntry(
      'method-advance-1-analytic-synthetic',
      'Analytic and Synthetic Method — Dialectical Moment',
      [1, 99],
      'Concrete totality = beginning of advance. Differentiated in itself, but immediacy = diversity. Self-referring universality = unity of diversity. Reflection = first stage = non-indifference/judgment/determining. Method finds determination of universal within it. Absolute method = immanent principle/soul, takes determinate from subject matter. Method = analytic (finds determinations in universal) + synthetic (universal proves to be other). Connection in diversity = within concept. Dialectical moment = judgment through which universal determines itself as other. Dialectic = necessary to reason.',
      [
        'concrete totality',
        'advance',
        'diversity',
        'self-referring universality',
        'reflection',
        'judgment',
        'absolute method',
        'analytic',
        'synthetic',
        'dialectical moment',
        'dialectic',
      ],
      { section: 'The Advance', order: 1 }
    ),

    createTopicMapEntry(
      'method-advance-2-dialectic-history',
      'Dialectic — History, Forms, and Prejudice',
      [100, 225],
      'Dialectic form: opposite determinations shown to accrue to subject matter. Eleatics, Plato, skepticism: dialectic against motion, notions, concepts. Conclusion = contradiction/nullity (objective or subjective sense). Prejudice: dialectic = only negative result. Determinations = true subject matter/content of reason. Determinations prove dialectical in themselves. Universal prius = other of itself. Immediate = mediated, universal = particular. Second universal = negative of first = first negative. Other = not empty negative, contains first, first preserved. Hold fast to positive in negative = important factor. Determinations in themselves = concept, succumb to dialectic.',
      [
        'dialectic',
        'contradiction',
        'nullity',
        'determinations',
        'universal prius',
        'immediate',
        'mediated',
        'negative',
        'positive',
        'concept',
      ],
      { section: 'The Advance', order: 2 }
    ),

    createTopicMapEntry(
      'method-advance-3-negativity-triplicity',
      'Negativity and Triplicity — The Third as Truth',
      [226, 410],
      'Second = truth of first, unity expressed. Second = mediates, reference/relation, includes positive, contradiction. Dialectical moment = positing difference (first) or unity (second). Negativity = turning point, negative self-reference, innermost source, dialectical soul. Second negative = sublation of contradiction, innermost moment of spirit. Third = immediate through sublation of mediation, concept realized through otherness, truth. Result = singular/concrete/subject, universal posited in subject. Triplicity/quadruplicity: third = unity of first/second, immediate/mediated. Syllogism/threefold = universal form of reason. Formalism seized triplicity, but inner worth remains.',
      [
        'negativity',
        'turning point',
        'dialectical soul',
        'second negative',
        'third',
        'truth',
        'triplicity',
        'syllogism',
        'universal form of reason',
      ],
      { section: 'The Advance', order: 3 }
    ),

    createTopicMapEntry(
      'method-advance-4-result-beginning',
      'Result as New Beginning — System Expansion',
      [411, 560],
      'Third = concept realized through otherness, truth, immediacy/mediation. Result = whole withdrawn into itself, form of immediacy, universal. Result can again be beginning. Method = analytic + synthetic, remains same on new foundation. Content enters circle of consideration, method expands into system. Beginning = indeterminate, method = formal soul. Subject matter receives determinateness = content. Negativity withdrawn = sublated form = simple determinateness.',
      [
        'third',
        'result',
        'truth',
        'beginning',
        'method',
        'analytic',
        'synthetic',
        'system',
        'content',
        'determinateness',
      ],
      { section: 'The Advance', order: 4 }
    ),

    createTopicMapEntry(
      'method-advance-5-infinite-progression',
      'Infinite Progression and Method\'s Self-Mediation',
      [561, 710],
      'Determinateness = proximate truth of beginning, denounces incompleteness. Demand: beginning = mediated/deduced (not immediate). Appears as infinite retrogression/progression. Infinite progression = reflection void of concept. Absolute method cannot lead into infinite progression. Logical beginnings (being, essence, universality) = immediate/indeterminate. Indeterminacy = determinateness (negativity, sublated mediation). Method = absolute form, concept knows itself, no content stands out. Determinateness = self-mediation, converts immediate to mediated beginning. Method runs course through content, back to beginning, restoration. Accomplishes as system of totality.',
      [
        'determinateness',
        'infinite progression',
        'absolute method',
        'logical beginnings',
        'self-mediation',
        'system of totality',
      ],
      { section: 'The Advance', order: 5 }
    ),

    createTopicMapEntry(
      'method-advance-6-circle-circles',
      'Forward Movement and Circle of Circles',
      [711, 735],
      'Result = new beginning (form of simplicity). Cognition rolls onwards from content to content. Forward movement: simple determinacies → richer/concrete. Universal = foundation, advance not flowing from other to other. Concept maintains itself in otherness, universal in particularization. Universal elevates preceding content, carries all gained, enriched/compressed. Expansion = moment of content, first premise. Enrichment = necessity of concept, reflection into itself. Extension/intensity: greater extension = denser intensity. Richest = most concrete/subjective. Pure personality = highest/intense point, embraces everything. Each step = getting away from beginning + getting back closer. Retrogressive grounding = progressive determination. Method coils in circle, circle of circles. Logic returned to simple unity = beginning. Method = pure concept relating to itself = being = fulfilled concept = concrete intensive totality.',
      [
        'forward movement',
        'universal',
        'concept',
        'otherness',
        'particularization',
        'expansion',
        'enrichment',
        'pure personality',
        'circle',
        'circle of circles',
        'logic',
        'method',
        'fulfilled concept',
      ],
      { section: 'The Advance', order: 6 }
    ),
  ]
);

