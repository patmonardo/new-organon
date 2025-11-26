/**
 * TopicMap for life-process.txt - The Life-Process
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
 * - life-process-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const LIFE_PROCESS_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/idea/life/sources/life-process.txt',
  'Hegel\'s Science of Logic - The Idea',
  'The Life-Process',
  [
    createTopicMapEntry(
      'life-process-1-need-pain',
      'Opposition to Objective World; Need and Pain as Absolute Contradiction',
      [3, 66],
      'Individual sets itself in opposition to objective world. Subject = purpose unto itself, self-subsistent. External world = negative, without self-subsistence. Self-feeling = certainty of nullity of otherness. Impulse = need to sublate otherness. Process begins with need = self-determination positing self as negated, refers to other, preserves self. Impulse to posit world as own, sublate world, objectify self. Self-determination = objective externality + self-identical = absolute contradiction. Negative moment realizes = objective particularity, concept splits = absolute inequality, living being = rupture = pain. Pain = prerogative of living natures, actuality of infinite power, contradiction as actual existence.',
      [
        'opposition',
        'objective world',
        'subject',
        'purpose',
        'need',
        'impulse',
        'contradiction',
        'pain',
        'rupture',
        'absolute contradiction',
      ],
      { section: 'The Life-Process', order: 1 }
    ),

    createTopicMapEntry(
      'life-process-2-assimilation',
      'Assimilation; Appropriation and Reproduction; Self-Identical',
      [68, 157],
      'Internal rupture taken up = feeling. From pain = need/impulse = transition, negation becomes identity = negation of negation. Impulse = subjective certainty, relates to external world as appearance/unessential. Objectivity = external aptitude, mechanical determinability, impotence. Object excites (not causes). Subject exercises violence, mechanism/chemism interrupted, externality transformed into interiority. External purposiveness sublated, concept posits itself as essence. Seizing object = internal process, appropriates, makes means, confers subjectivity. Assimilation = reproduction, individual feeds on itself. Life = truth of processes, power over them, permeates as universality. Transformation = turning back into itself, production becomes reproduction, self-identical.',
      [
        'internal rupture',
        'feeling',
        'pain',
        'need',
        'impulse',
        'assimilation',
        'appropriation',
        'reproduction',
        'externality',
        'interiority',
        'self-identical',
      ],
      { section: 'The Life-Process', order: 2 }
    ),

    createTopicMapEntry(
      'life-process-3-genus',
      'Universal Life as Genus; Actual Singularity',
      [159, 179],
      'Immediate idea = immediate identity of concept and reality (not for itself). Through objective process, living being gives itself feeling of self, posits itself as self-identical in otherness = negative unity of negative. Rejoining objectivity = actual singularity, sublated particularity, raised to universality. Particularity = disruption (individual life + external objectivity as species). External life-process = real universal life = genus.',
      [
        'immediate idea',
        'objective process',
        'feeling of self',
        'self-identical',
        'otherness',
        'negative unity',
        'actual singularity',
        'universality',
        'genus',
      ],
      { section: 'The Life-Process', order: 3 }
    ),
  ],
  {
    sectionDescription: 'The Life-Process - Opposition to objective world, need and pain as absolute contradiction. Assimilation, appropriation, reproduction. Universal life as genus.',
  }
);

