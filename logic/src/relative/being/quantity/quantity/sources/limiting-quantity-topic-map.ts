/**
 * TopicMap for limiting-quantity.txt - The Limiting of Quantity
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
 * - limiting-quantity-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const LIMITING_QUANTITY_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/being/quantity/quantity/sources/limiting-quantity.txt',
  'Hegel\'s Science of Logic - Quantity',
  'The Limiting of Quantity',
  [
    createTopicMapEntry(
      'quantity-limiting-intro',
      'Limiting of Quantity: discrete magnitude as quantum',
      [3, 21],
      'Limiting of Quantity: Discrete magnitude has one as principle, plurality of ones, essentially continuous (one sublated as unity). Posited as one magnitude; "one" is determinateness/limit. Distinguished from continuous magnitude, it is existence/something with "one" as limit.',
      [
        'discrete magnitude',
        'first',
        'one',
        'principle',
        'second',
        'plurality',
        'ones',
        'third',
        'essentially',
        'continuous',
        'one',
        'same time',
        'sublated',
        'unity',
        'self-continuing',
        'such',
        'discreteness',
        'ones',
        'consequently',
        'posited',
        'one magnitude',
        'one',
        'determinateness',
        'one',
        'posited',
        'determinate',
        'existence',
        'excludes',
        'limit',
        'unity',
        'discrete magnitude',
        'such',
        'supposed',
        'immediately',
        'limited',
        'distinguished',
        'continuous magnitude',
        'existence',
        'something',
        'determinateness',
        'first negation',
        'limit',
        'one',
      ],
      { section: 'C. THE LIMITING OF QUANTITY', order: 1 }
    ),

    createTopicMapEntry(
      'quantity-limiting-encompassing',
      'Encompassing limit: one as enclosing limit',
      [23, 34],
      'Encompassing Limit: Limit refers to unity (negation moment) and is self-referred as one - enclosing/encompassing. Limit is negative point itself (not distinct from something). But limited being is continuity, transcending limit, indifferent to it. Discrete quantity is quantum: quantity as existence/something.',
      [
        'limit',
        'besides',
        'referring',
        'unity',
        'moment',
        'negation',
        'also',
        'one',
        'self-referred',
        'thus',
        'enclosing',
        'encompassing',
        'limit',
        'limit',
        'here',
        'first',
        'distinct',
        'something',
        'existence',
        'one',
        'essentially',
        'negative point',
        'itself',
        'being',
        'limited',
        'essentially',
        'continuity',
        'virtue',
        'continuity',
        'transcends',
        'limit',
        'transcends',
        'one',
        'indifferent',
        'real',
        'discrete quantity',
        'one quantity',
        'quantum',
        'quantity',
        'existence',
        'something',
      ],
      { section: 'C. THE LIMITING OF QUANTITY', order: 2 }
    ),

    createTopicMapEntry(
      'quantity-limiting-quantum',
      'Quantum: limit encompassing many ones, passing over',
      [36, 45],
      'Quantum: One as limit encompasses many ones (sublated in it). Limit to continuity; distinction continuous/discrete becomes indifferent. Limit to continuity of one and other; both pass over into quanta.',
      [
        'since',
        'one',
        'limit',
        'encompasses',
        'within',
        'many ones',
        'discrete quantity',
        'posits',
        'equally',
        'sublated',
        'limit',
        'continuity',
        'simply',
        'such',
        'consequently',
        'distinction',
        'between',
        'continuous',
        'discrete magnitude',
        'here',
        'indifferent',
        'more',
        'precisely',
        'limit',
        'continuity',
        'one',
        'much',
        'other',
        'both',
        'pass over',
        'quanta',
      ],
      { section: 'C. THE LIMITING OF QUANTITY', order: 3 }
    ),
  ],
  {
    sectionDescription: 'The Limiting of Quantity - Discrete magnitude as quantum (one as principle, plurality, essentially continuous, one magnitude with one as limit). Encompassing limit (enclosing/encompassing, negative point, continuity transcending limit, quantum). Quantum (limit encompassing many ones, distinction becomes indifferent, both pass over into quanta).',
  }
);

