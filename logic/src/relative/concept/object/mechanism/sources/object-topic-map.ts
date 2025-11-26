/**
 * TopicMap for object.txt - The Mechanical Object
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
 * - object-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const MECHANICAL_OBJECT_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/object/mechanism/sources/object.txt',
  'Hegel\'s Science of Logic - The Concept',
  'The Mechanical Object',
  [
    createTopicMapEntry(
      'mech-obj-1-syllogism-equilibrium',
      'Object as Syllogism with Equilibrium',
      [2, 10],
      'Object is syllogism whose mediation attained equilibrium → immediate identity. Object is universal in and for itself. Not commonality of properties, but universality pervading particularity. In it is immediate singularity.',
      [
        'object',
        'syllogism',
        'equilibrium',
        'immediate identity',
        'universal',
        'particularity',
        'singularity',
      ],
      { section: 'The Mechanical Object', order: 1 }
    ),

    createTopicMapEntry(
      'mech-obj-2-no-matter-form',
      'Object Does Not Differentiate into Matter/Form',
      [12, 34],
      'Object does not differentiate into matter and form. Abstract differentiation of singularity/universality has no place. Object is informed matter, not thing with properties, not whole/parts, not substance/accidents. Past relations have come to an end in the concept. Object has neither properties nor accidents. Particularity absolutely reflected into totality. Differences are themselves essentially objects, totalities.',
      [
        'matter',
        'form',
        'differentiation',
        'properties',
        'accidents',
        'substance',
        'totality',
        'reflection',
      ],
      { section: 'The Mechanical Object', order: 2 }
    ),

    createTopicMapEntry(
      'mech-obj-3-indeterminate-plurality',
      'Object is Indeterminate, Plurality, Composite/Aggregate',
      [36, 74],
      'Object is indeterminate at first, no determinate opposition within. Mediation collapsed into immediate identity. Object has determinateness of manifold, complete but otherwise indeterminate (relationless). Totally indeterminate difference = several objects. Each contains determinateness reflected into universality, does not reflectively shine outwardly. Object is in itself plurality, composite, aggregate. Not atoms (not totalities). Leibniz\'s monad more of an object - total representation of world shut up in intensive subjectivity. Ground of manifold representations lies outside it.',
      [
        'indeterminate',
        'plurality',
        'composite',
        'aggregate',
        'atoms',
        'monad',
        'manifold',
        'determinateness',
        'universality',
      ],
      { section: 'The Mechanical Object', order: 3 }
    ),

    createTopicMapEntry(
      'mech-obj-4-indifferent-determinations',
      'Object Indifferent to Determinations',
      [75, 93],
      'Object is totality of determinateness, yet indifferent towards determinations as singulars. Determinations indifferent to each other. Not comprehensible from it nor from one another. Object\'s totality is form of overall reflectedness of manifoldness into singularity. Form constituting difference and combining into unity is external, indifferent. Mixture, order, arrangement - combinations indifferent to what they connect.',
      [
        'totality',
        'determinateness',
        'indifference',
        'singularity',
        'external form',
        'mixture',
        'order',
        'arrangement',
      ],
      { section: 'The Mechanical Object', order: 4 }
    ),

    createTopicMapEntry(
      'mech-obj-5-determinateness-outside',
      'Determinateness Outside in Other Objects, Infinite Progression',
      [94, 103],
      'Object has determinateness of totality outside it, in other objects. These again outside them, so forth to infinity. Immanent turning back represented as totality, world, universe. Universality brought to closure through indeterminate singularity.',
      [
        'determinateness',
        'outside',
        'other objects',
        'infinite progression',
        'world',
        'universe',
        'closure',
        'singularity',
      ],
      { section: 'The Mechanical Object', order: 5 }
    ),

    createTopicMapEntry(
      'mech-obj-6-determinism',
      'Determinism and Self-Determination',
      [105, 130],
      'Object is determinate yet indifferent to determinateness. Points for determinateness outside and beyond itself. Constantly to objects for which it is matter of indifference that they do determining. Nowhere is principle of self-determination. Determinism assigns for each determination that of another object. Other object likewise indifferent. Determinism bound to infinite progression, can halt at will anywhere. To explain determination is empty word - no self-determination in other object.',
      [
        'determinism',
        'self-determination',
        'indifference',
        'infinite progression',
        'explanation',
        'empty word',
      ],
      { section: 'The Mechanical Object', order: 6 }
    ),

    createTopicMapEntry(
      'mech-obj-7-contradiction-mechanical-process',
      'Contradiction of Indifference and Identity → Mechanical Process',
      [132, 157],
      'Determinateness of object lies in other, no determinate diversity separating the two. Determinateness merely doubled, once in one object, again in other. Utterly identical, explanation/comprehension is tautology. External back and forth movement. Objects self-subsistent in regard to one another. In identity remain utterly external. Contradiction: perfect indifference of objects to one another + identity of determinateness. Or: objects\' perfect externality in identity of determinateness. This contradiction is negative unity of plurality of objects reciprocally repelling each other in unity. This is the mechanical process.',
      [
        'contradiction',
        'indifference',
        'identity',
        'tautology',
        'externality',
        'negative unity',
        'plurality',
        'mechanical process',
      ],
      { section: 'The Mechanical Object', order: 7 }
    ),
  ]
);

