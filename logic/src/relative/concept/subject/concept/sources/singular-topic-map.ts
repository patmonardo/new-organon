/**
 * TopicMap for Singular.txt - The Singular Concept
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
 * - singular-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const SINGULAR_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/concept/sources/singular.txt',
  'Hegel\'s Science of Logic - The Concept',
  'The Singular Concept',
  [
    createTopicMapEntry(
      'sing-1-posited-through-particularity',
      'Singularity Posited Through Particularity; Determinate Universality',
      [4, 8],
      'Singularity already posited through particularity. = determinate universality. = self-referring determinateness. = the determinate determinate.',
      [
        'singularity',
        'posited',
        'particularity',
        'determinate universality',
        'self-referring determinateness',
        'determinate determinate',
      ],
      { section: 'The Singular Concept', order: 1 }
    ),

    createTopicMapEntry(
      'sing-2-reflection-self-mediation',
      'Reflection of Concept into Itself; Self-Mediation; Abstraction\'s False Start',
      [10, 41],
      'Singularity appears as reflection of concept out of determinateness into itself. = concept\'s self-mediation. Restores itself as self-equal in absolute negativity. Negative in universal = doubly reflective shine. Turning back twofold: abstraction (false start, surface void of content) vs singularity (true way, depth where concept grasps itself, posited as concept).',
      [
        'reflection into itself',
        'self-mediation',
        'otherness',
        'absolute negativity',
        'doubly reflective shine',
        'inward',
        'outward',
        'abstraction',
        'false start',
        'depth',
        'posited as concept',
      ],
      { section: 'The Singular Concept', order: 2 }
    ),

    createTopicMapEntry(
      'sing-3-total-concept',
      'Universality and Particularity as Total Concept',
      [43, 51],
      'Universality and particularity appeared as moments of becoming of singularity. But the two are in themselves the total concept. In singularity they do not pass over into other. What is posited in singularity = what they are in and for themselves.',
      [
        'moments of becoming',
        'total concept',
        'pass over',
        'in and for themselves',
      ],
      { section: 'The Singular Concept', order: 3 }
    ),

    createTopicMapEntry(
      'sing-4-abstract-universal-void',
      'Abstract Universal Void of Concept; Singularity as Principle of Individuality',
      [52, 73],
      'Universal is for itself because absolute mediation in itself. Abstract universal = sublating is external act. Negativity remains outside as mere condition. Universal does not have singularity in itself, remains void of concept. Life, spirit, God, pure concept = beyond grasp of abstraction. Singularity = principle of individuality and personality. Abstraction = lifeless universalities, void of spirit, color, content.',
      [
        'absolute mediation',
        'absolute negativity',
        'abstract universal',
        'external act',
        'dropping off',
        'mere condition',
        'void of concept',
        'individuality',
        'personality',
        'lifeless',
      ],
      { section: 'The Singular Concept', order: 4 }
    ),

    createTopicMapEntry(
      'sing-5-unity-indissoluble',
      'Unity Indissoluble; Products of Abstraction are Singulars',
      [75, 101],
      'Unity of concept so indissoluble that products of abstraction (supposed to drop singularity) are themselves singulars. Abstraction grasps universal as determinate universality = precisely singularity. Abstraction = partitioning, isolating. Difference: singularity of abstraction\'s products (content/form) vs singularity of concept (absolute form, totality). Product of abstraction = concrete (opposite of supposed).',
      [
        'indissoluble',
        'products of abstraction',
        'determinate universality',
        'partitioning',
        'isolating',
        'content and form',
        'absolute form',
        'totality of form',
        'concrete',
      ],
      { section: 'The Singular Concept', order: 5 }
    ),

    createTopicMapEntry(
      'sing-6-particular-singular-determinate',
      'Particular and Singular as Determinate Universal',
      [103, 121],
      'Particular (because determinate universal) = also singular. Conversely: singular = equally particular. Concept has three particular determinations (universal, particular, singular). Singularity = turning of concept as negative back to itself. Turning back from abstraction can be placed as indifferent moment alongside others.',
      [
        'determinate universal',
        'three particular determinations',
        'species',
        'turning back',
        'negative',
        'sublated',
        'indifferent moment',
      ],
      { section: 'The Singular Concept', order: 6 }
    ),

    createTopicMapEntry(
      'sing-7-particularity-totality-syllogism',
      'Particularity as Totality; Middle Term of Formal Syllogism',
      [123, 137],
      'If singularity listed as one particular determination: particularity = totality embracing them all. As totality = concretion of determinations = singularity itself. Also concrete as determinate universality. = immediate unity in which none of moments posited as distinct. In this form = middle term of formal syllogism.',
      [
        'totality',
        'embracing',
        'concretion',
        'immediate unity',
        'middle term',
        'formal syllogism',
      ],
      { section: 'The Singular Concept', order: 7 }
    ),

    createTopicMapEntry(
      'sing-8-dissolution-inseparability',
      'Dissolution of Determinations; Inseparability Posited; Each is Totality',
      [139, 177],
      'Each determination immediately dissolved itself, lost itself in its other. Only representational thinking holds them rigidly apart (relies on quantity - inappropriate). In singularity: inseparability of determinations posited. As negation of negation, singularity contains opposition and unity. Singularity = negativity of determinations. Each distinct determination = the totality. Turning back = determination is to be in its determinateness the whole concept.',
      [
        'dissolved',
        'lost itself',
        'confounded',
        'representational thinking',
        'inseparability',
        'negation of negation',
        'opposition',
        'unity',
        'negativity',
        'being-in-and-for-itself',
        'totality',
        'whole concept',
      ],
      { section: 'The Singular Concept', order: 8 }
    ),

    createTopicMapEntry(
      'sing-9-immediate-loss-actuality',
      'Singularity as Immediate Loss; Concept Steps into Actuality',
      [179, 198],
      'Singularity = not only turning back but immediate loss of concept. Through singularity (internal to itself), concept becomes external to itself and steps into actuality. Abstraction = soul of singularity, immanent in universal and particular. These concreted through it, become content, singular. As negativity, singularity = determinate determinateness, differentiation. Determining of particular occurs only by virtue of singularity.',
      [
        'immediate loss',
        'external to itself',
        'steps into actuality',
        'soul of singularity',
        'immanent',
        'concreted',
        'determinate determinateness',
        'differentiation',
        'fixed',
        'posited abstraction',
      ],
      { section: 'The Singular Concept', order: 9 }
    ),

    createTopicMapEntry(
      'sing-10-qualitative-one-this',
      'Singular as Qualitative One; This; Exclusive',
      [200, 227],
      'Singular = self-referring negativity = immediate identity of negative with itself. Exists for itself. = abstraction determining concept as immediate. Singular = one which is qualitative, or a this. First: repulsion of itself from itself. Second: negative reference to others (exclusive). Universality referred to singulars = only commonality. Lowest conception = external relation as mere commonality.',
      [
        'self-referring negativity',
        'immediate identity',
        'exists for itself',
        'qualitative',
        'this',
        'repulsion',
        'exclusive',
        'commonality',
        'immediacy of being',
        'external relation',
      ],
      { section: 'The Singular Concept', order: 10 }
    ),

    createTopicMapEntry(
      'sing-11-this-posited-immediacy',
      'This as Posited Immediacy; Reflective Mediation; Positive Connection',
      [229, 248],
      'Singular in reflective sphere = as a this. Does not have excluding reference (qualitative being-for-itself). This = one reflected into itself, without repulsion. Repulsion in reflection one with abstraction (reflective mediation). Makes it posited immediacy pointed at by someone external. Singular also a this but does not have mediation outside it. Itself repelling separation, posited abstraction. Yet, precisely in separation, positive connection.',
      [
        'reflective sphere',
        'concrete existence',
        'this',
        'excluding reference',
        'reflected into itself',
        'reflective mediation',
        'posited immediacy',
        'pointing at',
        'positive connection',
      ],
      { section: 'The Singular Concept', order: 11 }
    ),

    createTopicMapEntry(
      'sing-12-self-subsistent-differences',
      'Singular as Self-Subsistent Differences; Essential Relation',
      [250, 266],
      'Act of abstraction by singular = immanent reflection of difference. = first positing of differences as self-subsisting, reflected into themselves. They exist immediately. But separating = reflection in general (reflective shining). Differences stand in essential relation. Not singulars just existing next to each other (plurality belongs to being). Singularity posits itself in difference of concept. Excludes universal but refers to it essentially.',
      [
        'immanent reflection',
        'self-subsisting',
        'reflected into themselves',
        'essential relation',
        'difference of concept',
        'excludes',
        'refers essentially',
      ],
      { section: 'The Singular Concept', order: 12 }
    ),

    createTopicMapEntry(
      'sing-13-posited-as-judgment',
      'Concept Posited as Judgment; Absolute Originative Partition',
      [268, 283],
      'Concept as connection of self-subsistent determinations has lost itself. Concept no longer posited unity; these no longer moments but subsist in and for themselves. As singularity, concept returns in determinateness into itself. Determinate has itself become totality. Concept\'s turning back = absolute, originative partition of itself. As singularity it is posited as judgment.',
      [
        'lost itself',
        'self-subsistent',
        'moments',
        'returns into itself',
        'totality',
        'absolute originative partition',
        'posited as judgment',
      ],
      { section: 'The Singular Concept', order: 13 }
    ),
  ],
  {
    sectionDescription: 'The Singular Concept - Determinate universality, self-referring determinateness. Reflection into itself, self-mediation. Abstraction\'s false start vs singularity as depth. Concept becomes external to itself, steps into actuality. Posited as judgment (start of Logic).',
  }
);

