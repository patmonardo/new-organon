/**
 * Logical Operations: The Syllogism of Necessity
 *
 * The syllogism of necessity is the categorical, hypothetical, and disjunctive syllogism.
 * Middle term is objective universality (genus).
 *
 * Dialectical Movement:
 * - Categorical Syllogism: Subject conjoined through substance.
 * - Hypothetical Syllogism: Necessary connection without immediacy.
 * - Disjunctive Syllogism: Middle as totality, concept realized as objectivity.
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE SYLLOGISM OF NECESSITY
// ============================================================================

export const sylNecOp1IntroductionObjective: LogicalOperation = {
  id: 'syl-nec-op-1-introduction-objective',
  chunkId: 'syl-nec-1-introduction-objective',
  label: 'Introduction: Middle Term as Objective Universality; Genus',
  clauses: [
    'middle = simpleDeterminateUniversality',
    'middle = objectiveUniversality',
    'middle = genus',
    'middle = fullOfContent',
    'middle = immanentReflectionOfDeterminateness',
    'extremes.have = innerIdentity',
    'terms = momentsOfNecessaryExistence',
    'necessity = connectionOfPositedForm',
  ],
  predicates: [
    { name: 'simpleDeterminateUniversality', args: ['middle'] },
    { name: 'objectiveUniversality', args: ['middle'] },
    { name: 'genus', args: ['middle'] },
    { name: 'fullOfContent', args: ['middle'] },
    { name: 'immanentReflectionOfDeterminateness', args: ['middle'] },
    { name: 'innerIdentity', args: ['extremes.have'] },
    { name: 'momentsOfNecessaryExistence', args: ['terms'] },
    { name: 'connectionOfPositedForm', args: ['necessity'] },
  ],
  relations: [],
  candidateSummary: 'Middle determined: simple determinate universality (like particularity in existence) and objective universality (like allness). Completed simple universality = genus. Full of content. Middle = immanent reflection of determinateness. Extremes have inner identity. Terms = moments of necessary existence. Realization: extremes as totality, necessity = connection of posited form.',
  provenance: {
    sourceChunk: 'syl-nec-1-introduction-objective',
    sourceOp: 'syl-nec-op-1-introduction-objective',
  },
};

export const sylNecOp2CategoricalFirst: LogicalOperation = {
  id: 'syl-nec-op-2-categorical-first',
  chunkId: 'syl-nec-2-categorical-first',
  label: 'Categorical: First Syllogism of Necessity; Through Substance',
  clauses: [
    'premises = categoricalJudgment',
    'middle = objectiveUniversality',
    'syllogism = ofInherence',
    'syllogism = firstOfNecessity',
    'subject.conjoined = throughSubstance',
    'substance.elevated = universal',
    'extremes = universalityAndSingularity',
    'universality = abstractDeterminateness',
    'singularity = actual',
  ],
  predicates: [
    { name: 'categoricalJudgment', args: ['premises'] },
    { name: 'objectiveUniversality', args: ['middle'] },
    { name: 'ofInherence', args: ['syllogism'] },
    { name: 'firstOfNecessity', args: ['syllogism'] },
    { name: 'universal', args: ['substance.elevated'] },
    { name: 'universalityAndSingularity', args: ['extremes'] },
    { name: 'abstractDeterminateness', args: ['universality'] },
    { name: 'actual', args: ['singularity'] },
  ],
  relations: [
    { predicate: 'conjoined', from: 'subject', to: 'throughSubstance' },
  ],
  candidateSummary: 'Has categorical judgment for premises. Middle = objective universality. Superficially = syllogism of inherence. Full import: first syllogism of necessity (subject conjoined through substance). Substance elevated = universal. Differences = extremes (universality and singularity). Universality = abstract/universal determinateness. Singularity = actual.',
  provenance: {
    sourceChunk: 'syl-nec-2-categorical-first',
    sourceOp: 'syl-nec-op-2-categorical-first',
  },
};

export const sylNecOp3CategoricalObjectivity: LogicalOperation = {
  id: 'syl-nec-op-3-categorical-objectivity',
  chunkId: 'syl-nec-3-categorical-objectivity',
  label: 'Categorical: No Contingency; Objectivity Begins',
  clauses: [
    'schema = S_P_U',
    'middle = essentialNature',
    'extreme = specificDifference',
    'contingency = none',
    'demandForProof = none',
    'terms.in = connectionOfIdentity',
    'essence = one',
    'objectivity = begins',
    'middle = identityFullOfContent',
    'subjectiveElement = indifferentSubsistence',
  ],
  predicates: [
    { name: 'S_P_U', args: ['schema'] },
    { name: 'essentialNature', args: ['middle'] },
    { name: 'specificDifference', args: ['extreme'] },
    { name: 'none', args: ['contingency'] },
    { name: 'none', args: ['demandForProof'] },
    { name: 'connectionOfIdentity', args: ['terms.in'] },
    { name: 'one', args: ['essence'] },
    { name: 'begins', args: ['objectivity'] },
    { name: 'identityFullOfContent', args: ['middle'] },
    { name: 'indifferentSubsistence', args: ['subjectiveElement'] },
  ],
  relations: [],
  candidateSummary: 'Schema S-P-U. Middle = essential nature. Extreme = universal determinateness of genus. No contingency. No external immediacy. Terms in connection of identity in and for itself. No longer subjective (objectivity begins). Middle = identity full of content (self-subsistence = genus).',
  provenance: {
    sourceChunk: 'syl-nec-3-categorical-objectivity',
    sourceOp: 'syl-nec-op-3-categorical-objectivity',
  },
};

export const sylNecOp4CategoricalHypothetical: LogicalOperation = {
  id: 'syl-nec-op-4-categorical-hypothetical',
  chunkId: 'syl-nec-4-categorical-hypothetical',
  label: 'Categorical: Still Subjective; Passes to Hypothetical',
  clauses: [
    'categorical = stillSubjective',
    'identity = innerBond',
    'universality = solidPositive',
    'immediacy.not = posited',
    'trulyImmediate = singular',
    'contingency.because = connectedAsObjectiveUniversality',
    'posited = indifferentAndContingent',
    'determined = toHypothetical',
  ],
  predicates: [
    { name: 'stillSubjective', args: ['categorical'] },
    { name: 'innerBond', args: ['identity'] },
    { name: 'solidPositive', args: ['universality'] },
    { name: 'posited', args: ['immediacy.not'] },
    { name: 'singular', args: ['trulyImmediate'] },
    { name: 'connectedAsObjectiveUniversality', args: ['contingency.because'] },
    { name: 'indifferentAndContingent', args: ['posited'] },
    { name: 'toHypothetical', args: ['determined'] },
  ],
  relations: [],
  candidateSummary: 'Still subjective (identity = substantial/content, not form). Identity = inner bond (necessity). Universality = solid positive (not negativity). Immediacy not posited. Truly immediate = singular (contingent that only this). Contingency because connected as objective universality. Posited: objective universality/immediate actualities/indifferent and contingent. But identity = only formal inner → determined to hypothetical.',
  provenance: {
    sourceChunk: 'syl-nec-4-categorical-hypothetical',
    sourceOp: 'syl-nec-op-4-categorical-hypothetical',
  },
};

export const sylNecOp5HypotheticalImmediacy: LogicalOperation = {
  id: 'syl-nec-op-5-hypothetical-immediacy',
  chunkId: 'syl-nec-5-hypothetical-immediacy',
  label: 'Hypothetical: Adds Immediacy; A as Middle Term',
  clauses: [
    'hypotheticalJudgment = necessaryConnectionWithoutImmediacy',
    'syllogism.adds = immediacy',
    'minor.expresses = immediateBeing',
    'conclusion = accomplishedMediatingUnity',
    'beingOfA = essentiallyAsMiddleTerm',
  ],
  predicates: [
    { name: 'necessaryConnectionWithoutImmediacy', args: ['hypotheticalJudgment'] },
    { name: 'immediacy', args: ['syllogism.adds'] },
    { name: 'immediateBeing', args: ['minor.expresses'] },
    { name: 'accomplishedMediatingUnity', args: ['conclusion'] },
    { name: 'essentiallyAsMiddleTerm', args: ['beingOfA'] },
  ],
  relations: [],
  candidateSummary: 'Hypothetical judgment = necessary connection without immediacy. Syllogism adds immediacy: If A is, so is B, But A is, Therefore B is. Minor expresses immediate being. Conclusion = accomplished mediating unity. Being of A = essentially as middle term.',
  provenance: {
    sourceChunk: 'syl-nec-5-hypothetical-immediacy',
    sourceOp: 'syl-nec-op-5-hypothetical-immediacy',
  },
};

export const sylNecOp6HypotheticalCondition: LogicalOperation = {
  id: 'syl-nec-op-6-hypothetical-condition',
  chunkId: 'syl-nec-6-hypothetical-condition',
  label: 'Hypothetical: Condition and Conditioned; A as Mediating Being',
  clauses: [
    'connection = necessity',
    'sides = heldInNecessity',
    'conditions = innerAbstract',
    'conditions = dismemberedDispersed',
    'A = mediatingBeing',
    'A.translates = accordingToConcept',
    'negativity = mediatingMeans',
    'activity = contradiction',
    'contingency.returned = intoConcept',
  ],
  predicates: [
    { name: 'necessity', args: ['connection'] },
    { name: 'heldInNecessity', args: ['sides'] },
    { name: 'innerAbstract', args: ['conditions'] },
    { name: 'dismemberedDispersed', args: ['conditions'] },
    { name: 'mediatingBeing', args: ['A'] },
    { name: 'accordingToConcept', args: ['A.translates'] },
    { name: 'mediatingMeans', args: ['negativity'] },
    { name: 'contradiction', args: ['activity'] },
    { name: 'intoConcept', args: ['contingency.returned'] },
  ],
  relations: [],
  candidateSummary: 'Connection = necessity/inner substantial identity. Sides = held in necessity. Conditions = inner abstract or dismembered dispersed. A = mediating being. Translates: not abstract immediacy but according to concept. Negativity = mediating means. Activity. Existent necessity. In categorical: contingency returned into concept as unity.',
  provenance: {
    sourceChunk: 'syl-nec-6-hypothetical-condition',
    sourceOp: 'syl-nec-op-6-hypothetical-condition',
  },
};

export const sylNecOp7HypotheticalIdentity: LogicalOperation = {
  id: 'syl-nec-op-7-hypothetical-identity',
  chunkId: 'syl-nec-7-hypothetical-identity',
  label: 'Hypothetical: Identity of Mediator and Mediated',
  clauses: [
    'conclusion.expresses = contradiction',
    'concept = same',
    'absoluteContent = same',
    'identity = mediatingTermAndMediated',
  ],
  predicates: [
    { name: 'contradiction', args: ['conclusion.expresses'] },
    { name: 'same', args: ['concept'] },
    { name: 'same', args: ['absoluteContent'] },
    { name: 'mediatingTermAndMediated', args: ['identity'] },
  ],
  relations: [],
  candidateSummary: 'Conclusion expresses contradiction (immediately but through other/mediated). Same concept as middle. Absolute content same. Identity of mediating term and mediated.',
  provenance: {
    sourceChunk: 'syl-nec-7-hypothetical-identity',
    sourceOp: 'syl-nec-op-7-hypothetical-identity',
  },
};

export const sylNecOp8HypotheticalDisjunctive: LogicalOperation = {
  id: 'syl-nec-op-8-hypothetical-disjunctive',
  chunkId: 'syl-nec-8-hypothetical-disjunctive',
  label: 'Hypothetical: Negative Unity; Passes to Disjunctive',
  clauses: [
    'connection.through = negativeUnity',
    'necessity.merges = withNecessary',
    'unity = reflectedIntoItself',
    'beingOfA = beingOfB',
    'immediateBeing = mediated',
    'mediation = selfReferringNegativity',
    'mediation = disjunctiveSyllogism',
  ],
  predicates: [
    { name: 'negativeUnity', args: ['connection.through'] },
    { name: 'withNecessary', args: ['necessity.merges'] },
    { name: 'reflectedIntoItself', args: ['unity'] },
    { name: 'beingOfB', args: ['beingOfA'] },
    { name: 'mediated', args: ['immediateBeing'] },
    { name: 'selfReferringNegativity', args: ['mediation'] },
    { name: 'disjunctiveSyllogism', args: ['mediation'] },
  ],
  relations: [],
  candidateSummary: 'First to display necessary connection through form/negative unity. Necessity merges with necessary. Unity reflected into itself. Being of A = that of B and vice versa. Immediate being = mediated. Mediation = singularity/immediacy/self-referring negativity. = disjunctive syllogism.',
  provenance: {
    sourceChunk: 'syl-nec-8-hypothetical-disjunctive',
    sourceOp: 'syl-nec-op-8-hypothetical-disjunctive',
  },
};

export const sylNecOp9DisjunctiveTotality: LogicalOperation = {
  id: 'syl-nec-op-9-disjunctive-totality',
  chunkId: 'syl-nec-9-disjunctive-totality',
  label: 'Disjunctive: S-U-P; Middle as Totality',
  clauses: [
    'hypothetical = U_S_P',
    'disjunctive = S_U_P',
    'middle = universalityRepleteWithForm',
    'middle = totality',
    'middle = universalityParticularitySingularity',
    'particularization = differentiation',
    'eitherOr = negativeUnity',
    'A = subject',
  ],
  predicates: [
    { name: 'U_S_P', args: ['hypothetical'] },
    { name: 'S_U_P', args: ['disjunctive'] },
    { name: 'universalityRepleteWithForm', args: ['middle'] },
    { name: 'totality', args: ['middle'] },
    { name: 'universalityParticularitySingularity', args: ['middle'] },
    { name: 'differentiation', args: ['particularization'] },
    { name: 'negativeUnity', args: ['eitherOr'] },
    { name: 'subject', args: ['A'] },
  ],
  relations: [],
  candidateSummary: 'Hypothetical = U-S-P. Disjunctive = S-U-P. Middle = universality replete with form (totality). Middle = universality/particularity/singularity. Particularization = differentiation (either-or = negative unity). A = subject in premises and conclusion.',
  provenance: {
    sourceChunk: 'syl-nec-9-disjunctive-totality',
    sourceOp: 'syl-nec-op-9-disjunctive-totality',
  },
};

export const sylNecOp10DisjunctiveSublated: LogicalOperation = {
  id: 'syl-nec-op-10-disjunctive-sublated',
  chunkId: 'syl-nec-10-disjunctive-sublated',
  label: 'Disjunctive: No Longer Syllogism; Formalism Sublated',
  clauses: [
    'mediated = universalityWithSingularity',
    'mediatingMeans = A',
    'truthOfHypothetical = unityOfMediatorAndMediated',
    'noLonger = syllogism',
    'middle.contains = extremesInCompleteDeterminateness',
    'extremes = onlyPositedness',
    'formalism = sublated',
    'mediated = essentialMoment',
  ],
  predicates: [
    { name: 'universalityWithSingularity', args: ['mediated'] },
    { name: 'A', args: ['mediatingMeans'] },
    { name: 'unityOfMediatorAndMediated', args: ['truthOfHypothetical'] },
    { name: 'syllogism', args: ['noLonger'] },
    { name: 'extremesInCompleteDeterminateness', args: ['middle.contains'] },
    { name: 'onlyPositedness', args: ['extremes'] },
    { name: 'sublated', args: ['formalism'] },
    { name: 'essentialMoment', args: ['mediated'] },
  ],
  relations: [],
  candidateSummary: 'What mediated = universality with singularity. Mediating means = A. Truth of hypothetical. No longer syllogism. Middle as totality contains extremes in complete determinateness. Extremes = only positedness. Formalism sublated. Mediated = essential moment.',
  provenance: {
    sourceChunk: 'syl-nec-10-disjunctive-sublated',
    sourceOp: 'syl-nec-op-10-disjunctive-sublated',
  },
};

export const sylNecOp11ReviewObjectivity: LogicalOperation = {
  id: 'syl-nec-op-11-review-objectivity',
  chunkId: 'syl-nec-11-review-objectivity',
  label: 'Review: Concept Realized as Objectivity',
  clauses: [
    'figures = eachDeterminatenessSingly',
    'reflection = gatheringExternally',
    'necessity = developedTotalSimple',
    'concept.realized = objectivity',
    'firstReality = partitionsItself',
    'inwardnessOfExternality = equatedWithInnerUnity',
    'result = objectivity',
  ],
  predicates: [
    { name: 'eachDeterminatenessSingly', args: ['figures'] },
    { name: 'gatheringExternally', args: ['reflection'] },
    { name: 'developedTotalSimple', args: ['necessity'] },
    { name: 'objectivity', args: ['concept.realized'] },
    { name: 'partitionsItself', args: ['firstReality'] },
    { name: 'equatedWithInnerUnity', args: ['inwardnessOfExternality'] },
    { name: 'objectivity', args: ['result'] },
  ],
  relations: [],
  candidateSummary: 'Figures: each determinateness singly as middle. Reflection: gathering externally. Necessity: developed/total/simple, form sublated. Concept realized (reality = objectivity). First reality: partitions itself. Inwardness of externality: equated with inner unity. Result = immediacy emerged (concept restored). = objectivity.',
  provenance: {
    sourceChunk: 'syl-nec-11-review-objectivity',
    sourceOp: 'syl-nec-op-11-review-objectivity',
  },
};

export const necessitySyllogismOperations: LogicalOperation[] = [
  sylNecOp1IntroductionObjective,
  sylNecOp2CategoricalFirst,
  sylNecOp3CategoricalObjectivity,
  sylNecOp4CategoricalHypothetical,
  sylNecOp5HypotheticalImmediacy,
  sylNecOp6HypotheticalCondition,
  sylNecOp7HypotheticalIdentity,
  sylNecOp8HypotheticalDisjunctive,
  sylNecOp9DisjunctiveTotality,
  sylNecOp10DisjunctiveSublated,
  sylNecOp11ReviewObjectivity,
];
