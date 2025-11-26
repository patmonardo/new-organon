/**
 * TopicMap for cognition.txt - The Idea of Cognition (Introduction/Genera)
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
 * - cognition-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const COGNITION_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/cognition/sources/cognition.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Idea of Cognition (Introduction/Genera)',
  [
    createTopicMapEntry(
      'cognition-1-life-cognition',
      'Life as Immediate Idea; Cognition in General; Elevation Above Life',
      [6, 41],
      'Life = immediate idea, internally unrealized concept. In judgment, idea = cognition in general. Concept for itself = abstract universality/genus. Pure self-identity internally differentiates. Differentiated = liberated into subjectivity, not objectivity. Object facing concept = concept itself. Elevation above life: reality = concept-form liberated into universality. Through judgment: idea doubled = subjective concept (reality = concept itself) + objective concept (life). Thought, spirit, self-consciousness = determinations of idea having itself as subject matter.',
      [
        'life',
        'immediate idea',
        'cognition',
        'concept',
        'abstract universality',
        'genus',
        'self-identity',
        'subjectivity',
        'objectivity',
        'judgment',
        'spirit',
        'self-consciousness',
      ],
      { section: 'The Idea of Cognition (Introduction/Genera)', order: 1 }
    ),

    createTopicMapEntry(
      'cognition-2-kant-critique',
      'Metaphysics of Spirit/Soul; Kant\'s Critique; Self-Consciousness',
      [43, 239],
      'Former metaphysics: substance, simplicity, immateriality. Spirit = absolute unity in concept of opposites, contradiction at extreme. Kant\'s critique: rational psychology, "I" = empty representation, transcendental subject = x. Paralogism: modes of self-consciousness converted into concepts of understanding. Hegel\'s critique: "I" thinks itself = eternal nature of self-consciousness/concept. Self-reference = absolute self-reference, parting judgment, circle. Inseparability belongs to concept. Kant\'s abstraction = conceptual void. Mendelssohn\'s proof vs Kant\'s refutation. Former metaphysics aimed at truth; Kant sidelines truth investigation. Concept flies high, rises above appearance/representation.',
      [
        'metaphysics',
        'spirit',
        'soul',
        'substance',
        'simplicity',
        'Kant',
        'critique',
        'self-consciousness',
        'paralogism',
        'transcendental subject',
        'concept',
        'abstraction',
        'truth',
      ],
      { section: 'The Idea of Cognition (Introduction/Genera)', order: 2 }
    ),

    createTopicMapEntry(
      'cognition-3-life-spirit',
      'Transition from Life to Spirit; Soul, Consciousness, Spirit',
      [318, 460],
      'From idea of life → idea of spirit = truth of life. Life: reality = singularity, universality/genus = inwardness. Truth of life: sublate abstract/immediate singularity, be self-identical as genus. This idea = spirit. Spirit in logical form vs concrete sciences (soul, consciousness, spirit as such). Soul: singular finite spirit, thing-like, monads. Anthropology: concept in immediate existence. Consciousness: free concept as "I", withdrawn from objectivity. Phenomenology: spirit appearing, exhibiting in contrary. Spirit for itself: infinite in itself and form. Logical idea of spirit: already in pure science, "I" emerged from concept of nature. Free concept in judgment = subject matter confronting it = concept as idea. Still not consummated.',
      [
        'life',
        'spirit',
        'truth',
        'singularity',
        'universality',
        'genus',
        'soul',
        'consciousness',
        'anthropology',
        'phenomenology',
        'logical idea',
        'free concept',
      ],
      { section: 'The Idea of Cognition (Introduction/Genera)', order: 3 }
    ),

    createTopicMapEntry(
      'cognition-4-purpose-truth',
      'Idea as Purpose Seeking Truth; Theoretical Idea',
      [462, 531],
      'Idea = free concept having itself as subject matter, but immediate. Still idea in subjectivity = finitude. Purpose that ought to realize itself = absolute idea in appearance. Seeks truth = identity of concept and reality, but only seeks it, still subjective. Subject matter = given, but transformed into conceptual determination. Concept = active principle referring to itself, finds truth. Idea = one extreme of syllogism (concept as purpose) + other extreme (objective world). Unity posited through cognition = middle term. Process: positing concrete reality as identical with concept. Theoretical idea = cognition as such. Objective world = form of immediacy/being. Concept = form, universality/particularity = reality within, singularity/content = received from outside.',
      [
        'idea',
        'purpose',
        'truth',
        'subjectivity',
        'finitude',
        'syllogism',
        'cognition',
        'theoretical idea',
        'concept',
        'objectivity',
        'universality',
        'particularity',
        'singularity',
      ],
      { section: 'The Idea of Cognition (Introduction/Genera)', order: 4 }
    ),
  ],
  {
    sectionDescription: 'The Idea of Cognition (Introduction/Genera) - Life as immediate idea, cognition in general. Concept for itself, elevation above life. Metaphysics of spirit/soul, Kant\'s critique. Transition from life to spirit. Idea as purpose seeking truth, theoretical idea.',
  }
);

