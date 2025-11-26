/**
 * TopicMap for Particular.txt - The Particular Concept
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
 * - particular-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const PARTICULAR_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/concept/sources/particular.txt',
  'Hegel\'s Science of Logic - The Concept',
  'The Particular Concept',
  [
    createTopicMapEntry(
      'part-1-determinateness-immanent',
      'Determinateness as Particularity; Universal\'s Own Immanent Moment',
      [2, 9],
      'Determinateness as such belongs to being and the qualitative. As determinateness of the concept = particularity. Not a limit (related to other beyond it). Universal\'s own immanent moment. In particularity, universal is not in an other but with itself.',
      [
        'determinateness',
        'being',
        'qualitative',
        'particularity',
        'limit',
        'immanent moment',
        'universal',
        'with itself',
      ],
      { section: 'The Particular Concept', order: 1 }
    ),

    createTopicMapEntry(
      'part-2-contains-universality-totality',
      'Particular Contains Universality; Totality and Completeness',
      [11, 50],
      'Particular contains universality (constitutes its substance). Genus unaltered in its species; species differ only from each other, not from universal. Particular has same universality as other particulars. Diversity of particulars = universal (because of identity with universal) = totality. Particular exhibits universal through determinateness. Universal constitutes sphere that particular must exhaust. Totality appears as completeness. No inner standard/principle for diversity. Particularity = immanent connection (totality intrinsically, essential principle).',
      [
        'contains universality',
        'substance',
        'genus',
        'species',
        'diversity',
        'totality',
        'completeness',
        'dispersed difference',
        'external reflex',
        'contingent completeness',
        'opposition',
        'immanent connection',
        'essential principle',
      ],
      { section: 'The Particular Concept', order: 2 }
    ),

    createTopicMapEntry(
      'part-3-universal-itself-differentiation',
      'Particular is Universal Itself; Self-Differentiation',
      [52, 71],
      'Particular = universal itself. But it is its difference or reference to other (outwardly reflecting shine). No other at hand from which particular differentiated than universal itself. Universal determines itself, so is itself the particular. Determinateness = its difference; only differentiated from itself. Species are: (a) universal itself, (b) particular. Universal = concept itself and its opposite. Totality and principle of its diversity (determined wholly and solely through itself).',
      [
        'universal itself',
        'difference',
        'reference to other',
        'outwardly reflecting shine',
        'determines itself',
        'differentiated from itself',
        'species',
        'opposite',
        'posited determinateness',
        'overreaches',
        'totality',
        'principle',
      ],
      { section: 'The Particular Concept', order: 3 }
    ),

    createTopicMapEntry(
      'part-4-true-logical-division',
      'True Logical Division; Universal/Particular as Two Particulars',
      [73, 95],
      'No other true logical division than this: concept sets itself as immediate, indeterminate universality. This indeterminateness makes its determinateness (or that it is a particular). Two are both a particular and therefore coordinated. Both, as particular, also determinate as against universal (subordinated). But even this universal = just one of opposing sides. Their determinateness over against each other = essentially only one determinateness = negativity which in universal is simple.',
      [
        'true logical division',
        'immediate indeterminate universality',
        'coordinated',
        'subordinated',
        'opposing sides',
        'one determinateness',
        'negativity',
        'simple',
      ],
      { section: 'The Particular Concept', order: 4 }
    ),

    createTopicMapEntry(
      'part-5-difference-truth',
      'Difference in Its Truth; Concept Unity',
      [97, 131],
      'Difference here = in its concept and therefore in its truth. All previous difference has this unity in the concept. In being: difference = limit of an other. In reflection: difference = relative, posited as referring essentially to its other. Here: unity of concept begins to be posited. True significance of transitoriness/dissolution = they attain to their concept, their truth. Cause and effect = not two diverse concepts but only one determinate concept. Causality = simple concept.',
      [
        'difference in truth',
        'concept unity',
        'limit of other',
        'relative',
        'reflective shine',
        'transitoriness',
        'dissolution',
        'thought determinations',
        'determinate concepts',
        'unity',
        'opposition',
        'one determinateness',
        'simple concept',
      ],
      { section: 'The Particular Concept', order: 5 }
    ),

    createTopicMapEntry(
      'part-6-completeness-nature-impotence',
      'Completeness and Nature\'s Impotence',
      [133, 169],
      'Determinate moment of particularity complete in difference of universal and particular. Only these two make up particular species. More than two species found in any genus in nature. This = impotence of nature (cannot abide by rigor of concept, loses itself in blind manifoldness void of concept). Wonder at nature = without concept, object is irrational. Nature = self-externality of concept. Concept = absolute power (can let its difference go free). All of which = abstract side of nothingness.',
      [
        'completeness',
        'particular species',
        'impotence of nature',
        'blind manifoldness',
        'wonder',
        'irrational',
        'self-externality',
        'pictorial representation',
        'arbitrary notions',
        'traces',
        'intimations',
        'free self-externality',
        'absolute power',
        'nothingness',
      ],
      { section: 'The Particular Concept', order: 6 }
    ),

    createTopicMapEntry(
      'part-7-determinateness-principle-moment',
      'Determinateness as Principle and Moment; Abstract Universality as Form',
      [171, 199],
      'Determinateness of particular = simple as principle. Also simple as moment of totality. Concept, in determining itself, behaves negatively towards its unity. Gives itself form of one of its ideal moments of being. This being = immediacy equal to itself by virtue of absolute mediation. Universality with which determinate clothes itself = abstract universality. Universality = form in it, determinateness = content. In pure universal: only absolute negativity, not difference posited as such.',
      [
        'simple as principle',
        'moment of totality',
        'behaves negatively',
        'ideal moments',
        'determinate existence',
        'absolute mediation',
        'abstract universality',
        'form',
        'content',
        'essential',
        'absolute negativity',
      ],
      { section: 'The Particular Concept', order: 7 }
    ),

    createTopicMapEntry(
      'part-8-abstract-universality-form-content',
      'Abstract Universality; Form/Content; Unconceptualized Concept',
      [201, 236],
      'Determinateness = abstraction as against other determinateness. Other determinateness = only universality itself (abstract universality). Abstract-universal contains all moments of concept: (a) universality, (b) determinateness, (c) simple unity (but immediate, particularity not as totality). Essentially reference to other excluding it. Mediation only a condition, not posited. Abstract universal = concept, but unconceptualized concept (concept not posited as such).',
      [
        'abstraction',
        'determinate universality',
        'outside itself',
        'moments of concept',
        'simple unity',
        'immediate',
        'mediation',
        'reference to other',
        'sublation of negation',
        'condition',
        'immediacy',
        'indifference',
        'unconceptualized concept',
      ],
      { section: 'The Particular Concept', order: 8 }
    ),

    createTopicMapEntry(
      'part-9-understanding-abstract-universal',
      'Understanding and Abstract Universal; Principle of Differentiation',
      [238, 291],
      'Determinate concept (ordinarily meant) = abstract universal. Concept as such (generally understood) = unconceptualized concept. Understanding = its faculty. Demonstration belongs to understanding. Progression by way of concepts does not reach past finitude and necessity. Abstraction not as empty as usually said. Any determinate concept empty if determinateness not principle of differentiation. Principle contains beginning and essence of development/realization. To reproach concept as empty = ignore its absolute determinateness.',
      [
        'determinate concept',
        'abstract universal',
        'understanding',
        'faculty',
        'demonstration',
        'finitude',
        'necessity',
        'negative infinite',
        'absolute substance',
        'abstraction',
        'empty',
        'determinateness',
        'principle of differentiation',
        'absolute determinateness',
      ],
      { section: 'The Particular Concept', order: 9 }
    ),

    createTopicMapEntry(
      'part-10-understanding-force-fixity',
      'Understanding\'s Force and Fixity; Dialectical and Appearance of Reason',
      [293, 412],
      'Understanding held in low repute. Fixity = form of abstract universality (makes unalterable). Infinite force of understanding = splitting concrete into abstract determinacies. Understanding gives rigidity of being. By simplifying, understanding quickens with spirit, sharpens them. Only at that point do they obtain capacity to dissolve themselves. Ripest maturity = stage at which fall begins. Universality directly contradicts determinateness of finite. Common practice of separating understanding and reason = to be rejected. Determinate and abstract concept = essential moment of reason. Beginning of appearance of reason.',
      [
        'understanding',
        'fixity',
        'abstract universality',
        'unalterable',
        'immanent reflection',
        'eternity',
        'eternal essentialities',
        'imperfect universality',
        'infinite force',
        'splitting concrete',
        'transition',
        'sensuous totality',
        'mutability',
        'intellectual intuition',
        'objective totality',
        'idea',
        'stable subsistence',
        'subjective impotence',
        'dialectical force',
        'rigidity',
        'quickens with spirit',
        'ripest maturity',
        'dissolution',
        'disproportion',
        'void of reason',
        'essential moment',
        'dialectical',
        'appearance of reason',
      ],
      { section: 'The Particular Concept', order: 10 }
    ),

    createTopicMapEntry(
      'part-11-understanding-reason',
      'Understanding and Reason; Essential Moment',
      [414, 426],
      'Common practice of separating understanding and reason = to be rejected. To consider concept void of reason = incapacity of reason to recognize itself in concept. Determinate and abstract concept = condition (essential moment) of reason. Form quickened by spirit in which finite (through universality referring to itself) internally kindled, posited as dialectical. Beginning of appearance of reason.',
      [
        'separating understanding and reason',
        'void of reason',
        'essential moment',
        'quickened by spirit',
        'dialectical',
        'appearance of reason',
      ],
      { section: 'The Particular Concept', order: 11 }
    ),

    createTopicMapEntry(
      'part-12-transition-singularity',
      'Transition to Singularity; Absolute Turning Back',
      [428, 453],
      'Difference receives its due in determinate concept. Determinateness in form of universality united to form simple. Determinate universality = self-referring determinateness, absolute negativity posited for itself. Self-referring determinateness = singularity. Universality immediately = particularity in and for itself. Particularity immediately = singularity in and for itself. Singularity = third moment of concept. Also = absolute turning back of concept into itself. At same time = posited loss of itself.',
      [
        'determinate concept',
        'difference',
        'determinate universality',
        'self-referring determinateness',
        'absolute negativity',
        'singularity',
        'third moment',
        'absolute turning back',
        'posited loss',
      ],
      { section: 'The Particular Concept', order: 12 }
    ),
  ],
  {
    sectionDescription: 'The Particular Concept - Determinateness of the concept as particularity. Universal\'s own immanent moment. Totality, completeness, true logical division. Understanding and abstract universality. Transition to singularity.',
  }
);

