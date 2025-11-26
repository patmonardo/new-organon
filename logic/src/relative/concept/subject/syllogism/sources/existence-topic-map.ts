/**
 * TopicMap for Existence.txt - The Syllogism of Existence
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
 *
 * Note: All mainline text included, including examples, even though examples won't become Logical Operations.
 * Key Insight: Reason and Understanding are inseparable like Immediacy and Mediacy. The immediacy of Reason is internal.
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const EXISTENCE_SYLLOGISM_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/syllogism/sources/existence.txt',
  'Hegel\'s Science of Logic - The Syllogism',
  'The Syllogism of Existence',
  [
    createTopicMapEntry(
      'syl-exist-1-introduction-immediate',
      'Introduction: Immediate Form; Particularity as Middle Term',
      [4, 36],
      'Syllogism in immediate form has determinations as immediate. Abstract determinacies of form (not developed into concretion, only singular determinacies). First syllogism = strictly formal. Formalism = stopping short at form. Concept partitioned: S and U = extremes, itself = P between them. Particularity = middle term (unites S and U immediately). Dialectical movement = positing moments of mediation.',
      [
        'immediate form',
        'abstract determinacies',
        'singular determinacies',
        'strictly formal',
        'formalism',
        'extremes',
        'particularity',
        'middle term',
        'double-sidedness',
        'simple determinateness',
        'dialectical movement',
      ],
      { section: 'The Syllogism of Existence', order: 1 }
    ),

    createTopicMapEntry(
      'syl-exist-2-first-schema',
      'First Figure: S-P-U Schema',
      [40, 53],
      'S-P-U = general schema. Singularity connects with universality through particularity. Singular not universal immediately but by means of particularity. Universality lowers itself through particularity. Determinations stand as extremes, one in third term. Both determinateness (identical, universal determinateness = particularity). But also extremes with respect to particularity and each other.',
      [
        'S-P-U',
        'schema',
        'connects',
        'by means of',
        'lowers itself',
        'extremes',
        'third term',
        'identical',
        'universal determinateness',
        'immediate determinateness',
      ],
      { section: 'First Figure', order: 2 }
    ),

    createTopicMapEntry(
      'syl-exist-3-general-meaning',
      'General Meaning: Singular Emerges into Existence; Still Subjective',
      [55, 78],
      'Singular (infinite self-reference, inwardness) emerges through particularity into existence (universality, external conjunction). Conversely: sets itself apart as particularity (concreted term, self-referring universal, true singular). In universality: gone from externality into itself. Objective significance only superficially present. Still subjective (abstract meaning only in subjective consciousness). Relation S-P-U = necessary essential form-relation.',
      [
        'infinite self-reference',
        'inwardness',
        'emerges',
        'external conjunction',
        'concreted term',
        'self-referring universal',
        'true singular',
        'externality into itself',
        'objective significance',
        'subjective consciousness',
        'form-relation',
      ],
      { section: 'First Figure', order: 3 }
    ),

    createTopicMapEntry(
      'syl-exist-4-aristotle',
      'Aristotle\'s Definition: Relation of Inherence; Variant Forms',
      [87, 115],
      'Aristotle: mere relation of inherence. One extreme in entire middle, middle in entire other extreme → extremes united. Repetition of equal relation (not determinateness of three terms). Other relations have validity only reducible to original. Not diverse species but variant forms into which first abstract form necessarily passes over (further determines, becomes totality).',
      [
        'relation of inherence',
        'repetition',
        'equal relation',
        'determinateness',
        'inferences of understanding',
        'reducible',
        'variant forms',
        'necessarily passes over',
        'totality',
      ],
      { section: 'First Figure', order: 4 }
    ),

    createTopicMapEntry(
      'syl-exist-5-subsumption',
      'S-P-U: Subsumption and Inherence',
      [117, 132],
      'S-P-U = general schema. Singular subsumed under particular, particular under universal → singular subsumed under universal. Or: particular inheres in singular, universal in particular → universal inheres in singular. With respect to universal: particular = subject. With respect to singular: particular = predicate. Both determinations united → extremes joined.',
      [
        'subsumed',
        'inheres',
        'subject',
        'predicate',
        'united',
        'joined together',
      ],
      { section: 'First Figure', order: 5 }
    ),

    createTopicMapEntry(
      'syl-exist-6-therefore',
      '"Therefore" Grounded in Extremes; Not Merely Subjective',
      [133, 160],
      '"Therefore" appears as inference in subject (subjective insight). But not external determination. Grounded in nature of extremes themselves. Connection as judgment only for abstractive reflection. True connection = posited as middle term. Not merely subjective judgment (not empty "is" but determinate middle replete with content) = meaning of syllogistic inference. To regard as three judgments = formalistic view.',
      [
        'therefore',
        'inference',
        'subjective insight',
        'immediate judgments',
        'mediated connection',
        'grounded',
        'nature of extremes',
        'abstractive reflection',
        'middle term',
        'determinate middle',
        'replete with content',
        'formalistic view',
      ],
      { section: 'First Figure', order: 6 }
    ),

    createTopicMapEntry(
      'syl-exist-7-example-boredom',
      'Example: "All humans are mortal"; Boredom; All Things are Syllogism',
      [166, 210],
      'Subjective reflection splits into isolated premises and conclusion. Example: All humans mortal, Gaius human, Therefore Gaius mortal. Boredom from futility (separate propositions give illusion dissolved in fact). Appears as subjective expedient. Nature does not operate this way. Syllogistic inference = subjective form. Nature = determinations united in unity of essence. Rationality = objective element (contrast to immediacy in judgment). Prior immediacy = mere subjectivity. All things are syllogism (universal united through particularity with singularity).',
      [
        'subjective reflection',
        'isolated premises',
        'boredom',
        'futility',
        'subjective expedient',
        'unity of essence',
        'objective element',
        'mere subjectivity',
        'truth of judgment',
        'all things are syllogism',
      ],
      { section: 'First Figure', order: 7 }
    ),

    createTopicMapEntry(
      'syl-exist-8-terms-content',
      'Terms as Content: Qualitative, Singular Determinacies',
      [212, 246],
      'Terms have form of immediate determinations; consider as content. Qualitative. Terms = singular determinacies (self-reference as indifferent to form, hence content). Singular = immediate concrete. Particularity = determinacies/properties/relations. Universality = more abstract/singularized. Subject not posited in concept. Concretion not reduced. Self-referring determinateness = indeterminate, infinite manifoldness. Any determinacy may serve as middle term. Through each, attaches to another universal.',
      [
        'immediate determinations',
        'content',
        'qualitative',
        'singular determinacies',
        'self-reference',
        'indifferent to form',
        'immediate concrete',
        'infinite manifoldness',
        'middle term',
        'different arrangement',
        'concreted',
      ],
      { section: 'First Figure', order: 8 }
    ),

    createTopicMapEntry(
      'syl-exist-9-contingency-contradiction',
      'Contingency and Contradiction; Examples',
      [247, 304],
      'Entirely accidental/arbitrary which property taken. Other middle terms = transitions to other predicates. Same middle may be transition to different predicates. Syllogisms must run into contradiction. Difference = opposition. Concrete = unity in concept of opposites. Examples: wall blue/green/yellow; senses vs spirituality; gravitation vs centrifugal; sociability vs individuality.',
      [
        'accidental',
        'arbitrary',
        'transitions',
        'contingent',
        'contradiction',
        'difference',
        'opposition',
        'unity in concept',
        'opposites',
        'examples',
      ],
      { section: 'First Figure', order: 9 }
    ),

    createTopicMapEntry(
      'syl-exist-10-kant-antinomies',
      'Kant\'s Antinomies; Form\'s Abstractness',
      [305, 346],
      'Nothing as unsatisfactory as formal syllogism (middle term = chance/arbitrariness). Elegant deduction amounts to nothing (possibility opposite deduced). Kant\'s antinomies = one determination laid down at one time, another at another, both equal necessity. Insufficiency not blamed on content. Lies in form (content = one-sided quality because form\'s abstractness). One single quality (form = immediate single determinateness). Abstract singularity = infinite manifold. Abstract particularity = single quality. Abstract universal. Totally contingent as regards content.',
      [
        'unsatisfactory',
        'chance',
        'arbitrariness',
        'Kant\'s antinomies',
        'equal necessity',
        'insufficiency',
        'contingency',
        'one-sided quality',
        'abstractness',
        'immediate single determinateness',
        'abstract singularity',
        'infinite manifold',
        'abstract particularity',
        'abstract universal',
        'contingent',
      ],
      { section: 'First Figure', order: 10 }
    ),

    createTopicMapEntry(
      'syl-exist-11-determinations-connections',
      'Determinations as Connections; Premises Need Proof',
      [348, 378],
      'Determinations = determinations of content (reflected into themselves) but essence = determinations of form (essentially connections). Connections: extremes to middle (immediate, premises), extremes to one another (mediated, conclusion). Premises = propositions/judgments (contradict nature of syllogism). Truth of judgment = syllogistic conclusion. Premises cannot remain immediate (not identical unless empty tautologies).',
      [
        'determinations of content',
        'determinations of form',
        'essentially connections',
        'immediate connections',
        'premises',
        'major',
        'minor',
        'mediated connection',
        'conclusion',
        'contradict',
        'truth of judgment',
        'identical propositions',
        'empty tautologies',
      ],
      { section: 'First Figure', order: 11 }
    ),

    createTopicMapEntry(
      'syl-exist-12-infinite-progression',
      'Infinite Progression; Bad Infinity',
      [380, 430],
      'Premises will be proved (exhibited as conclusions). Two premises → two syllogisms → four premises → four syllogisms → eight → sixteen → geometrical progression to infinity. Progress to infinity (bad infinity, not expected in domain of concept). Contradiction of qualitative being and impotent ought. Progression = repeated demand for unity, fall back. Does not resolve but perpetually renews. Truth = sublation of it and form. Form S-P-U. If mediated in same manner: deficient form replicated to infinity. Relation supposed to be sublated comes up again.',
      [
        'proved',
        'conclusions',
        'geometrical progression',
        'infinity',
        'bad infinity',
        'qualitative being',
        'impotent ought',
        'repeated demand',
        'perpetually renews',
        'original deficiency',
        'sublation',
        'deficient form',
        'replicated',
        'comes up again',
      ],
      { section: 'First Figure', order: 12 }
    ),

    createTopicMapEntry(
      'syl-exist-13-mediation-other-way',
      'Mediation in Other Way; Second and Third Figures',
      [432, 482],
      'Mediation must occur in other way. For P-U: S available → P-S-U. For S-P: U available → S-U-P. Mediation = contingent (external arbitrary choice). Not necessary, not universal, not grounded in concept (basis = external, immediate = singular). Mediation presupposes immediacy (mediated by means of immediate). Singular become mediating term through conclusion. Conclusion S-U: singular posited as universal. In minor: already as particular. Singular = unity of two determinations. Conclusion expresses singular as universal (mediated, necessary connection). Singular posited as universality of extremes or middle (unity of extremes).',
      [
        'other way',
        'P-S-U',
        'S-U-P',
        'contingent',
        'external arbitrary choice',
        'presupposes immediacy',
        'mediated by means of immediate',
        'mediating term',
        'posited as universal',
        'already as particular',
        'united',
        'necessary connection',
        'universality of extremes',
        'unity',
      ],
      { section: 'First Figure', order: 13 }
    ),

    createTopicMapEntry(
      'syl-exist-14-second-figure',
      'Second Figure: P-S-U; Negative Unity; Singular as Middle',
      [486, 540],
      'Truth of first: not in and for itself united but by contingency/singularity. Subject conceived only in externality. Immediacy = basis → singular in truth = middle. Syllogistic connection = sublation of immediacy (negative unity). Mediation contains negative moment. Premises: P-S (immediate) and S-U (mediated). Second presupposes first, first presupposes second. Extremes = particular and universal. Particular exchanged places (subject/extreme of singularity). Objective meaning: universal not determinate particular (totality) but one species through singularity (rest excluded). Particular not immediately universal (negative unity removes determinateness, raises to universality).',
      [
        'truth',
        'contingency',
        'singularity',
        'externality',
        'basis',
        'sublation',
        'negative unity',
        'negative moment',
        'presupposes',
        'exchanged places',
        'objective meaning',
        'totality',
        'excluded',
        'negative unity',
        'removes determinateness',
      ],
      { section: 'Second Figure', order: 14 }
    ),

    createTopicMapEntry(
      'syl-exist-15-second-terms-immediate',
      'Second Figure: Terms Still Immediate',
      [542, 552],
      'Terms still immediate determinacies (not advanced to objective signification). Positions exchanged = form (external). Still each content indifferent (two qualities linked through accidental singularity).',
      [
        'immediate determinacies',
        'objective signification',
        'form',
        'external',
        'indifferent',
        'accidental singularity',
      ],
      { section: 'Second Figure', order: 15 }
    ),

    createTopicMapEntry(
      'syl-exist-16-transition-realization',
      'Transition: Realization and Alteration',
      [554, 600],
      'First = immediate syllogism (abstract form not realized). Transition = beginning of realization (negative moment) but also alteration (no longer conforms). Regarded as subjective = species should conform to genus S-P-U. But does not conform. True meaning = latter has passed over into it (truth = subjective contingent conjoining). Conclusion correct because so on its own. View that second only species = overlooks necessary transition.',
      [
        'immediate syllogism',
        'abstract form',
        'realization',
        'negative moment',
        'alteration',
        'conform',
        'species',
        'genus',
        'lack of conformity',
        'passed over',
        'subjective contingent conjoining',
        'necessary transition',
        'true form',
      ],
      { section: 'Second Figure', order: 16 }
    ),

    createTopicMapEntry(
      'syl-exist-17-second-particular',
      'Second Figure: Particular Conclusion',
      [601, 625],
      'Second figure (referred to as third). To be correct: must be commensurate with first. S-P would have opposite relation (P subsumed under S) = sublation of determinate judgment (could only occur in particular judgment). Conclusion can only be particular. Particular judgment = positive and negative (no great value). Particular and universal = extremes, indifferent. Relation indifferent (each can be major or minor).',
      [
        'second figure',
        'third',
        'commensurate',
        'opposite relation',
        'sublation',
        'particular judgment',
        'positive and negative',
        'indifferent',
        'major or minor',
      ],
      { section: 'Second Figure', order: 17 }
    ),

    createTopicMapEntry(
      'syl-exist-18-second-self-sublating',
      'Second Figure: Self-Sublating; Points to Universal',
      [627, 653],
      'Conclusion positive and negative = universal connection. Mediation of first = implicitly contingent. In second: contingency posited. Mediation = self-sublating (singularity/immediacy). What joined must be immediately identical (mediating middle = infinitely manifold/external). Self-external mediation. Externality of singularity = universality. Points beyond to mediation by means of universal. Immediacy to which points = opposite (sublated first immediacy = reflected into itself = abstract universal).',
      [
        'universal connection',
        'implicitly contingent',
        'self-sublating',
        'immediately identical',
        'infinitely manifold',
        'external determining',
        'self-external mediation',
        'externality',
        'points beyond',
        'sublated first immediacy',
        'reflected into itself',
        'abstract universal',
      ],
      { section: 'Second Figure', order: 18 }
    ),

    createTopicMapEntry(
      'syl-exist-19-transition-concept',
      'Transition: Qualitative Base, According Concept',
      [655, 670],
      'Transition like transition of being (alteration, qualitative base, immediacy of singularity). But according concept: singularity conjoins by sublating determinateness (presents as contingency). Extremes not conjoined by specific connective (not determinate unity). Positive unity = abstract universality. Middle term posited in this determination = another form.',
      [
        'transition of being',
        'alteration',
        'qualitative',
        'immediacy of singularity',
        'according concept',
        'sublating determinateness',
        'contingency',
        'specific connective',
        'determinate unity',
        'abstract universality',
        'another form',
      ],
      { section: 'Second Figure', order: 19 }
    ),

    createTopicMapEntry(
      'syl-exist-20-third-figure',
      'Third Figure: S-U-P; Reciprocal Mediation',
      [674, 705],
      'No single immediate premise. S-U mediated by first, P-U by second. Presupposes both, conversely presupposed. Determination brought to completion. Reciprocal mediation: each mediation but not totality (affected by immediacy outside). S-U-P = truth of formal syllogism. Expresses: middle = abstract universal, extremes not contained according essential determinateness. Precisely that not conjoined which supposed to be mediated. Formalism: terms have immediate content indifferent to form (form determinations not yet reflected into content).',
      [
        'no single immediate premise',
        'presupposes',
        'reciprocal mediation',
        'totality',
        'affected by immediacy',
        'truth of formal syllogism',
        'abstract universal',
        'not contained',
        'formalism',
        'immediate content',
        'indifferent to form',
        'not yet reflected',
      ],
      { section: 'Third Figure', order: 20 }
    ),

    createTopicMapEntry(
      'syl-exist-21-third-negative',
      'Third Figure: Negative Conclusion; Fourth Figure',
      [706, 759],
      'Middle = unity of extremes (abstraction from determinateness = indeterminate universal). As universal, middle = subsumes/predicate (not subsumed/subject). To conform: P-U must contain appropriate relation (occurs in negative judgment). Conclusion necessarily negative. Indifferent which determination predicate/subject, which extreme. Which premise major/minor = indifferent. Ground of fourth figure (unknown to Aristotle, void, uninteresting). Position reverse of first. Subject/predicate of negative do not have determinate relation. Totally idle.',
      [
        'unity of extremes',
        'abstraction',
        'indeterminate universal',
        'subsumes',
        'predicate',
        'conform',
        'negative judgment',
        'necessarily negative',
        'indifferent',
        'fourth figure',
        'void',
        'uninteresting',
        'totally idle',
      ],
      { section: 'Third Figure', order: 21 }
    ),

    createTopicMapEntry(
      'syl-exist-22-third-objective',
      'Third Figure: Objective Significance; Fourth Figure',
      [761, 783],
      'Objective significance: mediating term = essentially universal. But only qualitative/abstract (determinateness not contained). Conjunction must have ground outside (contingent). Since universal determined as middle and determinateness not contained, posited as indifferent/external. By bare abstraction: fourth figure arose (U-U-U, relationless). Abstracts from qualitative differentiation, external unity (equality).',
      [
        'objective significance',
        'essentially universal',
        'qualitative abstract',
        'not contained',
        'ground',
        'outside',
        'contingent',
        'indifferent external',
        'bare abstraction',
        'fourth figure',
        'relationless',
        'external unity',
        'equality',
      ],
      { section: 'Third Figure', order: 22 }
    ),

    createTopicMapEntry(
      'syl-exist-23-fourth-mathematical',
      'Fourth Figure: U-U-U, Mathematical Syllogism',
      [787, 831],
      'Mathematical syllogism: if two equal to third, then equal to each other. Inherence/subsumption done away. "Third" = mediating term (absolutely no determination). Each term can be mediating term. Which needed depends on external circumstances. Ranks as axiom (self-explanatory, neither capable nor in need of proof). Self-evidence lies in formalism (abstracts from qualitative, only quantitative). Not without presupposition (quantitative only by abstraction). Examples: lines/figures equal only magnitude; triangle equal square (not as triangle to square). No conceptual comprehension. Self-evidence rests on indigence and abstractness.',
      [
        'mathematical syllogism',
        'equal to third',
        'inherence subsumption done away',
        'absolutely no determination',
        'external circumstances',
        'axiom',
        'self-explanatory',
        'formalism',
        'quantitative equality',
        'abstraction',
        'qualitative differentiation',
        'concept determinations',
        'no conceptual comprehension',
        'indigence',
        'abstractness',
      ],
      { section: 'Fourth Figure', order: 23 }
    ),

    createTopicMapEntry(
      'syl-exist-24-result-positive',
      'Result: Positive Side; Mediation Based on Mediation',
      [833, 869],
      'Result not just abstraction. Negativity has positive side (other posited, determinateness concrete). Syllogisms all have one another for presupposition. Extremes truly conjoined only if otherwise united by identity grounded elsewhere. Middle ought to be conceptual unity but only formal determinateness. What presupposed = not given immediacy but itself mediation (of other two). Truly present = mediation based on mediation (not given immediacy). Self-referring mediation (mediation of reflection). Circle of reciprocal presupposing = turning back into itself (forms totality, other included, not outside).',
      [
        'positive side',
        'other posited',
        'concrete',
        'presupposition',
        'identity grounded elsewhere',
        'conceptual unity',
        'formal determinateness',
        'concrete unity',
        'mediation based on mediation',
        'self-referring mediation',
        'mediation of reflection',
        'circle',
        'reciprocal presupposing',
        'turning back',
        'totality',
        'included within circle',
      ],
      { section: 'Fourth Figure', order: 24 }
    ),

    createTopicMapEntry(
      'syl-exist-25-each-determination',
      'Each Determination as Middle; Transition to Reflection',
      [871, 896],
      'In whole of formal syllogisms, each determination has occupied place of middle term. As immediate: particularity. Through dialectical movement: singularity and universality. Each occupied places of extremes. Negative result = dissolution into quantitative (mathematical). Positive result: mediation through concrete identity of determinacies (not single qualitative determinateness). Deficiency/formalism = single determinateness supposed to constitute middle. Mediation = indifference of immediate/abstract determinations and positive reflection. Passed over into syllogism of reflection.',
      [
        'each determination',
        'middle term',
        'dialectical movement',
        'places of extremes',
        'negative result',
        'dissolution',
        'quantitative',
        'positive result',
        'concrete identity',
        'deficiency',
        'formalism',
        'single determinateness',
        'indifference',
        'positive reflection',
        'passed over',
        'syllogism of reflection',
      ],
      { section: 'Fourth Figure', order: 25 }
    ),
  ],
  {
    sectionDescription: 'The Syllogism of Existence - Immediate form, S-P-U schema. First figure (singular emerges into existence). Second figure P-S-U (negative unity). Third figure S-U-P (reciprocal mediation). Fourth figure U-U-U (mathematical syllogism). Transition to syllogism of reflection. Key Insight: Reason and Understanding inseparable like Immediacy and Mediacy. Reason\'s immediacy is internal.',
  }
);

