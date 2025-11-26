/**
 * TopicMap for idea-of-the-true.txt - The Idea of the True
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
 * - idea-of-the-true-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const IDEA_OF_THE_TRUE_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/cognition/sources/idea-of-the-true.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Idea of the True',
  [
    createTopicMapEntry(
      'idea-true-1-impulse-finite',
      'Impulse of Truth; Theoretical Idea; Finite Cognition',
      [1, 656],
      'Subjective idea = impulse. Contradiction: concept has itself as subject matter, is reality, but subject matter not other subsisting. Impulse: sublate subjectivity, make abstract reality concrete, fill with content. Concept = absolute certainty, presupposition of world = unessential. Impulse: sublate otherness, intuit identity in object. Immanent reflection = sublated opposition, singularity posited = content. Impulse of truth = theoretical idea. Truth = relation of judgment, comparison connecting concept with actuality. Cognition = idea as purpose, first negation of world. Conclusion: objective posited in subjective = only subjective. Neutral unity/synthesis = externally conjoined. Cognition retains finitude, not attained purpose/truth. Finite cognition: thing-in-itself as absolute beyond, categories as finite/subjective.',
      [
        'impulse',
        'truth',
        'theoretical idea',
        'contradiction',
        'immanent reflection',
        'content',
        'finite cognition',
        'synthesis',
        'thing-in-itself',
      ],
      { section: 'The Idea of the True', order: 1 }
    ),

    createTopicMapEntry(
      'idea-true-2-analytic-nature',
      'Analytic Cognition - Nature and Principle; Identity',
      [753, 947],
      'First premise of syllogism, no mediation. Immediate communication of concept, no otherness, activity divests negativity. Immediacy = mediation (negative reference annuls itself). Immanent reflection = subjective, difference still presupposition. Result = simple identity, abstract universality. Principle = identity, transition into other excluded. Starts from presupposition = singular concrete subject matter. Analysis = transformation into logical determinations. Activity = explication of what is in object (object = totality of concept). Not one-sided: not only imported, not only extracted. Positing = presupposing (both in one). Logical element: in cognition and in itself. No further middle terms, determination = immediate. Progress = explication of differences, but void of concept, only given difference. Relations = given, not progress. Kant: synthetic principles a priori, but takes connections from formal logic as given.',
      [
        'analytic cognition',
        'immediate communication',
        'identity',
        'abstract universality',
        'presupposition',
        'logical determinations',
        'positing',
        'presupposing',
      ],
      { section: 'The Idea of the True', order: 2 }
    ),

    createTopicMapEntry(
      'idea-true-3-analytic-arithmetic',
      'Analytic Cognition - Arithmetic and Analysis; Problems',
      [949, 1207],
      'Analytical science = arithmetic, discrete magnitude. Material = abstract indeterminate product, principle = discrete magnitude (one). Relationless atom, external determination/joining. Guiding principle = analytical identity (equality in diversity). Progression = reduction of unequal to greater equality. Addition = unequal numbers, multiplication = equal numbers, powers. Determinateness = posited, further operation = analytic. Analytical science = problems (not theorems). Theorem = problem already resolved, trivial identity. Kant: 5 + 7 = 12 synthetic, but Hegel: same content, continuation/repetition. Proof = operation of counting, problem form. Higher analysis: synthetic determinations when qualitative relations come into play. Differential/integral calculus: qualitative determination, transition not analytic/mathematical. Analysis becomes synthetic when determinations not posited by problems. General transition: immediacy → mediation, abstract identity → difference.',
      [
        'arithmetic',
        'discrete magnitude',
        'analytical identity',
        'equality',
        'problems',
        'theorems',
        'differential calculus',
        'qualitative determination',
        'synthetic',
      ],
      { section: 'The Idea of the True', order: 3 }
    ),

    createTopicMapEntry(
      'idea-true-4-synthetic-definition',
      'Synthetic Cognition - Definition; Genus and Marks',
      [1208, 1648],
      'Second premise of syllogism, diverse connected. Aim = necessity, identity in diversity (still inner, not subjective). Finitude: identity inner, determinations external, lacks singularity. Transforms objective world into concepts, but only form, singularity = given. Definition: given objectivity → simple form (concept). Moments = universality, particularity, singularity. Singular = object as immediate representation, to be defined. Universal = proximate genus (universal with determinateness = principle for differentiation). Definition reduces to concept, gets rid of externalities. Content = given and contingent (content contingent, which determinations chosen = accident). No principle for determining which aspects = conceptual vs external reality. Products of purposiveness: easy definition. Geometrical objects: abstract determinations of space. Concrete objects: many properties, depends on proximate genus/specific difference. Essentiality = universality (empirical). Definition = immediate concept, employs immediate property, abstraction. Forfeits true concept determinations, contents itself with marks. Single external determinateness = disproportionate. Concept vs realization: external presentation, transitoriness, inadequacy. Bad specimens: property missing. Immediacy proceeds from mediation → division.',
      [
        'synthetic cognition',
        'definition',
        'genus',
        'particularity',
        'singularity',
        'contingency',
        'marks',
        'externality',
        'immediacy',
        'mediation',
      ],
      { section: 'The Idea of the True', order: 4 }
    ),

    createTopicMapEntry(
      'idea-true-5-synthetic-division',
      'Synthetic Cognition - Division; Universal to Particular',
      [1649, 2013],
      'Universal must particularize itself, necessity lies in universal. Definition begins with particular → points to other → division. Particular separates from universal, universal presupposed. Singular content raised through particularity to universality. Transition: universal → particular (form of concept). Basis for synthetic science, system, systematic cognition. Beginning = universal (simple, abstracted from concrete). Universal = first moment (simple), particular = mediated. Examples: reading (elements first), geometry (point/line first), physics (simple properties), colors (spectrum). Abstract = starting point, element from which concrete spreads. Universal = determinate, member of division, higher universal to infinity. No immanent limit, proceeds from given, abstract universality = "first". Division = immediately next step, but lacks immanent principle. Follows form determination without immanent reflection, takes content from given. No specific reason for particular (basis or relation). Business = orderly arranging + discovering universal by comparison. Grounds of division = variety. Lack of self-determination → formal empty rules. Rule: exhaust concept (but each member exhausts, tautological). Empirical expansion: species discovered, genus altered or material excluded. Way without concept = game of chance. Physical nature = contingency in principles. General determinateness = diversity (not opposition). Externality = indifference of difference, number for division. Instinct of reason: bases more adequate to concept. Animals: teeth/claws = vital point. Plants: reproductive parts = highest point.',
      [
        'division',
        'universal',
        'particular',
        'systematic cognition',
        'abstract',
        'concrete',
        'contingency',
        'ground of division',
        'diversity',
        'instinct of reason',
      ],
      { section: 'The Idea of the True', order: 5 }
    ),

    createTopicMapEntry(
      'idea-true-6-synthetic-theorem',
      'Synthetic Cognition - The Theorem; Proof and Construction; Practical Idea',
      [2014, 2727],
      'Third stage: particularity → singularity = theorem. Self-referring determinateness, internal differentiation, connection of determinacies. Definition = one determinateness, division = determinateness against other, singularization = parted internally. Theorem = subject matter in reality, conditions/forms of real existence. Together with definition = idea (unity of concept and reality). But reality does not proceed from concept, unity not cognized. Theorem = synthetic element, relations necessary (grounded in inner identity). Content = determinations of reality, no longer simple immediate determinations. Concept gone over to otherness/reality = idea. Synthesis = joining diverse, unity to be demonstrated, proof necessary. Difficulty: which determinations = definition vs theorem. Euclid: axiom of parallel lines (presupposition, relative first). Axioms = theorems (mostly from logic), relative firsts. Euclidean geometry: order in progression. Initial theorems: congruence. Pythagorean theorem = perfect real definition, equation of sides. Truly synthetic progression: universal → singularity (determined in and for itself). Theorem must be proved: relation of real determinations (not concept determinations). Mediation: simple or several, mediating members joined. Construction: mediating determinations imported, provisory material. Material acquires meaning in proof, appears blind/meaningless. Proof: mediation, necessity only to insight, external reflection. Sequence = reverse of nature, ground = subjective ground. Geometry: science of magnitude, formal inference, abstract subject matter. Space = abstract emptiness, figures at rest, no immanent transition. Driven to infinity: positing equal what is qualitatively diverse. Limit: necessity not grounded in positive identity, but negative identity. Synthetic method insufficient for other sciences, philosophy. Empirical sciences: explanation = tautology/obfuscation. Kant: content leads to contradictions. Jacobi: inadequacy = method, bound to finite reality, freedom beyond. Philosophy = infinite free concept, method of finitude inadequate. Synthesis/mediation = necessity opposed to freedom, identity of dependent (implicit). Reality = self-subsistent diversity = finite. Identity = abstract, concept precluded. Idea achieves purpose: concept becomes for concept (identity + real determinations). But subject matter not adequate, concept not unity of itself with itself. In necessity: identity for it, but necessity not determinateness (material external). Concept not for itself, not determined in and for itself. Idea does not attain truth: disproportion between subject matter and subjective concept. Necessity = highest point of being/reflection, passes over into freedom of concept. Transition: inner identity → manifestation = concept as concept. Result: idea = practical idea, action.',
      [
        'theorem',
        'singularity',
        'synthetic element',
        'proof',
        'construction',
        'mediation',
        'geometry',
        'necessity',
        'freedom',
        'concept',
        'practical idea',
      ],
      { section: 'The Idea of the True', order: 6 }
    ),
  ],
  {
    sectionDescription: 'The Idea of the True - Impulse of truth, theoretical idea, finite cognition. Analytic cognition (nature, arithmetic). Synthetic cognition (definition, division, theorem). Proof and construction. Transition to practical idea.',
  }
);

