/**
 * TopicMap for process.txt - The Mechanical Process
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
 * - process-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const MECHANICAL_PROCESS_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/object/mechanism/sources/process.txt',
  'Hegel\'s Science of Logic - The Concept',
  'The Mechanical Process',
  [
    createTopicMapEntry(
      'mech-proc-1-monads',
      'Objects as Self-Enclosed Totalities, Monads',
      [2, 42],
      'If objects regarded only as self-enclosed totalities, cannot act on one another. Same as monads which have no influence on each other. Concept of monad is deficient reflection. Monad is determinate representation of only implicit totality. As certain degree of development, it is determinate. But since self-enclosed totality, also indifferent to determinateness. Not its own determinateness but posited through another object. Immediate in general, supposed to be mirroring. Self-reference is abstract universality, existence open to others. Self-reference that grasps nothing conceptually but only mirroring is passivity towards other. Determinateness is something external. Degree of development has limit in an other. Reciprocal influence projected into predetermined harmony = presupposition. Need to avoid interaction founded on absolute self-subsistence and originariness. Positedness does not correspond to assumed in-itselfness, has ground in an other.',
      [
        'self-enclosed',
        'totalities',
        'monads',
        'deficient reflection',
        'determinateness',
        'external',
        'passivity',
        'presupposition',
        'ground',
      ],
      { section: 'The Mechanical Process', order: 1 }
    ),

    createTopicMapEntry(
      'mech-proc-2-causality-ended',
      'Causal Relation Has Come to an End',
      [44, 94],
      'Relation of substantiality passes over into relation of causality. Existent no longer has determination of substance but that of object. Causal relation has come to an end in the concept. Originariness of one substance vis-à-vis another shown to be reflective shine. Substance\'s action a transition into opposite substance. This relation has no objectivity. One object posited in form of subjective unity, as efficient cause. No longer counts as originary determination but as something mediated. Active object has this determination only by means of another object. Mechanism has posited within it truth of relation of causality. Cause supposed to be existing in and for itself is in fact effect, positedness. In mechanism, originary causality of object is immediately non-originariness. Object is indifferent to this determination attributed to it. That it is a cause is accidental to it. Causality of substances is only product of representation. Precisely this causality as product of representation is what mechanism is. Mechanism: causality as identical determinateness of diversity of substances. Foundering into identity of their self-subsistence, is mere positedness. Objects are indifferent to this unity and maintain themselves in face of it. Their indifferent self-subsistence is mere positedness. For this reason they are capable of mixing and aggregating, becoming one object. Through this indifference both to transition and to self-subsistence, substances are objects.',
      [
        'substantiality',
        'causality',
        'object',
        'reflective shine',
        'mediated',
        'mechanism',
        'positedness',
        'indifference',
        'mixing',
        'aggregating',
      ],
      { section: 'The Mechanical Process', order: 2 }
    ),

    createTopicMapEntry(
      'mech-proc-3-formal-communication',
      'Formal Mechanical Process - Communication',
      [96, 147],
      'Mechanical process is positing of that contained in concept of mechanism. Positing in first place of a contradiction. Interaction of objects is positing of their identical connection. Positing consists in giving to determinateness form of universality. This is communication, occurs without transition into opposite. Spiritual communication takes place in element of universality. Idealized connection, determinateness continues undisturbed, generalizing itself unaltered. In communication between material objects, determinateness widens in equally idealizing manner. Personality is infinitely more intensive hardness than objects possess. Formal totality of object renders it indistinct from another object. Makes interaction unimpeded continuing of determinateness of one into other. In region of spirit, infinitely manifold content capable of communication. By being taken up into intelligence, content receives form of universality, becomes communicable. That which is universal not only by virtue of form, but in and for itself, is objective as such. In region of spirit and body, singularity of external objects/persons is unessential factor. Unable to offer resistance. Laws, morals, rational conceptions are communicable entities. Pervade individuals unconsciously imposing themselves. In region of body: motion, heat, magnetism, electricity. Even when imagined as stuffs/materials, must be termed imponderable agents. Lack aspect of materiality that grounds singularization.',
      [
        'communication',
        'universality',
        'idealized connection',
        'spiritual',
        'material',
        'objective',
        'laws',
        'morals',
        'imponderable agents',
      ],
      { section: 'The Formal Mechanical Process', order: 3 }
    ),

    createTopicMapEntry(
      'mech-proc-4-formal-reaction',
      'Formal Mechanical Process - Reaction',
      [149, 207],
      'In interaction of objects, identical universality posited first. Equally necessary to posit other moment of concept, particularity. Objects demonstrate their self-subsistence. Hold themselves outside each other, produce singularity. This production is reaction in general. Reaction not mere sublation of action and communicated determinateness. What is communicated is universal positively present in particular objects. Particularizes itself only in their diversity. What is communicated remains what it is, only distributed among objects. Determined by their particularity. Activity of causal substance in its action. But active object only becomes a universal. Its action is from start not loss of determinateness but particularization. Object which was whole determinateness present as single, now becomes species of it. Determinateness thereby posited for first time as universal. Two: raising in communication of singular determinateness into universality. And particularization of it in distribution. Reduction of what was only one to a species. Are one and the same. Reaction is equal to action. First: other object takes over entire universal, now active against first. Reaction is same as action, reciprocal repulsion of impulse. Second: what is communicated is objective. Remains substantial determination of object on presupposition of their diversity. Universal specifies itself in them. Each object does not simply give back whole action but possesses specific share. Third: reaction is wholly negative action. Each object, because of elasticity of self-subsistence. Repels within it positedness of other, retains self-reference. Specific particularity returns to singularity. Object asserts externality as against communicated universality. Action passes over into rest. Proves to be only superficial, transient alteration within self-enclosed indifferent totality.',
      [
        'reaction',
        'particularity',
        'self-subsistence',
        'singularity',
        'distribution',
        'species',
        'reciprocal repulsion',
        'elasticity',
        'externality',
        'rest',
      ],
      { section: 'The Formal Mechanical Process', order: 4 }
    ),

    createTopicMapEntry(
      'mech-proc-5-formal-product',
      'Formal Mechanical Process - Product',
      [209, 250],
      'Return constitutes product of mechanical process. Object presupposed as singular, then as particular as against another particular. Finally as indifferent towards particularity, as universal. Product is totality of concept previously presupposed but now posited. Conclusion in which communicated universal united with singularity through particularity. In rest, mediation posited at same time as sublated. What is posited: product is indifferent to this determining of it. Received determinateness is external in it. Product is same as object that first enters process. But at same time that object first determined through this movement. Mechanical object is, as such, object only as product. What it is, is only by virtue of mediation of other in it. As product it is what it was supposed to be in and for itself. Composite, mixture, certain arrangement of parts. Such that determinateness is not self-determination but something posited. Result of mechanical process not already there ahead of process itself. End is not in beginning, as in case of purpose. Product is in object determinateness externally posited in it. Product according to concept same as what object already is at beginning. But at beginning external determinateness not yet there as posited. Result is something quite other than first existence of object. Something utterly accidental for it.',
      [
        'product',
        'totality',
        'conclusion',
        'rest',
        'mediation',
        'composite',
        'mixture',
        'arrangement',
        'posited',
        'accidental',
      ],
      { section: 'The Formal Mechanical Process', order: 5 }
    ),

    createTopicMapEntry(
      'mech-proc-6-real-communication',
      'Real Mechanical Process - Communication',
      [252, 325],
      'Mechanical process passes over into rest. Determinateness object obtains through process only external. Rest also external, although determinateness opposed to activity. Two each indifferent to object. Rest can be viewed as brought about by external cause. Indifferent to object to be active. Since determinateness is posited, concept of object gone back to itself through mediation. Object contains determinateness as one reflected into itself. In mechanical process objects and process now have more closely determined relation. Not merely diverse, but determinedly differentiated as against one another. Result of formal process: determinationless rest. Through immanently reflected determinateness. Distribution among several objects mechanically relating to one another. Opposition which is in object as such. Object that lacks all determination, showing no elasticity, no self-subsistence. Has self-subsistence impenetrable to other objects. Objects have more determined opposition: self-subsistent singularity and non-self-subsistent universality. Precise difference may be had merely quantitatively. Difference in body of diverse magnitudes of mass, or intensity, or various ways. In general difference cannot be fixed at abstract level. Also as objects, both positively self-subsistent. First moment of real process is communication. Weaker can be seized and invaded by stronger only in so far as it accepts stronger. Constitutes one sphere with it. In material realm weaker secured against disproportionately strong. Weak organic receptivity not as vulnerable to strong stimuli as to weak. Wholly feeble spirit safer facing strong than one who stands closer to strong. Single effective defense against reason is not to get involved with it at all. Object with no standing unable to make contact with self-subsistent. No communication can take place between them. Latter unable to offer resistance, cannot specify communicated universal for itself. If not in same sphere, mutual connection would be infinite judgment. No process possible between them.',
      [
        'rest',
        'external',
        'reflected determinateness',
        'determined relation',
        'opposition',
        'singularity',
        'universality',
        'communication',
        'sphere',
        'resistance',
        'infinite judgment',
      ],
      { section: 'The Real Mechanical Process', order: 6 }
    ),

    createTopicMapEntry(
      'mech-proc-7-resistance-power-fate',
      'Real Mechanical Process - Resistance and Power',
      [326, 409],
      'Resistance is precise moment of overpowering of one object by other. Initial moment in distribution of communicated universal. In positing of self-referring negativity, singularity to be established. Resistance overpowered when determinateness not commensurate to communicated universal. Object has accepted and supposed to be singularized in latter. Object\'s relative lack of self-subsistence manifested. Singularity lacks capacity for what is communicated to it. Shattered by it, unable to constitute itself as subject in universal. Cannot make latter its predicate. Violence against object is something alien only according to second aspect. Power becomes violence when power, objective universality. Is identical with nature of object. Yet determinateness or negativity not object\'s own immanent negative reflection. According to which object is singular. In so far as negativity not reflected back into itself in power. Latter not object\'s own self-reference. Negativity as against power only abstract negativity. Manifestation is extinction. Power as objective universality and as violence against object is fate. Concept falls within mechanism in so far as fate called blind. Objective universality not recognized by subject in own specific sphere. Fate of living thing is genus. Genus manifests itself through fleetingness of living individuals. Do not possess it as genus in actual singularity. Merely animate natures, as mere objects, like other things at lower levels. Do not have fate. What befalls them is contingency. In concept as objects they are self-external. Alien power of fate simply and solely their own immediate nature. Externality and contingency itself. Only self-consciousness has fate in strict sense. Because free, in singularity of "I" absolutely exists in and for itself. Can oppose itself to objective universality, alienate itself from it. By separation, excites against itself mechanical relation of fate. For latter to have violent power over it, must have given itself determinateness. Over against essential universality, must have committed deed. Self-consciousness made itself into particular. This existence, like abstract universality, side open to communication of alienated essence. From this side drawn into process. People without deeds is without blame. Wrapped up in objective, ethical universality, dissolved into it. Without individuality that moves unmoved. Gives itself determinateness on outside and abstract universality separated from objective universality. In this individuality subject also divested of essence. Becomes object, enters into relation of externality towards nature, mechanism.',
      [
        'resistance',
        'overpowering',
        'singularity',
        'violence',
        'power',
        'fate',
        'blind',
        'genus',
        'self-consciousness',
        'deed',
        'particular',
        'ethical universality',
        'mechanism',
      ],
      { section: 'The Real Mechanical Process', order: 7 }
    ),

    createTopicMapEntry(
      'mech-proc-8-product-center-law',
      'Product of Mechanical Process - Center and Law',
      [411, 456],
      'Product of formal mechanism is object in general. Indifferent totality in which determinateness is as posited. Object entered process as determinate thing. In extinction of process, result: rest, original formalism of object. Negativity of determinateness-for-itself. Sublation of determinateness, positive reflection into itself. Determinateness withdrawn into itself, posited totality of concept. True singularity of object. Object determined at first in indeterminate universality. Then as particular, now determined as objective singular. Reflective semblance of singularity sublated. Only self-subsistence opposing itself to substantial universality. Resulting immanent reflection, objective oneness of objects. Now oneness which is individual self-subsistence: the center. Reflection of negativity is universality. Not fate standing over against determinateness. But rational fate, immanently determined. Universality that particularizes itself from within. Difference that remains at rest and fixed. In unstable particularity of objects and their process. It is the law. This result is truth, and consequently also foundation, of mechanical process.',
      [
        'product',
        'totality',
        'rest',
        'singularity',
        'center',
        'law',
        'rational fate',
        'immanently determined',
        'truth',
        'foundation',
      ],
      { section: 'The Product of Mechanical Process', order: 8 }
    ),
  ]
);

