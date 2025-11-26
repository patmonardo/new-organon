/**
 * TopicMap for mechanism.txt - Absolute Mechanism
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
 * - mechanism-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const ABSOLUTE_MECHANISM_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/object/mechanism/sources/mechanism.txt',
  'Hegel\'s Science of Logic - The Concept',
  'Absolute Mechanism',
  [
    createTopicMapEntry(
      'mech-abs-1-center-singularity',
      'Center - Empty Manifoldness Gathered into Objective Singularity',
      [4, 64],
      'Empty manifoldness of object gathered first into objective singularity. Simple self-determining middle point. Object retains as immediate totality indifference to determinateness. Latter present as unessential or outside-one-another of many objects. Essential determinateness constitutes real middle term between interacting objects. Unites them in and for themselves, is their objective universality. Universality exhibited first in relation of communication, as present only through positing. As objective universality, pervading immanent essence of objects. In material world, central body is genus or individualized universality. Single objects and their mechanical process. Unessential single bodies relate by impact and pressure. This relation does not hold between central body and objects. Their externality no longer constitutes fundamental determination. Identity with central body is rest, being at center. Unity is concept existing in and for itself. Remains only ought, since externality still posited, does not conform. Striving towards center is absolute universality. Not posited through communication. Constitutes true rest, concrete, not posited from outside. Process of instability must find way back. Empty abstraction to assume body would move straight to infinity. If did not lose movement because of external resistance. Friction is phenomenon of centrality. Brings body back to itself. That against which body rubs has power of resistance only because united with center. In things spiritual, center and union assume higher forms. Unity of concept and reality must constitute fundamental determination.',
      [
        'center',
        'objective singularity',
        'middle point',
        'objective universality',
        'central body',
        'genus',
        'rest',
        'striving',
        'friction',
        'centrality',
      ],
      { section: 'Absolute Mechanism - The Center', order: 1 }
    ),

    createTopicMapEntry(
      'mech-abs-2-center-syllogistic',
      'Center - Central Body as Individual, Syllogistic Structure',
      [66, 165],
      'Central body ceased to be mere object. Determinateness no longer unessential. Has being-for-itself of objective totality. Can be regarded as individual. Determinateness essentially different from order, arrangement, external combination. Immanent form, self-determining principle. Objects inhere, bound together in true One. Central individual at first only middle term with no true extremes. As negative unity dirempts into extremes. Previously non-self-subsistent objects become individuals by retreat of concept. Self-identity of central body, still striving, burdened by externality. Communicated to it. Objects positioned outside original center. Themselves centers for non-self-subsistent objects. Second centers and non-self-subsistent objects brought into unity by absolute middle term. Relative individual centers constitute middle term of second syllogism. Subsumed under higher extreme: objective universality and power of absolute center. Subsumes under it non-self-subsistent objects. Their superficiality and formal singularization it supports. Non-self-subsistent objects are middle term of third syllogism, formal syllogism. Link between absolute and relative central individuality. Formal objects have identical gravity of immediate central body. In which they inhere as in their subject and extreme of singularity. Through externality which they constitute, immediate central body subsumed under absolute central body. They are formal middle term of particularity. Absolute individual is objectively universal middle term. Brings into unity and holds firm inwardness of relative individual and its externality. Example: government, individual citizens, needs/external life. Three terms, each is middle term of other two. Government is absolute center. Extreme of singulars united with external existence. Singulars are middle term that incites universal individual into external concrete existence. Transposes ethical essence into extreme of actuality. Third syllogism is formal, syllogism of reflective shine. Singular citizens tied by needs and external existence to universal absolute individuality. Syllogism that, as merely subjective, passes over into others. Has truth in them. Totality whose moments are completed relations of concept. Syllogisms in which each of three different objects runs through determination of middle term and extreme. Constitutes free mechanism. Different objects have objective universality for fundamental determination. Pervasive gravity persists self-identical in particularization. Connections of pressure, impact, attraction, aggregations belong to relation of externality. Basis of third of three syllogisms. Order, merely external determinateness, passed over into immanent and objective determination. This is the law.',
      [
        'individual',
        'syllogistic structure',
        'middle term',
        'extremes',
        'absolute center',
        'relative centers',
        'formal objects',
        'free mechanism',
        'law',
      ],
      { section: 'Absolute Mechanism - The Center', order: 2 }
    ),

    createTopicMapEntry(
      'mech-abs-3-law',
      'Law - Idealized Reality vs External Reality',
      [167, 218],
      'In law, more specific difference of idealized reality vs external reality comes into view. Object as immediate totality does not yet possess externality differentiated from concept. Latter not posited for itself. Through mediation of process, object withdrawn into itself. Opposition of simple centrality vs externality determined as externality. Posited as not existing in-and-for-itself. Moment of identity or idealization is ought. Unity of concept determined in-and-for-itself, self-determining. To which external reality does not correspond. Mere striving towards it. Individuality is concrete principle of negative unity. As such is itself totality. Unity that dirempts into specific differences. While abiding within self-equal universality. Central point expanded inside pure ideality by difference. Reality corresponding to concept is idealized reality. Distinct from reality that is only striving. Difference, earlier plurality of objects, now in essential nature. Taken up into pure universality. Real ideality is soul of objective totality. Identity of system determined in and for itself. Objective being-in-and-for-itself manifests as negative unity of center. Divides into subjective individuality and external objectivity. Maintains former in latter, determines it in idealized difference. Self-determining unity reducing external objectivity to ideality. Principle of self-movement. Determinateness of animating principle, difference of concept itself, is the law. Dead mechanism was mechanical process of objects appearing as self-subsisting. But in truth non-self-subsistent, center outside them. Process passing over into rest exhibits contingency and indeterminate difference. Or formal uniformity. Uniformity is rule, but not law. Only free mechanism has law. Determination proper to pure individuality, concept existing for itself. As difference, law is inexhaustible source of self-igniting fire. In ideality of difference refers only to itself. Free necessity.',
      [
        'law',
        'idealized reality',
        'external reality',
        'ought',
        'individuality',
        'negative unity',
        'ideality',
        'self-movement',
        'free mechanism',
        'free necessity',
      ],
      { section: 'Absolute Mechanism - The Law', order: 3 }
    ),

    createTopicMapEntry(
      'mech-abs-4-transition-chemism',
      'Transition of Mechanism to Chemism',
      [236, 278],
      'Soul still immersed in body. Determined but inner concept of objective totality is free necessity. In sense that law has not yet stepped opposite its object. Concrete centrality as universality immediately diffused in objectivity. Ideality does not have objects themselves for determinate difference. These are self-subsistent individuals of totality. Or non-individual, external objects. Law is immanent in them, constitutes their nature and power. But difference shut up in ideality. Objects not themselves differentiated in idealized non-indifference of law. Object possesses essential self-subsistence solely in idealized centrality and laws. Has no power to resist judgment of concept. Maintain itself in abstract, indeterminate self-subsistence and remoteness. Because idealized difference immanent in it. Existence is determinateness posited by concept. Lack of self-subsistence no longer just striving towards middle point. With respect to which, because connection only that of striving. Still has appearance of self-subsistent external object. Rather striving towards object determinedly opposed to it. Center has itself for that reason fallen apart. Negativity passed over into objectified opposition. Centrality is now reciprocally negative and tense connection of objectivities. Free mechanism determines itself to chemism.',
      [
        'transition',
        'chemism',
        'free necessity',
        'law',
        'objectified opposition',
        'reciprocally negative',
        'tense connection',
      ],
      { section: 'Absolute Mechanism - Transition', order: 4 }
    ),
  ]
);

