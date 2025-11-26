/**
 * TopicMap for attribute.txt - The Absolute Attribute
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
 * - attribute-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const ATTRIBUTE_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/absolute/sources/attribute.txt',
  'Hegel\'s Science of Logic - Actuality',
  'The Absolute Attribute',
  [
    createTopicMapEntry(
      'abs-b-1-absolute-absolute-vs-attribute',
      'Absolute absolute vs attribute — relative absolute',
      [3, 20],
      '"Absolute absolute" = absolute whose form returned into itself, form = content. Attribute = relative absolute, absolute in form determination. Form at first = internally/externally, determinate form/negation. But form = form of absolute, attribute = whole content, totality (world, side of essential relation, each = whole).',
      [
        'absolute absolute',
        'form returned into itself',
        'form equals content',
        'attribute',
        'relative absolute',
        'form determination',
        'internally',
        'externally',
        'determinate form',
        'negation',
        'form of absolute',
        'whole content',
        'totality',
        'world',
        'side of essential relation',
        'each is whole',
      ],
      { section: 'The Absolute Attribute', order: 1 }
    ),

    createTopicMapEntry(
      'abs-b-2-worlds-reduced-to-shine',
      'Worlds reduced to reflective shine — true subsistence',
      [21, 42],
      'Both worlds opposed in essence. Each side equal to other (whole/parts, expression/force, outer/inner). But each had immediate subsistence (existent immediacy, reflected immediacy). In absolute: different immediacies reduced to reflective shine. Totality (attribute) = true single subsistence, determination = unessential subsistence.',
      [
        'phenomenal world',
        'world in and for itself',
        'opposed in essence',
        'each side equal',
        'whole',
        'parts',
        'expression of force',
        'force',
        'outer',
        'inner',
        'immediate subsistence',
        'existent immediacy',
        'reflected immediacy',
        'reduced to reflective shine',
        'true single subsistence',
        'unessential subsistence',
      ],
      { section: 'The Absolute Attribute', order: 2 }
    ),

    createTopicMapEntry(
      'abs-b-3-attribute-as-identity-determination',
      'Attribute as identity determination — absolute totality',
      [44, 60],
      'Absolute = attribute because simple absolute identity in determination of identity. Other determinations can be attached (several attributes). But absolute identity = all determinations sublated, reflection sublated itself, determinations posited as sublated. Totality = absolute totality. Attribute has absolute for content/subsistence, form determination posited as mere reflective shine, negative posited as negative.',
      [
        'absolute',
        'attribute',
        'simple absolute identity',
        'determination of identity',
        'other determinations',
        'several attributes',
        'all determinations sublated',
        'reflection sublated itself',
        'determinations posited as sublated',
        'absolute totality',
        'content',
        'subsistence',
        'form determination',
        'mere reflective shine',
        'negative posited as negative',
      ],
      { section: 'The Absolute Attribute', order: 3 }
    ),

    createTopicMapEntry(
      'abs-b-4-positive-reflective-shine-sublates',
      'Positive reflective shine sublates attribute',
      [61, 69],
      'Positive reflective shine (exposition through attribute): does not take finite in limitation as existing in and for itself, dissolves subsistence into absolute, expands into attribute. Sublates that attribute is attribute, sinks it and differentiating act into simple absolute.',
      [
        'positive reflective shine',
        'exposition',
        'attribute',
        'finite',
        'limitation',
        'existing in and for itself',
        'dissolves subsistence',
        'absolute',
        'expands into attribute',
        'sublates',
        'attribute is attribute',
        'differentiating act',
        'simple absolute',
      ],
      { section: 'The Absolute Attribute', order: 4 }
    ),

    createTopicMapEntry(
      'abs-b-5-reflection-reverts-to-identity',
      'Reflection reverts to identity — not true absolute',
      [71, 89],
      'Reflection reverts from differentiating act only to identity of absolute, has not left externality behind, not arrived at true absolute. Only reached indeterminate abstract identity (identity in determinateness of identity). Reflection determines absolute into attribute as inner form, determining distinct from externality, inner determination does not penetrate, attribute\'s expression (merely posited) to disappear into absolute.',
      [
        'reflection',
        'reverts',
        'differentiating act',
        'identity of absolute',
        'externality',
        'true absolute',
        'indeterminate abstract identity',
        'identity in determinateness',
        'determines absolute',
        'inner form',
        'distinct from externality',
        'inner determination',
        'does not penetrate',
        'attribute\'s expression',
        'merely posited',
        'disappear into absolute',
      ],
      { section: 'The Absolute Attribute', order: 5 }
    ),

    createTopicMapEntry(
      'abs-b-6-form-nullity',
      'Form as nullity — external reflective shine',
      [91, 96],
      'Form by which absolute would be attribute (outer or inner) = null in itself, external reflective shine, mere way and manner.',
      [
        'form',
        'absolute',
        'attribute',
        'outer',
        'inner',
        'null in itself',
        'external reflective shine',
        'mere way and manner',
      ],
      { section: 'The Absolute Attribute', order: 6 }
    ),
  ],
  {
    sectionDescription: 'The Absolute Attribute - Absolute absolute vs relative absolute (attribute). Worlds reduced to reflective shine, true subsistence. Attribute as identity determination, absolute totality. Positive reflective shine sublates attribute. Reflection reverts to identity, not true absolute. Form as nullity, external reflective shine.',
  }
);

