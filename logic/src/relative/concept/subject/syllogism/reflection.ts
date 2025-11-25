/**
 * Logical Operations: The Syllogism of Reflection
 *
 * The syllogism of reflection is the syllogism of allness, induction, and analogy.
 * Middle term is not merely abstract particularity but a totality.
 *
 * Dialectical Movement:
 * - Syllogism of Allness: Understanding in perfection, external universality.
 * - Syllogism of Induction: Singularity as completed, experience, bad infinity.
 * - Syllogism of Analogy: Singular in universal nature, superficiality.
 * - Transition to Necessity: Mediation as concrete unity.
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE SYLLOGISM OF REFLECTION
// ============================================================================

export const sylReflOp1IntroductionSublated: LogicalOperation = {
  id: 'syl-refl-op-1-introduction-sublated',
  chunkId: 'syl-refl-1-introduction-sublated',
  label: 'Introduction: Sublated Abstractness; Middle Term as Totality',
  clauses: [
    'qualitativeSyllogism.course = sublatedAbstractness',
    'term.posited = asDeterminateness',
    'other.shines = reflectively',
    'connection = mediatedAndNecessary',
    'determinateness = concrete',
    'middle = totality',
    'unity = reflectionEmbracingExtremes',
  ],
  predicates: [
    { name: 'sublatedAbstractness', args: ['qualitativeSyllogism.course'] },
    { name: 'asDeterminateness', args: ['term.posited'] },
    { name: 'reflectively', args: ['other.shines'] },
    { name: 'mediatedAndNecessary', args: ['connection'] },
    { name: 'concrete', args: ['determinateness'] },
    { name: 'totality', args: ['middle'] },
    { name: 'reflectionEmbracingExtremes', args: ['unity'] },
  ],
  relations: [],
  candidateSummary: 'Course of qualitative syllogism has sublated abstractness. Term posited as determinateness in which other shines reflectively. Connection present (mediated and necessary). Each determinateness concrete. Middle was abstract particularity. Now = totality (posited unity of extremes). Unity = reflection embracing extremes.',
  provenance: {
    sourceChunk: 'syl-refl-1-introduction-sublated',
    sourceOp: 'syl-refl-op-1-introduction-sublated',
  },
};

export const sylReflOp2ExtremesMiddle: LogicalOperation = {
  id: 'syl-refl-op-2-extremes-middle',
  chunkId: 'syl-refl-2-extremes-middle',
  label: 'Extremes from Judgment of Reflection; Middle Contains Totality',
  clauses: [
    'extremes = determinationsOfJudgmentOfReflection',
    'singular.contains = universalityAbsolutelyReflected',
    'middle.contains = singularityAndUniversality',
    'middle = totality',
    'middle = genuineDeterminateness',
    'immediate = indeterminate',
    'syllogism = ofAllness',
  ],
  predicates: [
    { name: 'determinationsOfJudgmentOfReflection', args: ['extremes'] },
    { name: 'universalityAbsolutelyReflected', args: ['singular.contains'] },
    { name: 'totality', args: ['middle'] },
    { name: 'genuineDeterminateness', args: ['middle'] },
    { name: 'indeterminate', args: ['immediate'] },
    { name: 'ofAllness', args: ['syllogism'] },
  ],
  relations: [
    { predicate: 'contains', from: 'middle', to: 'singularityAndUniversality' },
  ],
  candidateSummary: 'Extremes = determinations of judgment of reflection (singularity proper, universality as relation/reflection embracing manifold). Singular contains universality absolutely reflected. Middle contains: singularity, expanded into universality as "all", universality at basis. First to possess genuine determinateness (middle = totality). = syllogism of allness.',
  provenance: {
    sourceChunk: 'syl-refl-2-extremes-middle',
    sourceOp: 'syl-refl-op-2-extremes-middle',
  },
};

export const sylReflOp3AllnessUnderstanding: LogicalOperation = {
  id: 'syl-refl-op-3-allness-understanding',
  chunkId: 'syl-refl-3-allness-understanding',
  label: 'Allness: Understanding in Perfection; External Universality',
  clauses: [
    'allness = understandingInPerfection',
    'middle.developed = intoMoments',
    'form.gathers = singularIntoUniversalityExternally',
    'singular.behaves = asImmediateSubsisting',
    'negationOfImmediacy = firstNegation',
    'allness = externalUniversalityOfReflection',
  ],
  predicates: [
    { name: 'understandingInPerfection', args: ['allness'] },
    { name: 'intoMoments', args: ['middle.developed'] },
    { name: 'asImmediateSubsisting', args: ['singular.behaves'] },
    { name: 'firstNegation', args: ['negationOfImmediacy'] },
    { name: 'externalUniversalityOfReflection', args: ['allness'] },
  ],
  relations: [
    { predicate: 'gathers', from: 'form', to: 'singularIntoUniversalityExternally' },
  ],
  candidateSummary: 'Allness = understanding in perfection (but more not yet). Middle developed into moments (concrete). But form gathers singular into universality only externally. Singular behaves as immediate subsisting. Negation of immediacy = only first negation. Allness = external universality of reflection (not universality of concept).',
  provenance: {
    sourceChunk: 'syl-refl-3-allness-understanding',
    sourceOp: 'syl-refl-op-3-allness-understanding',
  },
};

export const sylReflOp4AllnessConcrete: LogicalOperation = {
  id: 'syl-refl-op-4-allness-concrete',
  chunkId: 'syl-refl-4-allness-concrete',
  label: 'Allness: Concrete Totality; Examples',
  clauses: [
    'existence = contingent',
    'middle.contains = singularity',
    'predicate.belongs = concretely',
    'allness.contains = asConcretedTerm',
    'predicates.mustBe = commensurateWithConcreteTotality',
    'allThingsGreen = allActualConcreteThings',
  ],
  predicates: [
    { name: 'contingent', args: ['existence'] },
    { name: 'concretely', args: ['predicate.belongs'] },
    { name: 'asConcretedTerm', args: ['allness.contains'] },
    { name: 'commensurateWithConcreteTotality', args: ['predicates.mustBe'] },
    { name: 'allActualConcreteThings', args: ['allThingsGreen'] },
  ],
  relations: [
    { predicate: 'contains', from: 'middle', to: 'singularity' },
  ],
  candidateSummary: 'Existence contingent. But middle contains singularity (concrete) → only predicate concretely belonging attached. When allness: contains as concreted term (not abstraction). Only predicates commensurate with concrete totality. "All things green" = all actual concrete things (concreted with all properties).',
  provenance: {
    sourceChunk: 'syl-refl-4-allness-concrete',
    sourceOp: 'syl-refl-op-4-allness-concrete',
  },
};

export const sylReflOp5AllnessIllusion: LogicalOperation = {
  id: 'syl-refl-op-5-allness-illusion',
  chunkId: 'syl-refl-5-allness-illusion',
  label: 'Allness: Mere Illusion; Major Premise Presupposes Conclusion',
  clauses: [
    'reflectivePerfection.makes = mereIllusion',
    'middle = all',
    'all = allSingulars',
    'subject.possesses = predicateImmediately',
    'major.presupposes = conclusion',
    'major.notCorrect = onItsOwn',
  ],
  predicates: [
    { name: 'mereIllusion', args: ['reflectivePerfection.makes'] },
    { name: 'all', args: ['middle'] },
    { name: 'allSingulars', args: ['all'] },
    { name: 'predicateImmediately', args: ['subject.possesses'] },
    { name: 'onItsOwn', args: ['major.notCorrect'] },
  ],
  relations: [
    { predicate: 'presupposes', from: 'major', to: 'conclusion' },
  ],
  candidateSummary: 'Reflective perfection makes mere illusion. Middle = "all" (immediately attached predicate). But "all" = "all singulars" → subject already possesses predicate immediately. Or: subject obtains as consequence, but major already contains conclusion. Major not correct on its own (presupposes conclusion of which should be ground).',
  provenance: {
    sourceChunk: 'syl-refl-5-allness-illusion',
    sourceOp: 'syl-refl-op-5-allness-illusion',
  },
};

export const sylReflOp6ExampleMortal: LogicalOperation = {
  id: 'syl-refl-op-6-example-mortal',
  chunkId: 'syl-refl-6-example-mortal',
  label: 'Example: "All humans are mortal"',
  clauses: [
    'major.correct = onlyBecauseConclusionCorrect',
    'conclusion.mustBe = correctOnItsOwnImmediately',
    'antecedentQuestion = whetherConclusionCounterInstance',
  ],
  predicates: [
    { name: 'onlyBecauseConclusionCorrect', args: ['major.correct'] },
    { name: 'correctOnItsOwnImmediately', args: ['conclusion.mustBe'] },
    { name: 'whetherConclusionCounterInstance', args: ['antecedentQuestion'] },
  ],
  relations: [],
  candidateSummary: 'All humans mortal, Gaius human, Therefore Gaius mortal. Major correct only because/to extent conclusion correct. Were Gaius not mortal, major would not be correct. Conclusion must be correct on its own immediately. Before major accepted, antecedent question: whether conclusion counter-instance.',
  provenance: {
    sourceChunk: 'syl-refl-6-example-mortal',
    sourceOp: 'syl-refl-op-6-example-mortal',
  },
};

export const sylReflOp7ResultPosited: LogicalOperation = {
  id: 'syl-refl-op-7-result-posited',
  chunkId: 'syl-refl-7-result-posited',
  label: 'Result Posited in Syllogism Itself; Essence Rests on Singularity',
  clauses: [
    'result.posited = inSyllogismItself',
    'syllogism = externalEmptyReflectiveSemblance',
    'essence.restsOn = subjectiveSingularity',
    'singular.connected = immediately',
    'major.mediated = throughSingularityAsAllness',
    'syllogism = ofInduction',
  ],
  predicates: [
    { name: 'inSyllogismItself', args: ['result.posited'] },
    { name: 'externalEmptyReflectiveSemblance', args: ['syllogism'] },
    { name: 'subjectiveSingularity', args: ['essence.restsOn'] },
    { name: 'immediately', args: ['singular.connected'] },
    { name: 'throughSingularityAsAllness', args: ['major.mediated'] },
    { name: 'ofInduction', args: ['syllogism'] },
  ],
  relations: [],
  candidateSummary: 'From concept (existence): premises contradicted. In reflection: result posited in syllogism itself (major presupposes conclusion). = only external empty reflective semblance. Essence rests on subjective singularity. Or: singular connected immediately. Major mediated through singularity as allness. = syllogism of induction.',
  provenance: {
    sourceChunk: 'syl-refl-7-result-posited',
    sourceOp: 'syl-refl-op-7-result-posited',
  },
};

export const sylReflOp8InductionSchema: LogicalOperation = {
  id: 'syl-refl-op-8-induction-schema',
  chunkId: 'syl-refl-8-induction-schema',
  label: 'Induction: U-S-P; Singularity as Completed',
  clauses: [
    'allness = S_P_U',
    'induction = U_S_P',
    'singularity = completed',
    'singularity.posited = withOpposite',
    'oneExtreme = predicateCommonToAll',
    'otherExtreme = immediateGenus',
    'configuration = U_P_with_s_s_s',
  ],
  predicates: [
    { name: 'S_P_U', args: ['allness'] },
    { name: 'U_S_P', args: ['induction'] },
    { name: 'completed', args: ['singularity'] },
    { name: 'withOpposite', args: ['singularity.posited'] },
    { name: 'predicateCommonToAll', args: ['oneExtreme'] },
    { name: 'immediateGenus', args: ['otherExtreme'] },
    { name: 'U_P_with_s_s_s', args: ['configuration'] },
  ],
  relations: [],
  candidateSummary: 'Allness = S-P-U. Induction = U-S-P (singularity as completed, posited with opposite = universality). One extreme = predicate common to all singulars. Other = immediate genus (exhausted in collection). Configuration: U -- P with s, s, s, s ad infinitum.',
  provenance: {
    sourceChunk: 'syl-refl-8-induction-schema',
    sourceOp: 'syl-refl-op-8-induction-schema',
  },
};

export const sylReflOp9InductionExperience: LogicalOperation = {
  id: 'syl-refl-op-9-induction-experience',
  chunkId: 'syl-refl-9-induction-experience',
  label: 'Induction: Deficiency Eliminated; Syllogism of Experience',
  clauses: [
    'deficiency = eliminated',
    'subject = objectiveUniversal',
    'predicate = equalExtension',
    'syllogism = ofExperience',
    'experience = subjectiveGathering',
    'genus.determined = throughTotality',
    'innerConcept = notYetPosited',
  ],
  predicates: [
    { name: 'eliminated', args: ['deficiency'] },
    { name: 'objectiveUniversal', args: ['subject'] },
    { name: 'equalExtension', args: ['predicate'] },
    { name: 'ofExperience', args: ['syllogism'] },
    { name: 'subjectiveGathering', args: ['experience'] },
    { name: 'throughTotality', args: ['genus.determined'] },
    { name: 'notYetPosited', args: ['innerConcept'] },
  ],
  relations: [],
  candidateSummary: 'Second figure did not correspond. In induction: deficiency eliminated ("all singulars"). U-S: subject = objective universal/genus, predicate = equal extension. = syllogism of experience (subjective gathering, conjoining genus). Objective significance: genus determined through totality as universal property. But only inner concept, not yet posited.',
  provenance: {
    sourceChunk: 'syl-refl-9-induction-experience',
    sourceOp: 'syl-refl-op-9-induction-experience',
  },
};

export const sylReflOp10InductionSubjective: LogicalOperation = {
  id: 'syl-refl-op-10-induction-subjective',
  chunkId: 'syl-refl-10-induction-subjective',
  label: 'Induction: Subjective; Bad Infinity',
  clauses: [
    'induction = essentiallySubjective',
    'middle = singularsInImmediacy',
    'universality = completeness',
    'universality = task',
    'progression = intoBadInfinity',
    'intendedUnity = perpetualOught',
    'conclusion = problematic',
  ],
  predicates: [
    { name: 'essentiallySubjective', args: ['induction'] },
    { name: 'singularsInImmediacy', args: ['middle'] },
    { name: 'completeness', args: ['universality'] },
    { name: 'task', args: ['universality'] },
    { name: 'intoBadInfinity', args: ['progression'] },
    { name: 'perpetualOught', args: ['intendedUnity'] },
    { name: 'problematic', args: ['conclusion'] },
  ],
  relations: [],
  candidateSummary: 'Essentially still subjective. Middle = singulars in immediacy (collecting = external reflection). Universality only completeness (remains task). Progression into bad infinity recurs. Singularity ought identical with universality, but singulars immediate → intended unity = perpetual ought. Conclusion problematic.',
  provenance: {
    sourceChunk: 'syl-refl-10-induction-subjective',
    sourceOp: 'syl-refl-op-10-induction-subjective',
  },
};

export const sylReflOp11InductionPresupposes: LogicalOperation = {
  id: 'syl-refl-op-11-induction-presupposes',
  chunkId: 'syl-refl-11-induction-presupposes',
  label: 'Induction: Presupposes Genus; Truth is Analogy',
  clauses: [
    'induction.presupposes = genus',
    'induction.presupposes = conclusionAsImmediate',
    'experience.assumed = valid',
    'singularity.middle = ifImmediatelyIdenticalWithUniversality',
    'universality = externalButEssential',
    'truth = syllogismOfAnalogy',
  ],
  predicates: [
    { name: 'genus', args: ['induction.presupposes'] },
    { name: 'conclusionAsImmediate', args: ['induction.presupposes'] },
    { name: 'valid', args: ['experience.assumed'] },
    { name: 'ifImmediatelyIdenticalWithUniversality', args: ['singularity.middle'] },
    { name: 'externalButEssential', args: ['universality'] },
    { name: 'syllogismOfAnalogy', args: ['truth'] },
  ],
  relations: [],
  candidateSummary: 'Expresses perception ought to infinity → presupposes genus in and for itself conjoined. Presupposes conclusion as immediate. Experience assumed valid though perception not complete. Based on immediacy, but not supposed immediacy. Truth = syllogism with middle singularity immediately in itself universality = analogy.',
  provenance: {
    sourceChunk: 'syl-refl-11-induction-presupposes',
    sourceOp: 'syl-refl-op-11-induction-presupposes',
  },
};

export const sylReflOp12AnalogySchema: LogicalOperation = {
  id: 'syl-refl-op-12-analogy-schema',
  chunkId: 'syl-refl-12-analogy-schema',
  label: 'Analogy: S-U-P; Singular in Universal Nature',
  clauses: [
    'analogy = S_U_P',
    'middle = universality',
    'universality = immanentReflectionOfConcretedTerm',
    'singular = middle',
    'singular = inUniversalNature',
    'otherSingular.has = sameUniversalNature',
  ],
  predicates: [
    { name: 'S_U_P', args: ['analogy'] },
    { name: 'universality', args: ['middle'] },
    { name: 'immanentReflectionOfConcretedTerm', args: ['universality'] },
    { name: 'middle', args: ['singular'] },
    { name: 'inUniversalNature', args: ['singular'] },
    { name: 'sameUniversalNature', args: ['otherSingular.has'] },
  ],
  relations: [],
  candidateSummary: 'Third figure S-U-P. Middle no longer single quality but universality (immanent reflection of concreted term = its nature). Universality of concreted term = in itself this concreted term. Singular = middle (taken in universal nature). Another singular (extreme) has same universal nature. Example: earth has inhabitants, moon is earth, Therefore moon has inhabitants.',
  provenance: {
    sourceChunk: 'syl-refl-12-analogy-schema',
    sourceOp: 'syl-refl-op-12-analogy-schema',
  },
};

export const sylReflOp13AnalogySuperficial: LogicalOperation = {
  id: 'syl-refl-op-13-analogy-superficial',
  chunkId: 'syl-refl-13-analogy-superficial',
  label: 'Analogy: Superficiality; Form vs Content',
  clauses: [
    'superficiality.increases = withUniversality',
    'universality = mereQuality',
    'identity = similarity',
    'form = content',
    'analogy = formPeculiarlyItsOwn',
    'form.determines = itselfToContent',
    'advance = necessary',
  ],
  predicates: [
    { name: 'withUniversality', args: ['superficiality.increases'] },
    { name: 'mereQuality', args: ['universality'] },
    { name: 'similarity', args: ['identity'] },
    { name: 'content', args: ['form'] },
    { name: 'formPeculiarlyItsOwn', args: ['analogy'] },
    { name: 'itselfToContent', args: ['form.determines'] },
    { name: 'necessary', args: ['advance'] },
  ],
  relations: [],
  candidateSummary: 'More superficial the more universal = mere quality/distinctive mark (identity = similarity). Superficiality should have no place. Unacceptable: major as "similar in one mark similar in other" (form as content). What matters = form (not empirical content). Analogy = form peculiarly its own. Form determines itself to content = necessary advance.',
  provenance: {
    sourceChunk: 'syl-refl-13-analogy-superficial',
    sourceOp: 'syl-refl-op-13-analogy-superficial',
  },
};

export const sylReflOp14AnalogyQuaternio: LogicalOperation = {
  id: 'syl-refl-op-14-analogy-quaternio',
  chunkId: 'syl-refl-14-analogy-quaternio',
  label: 'Analogy: Quaternio Terminorum; Essential Universality',
  clauses: [
    'analogy.seems = quaternioTerminorum',
    'middle = singularity',
    'middle = trueUniversality',
    'analogy = essentialUniversality',
    'analogy = stillSyllogismOfReflection',
    'externality = stillThere',
    'predicate.not = alreadyPredicateOfOther',
  ],
  predicates: [
    { name: 'quaternioTerminorum', args: ['analogy.seems'] },
    { name: 'singularity', args: ['middle'] },
    { name: 'trueUniversality', args: ['middle'] },
    { name: 'essentialUniversality', args: ['analogy'] },
    { name: 'stillSyllogismOfReflection', args: ['analogy'] },
    { name: 'stillThere', args: ['externality'] },
    { name: 'alreadyPredicateOfOther', args: ['predicate.not'] },
  ],
  relations: [],
  candidateSummary: 'With major premise: may seem four terms (quaternio terminorum). Middle = singularity but immediately true universality. In analogy: essential universality. Still syllogism of reflection (united immediately). Externality still there (singular = genus only in itself/implicitly). Predicate not already predicate of other.',
  provenance: {
    sourceChunk: 'syl-refl-14-analogy-quaternio',
    sourceOp: 'syl-refl-op-14-analogy-quaternio',
  },
};

export const sylReflOp15AnalogySublation: LogicalOperation = {
  id: 'syl-refl-op-15-analogy-sublation',
  chunkId: 'syl-refl-15-analogy-sublation',
  label: 'Analogy: Demands Sublation; Passes to Necessity',
  clauses: [
    'analogy.entails = requirement',
    'requirement = demandToCounterImmediacy',
    'demand = toBeMediated',
    'demand = sublationOfSingularity',
    'remains = objectiveUniversal',
    'negation.noLonger = immediate',
    'negation = posited',
    'externalUniversality.determined = asExistingInAndForItself',
    'identity = higherUniversality',
  ],
  predicates: [
    { name: 'requirement', args: ['analogy.entails'] },
    { name: 'demandToCounterImmediacy', args: ['requirement'] },
    { name: 'toBeMediated', args: ['demand'] },
    { name: 'sublationOfSingularity', args: ['demand'] },
    { name: 'objectiveUniversal', args: ['remains'] },
    { name: 'immediate', args: ['negation.noLonger'] },
    { name: 'posited', args: ['negation'] },
    { name: 'asExistingInAndForItself', args: ['externalUniversality.determined'] },
    { name: 'higherUniversality', args: ['identity'] },
  ],
  relations: [],
  candidateSummary: 'S-P conclusion; one premise likewise S-P. Entails requirement premise also S-P. = demand to counter immediacy (presupposes conclusion). = demand to be mediated. Demands sublation of moment of singularity. Remains: objective universal (genus purified). Negation no longer immediate but posited. External universality determined as existing in and for itself. Identity becomes higher universality.',
  provenance: {
    sourceChunk: 'syl-refl-15-analogy-sublation',
    sourceOp: 'syl-refl-op-15-analogy-sublation',
  },
};

export const sylReflOp16ReviewNecessity: LogicalOperation = {
  id: 'syl-refl-op-16-review-necessity',
  chunkId: 'syl-refl-16-review-necessity',
  label: 'Review: Mediation as Concrete Unity; Passes to Necessity',
  clauses: [
    'mediation = concreteUnity',
    'reflection = positingOneInOther',
    'singularity = essentialGround',
    'singular.notUnited = positively',
    'singular = sublated',
    'universal = genusPosited',
    'immediacy = sublated',
    'passedOver = intoSyllogismOfNecessity',
  ],
  predicates: [
    { name: 'concreteUnity', args: ['mediation'] },
    { name: 'positingOneInOther', args: ['reflection'] },
    { name: 'essentialGround', args: ['singularity'] },
    { name: 'positively', args: ['singular.notUnited'] },
    { name: 'sublated', args: ['singular'] },
    { name: 'genusPosited', args: ['universal'] },
    { name: 'sublated', args: ['immediacy'] },
    { name: 'intoSyllogismOfNecessity', args: ['passedOver'] },
  ],
  relations: [],
  candidateSummary: 'Reviewing: mediation = posited/concrete unity of form determinations. Reflection = positing one in other (middle = allness). Singularity = essential ground. Singular not united positively but sublated (negative moment). Universal = genus posited. But immediacy sublated → S-U-P. Passed over into syllogism of necessity.',
  provenance: {
    sourceChunk: 'syl-refl-16-review-necessity',
    sourceOp: 'syl-refl-op-16-review-necessity',
  },
};

export const reflectionSyllogismOperations: LogicalOperation[] = [
  sylReflOp1IntroductionSublated,
  sylReflOp2ExtremesMiddle,
  sylReflOp3AllnessUnderstanding,
  sylReflOp4AllnessConcrete,
  sylReflOp5AllnessIllusion,
  sylReflOp6ExampleMortal,
  sylReflOp7ResultPosited,
  sylReflOp8InductionSchema,
  sylReflOp9InductionExperience,
  sylReflOp10InductionSubjective,
  sylReflOp11InductionPresupposes,
  sylReflOp12AnalogySchema,
  sylReflOp13AnalogySuperficial,
  sylReflOp14AnalogyQuaternio,
  sylReflOp15AnalogySublation,
  sylReflOp16ReviewNecessity,
];
