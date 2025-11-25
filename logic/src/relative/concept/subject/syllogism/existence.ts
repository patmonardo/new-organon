/**
 * Logical Operations: The Syllogism of Existence
 *
 * The syllogism of existence is the immediate form of the syllogism.
 * It is the qualitative syllogism, where terms are immediate determinacies.
 *
 * Dialectical Movement:
 * - First Figure (S-P-U): Singular connects with Universal through Particular.
 * - Second Figure (P-S-U): Particular connects with Universal through Singular.
 * - Third Figure (S-U-P): Singular connects with Particular through Universal.
 * - Fourth Figure (U-U-U): Mathematical syllogism (relationless).
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE SYLLOGISM OF EXISTENCE
// ============================================================================

export const sylExistOp1IntroductionImmediate: LogicalOperation = {
  id: 'syl-exist-op-1-introduction-immediate',
  chunkId: 'syl-exist-1-introduction-immediate',
  label: 'Introduction: Immediate Form; Particularity as Middle Term',
  clauses: [
    'syllogism.form = immediate',
    'determinacies = abstract',
    'determinacies = singular',
    'firstSyllogism = strictlyFormal',
    'concept.partitioned = S_P_U',
    'particularity = middleTerm',
    'middleTerm.unites = S_and_U',
  ],
  predicates: [
    { name: 'immediate', args: ['syllogism.form'] },
    { name: 'abstract', args: ['determinacies'] },
    { name: 'singular', args: ['determinacies'] },
    { name: 'strictlyFormal', args: ['firstSyllogism'] },
    { name: 'middleTerm', args: ['particularity'] },
  ],
  relations: [
    { predicate: 'partitioned', from: 'concept', to: 'S_P_U' },
    { predicate: 'unites', from: 'middleTerm', to: 'S_and_U' },
  ],
  candidateSummary: 'Syllogism in immediate form has determinations as immediate. Abstract determinacies of form (not developed into concretion). First syllogism = strictly formal. Concept partitioned: S and U = extremes, itself = P between them. Particularity = middle term (unites S and U immediately). Dialectical movement = positing moments of mediation.',
  provenance: {
    sourceChunk: 'syl-exist-1-introduction-immediate',
    sourceOp: 'syl-exist-op-1-introduction-immediate',
  },
};

export const sylExistOp2FirstSchema: LogicalOperation = {
  id: 'syl-exist-op-2-first-schema',
  chunkId: 'syl-exist-2-first-schema',
  label: 'First Figure: S-P-U Schema',
  clauses: [
    'schema = S_P_U',
    'singularity.connects = withUniversality',
    'connection.means = particularity',
    'universality.lowers = itself',
    'determinations = extremes',
    'one = thirdTerm',
    'determinateness = identical',
  ],
  predicates: [
    { name: 'S_P_U', args: ['schema'] },
    { name: 'extremes', args: ['determinations'] },
    { name: 'thirdTerm', args: ['one'] },
    { name: 'identical', args: ['determinateness'] },
  ],
  relations: [
    { predicate: 'connects', from: 'singularity', to: 'withUniversality' },
    { predicate: 'means', from: 'connection', to: 'particularity' },
    { predicate: 'lowers', from: 'universality', to: 'itself' },
  ],
  candidateSummary: 'S-P-U = general schema. Singularity connects with universality through particularity. Singular not universal immediately but by means of particularity. Universality lowers itself through particularity. Determinations stand as extremes, one in third term. Both determinateness (identical, universal determinateness = particularity).',
  provenance: {
    sourceChunk: 'syl-exist-2-first-schema',
    sourceOp: 'syl-exist-op-2-first-schema',
  },
};

export const sylExistOp3GeneralMeaning: LogicalOperation = {
  id: 'syl-exist-op-3-general-meaning',
  chunkId: 'syl-exist-3-general-meaning',
  label: 'General Meaning: Singular Emerges into Existence; Still Subjective',
  clauses: [
    'singular.emerges = intoExistence',
    'emergence.through = particularity',
    'existence = universality',
    'singular.setsApart = asParticularity',
    'particularity = concretedTerm',
    'particularity = trueSingular',
    'objectiveSignificance = superficial',
    'significance = subjective',
    'relation = necessary',
  ],
  predicates: [
    { name: 'intoExistence', args: ['singular.emerges'] },
    { name: 'universality', args: ['existence'] },
    { name: 'concretedTerm', args: ['particularity'] },
    { name: 'trueSingular', args: ['particularity'] },
    { name: 'superficial', args: ['objectiveSignificance'] },
    { name: 'subjective', args: ['significance'] },
    { name: 'necessary', args: ['relation'] },
  ],
  relations: [
    { predicate: 'through', from: 'emergence', to: 'particularity' },
    { predicate: 'setsApart', from: 'singular', to: 'asParticularity' },
  ],
  candidateSummary: 'Singular (infinite self-reference, inwardness) emerges through particularity into existence (universality, external conjunction). Conversely: sets itself apart as particularity (concreted term, self-referring universal, true singular). In universality: gone from externality into itself. Objective significance only superficially present. Still subjective. Relation S-P-U = necessary essential form-relation.',
  provenance: {
    sourceChunk: 'syl-exist-3-general-meaning',
    sourceOp: 'syl-exist-op-3-general-meaning',
  },
};

export const sylExistOp4Aristotle: LogicalOperation = {
  id: 'syl-exist-op-4-aristotle',
  chunkId: 'syl-exist-4-aristotle',
  label: 'Aristotle\'s Definition: Relation of Inherence; Variant Forms',
  clauses: [
    'aristotle.definition = relationOfInherence',
    'extremes.united = inMiddle',
    'relation = equal',
    'otherRelations.validity = reducible',
    'variantForms = necessaryPassage',
    'firstForm.passesOver = intoVariant',
    'variant = totality',
  ],
  predicates: [
    { name: 'relationOfInherence', args: ['aristotle.definition'] },
    { name: 'equal', args: ['relation'] },
    { name: 'reducible', args: ['otherRelations.validity'] },
    { name: 'necessaryPassage', args: ['variantForms'] },
    { name: 'totality', args: ['variant'] },
  ],
  relations: [
    { predicate: 'united', from: 'extremes', to: 'inMiddle' },
    { predicate: 'passesOver', from: 'firstForm', to: 'intoVariant' },
  ],
  candidateSummary: 'Aristotle: mere relation of inherence. One extreme in entire middle, middle in entire other extreme → extremes united. Repetition of equal relation. Other relations have validity only reducible to original. Not diverse species but variant forms into which first abstract form necessarily passes over (further determines, becomes totality).',
  provenance: {
    sourceChunk: 'syl-exist-4-aristotle',
    sourceOp: 'syl-exist-op-4-aristotle',
  },
};

export const sylExistOp5Subsumption: LogicalOperation = {
  id: 'syl-exist-op-5-subsumption',
  chunkId: 'syl-exist-5-subsumption',
  label: 'S-P-U: Subsumption and Inherence',
  clauses: [
    'singular.subsumed = underParticular',
    'particular.subsumed = underUniversal',
    'singular.subsumed = underUniversal',
    'particular.inheres = inSingular',
    'universal.inheres = inParticular',
    'universal.inheres = inSingular',
    'particular.is = subject',
    'particular.is = predicate',
    'extremes = joined',
  ],
  predicates: [
    { name: 'subsumed', args: ['singular', 'underParticular'] },
    { name: 'subsumed', args: ['particular', 'underUniversal'] },
    { name: 'subsumed', args: ['singular', 'underUniversal'] },
    { name: 'inheres', args: ['particular', 'inSingular'] },
    { name: 'inheres', args: ['universal', 'inParticular'] },
    { name: 'inheres', args: ['universal', 'inSingular'] },
    { name: 'subject', args: ['particular'] },
    { name: 'predicate', args: ['particular'] },
    { name: 'joined', args: ['extremes'] },
  ],
  relations: [
    { predicate: 'is', from: 'particular', to: 'subject' },
    { predicate: 'is', from: 'particular', to: 'predicate' },
  ],
  candidateSummary: 'S-P-U = general schema. Singular subsumed under particular, particular under universal → singular subsumed under universal. Or: particular inheres in singular, universal in particular → universal inheres in singular. With respect to universal: particular = subject. With respect to singular: particular = predicate. Both determinations united → extremes joined.',
  provenance: {
    sourceChunk: 'syl-exist-5-subsumption',
    sourceOp: 'syl-exist-op-5-subsumption',
  },
};

export const sylExistOp6Therefore: LogicalOperation = {
  id: 'syl-exist-op-6-therefore',
  chunkId: 'syl-exist-6-therefore',
  label: '"Therefore" Grounded in Extremes; Not Merely Subjective',
  clauses: [
    'therefore = inference',
    'therefore.grounded = inExtremes',
    'connection.asJudgment = abstractiveReflection',
    'trueConnection.posited = asMiddleTerm',
    'middleTerm = determinate',
    'middleTerm = repleteWithContent',
    'syllogisticInference = determinateMiddle',
    'threeJudgments = formalisticView',
  ],
  predicates: [
    { name: 'inference', args: ['therefore'] },
    { name: 'abstractiveReflection', args: ['connection.asJudgment'] },
    { name: 'determinate', args: ['middleTerm'] },
    { name: 'repleteWithContent', args: ['middleTerm'] },
    { name: 'determinateMiddle', args: ['syllogisticInference'] },
    { name: 'formalisticView', args: ['threeJudgments'] },
  ],
  relations: [
    { predicate: 'grounded', from: 'therefore', to: 'inExtremes' },
    { predicate: 'posited', from: 'trueConnection', to: 'asMiddleTerm' },
  ],
  candidateSummary: '"Therefore" appears as inference in subject (subjective insight). But not external determination. Grounded in nature of extremes themselves. Connection as judgment only for abstractive reflection. True connection = posited as middle term. Not merely subjective judgment (not empty "is" but determinate middle replete with content) = meaning of syllogistic inference.',
  provenance: {
    sourceChunk: 'syl-exist-6-therefore',
    sourceOp: 'syl-exist-op-6-therefore',
  },
};

export const sylExistOp7ExampleBoredom: LogicalOperation = {
  id: 'syl-exist-op-7-example-boredom',
  chunkId: 'syl-exist-7-example-boredom',
  label: 'Example: "All humans are mortal"; Boredom; All Things are Syllogism',
  clauses: [
    'subjectiveReflection.splits = isolatedPremises',
    'boredom.from = futility',
    'syllogisticInference = subjectiveForm',
    'nature.united = inUnityOfEssence',
    'rationality = objectiveElement',
    'priorImmediacy = mereSubjectivity',
    'allThings = syllogism',
    'universal.united = withSingularity',
  ],
  predicates: [
    { name: 'subjectiveForm', args: ['syllogisticInference'] },
    { name: 'objectiveElement', args: ['rationality'] },
    { name: 'mereSubjectivity', args: ['priorImmediacy'] },
    { name: 'syllogism', args: ['allThings'] },
  ],
  relations: [
    { predicate: 'splits', from: 'subjectiveReflection', to: 'isolatedPremises' },
    { predicate: 'from', from: 'boredom', to: 'futility' },
    { predicate: 'united', from: 'nature', to: 'inUnityOfEssence' },
    { predicate: 'united', from: 'universal', to: 'withSingularity' },
  ],
  candidateSummary: 'Subjective reflection splits into isolated premises and conclusion. Example: All humans mortal, Gaius human, Therefore Gaius mortal. Boredom from futility. Appears as subjective expedient. Nature does not operate this way. Syllogistic inference = subjective form. Nature = determinations united in unity of essence. Rationality = objective element. All things are syllogism (universal united through particularity with singularity).',
  provenance: {
    sourceChunk: 'syl-exist-7-example-boredom',
    sourceOp: 'syl-exist-op-7-example-boredom',
  },
};

export const sylExistOp8TermsContent: LogicalOperation = {
  id: 'syl-exist-op-8-terms-content',
  chunkId: 'syl-exist-8-terms-content',
  label: 'Terms as Content: Qualitative, Singular Determinacies',
  clauses: [
    'terms = immediateDeterminations',
    'terms = content',
    'terms = qualitative',
    'terms = singularDeterminacies',
    'singular = immediateConcrete',
    'particularity = properties',
    'universality = abstract',
    'subject.notPosited = inConcept',
    'selfReferringDeterminateness = infiniteManifoldness',
    'anyDeterminacy = middleTerm',
  ],
  predicates: [
    { name: 'immediateDeterminations', args: ['terms'] },
    { name: 'content', args: ['terms'] },
    { name: 'qualitative', args: ['terms'] },
    { name: 'singularDeterminacies', args: ['terms'] },
    { name: 'immediateConcrete', args: ['singular'] },
    { name: 'properties', args: ['particularity'] },
    { name: 'abstract', args: ['universality'] },
    { name: 'infiniteManifoldness', args: ['selfReferringDeterminateness'] },
    { name: 'middleTerm', args: ['anyDeterminacy'] },
  ],
  relations: [
    { predicate: 'notPosited', from: 'subject', to: 'inConcept' },
  ],
  candidateSummary: 'Terms have form of immediate determinations; consider as content. Qualitative. Terms = singular determinacies (self-reference as indifferent to form, hence content). Singular = immediate concrete. Particularity = determinacies/properties/relations. Universality = more abstract/singularized. Subject not posited in concept. Self-referring determinateness = indeterminate, infinite manifoldness. Any determinacy may serve as middle term.',
  provenance: {
    sourceChunk: 'syl-exist-8-terms-content',
    sourceOp: 'syl-exist-op-8-terms-content',
  },
};

export const sylExistOp9ContingencyContradiction: LogicalOperation = {
  id: 'syl-exist-op-9-contingency-contradiction',
  chunkId: 'syl-exist-9-contingency-contradiction',
  label: 'Contingency and Contradiction; Examples',
  clauses: [
    'middleTerm.choice = accidental',
    'middleTerm.choice = arbitrary',
    'otherMiddleTerms = transitionsToOtherPredicates',
    'syllogisms.runInto = contradiction',
    'difference = opposition',
    'concrete = unityInConcept',
    'unityInConcept.contains = opposites',
  ],
  predicates: [
    { name: 'accidental', args: ['middleTerm.choice'] },
    { name: 'arbitrary', args: ['middleTerm.choice'] },
    { name: 'transitionsToOtherPredicates', args: ['otherMiddleTerms'] },
    { name: 'opposition', args: ['difference'] },
    { name: 'unityInConcept', args: ['concrete'] },
  ],
  relations: [
    { predicate: 'runInto', from: 'syllogisms', to: 'contradiction' },
    { predicate: 'contains', from: 'unityInConcept', to: 'opposites' },
  ],
  candidateSummary: 'Entirely accidental/arbitrary which property taken. Other middle terms = transitions to other predicates. Same middle may be transition to different predicates. Syllogisms must run into contradiction. Difference = opposition. Concrete = unity in concept of opposites.',
  provenance: {
    sourceChunk: 'syl-exist-9-contingency-contradiction',
    sourceOp: 'syl-exist-op-9-contingency-contradiction',
  },
};

export const sylExistOp10KantAntinomies: LogicalOperation = {
  id: 'syl-exist-op-10-kant-antinomies',
  chunkId: 'syl-exist-10-kant-antinomies',
  label: 'Kant\'s Antinomies; Form\'s Abstractness',
  clauses: [
    'formalSyllogism = unsatisfactory',
    'middleTerm = chance',
    'deduction = nothing',
    'kantAntinomies = equalNecessity',
    'insufficiency.liesIn = form',
    'content = oneSidedQuality',
    'form = abstractness',
    'abstractSingularity = infiniteManifold',
    'abstractParticularity = singleQuality',
    'abstractUniversal = contingent',
  ],
  predicates: [
    { name: 'unsatisfactory', args: ['formalSyllogism'] },
    { name: 'chance', args: ['middleTerm'] },
    { name: 'nothing', args: ['deduction'] },
    { name: 'equalNecessity', args: ['kantAntinomies'] },
    { name: 'oneSidedQuality', args: ['content'] },
    { name: 'abstractness', args: ['form'] },
    { name: 'infiniteManifold', args: ['abstractSingularity'] },
    { name: 'singleQuality', args: ['abstractParticularity'] },
    { name: 'contingent', args: ['abstractUniversal'] },
  ],
  relations: [
    { predicate: 'liesIn', from: 'insufficiency', to: 'form' },
  ],
  candidateSummary: 'Nothing as unsatisfactory as formal syllogism (middle term = chance/arbitrariness). Elegant deduction amounts to nothing. Kant\'s antinomies = one determination laid down at one time, another at another, both equal necessity. Insufficiency lies in form (content = one-sided quality because form\'s abstractness). Abstract singularity = infinite manifold. Abstract particularity = single quality. Totally contingent as regards content.',
  provenance: {
    sourceChunk: 'syl-exist-10-kant-antinomies',
    sourceOp: 'syl-exist-op-10-kant-antinomies',
  },
};

export const sylExistOp11DeterminationsConnections: LogicalOperation = {
  id: 'syl-exist-op-11-determinations-connections',
  chunkId: 'syl-exist-11-determinations-connections',
  label: 'Determinations as Connections; Premises Need Proof',
  clauses: [
    'determinations = content',
    'essence = connections',
    'connections = extremesToMiddle',
    'premises = immediate',
    'conclusion = mediated',
    'premises.contradict = natureOfSyllogism',
    'truthOfJudgment = syllogisticConclusion',
    'premises.cannotRemain = immediate',
  ],
  predicates: [
    { name: 'content', args: ['determinations'] },
    { name: 'connections', args: ['essence'] },
    { name: 'extremesToMiddle', args: ['connections'] },
    { name: 'immediate', args: ['premises'] },
    { name: 'mediated', args: ['conclusion'] },
    { name: 'syllogisticConclusion', args: ['truthOfJudgment'] },
  ],
  relations: [
    { predicate: 'contradict', from: 'premises', to: 'natureOfSyllogism' },
    { predicate: 'cannotRemain', from: 'premises', to: 'immediate' },
  ],
  candidateSummary: 'Determinations = determinations of content (reflected into themselves) but essence = determinations of form (essentially connections). Connections: extremes to middle (immediate, premises), extremes to one another (mediated, conclusion). Premises = propositions/judgments (contradict nature of syllogism). Truth of judgment = syllogistic conclusion. Premises cannot remain immediate.',
  provenance: {
    sourceChunk: 'syl-exist-11-determinations-connections',
    sourceOp: 'syl-exist-op-11-determinations-connections',
  },
};

export const sylExistOp12InfiniteProgression: LogicalOperation = {
  id: 'syl-exist-op-12-infinite-progression',
  chunkId: 'syl-exist-12-infinite-progression',
  label: 'Infinite Progression; Bad Infinity',
  clauses: [
    'premises.willBe = proved',
    'progression = geometrical',
    'progression = infinity',
    'infinity = badInfinity',
    'contradiction = qualitativeBeingAndImpotentOught',
    'progression = repeatedDemand',
    'truth = sublationOfForm',
    'relation.comesUp = again',
  ],
  predicates: [
    { name: 'proved', args: ['premises.willBe'] },
    { name: 'geometrical', args: ['progression'] },
    { name: 'infinity', args: ['progression'] },
    { name: 'badInfinity', args: ['infinity'] },
    { name: 'qualitativeBeingAndImpotentOught', args: ['contradiction'] },
    { name: 'repeatedDemand', args: ['progression'] },
    { name: 'sublationOfForm', args: ['truth'] },
  ],
  relations: [
    { predicate: 'comesUp', from: 'relation', to: 'again' },
  ],
  candidateSummary: 'Premises will be proved (exhibited as conclusions). Two premises → two syllogisms → four premises → geometrical progression to infinity. Bad infinity. Contradiction of qualitative being and impotent ought. Progression = repeated demand for unity, fall back. Truth = sublation of it and form. Relation supposed to be sublated comes up again.',
  provenance: {
    sourceChunk: 'syl-exist-12-infinite-progression',
    sourceOp: 'syl-exist-op-12-infinite-progression',
  },
};

export const sylExistOp13MediationOtherWay: LogicalOperation = {
  id: 'syl-exist-op-13-mediation-other-way',
  chunkId: 'syl-exist-13-mediation-other-way',
  label: 'Mediation in Other Way; Second and Third Figures',
  clauses: [
    'mediation.mustOccur = inOtherWay',
    'P_S_U = secondFigure',
    'S_U_P = thirdFigure',
    'mediation = contingent',
    'mediation.presupposes = immediacy',
    'singular.becomes = mediatingTerm',
    'conclusion.expresses = singularAsUniversal',
    'singular.posited = asUniversality',
  ],
  predicates: [
    { name: 'inOtherWay', args: ['mediation.mustOccur'] },
    { name: 'secondFigure', args: ['P_S_U'] },
    { name: 'thirdFigure', args: ['S_U_P'] },
    { name: 'contingent', args: ['mediation'] },
    { name: 'mediatingTerm', args: ['singular.becomes'] },
    { name: 'singularAsUniversal', args: ['conclusion.expresses'] },
    { name: 'asUniversality', args: ['singular.posited'] },
  ],
  relations: [
    { predicate: 'presupposes', from: 'mediation', to: 'immediacy' },
  ],
  candidateSummary: 'Mediation must occur in other way. For P-U: S available → P-S-U. For S-P: U available → S-U-P. Mediation = contingent (external arbitrary choice). Mediation presupposes immediacy. Singular become mediating term through conclusion. Conclusion S-U: singular posited as universal. Singular posited as universality of extremes or middle.',
  provenance: {
    sourceChunk: 'syl-exist-13-mediation-other-way',
    sourceOp: 'syl-exist-op-13-mediation-other-way',
  },
};

export const sylExistOp14SecondFigure: LogicalOperation = {
  id: 'syl-exist-op-14-second-figure',
  chunkId: 'syl-exist-14-second-figure',
  label: 'Second Figure: P-S-U; Negative Unity; Singular as Middle',
  clauses: [
    'truthOfFirst = contingency',
    'subject.conceived = inExternality',
    'singular = middle',
    'syllogisticConnection = sublationOfImmediacy',
    'connection = negativeUnity',
    'mediation.contains = negativeMoment',
    'extremes = particularAndUniversal',
    'particular.exchanged = places',
    'universal.not = determinateParticular',
  ],
  predicates: [
    { name: 'contingency', args: ['truthOfFirst'] },
    { name: 'inExternality', args: ['subject.conceived'] },
    { name: 'middle', args: ['singular'] },
    { name: 'sublationOfImmediacy', args: ['syllogisticConnection'] },
    { name: 'negativeUnity', args: ['connection'] },
    { name: 'particularAndUniversal', args: ['extremes'] },
    { name: 'determinateParticular', args: ['universal.not'] },
  ],
  relations: [
    { predicate: 'contains', from: 'mediation', to: 'negativeMoment' },
    { predicate: 'exchanged', from: 'particular', to: 'places' },
  ],
  candidateSummary: 'Truth of first: not in and for itself united but by contingency/singularity. Subject conceived only in externality. Immediacy = basis → singular in truth = middle. Syllogistic connection = sublation of immediacy (negative unity). Mediation contains negative moment. Extremes = particular and universal. Particular exchanged places. Universal not determinate particular but one species through singularity.',
  provenance: {
    sourceChunk: 'syl-exist-14-second-figure',
    sourceOp: 'syl-exist-op-14-second-figure',
  },
};

export const sylExistOp15SecondTermsImmediate: LogicalOperation = {
  id: 'syl-exist-op-15-second-terms-immediate',
  chunkId: 'syl-exist-15-second-terms-immediate',
  label: 'Second Figure: Terms Still Immediate',
  clauses: [
    'terms = immediateDeterminacies',
    'positionsExchanged = form',
    'content = indifferent',
    'qualities.linked = throughAccidentalSingularity',
  ],
  predicates: [
    { name: 'immediateDeterminacies', args: ['terms'] },
    { name: 'form', args: ['positionsExchanged'] },
    { name: 'indifferent', args: ['content'] },
  ],
  relations: [
    { predicate: 'linked', from: 'qualities', to: 'throughAccidentalSingularity' },
  ],
  candidateSummary: 'Terms still immediate determinacies (not advanced to objective signification). Positions exchanged = form (external). Still each content indifferent (two qualities linked through accidental singularity).',
  provenance: {
    sourceChunk: 'syl-exist-15-second-terms-immediate',
    sourceOp: 'syl-exist-op-15-second-terms-immediate',
  },
};

export const sylExistOp16TransitionRealization: LogicalOperation = {
  id: 'syl-exist-op-16-transition-realization',
  chunkId: 'syl-exist-16-transition-realization',
  label: 'Transition: Realization and Alteration',
  clauses: [
    'first = immediateSyllogism',
    'transition = beginningOfRealization',
    'transition = negativeMoment',
    'transition = alteration',
    'species.shouldConform = toGenus',
    'trueMeaning = passedOver',
    'conclusion = correct',
  ],
  predicates: [
    { name: 'immediateSyllogism', args: ['first'] },
    { name: 'beginningOfRealization', args: ['transition'] },
    { name: 'negativeMoment', args: ['transition'] },
    { name: 'alteration', args: ['transition'] },
    { name: 'passedOver', args: ['trueMeaning'] },
    { name: 'correct', args: ['conclusion'] },
  ],
  relations: [
    { predicate: 'shouldConform', from: 'species', to: 'toGenus' },
  ],
  candidateSummary: 'First = immediate syllogism (abstract form not realized). Transition = beginning of realization (negative moment) but also alteration (no longer conforms). Regarded as subjective = species should conform to genus S-P-U. But does not conform. True meaning = latter has passed over into it. Conclusion correct because so on its own.',
  provenance: {
    sourceChunk: 'syl-exist-16-transition-realization',
    sourceOp: 'syl-exist-op-16-transition-realization',
  },
};

export const sylExistOp17SecondParticular: LogicalOperation = {
  id: 'syl-exist-op-17-second-particular',
  chunkId: 'syl-exist-17-second-particular',
  label: 'Second Figure: Particular Conclusion',
  clauses: [
    'secondFigure = third',
    'S_P = oppositeRelation',
    'conclusion.canOnlyBe = particular',
    'particularJudgment = positiveAndNegative',
    'extremes = indifferent',
    'relation = indifferent',
  ],
  predicates: [
    { name: 'third', args: ['secondFigure'] },
    { name: 'oppositeRelation', args: ['S_P'] },
    { name: 'particular', args: ['conclusion.canOnlyBe'] },
    { name: 'positiveAndNegative', args: ['particularJudgment'] },
    { name: 'indifferent', args: ['extremes'] },
    { name: 'indifferent', args: ['relation'] },
  ],
  relations: [],
  candidateSummary: 'Second figure (referred to as third). To be correct: must be commensurate with first. S-P would have opposite relation (P subsumed under S) = sublation of determinate judgment. Conclusion can only be particular. Particular judgment = positive and negative. Particular and universal = extremes, indifferent. Relation indifferent.',
  provenance: {
    sourceChunk: 'syl-exist-17-second-particular',
    sourceOp: 'syl-exist-op-17-second-particular',
  },
};

export const sylExistOp18SecondSelfSublating: LogicalOperation = {
  id: 'syl-exist-op-18-second-self-sublating',
  chunkId: 'syl-exist-18-second-self-sublating',
  label: 'Second Figure: Self-Sublating; Points to Universal',
  clauses: [
    'conclusion = universalConnection',
    'mediationOfFirst = implicitlyContingent',
    'contingency.inSecond = posited',
    'mediation = selfSublating',
    'joined = immediatelyIdentical',
    'mediatingMiddle = infinitelyManifold',
    'externalityOfSingularity = universality',
    'pointsBeyond = toMediationByUniversal',
  ],
  predicates: [
    { name: 'universalConnection', args: ['conclusion'] },
    { name: 'implicitlyContingent', args: ['mediationOfFirst'] },
    { name: 'posited', args: ['contingency.inSecond'] },
    { name: 'selfSublating', args: ['mediation'] },
    { name: 'immediatelyIdentical', args: ['joined'] },
    { name: 'infinitelyManifold', args: ['mediatingMiddle'] },
    { name: 'universality', args: ['externalityOfSingularity'] },
  ],
  relations: [
    { predicate: 'pointsBeyond', from: 'externalityOfSingularity', to: 'toMediationByUniversal' },
  ],
  candidateSummary: 'Conclusion positive and negative = universal connection. Mediation of first = implicitly contingent. In second: contingency posited. Mediation = self-sublating (singularity/immediacy). What joined must be immediately identical. Externality of singularity = universality. Points beyond to mediation by means of universal.',
  provenance: {
    sourceChunk: 'syl-exist-18-second-self-sublating',
    sourceOp: 'syl-exist-op-18-second-self-sublating',
  },
};

export const sylExistOp19TransitionConcept: LogicalOperation = {
  id: 'syl-exist-op-19-transition-concept',
  chunkId: 'syl-exist-19-transition-concept',
  label: 'Transition: Qualitative Base, According Concept',
  clauses: [
    'transition = likeTransitionOfBeing',
    'singularity.conjoins = bySublatingDeterminateness',
    'conjunction = contingency',
    'positiveUnity = abstractUniversality',
    'middleTerm.posited = anotherForm',
  ],
  predicates: [
    { name: 'likeTransitionOfBeing', args: ['transition'] },
    { name: 'contingency', args: ['conjunction'] },
    { name: 'abstractUniversality', args: ['positiveUnity'] },
    { name: 'anotherForm', args: ['middleTerm.posited'] },
  ],
  relations: [
    { predicate: 'conjoins', from: 'singularity', to: 'bySublatingDeterminateness' },
  ],
  candidateSummary: 'Transition like transition of being (alteration, qualitative base, immediacy of singularity). But according concept: singularity conjoins by sublating determinateness (presents as contingency). Positive unity = abstract universality. Middle term posited in this determination = another form.',
  provenance: {
    sourceChunk: 'syl-exist-19-transition-concept',
    sourceOp: 'syl-exist-op-19-transition-concept',
  },
};

export const sylExistOp20ThirdFigure: LogicalOperation = {
  id: 'syl-exist-op-20-third-figure',
  chunkId: 'syl-exist-20-third-figure',
  label: 'Third Figure: S-U-P; Reciprocal Mediation',
  clauses: [
    'premise = noSingleImmediate',
    'S_U = mediatedByFirst',
    'P_U = mediatedBySecond',
    'determination = completed',
    'mediation = reciprocal',
    'S_U_P = truthOfFormalSyllogism',
    'middle = abstractUniversal',
    'formalism = indifferentToForm',
  ],
  predicates: [
    { name: 'noSingleImmediate', args: ['premise'] },
    { name: 'mediatedByFirst', args: ['S_U'] },
    { name: 'mediatedBySecond', args: ['P_U'] },
    { name: 'completed', args: ['determination'] },
    { name: 'reciprocal', args: ['mediation'] },
    { name: 'truthOfFormalSyllogism', args: ['S_U_P'] },
    { name: 'abstractUniversal', args: ['middle'] },
    { name: 'indifferentToForm', args: ['formalism'] },
  ],
  relations: [],
  candidateSummary: 'No single immediate premise. S-U mediated by first, P-U by second. Presupposes both, conversely presupposed. Reciprocal mediation: each mediation but not totality. S-U-P = truth of formal syllogism. Expresses: middle = abstract universal, extremes not contained according essential determinateness. Formalism: terms have immediate content indifferent to form.',
  provenance: {
    sourceChunk: 'syl-exist-20-third-figure',
    sourceOp: 'syl-exist-op-20-third-figure',
  },
};

export const sylExistOp21ThirdNegative: LogicalOperation = {
  id: 'syl-exist-op-21-third-negative',
  chunkId: 'syl-exist-21-third-negative',
  label: 'Third Figure: Negative Conclusion; Fourth Figure',
  clauses: [
    'middle = unityOfExtremes',
    'middle = indeterminateUniversal',
    'middle = subsumes',
    'conclusion = necessarilyNegative',
    'determination = indifferent',
    'premise = indifferent',
    'fourthFigure = void',
    'fourthFigure = uninteresting',
    'position = reverseOfFirst',
  ],
  predicates: [
    { name: 'unityOfExtremes', args: ['middle'] },
    { name: 'indeterminateUniversal', args: ['middle'] },
    { name: 'subsumes', args: ['middle'] },
    { name: 'necessarilyNegative', args: ['conclusion'] },
    { name: 'indifferent', args: ['determination'] },
    { name: 'indifferent', args: ['premise'] },
    { name: 'void', args: ['fourthFigure'] },
    { name: 'uninteresting', args: ['fourthFigure'] },
    { name: 'reverseOfFirst', args: ['position'] },
  ],
  relations: [],
  candidateSummary: 'Middle = unity of extremes (abstraction from determinateness = indeterminate universal). As universal, middle = subsumes/predicate. Conclusion necessarily negative. Indifferent which determination predicate/subject. Ground of fourth figure (unknown to Aristotle, void, uninteresting). Position reverse of first. Totally idle.',
  provenance: {
    sourceChunk: 'syl-exist-21-third-negative',
    sourceOp: 'syl-exist-op-21-third-negative',
  },
};

export const sylExistOp22ThirdObjective: LogicalOperation = {
  id: 'syl-exist-op-22-third-objective',
  chunkId: 'syl-exist-22-third-objective',
  label: 'Third Figure: Objective Significance; Fourth Figure',
  clauses: [
    'objectiveSignificance.mediatingTerm = essentiallyUniversal',
    'mediatingTerm = qualitativeAbstract',
    'conjunction.ground = outside',
    'conjunction = contingent',
    'posited = indifferentExternal',
    'fourthFigure.arose = byBareAbstraction',
    'fourthFigure = relationless',
  ],
  predicates: [
    { name: 'essentiallyUniversal', args: ['objectiveSignificance.mediatingTerm'] },
    { name: 'qualitativeAbstract', args: ['mediatingTerm'] },
    { name: 'outside', args: ['conjunction.ground'] },
    { name: 'contingent', args: ['conjunction'] },
    { name: 'indifferentExternal', args: ['posited'] },
    { name: 'relationless', args: ['fourthFigure'] },
  ],
  relations: [
    { predicate: 'arose', from: 'fourthFigure', to: 'byBareAbstraction' },
  ],
  candidateSummary: 'Objective significance: mediating term = essentially universal. But only qualitative/abstract. Conjunction must have ground outside (contingent). Since universal determined as middle and determinateness not contained, posited as indifferent/external. By bare abstraction: fourth figure arose (U-U-U, relationless). Abstracts from qualitative differentiation, external unity.',
  provenance: {
    sourceChunk: 'syl-exist-22-third-objective',
    sourceOp: 'syl-exist-op-22-third-objective',
  },
};

export const sylExistOp23FourthMathematical: LogicalOperation = {
  id: 'syl-exist-op-23-fourth-mathematical',
  chunkId: 'syl-exist-23-fourth-mathematical',
  label: 'Fourth Figure: U-U-U, Mathematical Syllogism',
  clauses: [
    'mathematicalSyllogism = U_U_U',
    'inherenceSubsumption = doneAway',
    'third = mediatingTerm',
    'mediatingTerm = noDetermination',
    'ranks = asAxiom',
    'selfEvidence.liesIn = formalism',
    'quantitative = byAbstraction',
    'comprehension = none',
  ],
  predicates: [
    { name: 'U_U_U', args: ['mathematicalSyllogism'] },
    { name: 'doneAway', args: ['inherenceSubsumption'] },
    { name: 'mediatingTerm', args: ['third'] },
    { name: 'noDetermination', args: ['mediatingTerm'] },
    { name: 'asAxiom', args: ['ranks'] },
    { name: 'formalism', args: ['selfEvidence.liesIn'] },
    { name: 'byAbstraction', args: ['quantitative'] },
    { name: 'none', args: ['comprehension'] },
  ],
  relations: [],
  candidateSummary: 'Mathematical syllogism: if two equal to third, then equal to each other. Inherence/subsumption done away. "Third" = mediating term (absolutely no determination). Ranks as axiom. Self-evidence lies in formalism (abstracts from qualitative, only quantitative). Not without presupposition. No conceptual comprehension.',
  provenance: {
    sourceChunk: 'syl-exist-23-fourth-mathematical',
    sourceOp: 'syl-exist-op-23-fourth-mathematical',
  },
};

export const sylExistOp24ResultPositive: LogicalOperation = {
  id: 'syl-exist-op-24-result-positive',
  chunkId: 'syl-exist-24-result-positive',
  label: 'Result: Positive Side; Mediation Based on Mediation',
  clauses: [
    'result.not = justAbstraction',
    'negativity.has = positiveSide',
    'syllogisms.have = oneAnotherForPresupposition',
    'extremes.conjoined = ifUnitedElsewhere',
    'middle = formalDeterminateness',
    'presupposed = mediation',
    'trulyPresent = mediationBasedOnMediation',
    'reciprocalPresupposing = turningBackIntoItself',
  ],
  predicates: [
    { name: 'justAbstraction', args: ['result.not'] },
    { name: 'positiveSide', args: ['negativity.has'] },
    { name: 'formalDeterminateness', args: ['middle'] },
    { name: 'mediation', args: ['presupposed'] },
    { name: 'mediationBasedOnMediation', args: ['trulyPresent'] },
    { name: 'turningBackIntoItself', args: ['reciprocalPresupposing'] },
  ],
  relations: [
    { predicate: 'have', from: 'syllogisms', to: 'oneAnotherForPresupposition' },
    { predicate: 'conjoined', from: 'extremes', to: 'ifUnitedElsewhere' },
  ],
  candidateSummary: 'Result not just abstraction. Negativity has positive side (other posited, determinateness concrete). Syllogisms all have one another for presupposition. Extremes truly conjoined only if otherwise united by identity grounded elsewhere. What presupposed = not given immediacy but itself mediation. Truly present = mediation based on mediation. Circle of reciprocal presupposing = turning back into itself.',
  provenance: {
    sourceChunk: 'syl-exist-24-result-positive',
    sourceOp: 'syl-exist-op-24-result-positive',
  },
};

export const sylExistOp25EachDetermination: LogicalOperation = {
  id: 'syl-exist-op-25-each-determination',
  chunkId: 'syl-exist-25-each-determination',
  label: 'Each Determination as Middle; Transition to Reflection',
  clauses: [
    'eachDetermination.occupied = middleTerm',
    'dialecticalMovement = singularityAndUniversality',
    'negativeResult = dissolutionIntoQuantitative',
    'positiveResult = mediationThroughConcreteIdentity',
    'deficiency = singleDeterminatenessAsMiddle',
    'mediation = indifferenceAndPositiveReflection',
    'passedOver = intoSyllogismOfReflection',
  ],
  predicates: [
    { name: 'middleTerm', args: ['eachDetermination.occupied'] },
    { name: 'singularityAndUniversality', args: ['dialecticalMovement'] },
    { name: 'dissolutionIntoQuantitative', args: ['negativeResult'] },
    { name: 'mediationThroughConcreteIdentity', args: ['positiveResult'] },
    { name: 'singleDeterminatenessAsMiddle', args: ['deficiency'] },
    { name: 'indifferenceAndPositiveReflection', args: ['mediation'] },
    { name: 'intoSyllogismOfReflection', args: ['passedOver'] },
  ],
  relations: [],
  candidateSummary: 'In whole of formal syllogisms, each determination has occupied place of middle term. Negative result = dissolution into quantitative. Positive result: mediation through concrete identity of determinacies. Deficiency/formalism = single determinateness supposed to constitute middle. Mediation = indifference of immediate/abstract determinations and positive reflection. Passed over into syllogism of reflection.',
  provenance: {
    sourceChunk: 'syl-exist-25-each-determination',
    sourceOp: 'syl-exist-op-25-each-determination',
  },
};

export const existenceSyllogismOperations: LogicalOperation[] = [
  sylExistOp1IntroductionImmediate,
  sylExistOp2FirstSchema,
  sylExistOp3GeneralMeaning,
  sylExistOp4Aristotle,
  sylExistOp5Subsumption,
  sylExistOp6Therefore,
  sylExistOp7ExampleBoredom,
  sylExistOp8TermsContent,
  sylExistOp9ContingencyContradiction,
  sylExistOp10KantAntinomies,
  sylExistOp11DeterminationsConnections,
  sylExistOp12InfiniteProgression,
  sylExistOp13MediationOtherWay,
  sylExistOp14SecondFigure,
  sylExistOp15SecondTermsImmediate,
  sylExistOp16TransitionRealization,
  sylExistOp17SecondParticular,
  sylExistOp18SecondSelfSublating,
  sylExistOp19TransitionConcept,
  sylExistOp20ThirdFigure,
  sylExistOp21ThirdNegative,
  sylExistOp22ThirdObjective,
  sylExistOp23FourthMathematical,
  sylExistOp24ResultPositive,
  sylExistOp25EachDetermination,
];
