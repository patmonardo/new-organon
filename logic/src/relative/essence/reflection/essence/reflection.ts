/**
 * Logical Operations: Reflection
 *
 * Reflection is the movement of essence, the movement from nothing to nothing.
 * It covers positing reflection, external reflection, and determining reflection.
 *
 * Dialectical Movement:
 * - Positing reflection: movement from nothing to nothing
 * - External reflection: presupposing being
 * - Determining reflection: unity of positing and external
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// REFLECTION
// ============================================================================

export const refOp1ShineAndReflection: LogicalOperation = {
  id: 'ref-op-1-shine-and-reflection',
  chunkId: 'ref-1',
  label: 'Introduction — shine and reflection',
  clauses: [
    'shine = reflectionAsImmediate',
    'reflection = internalizedShine',
    'reflection = alienatedFromImmediacy',
  ],
  predicates: [
    { name: 'reflectionAsImmediate', args: ['shine'] },
    { name: 'internalizedShine', args: ['reflection'] },
    { name: 'alienatedFromImmediacy', args: ['reflection'] },
  ],
  relations: [
    { predicate: 'isSameAs', from: 'shine', to: 'reflection' },
  ],
  candidateSummary: 'Shine is the same as reflection, but reflection as immediate. Internalized shine alienated from immediacy. German: \'Reflexion\' from alien language.',
  provenance: {
    sourceChunk: 'ref-1',
    sourceOp: 'ref-op-1-shine-and-reflection',
  },
};

export const refOp2MovementNothingToNothing: LogicalOperation = {
  id: 'ref-op-2-movement-nothing-to-nothing',
  chunkId: 'ref-2',
  label: 'Essence as reflection — movement from nothing to nothing',
  clauses: [
    'essence = reflection',
    'reflection = movementOfBecoming',
    'movement = fromNothingToNothing',
    'movement = backToItself',
    'being = movementOfNothingness',
  ],
  predicates: [
    { name: 'reflection', args: ['essence'] },
    { name: 'movementOfBecoming', args: ['reflection'] },
    { name: 'fromNothingToNothing', args: ['movement'] },
    { name: 'backToItself', args: ['movement'] },
    { name: 'movementOfNothingness', args: ['being'] },
  ],
  relations: [
    { predicate: 'is', from: 'essence', to: 'reflection' },
  ],
  candidateSummary: 'Essence is reflection, movement of becoming that remains within itself. Distinguished determined as negative in itself, as shine. Reflective movement: other as negation in itself, self-referring. Negation as negation, negation with negation. Immediacy is the movement itself, not substrate. Movement from nothing to nothing, back to itself. Being only as movement of nothingness to nothingness. Pure absolute reflection further determines itself.',
  provenance: {
    sourceChunk: 'ref-2',
    sourceOp: 'ref-op-2-movement-nothing-to-nothing',
  },
};

export const refOp3ThreeTypes: LogicalOperation = {
  id: 'ref-op-3-three-types',
  chunkId: 'ref-3',
  label: 'Three types of reflection',
  clauses: [
    'first = positingReflection',
    'second = externalReflection',
    'third = determiningReflection',
  ],
  predicates: [
    { name: 'positingReflection', args: ['first'] },
    { name: 'externalReflection', args: ['second'] },
    { name: 'determiningReflection', args: ['third'] },
  ],
  relations: [
    { predicate: 'has', from: 'reflection', to: 'threeTypes' },
  ],
  candidateSummary: 'First: positing reflection. Second: external reflection (takes presupposed immediate as starting point). Third: determining reflection (sublates presupposition, presupposes at same time).',
  provenance: {
    sourceChunk: 'ref-3',
    sourceOp: 'ref-op-3-three-types',
  },
};

export const refOp4PositingReflectionIntro: LogicalOperation = {
  id: 'ref-op-4-positing-reflection-intro',
  chunkId: 'ref-4',
  label: 'Positing reflection — introduction',
  clauses: [
    'shine = nothingness',
    'nothingness = equalityWithItself',
    'absoluteReflection = conversionOfNegative',
    'selfReferringNegativity = negatingOfItself',
    'unity = beingAndNotBeing',
  ],
  predicates: [
    { name: 'nothingness', args: ['shine'] },
    { name: 'equalityWithItself', args: ['nothingness'] },
    { name: 'conversionOfNegative', args: ['absoluteReflection'] },
    { name: 'negatingOfItself', args: ['selfReferringNegativity'] },
    { name: 'beingAndNotBeing', args: ['unity'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'nothingness' },
  ],
  candidateSummary: 'Shine is nothingness or lack of essence. Nothingness has being as its own equality with itself. Conversion of negative with itself: absolute reflection. Self-referring negativity is negating of itself. Sublated negativity as it is negativity. Being itself and not being itself, two in one unity.',
  provenance: {
    sourceChunk: 'ref-4',
    sourceOp: 'ref-op-4-positing-reflection-intro',
  },
};

export const refOp5PositingMovement: LogicalOperation = {
  id: 'ref-op-5-positing-movement',
  chunkId: 'ref-5',
  label: 'Positing reflection — movement from nothing to nothing',
  clauses: [
    'reflection = movementFromNothingToNothing',
    'negation = coincidesWithItself',
    'selfCoinciding = simpleEquality',
    'reflection = sublatingOfTransition',
    'being = toBeWhatItIsNot',
  ],
  predicates: [
    { name: 'movementFromNothingToNothing', args: ['reflection'] },
    { name: 'coincidesWithItself', args: ['negation'] },
    { name: 'simpleEquality', args: ['selfCoinciding'] },
    { name: 'sublatingOfTransition', args: ['reflection'] },
    { name: 'toBeWhatItIsNot', args: ['being'] },
  ],
  relations: [
    { predicate: 'is', from: 'reflection', to: 'movementFromNothingToNothing' },
  ],
  candidateSummary: 'Reflection is movement from nothing to nothing. Negation coinciding with itself. Self-coinciding is simple equality with itself, immediacy. Not transition into equality as being other than it. Reflection is transition as sublating of transition. Immediacy is self-equality of negative, self-negating equality. Its being is to be what it is not.',
  provenance: {
    sourceChunk: 'ref-5',
    sourceOp: 'ref-op-5-positing-movement',
  },
};

export const refOp6PositingTurningBack: LogicalOperation = {
  id: 'ref-op-6-positing-turning-back',
  chunkId: 'ref-6',
  label: 'Positing reflection — turning back, positedness',
  clauses: [
    'selfReference = turningBackIntoItself',
    'immediacy = selfSublating',
    'positedness = immediacyAsDeterminateness',
    'immediacy = determinatenessOfShine',
    'turningBack = reflection',
  ],
  predicates: [
    { name: 'turningBackIntoItself', args: ['selfReference'] },
    { name: 'selfSublating', args: ['immediacy'] },
    { name: 'immediacyAsDeterminateness', args: ['positedness'] },
    { name: 'determinatenessOfShine', args: ['immediacy'] },
    { name: 'reflection', args: ['turningBack'] },
  ],
  relations: [
    { predicate: 'is', from: 'positedness', to: 'immediacyAsDeterminateness' },
  ],
  candidateSummary: 'Self-reference of negative is turning back into itself. Immediacy as sublating of negative, self-sublating immediacy. This is positedness, immediacy purely as determinateness. Immediacy constitutes determinateness of shine. Cannot begin with this immediacy; it is the turning back itself. Reflection only in turning is that which starts out or returns.',
  provenance: {
    sourceChunk: 'ref-6',
    sourceOp: 'ref-op-6-positing-turning-back',
  },
};

export const refOp7PositingAndPresupposing: LogicalOperation = {
  id: 'ref-op-7-positing-and-presupposing',
  chunkId: 'ref-7',
  label: 'Positing reflection — positing and presupposing',
  clauses: [
    'positing = immediacyAsTurningBack',
    'immediacy = sublatedNegation',
    'presupposing = negationOfNegative',
    'presupposing = determinesTurningBack',
    'presupposing = selfRepulsion',
  ],
  predicates: [
    { name: 'immediacyAsTurningBack', args: ['positing'] },
    { name: 'sublatedNegation', args: ['immediacy'] },
    { name: 'negationOfNegative', args: ['presupposing'] },
    { name: 'determinesTurningBack', args: ['presupposing'] },
    { name: 'selfRepulsion', args: ['presupposing'] },
  ],
  relations: [
    { predicate: 'is', from: 'presupposing', to: 'negationOfNegative' },
  ],
  candidateSummary: 'Positing: immediacy as turning back, no other beforehand. Only as turning back, negative of itself. Immediacy is sublated negation and sublated return. Sublating of negative is sublating of immediacy. Equally negation of negative as negative: presupposing. Presupposing: determines turning back as negative of itself. Self-repulsion, presupposing of that from which reflection turns back.',
  provenance: {
    sourceChunk: 'ref-7',
    sourceOp: 'ref-op-7-positing-and-presupposing',
  },
};

export const refOp8SelfRepulsion: LogicalOperation = {
  id: 'ref-op-8-self-repulsion',
  chunkId: 'ref-8',
  label: 'Positing reflection — self-repulsion, absolute counter-repelling',
  clauses: [
    'essence = equalityWithItself',
    'essence = presupposesItself',
    'transcending = arrivingAtImmediate',
    'movement = selfMovement',
    'reflection = negativeOfItself',
  ],
  predicates: [
    { name: 'equalityWithItself', args: ['essence'] },
    { name: 'presupposesItself', args: ['essence'] },
    { name: 'arrivingAtImmediate', args: ['transcending'] },
    { name: 'selfMovement', args: ['movement'] },
    { name: 'negativeOfItself', args: ['reflection'] },
  ],
  relations: [
    { predicate: 'is', from: 'reflection', to: 'negativeOfItself' },
  ],
  candidateSummary: 'Only by sublating equality with itself is essence equality with itself. Essence presupposes itself, sublating of presupposing is essence itself. Presupposition is only in turning back itself. Transcending immediate occurs only through transcending. Transcending immediate is arriving at immediate. Forward movement turns immediately around into itself: self-movement. Positing reflection is presupposing reflection, and vice versa. Reflection is itself and its non-being, negative of itself.',
  provenance: {
    sourceChunk: 'ref-8',
    sourceOp: 'ref-op-8-self-repulsion',
  },
};

export const refOp9TransitionToExternal: LogicalOperation = {
  id: 'ref-op-9-transition-to-external',
  chunkId: 'ref-9',
  label: 'Transition to external reflection',
  clauses: [
    'immediacy = positedness',
    'immediacy = determinedAsNegative',
    'reflection = hasPresupposition',
    'reflection = takesStartFromImmediate',
  ],
  predicates: [
    { name: 'positedness', args: ['immediacy'] },
    { name: 'determinedAsNegative', args: ['immediacy'] },
    { name: 'hasPresupposition', args: ['reflection'] },
    { name: 'takesStartFromImmediate', args: ['reflection'] },
  ],
  relations: [
    { predicate: 'is', from: 'immediacy', to: 'positedness' },
  ],
  candidateSummary: 'Immediacy presupposed is simply and solely positedness. Determined as negative, in opposition to other. Reflection has presupposition, takes start from immediate as other. External reflection.',
  provenance: {
    sourceChunk: 'ref-9',
    sourceOp: 'ref-op-9-transition-to-external',
  },
};

export const refOp10ExternalReflectionDoubled: LogicalOperation = {
  id: 'ref-op-10-external-reflection-doubled',
  chunkId: 'ref-10',
  label: 'External reflection — doubled determination',
  clauses: [
    'absoluteReflection = positsShine',
    'externalReflection = presupposesItselfAsSublated',
    'determination = doubled',
    'presupposed = reflectionIntoItself',
    'reflection = negativelyReferring',
  ],
  predicates: [
    { name: 'positsShine', args: ['absoluteReflection'] },
    { name: 'presupposesItselfAsSublated', args: ['externalReflection'] },
    { name: 'doubled', args: ['determination'] },
    { name: 'reflectionIntoItself', args: ['presupposed'] },
    { name: 'negativelyReferring', args: ['reflection'] },
  ],
  relations: [
    { predicate: 'is', from: 'determination', to: 'doubled' },
  ],
  candidateSummary: 'Absolute reflection posits only shine, positedness, for presupposition. Presupposing reflection is immediately positing reflection. External reflection presupposes itself as sublated, negative of itself. Doubled: presupposed (reflection into itself as immediate) and reflection negatively referring to itself.',
  provenance: {
    sourceChunk: 'ref-10',
    sourceOp: 'ref-op-10-external-reflection-doubled',
  },
};

export const refOp11ExternalReflectionPresupposes: LogicalOperation = {
  id: 'ref-op-11-external-reflection-presupposes',
  chunkId: 'ref-11',
  label: 'External reflection — presupposes being',
  clauses: [
    'reflection = presupposesBeing',
    'immediacy = refersToItself',
    'reflection = refersToPresuppositionAsNegative',
    'externalReflection = infinite',
    'finite = first',
  ],
  predicates: [
    { name: 'presupposesBeing', args: ['reflection'] },
    { name: 'refersToItself', args: ['immediacy'] },
    { name: 'refersToPresuppositionAsNegative', args: ['reflection'] },
    { name: 'infinite', args: ['externalReflection'] },
    { name: 'first', args: ['finite'] },
  ],
  relations: [
    { predicate: 'presupposes', from: 'reflection', to: 'being' },
  ],
  candidateSummary: 'Presupposes being, immediacy refers to itself. Determinateness only as moment. Reflection refers to presupposition as negative, sublated as negative. In positing, immediately sublates positing, has immediate presupposition. Finds presupposition before it, from which it starts. Presupposition as negative/positedness not its concern. In sphere of being, external reflection was infinite. Finite stands as first, infinite as reflection into itself.',
  provenance: {
    sourceChunk: 'ref-11',
    sourceOp: 'ref-op-11-external-reflection-presupposes',
  },
};

export const refOp12ExternalReflectionSyllogism: LogicalOperation = {
  id: 'ref-op-12-external-reflection-syllogism',
  chunkId: 'ref-12',
  label: 'External reflection — syllogism structure',
  clauses: [
    'externalReflection = syllogism',
    'extremes = immediateAndReflection',
    'middleTerm = reference',
    'immediate = oneExtreme',
    'determinateness = otherExtreme',
  ],
  predicates: [
    { name: 'syllogism', args: ['externalReflection'] },
    { name: 'immediateAndReflection', args: ['extremes'] },
    { name: 'reference', args: ['middleTerm'] },
    { name: 'oneExtreme', args: ['immediate'] },
    { name: 'otherExtreme', args: ['determinateness'] },
  ],
  relations: [
    { predicate: 'is', from: 'externalReflection', to: 'syllogism' },
  ],
  candidateSummary: 'External reflection is syllogism. Two extremes: immediate and reflection into itself. Middle term: reference connecting two, determinate immediate. Immediate falls to one extreme, determinateness/negation to other.',
  provenance: {
    sourceChunk: 'ref-12',
    sourceOp: 'ref-op-12-external-reflection-syllogism',
  },
};

export const refOp13ExternalReflectionPositing: LogicalOperation = {
  id: 'ref-op-13-external-reflection-positing',
  chunkId: 'ref-13',
  label: 'External reflection — positing and sublating',
  clauses: [
    'positing = becomesNegative',
    'negating = positing',
    'externality = sublated',
    'externalReflection = immanentReflection',
    'externalReflection = determiningReflection',
  ],
  predicates: [
    { name: 'becomesNegative', args: ['positing'] },
    { name: 'positing', args: ['negating'] },
    { name: 'sublated', args: ['externality'] },
    { name: 'immanentReflection', args: ['externalReflection'] },
    { name: 'determiningReflection', args: ['externalReflection'] },
  ],
  relations: [
    { predicate: 'is', from: 'externalReflection', to: 'determiningReflection' },
  ],
  candidateSummary: 'Positing of immediate, becomes negative/determined. Immediately sublates positing, presupposes immediate. Negating is negating of its negating, equally positing. Immediate posited as same as reflection. Externality sublated, coinciding with immediate is immediacy of essence. External reflection is immanent reflection of immediacy itself. Result of positing reflection is essence existing in and for itself. External reflection is determining reflection.',
  provenance: {
    sourceChunk: 'ref-13',
    sourceOp: 'ref-op-13-external-reflection-positing',
  },
};

export const refOp14DeterminingReflectionUnity: LogicalOperation = {
  id: 'ref-op-14-determining-reflection-unity',
  chunkId: 'ref-14',
  label: 'Determining reflection — unity of positing and external',
  clauses: [
    'determiningReflection = unityOfPositingAndExternal',
    'externalReflection = beginsFromImmediate',
    'positing = beginsFromNothing',
    'determination = posited',
    'connection = absolute',
  ],
  predicates: [
    { name: 'unityOfPositingAndExternal', args: ['determiningReflection'] },
    { name: 'beginsFromImmediate', args: ['externalReflection'] },
    { name: 'beginsFromNothing', args: ['positing'] },
    { name: 'posited', args: ['determination'] },
    { name: 'absolute', args: ['connection'] },
  ],
  relations: [
    { predicate: 'is', from: 'determiningReflection', to: 'unityOfPositingAndExternal' },
  ],
  candidateSummary: 'Determining reflection is unity of positing and external reflection. External reflection begins from immediate being, positing from nothing. External reflection posits another (essence) in place of sublated being. Positing has no presupposition, not complete as determining reflection. Determination posited is only posited, immediate as self-negating. Connection with turning back into itself is absolute. Only in reflection-into-itself but not this reflection itself.',
  provenance: {
    sourceChunk: 'ref-14',
    sourceOp: 'ref-op-14-determining-reflection-unity',
  },
};

export const refOp15PositednessVsExistence: LogicalOperation = {
  id: 'ref-op-15-positedness-vs-existence',
  chunkId: 'ref-15',
  label: 'Determining reflection — positedness vs. existence',
  clauses: [
    'positedness = correspondsToExistence',
    'positedness = existence',
    'ground = essence',
    'existence = onlyPositedness',
    'positedness = superior',
  ],
  predicates: [
    { name: 'correspondsToExistence', args: ['positedness'] },
    { name: 'existence', args: ['positedness'] },
    { name: 'essence', args: ['ground'] },
    { name: 'onlyPositedness', args: ['existence'] },
    { name: 'superior', args: ['positedness'] },
  ],
  relations: [
    { predicate: 'is', from: 'positedness', to: 'existence' },
  ],
  candidateSummary: 'Posited is other, but self-equality of reflection retained. Posited only as sublated, reference to turning back. In sphere of being, existence had negation in it. In sphere of essence, positedness corresponds to existence. Positedness is existence, but ground is essence as pure negativity. Determinateness not as existent but immediately as sublated. Existence is only positedness: principle of essence of existence. Positedness conjoins existence with essence. Twofold meaning: in opposition to existence or to essence. Positedness is superior, existence is negative, refers to turning back.',
  provenance: {
    sourceChunk: 'ref-15',
    sourceOp: 'ref-op-15-positedness-vs-existence',
  },
};

export const refOp16DeterminationOfReflection: LogicalOperation = {
  id: 'ref-op-16-determination-of-reflection',
  chunkId: 'ref-16',
  label: 'Determining reflection — determination of reflection',
  clauses: [
    'positedness = determinatenessAsNegation',
    'determinationOfReflection = distinctFromBeing',
    'positedness = referenceToImmanentReflectedBeing',
    'determination = persistsBySelfEquality',
    'determination = essential',
  ],
  predicates: [
    { name: 'determinatenessAsNegation', args: ['positedness'] },
    { name: 'distinctFromBeing', args: ['determinationOfReflection'] },
    { name: 'referenceToImmanentReflectedBeing', args: ['positedness'] },
    { name: 'persistsBySelfEquality', args: ['determination'] },
    { name: 'essential', args: ['determination'] },
  ],
  relations: [
    { predicate: 'is', from: 'determination', to: 'essential' },
  ],
  candidateSummary: 'Positedness is determinateness as negation in general. Positing united with external reflection: absolute presupposing. Repelling of reflection from itself, positing determinateness as its own. As posited, positedness is negation; as presupposed, reflected into itself. Determination of reflection distinct from determinateness of being. Quality is immediate reference to other; positedness is reference to immanently reflected being. Negation as quality is existent negation; determination of reflection has immanent reflectedness as ground. Determination persists by self-equality, not by being. Quality is transient; determination of reflection is essential. Self-equality of reflection gives subsistence, negative only as sublated/posited.',
  provenance: {
    sourceChunk: 'ref-16',
    sourceOp: 'ref-op-16-determination-of-reflection',
  },
};

export const refOp17FreeEssentialities: LogicalOperation = {
  id: 'ref-op-17-free-essentialities',
  chunkId: 'ref-17',
  label: 'Determining reflection — free essentialities, essential shine',
  clauses: [
    'determinations = freeEssentialities',
    'determinateness = entranced',
    'shine = essentialShine',
    'determiningReflection = exitedFromItself',
    'negation = predominates',
  ],
  predicates: [
    { name: 'freeEssentialities', args: ['determinations'] },
    { name: 'entranced', args: ['determinateness'] },
    { name: 'essentialShine', args: ['shine'] },
    { name: 'exitedFromItself', args: ['determiningReflection'] },
    { name: 'predominates', args: ['negation'] },
  ],
  relations: [
    { predicate: 'appearAs', from: 'determinations', to: 'freeEssentialities' },
  ],
  candidateSummary: 'Determinations of reflection appear as free essentialities. Sublated in void, without reciprocal attraction or repulsion. Determinateness entranced, infinitely fixed by reference to itself. Deflected reflection-into-other into reflection-into-itself. Constitute determinate shine as it is in essence: essential shine. Determining reflection is reflection that has exited from itself. Equality of essence with itself lost in negation, negation predominates.',
  provenance: {
    sourceChunk: 'ref-17',
    sourceOp: 'ref-op-17-free-essentialities',
  },
};

export const refOp18TwoSides: LogicalOperation = {
  id: 'ref-op-18-two-sides',
  chunkId: 'ref-18',
  label: 'Determining reflection — two sides: positedness and immanent reflection',
  clauses: [
    'sides = positednessAndImmanentReflection',
    'positedness = negationAsNegation',
    'reflection = abidesInItself',
    'distinctions = posited',
    'negation = equalityWithItself',
  ],
  predicates: [
    { name: 'positednessAndImmanentReflection', args: ['sides'] },
    { name: 'negationAsNegation', args: ['positedness'] },
    { name: 'abidesInItself', args: ['reflection'] },
    { name: 'posited', args: ['distinctions'] },
    { name: 'equalityWithItself', args: ['negation'] },
  ],
  relations: [
    { predicate: 'has', from: 'determiningReflection', to: 'twoSides' },
  ],
  candidateSummary: 'Two distinct sides: positedness (negation as such) and immanent reflection. According to positedness: negation as negation, unity with itself (implicitly). Immediate which sublates itself within, other of itself. Reflection is determining that abides in itself, essence does not exit. Distinctions solely posited, taken back into essence. From other side: reflected into themselves. Negation as negation is equality with itself, not reflected into non-being.',
  provenance: {
    sourceChunk: 'ref-18',
    sourceOp: 'ref-op-18-two-sides',
  },
};

export const refOp19NatureAsBoth: LogicalOperation = {
  id: 'ref-op-19-nature-as-both',
  chunkId: 'ref-19',
  label: 'Determining reflection — nature as both',
  clauses: [
    'determination = immanentlyReflectedReferenceAndPositedness',
    'positedness = sublatednessOfDetermination',
    'immanentReflectedness = subsisting',
    'determination = referenceToNegation',
    'positedness = infiniteReferenceToItself',
  ],
  predicates: [
    { name: 'immanentlyReflectedReferenceAndPositedness', args: ['determination'] },
    { name: 'sublatednessOfDetermination', args: ['positedness'] },
    { name: 'subsisting', args: ['immanentReflectedness'] },
    { name: 'referenceToNegation', args: ['determination'] },
    { name: 'infiniteReferenceToItself', args: ['positedness'] },
  ],
  relations: [
    { predicate: 'is', from: 'determination', to: 'both' },
  ],
  candidateSummary: 'Determination of reflection is both immanently reflected reference and positedness. As positedness: negation as such, non-being as against essence. As self-reference: reflected within itself. Positedness is sublatedness of determination; immanent reflectedness is subsisting. Positedness at same time immanent reflection: determinateness is reference to otherness. Not quiescent determinateness, not separate from reference. Determination is determinate side and reference to its negation. Quality passes over into another; determination of reflection takes otherness back into itself. Positedness, negation deflected into itself, unity of itself and its other. Essentiality through this unity. Positedness, negation, but as reflection into itself is sublatedness of positedness. Infinite reference to itself.',
  provenance: {
    sourceChunk: 'ref-19',
    sourceOp: 'ref-op-19-nature-as-both',
  },
};

export const reflectionOperations: LogicalOperation[] = [
  refOp1ShineAndReflection,
  refOp2MovementNothingToNothing,
  refOp3ThreeTypes,
  refOp4PositingReflectionIntro,
  refOp5PositingMovement,
  refOp6PositingTurningBack,
  refOp7PositingAndPresupposing,
  refOp8SelfRepulsion,
  refOp9TransitionToExternal,
  refOp10ExternalReflectionDoubled,
  refOp11ExternalReflectionPresupposes,
  refOp12ExternalReflectionSyllogism,
  refOp13ExternalReflectionPositing,
  refOp14DeterminingReflectionUnity,
  refOp15PositednessVsExistence,
  refOp16DeterminationOfReflection,
  refOp17FreeEssentialities,
  refOp18TwoSides,
  refOp19NatureAsBoth,
];
