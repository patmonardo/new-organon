/**
 * Logical Operations: The Relation of Force and Its Expression
 *
 * The Relation of Force and Its Expression is the negative unity into which the
 * contradiction of whole and parts resolved itself, mediating between whole-parts
 * and outer-inner.
 *
 * Dialectical Movement:
 * - Force: negative unity
 * - Solicitation: reciprocal stimulation
 * - Infinity: externality identical with inwardness
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE RELATION OF FORCE AND ITS EXPRESSION
// ============================================================================

export const fexOp1TruthOfWholeAndParts: LogicalOperation = {
  id: 'fex-op-1-truth-of-whole-and-parts',
  chunkId: 'fex-1',
  label: 'Force — truth of whole and parts',
  clauses: [
    'force = negativeUnity',
    'force = truthOfWholeAndParts',
    'force = higherImmanentTurningBack',
    'unity = ceasesToBeExternal',
  ],
  predicates: [
    { name: 'negativeUnity', args: ['force'] },
    { name: 'truthOfWholeAndParts', args: ['force'] },
    { name: 'higherImmanentTurningBack', args: ['force'] },
    { name: 'ceasesToBeExternal', args: ['unity'] },
  ],
  relations: [
    { predicate: 'is', from: 'force', to: 'truthOfWholeAndParts' },
  ],
  candidateSummary: 'Force is negative unity into which contradiction of whole and parts resolved itself. Truth of that first relation. Whole and parts is thoughtless relation understanding first comes up with, dead mechanical aggregate. Unity external to manifoldness. Relation of force is higher immanent turning back in which unity of whole ceases to be external and indifferent to manifoldness.',
  provenance: {
    sourceChunk: 'fex-1',
    sourceOp: 'fex-op-1-truth-of-whole-and-parts',
  },
};

export const fexOp2Moments: LogicalOperation = {
  id: 'fex-op-2-moments',
  chunkId: 'fex-2',
  label: 'Essential relation — moments',
  clauses: [
    'immediateAndReflected = positedAsMoments',
    'precedingRelation = selfSubsistingSides',
  ],
  predicates: [
    { name: 'positedAsMoments', args: ['immediateAndReflected'] },
    { name: 'selfSubsistingSides', args: ['precedingRelation'] },
  ],
  relations: [
    { predicate: 'positedAs', from: 'immediateAndReflected', to: 'moments' },
  ],
  candidateSummary: 'In essential relation as now determined, immediate and reflected self-subsistence now posited in manifoldness as sublated or as moments. Whereas in preceding relation were self-subsisting sides or extremes.',
  provenance: {
    sourceChunk: 'fex-2',
    sourceOp: 'fex-op-2-moments',
  },
};

export const fexOp3ThreeMoments: LogicalOperation = {
  id: 'fex-op-3-three-moments',
  chunkId: 'fex-3',
  label: 'Three moments — transition, translation, mediation',
  clauses: [
    'force = passesIntoExpression',
    'movement = notTransitionButTranslation',
    'force = mediatedThroughOther',
  ],
  predicates: [
    { name: 'passesIntoExpression', args: ['force'] },
    { name: 'notTransitionButTranslation', args: ['movement'] },
    { name: 'mediatedThroughOther', args: ['force'] },
  ],
  relations: [
    { predicate: 'passesInto', from: 'force', to: 'expression' },
  ],
  candidateSummary: 'First, reflected unity and immediate existence pass over into other. Force passes over into expression. What expressed is disappearing something that returns into force as ground. Second, transition not only becoming and disappearing but negative reference to itself. Movement of force not transition as translation, remains what it is. Third, reflected self-referring unity itself sublated and moment. Mediated through other, has presupposition by which solicited, other from which begins.',
  provenance: {
    sourceChunk: 'fex-3',
    sourceOp: 'fex-op-3-three-moments',
  },
};

export const fexOp4Conditionedness: LogicalOperation = {
  id: 'fex-op-4-conditionedness',
  chunkId: 'fex-4',
  label: 'The conditionedness of force — immediate subsistence',
  clauses: [
    'force = containsExistingImmediacy',
    'thing = indifferentToForce',
    'force = externallyConnected',
  ],
  predicates: [
    { name: 'containsExistingImmediacy', args: ['force'] },
    { name: 'indifferentToForce', args: ['thing'] },
    { name: 'externallyConnected', args: ['force'] },
  ],
  relations: [
    { predicate: 'presupposes', from: 'force', to: 'thing' },
  ],
  candidateSummary: 'Force contains moment of existing immediacy. Determined over against immediacy as negative unity. Unity, in determination of immediate being, is existing something. Something appears as first, force appears as positedness. Thing is immediate indifferent to it. No ground in thing for having force. Force presupposes thing essentially. If asked how thing has force, appears externally connected and impressed by alien power.',
  provenance: {
    sourceChunk: 'fex-4',
    sourceOp: 'fex-op-4-conditionedness',
  },
};

export const fexOp5ForceAsMatter: LogicalOperation = {
  id: 'fex-op-5-force-as-matter',
  chunkId: 'fex-5',
  label: 'Force as matter — quiescent',
  clauses: [
    'force = quiescentDeterminateness',
    'force = designatedAsMatter',
    'force = positingOfExternality',
  ],
  predicates: [
    { name: 'quiescentDeterminateness', args: ['force'] },
    { name: 'designatedAsMatter', args: ['force'] },
    { name: 'positingOfExternality', args: ['force'] },
  ],
  relations: [
    { predicate: 'is', from: 'force', to: 'matter' },
  ],
  candidateSummary: 'As immediate subsistence, force is quiescent determinateness of thing, immediately external. Force also designated as matter. Instead of magnetic force, magnetic matter assumed. Instead of force of attraction, fine ether assumed. Matters into which inert negative unity dissolved itself. But force contains immediate concrete existence as moment, transient and self-sublating. Not as concretely existing thing. Negative unity reflected into itself. Force itself is positing of externality. No longer merely determinate matter, self-subsistence passed over into positedness.',
  provenance: {
    sourceChunk: 'fex-5',
    sourceOp: 'fex-op-5-force-as-matter',
  },
};

export const fexOp6UnityAndActivity: LogicalOperation = {
  id: 'fex-op-6-unity-and-activity',
  chunkId: 'fex-6',
  label: 'Force — unity and activity',
  clauses: [
    'force = unityOfReflectedAndImmediate',
    'force = selfRepellingContradiction',
    'force = active',
    'force = becomingExternalManifoldness',
  ],
  predicates: [
    { name: 'unityOfReflectedAndImmediate', args: ['force'] },
    { name: 'selfRepellingContradiction', args: ['force'] },
    { name: 'active', args: ['force'] },
    { name: 'becomingExternalManifoldness', args: ['force'] },
  ],
  relations: [
    { predicate: 'is', from: 'force', to: 'active' },
  ],
  candidateSummary: 'Force is unity of reflected and immediate subsistence, form-unity and external self-subsistence. Both in one. Contact of sides of which one is in so far as other is not. Self-identical positive reflection and negated reflection. Force is self-repelling contradiction, active. Self-referring negative unity. Reflected immediacy posited as only sublated or moment, passing over into immediate concrete existence. Force posited as becoming concretely existent external manifoldness from out of itself. But force is activity at first only in principle. Essentially refers to identity as immediacy external to it, has as presupposition and condition.',
  provenance: {
    sourceChunk: 'fex-6',
    sourceOp: 'fex-op-6-unity-and-activity',
  },
};

export const fexOp7ForceConditionedThroughForce: LogicalOperation = {
  id: 'fex-op-7-force-conditioned-through-force',
  chunkId: 'fex-7',
  label: 'Force conditioned through force',
  clauses: [
    'selfSubsistentOther = itselfForce',
    'activity = conditionedThroughItself',
    'eachSide = sameAsOther',
  ],
  predicates: [
    { name: 'itselfForce', args: ['selfSubsistentOther'] },
    { name: 'conditionedThroughItself', args: ['activity'] },
    { name: 'sameAsOther', args: ['eachSide'] },
  ],
  relations: [
    { predicate: 'is', from: 'selfSubsistentOther', to: 'force' },
  ],
  candidateSummary: 'Presupposition is not thing standing over against it. In force any indifferent self-subsistence sublated. As condition of force, thing is self-subsistent other. But because not thing, and self-subsistent immediacy attained determination of self-referring negative unity, self-subsistent other is itself force. Activity of force is conditioned through itself as through other to itself, through force. Force is relation in which each side same as other. Forces stand in relation, refer to each other essentially. Unity of relation at first internal and exists only implicitly. Conditionedness of force through another force is doing of force itself in itself.',
  provenance: {
    sourceChunk: 'fex-7',
    sourceOp: 'fex-op-7-force-conditioned-through-force',
  },
};

export const fexOp8Solicitation: LogicalOperation = {
  id: 'fex-op-8-solicitation',
  chunkId: 'fex-8',
  label: 'The solicitation of force — presupposing',
  clauses: [
    'externality = ownActivityOfPresupposing',
    'presupposing = reciprocal',
    'each = positsExternalityAsAnotherForce',
  ],
  predicates: [
    { name: 'ownActivityOfPresupposing', args: ['externality'] },
    { name: 'reciprocal', args: ['presupposing'] },
    { name: 'positsExternalityAsAnotherForce', args: ['each'] },
  ],
  relations: [
    { predicate: 'is', from: 'externality', to: 'ownActivityOfPresupposing' },
  ],
  candidateSummary: 'Force is conditioned because moment of immediate concrete existence it contains is only posited, but because immediate, posited as presupposed in which force negates itself. Externality present to force is own activity of presupposing posited at first as another force. Presupposing is reciprocal. Each of two forces contains unity reflected into itself as sublated, therefore presupposing. Posits itself as external. Moment of externality is own. But since equally unity reflected into itself, posits externality not within itself but as another force.',
  provenance: {
    sourceChunk: 'fex-8',
    sourceOp: 'fex-op-8-solicitation',
  },
};

export const fexOp9Stimulus: LogicalOperation = {
  id: 'fex-op-9-stimulus',
  chunkId: 'fex-9',
  label: 'Solicitation — stimulus',
  clauses: [
    'external = selfSublating',
    'force = reciprocallyStimulus',
    'repelling = ownPositing',
  ],
  predicates: [
    { name: 'selfSublating', args: ['external'] },
    { name: 'reciprocallyStimulus', args: ['force'] },
    { name: 'ownPositing', args: ['repelling'] },
  ],
  relations: [
    { predicate: 'is', from: 'repelling', to: 'ownPositing' },
  ],
  candidateSummary: 'External as such is self-sublating. Activity that reflects itself into itself essentially refers to externality as to other, equally to it as to something null in itself and identical with it. Since presupposing activity equally immanent reflection, sublates external negation, posits it as externality. Force, as conditioning, reciprocally stimulus for other force. Attitude not passive determination. Stimulus only solicits it. Force within it negativity of itself. Repelling of itself from itself is own positing. Act consists in sublating externality of stimulus, reducing it to stimulus and positing it as own repelling, as own expression.',
  provenance: {
    sourceChunk: 'fex-9',
    sourceOp: 'fex-op-9-stimulus',
  },
};

export const fexOp10ForceExpressesItself: LogicalOperation = {
  id: 'fex-op-10-force-expresses-itself',
  chunkId: 'fex-10',
  label: 'Force expresses itself — reciprocal',
  clauses: [
    'concept = identityOfPositingAndPresupposing',
    'soliciting = onlyToExtentSolicited',
    'eachForce = mediatedThroughOther',
  ],
  predicates: [
    { name: 'identityOfPositingAndPresupposing', args: ['concept'] },
    { name: 'onlyToExtentSolicited', args: ['soliciting'] },
    { name: 'mediatedThroughOther', args: ['eachForce'] },
  ],
  relations: [
    { predicate: 'is', from: 'concept', to: 'identityOfPositingAndPresupposing' },
  ],
  candidateSummary: 'Force that expresses itself same as presupposing activity. Concept of force is identity of positing and presupposing reflection, reflected and immediate unity. Each determination moment, in unity, mediated through other. Nothing in two forces determines which soliciting and which solicited. Both form determinations belong to each in equal manner. Identity not external comparison but essential unity. One force solicited, stimulus posited from outside. But force itself presupposing, reflects into itself, sublates externality. That solicited is own doing. Through own determining, other force is other force and soliciting. Soliciting only to extent has externality in it, to extent solicited. Soliciting only to extent solicited to be soliciting. Each receives stimulus from other. Stimulus each delivers as active consists in receiving stimulus from other. Stimulus received solicited by itself. Both nothing immediate but mediated. Each force is determinateness other has over against it, mediated through other, mediating other is own determining positing.',
  provenance: {
    sourceChunk: 'fex-10',
    sourceOp: 'fex-op-10-force-expresses-itself',
  },
};

export const fexOp11TurningBackInfinity: LogicalOperation = {
  id: 'fex-op-11-turning-back-infinity',
  chunkId: 'fex-11',
  label: 'Turning back — infinity',
  clauses: [
    'turningBack = forceIntoItself',
    'expressionAndTurning = one',
    'activity = essentiallyReactive',
  ],
  predicates: [
    { name: 'forceIntoItself', args: ['turningBack'] },
    { name: 'one', args: ['expressionAndTurning'] },
    { name: 'essentiallyReactive', args: ['activity'] },
  ],
  relations: [
    { predicate: 'is', from: 'activity', to: 'essentiallyReactive' },
  ],
  candidateSummary: 'Force happens to incur stimulus through another force. Behaves passively but passes over from passivity into activity. Turning back of force into itself. Force expresses itself. External expression is reaction. Posits externality as own moment, sublates having been solicited through other force. Two are one: expression of force through negative activity directed at itself, imparts determinate being-for-other to itself, and infinite turning in externality back to itself, only refers to itself. Presupposing reflection immediately also reflection that returns into itself. Activity essentially reactive, against itself. Positing of stimulus is itself sublation of it. Conversely, sublation of stimulus is positing of externality.',
  provenance: {
    sourceChunk: 'fex-11',
    sourceOp: 'fex-op-11-turning-back-infinity',
  },
};

export const fexOp12InfinityOfForce: LogicalOperation = {
  id: 'fex-op-12-infinity-of-force',
  chunkId: 'fex-12',
  label: 'The infinity of force — externality identical with inwardness',
  clauses: [
    'referenceToOther = referenceToItself',
    'passivity = consistsInActivity',
    'externality = identicalWithInwardness',
  ],
  predicates: [
    { name: 'referenceToItself', args: ['referenceToOther'] },
    { name: 'consistsInActivity', args: ['passivity'] },
    { name: 'identicalWithInwardness', args: ['externality'] },
  ],
  relations: [
    { predicate: 'is', from: 'externality', to: 'identicalWithInwardness' },
  ],
  candidateSummary: 'Force is finite inasmuch as moments still have form of immediacy. Presupposing and self-referring reflection different. Force still conditioned according to form and content. But activity of force consists in expressing itself, sublating externality and determining it as that in which identical with itself. What force truly expresses: reference to other is reference to itself. Passivity consists in activity. Stimulus by virtue of which solicited to activity is own soliciting. Externality that comes to it nothing immediate but mediated by it. Own essential self-identity not immediate but mediated by virtue of negation. Force expresses: externality is identical with inwardness.',
  provenance: {
    sourceChunk: 'fex-12',
    sourceOp: 'fex-op-12-infinity-of-force',
  },
};

export const forceExpressionOperations: LogicalOperation[] = [
  fexOp1TruthOfWholeAndParts,
  fexOp2Moments,
  fexOp3ThreeMoments,
  fexOp4Conditionedness,
  fexOp5ForceAsMatter,
  fexOp6UnityAndActivity,
  fexOp7ForceConditionedThroughForce,
  fexOp8Solicitation,
  fexOp9Stimulus,
  fexOp10ForceExpressesItself,
  fexOp11TurningBackInfinity,
  fexOp12InfinityOfForce,
];
