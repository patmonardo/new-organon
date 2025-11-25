/**
 * Logical Operations: The Particular Concept
 *
 * The particular concept is the universal itself, but determined.
 * It is the moment of determinateness, difference, and distinction.
 *
 * Dialectical Movement:
 * - Determinateness as immanent moment
 * - Particular contains universality (totality)
 * - Particular is universal itself (self-differentiation)
 * - Transition to singularity
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE PARTICULAR CONCEPT
// ============================================================================

export const partOp1DeterminatenessImmanent: LogicalOperation = {
  id: 'part-op-1-determinateness-immanent',
  chunkId: 'part-1-determinateness-immanent',
  label: 'Determinateness as Particularity; Universal\'s Own Immanent Moment',
  clauses: [
    'determinateness.belongsTo = being',
    'determinateness.belongsTo = qualitative',
    'determinatenessOfConcept = particularity',
    'particularity != limit',
    'limit.relatedTo = otherBeyond',
    'particularity = immanentMoment',
    'immanentMoment = ofUniversal',
    'inParticularity.universal = withItself',
    'inParticularity.universal != inOther',
  ],
  predicates: [
    { name: 'particularity', args: ['determinatenessOfConcept'] },
    { name: 'immanentMoment', args: ['particularity', 'ofUniversal'] },
    { name: 'withItself', args: ['inParticularity.universal'] },
  ],
  relations: [
    { predicate: 'belongsTo', from: 'determinateness', to: 'being' },
    { predicate: 'is', from: 'determinatenessOfConcept', to: 'particularity' },
    { predicate: 'is', from: 'particularity', to: 'immanentMoment' },
    { predicate: 'is', from: 'inParticularity.universal', to: 'withItself' },
  ],
  candidateSummary: 'Determinateness as such belongs to being and the qualitative. As determinateness of the concept = particularity. Not a limit (related to other beyond it). Universal\'s own immanent moment. In particularity, universal is not in an other but with itself.',
  provenance: {
    sourceChunk: 'part-1-determinateness-immanent',
    sourceOp: 'part-op-1-determinateness-immanent',
  },
};

export const partOp2ContainsUniversalityTotality: LogicalOperation = {
  id: 'part-op-2-contains-universality-totality',
  chunkId: 'part-2-contains-universality-totality',
  label: 'Particular Contains Universality; Totality and Completeness',
  clauses: [
    'particular.contains = universality',
    'universality = substanceOfParticular',
    'genus = unalteredInSpecies',
    'species.differ = fromEachOther',
    'species.differ != fromUniversal',
    'particular.has = sameUniversality',
    'diversityOfParticulars = universal',
    'diversityOfParticulars = totality',
    'particular.exhibits = universal',
    'universal = sphereToExhaust',
    'totality.appears = asCompleteness',
    'noInnerStandard = forDiversity',
    'particularity = immanentConnection',
    'particularity = essentialPrinciple',
  ],
  predicates: [
    { name: 'substanceOfParticular', args: ['universality'] },
    { name: 'unalteredInSpecies', args: ['genus'] },
    { name: 'totality', args: ['diversityOfParticulars'] },
    { name: 'immanentConnection', args: ['particularity'] },
    { name: 'essentialPrinciple', args: ['particularity'] },
  ],
  relations: [
    { predicate: 'contains', from: 'particular', to: 'universality' },
    { predicate: 'is', from: 'universality', to: 'substanceOfParticular' },
    { predicate: 'differ', from: 'species', to: 'fromEachOther' },
    { predicate: 'exhibits', from: 'particular', to: 'universal' },
    { predicate: 'is', from: 'particularity', to: 'immanentConnection' },
  ],
  candidateSummary: 'Particular contains universality (constitutes its substance). Genus unaltered in its species; species differ only from each other, not from universal. Particular has same universality as other particulars. Diversity of particulars = universal (because of identity with universal) = totality. Particular exhibits universal through determinateness. Universal constitutes sphere that particular must exhaust. Totality appears as completeness. No inner standard/principle for diversity. Particularity = immanent connection (totality intrinsically, essential principle).',
  provenance: {
    sourceChunk: 'part-2-contains-universality-totality',
    sourceOp: 'part-op-2-contains-universality-totality',
  },
};

export const partOp3UniversalItselfDifferentiation: LogicalOperation = {
  id: 'part-op-3-universal-itself-differentiation',
  chunkId: 'part-3-universal-itself-differentiation',
  label: 'Particular is Universal Itself; Self-Differentiation',
  clauses: [
    'particular = universalItself',
    'particular = difference',
    'particular = referenceToOther',
    'referenceToOther = outwardlyReflectingShine',
    'universal.determines = itself',
    'universal.is = particular',
    'determinateness = difference',
    'determinateness = differentiatedFromItself',
    'species = universalItself',
    'species = particular',
    'universal = conceptItself',
    'universal = opposite',
    'universal = totality',
    'universal = principleOfDiversity',
  ],
  predicates: [
    { name: 'universalItself', args: ['particular'] },
    { name: 'outwardlyReflectingShine', args: ['referenceToOther'] },
    { name: 'totality', args: ['universal'] },
    { name: 'principleOfDiversity', args: ['universal'] },
  ],
  relations: [
    { predicate: 'is', from: 'particular', to: 'universalItself' },
    { predicate: 'determines', from: 'universal', to: 'itself' },
    { predicate: 'is', from: 'species', to: 'universalItself' },
    { predicate: 'is', from: 'universal', to: 'totality' },
  ],
  candidateSummary: 'Particular = universal itself. But it is its difference or reference to other (outwardly reflecting shine). No other at hand from which particular differentiated than universal itself. Universal determines itself, so is itself the particular. Determinateness = its difference; only differentiated from itself. Species are: (a) universal itself, (b) particular. Universal = concept itself and its opposite. Totality and principle of its diversity (determined wholly and solely through itself).',
  provenance: {
    sourceChunk: 'part-3-universal-itself-differentiation',
    sourceOp: 'part-op-3-universal-itself-differentiation',
  },
};

export const partOp4TrueLogicalDivision: LogicalOperation = {
  id: 'part-op-4-true-logical-division',
  chunkId: 'part-4-true-logical-division',
  label: 'True Logical Division; Universal/Particular as Two Particulars',
  clauses: [
    'concept.setsItself = asImmediateUniversality',
    'indeterminateness.makes = determinateness',
    'indeterminateness.makes = particular',
    'twoSides = particular',
    'twoSides = coordinated',
    'twoSides = determinateAgainstUniversal',
    'twoSides = subordinated',
    'universal = oneOfOpposingSides',
    'determinateness.overAgainstEachOther = oneDeterminateness',
    'oneDeterminateness = negativity',
    'negativity.inUniversal = simple',
  ],
  predicates: [
    { name: 'immediateUniversality', args: ['concept.setsItself'] },
    { name: 'particular', args: ['twoSides'] },
    { name: 'coordinated', args: ['twoSides'] },
    { name: 'subordinated', args: ['twoSides'] },
    { name: 'oneDeterminateness', args: ['determinateness.overAgainstEachOther'] },
    { name: 'simple', args: ['negativity.inUniversal'] },
  ],
  relations: [
    { predicate: 'setsItself', from: 'concept', to: 'asImmediateUniversality' },
    { predicate: 'makes', from: 'indeterminateness', to: 'determinateness' },
    { predicate: 'is', from: 'twoSides', to: 'particular' },
    { predicate: 'is', from: 'oneDeterminateness', to: 'negativity' },
  ],
  candidateSummary: 'No other true logical division than this: concept sets itself as immediate, indeterminate universality. This indeterminateness makes its determinateness (or that it is a particular). Two are both a particular and therefore coordinated. Both, as particular, also determinate as against universal (subordinated). But even this universal = just one of opposing sides. Their determinateness over against each other = essentially only one determinateness = negativity which in universal is simple.',
  provenance: {
    sourceChunk: 'part-4-true-logical-division',
    sourceOp: 'part-op-4-true-logical-division',
  },
};

export const partOp5DifferenceTruth: LogicalOperation = {
  id: 'part-op-5-difference-truth',
  chunkId: 'part-5-difference-truth',
  label: 'Difference in Its Truth; Concept Unity',
  clauses: [
    'difference.here = inItsConcept',
    'difference.here = inItsTruth',
    'previousDifference.has = unityInConcept',
    'differenceInBeing = limitOfOther',
    'differenceInReflection = relative',
    'differenceInReflection = referringToOther',
    'unityOfConcept.begins = toBePosited',
    'transitoriness.significance = attainConcept',
    'causeAndEffect = oneDeterminateConcept',
    'causality = simpleConcept',
  ],
  predicates: [
    { name: 'inItsConcept', args: ['difference.here'] },
    { name: 'inItsTruth', args: ['difference.here'] },
    { name: 'limitOfOther', args: ['differenceInBeing'] },
    { name: 'relative', args: ['differenceInReflection'] },
    { name: 'simpleConcept', args: ['causality'] },
  ],
  relations: [
    { predicate: 'is', from: 'difference.here', to: 'inItsConcept' },
    { predicate: 'has', from: 'previousDifference', to: 'unityInConcept' },
    { predicate: 'is', from: 'differenceInBeing', to: 'limitOfOther' },
    { predicate: 'is', from: 'causality', to: 'simpleConcept' },
  ],
  candidateSummary: 'Difference here = in its concept and therefore in its truth. All previous difference has this unity in the concept. In being: difference = limit of an other. In reflection: difference = relative, posited as referring essentially to its other. Here: unity of concept begins to be posited. True significance of transitoriness/dissolution = they attain to their concept, their truth. Cause and effect = not two diverse concepts but only one determinate concept. Causality = simple concept.',
  provenance: {
    sourceChunk: 'part-5-difference-truth',
    sourceOp: 'part-op-5-difference-truth',
  },
};

export const partOp6CompletenessNatureImpotence: LogicalOperation = {
  id: 'part-op-6-completeness-nature-impotence',
  chunkId: 'part-6-completeness-nature-impotence',
  label: 'Completeness and Nature\'s Impotence',
  clauses: [
    'particularity.complete = inDifferenceOfUniversalAndParticular',
    'twoSpecies = makeUpParticularity',
    'nature.has = moreThanTwoSpecies',
    'impotenceOfNature = cannotAbideByConcept',
    'impotenceOfNature = losesItselfInBlindManifoldness',
    'nature = selfExternalityOfConcept',
    'concept = absolutePower',
    'concept.canLet = differenceGoFree',
    'manifoldness = abstractSideOfNothingness',
  ],
  predicates: [
    { name: 'complete', args: ['particularity'] },
    { name: 'impotenceOfNature', args: ['nature'] },
    { name: 'selfExternalityOfConcept', args: ['nature'] },
    { name: 'absolutePower', args: ['concept'] },
    { name: 'abstractSideOfNothingness', args: ['manifoldness'] },
  ],
  relations: [
    { predicate: 'complete', from: 'particularity', to: 'inDifferenceOfUniversalAndParticular' },
    { predicate: 'is', from: 'nature', to: 'selfExternalityOfConcept' },
    { predicate: 'is', from: 'concept', to: 'absolutePower' },
    { predicate: 'canLet', from: 'concept', to: 'differenceGoFree' },
  ],
  candidateSummary: 'Determinate moment of particularity complete in difference of universal and particular. Only these two make up particular species. More than two species found in any genus in nature. This = impotence of nature (cannot abide by rigor of concept, loses itself in blind manifoldness void of concept). Wonder at nature = without concept, object is irrational. Nature = self-externality of concept. Concept = absolute power (can let its difference go free). All of which = abstract side of nothingness.',
  provenance: {
    sourceChunk: 'part-6-completeness-nature-impotence',
    sourceOp: 'part-op-6-completeness-nature-impotence',
  },
};

export const partOp7DeterminatenessPrincipleMoment: LogicalOperation = {
  id: 'part-op-7-determinateness-principle-moment',
  chunkId: 'part-7-determinateness-principle-moment',
  label: 'Determinateness as Principle and Moment; Abstract Universality as Form',
  clauses: [
    'determinateness = simpleAsPrinciple',
    'determinateness = simpleAsMomentOfTotality',
    'concept.behaves = negativelyTowardsUnity',
    'concept.givesItself = formOfIdealMoment',
    'being = immediacyEqualToItself',
    'being = absoluteMediation',
    'universality = abstractUniversality',
    'universality = form',
    'determinateness = content',
    'pureUniversal = absoluteNegativity',
  ],
  predicates: [
    { name: 'simpleAsPrinciple', args: ['determinateness'] },
    { name: 'simpleAsMomentOfTotality', args: ['determinateness'] },
    { name: 'immediacyEqualToItself', args: ['being'] },
    { name: 'absoluteMediation', args: ['being'] },
    { name: 'abstractUniversality', args: ['universality'] },
    { name: 'absoluteNegativity', args: ['pureUniversal'] },
  ],
  relations: [
    { predicate: 'is', from: 'determinateness', to: 'simpleAsPrinciple' },
    { predicate: 'behaves', from: 'concept', to: 'negativelyTowardsUnity' },
    { predicate: 'is', from: 'universality', to: 'form' },
    { predicate: 'is', from: 'determinateness', to: 'content' },
  ],
  candidateSummary: 'Determinateness of particular = simple as principle. Also simple as moment of totality. Concept, in determining itself, behaves negatively towards its unity. Gives itself form of one of its ideal moments of being. This being = immediacy equal to itself by virtue of absolute mediation. Universality with which determinate clothes itself = abstract universality. Universality = form in it, determinateness = content. In pure universal: only absolute negativity, not difference posited as such.',
  provenance: {
    sourceChunk: 'part-7-determinateness-principle-moment',
    sourceOp: 'part-op-7-determinateness-principle-moment',
  },
};

export const partOp8AbstractUniversalityFormContent: LogicalOperation = {
  id: 'part-op-8-abstract-universality-form-content',
  chunkId: 'part-8-abstract-universality-form-content',
  label: 'Abstract Universality; Form/Content; Unconceptualized Concept',
  clauses: [
    'determinateness = abstraction',
    'otherDeterminateness = universalityItself',
    'concept.inUniversality = outsideItself',
    'abstractUniversal.contains = universality',
    'abstractUniversal.contains = determinateness',
    'abstractUniversal.contains = simpleUnity',
    'mediation = condition',
    'mediation != posited',
    'unityOfAbstraction = formOfImmediacy',
    'content = indifferenceToUniversality',
    'abstractUniversal = unconceptualizedConcept',
  ],
  predicates: [
    { name: 'abstraction', args: ['determinateness'] },
    { name: 'outsideItself', args: ['concept.inUniversality'] },
    { name: 'condition', args: ['mediation'] },
    { name: 'formOfImmediacy', args: ['unityOfAbstraction'] },
    { name: 'unconceptualizedConcept', args: ['abstractUniversal'] },
  ],
  relations: [
    { predicate: 'is', from: 'determinateness', to: 'abstraction' },
    { predicate: 'contains', from: 'abstractUniversal', to: 'universality' },
    { predicate: 'is', from: 'unityOfAbstraction', to: 'formOfImmediacy' },
    { predicate: 'is', from: 'abstractUniversal', to: 'unconceptualizedConcept' },
  ],
  candidateSummary: 'Determinateness = abstraction as against other determinateness. Other determinateness = only universality itself (abstract universality). Abstract-universal contains all moments of concept: (a) universality, (b) determinateness, (c) simple unity (but immediate, particularity not as totality). Essentially reference to other excluding it. Mediation only a condition, not posited. Abstract universal = concept, but unconceptualized concept (concept not posited as such).',
  provenance: {
    sourceChunk: 'part-8-abstract-universality-form-content',
    sourceOp: 'part-op-8-abstract-universality-form-content',
  },
};

export const partOp9UnderstandingAbstractUniversal: LogicalOperation = {
  id: 'part-op-9-understanding-abstract-universal',
  chunkId: 'part-9-understanding-abstract-universal',
  label: 'Understanding and Abstract Universal; Principle of Differentiation',
  clauses: [
    'determinateConcept = abstractUniversal',
    'conceptAsSuch = unconceptualizedConcept',
    'understanding = facultyOfConcept',
    'demonstration.belongsTo = understanding',
    'progression.doesNotReach = pastFinitude',
    'highestEssence = determinatenessOfIndeterminateness',
    'absoluteSubstance = abstract',
    'abstraction != empty',
    'determinateConcept.empty = ifNotPrincipleOfDifferentiation',
    'principle.contains = beginningAndEssence',
    'concept.absoluteDeterminateness = trueContent',
  ],
  predicates: [
    { name: 'abstractUniversal', args: ['determinateConcept'] },
    { name: 'unconceptualizedConcept', args: ['conceptAsSuch'] },
    { name: 'facultyOfConcept', args: ['understanding'] },
    { name: 'abstract', args: ['absoluteSubstance'] },
    { name: 'trueContent', args: ['concept.absoluteDeterminateness'] },
  ],
  relations: [
    { predicate: 'is', from: 'determinateConcept', to: 'abstractUniversal' },
    { predicate: 'belongsTo', from: 'demonstration', to: 'understanding' },
    { predicate: 'contains', from: 'principle', to: 'beginningAndEssence' },
    { predicate: 'is', from: 'concept.absoluteDeterminateness', to: 'trueContent' },
  ],
  candidateSummary: 'Determinate concept (ordinarily meant) = abstract universal. Concept as such (generally understood) = unconceptualized concept. Understanding = its faculty. Demonstration belongs to understanding. Progression by way of concepts does not reach past finitude and necessity. Abstraction not as empty as usually said. Any determinate concept empty if determinateness not principle of differentiation. Principle contains beginning and essence of development/realization. To reproach concept as empty = ignore its absolute determinateness.',
  provenance: {
    sourceChunk: 'part-9-understanding-abstract-universal',
    sourceOp: 'part-op-9-understanding-abstract-universal',
  },
};

export const partOp10UnderstandingForceFixity: LogicalOperation = {
  id: 'part-op-10-understanding-force-fixity',
  chunkId: 'part-10-understanding-force-fixity',
  label: 'Understanding\'s Force and Fixity; Dialectical and Appearance of Reason',
  clauses: [
    'fixity = formOfAbstractUniversality',
    'infiniteForce = splittingConcrete',
    'understanding.gives = rigidityOfBeing',
    'understanding.quickens = withSpirit',
    'ripestMaturity = beginningOfFall',
    'universality.contradicts = determinatenessOfFinite',
    'determinateConcept = essentialMomentOfReason',
    'appearanceOfReason = beginning',
  ],
  predicates: [
    { name: 'formOfAbstractUniversality', args: ['fixity'] },
    { name: 'splittingConcrete', args: ['infiniteForce'] },
    { name: 'rigidityOfBeing', args: ['understanding.gives'] },
    { name: 'essentialMomentOfReason', args: ['determinateConcept'] },
  ],
  relations: [
    { predicate: 'is', from: 'fixity', to: 'formOfAbstractUniversality' },
    { predicate: 'gives', from: 'understanding', to: 'rigidityOfBeing' },
    { predicate: 'quickens', from: 'understanding', to: 'withSpirit' },
    { predicate: 'contradicts', from: 'universality', to: 'determinatenessOfFinite' },
  ],
  candidateSummary: 'Understanding held in low repute. Fixity = form of abstract universality (makes unalterable). Infinite force of understanding = splitting concrete into abstract determinacies. Understanding gives rigidity of being. By simplifying, understanding quickens with spirit, sharpens them. Only at that point do they obtain capacity to dissolve themselves. Ripest maturity = stage at which fall begins. Universality directly contradicts determinateness of finite. Common practice of separating understanding and reason = to be rejected. Determinate and abstract concept = essential moment of reason. Beginning of appearance of reason.',
  provenance: {
    sourceChunk: 'part-10-understanding-force-fixity',
    sourceOp: 'part-op-10-understanding-force-fixity',
  },
};

export const partOp11UnderstandingReason: LogicalOperation = {
  id: 'part-op-11-understanding-reason',
  chunkId: 'part-11-understanding-reason',
  label: 'Understanding and Reason; Essential Moment',
  clauses: [
    'separatingUnderstandingReason = rejected',
    'conceptVoidOfReason = incapacityOfReason',
    'determinateConcept = conditionOfReason',
    'finite.internallyKindled = bySpirit',
    'finite.posited = asDialectical',
  ],
  predicates: [
    { name: 'rejected', args: ['separatingUnderstandingReason'] },
    { name: 'incapacityOfReason', args: ['conceptVoidOfReason'] },
    { name: 'conditionOfReason', args: ['determinateConcept'] },
    { name: 'dialectical', args: ['finite.posited'] },
  ],
  relations: [
    { predicate: 'is', from: 'separatingUnderstandingReason', to: 'rejected' },
    { predicate: 'is', from: 'determinateConcept', to: 'conditionOfReason' },
    { predicate: 'posited', from: 'finite', to: 'asDialectical' },
  ],
  candidateSummary: 'Common practice of separating understanding and reason = to be rejected. To consider concept void of reason = incapacity of reason to recognize itself in concept. Determinate and abstract concept = condition (essential moment) of reason. Form quickened by spirit in which finite (through universality referring to itself) internally kindled, posited as dialectical. Beginning of appearance of reason.',
  provenance: {
    sourceChunk: 'part-11-understanding-reason',
    sourceOp: 'part-op-11-understanding-reason',
  },
};

export const partOp12TransitionSingularity: LogicalOperation = {
  id: 'part-op-12-transition-singularity',
  chunkId: 'part-12-transition-singularity',
  label: 'Transition to Singularity; Absolute Turning Back',
  clauses: [
    'difference.receivesDue = inDeterminateConcept',
    'determinateUniversality = selfReferringDeterminateness',
    'determinateUniversality = absoluteNegativity',
    'selfReferringDeterminateness = singularity',
    'universality.immediately = particularity',
    'particularity.immediately = singularity',
    'singularity = thirdMoment',
    'singularity = absoluteTurningBack',
    'singularity = positedLoss',
  ],
  predicates: [
    { name: 'selfReferringDeterminateness', args: ['determinateUniversality'] },
    { name: 'absoluteNegativity', args: ['determinateUniversality'] },
    { name: 'singularity', args: ['selfReferringDeterminateness'] },
    { name: 'thirdMoment', args: ['singularity'] },
    { name: 'absoluteTurningBack', args: ['singularity'] },
    { name: 'positedLoss', args: ['singularity'] },
  ],
  relations: [
    { predicate: 'receivesDue', from: 'difference', to: 'inDeterminateConcept' },
    { predicate: 'is', from: 'determinateUniversality', to: 'selfReferringDeterminateness' },
    { predicate: 'is', from: 'selfReferringDeterminateness', to: 'singularity' },
    { predicate: 'is', from: 'singularity', to: 'absoluteTurningBack' },
  ],
  candidateSummary: 'Difference receives its due in determinate concept. Determinateness in form of universality united to form simple. Determinate universality = self-referring determinateness, absolute negativity posited for itself. Self-referring determinateness = singularity. Universality immediately = particularity in and for itself. Particularity immediately = singularity in and for itself. Singularity = third moment of concept. Also = absolute turning back of concept into itself. At same time = posited loss of itself.',
  provenance: {
    sourceChunk: 'part-12-transition-singularity',
    sourceOp: 'part-op-12-transition-singularity',
  },
};

export const particularConceptOperations: LogicalOperation[] = [
  partOp1DeterminatenessImmanent,
  partOp2ContainsUniversalityTotality,
  partOp3UniversalItselfDifferentiation,
  partOp4TrueLogicalDivision,
  partOp5DifferenceTruth,
  partOp6CompletenessNatureImpotence,
  partOp7DeterminatenessPrincipleMoment,
  partOp8AbstractUniversalityFormContent,
  partOp9UnderstandingAbstractUniversal,
  partOp10UnderstandingForceFixity,
  partOp11UnderstandingReason,
  partOp12TransitionSingularity,
];
