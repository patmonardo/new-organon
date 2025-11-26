/**
 * TopicMap for idea-of-the-good.txt - The Idea of the Good
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
 * - idea-of-the-good-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const IDEA_OF_THE_GOOD_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/cognition/sources/idea-of-the-good.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Idea of the Good',
  [
    createTopicMapEntry(
      'idea-good-1-practical-good',
      'Practical Idea; The Good; Self-Determination',
      [1, 95],
      'Concept determined in and for itself → subject determined as singular. Subjective has implicit otherness as presupposition. Impulse to realize itself, purpose wanting to give itself objectivity. Theoretical idea: subjective concept = universal lacking determination, stands opposed to objective world. Practical idea: as actual stands over against actual. Certainty of itself = certainty of actuality + non-actuality of world. Subject vindicated objectivity for itself, inner determinateness = objective (universality absolutely determined). Previously objective world = posited, immediate, eludes unity of concept = nullity. Determinateness in concept = equal to concept, demand for singular external actuality = the good. Good = absolute, totality of concept, objective in form of free unity and subjectivity. Superior to idea of cognition: value of universal + absolutely actual. Impulse: actual still subjective, self-positing, without immediate presupposition. Activity: positing own determination, sublating determinations of external world, giving itself reality as external actuality. Will as self-determining = content within itself. Content = determinate, finite, restricted, but infinite by form of concept. Good = valid in and for itself, certain particular purpose, already true.',
      [
        'practical idea',
        'good',
        'impulse',
        'purpose',
        'subjectivity',
        'objectivity',
        'will',
        'self-determination',
        'finitude',
        'absolute',
      ],
      { section: 'The Idea of the Good', order: 1 }
    ),

    createTopicMapEntry(
      'idea-good-2-realization-finitude',
      'Realization and Finitude; The Ought; Completion in True',
      [96, 195],
      'Syllogism of immediate realization = syllogism of external purposiveness (content different). Content: finite but absolutely valid (good). Realized purpose: finite purpose = means. Good fixed as finite: notwithstanding inner infinity, cannot escape finitude. Realized good: good by virtue of what already is in subjective purpose/idea. Realization = external existence (externality in and for itself null) = accidental/fragile existence. Not realization corresponding to idea. Good restricted in content: several kinds, subject to destruction (external contingency/evil, collision/conflict in good itself). Objective world presupposed: subjectivity/finitude of good, distinct world runs own course. Realization exposed to obstacles, might be impossible. Good = ought, exists in and for itself, but being (abstract immediacy) = non-being over against it. Idea of fulfilled good = absolute postulate (absolute encumbered with determinateness of subjectivity). Two worlds in opposition: subjectivity (transparent thought) vs objectivity (externally manifold actuality, darkness). Unresolved contradiction: absolute purpose vs restriction of reality. Idea enters shape of self-consciousness. Practical idea lacks: moment of real consciousness, actuality attained determination of external being. Lacks moment of theoretical idea. Theoretical idea: concept = universality, cognition = apprehension, objectivity = given. Practical idea: actuality confronting = insuperable restriction, in and for itself nullity. Will stands in way: separates from cognition, external actuality does not receive form of true existence. Idea of good finds completion only in idea of true.',
      [
        'realization',
        'finitude',
        'ought',
        'postulate',
        'contradiction',
        'self-consciousness',
        'theoretical idea',
        'will',
        'completion',
      ],
      { section: 'The Idea of the Good', order: 2 }
    ),

    createTopicMapEntry(
      'idea-good-3-absolute-idea',
      'Transition to Absolute Idea; Sublation of Presupposition',
      [196, 363],
      'Transition through itself. Syllogism of action: first premise = immediate reference of good purpose to actuality (appropriates), second premise = directs as external means against external actuality. Good = objective for subjective concept. Abstract being confronting good in second premise = already sublated by practical idea itself. First premise = immediate objectivity of concept, purpose communicated without resistance, simple connection of identity. Second premise already present in first in itself, but immediacy not sufficient, second premise postulated. Realization = mediation essentially necessary for immediate connection and consummation. First premise = first negation/otherness of concept (immersion into externality). Second premise = sublation of otherness, immediate realization becomes actuality of good as concept existing for itself. Concept posited as identical with itself (not with other) = free concept. Claim purpose not realized = relapse to standpoint prior to activity (actual determined as worthless yet presupposed as real). Relapse = progression to bad infinity. Ground: in sublating abstract reality, sublating itself forgotten, reality already presupposed as worthless/nothing objective. Repetition of presupposition = subjective attitude reproduced/perpetuated. Finitude appears as abiding truth, actualization always singular (never universal). But state already sublated in realization. What limits objective concept = its own view of itself, vanishes in reflection on what realization is in itself. Concept stands in its own way, must turn against itself (not external actuality). Activity in second premise: produces one-sided being-for-itself, but activity = positing implicit identity of objective concept and immediate actuality. External actuality altered: determination sublated, apparent reality/worthlessness removed, posited as having existence in and for itself. Presupposition sublated: good as merely subjective purpose restricted, necessity of realizing by subjective activity, activity itself. Result: mediation sublates itself, immediacy = presupposition as sublated (not restoration). Idea of concept determined in and for itself posited: no longer just in active subject, equally as immediate actuality. Actuality posited as in cognition: objectivity that truly exists. Singularity of subject vanished with presupposition. Subject = free, universal self-identity, objectivity of concept = given, immediately present. Subject immediately knows itself = concept determined in and for itself. Cognition restored and united with practical idea. Previously discovered reality = realized absolute purpose, no longer object of investigation. Objective world whose inner ground and actual subsistence = concept. This = absolute idea.',
      [
        'transition',
        'syllogism of action',
        'mediation',
        'sublation',
        'free concept',
        'bad infinity',
        'presupposition',
        'absolute idea',
      ],
      { section: 'The Idea of the Good', order: 3 }
    ),
  ],
  {
    sectionDescription: 'The Idea of the Good - Practical idea, the good, self-determination. Realization and finitude, the ought. Transition to absolute idea, sublation of presupposition.',
  }
);

