/**
 * TopicMap for method-beginning.txt - The Beginning
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
 * - method-beginning-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const METHOD_BEGINNING_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/speculation/sources/method-beginning.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Beginning',
  [
    createTopicMapEntry(
      'method-beginning-1-simple-universal',
      'Beginning as Simple and Universal',
      [1, 66],
      'Method = determinations of concept and connections. Beginning = simple nature. Content = immediate = abstract universality. Not sense-intuition/representation, but thought = supersensuous inner intuiting. Beginning = simple/universal in element of thought. Universality = immediate universality = being (abstract self-reference). Being needs no derivation. Universal = immediate = abstract self-reference = being. Demand for being exhibited = demand for realization of concept (goal of development).',
      [
        'method',
        'beginning',
        'immediate',
        'abstract universality',
        'thought',
        'being',
        'abstract self-reference',
        'realization of concept',
      ],
      { section: 'The Beginning', order: 1 }
    ),

    createTopicMapEntry(
      'method-beginning-2-deficient-absolute',
      'Beginning as Deficient — Absolute as Implicit',
      [68, 123],
      'Beginning = simple/universal (deficient). Universality = pure simple concept, only moment, concept not determined. Method objective/immanent: beginning has lack, impulse to carry further. Absolute method: universal = objective universal = concrete totality (not posited/for itself). Abstract universal = posited afflicted by negation. Nothing so simple/abstract. Beginning = immediate = in-itself without being-for-itself. Every beginning = absolute (implicit in existence = concept). Absolute implicit = not absolute/not posited concept/not idea. Advance = universal determines itself = universal for itself = singular/subject. Consummation = absolute.',
      [
        'beginning',
        'deficient',
        'universality',
        'pure simple concept',
        'objective universal',
        'concrete totality',
        'absolute',
        'implicit',
        'advance',
        'consummation',
      ],
      { section: 'The Beginning', order: 2 }
    ),

    createTopicMapEntry(
      'method-beginning-3-concrete-totality',
      'Concrete Totality as Beginning',
      [125, 153],
      'Beginning in itself concrete totality = free, immediacy = external existence. Germ of living, subjective purpose = such beginnings = impulses. Non-spiritual/inanimate = concrete concept only as real possibility. Cause = highest stage in necessity, immediate existence, but not subject maintaining itself. Inanimate = determinate concrete existences, real possibility = inner totality. Moments not posited in subjective form, attain concrete existence through other corporeal individuals.',
      [
        'concrete totality',
        'beginning',
        'germ',
        'living',
        'subjective purpose',
        'impulse',
        'real possibility',
        'cause',
        'inanimate',
        'corporeal individuals',
      ],
      { section: 'The Beginning', order: 3 }
    ),
  ]
);

