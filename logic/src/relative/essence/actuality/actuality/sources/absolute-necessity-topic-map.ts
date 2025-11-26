/**
 * TopicMap for absolute-necessity.txt - Absolute Necessity
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
 * - absolute-necessity-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const ABSOLUTE_NECESSITY_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/actuality/sources/absolute-necessity.txt',
  'Hegel\'s Science of Logic - Actuality',
  'Absolute Necessity',
  [
    createTopicMapEntry(
      'act-c-1-determinate-necessity',
      'Determinate necessity — contains contingency',
      [3, 16],
      'Real necessity = determinate necessity (formal necessity has no content/determinateness). Determinateness = having negation (contingency) within. In first simplicity: determinateness = actuality, determinate necessity = immediate actual necessity. Actuality necessary, contains necessity as in-itself = absolute actuality (cannot be otherwise, in-itself = necessity itself).',
      [
        'real necessity',
        'determinate necessity',
        'formal necessity',
        'no content',
        'no determinateness',
        'determinateness',
        'having negation',
        'contingency',
        'first simplicity',
        'actuality',
        'immediate actual necessity',
        'necessary',
        'contains necessity as in-itself',
        'absolute actuality',
        'cannot be otherwise',
        'necessity itself',
      ],
      { section: 'Absolute Necessity', order: 1 }
    ),

    createTopicMapEntry(
      'act-c-2-absolute-actuality-empty',
      'Absolute actuality empty — absolute possibility',
      [18, 31],
      'Actuality posited as absolute = unity of itself and possibility = only empty determination = contingency. Emptiness makes it mere possibility (can be other, determined as possibility). But this possibility = absolute possibility (possibility of being equally determined as possibility and actuality). Indifference towards itself = empty contingent determination.',
      [
        'actuality posited as absolute',
        'unity of itself and possibility',
        'empty determination',
        'contingency',
        'emptiness',
        'mere possibility',
        'can be other',
        'determined as possibility',
        'absolute possibility',
        'possibility of being equally determined',
        'possibility and actuality',
        'indifference towards itself',
        'empty contingent determination',
      ],
      { section: 'Absolute Necessity', order: 2 }
    ),

    createTopicMapEntry(
      'act-c-3-real-necessity-becoming',
      'Real necessity becoming — own positing',
      [33, 52],
      'Real necessity contains contingency implicitly, contingency becomes in it. Becoming as externality = in-itself of necessity (immediate determinateness). But also necessity\'s own becoming, presupposition = own positing. Real necessity = sublatedness of actuality into possibility and possibility into actuality. Simple conversion = positive unity (each rejoins itself in other). Actuality = rejoining of form with itself. Negative positing = presupposing, positing of itself as sublated, positing of immediacy.',
      [
        'real necessity',
        'contains contingency implicitly',
        'contingency becomes',
        'becoming as externality',
        'in-itself of necessity',
        'immediate determinateness',
        'necessity\'s own becoming',
        'presupposition',
        'own positing',
        'sublatedness',
        'actuality into possibility',
        'possibility into actuality',
        'simple conversion',
        'positive unity',
        'each rejoins itself',
        'rejoining of form with itself',
        'negative positing',
        'presupposing',
        'positing of itself as sublated',
        'positing of immediacy',
      ],
      { section: 'Absolute Necessity', order: 3 }
    ),

    createTopicMapEntry(
      'act-c-4-necessity-determines-itself',
      'Necessity determines itself as contingency',
      [54, 77],
      'In positing: actuality determined as negative, rejoins itself from real possibility, comes to be out of in-itself/negation. Immediately determined as possibility (mediated by negation). Possibility = mediating (in-itself/possibility/mediating = positedness). Necessity = sublating positedness, positing immediacy/in-itself, determining as positedness. Necessity determines itself as contingency: repels itself from itself, in repelling returns to itself, in turning back repels itself.',
      [
        'positing',
        'actuality determined as negative',
        'rejoins itself',
        'real possibility',
        'comes to be out of in-itself',
        'comes to be out of negation',
        'immediately determined as possibility',
        'mediated by negation',
        'possibility',
        'mediating',
        'in-itself',
        'positedness',
        'necessity',
        'sublating positedness',
        'positing immediacy',
        'positing in-itself',
        'determining as positedness',
        'necessity determines itself as contingency',
        'repels itself from itself',
        'returns to itself',
        'turning back',
      ],
      { section: 'Absolute Necessity', order: 4 }
    ),

    createTopicMapEntry(
      'act-c-5-form-pervaded',
      'Form pervaded — absolute necessity',
      [79, 103],
      'Form pervaded all distinctions, made itself transparent, as absolute necessity = simple self-identity of being in negation/essence. Distinction of content/form vanished. Unity of possibility in actuality and actuality in possibility = form (indifferent in determinateness/positedness) = fact full of content. Necessity = reflected identity of two determinations (indifferent), form determination of in-itself vs positedness, possibility = limitation of content. Resolution of difference = absolute necessity (content = difference penetrating itself).',
      [
        'form pervaded',
        'all distinctions',
        'made itself transparent',
        'absolute necessity',
        'simple self-identity',
        'being in negation',
        'being in essence',
        'distinction of content and form',
        'vanished',
        'unity of possibility in actuality',
        'actuality in possibility',
        'form',
        'indifferent',
        'determinateness',
        'positedness',
        'fact full of content',
        'reflected identity',
        'two determinations',
        'form determination of in-itself',
        'limitation of content',
        'resolution of difference',
        'difference penetrating itself',
      ],
      { section: 'Absolute Necessity', order: 5 }
    ),

    createTopicMapEntry(
      'act-c-6-absolute-necessity-truth',
      'Absolute necessity truth — being and essence',
      [105, 127],
      'Absolute necessity = truth (actuality/possibility, formal/real necessity return). Being in negation/essence refers itself to itself = being. Equally simple immediacy/pure being and simple immanent reflection/pure essence; two are one and same. Absolutely necessary only is because it is (no condition/ground). Equally pure essence (being = simple immanent reflection). As reflection: has ground/condition = itself. In-itself = immediacy, possibility = actuality. Because it is; rejoining of being with itself = essence; immediate simplicity = being.',
      [
        'absolute necessity',
        'truth',
        'actuality',
        'possibility',
        'formal necessity',
        'real necessity',
        'return',
        'being in negation',
        'being in essence',
        'refers itself to itself',
        'being',
        'simple immediacy',
        'pure being',
        'simple immanent reflection',
        'pure essence',
        'two are one and same',
        'absolutely necessary',
        'only is because it is',
        'no condition',
        'no ground',
        'pure essence',
        'simple immanent reflection',
        'as reflection',
        'has ground',
        'has condition',
        'itself',
        'in-itself',
        'immediacy',
        'possibility',
        'actuality',
        'rejoining of being with itself',
        'essence',
        'immediate simplicity',
      ],
      { section: 'Absolute Necessity', order: 6 }
    ),

    createTopicMapEntry(
      'act-c-7-absolute-necessity-form',
      'Absolute necessity form — blind',
      [129, 143],
      'Absolute necessity = reflection/form of absolute, unity of being/essence, simple immediacy = absolute negativity. Differences = existing manifoldness, differentiated actuality (others independently subsisting). Connection = absolute identity, absolute conversion of actuality into possibility and possibility into actuality. Absolute necessity = blind.',
      [
        'absolute necessity',
        'reflection',
        'form of absolute',
        'unity of being and essence',
        'simple immediacy',
        'absolute negativity',
        'differences',
        'not like determinations of reflection',
        'existing manifoldness',
        'differentiated actuality',
        'others',
        'independently subsisting',
        'connection',
        'absolute identity',
        'absolute conversion',
        'actuality into possibility',
        'possibility into actuality',
        'blind',
      ],
      { section: 'Absolute Necessity', order: 7 }
    ),

    createTopicMapEntry(
      'act-c-8-blind-necessity',
      'Blind necessity — free actualities',
      [144, 163],
      'Two terms (actuality/possibility) = shape of immanent reflection as being = free actualities (neither shines in other, no trace of reference, grounded in itself, inherently necessary). Necessity as essence concealed in being; reciprocal contact = empty externality; actuality of one in other = possibility = contingency. Being posited as absolutely necessary = self-mediation, absolute negation of mediation-through-other, identical only with being. Other with actuality in being = merely possible, empty positedness.',
      [
        'two terms',
        'actuality',
        'possibility',
        'shape of immanent reflection as being',
        'free actualities',
        'neither shines in other',
        'no trace of reference',
        'grounded in itself',
        'inherently necessary',
        'necessity as essence',
        'concealed in being',
        'reciprocal contact',
        'empty externality',
        'actuality of one in other',
        'contingency',
        'being posited as absolutely necessary',
        'self-mediation',
        'absolute negation of mediation-through-other',
        'identical only with being',
        'other with actuality in being',
        'merely possible',
        'empty positedness',
      ],
      { section: 'Absolute Necessity', order: 8 }
    ),

    createTopicMapEntry(
      'act-c-9-contingency-absolute-necessity',
      'Contingency is absolute necessity — essence breaks forth',
      [165, 189],
      'Contingency = absolute necessity = essence of free inherently necessary actualities. Essence averse to light (no reflective shining, no reflex, grounded in themselves, shaped for themselves, manifest only to themselves, only being). Essence will break forth, reveal what it is/they are. Simplicity of being, resting on themselves = absolute negativity = freedom of reflectionless immediacy. Negative breaks forth: being through negativity (essence) = self-contradiction, breaks forth as negation of actualities, as nothing, as otherness (free towards them).',
      [
        'contingency',
        'absolute necessity',
        'essence of free actualities',
        'inherently necessary actualities',
        'essence averse to light',
        'no reflective shining',
        'no reflex',
        'grounded in themselves',
        'shaped for themselves',
        'manifest only to themselves',
        'only being',
        'essence will break forth',
        'reveal',
        'simplicity of being',
        'resting on themselves',
        'absolute negativity',
        'freedom of reflectionless immediacy',
        'negative breaks forth',
        'being through negativity',
        'self-contradiction',
        'breaks forth as negation',
        'nothing',
        'otherness',
        'free towards them',
      ],
      { section: 'Absolute Necessity', order: 9 }
    ),

    createTopicMapEntry(
      'act-c-10-mark-necessity',
      'Mark of necessity — actualities perish',
      [190, 221],
      'Negative not to be missed. In self-based shape: indifferent to form, content, different actualities, determinate content. Content = mark necessity impressed (letting go free as absolutely actual), absolute turning back into itself. Mark = witness to necessity\'s right, actualities perish. Manifestation: determinateness in truth = negative self-reference = blind collapse into otherness. In immediate existence: shining/reflection = becoming, transition of being into nothing. But being = essence, becoming = reflection/shining. Externality = inwardness, connection = absolute identity. Transition of actual into possible, being into nothing = self-rejoining. Contingency = absolute necessity = presupposing of first absolute actuality.',
      [
        'negative not to be missed',
        'self-based shape',
        'indifferent to form',
        'content',
        'different actualities',
        'determinate content',
        'mark',
        'necessity impressed',
        'letting go free',
        'absolutely actual',
        'absolute turning back into itself',
        'witness to necessity\'s right',
        'actualities perish',
        'manifestation',
        'determinateness in truth',
        'negative self-reference',
        'blind collapse into otherness',
        'immediate existence',
        'shining',
        'reflection',
        'becoming',
        'transition of being into nothing',
        'being',
        'essence',
        'externality',
        'inwardness',
        'connection',
        'absolute identity',
        'transition of actual into possible',
        'self-rejoining',
        'contingency',
        'presupposing of first absolute actuality',
      ],
      { section: 'Absolute Necessity', order: 10 }
    ),

    createTopicMapEntry(
      'act-c-11-substance',
      'Substance — transition',
      [223, 231],
      'Identity of being with itself in negation = substance. Unity in negation/contingency, relation to itself = substance. Blind transition of necessity = absolute\'s own exposition, movement in itself, in externalization reveals itself.',
      [
        'identity of being with itself',
        'in negation',
        'substance',
        'unity in negation',
        'unity in contingency',
        'relation to itself',
        'blind transition of necessity',
        'absolute\'s own exposition',
        'movement in itself',
        'externalization',
        'reveals itself',
      ],
      { section: 'Absolute Necessity', order: 11 }
    ),
  ],
  {
    sectionDescription: 'Absolute Necessity - Real necessity is determinate necessity, contains contingency. Absolute actuality empty, absolute possibility. Real necessity becoming, own positing. Necessity determines itself as contingency. Form pervaded, absolute necessity. Absolute necessity truth, being and essence. Absolute necessity form, blind. Blind necessity, free actualities. Contingency is absolute necessity, essence breaks forth. Mark of necessity, actualities perish. Substance, transition.',
  }
);

