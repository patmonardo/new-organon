/**
 * TopicMap for relation-substantiality.txt - The Relation of Substantiality
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
 * - relation-substantiality-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const RELATION_SUBSTANTIALITY_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/substance/sources/relation-substantiality.txt',
  'Hegel\'s Science of Logic - The Absolute Relation',
  'The Relation of Substantiality',
  [
    createTopicMapEntry(
      'sub-a-1-substance-being',
      'Substance — being that is because it is',
      [3, 23],
      'Absolute necessity = absolute relation (not being as such, but being that is because it is, absolute mediation of itself with itself). This being = substance (final unity of essence and being, being in all being). Not unreflected immediate, not abstract behind concrete existence/appearance, but immediate actuality, absolutely reflected into itself, subsisting in and for itself. Substance = unity of being and reflection, shining and positedness. Shining = self-referring shining = being = substance. Being = self-identical positedness = shining totality = accidentality.',
      [
        'absolute necessity',
        'absolute relation',
        'not being as such',
        'being that is because it is',
        'absolute mediation',
        'itself with itself',
        'substance',
        'final unity of essence and being',
        'being in all being',
        'not unreflected immediate',
        'not abstract behind',
        'immediate actuality',
        'absolutely reflected into itself',
        'subsisting in and for itself',
        'unity of being and reflection',
        'shining',
        'positedness',
        'self-referring shining',
        'being',
        'self-identical positedness',
        'shining totality',
        'accidentality',
      ],
      { section: 'The Relation of Substantiality', order: 1 }
    ),

    createTopicMapEntry(
      'sub-a-2-shining-becoming',
      'Shining as becoming — movement of accidentality',
      [25, 57],
      'Shining = identity of form, unity of possibility and actuality. Becoming at first, contingency (coming-to-be and passing-away). In immediacy: connection = immediate conversion into each other as existents. But being = shine, relation = identical terms/shining in one another = reflection. Movement of accidentality exhibits mutual reflective shine of categories of being and reflective determinations of essence. Immediate something has content, immediacy = reflected indifference to form. Content determinate, determinateness of being, passes into other. Quality = determinateness of reflection = indifferent diversity, animated into opposition, returns to ground (nothing, immanent reflection). Reflection sublates itself, reflected in-itselfness = possibility, transition = immanent reflection = necessary actuality.',
      [
        'shining',
        'identity of form',
        'unity of possibility and actuality',
        'becoming',
        'contingency',
        'coming-to-be',
        'passing-away',
        'immediacy',
        'connection',
        'immediate conversion',
        'existents',
        'being',
        'shine',
        'relation',
        'identical terms',
        'shining in one another',
        'reflection',
        'movement of accidentality',
        'mutual reflective shine',
        'categories of being',
        'reflective determinations of essence',
        'immediate something',
        'content',
        'reflected indifference',
        'form',
        'determinate',
        'determinateness of being',
        'passes into other',
        'quality',
        'determinateness of reflection',
        'indifferent diversity',
        'opposition',
        'ground',
        'nothing',
        'immanent reflection',
        'reflection sublates itself',
        'reflected in-itselfness',
        'possibility',
        'transition',
        'necessary actuality',
      ],
      { section: 'The Relation of Substantiality', order: 2 }
    ),

    createTopicMapEntry(
      'sub-a-3-actuosity-substance',
      'Actuosity of substance — tranquil coming forth',
      [59, 67],
      'Movement of accidentality = actuosity of substance (tranquil coming forth of itself). Not active against something, only against itself (simple unresisting element). Sublating presupposition = disappearing shine. In sublating immediate, immediate comes to be = shining. Beginning from itself = positing of itself from which beginning is made.',
      [
        'movement of accidentality',
        'actuosity of substance',
        'tranquil coming forth',
        'not active against something',
        'only against itself',
        'simple unresisting element',
        'sublating presupposition',
        'disappearing shine',
        'sublating immediate',
        'immediate comes to be',
        'shining',
        'beginning from itself',
        'positing of itself',
        'beginning made',
      ],
      { section: 'The Relation of Substantiality', order: 3 }
    ),

    createTopicMapEntry(
      'sub-a-4-substance-totality-power',
      'Substance as totality and power — creative and destructive',
      [69, 101],
      'Substance = identity of reflective shining = totality embracing accidentality, accidentality = whole substance. Differentiation into simple identity of being and flux of accidents = one form of shining. Simple being = formless substance of imagination (shine not determined as shine, indeterminate identity, no truth, determinateness of immediate actuality/in-itselfness/possibility, falls into accidentality). Flux of accidents = absolute form-unity of accidentality = substance as absolute power. Ceasing-to-be of accident = return as actuality into itself (in-itself/possibility), in-itself = positedness = actuality, form determinations = content determinations, possible = actual differently determined. Substance manifests as creative power (through actuality, translates possible) and destructive power (through possibility, reduces actual). Creating = destructive, destructing = creative (negative/positive, possibility/negativity absolutely united).',
      [
        'substance',
        'identity of reflective shining',
        'totality',
        'embraces accidentality',
        'accidentality',
        'whole substance',
        'differentiation',
        'simple identity of being',
        'flux of accidents',
        'one form of shining',
        'simple being',
        'formless substance of imagination',
        'shine not determined as shine',
        'indeterminate identity',
        'no truth',
        'determinateness of immediate actuality',
        'in-itselfness',
        'possibility',
        'form determinations',
        'fall into accidentality',
        'flux of accidents',
        'absolute form-unity of accidentality',
        'substance as absolute power',
        'ceasing-to-be of accident',
        'return as actuality',
        'into itself',
        'in-itself',
        'positedness',
        'actuality',
        'form determinations',
        'content determinations',
        'possible',
        'actual differently determined',
        'substance manifests',
        'creative power',
        'through actuality',
        'translates possible',
        'destructive power',
        'through possibility',
        'reduces actual',
        'creating',
        'destructive',
        'destructing',
        'creative',
        'negative',
        'positive',
        'possibility',
        'negativity',
        'absolutely united',
      ],
      { section: 'The Relation of Substantiality', order: 4 }
    ),

    createTopicMapEntry(
      'sub-a-5-accidents-no-power',
      'Accidents have no power over each other — substance encompasses',
      [103, 130],
      'Accidents (several, plurality) have no power over each other. They = immediately existent something, concretely existing things, wholes/parts, forces needing reciprocal solicitation. When accidental being seems to exercise power, power = substance (encompasses both, as negativity posits inequality: one ceasing-to-be, another coming-to-be, one into possibility, other into actuality). Ever dividing into form/content, purifying one-sidedness, falling back into determination/division. One accident drives out another because its subsisting = totality of form/content into which both perish.',
      [
        'accidents',
        'several',
        'plurality',
        'have no power over each other',
        'immediately existent something',
        'concretely existing things',
        'wholes',
        'parts',
        'self-subsisting parts',
        'forces',
        'reciprocal solicitation',
        'conditioning each other',
        'accidental being',
        'exercise power',
        'power',
        'substance',
        'encompasses both',
        'negativity',
        'posits inequality',
        'ceasing-to-be',
        'coming-to-be',
        'different content',
        'passing over into possibility',
        'passing over into actuality',
        'ever dividing',
        'difference of form and content',
        'ever purifying',
        'one-sidedness',
        'falling back',
        'determination',
        'division',
        'one accident drives out another',
        'subsisting',
        'totality of form and content',
        'perish',
      ],
      { section: 'The Relation of Substantiality', order: 5 }
    ),

    createTopicMapEntry(
      'sub-a-6-no-real-difference',
      'No real difference — substance as power mediates',
      [132, 155],
      'Because immediate identity/presence of substance in accidents, no real difference. Substance not yet manifested according to whole concept. When substance (self-identical being-in-and-for-itself) differentiated from itself as totality of accidents, substance as power mediates difference. Power = necessity (positive persistence of accidents in negativity, mere positedness in subsistence). Middle = unity of substantiality/accidentality, extremes have no subsistence. Substantiality = relation immediately vanishing, refers to itself not as negative, immediate unity of power with itself, form only of identity (not negative essence). Moment of negativity/difference vanishes; moment of identity does not.',
      [
        'immediate identity',
        'presence of substance',
        'in accidents',
        'no real difference',
        'substance not yet manifested',
        'whole concept',
        'self-identical being-in-and-for-itself',
        'differentiated from itself',
        'totality of accidents',
        'substance as power',
        'mediates difference',
        'power',
        'necessity',
        'positive persistence',
        'accidents in negativity',
        'mere positedness',
        'subsistence',
        'middle',
        'unity of substantiality and accidentality',
        'extremes',
        'no subsistence',
        'substantiality',
        'relation immediately vanishing',
        'refers to itself',
        'not as negative',
        'immediate unity of power with itself',
        'form only of identity',
        'not negative essence',
        'moment of negativity',
        'moment of difference',
        'vanishes',
        'moment of identity',
        'does not vanish',
      ],
      { section: 'The Relation of Substantiality', order: 6 }
    ),

    createTopicMapEntry(
      'sub-a-7-substance-not-substance',
      'Substance not substance as substance — passes to causality',
      [157, 178],
      'Shine/accidentality in itself = substance by virtue of power, but not posited as self-identical shine. Substance has only accidentality (not itself) for shape/positedness; not substance as substance. Relation of substantiality = substance manifests as formal power (differences not substantial). Substance only is as inner of accidents, accidents only are in substance. Relation = shining of totality as becoming, equally reflection, accidentality posited, determined as self-referring negativity/identity, substance in and for itself endowed with power. Relation of substantiality passes over into causality.',
      [
        'shine',
        'accidentality',
        'in itself',
        'substance',
        'by virtue of power',
        'not posited',
        'self-identical shine',
        'substance has only accidentality',
        'not itself',
        'shape',
        'positedness',
        'not substance as substance',
        'relation of substantiality',
        'substance manifests',
        'formal power',
        'differences not substantial',
        'substance only is as inner of accidents',
        'accidents only are in substance',
        'shining of totality as becoming',
        'reflection',
        'accidentality posited',
        'determined as self-referring negativity',
        'self-referring simple identity',
        'substance exists in and for itself',
        'endowed with power',
        'passes over into causality',
      ],
      { section: 'The Relation of Substantiality', order: 7 }
    ),
  ],
  {
    sectionDescription: 'The Relation of Substantiality - Absolute necessity as absolute relation, substance as being that is because it is. Shining as becoming, movement of accidentality. Actuosity of substance, tranquil coming forth. Substance as totality and power, creative and destructive. Accidents have no power over each other, substance encompasses. No real difference, substance as power mediates. Substance not substance as substance, passes to causality.',
  }
);

