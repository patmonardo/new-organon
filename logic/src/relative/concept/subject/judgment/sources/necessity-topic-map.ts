/**
 * TopicMap for Necessity.txt - The Judgment of Necessity
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
 * - necessity-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const NECESSITY_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/judgment/sources/necessity.txt',
  'Hegel\'s Science of Logic - The Judgment',
  'The Judgment of Necessity',
  [
    createTopicMapEntry(
      'nec-1-introduction-objective',
      'Introduction: Objective Universality; Distinguished from Substantiality; Genus and Species',
      [4, 24],
      'Universality advanced to = objective universality (exists in and for itself). In essence corresponds to substantiality. Distinguished: belongs to concept, posited necessity, distinction immanent. Substance has distinction only in accidents. In judgment: posited with determinateness as essential/immanent, and as diverse (particularity). Determined as genus and species.',
      [
        'objective universality',
        'exists in and for itself',
        'substantiality',
        'posited necessity',
        'immanent',
        'principle',
        'genus',
        'species',
      ],
      { section: 'The Judgment of Necessity', order: 1 }
    ),

    createTopicMapEntry(
      'nec-2-categorical-genus-divides',
      'Categorical Judgment: Genus Divides into Species; Substantial Identity',
      [28, 58],
      'Genus essentially divides/repels into species. Genus only in so far as comprehends species. Species: exists in singulars, possesses higher universality. Categorical: predicate = universality in which subject possesses immanent nature. First immediate judgment of necessity. Substantial identity of subject and predicate. Subject reflected into being-in-and-for-itself.',
      [
        'genus divides',
        'repels',
        'comprehends',
        'species',
        'immanent nature',
        'immediate judgment',
        'external concrete existence',
        'substantial identity',
        'unessential positedness',
        'being-in-and-for-itself',
      ],
      { section: 'The Categorical Judgment', order: 2 }
    ),

    createTopicMapEntry(
      'nec-3-categorical-distinguished',
      'Categorical Distinguished from Positive/Negative; Totality of Form',
      [59, 84],
      'Categorical predicate not classed with preceding. Example: "rose is red" vs "rose is a plant". Distinguished: positive/negative = singular accidental content; categorical = totality of form reflected into itself. Copula in categorical = necessity; in positive/negative = abstract immediate being.',
      [
        'external property',
        'vegetable nature',
        'singular accidental content',
        'totality of form',
        'reflected into itself',
        'necessity',
        'abstract immediate being',
      ],
      { section: 'The Categorical Judgment', order: 3 }
    ),

    createTopicMapEntry(
      'nec-4-categorical-passes-hypothetical',
      'Categorical: Inner Necessity; Passes to Hypothetical',
      [86, 107],
      'Determinateness of subject = at first contingent. Not connected with necessity by form. Necessity = inner one. Subject = subject only as particular. Objective universal in determining itself = connection of identity with repelled determinateness (essentially). Through necessity of immediate being, passes to hypothetical.',
      [
        'contingent',
        'inner necessity',
        'essentially',
        'not merely accidental',
        'necessity of immediate being',
        'passes over',
        'hypothetical judgment',
      ],
      { section: 'The Categorical Judgment', order: 4 }
    ),

    createTopicMapEntry(
      'nec-5-hypothetical-if-then',
      'Hypothetical Judgment: "If A is, then B is"; Being of an Other',
      [111, 163],
      '"If A is, then B is" or "The being of A is not its own being but the being of an other, of B". Necessary connectedness of immediate determinacies. Two immediate/externally contingent existences. Contents indifferent (empty form). Being = mere possibility. Each extreme = being of an other. Concept: identity posited (concrete self-identity, immediately = being of an other).',
      [
        'necessary connectedness',
        'immediate determinacies',
        'externally contingent',
        'indifferent',
        'empty form',
        'mere possibility',
        'being of an other',
        'principle of identity',
        'concrete self-identity',
      ],
      { section: 'The Hypothetical Judgment', order: 5 }
    ),

    createTopicMapEntry(
      'nec-6-hypothetical-relations',
      'Hypothetical: Relations of Reflection; Indeterminate Form; Passes to Disjunctive',
      [165, 204],
      'Determined as: ground/consequence, condition/conditioned, causality. Relations = moments of one identity. Shape more proposition (indeterminate form). Being = unity of itself and other = universality (but also particular). Particularity = totality. Universality as concrete identity of concept. So it is disjunctive judgment.',
      [
        'ground',
        'consequence',
        'condition',
        'causality',
        'moments',
        'one identity',
        'indeterminate form',
        'unity',
        'universality',
        'particular',
        'totality',
        'concrete identity',
        'disjunctive judgment',
      ],
      { section: 'The Hypothetical Judgment', order: 6 }
    ),

    createTopicMapEntry(
      'nec-7-disjunctive-either-or',
      'Disjunctive Judgment: "A is either B or C"; Necessity of Concept',
      [208, 243],
      'Disjunctive = objective universality posited in union with form. Contains: genus in simple form (subject), totality of differentiated determinations. "A is either B or C" = necessity of concept. Self-identity of extremes. Differentiated according form (determination = mere form). Particularity of species and totality. Predicate = greater universal.',
      [
        'negative identity',
        'union with form',
        'concrete universality',
        'genus',
        'totality',
        'differentiated determinations',
        'necessity of concept',
        'self-identity',
        'conceptual determination',
        'particularity',
        'species',
        'greater universal',
      ],
      { section: 'The Disjunctive Judgment', order: 7 }
    ),

    createTopicMapEntry(
      'nec-8-disjunctive-positive-negative',
      'Disjunctive: Positive and Negative Identity; Unity of Determinate Particulars',
      [245, 260],
      'Genus = substantial universality. Subject = B as well as C ("as well as" = positive identity). Objective universal maintains itself in particularity. Species mutually exclude ("either or" = negative connection). Just as identical in negative as positive. Genus = unity of determinate particulars. Not abstract but immanent and concrete.',
      [
        'substantial universality',
        'as well as',
        'positive identity',
        'mutually exclude',
        'either or',
        'negative connection',
        'unity',
        'determinate particulars',
        'abstract universality',
        'immanent',
        'concrete',
      ],
      { section: 'The Disjunctive Judgment', order: 8 }
    ),

    createTopicMapEntry(
      'nec-9-empirical-without-necessity',
      'Empirical Disjunctive Without Necessity; Negative Unity',
      [270, 295],
      'Empirical disjunctive = without necessity. Species found beforehand (subjective completeness). "Either or" excludes entire sphere. Totality has necessity in negative unity (dissolved singularity, immanent simple principle). Empirical species: differences in accidentality (external principle, not immanent determinateness). Not reciprocally connected.',
      [
        'empirical disjunctive',
        'without necessity',
        'found beforehand',
        'subjective completeness',
        'negative unity',
        'simple principle',
        'differentiation',
        'accidentality',
        'external principle',
        'immanent determinateness',
        'reciprocally connected',
      ],
      { section: 'The Disjunctive Judgment', order: 9 }
    ),

    createTopicMapEntry(
      'nec-10-contrary-contradictory',
      'Contrary and Contradictory Concepts; Unity as Truth',
      [296, 316],
      'Contrary and contradictory find proper place (disjunctive = essential difference). Find truth: differentiated. Species contrary: merely diverse (immediate existence). Species contradictory: exclude. Each one-sided and void of truth. In "either or": unity posited as truth (independent subsistence = principle of negative unity).',
      [
        'contrary',
        'contradictory',
        'essential difference',
        'differentiated',
        'merely diverse',
        'immediate existence',
        'exclude',
        'one-sided',
        'void of truth',
        'unity',
        'independent subsistence',
        'negative unity',
      ],
      { section: 'The Disjunctive Judgment', order: 10 }
    ),

    createTopicMapEntry(
      'nec-11-proximate-genus',
      'Proximate Genus; Concrete Essentially Determined Universality',
      [318, 353],
      'Through identity (negative unity), genus = proximate genus. Mere quantitative difference. If genus = abstraction: cannot form disjunctive (contingent completeness). When genus = concrete, essentially determined: simple determinateness = unity of moments (real difference in species). Genus = proximate genus (species has specific difference in essential determinateness).',
      [
        'proximate genus',
        'quantitative difference',
        'contingent',
        'abstraction',
        'concrete',
        'essentially determined',
        'simple determinateness',
        'unity of moments',
        'real difference',
        'specific difference',
      ],
      { section: 'The Disjunctive Judgment', order: 11 }
    ),

    createTopicMapEntry(
      'nec-12-identity-determinateness',
      'Identity from Aspect of Determinateness; Difference of Concept',
      [355, 374],
      'Identity from aspect of determinateness (posited by hypothetical). Necessity = negative unity. Separates subject/predicate but now differentiated (subject = simple determinateness, predicate = totality). Parting = difference of concept. Totality of species = this difference. Concept alone disjoins itself, manifests negative unity.',
      [
        'aspect of determinateness',
        'negative unity',
        'separates',
        'differentiated',
        'simple determinateness',
        'totality',
        'difference of concept',
        'disjoins',
        'manifests',
      ],
      { section: 'The Disjunctive Judgment', order: 12 }
    ),

    createTopicMapEntry(
      'nec-13-progressive-determination',
      'Concept\'s Own Progressive Determination; Disjunction',
      [375, 399],
      'Species = simple conceptual determinateness. Essential differentiation = moment of concept. Concept\'s own progressive determination posits disjunction. Concept = universal (positive and negative totality) = one of disjunctive members. Other = universality resolved into particularity. If not this form: not risen to determinateness of concept.',
      [
        'simple conceptual determinateness',
        'shape',
        'idea',
        'essential differentiation',
        'moment of concept',
        'progressive determination',
        'disjunction',
        'positive and negative totality',
        'resolved into particularity',
        'not proceeded from it',
      ],
      { section: 'The Disjunctive Judgment', order: 13 }
    ),

    createTopicMapEntry(
      'nec-14-example-color',
      'Example: Color Disjunction; Concrete Unity of Light and Darkness',
      [400, 422],
      'Example: Color either violet, indigo, blue, etc. (confusion, impurity, barbarism). If color = concrete unity of light and darkness: genus has determinateness constituting principle. One = utterly simple color (opposition in balance). Relation of opposition, indifferent neutrality. Taking mixtures/shades as genus = inconsiderate procedure.',
      [
        'confusion',
        'impurity',
        'barbarism',
        'concrete unity',
        'light and darkness',
        'principle of particularization',
        'utterly simple',
        'opposition in balance',
        'indifferent neutrality',
        'inconsiderate procedure',
      ],
      { section: 'The Disjunctive Judgment', order: 14 }
    ),

    createTopicMapEntry(
      'nec-15-transition-concept',
      'Judgment Itself Disjoined; Transition to Judgment of Concept',
      [424, 450],
      'Disjunctive has members in predicate. But judgment itself disjoined (subject/predicate = members). They = moments of concept (identical). Identical: (a) objective universality, (b) negative unity (developed connectedness, essential connection and self-identity). Unity (copula) = concept itself (concept as posited). Judgment of necessity risen to judgment of concept.',
      [
        'disjoined',
        'members of disjunction',
        'moments of concept',
        'identical',
        'objective universality',
        'negative unity',
        'developed connectedness',
        'essential connection',
        'self-identity',
        'copula',
        'concept as posited',
        'judgment of concept',
      ],
      { section: 'The Disjunctive Judgment', order: 15 }
    ),
  ],
  {
    sectionDescription: 'The Judgment of Necessity - Objective universality, genus and species. Categorical judgment (substantial identity). Hypothetical judgment ("If A is, then B is"). Disjunctive judgment ("A is either B or C", proximate genus). Transition to judgment of concept.',
  }
);

