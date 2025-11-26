/**
 * TopicMap for living-individual.txt - The Living Individual
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
 * - living-individual-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const LIVING_INDIVIDUAL_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/life/sources/living-individual.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Living Individual',
  [
    createTopicMapEntry(
      'living-individual-1-creative-presupposing',
      'Creative Presupposing; Diremption into Judgment and Syllogism',
      [3, 74],
      'Universal life = immediate idea, concept with corresponding objectivity. Concept = negative unity of externality. Infinite reference = self-determining, diremption into subjective singularity and indifferent universality. Creative universal soul. Self-determination = creative presupposing. Universal life particularized, splits into judgment extremes = syllogism. Objectivity = predicate of judgment, moment of concept.',
      [
        'universal life',
        'immediate idea',
        'concept',
        'negative unity',
        'diremption',
        'creative presupposing',
        'judgment',
        'syllogism',
        'objectivity',
        'predicate',
      ],
      { section: 'The Living Individual', order: 1 }
    ),

    createTopicMapEntry(
      'living-individual-2-organism',
      'The Living Individual and Organism; Members and Impulse',
      [76, 160],
      'Objectivity = totality of concept, subjectivity = negative unity = true centrality = concept\'s free unity. Subject = idea as singularity = living individual. Individual = soul = self-moving principle. Soul has objective being = corporeity = immediate means. Organism = means/instrument of purpose, concept = substance, accomplished purpose. Organism = manifold of members (not parts). Members exist only in individuality, separable but revert to mechanism/chemism when separated. Individuality = impulse to posit concrete difference, each moment produces itself, raises particularity to universality.',
      [
        'objectivity',
        'subjectivity',
        'negative unity',
        'living individual',
        'soul',
        'corporeity',
        'organism',
        'members',
        'impulse',
        'particularity',
        'universality',
      ],
      { section: 'The Living Individual', order: 2 }
    ),

    createTopicMapEntry(
      'living-individual-3-internal-process',
      'Internal Process; Premise as Conclusion; Concept Produces Itself',
      [162, 204],
      'Process restricted to individuality. First premise = premise also conclusion. Immediate reference = negative unity of concept. Purpose realizes itself = subjective power, externality self-dissolves, returns to negative unity. Unrest/mutability = manifestation of concept. Concept = negativity, objectivity shows self-sublating. Concept produces itself through impulse, product = producing factor = process of production.',
      [
        'process',
        'premise',
        'conclusion',
        'negative unity',
        'purpose',
        'externality',
        'concept',
        'impulse',
        'product',
        'production',
      ],
      { section: 'The Living Individual', order: 3 }
    ),

    createTopicMapEntry(
      'living-individual-4-sensibility-irritability-reproduction',
      'Sensibility, Irritability, Reproduction; Three Determinations',
      [206, 331],
      'Living objectivity = ensouled by concept, determinations = universality/particularity/singularity. Universality = sensibility = internal pulsating, infinite determinable receptivity, reflected into itself, external existence of inward soul. Particularity = irritability = posited difference, opening negativity, impulse, self-determination = judgment. Singularity = reproduction = immanent reflection sublating immediacy, theoretical/real reflection, concrete/vital, totality of all moments. Reproduction = actual individuality, self-referring, outward reference, passes over to presupposed objectivity.',
      [
        'sensibility',
        'irritability',
        'reproduction',
        'universality',
        'particularity',
        'singularity',
        'immanent reflection',
        'totality',
        'actual individuality',
      ],
      { section: 'The Living Individual', order: 4 }
    ),
  ],
  {
    sectionDescription: 'The Living Individual - Creative presupposing, diremption into judgment. Living individual as organism with members. Internal process (premise as conclusion). Sensibility, irritability, reproduction as three determinations.',
  }
);

