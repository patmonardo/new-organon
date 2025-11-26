/**
 * TopicMap for Universal.txt - The Universal Concept
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
 * - universal-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const UNIVERSAL_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/concept/sources/universal.txt',
  'Hegel\'s Science of Logic - The Concept',
  'The Universal Concept',
  [
    createTopicMapEntry(
      'univ-1-genesis-absoluteness',
      'Pure Concept: Absolutely Infinite, Unconditioned; Genesis via Being → Essence → Concept',
      [1, 28],
      'Pure concept is absolutely infinite, unconditioned, free. Genesis: Being → Essence → Concept via self-repulsion. Becoming = self-repulsion → unconditional and originative. Being → Essence: reflective shine/positedness, becoming → positing. Essence → Being: positing sublated, restored to original being. Concept = mutual penetration of moments. Qualitative/originative existent = positing + immanent turning back. Pure immanent reflection = becoming-other/determinateness. Determinateness = infinite, self-referring.',
      [
        'pure concept',
        'absolutely infinite',
        'unconditioned',
        'free',
        'genesis',
        'being → essence → concept',
        'self-repulsion',
        'unconditional',
        'originative',
        'reflective shine',
        'positedness',
        'positing',
        'mutual penetration',
        'immanent reflection',
        'becoming-other',
        'determinateness',
        'infinite self-referring',
      ],
      { section: 'The Universal Concept', order: 1 }
    ),

    createTopicMapEntry(
      'univ-2-universality-negation-of-negation',
      'Concept Self-Identity; Universality as Negation of Negation',
      [30, 45],
      'Concept = absolute self-identity = negation of negation = infinite unity of negativity with itself. Universality = pure self-reference via negativity. Universality seems incapable of explanation (simplest determination). Explanation requires determinations/differentiations → would alter the simple. Universal = simple that contains difference/determinateness by absolute negativity (in highest degree).',
      [
        'absolute self-identity',
        'negation of negation',
        'infinite unity of negativity',
        'universality',
        'pure self-reference',
        'simplest determination',
        'absolute negativity',
        'difference and determinateness',
      ],
      { section: 'The Universal Concept', order: 2 }
    ),

    createTopicMapEntry(
      'univ-3-contrast-being-richness',
      'Contrast with Being; Universal as Simple Yet Rich, Identity as Absolute Mediation',
      [46, 63],
      'Being = simple as immediate → can only intend it, cannot say what it is. Being immediately one with its other (non-being) → becoming. Concept of being = so simple it vanishes into opposite immediately. Universal = simple yet all the richer in itself (because it is the concept). First: simple self-reference (only in itself). Second: identity = absolute mediation (but not anything mediated).',
      [
        'being',
        'simple immediate',
        'non-being',
        'becoming',
        'universal',
        'simple yet rich',
        'concept',
        'simple self-reference',
        'absolute mediation',
        'not anything mediated',
      ],
      { section: 'The Universal Concept', order: 3 }
    ),

    createTopicMapEntry(
      'univ-4-abstract-universal-externality',
      'Abstract Universal, Double Negation, and Not-Yet Externality',
      [64, 91],
      'Abstract universal (opposed to particular/singular) - to be discussed with determinate concept. Abstract universal requires leaving aside other determinations of concrete. Determinations = negations; leaving aside = further negating. Abstract universal already contains negation of negation. Double negation misrepresented as external: properties left out vs retained seem different, operation of leaving aside seems external. Universal has not yet acquired determination of externality. Universal still = absolute negation = negation of negation = absolute negativity.',
      [
        'abstract universal',
        'mediated',
        'leaving aside',
        'determinations',
        'negations',
        'double negation',
        'negation of negation',
        'externality',
        'absolute negativity',
      ],
      { section: 'The Universal Concept', order: 4 }
    ),

    createTopicMapEntry(
      'univ-5-persistence-determination',
      'Universal Maintains Itself in Its Determination; Persistence vs Qualitative Perishing',
      [93, 116],
      'First negative (determination) = not restriction for universal. Universal maintains itself in determination; self-identity is positive. Categories of being = identities in restriction/otherness (only implicitly concept, not manifest). Qualitative determination perished in its other; truth = diverse determination. Universal, when positing itself in determination, remains what it is. Universal = soul of concrete (inhabits it). Unhindered, equal to itself in manifoldness/diversity. Not swept away in becoming; persists undisturbed. Power of unalterable, undying self-preservation.',
      [
        'original unity',
        'determination',
        'restriction',
        'maintains itself',
        'positive self-identity',
        'categories of being',
        'qualitative determination',
        'perished',
        'soul of concrete',
        'unhindered',
        'persists',
        'self-preservation',
      ],
      { section: 'The Universal Concept', order: 5 }
    ),

    createTopicMapEntry(
      'univ-6-essence-creative-principle',
      'Not Mere Reflective Shine; Universal as Essence and Creative Principle',
      [118, 155],
      'Universal does not simply shine reflectively (like determination of reflection). Determination of reflection = relative, relating, shines in other, external activity alongside self-subsistence. Universal = essence of its determination = determination\'s own positive nature. Determination (negative of universal) = in concept simply positedness. Determination = negative of negative = self-identity of negative (which is universal). Universal = substance of determinations. What for substance was accident = concept\'s own self-mediation (immanent reflection). Mediation raises accidental to necessity = manifested reference. Concept ≠ abyss of formless substance ≠ inner identity of different things/circumstances. Concept = absolute negativity = informing and creative principle. Determination = not limitation but sublated as determination = positedness. Reflective shine = appearance as appearance of identical.',
      [
        'reflective shine',
        'determination of reflection',
        'relative',
        'relating',
        'essence',
        'positive nature',
        'positedness',
        'negative of negative',
        'substance',
        'accident',
        'self-mediation',
        'immanent reflection',
        'manifested reference',
        'absolute negativity',
        'informing',
        'creative principle',
        'appearance of identical',
      ],
      { section: 'The Universal Concept', order: 6 }
    ),

    createTopicMapEntry(
      'univ-7-free-power-love',
      'Universal as Free Power/Love; Rest in Its Other',
      [157, 168],
      'Universal = free power. Itself while reaching out to other, embracing it, without violence. At rest in its other as in its own. Also = free love and boundless blessedness. Relates to distinct as to itself; in it, returned to itself.',
      [
        'free power',
        'reaching out',
        'embracing',
        'without violence',
        'rest in other',
        'free love',
        'boundless blessedness',
        'returned to itself',
      ],
      { section: 'The Universal Concept', order: 7 }
    ),

    createTopicMapEntry(
      'univ-8-determinateness-totality',
      'Determinateness Within the Universal; Totality vs Abstract Universal',
      [169, 204],
      'Cannot speak of universal apart from determinateness (particularity and singularity). Universal contains determinateness in and for itself (via absolute negativity). Determinateness not imported from outside. As negativity (first immediate negation) → determinateness = particularity. As second universal (negation of negation) → absolute determinateness = singularity and concreteness. Universal = totality of concept = concrete, not empty, has content by virtue of concept. Content in which universal preserves itself = universal\'s own, immanent. Abstract from content → abstract universal (isolated, imperfect moment, void of truth).',
      [
        'determinateness',
        'particularity',
        'singularity',
        'absolute negativity',
        'first negation',
        'second negation',
        'totality of concept',
        'concrete',
        'content',
        'abstract universal',
        'isolated',
        'imperfect',
        'void of truth',
      ],
      { section: 'The Universal Concept', order: 8 }
    ),

    createTopicMapEntry(
      'univ-9-total-reflection-inward-outward',
      'Universal as Total Reflection: Outward and Inward Shining; Determinate Concept as Immanent Character',
      [206, 246],
      'Universal shows itself as totality via determinateness. Determinateness = first negation + reflection of negation into itself. First negation (by itself) → universal = particular (to be considered). Other determinateness → universal still essentially universal. Determinateness in concept = total reflection = doubly reflective shine: outward (reflection into other, establishes distinction, particularity resolved in higher universality) and inward (reflection into itself). Relative universal preserves universality in determinateness (not indifferent, but via inward shining). Determinateness as determinate concept = bent back into itself = concept\'s own immanent character. Character made essential by taken up into universality, pervaded by it (and pervades it). Equal in extension and identical. Character = genus (determinateness not separated from universal). Not outwardly directed limitation, but positive (free self-reference by universality). Determinate concept = infinitely free concept.',
      [
        'totality',
        'determinateness',
        'first negation',
        'reflection into itself',
        'total reflection',
        'doubly reflective shine',
        'outward',
        'inward',
        'particularity',
        'relative universal',
        'immanent character',
        'genus',
        'equal in extension',
        'free self-reference',
        'infinitely free',
      ],
      { section: 'The Universal Concept', order: 9 }
    ),

    createTopicMapEntry(
      'univ-10-higher-universal-concretes',
      'Higher Universal: Outward Turned Inward; Life, I, Spirit as Concretes; Idea of Infinite Spirit',
      [248, 277],
      'Other side: genus limited by determinate character = lower genus → resolution in higher universal. Higher universal can be grasped as more abstract genus (pertains to outwardly directed side). Truly higher universal = outwardly directed side redirected inwardly (second negation). Determinateness = positedness/reflective shine. Life, I, spirit, absolute concept = not universals only as higher genera. They = concretes whose determinacies not mere species/lower genera. Determinacies in reality = self-contained and self-complete. Life, I, finite spirit = also determinate concepts. Their resolution = universal as truly absolute concept = idea of infinite spirit. Infinite spirit = posited being = infinite, transparent reality. Contemplates its creation and, in creation, itself.',
      [
        'lower genus',
        'higher universal',
        'abstract genus',
        'truly higher universal',
        'second negation',
        'positedness',
        'reflective shine',
        'concretes',
        'self-contained',
        'self-complete',
        'idea of infinite spirit',
        'infinite transparent reality',
        'contemplates creation',
      ],
      { section: 'The Universal Concept', order: 10 }
    ),

    createTopicMapEntry(
      'univ-11-creative-differentiation',
      'True Infinite Universal as Particularity: Creative Self-Differentiation; Universal Differences',
      [279, 296],
      'True infinite universal = immediately in itself = particularity and singularity. Now examined as particularity. Determines itself freely. Process of becoming finite = not transition (sphere of being). = creative power as self-referring absolute negativity. Differentiates itself internally = determining (differentiating one with universality). Posits differences that are themselves universals (self-referring). Differences become fixed, isolated.',
      [
        'true infinite universal',
        'particularity',
        'singularity',
        'determines freely',
        'creative power',
        'self-referring absolute negativity',
        'differentiates internally',
        'posits differences',
        'universals',
        'self-referring',
        'fixed isolated',
      ],
      { section: 'The Universal Concept', order: 11 }
    ),

    createTopicMapEntry(
      'univ-12-finite-universality',
      'Finite Subsistence as Universality; The Concept\'s Creativity',
      [298, 307],
      'Isolated subsistence of finite = earlier determined as being-for-itself, thinghood, substance. In truth = universality. = form with which infinite concept clothes its differences. Form = equally itself one of its differences. This = creativity of concept. To be comprehended only in concept\'s innermost core.',
      [
        'isolated subsistence',
        'finite',
        'being-for-itself',
        'thinghood',
        'substance',
        'universality',
        'form',
        'infinite concept',
        'clothes differences',
        'creativity',
        'innermost core',
      ],
      { section: 'The Universal Concept', order: 12 }
    ),
  ],
  {
    sectionDescription: 'The Universal Concept - Pure concept as absolutely infinite, unconditioned, free. Genesis via Being → Essence → Concept. Universality as negation of negation, creative principle, free power/love.',
  }
);

