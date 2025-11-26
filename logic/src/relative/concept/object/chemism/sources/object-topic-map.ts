/**
 * TopicMap for object.txt - The Chemical Object
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
 * - object-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const CHEMICAL_OBJECT_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/object/chemism/sources/object.txt',
  'Hegel\'s Science of Logic - The Concept',
  'The Chemical Object',
  [
    createTopicMapEntry(
      'chem-obj-1-distinguished-mechanical',
      'Chemical Object Distinguished from Mechanical',
      [2, 29],
      'Chemical object distinguished from mechanical. Mechanical is totality indifferent to determinateness. In chemical object, determinateness belongs to its nature. Reference to other, mode and manner of reference belong to nature. Determinateness is essentially particularization. Taken up into universality, thus a principle. Determinateness which is universal, not only of one singular object but also of other. Distinction in concept: Inner totality of two determinacies vs determinateness constituting nature of singular object in externality and concrete existence. Object implicitly whole concept. Has within it necessity and impulse to sublate opposed, one-sided subsistence. Bring itself in existence to real whole which it is according to concept.',
      [
        'chemical object',
        'mechanical object',
        'determinateness',
        'particularization',
        'universality',
        'principle',
        'inner totality',
        'necessity',
        'impulse',
      ],
      { section: 'The Chemical Object', order: 1 }
    ),

    createTopicMapEntry(
      'chem-obj-2-expression-chemism',
      'Expression "Chemism" Not Limited to Elemental Nature',
      [31, 42],
      'Expression "chemism" for relation of non-indifference of objectivity. Not to be understood as only found in elemental nature strictly going by that name. Meteorological relation must be regarded as process whose parts have more nature of physical than chemical elements. In animate things, sex relation falls under this schema. Schema constitutes formal basis for spiritual relations of love, friendship, and the like.',
      [
        'chemism',
        'non-indifference',
        'meteorological',
        'sex relation',
        'spiritual relations',
        'love',
        'friendship',
      ],
      { section: 'The Chemical Object', order: 2 }
    ),

    createTopicMapEntry(
      'chem-obj-3-self-subsistent-totality',
      'Chemical Object as Self-Subsistent Totality',
      [44, 66],
      'Chemical object at first self-subsistent totality in general. Reflected into itself, distinct from reflectedness outwards. Indifferent basis, individual not yet determined as non-indifferent. Person too is basis of this kind, refers only to itself. Immanent determinateness constituting object\'s non-indifference. First: Reflected into itself such that retraction of reference outwards is only formal abstract universality. Outwards reference is determination of object\'s immediacy and concrete existence. From this side object does not return within it to individual totality. Negative unity has two moments of opposition in two particular objects. Chemical object not comprehensible from itself. Being of one object is being of another.',
      [
        'self-subsistent',
        'totality',
        'reflected into itself',
        'indifferent basis',
        'immanent determinateness',
        'formal abstract universality',
        'negative unity',
        'opposition',
      ],
      { section: 'The Chemical Object', order: 3 }
    ),

    createTopicMapEntry(
      'chem-obj-4-contradiction-striving',
      'Determinateness Absolutely Reflected, Contradiction, Striving',
      [67, 84],
      'Second: Determinateness absolutely reflected into itself. Concrete moment of individual concept of whole. Universal essence, real genus of particular objects. Chemical object is contradiction of immediate positedness and immanent individual concept. Striving to sublate immediate determinateness of existence. Give concrete existence to objective totality of concept. Remains non-self-subsistent object. But by nature in tension with lack of self-subsistence. Initiates process as self-determining.',
      [
        'absolutely reflected',
        'universal essence',
        'real genus',
        'contradiction',
        'striving',
        'objective totality',
        'tension',
        'self-determining',
      ],
      { section: 'The Chemical Object', order: 4 }
    ),
  ]
);

