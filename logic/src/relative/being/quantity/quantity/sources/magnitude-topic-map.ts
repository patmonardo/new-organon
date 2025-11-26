/**
 * TopicMap for continuous-discrete-magnitude.txt - Continuous and Discrete Magnitude
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
 * - continuous-discrete-magnitude-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const CONTINUOUS_DISCRETE_MAGNITUDE_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/being/quantity/quantity/sources/continuous-discrete-magnitude.txt',
  'Hegel\'s Science of Logic - Quantity',
  'Continuous and Discrete Magnitude',
  [
    createTopicMapEntry(
      'quantity-continuous-discrete-intro',
      'Continuous and Discrete Magnitude: two moments of quantity',
      [3, 23],
      'Continuous/Discrete Magnitude: Quantity contains both moments. Posited at first as continuity (continuous magnitude). Continuity is compact unity holding discrete together; posited as such, it is whole quantity, not just moment.',
      [
        'quantity',
        'contains',
        'two moments',
        'continuity',
        'discreteness',
        'posited',
        'both',
        'each',
        'determination',
        'already',
        'start',
        'immediate unity',
        'two',
        'quantity',
        'itself',
        'posited',
        'first',
        'one',
        'two determinations',
        'continuity',
        'such',
        'continuous magnitude',
        'continuity',
        'indeed',
        'one',
        'moments',
        'quantity',
        'brought',
        'completion',
        'other',
        'discreteness',
        'quantity',
        'concrete unity',
        'far',
        'unity',
        'distinct moments',
        'taken',
        'therefore',
        'distinct',
        'without',
        'resolving',
        'again',
        'attraction',
        'repulsion',
        'rather',
        'truly',
        'each',
        'remaining',
        'unity',
        'other',
        'remaining',
        'whole',
        'continuity',
        'only',
        'compact unity',
        'holding together',
        'unity',
        'discrete',
        'posited',
        'such',
        'longer',
        'only',
        'moment',
        'whole quantity',
        'continuous magnitude',
      ],
      { section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE', order: 1 }
    ),

    createTopicMapEntry(
      'quantity-continuous-magnitude',
      'Continuous Magnitude: immediate quantity',
      [25, 32],
      'Continuous Magnitude: Immediate quantity is continuous magnitude. But quantity is not immediate; immediacy is sublated. Quantity must be posited in immanent determinateness (the one) - discrete magnitude.',
      [
        'immediate quantity',
        'continuous magnitude',
        'quantity',
        'however',
        'such',
        'immediate',
        'immediacy',
        'determinateness',
        'sublated',
        'being',
        'precisely',
        'quantity',
        'quantity',
        'posited',
        'therefore',
        'determinateness',
        'immanent',
        'one',
        'quantity',
        'discrete magnitude',
      ],
      { section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE', order: 2 }
    ),

    createTopicMapEntry(
      'quantity-discrete-magnitude',
      'Discrete Magnitude: outsideness-of-one-another as discontinuous',
      [34, 57],
      'Discrete Magnitude: Moment and whole of quantity. Outsideness-of-one-another as discontinuous/broken off. But discreteness is itself continuous (ones are same, have same unity). Discrete magnitude is many ones of a unity, not many ones in general.',
      [
        'discreteness',
        'like',
        'continuity',
        'moment',
        'quantity',
        'itself',
        'also',
        'whole quantity',
        'just',
        'because',
        'moment',
        'whole',
        'therefore',
        'distinct moment',
        'not',
        'diverge',
        'unity',
        'other moment',
        'quantity',
        'outsideness-of-one-another',
        'such',
        'continuous magnitude',
        'outsideness-of-one-another',
        'onwardly',
        'positing',
        'itself',
        'without',
        'negation',
        'internally',
        'self-same',
        'connectedness',
        'other hand',
        'discrete magnitude',
        'outsideness-of-one-another',
        'discontinuous',
        'broken off',
        'aggregate',
        'ones',
        'however',
        'aggregate',
        'atom',
        'void',
        'repulsion',
        'general',
        'thereby',
        'reinstated',
        'because',
        'discrete magnitude',
        'quantity',
        'discreteness',
        'itself',
        'continuous',
        'continuity',
        'discrete',
        'consists',
        'ones',
        'same',
        'another',
        'ones',
        'same unity',
        'discrete magnitude',
        'therefore',
        'one-outside-the-other',
        'many ones',
        'same',
        'many ones',
        'general',
        'posited',
        'rather',
        'many',
        'unity',
      ],
      { section: 'B. CONTINUOUS AND DISCRETE MAGNITUDE', order: 3 }
    ),
  ],
  {
    sectionDescription: 'Continuous and Discrete Magnitude - Two moments of quantity (posited as continuity, continuous magnitude). Continuous Magnitude (immediate quantity, but quantity not immediate, must be posited in one). Discrete Magnitude (moment and whole, discontinuous but continuous, many ones of unity).',
  }
);

