/**
 * TopicMap for genus.txt - The Genus
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
 * - genus-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const GENUS_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/life/sources/genus.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Genus',
  [
    createTopicMapEntry(
      'genus-1-identity-duplication',
      'Genus as Identity and Duplication; Third Stage',
      [3, 49],
      'Individual cut off from universal concept, presupposition unproven. Through process with world, posited as negative unity of otherness = foundation = actuality of idea. Brings itself forth from actuality (not just concept), coming to be = production. Further determination = genus = identity with hitherto indifferent otherness. Idea = essential identity = particularization. Particularization = disruption = duplication = presupposing objectivity identical with it, relating to itself as another living being. Universal = third stage = truth of life (shut up within itself). Process = individual refers to itself, externality = immanent moment = living totality, certainty of self as subsisting.',
      [
        'individual',
        'universal concept',
        'presupposition',
        'negative unity',
        'foundation',
        'actuality',
        'genus',
        'identity',
        'duplication',
        'third stage',
        'truth of life',
      ],
      { section: 'The Genus', order: 1 }
    ),

    createTopicMapEntry(
      'genus-2-propagation-germ',
      'Propagation and Germ; Reflection into Itself; Higher Form',
      [51, 124],
      'Genus relation = identity of individual self-feeling in another self-subsistent individual = contradiction. Living being = impulse. Genus = completion of idea of life (within immediacy). Universality = actual in singular shape, concept with immediate objectivity. Individual = genus in itself (not for itself), object = another living individual. Identity = inner/subjective, longing to posit identity, realize as universal. Impulse realizes through sublation of singular individualities. Realized identity = negative unity of genus reflecting into itself. Individuality = generated from actual idea (not concept). Concept = germ of living individual. Germ = complete concretion of individuality, all sides/properties/differences contained, immaterial subjective totality undeveloped. Germ = whole living being in inner form of concept. Genus obtains actuality = reflection into itself = propagation. Idea falls back into actuality = repetition/infinite process (within finitude). But also: sublated immediacy, elevated to higher form.',
      [
        'genus relation',
        'contradiction',
        'impulse',
        'completion',
        'immediacy',
        'germ',
        'concretion',
        'individuality',
        'propagation',
        'reflection',
        'higher form',
      ],
      { section: 'The Genus', order: 2 }
    ),

    createTopicMapEntry(
      'genus-3-transition-cognition',
      'Transition to Cognition; Death of Life; Idea of Cognition',
      [126, 156],
      'Process of genus: individuals sublate indifferent/immediate/concrete existence, die away in negative unity. Realized genus = posited as identical with concept. Isolated singularities perish. Negative identity = generation of singularity + sublation = genus rejoining itself = universality explicitly for itself. In copulation, immediacy of living individuality perishes. Death of life = coming to be of spirit. Idea implicit as genus becomes explicit, sublated particularity (living species), reality = simple universality. Idea relates to itself as idea, universal with universality as determinateness/existence = idea of cognition.',
      [
        'process of genus',
        'sublation',
        'negative unity',
        'copulation',
        'death',
        'spirit',
        'idea',
        'universality',
        'idea of cognition',
      ],
      { section: 'The Genus', order: 3 }
    ),
  ],
  {
    sectionDescription: 'The Genus - Genus as identity and duplication, third stage. Propagation and germ, reflection into itself. Transition to cognition, death of life, idea of cognition.',
  }
);

