/**
 * TopicMap for Cognition.txt - Synthetic Cognition Sections
 *
 * SOURCE ANALYSIS PHASE 1: Topics
 *
 * COGNITIVE SCIENCE: This is where the real cognitive work happens.
 * The skill in producing good chunks and topics is what makes everything else meaningful.
 * The TopicMap helps check and improve understanding of Hegel through step-by-step analysis.
 *
 * STRUCTURE: Two Levels
 * 1. Analytic (Idea is Unitary) - immediate, no mediation
 * 2. Synthetic (Has a Subtriad) - Definition, Division, Theorem
 *    - Follows Kantian General Logic Methodology
 *    - Definition (universality)
 *    - Division (particularity)
 *    - Theorem (singularity - transition to idea)
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
 * - cognition-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 * - tools/source-analysis/COGNITION-STRUCTURE.md for structure details
 */

import type { TopicMap } from '../../../types/topic-map';
import { createTopicMap, createTopicMapEntry } from '../../../types/topic-map';

export const COGNITION_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/sources/cognition.txt',
  'Hegel\'s Science of Logic - The Idea of Cognition',
  'Synthetic Cognition: Definition, Division, Theorem',
  [
    // ========================================================================
    // SECTION 1: DEFINITION (lines 1289-1648)
    // ========================================================================

    createTopicMapEntry(
      'definition-1-transformation',
      'Transformation to Concept Form',
      [1291, 1312],
      'Still given objectivity transformed into simple form (form of concept). Moments: universality, particularity, singularity. Singular = object to be defined. Universal = proximate genus = universal with determinateness which is at the same time the principle for the differentiation of the particular. Particular = specific difference, determinate species.',
      [
        'objectivity transformed to concept form',
        'moments: universality, particularity, singularity',
        'singular = object to be defined',
        'universal = proximate genus',
        'proximate genus = universal with determinateness that is principle for differentiation of particular',
        'particular = specific difference',
      ],
      { section: 'Definition', order: 1 }
    ),

    createTopicMapEntry(
      'definition-2-reduces-concept',
      'Definition Reduces to Concept',
      [1314, 1337],
      'Gets rid of externalities. Abstracts from what is added in realization. Reduces manifold determinations to simplest moments. Subject matter = universal which is determined. Singular in which genus and particularization posited in one.',
      [
        'gets rid of externalities',
        'abstracts from realization',
        'reduces to simplest moments',
        'subject matter = universal determined',
        'singular = genus + particularization in one',
      ],
      { section: 'Definition', order: 2 }
    ),

    createTopicMapEntry(
      'definition-3-subjective-cognition',
      'Concept Finds Itself but Cognition is Subjective',
      [1339, 1359],
      'Concept finds reality that corresponds to it. But singularity not contained in this reality. Cognition is subjective, has external beginning. Content is given and contingent. Contingent in two respects: content and choice of determinations.',
      [
        'concept finds corresponding reality',
        'singularity not contained',
        'cognition = subjective with external beginning',
        'content = given and contingent',
        'contingent in two respects',
      ],
      { section: 'Definition', order: 3 }
    ),

    createTopicMapEntry(
      'definition-4-no-principle',
      'Difficulty - No Principle for Determining',
      [1361, 1385],
      'Singularity escapes conceptual determination. No principle for determining conceptual vs external reality. First case: Products of self-conscious purposiveness (easy).',
      [
        'singularity escapes conceptual determination',
        'no principle for conceptual vs external',
        'first case: products of purposiveness (easy)',
      ],
      { section: 'Definition', order: 4 }
    ),

    createTopicMapEntry(
      'definition-5-geometrical-arithmetic',
      'Second Case - Geometrical and Arithmetic Objects',
      [1387, 1421],
      'Geometrical objects = abstract determinations of space. Same as products of purposiveness. Space has further determinations (tri-dimensionality, continuity). Number determinations based on simple principle of one.',
      [
        'geometrical objects = abstract space determinations',
        'same as products of purposiveness',
        'space: tri-dimensionality, continuity',
        'number determinations = principle of one',
      ],
      { section: 'Definition', order: 5 }
    ),

    createTopicMapEntry(
      'definition-6-concrete-objects',
      'Third Case - Concrete Objects',
      [1423, 1471],
      'Concrete objects (nature/spirit) = things of many properties. Must determine genus, specific difference, essential property. No criterion available than existence itself. Universality is empirical (time or comparison). Would require theorems and proof.',
      [
        'concrete objects = many properties',
        'must determine genus, specific difference, essential',
        'no criterion but existence itself',
        'universality = empirical',
        'would require theorems and proof',
      ],
      { section: 'Definition', order: 6 }
    ),

    createTopicMapEntry(
      'definition-7-resorts-marks',
      'Definition Resorts to Marks',
      [1473, 1500],
      'Connections of manifold determinations to simple concept would require theorems and proof. But definition is first, still undeveloped concept. Must apprehend simple determinateness of subject matter. Apprehension must be something simple. Can employ only one of subject\'s immediate so-called properties (determination of sensuous existence or representation). Singling out property through abstraction = what constitutes simplicity. For universality and essentiality, concept must resort to empirical universality, persistence under altered circumstances, and reflection that seeks determination of concept in external existence and pictorial representation (seeks it where it is not to be found). Defining forfeits true concept determinations that would by essence be principles of subject matter. Contents itself with marks = determinations in which that they are essential to subject matter is matter of indifference, whose only purpose is to be markers for external reflection.',
      [
        'connections require theorems and proof',
        'definition = first undeveloped concept',
        'must apprehend simple determinateness',
        'apprehension must be simple',
        'can employ only one immediate property',
        'property = sensuous existence or representation',
        'abstraction = simplicity',
        'resorts to empirical universality',
        'resorts to persistence under altered circumstances',
        'resorts to reflection seeking in external existence',
        'seeks where it is not to be found',
        'forfeits true concept determinations',
        'forfeits principles of subject matter',
        'contents itself with marks',
        'marks = essentiality is matter of indifference',
        'marks = markers for external reflection',
      ],
      { section: 'Definition', order: 7 }
    ),

    createTopicMapEntry(
      'definition-8-disproportionate',
      'External Determinateness Disproportionate',
      [1501, 1551],
      'Single external determinateness too disproportionate. Example: lobe of ear for human definition. Properties external to themselves. Concept unfolded into differences, cannot attach to single property.',
      [
        'single determinateness disproportionate',
        'example: lobe of ear',
        'properties external to themselves',
        'concept unfolded, cannot attach to single property',
      ],
      { section: 'Definition', order: 8 }
    ),

    createTopicMapEntry(
      'definition-9-bad-specimens',
      'Bad Specimens',
      [1553, 1600],
      'Difference between concept and realization. Bad specimens may lack properties. Empirical search frustrated by malformation.',
      [
        'difference: concept vs realization',
        'bad specimens lack properties',
        'empirical search frustrated',
      ],
      { section: 'Definition', order: 9 }
    ),

    createTopicMapEntry(
      'definition-10-concept-maintained',
      'Concept Maintained Despite Contradiction',
      [1602, 1623],
      'Concept stands on its own. Goes against meaning of definition. Formal truth cannot be established.',
      [
        'concept stands on its own',
        'goes against definition meaning',
        'formal truth cannot be established',
      ],
      { section: 'Definition', order: 10 }
    ),

    createTopicMapEntry(
      'definition-11-immediate-existence',
      'Content from Immediate Existence',
      [1625, 1637],
      'Content taken from immediate existence. No justification, no necessity. Renounces comprehending conceptually. Form determination without reflection.',
      [
        'content from immediate existence',
        'no justification, no necessity',
        'renounces conceptual comprehension',
        'form without reflection',
      ],
      { section: 'Definition', order: 11 }
    ),

    createTopicMapEntry(
      'definition-12-passes-division',
      'Passes Over to Division',
      [1639, 1647],
      'Immediacy proceeds from mediation. Determinateness mediated by its other. Must pass over into division.',
      [
        'immediacy proceeds from mediation',
        'determinateness mediated by other',
        'must pass to division',
      ],
      { section: 'Definition', order: 12, relatedChunks: ['division-1-necessity'] }
    ),

    // ========================================================================
    // SECTION 2: DIVISION (lines 1649-2013)
    // ========================================================================

    createTopicMapEntry(
      'division-1-necessity',
      'Necessity of Division',
      [1651, 1668],
      'Universal must particularize itself. Necessity lies in universal. But definition begins with particular. Particular points to an other. Universal presupposed for division. Singular content raised to universality.',
      [
        'universal must particularize',
        'necessity in universal',
        'definition begins with particular',
        'particular points to other',
        'universal presupposed',
        'singular raised to universality',
      ],
      { section: 'Division', order: 1, relatedChunks: ['definition-12-passes-division'] }
    ),

    createTopicMapEntry(
      'division-2-transition',
      'Transition from Universal to Particular',
      [1670, 1679],
      'Transition determined by form of concept. Advance from universal to particular. Basis for synthetic science, system, systematic cognition.',
      [
        'transition by concept form',
        'advance: universal to particular',
        'basis for synthetic science',
      ],
      { section: 'Division', order: 2 }
    ),

    createTopicMapEntry(
      'division-3-beginning-universal',
      'Beginning with Universal',
      [1681, 1712],
      'Beginning must be with universal. In actuality, concrete singularity given first. But in conceptual comprehension, simple/abstract comes first. Objection: intuition easier than cognition. But cognition requires method appropriate to cognition. Abstract simple easier to grasp than concrete.',
      [
        'beginning = universal',
        'actuality: concrete first',
        'conceptual: abstract first',
        'objection: intuition easier',
        'cognition requires cognitive method',
        'abstract easier than concrete',
      ],
      { section: 'Division', order: 3 }
    ),

    createTopicMapEntry(
      'division-4-universal-first',
      'Universal First Moment',
      [1723, 1738],
      'Universal is first moment of concept (simple). Particular comes after (mediated). Simple is more universal. Concrete presupposes transition from first. Applies to ordering of whole.',
      [
        'universal = first moment (simple)',
        'particular = after (mediated)',
        'simple = more universal',
        'concrete presupposes transition',
        'applies to whole ordering',
      ],
      { section: 'Division', order: 4 }
    ),

    createTopicMapEntry(
      'division-5-examples',
      'Examples - Reading, Geometry, Physics',
      [1739, 1783],
      'Learning to read: begin with elements, not whole words. Geometry: begin with point/line, not concrete space. Physics: free properties from entanglements. Experiment must admit only necessary conditions.',
      [
        'reading: elements first',
        'geometry: point/line first',
        'physics: free from entanglements',
        'experiment: only necessary conditions',
      ],
      { section: 'Division', order: 5 }
    ),

    createTopicMapEntry(
      'division-6-colors',
      'Example - Colors',
      [1784, 1816],
      'Colors: universal form is spectrum (middle). Not subjective sense or fixed in objects. Abstract must be starting point. Particularities spread out from abstract.',
      [
        'colors: spectrum = universal (middle)',
        'not subjective or fixed',
        'abstract = starting point',
        'particularities spread from abstract',
      ],
      { section: 'Division', order: 6 }
    ),

    createTopicMapEntry(
      'division-7-specific-science-dharma',
      'Specific Science = Dharma',
      [1818, 1837],
      'Universal itself only member of division. Higher universal, higher yet, to infinity. No immanent limit. Proceeds from given. Abstract universality defines "first". Object with elementary universality = subject matter of a specific science. A Dharma is a Specific Science - Not just any concept or idea. Philosophy as knowing has a special membership protocol: Pure a priori syntheses as Dichotomies with Two Members. Perfect Divisions yield Perfect Sciences - Certainty! Makes absolute beginning (ordinary acquaintance presupposed, stands on its own, no derivation needed). Definition takes it as an immediate.',
      [
        'universal = member of division',
        'higher universal to infinity',
        'no immanent limit',
        'proceeds from given',
        'abstract universality = "first"',
        'elementary universality = specific science',
        'dharma = specific science',
        'not just any concept or idea',
        'philosophy as knowing = special membership protocol',
        'pure a priori syntheses = dichotomies with two members',
        'perfect divisions = perfect sciences',
        'certainty',
        'absolute beginning',
        'ordinary acquaintance presupposed',
        'stands on its own',
        'no derivation needed',
        'definition takes as immediate',
      ],
      { section: 'Division', order: 7 }
    ),

    createTopicMapEntry(
      'division-8-lacks-principle',
      'Division Lacks Principle',
      [1839, 1873],
      'Division immediately next step. Would require immanent principle. But lacks such principle. Follows form determination without immanent reflection. Takes determinateness from what is given. No specific reason for particular. Can only arrange empirical material, discover by comparison.',
      [
        'division = next step',
        'would require immanent principle',
        'lacks principle',
        'follows form without reflection',
        'takes from given',
        'no reason for particular',
        'arrange empirical, discover by comparison',
      ],
      { section: 'Division', order: 8 }
    ),

    createTopicMapEntry(
      'division-9-formal-rules',
      'Formal Rules Lead Nowhere',
      [1875, 1897],
      'Only formal, empty rules. Rule: division should exhaust concept. But each member must exhaust concept. Empirical manifold internally void of determination. Exhaustion = tautological (list all species).',
      [
        'only formal empty rules',
        'rule: exhaust concept',
        'each member exhausts concept',
        'empirical manifold void',
        'exhaustion = tautological',
      ],
      { section: 'Division', order: 9 }
    ),

    createTopicMapEntry(
      'division-10-genus-altered',
      'Genus Altered by New Species',
      [1898, 1934],
      'New species may not fit assumed determination. Genus usually adopted from obscure representation. Genus would have to be altered. Standpoint becomes ground of division. Or exclude material. Game of chance.',
      [
        'new species may not fit',
        'genus from obscure representation',
        'genus altered',
        'standpoint = ground',
        'or exclude material',
        'game of chance',
      ],
      { section: 'Division', order: 10 }
    ),

    createTopicMapEntry(
      'division-11-physical-contingency',
      'Physical Nature\'s Contingency',
      [1935, 1967],
      'Physical nature presents contingency. External dependency, manifold connectedness. Assortment of principles. Hybrids go in different directions. Marks significant in one series, inconspicuous in another. Impossible to abide by any principle. Species simply diverse, not opposed. Difference stays at diversity. Common to take determination from number.',
      [
        'physical nature = contingency',
        'external dependency',
        'assortment of principles',
        'hybrids different directions',
        'marks vary by series',
        'impossible to abide by principle',
        'species diverse, not opposed',
        'difference = diversity',
        'determination from number',
      ],
      { section: 'Division', order: 11 }
    ),

    createTopicMapEntry(
      'division-12-instinct-reason',
      'Instinct of Reason',
      [1969, 2012],
      'Contingency of particular. May be attributed to instinct of reason. Discover bases more adequate to concept. Example: animals - teeth/claws as criterion. Vital point of animal individuality. Plant: reproductive parts. Highest point, transition to sexual difference. Determinateness in and for itself highest.',
      [
        'contingency of particular',
        'instinct of reason',
        'bases adequate to concept',
        'animals: teeth/claws',
        'vital point of individuality',
        'plant: reproductive parts',
        'highest point',
        'transition to sexual difference',
        'determinateness highest',
      ],
      { section: 'Division', order: 12 }
    ),

    // ========================================================================
    // SECTION 3: THEOREM (lines 2014-2728)
    // ========================================================================

    createTopicMapEntry(
      'theorem-1-transition-singularity',
      'Transition of Particularity to Singularity',
      [2016, 2038],
      'Third stage: transition of particularity to singularity. Self-referring determinateness, internal differentiation, connection of differentiated determinacies. Definition = one determinateness, division = determinateness against other, singularization = subject matter parted internally. Definition stops at universal concept, theorems = subject matter known in reality/conditions/forms. With definition, subject matter exhibits idea = unity of concept and reality. But cognition still seeking, reality does not proceed from concept, dependency/unity not cognized.',
      [
        'third stage: particularity to singularity',
        'self-referring determinateness',
        'internal differentiation',
        'definition = one determinateness',
        'division = determinateness against other',
        'singularization = parted internally',
        'theorems = known in reality',
        'exhibits idea = unity concept/reality',
        'cognition still seeking',
      ],
      { section: 'Theorem', order: 1, status: 'completed' }
    ),

    createTopicMapEntry(
      'theorem-2-synthetic-element',
      'Theorem as Properly Synthetic Element',
      [2040, 2078],
      'Theorem = properly synthetic element, relations necessary, grounded in inner identity of concept. Definition/division = external connectedness, content displayed; theorem = demonstratively displayed. Theorem content proceeds from concept\'s moment of singularity = determinations of reality, no longer simple/immediate concept determinations. In singularity, concept goes over to otherness/reality, becoming idea. Synthesis in theorem lacks concept form justification, is joining of diverse, unity needs demonstration, proof necessary.',
      [
        'theorem = synthetic element',
        'relations necessary',
        'grounded in concept identity',
        'definition/division = external',
        'theorem = demonstrative',
        'content from singularity',
        'determinations of reality',
        'concept goes to otherness/reality',
        'becoming idea',
        'synthesis lacks concept form',
        'proof necessary',
      ],
      { section: 'Theorem', order: 2, status: 'completed' }
    ),

    // TODO: Continue chunking Theorem section (lines 2080-2728)
    // This is a large section that needs careful chunking

  ],
  {
    sectionDescription: 'Synthetic cognition consists of three stages: Definition (universality), Division (particularity), and Theorem (singularity). Each stage has specific characteristics and limitations that synthetic cognition must work through.',
    metadata: {
      createdAt: new Date().toISOString(),
      version: '1.0.0',
    },
  }
);

