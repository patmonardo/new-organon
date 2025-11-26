/**
 * TopicMap for exposition.txt - The Exposition of the Absolute
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
 * - exposition-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const EXPOSITION_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/absolute/sources/exposition.txt',
  'Hegel\'s Science of Logic - Actuality',
  'The Exposition of the Absolute',
  [
    createTopicMapEntry(
      'abs-a-1-absolute-not-being-nor-essence',
      'Absolute not being nor essence — absolute unity',
      [3, 22],
      'Absolute not = being (first unreflected immediacy) nor essence (reflected immediacy). Each = determinate totality. Being emerges in essence = concrete existence. Connection develops = relation of inner and outer. Inner = essence (referred to being, being immediately). Outer = being (connected with reflection, relationless identity with essence). Absolute = absolute unity of two, ground of essential relation.',
      [
        'absolute',
        'being',
        'essence',
        'first unreflected immediacy',
        'reflected immediacy',
        'determinate totality',
        'concrete existence',
        'relation of inner and outer',
        'inner',
        'outer',
        'absolute unity',
        'ground of essential relation',
      ],
      { section: 'The Exposition of the Absolute', order: 1 }
    ),

    createTopicMapEntry(
      'abs-a-2-absolute-form-and-content',
      'Absolute form and content — each moment totality',
      [24, 37],
      'Determination of absolute = absolute form. Not identity with simple determinacies, but identity whose moments = each explicitly totality, indifferent to form = complete content. Conversely, absolute = absolute content (indifferent plurality) with negative connection of form, manifold = one substantial identity.',
      [
        'determination of absolute',
        'absolute form',
        'simple determinacies',
        'moments as totality',
        'indifferent to form',
        'complete content',
        'absolute content',
        'indifferent plurality',
        'negative connection of form',
        'manifold',
        'one substantial identity',
      ],
      { section: 'The Exposition of the Absolute', order: 2 }
    ),

    createTopicMapEntry(
      'abs-a-3-absolute-identity-transparent',
      'Absolute identity — transparent reflective shine',
      [39, 51],
      'Identity of absolute = absolute identity because each part = whole, each determinateness = totality. Determinateness = transparent reflective shine, difference disappeared in positedness. Reflected determinations (essence, concrete existence, world, whole, parts, force) appear as true being, but absolute = ground into which they foundered.',
      [
        'absolute identity',
        'each part = whole',
        'each determinateness = totality',
        'transparent reflective shine',
        'difference disappeared',
        'positedness',
        'reflected determinations',
        'essence',
        'concrete existence',
        'world existing in itself',
        'whole',
        'parts',
        'force',
        'ground',
        'foundered',
      ],
      { section: 'The Exposition of the Absolute', order: 3 }
    ),

    createTopicMapEntry(
      'abs-a-4-absolute-does-not-determine',
      'Absolute does not determine itself — movement stands over',
      [52, 75],
      'In absolute: form = simple self-identity, absolute does not determine itself (determination = difference of form). But absolute contains every difference/form determination = absolute form and reflection, difference of content must come into it. Absolute = absolute identity (manifoldness sublated). No becoming, not being; not reflective determination, not essence; not externalization, identity of inner/outer. Movement of reflection stands over against absolute identity, sublated = inner, consequently outer.',
      [
        'simple self-identity',
        'does not determine itself',
        'difference of form',
        'contains every difference',
        'form determination',
        'absolute form and reflection',
        'difference of content',
        'absolute identity',
        'manifoldness sublated',
        'no becoming',
        'not being',
        'not essence',
        'not externalization',
        'identity of inner and outer',
        'movement of reflection',
        'stands over against',
        'sublated',
        'inner',
        'outer',
      ],
      { section: 'The Exposition of the Absolute', order: 4 }
    ),

    createTopicMapEntry(
      'abs-a-5-negative-exposition',
      'Negative exposition — beyond of differences',
      [77, 96],
      'Movement = sublating its act in absolute. Beyond of manifold differences/determinations/movement, beyond at back of absolute = negative exposition. True presentation = preceding whole of logical movement (being and essence). Content not gathered from outside, not sunk by external reflection, but determined itself by inner necessity. Being\'s own becoming and reflection of essence returned into absolute as ground.',
      [
        'movement',
        'sublating act in absolute',
        'beyond of differences',
        'beyond of determinations',
        'beyond at back of absolute',
        'negative exposition',
        'preceding whole',
        'logical movement',
        'being',
        'essence',
        'not gathered from outside',
        'not sunk by external reflection',
        'determined by inner necessity',
        'being\'s own becoming',
        'reflection of essence',
        'returned into absolute',
        'ground',
      ],
      { section: 'The Exposition of the Absolute', order: 5 }
    ),

    createTopicMapEntry(
      'abs-a-6-positive-exposition',
      'Positive exposition — finite as expression of absolute',
      [98, 124],
      'Exposition has positive side: finite foundering demonstrates nature = referred to absolute, contains absolute. But = exposition of determinations (absolute = abyss and ground, imparts subsistence to reflective shine). Being as shine = reflection, reference to absolute. Positive exposition halts finite before disappearing, considers it expression/copy of absolute. Transparency lets absolute transpire, ends in complete disappearance (nothing retains difference, absorbed as medium).',
      [
        'positive exposition',
        'positive side',
        'finite',
        'foundering',
        'referred to absolute',
        'contains absolute',
        'exposition of determinations',
        'abyss',
        'ground',
        'imparts subsistence',
        'reflective shine',
        'being as shine',
        'reflection',
        'reference to absolute',
        'halts finite',
        'expression',
        'copy',
        'transparency',
        'complete disappearance',
        'absorbed as medium',
      ],
      { section: 'The Exposition of the Absolute', order: 6 }
    ),

    createTopicMapEntry(
      'abs-a-7-positive-exposition-reflective-shine',
      'Positive exposition as reflective shine — absolute\'s own doing',
      [126, 167],
      'Positive exposition = reflective shine. True positive = absolute itself (contains exposition and content). Form in which absolute shines = nullity gathered from outside, starting point. Determination has in absolute not beginning but end. Expository process = absolute act (returns to absolute) but not at starting point (external determination). But in fact: exposition = absolute\'s own doing, begins from itself, arrives at itself. Absolute as absolute identity = determined guise, posited by reflection over against opposition/manifoldness, negative of reflection/determination. Not just exposition incomplete but absolute itself incomplete. Absolute as absolute identity = absolute of external reflection, not absolutely absolute but absolute in determination = attribute.',
      [
        'positive exposition',
        'reflective shine',
        'true positive',
        'absolute itself',
        'nullity',
        'gathered from outside',
        'starting point',
        'determination',
        'beginning',
        'end',
        'expository process',
        'absolute act',
        'returns to absolute',
        'external determination',
        'absolute\'s own doing',
        'begins from itself',
        'arrives at itself',
        'absolute identity',
        'determined guise',
        'identical absolute',
        'posited by reflection',
        'opposition',
        'manifoldness',
        'negative of reflection',
        'negative of determination',
        'incomplete',
        'absolute of external reflection',
        'absolutely absolute',
        'absolute in determination',
        'attribute',
      ],
      { section: 'The Exposition of the Absolute', order: 7 }
    ),

    createTopicMapEntry(
      'abs-a-8-absolute-as-attribute',
      'Absolute as attribute — absolute form',
      [169, 181],
      'Absolute not attribute just because subject matter of external reflection. Reflection not only external but immediately internal (because external). Absolute absolute only because not abstract identity but identity of being and essence (inner and outer). Absolute = absolute form that makes it shine within itself and determines as attribute.',
      [
        'attribute',
        'subject matter',
        'external reflection',
        'immediately internal',
        'abstract identity',
        'identity of being and essence',
        'identity of inner and outer',
        'absolute form',
        'reflectively shine',
        'within itself',
        'determines as attribute',
      ],
      { section: 'The Exposition of the Absolute', order: 8 }
    ),
  ],
  {
    sectionDescription: 'The Exposition of the Absolute - Absolute as unity of being and essence, absolute form and content, absolute identity as transparent reflective shine. Absolute does not determine itself, movement stands over. Negative and positive exposition. Positive exposition as reflective shine, absolute\'s own doing. Absolute as attribute, absolute form.',
  }
);

