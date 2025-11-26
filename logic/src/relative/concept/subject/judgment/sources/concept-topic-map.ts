/**
 * TopicMap for Concept.txt - The Judgment of the Concept
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
 * - concept-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 *
 * Note: All mainline text is included, including examples, even though examples won't become Logical Operations.
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const CONCEPT_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/judgment/sources/concept.txt',
  'Hegel\'s Science of Logic - The Judgment',
  'The Judgment of the Concept',
  [
    createTopicMapEntry(
      'conc-1-introduction-ought',
      'Introduction: Concept as Ought; True Adjudication',
      [4, 19],
      'Judgments of existence = hardly sign of great power. Judgments of reflection = propositions. Judgment of necessity = objective universality. Judgment of concept = connection with concept found. Concept = ought to which reality may or may not conform. First contains true adjudication. Predicates: "good," "bad," "true," "right" = fact measured against concept as ought.',
      [
        'judgments of existence',
        'judgments of reflection',
        'objective universality',
        'connection with concept',
        'ought',
        'true adjudication',
        'measured against concept',
        'agreement',
      ],
      { section: 'The Judgment of the Concept', order: 1 }
    ),

    createTopicMapEntry(
      'conc-2-modality',
      'Judgment of Modality: Problematic, Assertoric, Apodictic; Objective Truth',
      [21, 56],
      'Called judgment of modality. Problematic: optional/possible. Assertoric: true/actual. Apodictic: necessary. Concept (subjective) relates to immediate actuality. Not confused with external reflection. Concept from disjunctive = opposite of mere mode. Earlier judgments subjective (abstract one-sidedness). Judgment of concept = objective, truth (rests on concept in determinateness as concept).',
      [
        'judgment of modality',
        'problematic',
        'assertoric',
        'apodictic',
        'external understanding',
        'subjective',
        'external reflection',
        'mere mode',
        'objective',
        'truth',
        'determinateness as concept',
      ],
      { section: 'The Judgment of the Concept', order: 2 }
    ),

    createTopicMapEntry(
      'conc-3-diremption',
      'Concept from Disjunctive; Diremption into Extremes',
      [58, 91],
      'In disjunctive: concept = identity of universal nature and particularization (relation sublated). Concretion = simple result (must develop into totality). Shortcoming: negative unity not determined as singularity. Result = negative unity = singularity. Must posit negativity, part into extremes, conclude in syllogistic conclusion. Proximate diremption = judgment (unity as subject and predicate).',
      [
        'identity',
        'universal nature',
        'particularization',
        'sublated',
        'concretion',
        'totality',
        'determinate self-subsistence',
        'negative unity',
        'singularity',
        'diremption',
        'extremes',
        'syllogistic conclusion',
      ],
      { section: 'The Judgment of the Concept', order: 3 }
    ),

    createTopicMapEntry(
      'conc-4-assertoric-immediate',
      'Assertoric Judgment: Immediate; Ought and Constitution',
      [95, 125],
      'At first immediate = assertoric. Subject = concrete singular. Predicate = connection of actuality/determinateness/constitution to concept. Examples: "This house is bad," "this action is good." Contains: (a) subject ought to be (universal nature = self-subsistent concept), (b) particularity = constituted/external (differs, indifferent, may or may not conform). Constitution = singularity. Concrete universality divides into extremes (concept as unity still lacking).',
      [
        'immediate',
        'assertoric',
        'concrete singular',
        'connection',
        'actuality',
        'determinateness',
        'constitution',
        'ought',
        'self-subsistent concept',
        'external concrete existence',
        'indifferent',
        'conform',
        'singularity',
        'extremes',
      ],
      { section: 'The Assertoric Judgment', order: 4 }
    ),

    createTopicMapEntry(
      'conc-5-assertoric-subjective',
      'Assertoric: Subjective Assurance; External Third',
      [127, 147],
      'Only assertoric (credential = subjective assurance). Good/bad/right hangs on external third. Connectedness externally posited = still in itself/internal. Don\'t mean only in subjective consciousness (may be predicates of object). Merely subjective = implicitly present connectedness not posited (or only external). Copula = immediate abstract being.',
      [
        'subjective assurance',
        'external third',
        'externally posited',
        'in itself',
        'internal',
        'subjective consciousness',
        'implicitly present',
        'not posited',
        'immediate abstract being',
      ],
      { section: 'The Assertoric Judgment', order: 5 }
    ),

    createTopicMapEntry(
      'conc-6-assertoric-problematic',
      'Assertoric Passes to Problematic; Contingent Conformity',
      [149, 164],
      'Assurance can be confronted by opposing ("this action is good" vs "bad" - equal justification). Subject = immediate singular (abstraction, no determinateness containing connection). Contingent whether conformity to concept. Essentially = problematic.',
      [
        'opposing assurance',
        'equal justification',
        'immediate singular',
        'abstraction',
        'determinateness',
        'connection',
        'contingent',
        'conformity',
        'problematic',
      ],
      { section: 'The Assertoric Judgment', order: 6 }
    ),

    createTopicMapEntry(
      'conc-7-problematic-differentiated',
      'Problematic Judgment: Subject Differentiated into Ought and Constitution',
      [168, 224],
      'Problematic = assertoric positive and negative. More immanent (predicate = connection to concept, immediate as contingent). Problematic element falls on immediacy of subject (contingency). Must not abstract from singularity. Subject differentiated into universality/ought and particularized constitution. Contains ground for being or not being what it ought. Equated with predicate. Negativity = original partition into universal and particular (already unity).',
      [
        'positive and negative',
        'immanent',
        'connection to concept',
        'contingent',
        'indeterminateness',
        'copula',
        'objective concrete universality',
        'immediacy',
        'contingency',
        'singularity',
        'moment of contingency',
        'differentiated',
        'ought',
        'particularized constitution',
        'ground',
        'equated',
        'original partition',
      ],
      { section: 'The Problematic Judgment', order: 7 }
    ),

    createTopicMapEntry(
      'conc-8-two-meanings-subjectivity',
      'Two Meanings of Subjectivity; Duplicity',
      [226, 252],
      'Both sides (concept and constitution) = subjectivity. Concept = universal essence, negative self-unity (fact\'s subjectivity). Fact = contingent, external constitution (mere subjectivity vs objectivity). Fact = concept negates universality, projects into externality. Subject = duplicity. Truth: two meanings = in one. Meaning become problematic. Subjectivity has no truth in one alone. Duplicity = manifestation of one-sidedness.',
      [
        'two meanings',
        'subjectivity',
        'universal essence',
        'negative self-unity',
        'contingent',
        'external constitution',
        'mere subjectivity',
        'objectivity',
        'self-negating unity',
        'projects',
        'externality',
        'duplicity',
        'in one',
        'problematic',
        'one-sidedness',
      ],
      { section: 'The Problematic Judgment', order: 8 }
    ),

    createTopicMapEntry(
      'conc-9-problematic-apodictic',
      'Problematic Passes to Apodictic',
      [254, 258],
      'When problematic character posited as character of fact (with constitution), judgment no longer problematic but apodictic.',
      [
        'problematic character',
        'character of fact',
        'constitution',
        'apodictic',
      ],
      { section: 'The Problematic Judgment', order: 9 }
    ),

    createTopicMapEntry(
      'conc-10-apodictic-objective',
      'Apodictic Judgment: Truly Objective; Correspondence',
      [262, 288],
      'Examples: "the house, as so and so constituted, is good," "the action, as so and so constituted, is right." Subject includes: universal/ought and constitution (ground). Truly objective = truth of judgment. Subject and predicate correspond, same concept. Content = concrete universality (objective universal/genus and singularized universal). Universal continues through opposite, universal only in unity. Correspondence = universality constituting predicate.',
      [
        'as so and so constituted',
        'universal',
        'ought',
        'constitution',
        'ground',
        'corresponds',
        'truly objective',
        'truth',
        'correspond',
        'same concept',
        'concrete universality',
        'objective universal',
        'genus',
        'singularized universal',
        'continues through opposite',
        'unity',
        'correspondence',
      ],
      { section: 'The Apodictic Judgment', order: 10 }
    ),

    createTopicMapEntry(
      'conc-11-absolute-judgment',
      'Apodictic: Absolute Judgment on Actuality; Soul',
      [290, 302],
      'Subject contains two moments in immediate unity as fact. Truth = internally fractured into ought and being (absolute judgment on all actuality). Original partition = omnipotence of concept. Equally turning back into unity. Absolute connection of "ought" and "being" = makes actual into fact. Inner connection (concrete identity) = its soul.',
      [
        'immediate unity',
        'fact',
        'internally fractured',
        'ought and being',
        'absolute judgment',
        'actuality',
        'original partition',
        'omnipotence of concept',
        'turning back',
        'absolute connection',
        'inner connection',
        'concrete identity',
        'soul',
      ],
      { section: 'The Apodictic Judgment', order: 11 }
    ),

    createTopicMapEntry(
      'conc-12-ground-copula',
      'Ground in Constitution; Accomplished Copula',
      [304, 325],
      'Transition lies in particular determinateness. Genus = unconnected. Determinateness = reflected into itself and into other. Judgment has ground in constitution = apodictic. Accomplished copula (abstract "is" developed into ground). Attaches to subject as immediate determinateness, equally connection to predicate. Predicate = correspondence itself (connection to universality).',
      [
        'transition',
        'immediate simplicity',
        'correspondence',
        'determinate connection',
        'particular determinateness',
        'genus',
        'unconnected',
        'reflected into itself',
        'into an other',
        'ground',
        'constitution',
        'accomplished copula',
        'ground in general',
        'immediate determinateness',
        'connection',
        'correspondence',
      ],
      { section: 'The Apodictic Judgment', order: 12 }
    ),

    createTopicMapEntry(
      'conc-13-form-passes-away',
      'Form of Judgment Passes Away; Identity Recovered',
      [327, 344],
      'Form passed away: (1) subject/predicate same content, (2) subject points beyond, connects, (3) connecting passed over into predicate (constitutes content, connecting as posited = judgment). Concrete identity of concept (result of disjunctive, inner foundation) = recovered in whole.',
      [
        'passed away',
        'same content',
        'determinateness',
        'points beyond',
        'connects',
        'passed over',
        'constitutes content',
        'connecting as posited',
        'concrete identity',
        'inner foundation',
        'recovered',
      ],
      { section: 'The Apodictic Judgment', order: 13 }
    ),

    createTopicMapEntry(
      'conc-14-transition-syllogism',
      'Transition to Syllogism; Copula Replete of Content',
      [346, 374],
      'Subject and predicate each whole concept. Unity of concept (determinateness constituting copula) = distinct. Essence = to connect (universal running through). Same content. Form of connection posited through determinateness (as universal or particularity). Contains form determinations of extremes and determinate connection. Accomplished copula, replete of content, unity re-emerging. By virtue of repletion, judgment has become syllogism.',
      [
        'whole concept',
        'unity of concept',
        'determinateness',
        'copula',
        'distinct',
        'immediate constitution',
        'essence',
        'to connect',
        'universal running through',
        'same content',
        'form of connection',
        'form determinations',
        'extremes',
        'accomplished copula',
        'replete of content',
        're-emerging',
        'repletion',
        'syllogism',
      ],
      { section: 'The Apodictic Judgment', order: 14 }
    ),
  ],
  {
    sectionDescription: 'The Judgment of the Concept - Concept as ought, true adjudication. Assertoric (immediate, subjective assurance). Problematic (subject differentiated). Apodictic (truly objective, ground in constitution). Transition to syllogism. Note: All mainline text included, including examples.',
  }
);

