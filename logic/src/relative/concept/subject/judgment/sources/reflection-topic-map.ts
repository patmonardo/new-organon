/**
 * TopicMap for Reflection.txt - The Judgment of Reflection
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
 * - reflection-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const REFLECTION_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/judgment/sources/reflection.txt',
  'Hegel\'s Science of Logic - The Judgment',
  'The Judgment of Reflection',
  [
    createTopicMapEntry(
      'refl-1-introduction-unity',
      'Introduction: Universal Collected into Unity; Determinate Content; Relational Determination',
      [3, 57],
      'Subject = singular as such. Universal = collected into unity through connection of different terms (coalescing of manifold properties). First have determinate content strictly speaking (form determination reflected into identity). Examples: mortal, perishable, useful/harmful. Express essentiality = relational determination or comprehensive universality. True universal = inner essence of multiplicity (in sphere of appearance).',
      [
        'singular as such',
        'collected into unity',
        'coalescing',
        'determinate content',
        'form determination',
        'reflected into identity',
        'relational determination',
        'comprehensive universality',
        'inner essence',
        'sphere of appearance',
      ],
      { section: 'The Judgment of Reflection', order: 1 }
    ),

    createTopicMapEntry(
      'refl-2-not-merely-quantity',
      'Not Merely Quantity; Most External Determination of Mediation',
      [59, 72],
      'May seem fitting to define as judgment of quantity. But immediacy sublated not just sublated quality, not merely quantity. Quality = most external immediacy. Quantity = most external determination belonging to mediation.',
      [
        'judgment of quantity',
        'qualitative judgment',
        'immediacy',
        'mediated',
        'abstract',
        'most external',
        'mediation',
      ],
      { section: 'The Judgment of Reflection', order: 2 }
    ),

    createTopicMapEntry(
      'refl-3-movement-subject',
      'Movement in Subject, Not Predicate; Reflected In-Itselfness',
      [74, 95],
      'In judgment of existence: movement in predicate (subject = underlying basis). In judgment of reflection: movement in subject (reflected in-itselfness). Essential = universal/predicate (basis against which subject measured). Predicate receives determination indirectly. Subject = direct advance.',
      [
        'movement in predicate',
        'movement in subject',
        'underlying basis',
        'reflected in-itselfness',
        'essential',
        'measured',
        'indirectly',
        'direct advance',
      ],
      { section: 'The Judgment of Reflection', order: 3 }
    ),

    createTopicMapEntry(
      'refl-4-subsumption',
      'Objective Signification; Judgments of Subsumption',
      [97, 113],
      'Singular enters existence by virtue of universality (relational). Essentiality maintains itself across manifold of appearance. Subject = determined in and for itself. Predicate no longer inheres but implicit being (subject subsumed as accidental). Judgments of existence = inherence. Judgments of reflection = subsumption.',
      [
        'objective signification',
        'relational',
        'essentiality',
        'manifold of appearance',
        'determined in and for itself',
        'inheres',
        'subsumed',
        'accidental',
        'inherence',
        'subsumption',
      ],
      { section: 'The Judgment of Reflection', order: 4 }
    ),

    createTopicMapEntry(
      'refl-5-singular-judgment',
      'Singular Judgment: "This is Essential Universal"; Truth in Particular',
      [117, 142],
      'Immediate judgment: "this is an essential universal". But "this" not essential universal. Positive judgment must be taken negatively. Negation does not affect predicate (implicit being). Subject = alterable, needs determination. Negative: "not a this" = universal of reflection. Singular judgment has proximate truth in particular judgment.',
      [
        'essential universal',
        'alterable',
        'needs determination',
        'not a this',
        'universal of reflection',
        'proximate truth',
        'particular judgment',
      ],
      { section: 'The Singular Judgment', order: 5 }
    ),

    createTopicMapEntry(
      'refl-6-particular-extension',
      'Particular Judgment: Extension of Singular; Contains Both Positive/Negative',
      [146, 183],
      'Non-singularity = particularity (essential singularity, not abstract). Extension of singular in external reflection. Subject: "these ones" or "particular number of singulars". "Some singulars are universal" = positive and negative. "Some" contains universality but disproportionate. Positive and negative no longer fall outside one another. Particular judgment = indeterminate.',
      [
        'non-singularity',
        'particularity',
        'essential singularity',
        'extension',
        'external reflection',
        'these ones',
        'some',
        'comprehensive',
        'disproportionate',
        'copula',
        'indeterminate',
      ],
      { section: 'The Particular Judgment', order: 6 }
    ),

    createTopicMapEntry(
      'refl-7-universal-nature',
      'Subject Contains Universal Nature; Species Anticipated',
      [185, 211],
      'Subject contains: particular form ("some") + content ("humans"). Singular judgment: "this human" (Gaius). Particular judgment: "some humans" (cannot be "some Gaiuses"). Universal content determined by form. Universality = universal nature or species (result anticipated).',
      [
        'form determination',
        'content determination',
        'external pointing',
        'universal content',
        'empirical content',
        'universal nature',
        'species',
        'anticipated',
      ],
      { section: 'The Particular Judgment', order: 7 }
    ),

    createTopicMapEntry(
      'refl-8-totality-extension',
      'Subject as Totality; Extension to Universality (Allness)',
      [213, 239],
      'Subject contains: singulars, particularity, universal nature (totality, but external consideration). Extension of "this" to particularity not commensurate ("this" determinate, "some" indeterminate). Extension ought to be completely determined = totality or universality. Universality has "this" for basis. Particular judgment passed over into universal.',
      [
        'totality',
        'determinations of concept',
        'external consideration',
        'extension',
        'commensurate',
        'perfectly determinate',
        'indeterminate',
        'allness',
        'passed over',
      ],
      { section: 'The Particular Judgment', order: 8 }
    ),

    createTopicMapEntry(
      'refl-9-allness-external',
      'Universal Judgment: "Allness" as External Universality; Bad Infinity',
      [243, 295],
      'Universality = external universality of reflection, "allness". Only commonality of self-subsisting singulars (association by comparison). Polynomial vs binomial (method/rule = true universal). Bad infinity: concept = achieved beyond; bad infinity = unattainable beyond. Allness exhausted in singulars = relapse into bad infinity. Plurality = particularity, not allness. Obscure intimation: universality of concept in and for itself.',
      [
        'allness',
        'external universality',
        'commonality',
        'self-subsisting',
        'association',
        'comparison',
        'polynomial',
        'binomial',
        'pantonomial',
        'method',
        'rule',
        'bad infinity',
        'achieved beyond',
        'plurality',
        'totality',
        'in-and-for-itself',
      ],
      { section: 'The Universal Judgment', order: 9 }
    ),

    createTopicMapEntry(
      'refl-10-empirical-allness',
      'Empirical Allness: Task and Ought; Subjective vs Objective',
      [306, 326],
      'Allness = empirical universality. Singular = immediate (pre-given, externally picked). Reflection = external. Singular indifferent to reflection. Cannot combine to unity. Empirical allness = task (ought, cannot be represented). Empirically universal proposition: plurality counts for allness (if no contrary). Subjective allness = objective allness.',
      [
        'empirical universality',
        'immediate',
        'pre-given',
        'externally picked',
        'external',
        'indifferent',
        'cannot combine',
        'task',
        'ought',
        'tacit agreement',
        'subjective allness',
        'objective allness',
      ],
      { section: 'The Universal Judgment', order: 10 }
    ),

    createTopicMapEntry(
      'refl-11-achieved-universality',
      'Subject Contains Achieved Universality; Posited Equal to Presupposed',
      [328, 340],
      'Subject contains achieved universality as presupposed (posited in it). "All humans" expresses: (1) species "human", (2) species in singularization (singulars expanded to universality). Universality perfectly determined as singularity. Posited = equal to presupposed.',
      [
        'achieved universality',
        'presupposed',
        'posited',
        'species',
        'singularization',
        'expanded',
        'perfectly determined',
        'equal',
      ],
      { section: 'The Universal Judgment', order: 11 }
    ),

    createTopicMapEntry(
      'refl-12-objective-universality',
      'Singularity Expanded to Allness; Objective Universality; "The Human Being"',
      [342, 370],
      'Singularity expanded to allness = posited as negativity (identical self-reference). Not remained first singularity but determination identical with universality (absolute determinateness). First singularity = negative identity (dialectical movement). True presupposition = in-itself. Reflection not external (makes explicit implicit). Result = objective universality. Subject shed form determination. "All humans" → "the human being".',
      [
        'expanded to allness',
        'negativity',
        'identical self-reference',
        'absolute determinateness',
        'dialectical movement',
        'negative identity',
        'presupposition',
        'in-itself',
        'implicit',
        'explicit',
        'objective universality',
        'shed form determination',
      ],
      { section: 'The Universal Judgment', order: 12 }
    ),

    createTopicMapEntry(
      'refl-13-genus',
      'Genus: Concrete Universality; Relation Reversed; Judgment Sublated',
      [372, 401],
      'Universality = genus (concrete in universality). Does not inhere (not property). Contains determinacies dissolved into substantial purity. Negative self-identity = essentially subject (no longer subsumed). Judgment was subsumption (predicate = implicit universal, mark). But objective universality: subject not subsumed, predicate = particular. Relation reversed, judgment sublated.',
      [
        'genus',
        'concrete universality',
        'inheres',
        'property',
        'substantial purity',
        'negative self-identity',
        'essentially subject',
        'subsumption',
        'implicit universal',
        'mark',
        'appearance',
        'objective universality',
        'reversed',
        'sublated',
      ],
      { section: 'The Universal Judgment', order: 13 }
    ),

    createTopicMapEntry(
      'refl-14-transition-necessity',
      'Transition to Judgment of Necessity; Identity in Copula',
      [403, 432],
      'Sublation of judgment = determination of copula (one and same). Subject raised to universality = equal to predicate. Subject and predicate identical (coincide in copula). Identity = genus or nature in and for itself. As divides: inner nature connecting subject/predicate. Connection of necessity (unessential distinctions). Basis of judgment of necessity.',
      [
        'sublation',
        'copula',
        'transition',
        'equal',
        'identical',
        'coincide',
        'genus',
        'nature in and for itself',
        'inner nature',
        'connection of necessity',
        'unessential distinctions',
        'judgment of necessity',
      ],
      { section: 'The Universal Judgment', order: 14 }
    ),
  ],
  {
    sectionDescription: 'The Judgment of Reflection - Universal collected into unity, determinate content, relational determination. Movement in subject. Judgments of subsumption. Singular → Particular → Universal. "Allness" vs objective universality. Genus. Transition to judgment of necessity.',
  }
);

