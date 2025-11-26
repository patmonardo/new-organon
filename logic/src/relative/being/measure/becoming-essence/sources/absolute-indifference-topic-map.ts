/**
 * TopicMap for absolute-indifference.txt - Absolute Indifference
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
 * - absolute-indifference-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const ABSOLUTE_INDIFFERENCE_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/being/measure/becoming-essence/sources/absolute-indifference.txt',
  'Hegel\'s Science of Logic - Measure',
  'Absolute Indifference',
  [
    createTopicMapEntry(
      'be-a-1-being-abstract-indifference',
      'Being as abstract indifference — indifferentness',
      [3, 12],
      'Being = abstract indifference. Abstract expression = "indifferentness" (no determinateness yet). Pure quantity = indifference open to any determinations (external, no link originating in it).',
      [
        'being',
        'abstract indifference',
        'trait',
        'thought by itself',
        'being',
        'abstract expression',
        'indifferentness',
        'not supposed',
        'as yet',
        'any kind',
        'determinateness',
        'pure quantity',
        'indifference',
        'sense',
        'open',
        'any determinations',
        'provided',
        'external',
        'quantity itself',
        'does not have',
        'link',
        'originating',
        'it',
      ],
      { section: 'Absolute Indifference', order: 1 }
    ),

    createTopicMapEntry(
      'be-a-2-absolute-indifference',
      'Absolute indifference — mediated unity',
      [13, 20],
      'Absolute indifference = through negation of every determinateness (being, quality, quantity, measure), mediates itself with itself = simple unity. Determinateness = state (qualitative, external), has indifference as substrate.',
      [
        'indifference',
        'called absolute',
        'however',
        'through',
        'negation',
        'every determinateness',
        'being',
        'quality',
        'quantity',
        'at first immediate unity',
        'measure',
        'mediates itself',
        'with itself',
        'form',
        'simple unity',
        'determinateness',
        'in it',
        'still only',
        'state',
        'something qualitative',
        'external',
        'has',
        'indifference',
        'substrate',
      ],
      { section: 'Absolute Indifference', order: 2 }
    ),

    createTopicMapEntry(
      'be-a-3-vanishing-determinateness',
      'Vanishing determinateness — empty differentiation',
      [22, 43],
      'Determined as qualitative/external = vanishing something. Qualitative sphere = opposite of itself, sublating itself. Determinateness = empty differentiation in substrate. Empty differentiation = indifference as result. Indifference = concrete, self-mediated through negation of all determinations. Contains negation and relation. State = differentiation immanent and self-referring. Externality and vanishing make unity into indifference. Inside indifference, ceases to be only substrate/abstract.',
      [
        'that which',
        'determined',
        'qualitative',
        'external',
        'only',
        'vanishing something',
        'external',
        'respect',
        'being',
        'qualitative sphere',
        'opposite',
        'itself',
        'as such',
        'sublating',
        'itself',
        'way',
        'determinateness',
        'still',
        'only posited',
        'substrate',
        'empty differentiation',
        'precisely',
        'empty differentiation',
        'indifference',
        'itself',
        'result',
        'indifference',
        'indeed',
        'concrete',
        'sense',
        'self-mediated',
        'through',
        'negation',
        'all',
        'determinations',
        'being',
        'mediation',
        'contains',
        'negation',
        'relation',
        'called',
        'state',
        'differentiation',
        'immanent',
        'self-referring',
        'externality',
        'vanishing',
        'make',
        'unity',
        'being',
        'indifference',
        'consequently',
        'inside',
        'indifference',
        'thereby',
        'ceases',
        'only',
        'substrate',
        'within',
        'abstract',
      ],
      { section: 'Absolute Indifference', order: 3 }
    ),
  ],
  {
    sectionDescription: 'Absolute Indifference - Being as abstract indifference (indifferentness, pure quantity). Absolute indifference (mediated unity through negation of all determinations). Vanishing determinateness (empty differentiation, concrete indifference).',
  }
);

