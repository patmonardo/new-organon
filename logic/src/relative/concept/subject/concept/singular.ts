/**
 * Logical Operations: The Singular Concept
 *
 * The singular concept is the return of the concept into itself.
 * It is the negative unity, the subject, and the principle of individuality.
 *
 * Dialectical Movement:
 * - Posited through particularity (determinate universality)
 * - Reflection into itself (self-mediation)
 * - Total concept (universality + particularity)
 * - Principle of individuality
 * - Posited as judgment
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE SINGULAR CONCEPT
// ============================================================================

export const singOp1PositedThroughParticularity: LogicalOperation = {
  id: 'sing-op-1-posited-through-particularity',
  chunkId: 'sing-1-posited-through-particularity',
  label: 'Singularity Posited Through Particularity; Determinate Universality',
  clauses: [
    'singularity.posited = throughParticularity',
    'singularity = determinateUniversality',
    'singularity = selfReferringDeterminateness',
    'singularity = determinateDeterminate',
  ],
  predicates: [
    { name: 'determinateUniversality', args: ['singularity'] },
    { name: 'selfReferringDeterminateness', args: ['singularity'] },
    { name: 'determinateDeterminate', args: ['singularity'] },
  ],
  relations: [
    { predicate: 'posited', from: 'singularity', to: 'throughParticularity' },
    { predicate: 'is', from: 'singularity', to: 'determinateUniversality' },
    { predicate: 'is', from: 'singularity', to: 'selfReferringDeterminateness' },
  ],
  candidateSummary: 'Singularity already posited through particularity. = determinate universality. = self-referring determinateness. = the determinate determinate.',
  provenance: {
    sourceChunk: 'sing-1-posited-through-particularity',
    sourceOp: 'sing-op-1-posited-through-particularity',
  },
};

export const singOp2ReflectionSelfMediation: LogicalOperation = {
  id: 'sing-op-2-reflection-self-mediation',
  chunkId: 'sing-2-reflection-self-mediation',
  label: 'Reflection of Concept into Itself; Self-Mediation; Abstraction\'s False Start',
  clauses: [
    'singularity.appears = asReflectionIntoItself',
    'singularity = conceptSelfMediation',
    'singularity.restores = itselfAsSelfEqual',
    'negativeInUniversal = doublyReflectiveShine',
    'turningBack = twofold',
    'abstraction = falseStart',
    'abstraction = voidOfContent',
    'singularity = trueWay',
    'singularity = depth',
    'singularity.grasps = itself',
  ],
  predicates: [
    { name: 'reflectionIntoItself', args: ['singularity.appears'] },
    { name: 'conceptSelfMediation', args: ['singularity'] },
    { name: 'doublyReflectiveShine', args: ['negativeInUniversal'] },
    { name: 'falseStart', args: ['abstraction'] },
    { name: 'trueWay', args: ['singularity'] },
  ],
  relations: [
    { predicate: 'appears', from: 'singularity', to: 'asReflectionIntoItself' },
    { predicate: 'is', from: 'singularity', to: 'conceptSelfMediation' },
    { predicate: 'restores', from: 'singularity', to: 'itselfAsSelfEqual' },
    { predicate: 'grasps', from: 'singularity', to: 'itself' },
  ],
  candidateSummary: 'Singularity appears as reflection of concept out of determinateness into itself. = concept\'s self-mediation. Restores itself as self-equal in absolute negativity. Negative in universal = doubly reflective shine. Turning back twofold: abstraction (false start, surface void of content) vs singularity (true way, depth where concept grasps itself, posited as concept).',
  provenance: {
    sourceChunk: 'sing-2-reflection-self-mediation',
    sourceOp: 'sing-op-2-reflection-self-mediation',
  },
};

export const singOp3TotalConcept: LogicalOperation = {
  id: 'sing-op-3-total-concept',
  chunkId: 'sing-3-total-concept',
  label: 'Universality and Particularity as Total Concept',
  clauses: [
    'universalityParticularity = momentsOfBecoming',
    'universalityParticularity = totalConcept',
    'inSingularity.doNotPassOver = intoOther',
    'positedInSingularity = whatTheyAreInThemselves',
  ],
  predicates: [
    { name: 'momentsOfBecoming', args: ['universalityParticularity'] },
    { name: 'totalConcept', args: ['universalityParticularity'] },
  ],
  relations: [
    { predicate: 'is', from: 'universalityParticularity', to: 'momentsOfBecoming' },
    { predicate: 'is', from: 'universalityParticularity', to: 'totalConcept' },
    { predicate: 'doNotPassOver', from: 'inSingularity', to: 'intoOther' },
  ],
  candidateSummary: 'Universality and particularity appeared as moments of becoming of singularity. But the two are in themselves the total concept. In singularity they do not pass over into other. What is posited in singularity = what they are in and for themselves.',
  provenance: {
    sourceChunk: 'sing-3-total-concept',
    sourceOp: 'sing-op-3-total-concept',
  },
};

export const singOp4AbstractUniversalVoid: LogicalOperation = {
  id: 'sing-op-4-abstract-universal-void',
  chunkId: 'sing-4-abstract-universal-void',
  label: 'Abstract Universal Void of Concept; Singularity as Principle of Individuality',
  clauses: [
    'universal.is = forItself',
    'abstractUniversal.sublating = externalAct',
    'negativity.remains = outside',
    'universal.voidOf = concept',
    'lifeSpiritGod = beyondAbstraction',
    'singularity = principleOfIndividuality',
    'singularity = principleOfPersonality',
    'abstraction = lifeless',
    'abstraction = voidOfSpirit',
  ],
  predicates: [
    { name: 'forItself', args: ['universal'] },
    { name: 'externalAct', args: ['abstractUniversal.sublating'] },
    { name: 'voidOf', args: ['universal', 'concept'] },
    { name: 'principleOfIndividuality', args: ['singularity'] },
    { name: 'lifeless', args: ['abstraction'] },
  ],
  relations: [
    { predicate: 'is', from: 'universal', to: 'forItself' },
    { predicate: 'remains', from: 'negativity', to: 'outside' },
    { predicate: 'is', from: 'singularity', to: 'principleOfIndividuality' },
  ],
  candidateSummary: 'Universal is for itself because absolute mediation in itself. Abstract universal = sublating is external act. Negativity remains outside as mere condition. Universal does not have singularity in itself, remains void of concept. Life, spirit, God, pure concept = beyond grasp of abstraction. Singularity = principle of individuality and personality. Abstraction = lifeless universalities, void of spirit, color, content.',
  provenance: {
    sourceChunk: 'sing-4-abstract-universal-void',
    sourceOp: 'sing-op-4-abstract-universal-void',
  },
};

export const singOp5UnityIndissoluble: LogicalOperation = {
  id: 'sing-op-5-unity-indissoluble',
  chunkId: 'sing-5-unity-indissoluble',
  label: 'Unity Indissoluble; Products of Abstraction are Singulars',
  clauses: [
    'unityOfConcept = indissoluble',
    'productsOfAbstraction = singulars',
    'determinateUniversality = singularity',
    'abstraction = partitioning',
    'abstraction = isolating',
    'singularityOfAbstraction = contentForm',
    'singularityOfConcept = absoluteForm',
    'singularityOfConcept = totality',
    'productOfAbstraction = concrete',
  ],
  predicates: [
    { name: 'indissoluble', args: ['unityOfConcept'] },
    { name: 'singulars', args: ['productsOfAbstraction'] },
    { name: 'singularity', args: ['determinateUniversality'] },
    { name: 'partitioning', args: ['abstraction'] },
    { name: 'absoluteForm', args: ['singularityOfConcept'] },
    { name: 'concrete', args: ['productOfAbstraction'] },
  ],
  relations: [
    { predicate: 'is', from: 'unityOfConcept', to: 'indissoluble' },
    { predicate: 'is', from: 'productsOfAbstraction', to: 'singulars' },
    { predicate: 'is', from: 'determinateUniversality', to: 'singularity' },
    { predicate: 'is', from: 'productOfAbstraction', to: 'concrete' },
  ],
  candidateSummary: 'Unity of concept so indissoluble that products of abstraction (supposed to drop singularity) are themselves singulars. Abstraction grasps universal as determinate universality = precisely singularity. Abstraction = partitioning, isolating. Difference: singularity of abstraction\'s products (content/form) vs singularity of concept (absolute form, totality). Product of abstraction = concrete (opposite of supposed).',
  provenance: {
    sourceChunk: 'sing-5-unity-indissoluble',
    sourceOp: 'sing-op-5-unity-indissoluble',
  },
};

export const singOp6ParticularSingularDeterminate: LogicalOperation = {
  id: 'sing-op-6-particular-singular-determinate',
  chunkId: 'sing-6-particular-singular-determinate',
  label: 'Particular and Singular as Determinate Universal',
  clauses: [
    'particular = determinateUniversal',
    'particular = singular',
    'singular = particular',
    'concept.has = threeDeterminations',
    'singularity = turningBackToItself',
    'turningBack = indifferentMoment',
  ],
  predicates: [
    { name: 'determinateUniversal', args: ['particular'] },
    { name: 'singular', args: ['particular'] },
    { name: 'particular', args: ['singular'] },
    { name: 'turningBackToItself', args: ['singularity'] },
    { name: 'indifferentMoment', args: ['turningBack'] },
  ],
  relations: [
    { predicate: 'is', from: 'particular', to: 'determinateUniversal' },
    { predicate: 'is', from: 'particular', to: 'singular' },
    { predicate: 'is', from: 'singular', to: 'particular' },
    { predicate: 'has', from: 'concept', to: 'threeDeterminations' },
  ],
  candidateSummary: 'Particular (because determinate universal) = also singular. Conversely: singular = equally particular. Concept has three particular determinations (universal, particular, singular). Singularity = turning of concept as negative back to itself. Turning back from abstraction can be placed as indifferent moment alongside others.',
  provenance: {
    sourceChunk: 'sing-6-particular-singular-determinate',
    sourceOp: 'sing-op-6-particular-singular-determinate',
  },
};

export const singOp7ParticularityTotalitySyllogism: LogicalOperation = {
  id: 'sing-op-7-particularity-totality-syllogism',
  chunkId: 'sing-7-particularity-totality-syllogism',
  label: 'Particularity as Totality; Middle Term of Formal Syllogism',
  clauses: [
    'particularity = totality',
    'particularity.embraces = allDeterminations',
    'particularity = concretion',
    'particularity = singularityItself',
    'particularity = determinateUniversality',
    'particularity = immediateUnity',
    'particularity = middleTerm',
    'middleTerm = ofFormalSyllogism',
  ],
  predicates: [
    { name: 'totality', args: ['particularity'] },
    { name: 'concretion', args: ['particularity'] },
    { name: 'singularityItself', args: ['particularity'] },
    { name: 'middleTerm', args: ['particularity'] },
  ],
  relations: [
    { predicate: 'is', from: 'particularity', to: 'totality' },
    { predicate: 'embraces', from: 'particularity', to: 'allDeterminations' },
    { predicate: 'is', from: 'particularity', to: 'middleTerm' },
    { predicate: 'of', from: 'middleTerm', to: 'formalSyllogism' },
  ],
  candidateSummary: 'If singularity listed as one particular determination: particularity = totality embracing them all. As totality = concretion of determinations = singularity itself. Also concrete as determinate universality. = immediate unity in which none of moments posited as distinct. In this form = middle term of formal syllogism.',
  provenance: {
    sourceChunk: 'sing-7-particularity-totality-syllogism',
    sourceOp: 'sing-op-7-particularity-totality-syllogism',
  },
};

export const singOp8DissolutionInseparability: LogicalOperation = {
  id: 'sing-op-8-dissolution-inseparability',
  chunkId: 'sing-8-dissolution-inseparability',
  label: 'Dissolution of Determinations; Inseparability Posited; Each is Totality',
  clauses: [
    'determination.dissolved = immediately',
    'determination.lost = inOther',
    'representationalThinking.holds = apart',
    'inSingularity.posited = inseparability',
    'singularity = negationOfNegation',
    'singularity.contains = oppositionAndUnity',
    'singularity = negativityOfDeterminations',
    'eachDetermination = totality',
    'determination = wholeConcept',
  ],
  predicates: [
    { name: 'dissolved', args: ['determination'] },
    { name: 'inseparability', args: ['inSingularity.posited'] },
    { name: 'negationOfNegation', args: ['singularity'] },
    { name: 'totality', args: ['eachDetermination'] },
    { name: 'wholeConcept', args: ['determination'] },
  ],
  relations: [
    { predicate: 'lost', from: 'determination', to: 'inOther' },
    { predicate: 'posited', from: 'inseparability', to: 'inSingularity' },
    { predicate: 'contains', from: 'singularity', to: 'oppositionAndUnity' },
    { predicate: 'is', from: 'eachDetermination', to: 'totality' },
  ],
  candidateSummary: 'Each determination immediately dissolved itself, lost itself in its other. Only representational thinking holds them rigidly apart (relies on quantity - inappropriate). In singularity: inseparability of determinations posited. As negation of negation, singularity contains opposition and unity. Singularity = negativity of determinations. Each distinct determination = the totality. Turning back = determination is to be in its determinateness the whole concept.',
  provenance: {
    sourceChunk: 'sing-8-dissolution-inseparability',
    sourceOp: 'sing-op-8-dissolution-inseparability',
  },
};

export const singOp9ImmediateLossActuality: LogicalOperation = {
  id: 'sing-op-9-immediate-loss-actuality',
  chunkId: 'sing-9-immediate-loss-actuality',
  label: 'Singularity as Immediate Loss; Concept Steps into Actuality',
  clauses: [
    'singularity = immediateLoss',
    'concept.becomes = externalToItself',
    'concept.stepsInto = actuality',
    'abstraction = soulOfSingularity',
    'abstraction = immanentInUniversalParticular',
    'universalParticular.concreted = throughAbstraction',
    'singularity = determinateDeterminateness',
    'singularity = differentiation',
    'determiningOfParticular.occurs = byVirtueOfSingularity',
  ],
  predicates: [
    { name: 'immediateLoss', args: ['singularity'] },
    { name: 'externalToItself', args: ['concept.becomes'] },
    { name: 'soulOfSingularity', args: ['abstraction'] },
    { name: 'determinateDeterminateness', args: ['singularity'] },
    { name: 'differentiation', args: ['singularity'] },
  ],
  relations: [
    { predicate: 'is', from: 'singularity', to: 'immediateLoss' },
    { predicate: 'stepsInto', from: 'concept', to: 'actuality' },
    { predicate: 'is', from: 'abstraction', to: 'soulOfSingularity' },
    { predicate: 'occurs', from: 'determiningOfParticular', to: 'byVirtueOfSingularity' },
  ],
  candidateSummary: 'Singularity = not only turning back but immediate loss of concept. Through singularity (internal to itself), concept becomes external to itself and steps into actuality. Abstraction = soul of singularity, immanent in universal and particular. These concreted through it, become content, singular. As negativity, singularity = determinate determinateness, differentiation. Determining of particular occurs only by virtue of singularity.',
  provenance: {
    sourceChunk: 'sing-9-immediate-loss-actuality',
    sourceOp: 'sing-op-9-immediate-loss-actuality',
  },
};

export const singOp10QualitativeOneThis: LogicalOperation = {
  id: 'sing-op-10-qualitative-one-this',
  chunkId: 'sing-10-qualitative-one-this',
  label: 'Singular as Qualitative One; This; Exclusive',
  clauses: [
    'singular = selfReferringNegativity',
    'singular = immediateIdentity',
    'singular.exists = forItself',
    'singular = qualitativeOne',
    'singular = this',
    'first = repulsionOfSelf',
    'second = negativeReferenceToOthers',
    'negativeReference = exclusive',
    'universality.referredToSingulars = commonality',
    'lowestConception = externalRelation',
  ],
  predicates: [
    { name: 'selfReferringNegativity', args: ['singular'] },
    { name: 'immediateIdentity', args: ['singular'] },
    { name: 'qualitativeOne', args: ['singular'] },
    { name: 'this', args: ['singular'] },
    { name: 'exclusive', args: ['negativeReference'] },
    { name: 'commonality', args: ['universality.referredToSingulars'] },
  ],
  relations: [
    { predicate: 'is', from: 'singular', to: 'selfReferringNegativity' },
    { predicate: 'exists', from: 'singular', to: 'forItself' },
    { predicate: 'is', from: 'singular', to: 'qualitativeOne' },
    { predicate: 'is', from: 'singular', to: 'this' },
  ],
  candidateSummary: 'Singular = self-referring negativity = immediate identity of negative with itself. Exists for itself. = abstraction determining concept as immediate. Singular = one which is qualitative, or a this. First: repulsion of itself from itself. Second: negative reference to others (exclusive). Universality referred to singulars = only commonality. Lowest conception = external relation as mere commonality.',
  provenance: {
    sourceChunk: 'sing-10-qualitative-one-this',
    sourceOp: 'sing-op-10-qualitative-one-this',
  },
};

export const singOp11ThisPositedImmediacy: LogicalOperation = {
  id: 'sing-op-11-this-posited-immediacy',
  chunkId: 'sing-11-this-posited-immediacy',
  label: 'This as Posited Immediacy; Reflective Mediation; Positive Connection',
  clauses: [
    'singular.inReflectiveSphere = this',
    'singular.noExcludingReference = qualitativeBeingForItself',
    'this = reflectedIntoItself',
    'this = withoutRepulsion',
    'repulsion.inReflection = oneWithAbstraction',
    'abstraction = reflectiveMediation',
    'this = positedImmediacy',
    'singular.repelling = separation',
    'inSeparation = positiveConnection',
  ],
  predicates: [
    { name: 'this', args: ['singular.inReflectiveSphere'] },
    { name: 'reflectedIntoItself', args: ['this'] },
    { name: 'withoutRepulsion', args: ['this'] },
    { name: 'reflectiveMediation', args: ['abstraction'] },
    { name: 'positedImmediacy', args: ['this'] },
    { name: 'positiveConnection', args: ['inSeparation'] },
  ],
  relations: [
    { predicate: 'is', from: 'singular.inReflectiveSphere', to: 'this' },
    { predicate: 'is', from: 'this', to: 'reflectedIntoItself' },
    { predicate: 'is', from: 'this', to: 'positedImmediacy' },
    { predicate: 'repelling', from: 'singular', to: 'separation' },
  ],
  candidateSummary: 'Singular in reflective sphere = as a this. Does not have excluding reference (qualitative being-for-itself). This = one reflected into itself, without repulsion. Repulsion in reflection one with abstraction (reflective mediation). Makes it posited immediacy pointed at by someone external. Singular also a this but does not have mediation outside it. Itself repelling separation, posited abstraction. Yet, precisely in separation, positive connection.',
  provenance: {
    sourceChunk: 'sing-11-this-posited-immediacy',
    sourceOp: 'sing-op-11-this-posited-immediacy',
  },
};

export const singOp12SelfSubsistentDifferences: LogicalOperation = {
  id: 'sing-op-12-self-subsistent-differences',
  chunkId: 'sing-12-self-subsistent-differences',
  label: 'Singular as Self-Subsistent Differences; Essential Relation',
  clauses: [
    'actOfAbstraction = immanentReflectionOfDifference',
    'positingOfDifferences = selfSubsisting',
    'differences.exist = immediately',
    'separating = reflectionInGeneral',
    'differences.standIn = essentialRelation',
    'plurality.belongsTo = being',
    'singularity.positsItself = inDifferenceOfConcept',
    'singularity.excludes = universal',
    'singularity.refersTo = universal',
  ],
  predicates: [
    { name: 'immanentReflectionOfDifference', args: ['actOfAbstraction'] },
    { name: 'selfSubsisting', args: ['positingOfDifferences'] },
    { name: 'reflectionInGeneral', args: ['separating'] },
    { name: 'essentialRelation', args: ['differences.standIn'] },
  ],
  relations: [
    { predicate: 'is', from: 'actOfAbstraction', to: 'immanentReflectionOfDifference' },
    { predicate: 'exist', from: 'differences', to: 'immediately' },
    { predicate: 'standIn', from: 'differences', to: 'essentialRelation' },
    { predicate: 'positsItself', from: 'singularity', to: 'inDifferenceOfConcept' },
    { predicate: 'excludes', from: 'singularity', to: 'universal' },
    { predicate: 'refersTo', from: 'singularity', to: 'universal' },
  ],
  candidateSummary: 'Act of abstraction by singular = immanent reflection of difference. = first positing of differences as self-subsisting, reflected into themselves. They exist immediately. But separating = reflection in general (reflective shining). Differences stand in essential relation. Not singulars just existing next to each other (plurality belongs to being). Singularity posits itself in difference of concept. Excludes universal but refers to it essentially.',
  provenance: {
    sourceChunk: 'sing-12-self-subsistent-differences',
    sourceOp: 'sing-op-12-self-subsistent-differences',
  },
};

export const singOp13PositedAsJudgment: LogicalOperation = {
  id: 'sing-op-13-posited-as-judgment',
  chunkId: 'sing-13-posited-as-judgment',
  label: 'Concept Posited as Judgment; Absolute Originative Partition',
  clauses: [
    'concept.hasLost = itself',
    'concept.noLonger = positedUnity',
    'determinations.subsist = inAndForThemselves',
    'concept.returns = intoItself',
    'determinate.hasBecome = totality',
    'turningBack = absoluteOriginativePartition',
    'concept.posited = asJudgment',
  ],
  predicates: [
    { name: 'positedUnity', args: ['concept.noLonger'] },
    { name: 'inAndForThemselves', args: ['determinations.subsist'] },
    { name: 'totality', args: ['determinate.hasBecome'] },
    { name: 'absoluteOriginativePartition', args: ['turningBack'] },
    { name: 'asJudgment', args: ['concept.posited'] },
  ],
  relations: [
    { predicate: 'hasLost', from: 'concept', to: 'itself' },
    { predicate: 'returns', from: 'concept', to: 'intoItself' },
    { predicate: 'hasBecome', from: 'determinate', to: 'totality' },
    { predicate: 'is', from: 'turningBack', to: 'absoluteOriginativePartition' },
    { predicate: 'posited', from: 'concept', to: 'asJudgment' },
  ],
  candidateSummary: 'Concept as connection of self-subsistent determinations has lost itself. Concept no longer posited unity; these no longer moments but subsist in and for themselves. As singularity, concept returns in determinateness into itself. Determinate has itself become totality. Concept\'s turning back = absolute, originative partition of itself. As singularity it is posited as judgment.',
  provenance: {
    sourceChunk: 'sing-13-posited-as-judgment',
    sourceOp: 'sing-op-13-posited-as-judgment',
  },
};

export const singularConceptOperations: LogicalOperation[] = [
  singOp1PositedThroughParticularity,
  singOp2ReflectionSelfMediation,
  singOp3TotalConcept,
  singOp4AbstractUniversalVoid,
  singOp5UnityIndissoluble,
  singOp6ParticularSingularDeterminate,
  singOp7ParticularityTotalitySyllogism,
  singOp8DissolutionInseparability,
  singOp9ImmediateLossActuality,
  singOp10QualitativeOneThis,
  singOp11ThisPositedImmediacy,
  singOp12SelfSubsistentDifferences,
  singOp13PositedAsJudgment,
];
