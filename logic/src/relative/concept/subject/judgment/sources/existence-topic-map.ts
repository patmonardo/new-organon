/**
 * TopicMap for Existence.txt - The Judgment of Existence
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
 * - existence-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const EXISTENCE_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/judgment/sources/existence.txt',
  'Hegel\'s Science of Logic - The Judgment',
  'The Judgment of Existence',
  [
    createTopicMapEntry(
      'exist-1-introduction-truth',
      'Introduction: Judgment as Truth; Agreement of Concept and Reality',
      [3, 24],
      'In subjective judgment: same object double (singular actuality + essential identity/concept). Singular raised into universality = universal made singular into actuality. Judgment = truth (agreement of concept and reality). But at first immediate (no reflection, no movement). Judgment of immediate existence = qualitative judgment.',
      [
        'subjective judgment',
        'singular actuality',
        'essential identity',
        'concept',
        'truth',
        'agreement',
        'concept and reality',
        'immediate',
        'qualitative judgment',
        'immediacy',
      ],
      { section: 'The Judgment of Existence', order: 1 }
    ),

    createTopicMapEntry(
      'exist-2-judgment-inherence',
      'Judgment of Inherence; Subject Immediate, Predicate Has Foundation',
      [26, 35],
      'Judgment of existence = judgment of inherence. Subject = immediate, first and essential term. Predicate = does not subsist on its own, has foundation in subject.',
      [
        'judgment of inherence',
        'subject',
        'immediate',
        'essential',
        'predicate',
        'foundation',
      ],
      { section: 'The Judgment of Existence', order: 2 }
    ),

    createTopicMapEntry(
      'exist-3-subject-predicate-determinations',
      'Subject and Predicate Determinations; Abstract Singularity and Universality',
      [39, 87],
      'Subject and predicate = names at first. As sides of judgment (posited determinate concept), have determination of moments of concept. But immediacy makes determination simple, caught up in abstract opposition of abstract singularity and abstract universality. Predicate = abstract universal. Subject = abstract singular (concept passed over into externality). Copula = immediate, abstract being. Connection does not contain mediation or negation = "positive".',
      [
        'names',
        'moments of concept',
        'immediacy',
        'abstract singularity',
        'abstract universality',
        'abstract universal',
        'mediation',
        'presupposition',
        'immediacy of universal',
        'negativity',
        'abstract singular',
        'externality',
        'copula',
        'positive',
      ],
      { section: 'The Positive Judgment', order: 3 }
    ),

    createTopicMapEntry(
      'exist-4-first-pure-expression',
      'First Pure Expression: "The Singular is Universal"',
      [89, 127],
      'First pure expression: "the singular is universal". Must not be "A is B" (formless, meaningless). Judgment has determinations of concept for extremes. What asserted in every judgment = "the singular is universal" (every judgment in principle also abstract judgment).',
      [
        'first pure expression',
        'singular is universal',
        'A is B',
        'formless',
        'determinations of concept',
        'abstract judgment',
        'form',
        'content',
      ],
      { section: 'The Positive Judgment', order: 4 }
    ),

    createTopicMapEntry(
      'exist-5-objective-meaning-resolution',
      'Objective Meaning: Perishableness and Resolution; Universal Resolves into Singular',
      [129, 191],
      'Objective meaning: perishableness of singular things + positive subsistence in concept. Concept = imperishable. Universal resolves itself into singular. Judgment = resolution of universal, development of negativity. Subject referred to universal, posited as concrete (something of many qualities, thing of manifold properties). Subject inherently = universal. Predicate = abstract determinateness (contains one moment, excludes others) = abstract singular. "The universal is singular".',
      [
        'perishableness',
        'positive subsistence',
        'imperishable',
        'alteration',
        'resolution',
        'development of negativity',
        'concrete',
        'many qualities',
        'manifold properties',
        'reflected into itself',
        'abstract singular',
        'isolates',
      ],
      { section: 'The Positive Judgment', order: 5 }
    ),

    createTopicMapEntry(
      'exist-6-twofold-result-form-content',
      'Twofold Result: Form and Content; Reciprocal Determination',
      [192, 272],
      'Twofold result: (1) Subject determined as universal by predicate. (2) Predicate determined in subject = singular. Subject and predicate should retain opposition. "Universal is singular" = content (isolated determination, totality). "Singular is universal" = form (immediately given). In immediate positive judgment, extremes simple: form and content united. Difference of form/content present implicitly.',
      [
        'twofold result',
        'reciprocal determination',
        'opposition',
        'form',
        'content',
        'immediate determinateness',
        'immanent reflection',
        'identity',
      ],
      { section: 'The Positive Judgment', order: 6 }
    ),

    createTopicMapEntry(
      'exist-7-cannot-be-united',
      'Two Propositions Cannot Be United; External Reflection',
      [274, 309],
      'If united (both = particular) → "particular is particular" = empty identical proposition, no longer judgment. Singularity and universality cannot yet be united into particularity (still immediate). Judgment must be distinguished according to form and content. Both self-subsistence and reciprocal determination.',
      [
        'united',
        'particular',
        'external reflection',
        'empty identical proposition',
        'immediacy',
        'mediated',
        'self-subsistence',
        'reciprocal determination',
      ],
      { section: 'The Positive Judgment', order: 7 }
    ),

    createTopicMapEntry(
      'exist-8-positive-not-true',
      'Positive Judgment Not True; Form and Content Fail',
      [310, 335],
      'Form: immediate singular not universal (predicate wider extension). Content: subject = universe of qualities, bad infinite plurality, not one single property. Both propositions must be united, positive judgment must be posited as negative.',
      [
        'wider extension',
        'does not correspond',
        'immediately for itself',
        'universe of qualities',
        'bad infinite plurality',
        'posited as negative',
      ],
      { section: 'The Positive Judgment', order: 8 }
    ),

    createTopicMapEntry(
      'exist-9-truth-in-negative',
      'Truth of Positive Judgment in Negative; "The Singular is a Particular"',
      [338, 411],
      'Common notion: truth/falsity depends on content. But two concepts relate as singular and universal (truly logical content). Positive judgment not true, has truth in negative judgment. Content contradicts itself. "Singular is not abstractly universal" → singular = particular. "Universal is not abstractly singular" → universal = particular. Both reduce to: "the singular is a particular".',
      [
        'common notion',
        'logical truth',
        'form',
        'contradiction',
        'truly logical content',
        'truth of reason',
        'negative judgment',
        'abstractly universal',
        'abstractly singular',
        'particular',
      ],
      { section: 'The Negative Judgment', order: 9 }
    ),

    createTopicMapEntry(
      'exist-10-particularity-arises',
      'Particularity Arises Through Negative Connection; Mediated Determination',
      [413, 441],
      'Particularity not posited by external reflection but arisen as mediated by negative connection. Determination results for predicate. First negation cannot yet be determination (positing = second moment). "Singular is particular" = positive expression of negative judgment. Particular = first mediated determination. Judgment also to be considered as negative.',
      [
        'external reflection',
        'negative connection',
        'underlying basis',
        'first negation',
        'second moment',
        'positive expression',
        'mediated determination',
      ],
      { section: 'The Negative Judgment', order: 10 }
    ),

    createTopicMapEntry(
      'exist-11-transition-founded',
      'Transition Founded on Relation of Extremes; Connection Has Negativity',
      [443, 478],
      'Transition founded on relation of extremes and connection. Positive judgment = connection of singular and universal (each not what other is). Connection = essentially separation, negative. Judgment determination = universal within, extending continuously. Connection = same determination as extremes. In so far as distinguished, connection also has negativity.',
      [
        'transition',
        'relation of extremes',
        'essentially separation',
        'copula',
        'determinate connection',
        'universal within',
        'extending continuously',
        'negativity',
      ],
      { section: 'The Negative Judgment', order: 11 }
    ),

    createTopicMapEntry(
      'exist-12-not-universal-particular',
      'Not-Universal is Particular; Fluid Continuity of Concept',
      [480, 557],
      'Not of copula attached to predicate → not-universal = particular. If focus on abstract non-being → totally indeterminate. But white = unconceptualized. Concept = true thing-in-itself. In sphere of immediate existence: non-being = limit. In reflection: negative refers to positive (ground). In fluid continuity of concept: not immediately positive, negation taken up into universality, identical. Non-universal = directly particular.',
      [
        'not-universal',
        'particular',
        'contradictory concepts',
        'unconceptualized',
        'thing-in-itself',
        'limit',
        'ground',
        'fluid continuity',
        'identical',
      ],
      { section: 'The Negative Judgment', order: 12 }
    ),

    createTopicMapEntry(
      'exist-13-still-judgment',
      'Negative Judgment Still a Judgment; Universal Sphere Retained',
      [559, 609],
      'Negative judgment still a judgment. Subject untouched by negation, retains reference to universality. What negated = not universality but abstraction/determinateness. Universal sphere remains standing. Connection still essentially positive. "Singular is particular" expresses: particular contains universality; predicate determinate. Determinateness transformed into particularity.',
      [
        'still a judgment',
        'untouched',
        'universal sphere',
        'essentially positive',
        'determinate',
        'indeterminate determinateness',
        'particularity',
      ],
      { section: 'The Negative Judgment', order: 13 }
    ),

    createTopicMapEntry(
      'exist-14-particularity-mediating',
      'Particularity as Mediating Term; Determinate Determinate',
      [611, 639],
      'Particularity = positive determination of negative judgment. = term mediating singularity and universality. Negative judgment provides mediation for third step. Objective meaning: moment of alteration of accidents. "Singular is particular" but singular also not particular (particularity wider extension). "Singular is only a singular" = determinate determinate.',
      [
        'mediating term',
        'third step',
        'alteration',
        'accidents',
        'determinate determinate',
      ],
      { section: 'The Negative Judgment', order: 14 }
    ),

    createTopicMapEntry(
      'exist-15-negation-negative',
      'Negation of Negative Judgment; Second Negation',
      [641, 697],
      'Negation appears to be first negation but negative judgment already = second negation (negation of negation). Negation of determinateness = infinite turning back of singularity into itself. Subject for first time posited as singular (mediated with itself). Predicate = absolute determinateness, equal to subject. "The singular is singular". "The universal is the universal".',
      [
        'second negation',
        'negation of negation',
        'infinite turning back',
        'concrete totality',
        'absolute determinateness',
        'purification',
      ],
      { section: 'The Negative Judgment', order: 15 }
    ),

    createTopicMapEntry(
      'exist-16-infinite-negative',
      'Infinite Judgment: Negative Infinite; Nonsensical',
      [699, 744],
      'Negative infinite = judgment in which even form sublated (nonsensical). Ought to be judgment but connection ought not be there. Examples: spirit not red, rose not elephant, understanding not table. Correct yet nonsensical. More realistic: evil action (crime). Crime = infinite judgment negating not only particular right but universal sphere (right as right).',
      [
        'infinite judgment',
        'negative infinite',
        'nonsensical',
        'sublated',
        'evil action',
        'crime',
        'universal sphere',
      ],
      { section: 'The Infinite Judgment', order: 16 }
    ),

    createTopicMapEntry(
      'exist-17-infinite-positive',
      'Positive Infinite: "The Singular is Singular"; Judgment Sublated',
      [746, 782],
      'Positive element = negation of negation = reflection of singularity into itself. Singularity first posited as determinate determinate. "The singular is singular". Through mediation, posited as singular for first time. Universality = summing of distincts. "The universal is universal". Through reflection, judgment sublated. Negatively infinite: difference too great. Positively infinite: only identity, total lack of difference, no longer judgment.',
      [
        'positive element',
        'negation of negation',
        'determinate determinate',
        'expanding',
        'summing of distincts',
        'turning back',
        'sublated',
        'total lack of difference',
      ],
      { section: 'The Infinite Judgment', order: 17 }
    ),

    createTopicMapEntry(
      'exist-18-transition-reflection',
      'Transition to Judgment of Reflection; Terms Reflected into Themselves',
      [784, 798],
      'Judgment of existence has sublated itself. Posited what copula contains: in identity, qualitative extremes sublated. But since unity = concept, immediately torn apart = judgment. Terms no longer immediately determined but reflected into themselves. Judgment of existence passed over into judgment of reflection.',
      [
        'sublated itself',
        'copula',
        'qualitative extremes',
        'concept',
        'torn apart',
        'reflected into themselves',
        'judgment of reflection',
      ],
      { section: 'The Infinite Judgment', order: 18 }
    ),
  ],
  {
    sectionDescription: 'The Judgment of Existence - Judgment as truth (agreement of concept and reality). Positive judgment: "the singular is universal". Negative judgment: "the singular is a particular". Infinite judgment: "the singular is singular". Transition to judgment of reflection.',
  }
);

