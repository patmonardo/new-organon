/**
 * TopicMap for reciprocity-action.txt - Reciprocity of Action
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
 * - reciprocity-action-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 *
 * SIGNIFICANCE: This section completes the Sphere of Essence and transitions to the Concept
 * (subjectivity/freedom). As Hegel says, what appears is moral - Dharma in the Sphere of Essence
 * as Absolute Relation has its truth in the Idea that is being Projected.
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const RECIPROCITY_ACTION_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/substance/sources/reciprocity-action.txt',
  'Hegel\'s Science of Logic - The Absolute Relation',
  'Reciprocity of Action',
  [
    createTopicMapEntry(
      'sub-c-1-mechanism-sublated',
      'Mechanism sublated — reciprocity',
      [3, 17],
      'Finite causality: substances actively relate. Mechanism = externality of causality (cause\'s reflection in effect = repelling being, self-identity = external to itself, effect transposed into another substance). Reciprocity of action sublates mechanism: (1) disappearing of original persistence of immediate substantiality, (2) coming to be of cause, originariness mediating itself through negation.',
      [
        'finite causality',
        'substances',
        'actively relate',
        'mechanism',
        'externality of causality',
        'cause\'s reflection in effect',
        'repelling being',
        'self-identity',
        'external to itself',
        'effect transposed',
        'another substance',
        'reciprocity of action',
        'sublates mechanism',
        'disappearing',
        'original persistence',
        'immediate substantiality',
        'coming to be of cause',
        'originariness',
        'mediating itself',
        'through negation',
      ],
      { section: 'Reciprocity of Action', order: 1 }
    ),

    createTopicMapEntry(
      'sub-c-2-reciprocal-causality',
      'Reciprocal causality — active and passive',
      [19, 60],
      'Reciprocity = reciprocal causality of substances (presupposed, condition each other). Each = both active and passive. Difference sublated, totally transparent reflective shine, substances = identity of active/passive. Reciprocity = empty way and manner, external bringing together. No longer substrates but substances. Presupposed immediacy sublated, conditioning = influence/passivity. Influence not from another substance but from causality conditioned by influence/mediated. External factor = passivity mediated through causality, produced through own activity, passivity posited by own activity. Causality = conditioned and conditioning (passive as both). Conditioning/passivity = negation of cause through itself (makes itself effect, is cause). Reciprocity = causality itself; cause in effect refers back to itself.',
      [
        'reciprocity',
        'reciprocal causality',
        'substances',
        'presupposed',
        'condition each other',
        'active',
        'passive',
        'both active and passive',
        'difference sublated',
        'totally transparent reflective shine',
        'identity of active and passive',
        'empty way and manner',
        'external bringing together',
        'no longer substrates',
        'substances',
        'presupposed immediacy sublated',
        'conditioning',
        'influence',
        'passivity',
        'influence not from another substance',
        'causality conditioned by influence',
        'mediated',
        'external factor',
        'passivity mediated through causality',
        'produced through own activity',
        'passivity posited by own activity',
        'causality',
        'conditioned',
        'conditioning',
        'passive as both',
        'conditioning or passivity',
        'negation of cause through itself',
        'makes itself effect',
        'is cause',
        'reciprocity of action',
        'causality itself',
        'cause in effect refers back to itself',
      ],
      { section: 'Reciprocity of Action', order: 2 }
    ),

    createTopicMapEntry(
      'sub-c-3-concept-attained',
      'Concept attained — necessity to freedom',
      [62, 81],
      'Causality returned to absolute concept, attained concept itself. At first = real necessity (absolute self-identity, differences = substances/free actualities). Necessity = inner identity; causality = manifestation (reflective shine of substantial otherness sublated), necessity elevated to freedom. In reciprocity: originative causality displays as arising from negation/passivity, passing away into it = becoming (only shining), transition into otherness = reflection-into-itself, negation (ground of cause) = positive rejoining with itself.',
      [
        'causality',
        'returned to absolute concept',
        'attained concept itself',
        'real necessity',
        'absolute self-identity',
        'differences',
        'substances',
        'free actualities',
        'over against one another',
        'necessity',
        'inner identity',
        'causality',
        'manifestation',
        'reflective shine of substantial otherness',
        'sublated',
        'necessity elevated to freedom',
        'reciprocity of action',
        'originative causality',
        'displays itself',
        'arising from negation',
        'arising from passivity',
        'passing away into it',
        'becoming',
        'only shining',
        'transition into otherness',
        'reflection-into-itself',
        'negation',
        'ground of cause',
        'positive rejoining with itself',
      ],
      { section: 'Reciprocity of Action', order: 3 }
    ),

    createTopicMapEntry(
      'sub-c-4-necessity-causality-disappeared',
      'Necessity and causality disappeared — concept',
      [83, 119],
      'In reciprocity: necessity/causality disappeared. Contain: immediate identity (combination/reference), absolute substantiality of differences, contingency, original unity of substantial difference, absolute contradiction. Necessity = being (because being is), unity with itself (has itself as ground), but has ground = not being, reflective shining/reference/mediation. Causality = posited transition (original being/cause into reflective shine/positedness, positedness into originariness). Identity of being/reflective shine = inner necessity. Inwardness/in-itself sublates movement of causality; substantiality lost, necessity unveils itself. Necessity to freedom: inner identity manifested, identical movement immanent, immanent reflection of shine. Contingency to freedom: sides of necessity (independent free actualities, not shining into each other) posited as identity, totalities of immanent reflection shine as identical, one same reflection.',
      [
        'reciprocity of action',
        'necessity disappeared',
        'causality disappeared',
        'immediate identity',
        'combination',
        'reference',
        'absolute substantiality of differences',
        'contingency',
        'original unity of substantial difference',
        'absolute contradiction',
        'necessity',
        'being',
        'because being is',
        'unity of being with itself',
        'has itself as ground',
        'has ground',
        'not being',
        'reflective shining',
        'reference',
        'mediation',
        'causality',
        'posited transition',
        'original being',
        'cause',
        'reflective shine',
        'mere positedness',
        'positedness',
        'originariness',
        'identity of being and reflective shine',
        'inner necessity',
        'inwardness',
        'in-itself',
        'sublates movement of causality',
        'substantiality lost',
        'necessity unveils itself',
        'necessity to freedom',
        'not by vanishing',
        'inner identity manifested',
        'identical movement immanent',
        'immanent reflection of shine',
        'contingency to freedom',
        'sides of necessity',
        'independent free actualities',
        'not reflectively shine into each other',
        'posited as identity',
        'totalities of immanent reflection',
        'shine as identical',
        'one and the same reflection',
      ],
      { section: 'Reciprocity of Action', order: 4 }
    ),

    createTopicMapEntry(
      'sub-c-5-concept-universal-singular',
      'Concept — universal, singular, particular',
      [121, 162],
      'Absolute substance no longer repels as necessity, no longer falls apart as contingency. Differentiates: (1) totality (passive substance) = at origin, reflection from internal determinateness, simple whole containing positedness, posited as self-identical = universal. (2) Totality (causal substance) = reflection from internal determinateness into negative determinateness, whole posited as self-identical negativity = singular. Universal self-identical only in sublated determinateness = negative as negative = same negativity as singular. Singular = determinedly determined, negative as negative = same identity as universal. Simple identity = particularity (from singular: determinateness; from universal: immanent reflection; two in immediate unity). Three totalities = one same reflection (negative self-reference) differentiating into other two (perfectly transparent difference, determinate simplicity, simple determinateness, one same identity) = concept = realm of subjectivity/freedom.',
      [
        'absolute substance',
        'no longer repels',
        'necessity',
        'no longer falls apart',
        'contingency',
        'self-differentiating absolute form',
        'differentiates itself',
        'first totality',
        'passive substance',
        'heretofore passive substance',
        'at the origin',
        'reflection from internal determinateness',
        'simple whole',
        'contains positedness',
        'posited as self-identical',
        'universal',
        'second totality',
        'causal substance',
        'hitherto causal substance',
        'reflection',
        'from internal determinateness',
        'into negative determinateness',
        'whole',
        'posited as self-identical negativity',
        'singular',
        'universal',
        'self-identical',
        'sublated determinateness',
        'negative as negative',
        'same negativity',
        'singularity',
        'singular',
        'determinedly determined',
        'negative as negative',
        'same identity',
        'universality',
        'simple identity',
        'particularity',
        'from singular',
        'moment of determinateness',
        'from universal',
        'immanent reflection',
        'two in immediate unity',
        'three totalities',
        'one and the same reflection',
        'negative self-reference',
        'differentiates itself',
        'other two totalities',
        'perfectly transparent difference',
        'determinate simplicity',
        'simple determinateness',
        'one same identity',
        'concept',
        'realm of subjectivity',
        'realm of freedom',
      ],
      { section: 'Reciprocity of Action', order: 5 }
    ),
  ],
  {
    sectionDescription: 'Reciprocity of Action - Mechanism sublated, reciprocal causality (active and passive), concept attained (necessity to freedom), necessity and causality disappeared, concept (universal, singular, particular) = realm of subjectivity/freedom. This completes the Sphere of Essence and transitions to the Concept. As Hegel says, what appears is moral - Dharma in the Sphere of Essence as Absolute Relation has its truth in the Idea that is being Projected.',
  }
);

