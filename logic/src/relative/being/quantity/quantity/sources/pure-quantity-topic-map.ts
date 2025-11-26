/**
 * TopicMap for pure-quantity.txt - Pure Quantity
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
 * - pure-quantity-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const PURE_QUANTITY_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/being/quantity/quantity/sources/pure-quantity.txt',
  'Hegel\'s Science of Logic - Quantity',
  'Pure Quantity',
  [
    createTopicMapEntry(
      'quantity-pure-intro',
      'Pure Quantity: sublated being-for-itself, attraction',
      [3, 14],
      'Pure Quantity: Sublated being-for-itself. Repelling one now behaves as identical to itself; being-for-itself passes into attraction. The one\'s obduracy melts into unity (containing repulsion); attraction is continuity.',
      [
        'quantity',
        'sublated',
        'being-for-itself',
        'repelling one',
        'behaved',
        'negatively',
        'excluded one',
        'gone over',
        'connection',
        'behaves',
        'other',
        'identical',
        'itself',
        'lost',
        'determination',
        'being-for-itself',
        'passed over',
        'attraction',
        'absolute obduracy',
        'one',
        'melted away',
        'unity',
        'containing',
        'one',
        'determined',
        'repulsion',
        'residing',
        'unity',
        'self-externality',
        'unity',
        'itself',
        'attraction',
        'moment',
        'continuity',
        'quantity',
      ],
      { section: 'A. PURE QUANTITY', order: 1 }
    ),

    createTopicMapEntry(
      'quantity-pure-continuity',
      'Continuity: simple self-same reference, unity of ones',
      [16, 30],
      'Continuity: Simple self-same reference unbroken by limit/exclusion. Unity of ones (not immediate unity). Contains outsideness-of-one-another without distinctions. Many are each what others are - simple undifferentiated equality. Self-continuation of different ones into ones from which distinguished.',
      [
        'continuity',
        'therefore',
        'simple',
        'self-same',
        'reference',
        'itself',
        'unbroken',
        'limit',
        'exclusion',
        'however',
        'immediate unity',
        'unity',
        'ones',
        'existence',
        'themselves',
        'still',
        'contained',
        'outside-one-another',
        'plurality',
        'same time',
        'something',
        'without',
        'distinctions',
        'unbroken',
        'plurality',
        'posited',
        'continuity',
        'implicitly',
        'itself',
        'many',
        'each',
        'others',
        'like',
        'other',
        'plurality',
        'consequently',
        'simple',
        'undifferentiated',
        'equality',
        'continuity',
        'moment',
        'self-equality',
        'outsideness-of-one-another',
        'self-continuation',
        'different ones',
        'ones',
        'distinguished',
      ],
      { section: 'A. PURE QUANTITY', order: 2 }
    ),

    createTopicMapEntry(
      'quantity-pure-discreteness',
      'Discreteness: repulsion as moment in quantity',
      [32, 40],
      'Discreteness: Magnitude immediately possesses discreteness (repulsion) as moment in quantity. Repulsion expands self-equality to continuity. Discreteness is of confluents (ones without void), steady advance that does not interrupt self-equality.',
      [
        'continuity',
        'therefore',
        'magnitude',
        'immediately',
        'possesses',
        'moment',
        'discreteness',
        'repulsion',
        'moment',
        'quantity',
        'steady',
        'continuity',
        'self-equality',
        'many',
        'become',
        'exclusive',
        'repulsion',
        'first',
        'expands',
        'self-equality',
        'continuity',
        'hence',
        'discreteness',
        'part',
        'discreteness',
        'confluents',
        'ones',
        'void',
        'connect',
        'negative',
        'own',
        'steady',
        'advance',
        'many',
        'interrupt',
        'self-equality',
      ],
      { section: 'A. PURE QUANTITY', order: 3 }
    ),

    createTopicMapEntry(
      'quantity-pure-unity',
      'Quantity as unity of continuity and discreteness',
      [42, 65],
      'Quantity: Unity of continuity and discreteness. At first continuity (being-for-itself collapsed to self-equal immediacy). Contains moments as being-for-itself in truth. Repulsion is creative flowing away; sameness yields unbroken continuity; coming-out-of-itself yields plurality persisting in equality.',
      [
        'quantity',
        'unity',
        'moments',
        'continuity',
        'discreteness',
        'first',
        'however',
        'continuity',
        'form',
        'one',
        'continuity',
        'result',
        'dialectic',
        'being-for-itself',
        'collapsed',
        'form',
        'self-equal',
        'immediacy',
        'quantity',
        'simple',
        'result',
        'far',
        'being-for-itself',
        'developed',
        'moments',
        'posited',
        'within',
        'quantity',
        'contains',
        'moments',
        'first',
        'being-for-itself',
        'posited',
        'truth',
        'determination',
        'being-in-itself',
        'self-sublating',
        'self-reference',
        'perpetual',
        'coming-out-of-itself',
        'repelled',
        'itself',
        'repulsion',
        'creative',
        'flowing away',
        'itself',
        'account',
        'sameness',
        'repelled',
        'discerning',
        'unbroken',
        'continuity',
        'account',
        'coming-out-of-itself',
        'continuity',
        'same time',
        'broken off',
        'plurality',
        'plurality',
        'persists',
        'immediately',
        'equality',
        'itself',
      ],
      { section: 'A. PURE QUANTITY', order: 4 }
    ),
  ],
  {
    sectionDescription: 'Pure Quantity - Sublated being-for-itself (attraction, continuity). Continuity (simple self-same reference, unity of ones, self-equality). Discreteness (repulsion as moment, confluents, steady advance). Unity of continuity and discreteness (being-for-itself in truth, creative flowing away, plurality in equality).',
  }
);

