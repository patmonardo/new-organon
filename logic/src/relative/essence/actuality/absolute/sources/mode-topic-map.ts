/**
 * TopicMap for mode.txt - The Mode of the Absolute
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
 * - mode-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const MODE_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/absolute/sources/mode.txt',
  'Hegel\'s Science of Logic - Actuality',
  'The Mode of the Absolute',
  [
    createTopicMapEntry(
      'abs-c-1-attribute-two-sides',
      'Attribute two sides — extremes and middle term',
      [3, 23],
      'Attribute first = absolute in simple self-identity. Second = negation (formal immanent reflection). Two sides = two extremes, middle term = attribute itself (absolute and determinateness). Second extreme = negative as negative, reflection external to absolute. Negative as inner, determination = posit itself as mode = self-externality of absolute, loss in changeability/contingency, passed over into opposite without turning back, manifoldness lacking totality.',
      [
        'attribute',
        'absolute in simple self-identity',
        'negation',
        'formal immanent reflection',
        'two sides',
        'two extremes',
        'middle term',
        'absolute',
        'determinateness',
        'negative as negative',
        'reflection external to absolute',
        'inner',
        'posit itself as mode',
        'self-externality',
        'loss',
        'changeability',
        'contingency',
        'passed over into opposite',
        'without turning back',
        'manifoldness',
        'lacks totality',
      ],
      { section: 'The Mode of the Absolute', order: 1 }
    ),

    createTopicMapEntry(
      'abs-c-2-mode-externality-posited',
      'Mode as externality posited — self-identity',
      [25, 35],
      'Mode (externality of absolute) not just self-externality. Rather = externality posited as externality, mere way and manner, reflective shine as reflective shine, reflection of form into itself = self-identity (absolute). Absolute first posited as absolute identity only in mode. Self-identity only as self-referring negativity, reflective shining posited as reflective shining.',
      [
        'mode',
        'externality of absolute',
        'self-externality',
        'externality posited as externality',
        'mere way and manner',
        'reflective shine as reflective shine',
        'reflection of form into itself',
        'self-identity',
        'absolute',
        'absolute identity',
        'self-referring negativity',
        'reflective shining',
        'posited as reflective shining',
      ],
      { section: 'The Mode of the Absolute', order: 2 }
    ),

    createTopicMapEntry(
      'abs-c-3-exposition-exhaustive',
      'Exposition exhaustive — three moments',
      [37, 41],
      'Exposition begins from absolute identity, passes to attribute, then to mode = exhaustively run through moments.',
      [
        'exposition',
        'absolute identity',
        'attribute',
        'mode',
        'exhaustively run through',
        'moments',
      ],
      { section: 'The Mode of the Absolute', order: 3 }
    ),

    createTopicMapEntry(
      'abs-c-4-reflective-movement',
      'Reflective movement — absolute identity',
      [43, 47],
      'First: course not just behaves negatively, act = reflective movement itself. Absolute truly absolute identity only as such movement.',
      [
        'first',
        'course',
        'behaves negatively',
        'act',
        'reflective movement',
        'absolute truly absolute identity',
        'such movement',
      ],
      { section: 'The Mode of the Absolute', order: 4 }
    ),

    createTopicMapEntry(
      'abs-c-5-mode-immanent-turning-back',
      'Mode as immanent turning back — absolute being',
      [49, 54],
      'Second: exposition not mere externality, mode not only most external externality. Mode = reflective shine as shine = immanent turning back, self-dissolving reflection. In being this reflection, absolute = absolute being.',
      [
        'second',
        'exposition',
        'mere externality',
        'most external externality',
        'mode',
        'reflective shine as shine',
        'immanent turning back',
        'self-dissolving reflection',
        'absolute being',
      ],
      { section: 'The Mode of the Absolute', order: 5 }
    ),

    createTopicMapEntry(
      'abs-c-6-determinateness-from-absolute',
      'Determinateness from absolute itself — absolute form',
      [56, 74],
      'Third: reflective act seems to begin from own determinations/external, take up modes/determinations as if outside absolute, reduce to undifferentiated identity. But found determinateness in absolute itself. As first undifferentiated identity, absolute = determinate absolute/attribute (unmoved, unreflected). Determinateness belongs to reflective movement. Through movement alone: absolute determined as first identity, has absolute form, posits itself as self-equal.',
      [
        'third',
        'reflective act',
        'own determinations',
        'external',
        'modes',
        'determinations',
        'outside absolute',
        'reduce to undifferentiated identity',
        'determinateness',
        'absolute itself',
        'first undifferentiated identity',
        'determinate absolute',
        'attribute',
        'unmoved',
        'unreflected',
        'reflective movement',
        'first identity',
        'absolute form',
        'posits itself as self-equal',
      ],
      { section: 'The Mode of the Absolute', order: 6 }
    ),

    createTopicMapEntry(
      'abs-c-7-true-meaning-of-mode',
      'True meaning of mode — absolute\'s own reflective movement',
      [76, 88],
      'True meaning of mode = absolute\'s own reflective movement. Determining by which absolute becomes what it already is (not other). Transparent externality pointing to itself. Movement out of itself, being outwardly = inwardness, positing = absolute being (not mere positedness).',
      [
        'true meaning of mode',
        'absolute\'s own reflective movement',
        'determining',
        'becomes what it already is',
        'not other',
        'transparent externality',
        'pointing to itself',
        'movement out of itself',
        'being outwardly',
        'inwardness',
        'positing',
        'absolute being',
        'not mere positedness',
      ],
      { section: 'The Mode of the Absolute', order: 7 }
    ),

    createTopicMapEntry(
      'abs-c-8-content-of-exposition',
      'Content of exposition — absolute manifests itself',
      [90, 104],
      'Content of exposition = what absolute manifests. Distinction of form and content dissolved. Content = absolute manifests itself. Absolute = absolute form, in diremption utterly identical with itself, negative as negative rejoining itself = absolute self-identity, indifferent towards distinctions = absolute content. Content = exposition itself.',
      [
        'content of exposition',
        'what absolute manifests',
        'distinction of form and content',
        'dissolved',
        'absolute manifests itself',
        'absolute form',
        'diremption',
        'utterly identical with itself',
        'negative as negative',
        'rejoining itself',
        'absolute self-identity',
        'indifferent towards distinctions',
        'absolute content',
        'exposition itself',
      ],
      { section: 'The Mode of the Absolute', order: 8 }
    ),

    createTopicMapEntry(
      'abs-c-9-absolute-as-actuality',
      'Absolute as actuality — absolute manifestation',
      [106, 114],
      'As self-bearing movement of exposition, way and manner = absolute identity with itself, absolute = expression (not of inner, not over against other) = absolute manifestation of itself for itself = actuality.',
      [
        'self-bearing movement',
        'exposition',
        'way and manner',
        'absolute identity with itself',
        'expression',
        'not of inner',
        'not over against other',
        'absolute manifestation',
        'itself for itself',
        'actuality',
      ],
      { section: 'The Mode of the Absolute', order: 9 }
    ),
  ],
  {
    sectionDescription: 'The Mode of the Absolute - Attribute two sides (extremes and middle term). Mode as externality posited, self-identity. Exposition exhaustive (three moments). Reflective movement, absolute identity. Mode as immanent turning back, absolute being. Determinateness from absolute itself, absolute form. True meaning of mode as absolute\'s own reflective movement. Content of exposition, absolute manifests itself. Absolute as actuality, absolute manifestation.',
  }
);

