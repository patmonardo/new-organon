/**
 * TopicMap for speculation-intro.txt - The Absolute Idea (Introduction/Transition)
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
 * - speculation-intro-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '../../../types/topic-map';
import { createTopicMap, createTopicMapEntry } from '../../../types/topic-map';

export const SPECULATION_INTRO_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/speculation/sources/speculation-intro.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Absolute Idea (Introduction/Transition)',
  [
    createTopicMapEntry(
      'spec-intro-1-absolute-idea',
      'Absolute Idea as Identity — All Truth',
      [6, 72],
      'Absolute idea = identity of theoretical and practical idea. Each one-sided, idea as beyond/unattained goal, synthesis of striving, contradiction. Absolute idea = rational concept rejoining itself, turning back to life, sublated immediacy, extreme opposition. Concept = soul + free subjective concept + personality + practical objective concept + universality/cognition. Absolute idea alone = being/imperishable life/self-knowing truth/all truth. Sole subject matter of philosophy, contains all determinateness, returns through self-determination. Various shapes, philosophy recognizes it. Nature/spirit = modes of exhibiting, art/religion = modes of apprehending. Philosophy = highest mode (concept highest).',
      [
        'absolute idea',
        'identity',
        'theoretical idea',
        'practical idea',
        'contradiction',
        'rational concept',
        'personality',
        'universality',
        'cognition',
        'all truth',
        'philosophy',
        'nature',
        'spirit',
        'art',
        'religion',
      ],
      { section: 'Introduction', order: 1 }
    ),

    createTopicMapEntry(
      'spec-intro-2-logical-idea-method',
      'Logical Idea and Method as Universal Form',
      [59, 111],
      'Logicality = universal mode (all modes sublated/enveloped). Logical idea = pure essence, simple identity, reflective shining. Logic exhibits self-movement = original word (uttered, vanished). Idea = self-determination apprehending itself, pure thought, transparent. Logical idea = infinite form as content. Form determination = completed totality = pure content. Determinateness = form (not content), idea = absolutely universal. Left to consider = universal character of form = method.',
      [
        'logicality',
        'universal mode',
        'logical idea',
        'pure essence',
        'self-movement',
        'original word',
        'infinite form',
        'form determination',
        'pure content',
        'method',
      ],
      { section: 'Introduction', order: 2 }
    ),

    createTopicMapEntry(
      'spec-intro-3-method-force',
      'Method as Absolute Form and Infinite Force',
      [113, 221],
      'Method = manner of cognition proceeding, modality determined by concept, form = soul of objectivity. Content given to method = external form. Absolute form = absolute foundation/ultimate truth. Method = absolutely self-knowing concept, pure correspondence of concept and reality. Method = movement of concept itself, concept = all, universal absolute activity. Method = universal/internal/external mode, infinite force, soul/substance, proper to each fact. Universality = manner of cognition + substantiality of things. Highest/sole force of reason, impulse to find/recognize itself. Method distinction from concept, particularization. True cognition: method = determinateness in-and-for-itself of concept, identity with subjective concept.',
      [
        'method',
        'absolute form',
        'infinite force',
        'self-knowing concept',
        'universal absolute activity',
        'soul',
        'substance',
        'reason',
        'true cognition',
        'identity',
      ],
      { section: 'Introduction', order: 3 }
    ),

    createTopicMapEntry(
      'spec-intro-4-transition-nature',
      'Logic\'s Completion and Transition to Nature',
      [223, 295],
      'Logic has apprehended its own concept. In being, concept external to content. In absolute cognition, concept = idea\'s own content. Idea = pure concept with itself as subject matter, runs through determinations, builds to system of science. Idea still logical, shut up in pure thought, science of divine concept. Impulse to sublate subjectivity, pure truth = beginning of another sphere. Transition: idea positing itself as absolute unity = nature (totality). Not transition (not become), but absolute liberation. No immediate determination not posited/concept. Idea freely discharges itself, certain/at rest. Form = absolutely free, externality of space/time without subjectivity. Externality = objectivity/external life (for consciousness), but within idea = totality of concept. Mediation: concept raises itself up, completes self-liberation in spirit, finds highest concept in logic.',
      [
        'logic',
        'concept',
        'absolute cognition',
        'system of science',
        'pure thought',
        'divine concept',
        'transition',
        'nature',
        'absolute liberation',
        'externality',
        'space',
        'time',
        'spirit',
        'self-liberation',
      ],
      { section: 'Introduction', order: 4 }
    ),
  ]
);

