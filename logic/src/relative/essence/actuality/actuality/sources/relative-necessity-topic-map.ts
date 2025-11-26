/**
 * TopicMap for relative-necessity.txt - Relative Necessity or Real Actuality, Possibility, and Necessity
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
 * - relative-necessity-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const RELATIVE_NECESSITY_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/actuality/sources/relative-necessity.txt',
  'Hegel\'s Science of Logic - Actuality',
  'Relative Necessity or Real Actuality, Possibility, and Necessity',
  [
    createTopicMapEntry(
      'act-b-1-formal-necessity-real-actuality',
      'Formal necessity — real actuality',
      [3, 18],
      'Necessity = formal (moments formal, simple determinations, totality as immediate unity/conversion, lack self-subsistence). Unity = simple, indifferent to differences. Necessity = actuality (unity indifferent to form determinations = has content). Content = indifferent identity, contains form as indifferent (variety of determinations) = manifold content. This actuality = real actuality.',
      [
        'necessity',
        'formal',
        'moments formal',
        'simple determinations',
        'totality',
        'immediate unity',
        'immediate conversion',
        'lack self-subsistence',
        'unity simple',
        'indifferent to differences',
        'actuality',
        'unity indifferent to form determinations',
        'has content',
        'indifferent identity',
        'form as indifferent',
        'variety of determinations',
        'manifold content',
        'real actuality',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 1 }
    ),

    createTopicMapEntry(
      'act-b-2-real-actuality-can-act',
      'Real actuality can act — manifestation',
      [20, 38],
      'Real actuality = thing of many properties, concretely existing world. Not concrete existence dissolving into appearance, but actuality = in-itself and immanent reflection, preserves itself in manifoldness, externality = inner relating to itself. What is actual can act, announces actuality by what it produces. Relating to other = manifestation (not transition, not appearing). Self-subsistent with immanent reflection/determinate essentiality in another self-subsistent.',
      [
        'real actuality',
        'thing of many properties',
        'concretely existing world',
        'concrete existence',
        'dissolves into appearance',
        'in-itself',
        'immanent reflection',
        'preserves itself',
        'manifoldness',
        'externality',
        'inner relating to itself',
        'what is actual can act',
        'announces actuality',
        'what it produces',
        'relating to other',
        'manifestation',
        'not transition',
        'not appearing',
        'self-subsistent',
        'determinate essentiality',
        'another self-subsistent',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 2 }
    ),

    createTopicMapEntry(
      'act-b-3-real-actuality-has-possibility',
      'Real actuality has possibility — distinguished',
      [40, 45],
      'Real actuality has possibility immediately present, contains moment of in-itself. But only immediate unity, in one determination of form, distinguished as immediate existent from in-itself/possibility.',
      [
        'real actuality',
        'possibility immediately present',
        'moment of in-itself',
        'immediate unity',
        'one determination of form',
        'distinguished',
        'immediate existent',
        'in-itself',
        'possibility',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 3 }
    ),

    createTopicMapEntry(
      'act-b-4-real-possibility-content',
      'Real possibility — in-itself full of content',
      [47, 55],
      'Possibility as in-itself of real actuality = real possibility, in-itself full of content. Formal possibility = immanent reflection as abstract identity, absence of contradiction. But delving into determinations/circumstances/conditions = real possibility (not formal).',
      [
        'possibility',
        'in-itself of real actuality',
        'real possibility',
        'in-itself full of content',
        'formal possibility',
        'immanent reflection',
        'abstract identity',
        'absence of contradiction',
        'delving into determinations',
        'circumstances',
        'conditions',
        'real possibility',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 4 }
    ),

    createTopicMapEntry(
      'act-b-5-real-possibility-circumstances',
      'Real possibility as circumstances — totality of conditions',
      [57, 117],
      'Real possibility = immediate concrete existence (not because formal moment opposite, but because determination of being real possibility). Real possibility of fact = immediately existent manifoldness of circumstances. Manifoldness = both possibility and actuality, identity = content indifferent to form determinations, constitute form. Immediate real actuality determined against possibility = real possibility. Real possibility = posited whole of form (actuality as formal/immediate, possibility as abstract in-itself). Actuality constituting possibility = in-itself of other actual, ought to be sublated, only possibility. Real possibility = totality of conditions, dispersed actuality, in-itself of other, intended to return to itself.',
      [
        'real possibility',
        'immediate concrete existence',
        'formal moment',
        'determination of being real possibility',
        'real possibility of fact',
        'immediately existent manifoldness',
        'circumstances',
        'manifoldness',
        'possibility',
        'actuality',
        'identity',
        'content',
        'indifferent to form determinations',
        'constitute form',
        'immediate real actuality',
        'determined against possibility',
        'posited whole of form',
        'actuality as formal',
        'actuality as immediate',
        'possibility as abstract in-itself',
        'actuality constituting possibility',
        'in-itself of other actual',
        'ought to be sublated',
        'only possibility',
        'totality of conditions',
        'dispersed actuality',
        'in-itself of other',
        'intended to return to itself',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 5 }
    ),

    createTopicMapEntry(
      'act-b-6-real-possibility-contradiction',
      'Real possibility — contradiction, completeness of conditions',
      [88, 159],
      'Really possible = formally identical according to in-itself, free of contradiction (simple content). But self-identical must not contradict developed circumstances. But manifold in itself, manifold connection, variety passes into opposition = contradictory. To demonstrate contradiction: fasten on multiplicity. Contradiction not function of comparing; manifold sublates itself, founders to ground, determination = only possibility. When all conditions present = fact actually there. Completeness = totality in content, fact = content equally actual/possible. In conditioned ground: conditions have form outside. Here: immediate actuality = itself possibility (not determined by presupposing reflection).',
      [
        'really possible',
        'formally identical',
        'in-itself',
        'free of contradiction',
        'simple content',
        'self-identical',
        'must not contradict',
        'developed circumstances',
        'manifold in itself',
        'manifold connection',
        'variety',
        'passes into opposition',
        'contradictory',
        'demonstrate contradiction',
        'fasten on multiplicity',
        'contradiction not function of comparing',
        'manifold sublates itself',
        'founders to ground',
        'determination',
        'only possibility',
        'all conditions present',
        'fact actually there',
        'completeness',
        'totality in content',
        'fact',
        'content equally actual possible',
        'conditioned ground',
        'conditions have form outside',
        'immediate actuality',
        'itself possibility',
        'presupposing reflection',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 6 }
    ),

    createTopicMapEntry(
      'act-b-7-self-sublating-real-possibility',
      'Self-sublating real possibility — self-rejoining',
      [126, 199],
      'Self-sublating real possibility = twofold sublated (actuality and possibility). (1) Actuality formal/concrete existence, through sublating = reflected being, moment of other, in-itself. (2) Concrete existence as possibility/in-itself of other, sublates itself, passes into actuality. Movement produces same moments, each from other; not transition but self-rejoining. Formal possibility: other also possible. Real possibility: no other over against it (itself also actuality). Circle of conditions sublates itself, makes itself into in-itselfness (in-itself of other). Moment of in-itselfness sublates itself, becomes actuality. Disappears: actuality as possibility of other, possibility as actuality not of which it is possibility.',
      [
        'self-sublating real possibility',
        'twofold sublated',
        'actuality',
        'possibility',
        'actuality formal',
        'concrete existence',
        'through sublating',
        'reflected being',
        'moment of other',
        'in-itself',
        'concrete existence as possibility',
        'in-itself of other',
        'sublates itself',
        'passes into actuality',
        'movement',
        'produces same moments',
        'each from other',
        'not transition',
        'self-rejoining',
        'formal possibility',
        'other also possible',
        'real possibility',
        'no other over against it',
        'itself also actuality',
        'circle of conditions',
        'sublates itself',
        'makes itself into in-itselfness',
        'in-itself of other',
        'moment of in-itselfness',
        'becomes actuality',
        'disappears',
        'actuality as possibility of other',
        'possibility as actuality',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 7 }
    ),

    createTopicMapEntry(
      'act-b-8-real-necessity',
      'Real necessity — cannot be otherwise',
      [162, 228],
      'Negation of real possibility = self-identity. In sublating, recoiling within itself = real necessity. Necessary cannot be otherwise; only possible can be (possibility = in-itself, positedness, otherness). Formal possibility = transition into other. Real possibility has actuality within = already necessity. Really possible cannot be otherwise; under conditions, nothing else can follow. Real possibility and necessity apparently distinguished; identity already presupposed. Real possibility = connection full of content (identity existing in itself, indifferent to form).',
      [
        'negation of real possibility',
        'self-identity',
        'sublating',
        'recoiling within itself',
        'real necessity',
        'necessary',
        'cannot be otherwise',
        'only possible',
        'can be',
        'possibility',
        'in-itself',
        'positedness',
        'otherness',
        'formal possibility',
        'transition into other',
        'real possibility has actuality within',
        'already necessity',
        'really possible',
        'under conditions',
        'nothing else can follow',
        'real possibility and necessity',
        'apparently distinguished',
        'identity already presupposed',
        'real possibility',
        'connection full of content',
        'identity existing in itself',
        'indifferent to form',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 8 }
    ),

    createTopicMapEntry(
      'act-b-9-relative-necessity',
      'Relative necessity — begins from contingent',
      [187, 260],
      'Necessity = relative (has presupposition, begins from contingent). Real actual = determinate actual, determinateness as immediate being = multiplicity of circumstances. Immediate being as determinateness = negative of itself = in-itself/possibility = real possibility. Unity of two moments = totality of form (external to itself). Unity: (1) manifold concrete existence = possibility immediately/positively (possible because actual), (2) possibility posited = only possibility, conversion into opposite = contingency. Possibility in condition = in-itself of other. In-itself sublates itself, positedness posited = real possibility becomes necessity. But begins from unity not reflected into itself; presupposing and movement separate; necessity not determined out of itself into contingency.',
      [
        'necessity',
        'relative',
        'has presupposition',
        'begins from contingent',
        'real actual',
        'determinate actual',
        'determinateness',
        'immediate being',
        'multiplicity of circumstances',
        'immediate being as determinateness',
        'negative of itself',
        'in-itself',
        'possibility',
        'real possibility',
        'unity of two moments',
        'totality of form',
        'external to itself',
        'manifold concrete existence',
        'possibility immediately',
        'possibility positively',
        'possible because actual',
        'possibility posited',
        'only possibility',
        'conversion into opposite',
        'contingency',
        'possibility in condition',
        'in-itself of other',
        'in-itself sublates itself',
        'positedness posited',
        'real possibility becomes necessity',
        'begins from unity not reflected',
        'presupposing',
        'movement separate',
        'necessity not determined out of itself',
        'into contingency',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 9 }
    ),

    createTopicMapEntry(
      'act-b-10-relativity-manifested',
      'Relativity manifested — limited actuality',
      [215, 222],
      'Relativity manifested in content: content = identity indifferent to form, distinct from it, determinate content. Necessary reality = limited actuality, because limitation = also contingent in some respect.',
      [
        'relativity',
        'manifested in content',
        'content',
        'identity indifferent to form',
        'distinct from it',
        'determinate content',
        'necessary reality',
        'limited actuality',
        'limitation',
        'also contingent',
        'some respect',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 10 }
    ),

    createTopicMapEntry(
      'act-b-11-real-necessity-contains-contingency',
      'Real necessity contains contingency — absolute actuality',
      [224, 241],
      'Real necessity in itself = also contingency. Apparent because: necessary according to form, limited according to content, derives contingency through content. Contingency also in form: real possibility necessary only in itself, as posited = mutual otherness of actuality/possibility. Real necessity contains contingency; turning back from restless being-other, but not from itself to itself. Unity of necessity and contingency = absolute actuality.',
      [
        'real necessity',
        'in itself',
        'also contingency',
        'apparent because',
        'necessary according to form',
        'limited according to content',
        'derives contingency through content',
        'contingency also in form',
        'real possibility',
        'necessary only in itself',
        'as posited',
        'mutual otherness',
        'actuality',
        'possibility',
        'real necessity contains contingency',
        'turning back',
        'restless being-other',
        'not from itself to itself',
        'unity',
        'necessity',
        'contingency',
        'absolute actuality',
      ],
      { section: 'Relative Necessity or Real Actuality, Possibility, and Necessity', order: 11 }
    ),
  ],
  {
    sectionDescription: 'Relative Necessity or Real Actuality, Possibility, and Necessity - Formal necessity results in real actuality. Real actuality can act, manifests itself. Real actuality has possibility, distinguished. Real possibility is in-itself full of content. Real possibility as circumstances, totality of conditions. Real possibility contains contradiction, completeness of conditions. Self-sublating real possibility, self-rejoining. Real necessity, cannot be otherwise. Relative necessity, begins from contingent. Relativity manifested, limited actuality. Real necessity contains contingency, absolute actuality.',
  }
);

