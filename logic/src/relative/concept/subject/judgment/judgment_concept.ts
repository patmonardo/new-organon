import type { Chunk, LogicalOperation } from '../syllogism/index';

/**
 * JUDGMENT OF THE CONCEPT — Complete Structure
 *
 * NOTE: The judgment of the concept is the judgment that first contains true
 * adjudication. The concept is at the basis of this judgment, and it is there
 * with reference to the subject matter, as an ought to which reality may or
 * may not conform. This judgment has been called the judgment of modality,
 * but in Kant's Logic, for "Science", modality becomes Certainty—this reflects
 * the Concept:Idea dyad. The Idea intrudes on the Concept here as the Judgment
 * of the Concept, just as the UUU Syllogism (the Mathematical notion) appears
 * in Being.
 *
 * Structure:
 * Introduction: Judgment of the Concept overview and modality
 * a. The assertoric judgment (immediate, subjective assurance)
 * b. The problematic judgment (contingency, subject split)
 * c. The apodictic judgment (certainty, copula replete, transition to syllogism)
 *
 * Transition: Judgment of the Concept becomes Syllogism (Idea)
 */

// ============================================================================
// INTRODUCTION: JUDGMENT OF THE CONCEPT — MODALITY TO CERTAINTY
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-con-intro-1-power-of-judgment',
    title: 'Judgment of the concept — true adjudication',
    text: `To know how to form judgments of existence, such as "the rose is red," "the snow is white," etc., hardly counts as a sign of great power of judgment. The judgments of reflection are more in the nature of propositions; to be sure, in the judgment of necessity the subject matter is present in its objective universality, but it is only in the judgment now to be considered that its connection with the concept is to be found.`,
  },
  {
    id: 'j-con-intro-2-concept-as-ought',
    title: 'Concept as ought — true adjudication',
    text: `The concept is at the basis of this judgment, and it is there with reference to the subject matter, as an ought to which reality may or may not conform. This is the judgment, therefore, that first contains true adjudication; the predicates, "good," "bad," "true," "right," etc., express that the fact is measured against the concept as an ought which is simply presupposed, and is, or is not, in agreement with it.`,
  },
  {
    id: 'j-con-intro-3-judgment-of-modality',
    title: 'Judgment of modality — problematic, assertoric, apodictic',
    text: `The judgment of the concept has been called the judgment of modality, and has been regarded as containing the form of the connection of subject and predicate as this obtains in an external understanding, and as concerned with the value of the copula only in connection with thought. Accordingly, judgment is said to be problematic when the affirmation or negation is taken as optional or possible; assertoric, when it is taken as true, that is, actual, and apodictic when it is taken as necessary.`,
  },
  {
    id: 'j-con-intro-4-subjectivity-not-external-reflection',
    title: 'Subjectivity not external reflection — concept as objective',
    text: `It is easy to see why it would be an easy step in this judgment to go outside the judgment itself and to regard its determination as something merely subjective. For it is the concept here, the subjective, that comes into play again in judgment and relates to an immediate actuality. But this subjectivity is not to be confused with external reflection, which is of course also something subjective but in a different sense than the concept itself; on the contrary, the concept that has again emerged out of the disjunctive judgment is the very opposite of a mere mode or manner.`,
  },
  {
    id: 'j-con-intro-5-judgment-objective-truth',
    title: 'Judgment of concept as objective truth',
    text: `The earlier judgments are subjective in this sense, for they rest on an abstract one-sidedness in which the concept is lost. But the judgment of the concept is instead objective and, as contrasted with the others, it is the truth, for it rests on the concept precisely in its determinateness as concept, not in some external reflection or with reference to some subjective, that is, accidental, thought.`,
  },
  {
    id: 'j-con-intro-6-disjunctive-result',
    title: 'Disjunctive judgment result — concretion must develop into totality',
    text: `In the disjunctive judgment, the concept was posited as the identity of universal nature and its particularization, and with that the relation of the judgment was sublated. This concretion of universality and particularization is at first a simple result; it must now further develop itself into totality, for its moments have at first collapsed into it and do not as yet stand over against each other in determinate self-subsistence.`,
  },
  {
    id: 'j-con-intro-7-singularity-moment',
    title: 'Singularity as third moment — negative unity',
    text: `The shortcoming of that result may also be stated more incisively by saying that although in the disjunctive judgment the objective universality has attained completion in its particularization, the negative unity of the latter has only retreated into it and has not as yet determined itself as the third moment, that of singularity. But to the extent that the result is itself negative unity, it is already this singularity; it is then this one determinateness alone that must now posit its negativity, that must part itself into extremes and in this way concludes its development in the syllogistic conclusion.`,
  },
  {
    id: 'j-con-intro-8-proximate-diremption',
    title: 'Proximate diremption — unity posited as subject and predicate',
    text: `The proximate diremption of this unity is the judgment in which the unity is posited first as subject, as an immediate singular, and then as predicate, as the determinate connection of its moments.`,
  },
];

// ============================================================================
// a. THE ASSERTORIC JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-con-1-assertoric-immediate',
    title: 'Assertoric judgment — immediate form',
    text: `The judgment of the concept is at first immediate; as such, it is the assertoric judgment. The subject is a concrete singular in general, and the predicate expresses this same singular as the connection of its actuality, its determinateness or constitution, to its concept. ("This house is bad," "this action is good.")`,
  },
  {
    id: 'j-con-2-assertoric-ought',
    title: 'Assertoric judgment — the "ought" (universal nature)',
    text: `More closely considered, it contains, therefore, (a) that the subject ought to be something; its universal nature has posited itself as the self-subsistent concept.`,
  },
  {
    id: 'j-con-3-assertoric-particularity',
    title: 'Assertoric judgment — particularity and constitution',
    text: `More closely considered, it contains, therefore, (b) that particularity is something constituted or an external concrete existence, not only because of its immediacy, but because it expressly differs from its self-subsisting universal nature; its external concrete existence, for its part, because of this self-subsistence of the concept, is also indifferent with respect to the universal and may or may not conform to it.`,
  },
  {
    id: 'j-con-4-assertoric-constitution-singularity',
    title: 'Constitution as singularity — disjunctive link',
    text: `This constitution is the singularity which in the disjunctive judgment escapes the necessary determination of the universal, a determination that exists only as the particularization of the species and as the negative principle of the genus. Thus the concrete universality that has come out of the disjunctive judgment divides in the assertoric judgment into the form of extremes to which the concept itself, as the posited unity connecting them, is still lacking.`,
  },
  {
    id: 'j-con-5-assertoric-subjective-assurance',
    title: 'Assertoric judgment — subjective assurance',
    text: `For this reason the judgment is so far only assertoric; its credential is only a subjective assurance. That something is good or bad, right, suitable or not, hangs on an external third. But to say that the connectedness is thus externally posited is the same as saying that it is still only in itself or internal.`,
  },
  {
    id: 'j-con-6-assertoric-not-merely-subjective',
    title: 'Not merely subjective consciousness — predicates of object',
    text: `When we say that something is good or bad, etc., we certainly do not mean to say that it is good only in a subjective consciousness but may perhaps be bad in itself, or that "good and bad," "right," "suitable," etc. may not be predicates of the object itself. The merely subjective character of the assertion of this judgment consists, therefore, in the fact that the implicitly present connectedness of subject and predicate has not been posited yet, or, what amounts to the same thing, that it is only external; the copula still is an immediate abstract being.`,
  },
  {
    id: 'j-con-7-assertoric-conflict',
    title: 'Assertoric judgment — conflict and problematic status',
    text: `Thus the assurance of the assertoric judgment can with right be confronted by an opposing one. When the assurance is given that "this action is good," the opposite, "this action is bad," has equal justification. Or, considering the judgment in itself, since its subject is an immediate singular, in this abstraction it still does not have, posited in it, the determinateness that would contain its connection with the universal concept; it still is a contingent matter, therefore, whether there is or there is not conformity to the concept. Essentially, therefore, the judgment is problematic.`,
  },
);

// ============================================================================
// b. THE PROBLEMATIC JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-con-8-problematic-assertoric-both',
    title: 'Problematic judgment — assertoric taken positively and negatively',
    text: `The problematic judgment is the assertoric judgment in so far as the latter must be taken positively as well as negatively. According to this qualitative side, the particular judgment is likewise a problematic one, for it has positive just as much as negative value (equally problematic is also the being of the subject and predicate in the hypothetical judgment), and also posited through this side is that the singular judgment and the categorical are still something merely subjective.`,
  },
  {
    id: 'j-con-9-problematic-immanent',
    title: 'Problematic judgment — more immanent positing',
    text: `In the problematic judgment as such, however, this positing is more immanent than it is in these others, for in it the content of the predicate is the connection of the subject to the concept; here, therefore, the determination of the immediate as something contingent is itself present.`,
  },
  {
    id: 'j-con-10-problematic-copula-indeterminacy',
    title: 'Indeterminateness falls on copula — predicate already objective',
    text: `Whether the predicate ought to be or not to be coupled with a certain subject appears at first only as problematic, and to this extent the indeterminateness falls on the side of the copula. The predicate has no determination to gain from this coupling, since it is already the objective, concrete universality. The problematic element falls, therefore, on the immediacy of the subject, which is thereby determined as a contingency.`,
  },
  {
    id: 'j-con-11-problematic-singularity-essential',
    title: 'Singularity essential — not to be abstracted',
    text: `But further, we must not for that reason abstract from the singularity of the subject; purified of such a singularity, the subject would be only a universal, whereas the predicate entails precisely this, that the concept of the subject ought to be posited with reference to its singularity. We may not say, "the house or a house is good," but, "so indeed it is in the way it is made."`,
  },
  {
    id: 'j-con-12-problematic-subject-contingency',
    title: 'Problematic element in subject — moment of contingency',
    text: `The problematic element in the subject itself constitutes its moment of contingency, the subjectivity of the fact it expresses as contrasted with its objective nature or its concept, its mere mode and manner or its constitution.`,
  },
  {
    id: 'j-con-13-problematic-subject-differentiated',
    title: 'Subject differentiated into universality and constitution',
    text: `Consequently the subject is itself differentiated into its universality or objective nature, that is, its ought, and the particularized constitution of immediate existence. It thereby contains the ground for being or not being what it ought to be. In this way, it is equated with the predicate.`,
  },
  {
    id: 'j-con-14-problematic-negativity-partition',
    title: 'Negativity of problematic character — original partition',
    text: `Accordingly, the negativity of the problematic character of the judgment, inasmuch as it implicates the immediacy of the subject, only amounts to this original partition of the latter into its moments of universal and particular of which it is already the unity, a partition which is the judgment itself.`,
  },
  {
    id: 'j-con-15-problematic-subjectivity-duplicity',
    title: 'Duplicity of subjectivity — concept vs constitution',
    text: `One more comment that can be made is that both sides of the subject, its concept and the way it is constituted, could each be called its subjectivity. The concept is the universal essence of a fact, withdrawn into itself, the fact's negative self-unity; this unity constitutes the fact's subjectivity. But a fact is also essentially contingent and has an external constitution; this last may also be called its mere subjectivity, as contrasted with the objectivity of the concept.`,
  },
  {
    id: 'j-con-16-problematic-fact-duplicity',
    title: 'Fact as duplicity — concept negates universality',
    text: `The fact consists just in this, that its concept, as self-negating unity, negates its universality and projects itself into the externality of singularity. As this duplicity, the subject of the judgment is here posited; the truth of those two opposite meanings of subjectivity is that they are in one.`,
  },
  {
    id: 'j-con-17-problematic-subjective-meaning',
    title: 'Subjective meaning becomes problematic',
    text: `The meaning of subjective has itself become problematic by having lost the immediate determinateness that it had in the immediate judgment and its determinate opposition to the predicate. These opposite meanings of subjectivity that surface even in the ratiocination of ordinary reflection should by themselves at least call attention to the fact that subjectivity has no truth in one of them alone. The duplicity of meaning is the manifestation of the one-sidedness of each when taken by itself.`,
  },
  {
    id: 'j-con-18-problematic-to-apodictic',
    title: 'Transition: problematic to apodictic',
    text: `When this problematic character of the judgment is thus posited as the character of the fact, the fact with its constitution, the judgment itself is no longer problematic but apodictic.`,
  },
);

// ============================================================================
// c. THE APODICTIC JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-con-19-apodictic-subject-structure',
    title: 'Apodictic judgment — subject includes universal and constitution',
    text: `The subject of the apodictic judgment ("the house, as so and so constituted, is good," "the action, as so and so constituted, is right") includes, first, the universal, or what it ought to be; second, its constitution; the latter contains the ground why a predicate of the judgment of the concept does or does not pertain to it, that is, whether the subject corresponds to its concept or not.`,
  },
  {
    id: 'j-con-20-apodictic-truly-objective',
    title: 'Apodictic judgment — truly objective, truth of judgment',
    text: `This judgment is now truly objective; or it is the truth of the judgment in general. Subject and predicate correspond to each other, and have the same concept, and this content is itself posited concrete universality; that is to say, it contains the two moments, the objective universal or the genus and the singularized universal.`,
  },
  {
    id: 'j-con-21-apodictic-universal-through-opposite',
    title: 'Universal through its opposite — correspondence',
    text: `Here we have, therefore, the universal that is itself and continues through its opposite, and is a universal only in unity with the latter. Such a universal, like "good," "fitting," "right," etc., has an ought for its ground, and contains at the same time the correspondence of existence; it is not the ought or the genus by itself, but this correspondence which is the universality that constitutes the predicate of the apodictic judgment.`,
  },
  {
    id: 'j-con-22-apodictic-subject-fact',
    title: 'Subject as fact — contains both moments',
    text: `The subject likewise contains these two moments in immediate unity as fact. The truth of the latter, however, is that it is internally fractured into its ought and its being; this is the absolute judgment on all actuality.`,
  },
  {
    id: 'j-con-23-apodictic-absolute-judgment',
    title: 'Absolute judgment — omnipotence of concept',
    text: `That this original partition, which is the omnipotence of the concept, is equally a turning back into the concept's unity and the absolute connection of "ought" and "being" to each other, is what makes the actual into a fact; the fact's inner connection, this concrete identity, constitutes its soul.`,
  },
  {
    id: 'j-con-24-apodictic-copula-ground',
    title: 'Copula determinateness — ground in constitution',
    text: `The transition from the immediate simplicity of the fact to the correspondence which is the determinate connection of its ought and its being, the copula, now shows itself upon closer examination to lie in the particular determinateness of the fact. The genus is the universal existing in and for itself which, to that extent, appears as unconnected; the determinateness, however, is that which in that universality is reflected into itself but at the same time into an other.`,
  },
  {
    id: 'j-con-25-apodictic-ground-constitution',
    title: 'Judgment has ground in constitution — apodictic',
    text: `The judgment, therefore, has its ground in the constitution of the subject and is thereby apodictic. Consequently, we now have the determinate and accomplished copula which hitherto consisted in the abstract "is" but has now further developed into ground in general.`,
  },
  {
    id: 'j-con-26-apodictic-copula-determinateness',
    title: 'Copula as determinate connection',
    text: `It first attaches to the subject as immediate determinateness, but it is equally the connection to the predicate, a predicate that has no other content than this correspondence itself, or the connection of the subject to the universality.`,
  },
  {
    id: 'j-con-27-apodictic-form-passes-away',
    title: 'Form of judgment passes away — three reasons',
    text: `Thus the form of judgment has passed away, first, because subject and predicate are in themselves the same content; but, second, because through its determinateness the subject points beyond itself and connects itself to the predicate; but again, third, this connecting has equally passed over into the predicate, only constitutes the content of it, and so it is the connecting as posited or the judgment itself.`,
  },
  {
    id: 'j-con-28-apodictic-concrete-identity-recovered',
    title: 'Concrete identity recovered — inner foundation',
    text: `The concrete identity of the concept that was the result of the disjunctive judgment and constitutes the inner foundation of the judgment of the concept (the identity that was posited at first only in the predicate) is thus recovered in the whole.`,
  },
  {
    id: 'j-con-29-apodictic-subject-predicate-whole-concept',
    title: 'Subject and predicate each whole concept',
    text: `On closer examination, the positive factor in this result which is responsible for the transition of the judgment into another form is that, as we have just seen, the subject and predicate are in the apodictic judgment each the whole concept.`,
  },
  {
    id: 'j-con-30-apodictic-copula-unity',
    title: 'Copula as unity of concept — distinct yet connecting',
    text: `The unity of the concept, as the determinateness constituting the copula that connects them, is at the same time distinct from them. At first, it stands only on the other side of the subject as the latter's immediate constitution. But since its essence is to connect, it is not only that immediate constitution but the universal that runs through the subject and predicate.`,
  },
  {
    id: 'j-con-31-apodictic-form-connection',
    title: 'Form of connection posited through copula determinateness',
    text: `While subject and predicate have the same content, it is the form of their connection that is instead posited through the determinateness of the copula, the determinateness as a universal or the particularity.`,
  },
  {
    id: 'j-con-32-apodictic-copula-replete',
    title: 'Copula replete — contains form determinations',
    text: `Thus it contains in itself both the form determinations of the extremes and is the determinate connection of the subject and predicate: the accomplished copula of the judgment, the copula replete of content, the unity of the concept that re-emerges from the judgment wherein it was lost in the extremes.`,
  },
  {
    id: 'j-con-33-apodictic-to-syllogism',
    title: 'Transition: apodictic judgment becomes syllogism',
    text: `By virtue of this repletion of the copula, the judgment has become syllogism.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // INTRODUCTION OPERATIONS
  // ============================================================================
  {
    id: 'j-con-op-intro-1-power-judgment',
    chunkId: 'j-con-intro-1-power-of-judgment',
    label: 'Declare judgment of concept as true adjudication',
    clauses: [
      'judgmentsOfExistence.not = greatPowerOfJudgment',
      'judgmentsOfReflection = morePropositions',
      'judgmentOfNecessity.has = objectiveUniversality',
      'judgmentOfConcept.has = connectionWithConcept',
    ],
    predicates: [{ name: 'IsTrueAdjudication', args: ['judgmentOfConcept'] }],
    relations: [
      { predicate: 'contains', from: 'judgmentOfConcept', to: 'trueAdjudication' },
    ],
  },
  {
    id: 'j-con-op-intro-2-concept-ought',
    chunkId: 'j-con-intro-2-concept-as-ought',
    label: 'Concept as ought — reality may or may not conform',
    clauses: [
      'concept.atBasis = judgmentOfConcept',
      'concept.referenceTo = subjectMatter',
      'concept = ought',
      'reality.mayConform = toOught',
      'reality.mayNotConform = toOught',
      'predicates = {good, bad, true, right}',
      'predicates.express = factMeasuredAgainstConcept',
      'fact.agreement = withOught (orNot)',
    ],
    predicates: [{ name: 'ConceptAsOught', args: ['concept'] }],
    relations: [
      { predicate: 'measures', from: 'concept', to: 'fact' },
    ],
  },
  {
    id: 'j-con-op-intro-3-modality',
    chunkId: 'j-con-intro-3-judgment-of-modality',
    label: 'Judgment of modality — problematic, assertoric, apodictic',
    clauses: [
      'judgmentOfConcept.called = judgmentOfModality',
      'modality.contains = formOfConnection',
      'modality.concernedWith = valueOfCopula',
      'problematic = affirmationNegationOptionalPossible',
      'assertoric = takenAsTrueActual',
      'apodictic = takenAsNecessary',
    ],
    predicates: [{ name: 'IsModalityJudgment', args: ['judgmentOfConcept'] }],
    relations: [
      { predicate: 'contains', from: 'modality', to: 'formOfConnection' },
    ],
  },
  {
    id: 'j-con-op-intro-4-subjectivity-not-external',
    chunkId: 'j-con-intro-4-subjectivity-not-external-reflection',
    label: 'Subjectivity not external reflection — concept objective',
    clauses: [
      'concept.comesIntoPlay = inJudgment',
      'concept.relatesTo = immediateActuality',
      'subjectivity.not = externalReflection',
      'concept.emergedFrom = disjunctiveJudgment',
      'concept.not = mereModeOrManner',
    ],
    predicates: [{ name: 'IsObjectiveConcept', args: ['concept'] }],
    relations: [
      { predicate: 'relates', from: 'concept', to: 'immediateActuality' },
    ],
  },
  {
    id: 'j-con-op-intro-5-objective-truth',
    chunkId: 'j-con-intro-5-judgment-objective-truth',
    label: 'Judgment of concept as objective truth',
    clauses: [
      'earlierJudgments.subjective = abstractOneSidedness',
      'earlierJudgments.concept = lost',
      'judgmentOfConcept.objective = true',
      'judgmentOfConcept.truth = true',
      'judgmentOfConcept.restsOn = conceptInDeterminateness',
      'judgmentOfConcept.not = externalReflection',
      'judgmentOfConcept.not = accidentalThought',
    ],
    predicates: [{ name: 'IsObjectiveTruth', args: ['judgmentOfConcept'] }],
    relations: [
      { predicate: 'restsOn', from: 'judgmentOfConcept', to: 'concept' },
    ],
  },
  {
    id: 'j-con-op-intro-6-disjunctive-result',
    chunkId: 'j-con-intro-6-disjunctive-result',
    label: 'Disjunctive result — concretion must develop into totality',
    clauses: [
      'disjunctiveJudgment.concept = identityOfUniversalNatureAndParticularization',
      'disjunctiveJudgment.relation = sublated',
      'concretion = universalityAndParticularization',
      'concretion.atFirst = simpleResult',
      'concretion.mustDevelop = intoTotality',
      'moments.collapsed = intoResult',
      'moments.notStand = overAgainstEachOther',
    ],
    predicates: [{ name: 'MustDevelopIntoTotality', args: ['concretion'] }],
    relations: [
      { predicate: 'develops', from: 'concretion', to: 'totality' },
    ],
  },
  {
    id: 'j-con-op-intro-7-singularity-moment',
    chunkId: 'j-con-intro-7-singularity-moment',
    label: 'Singularity as third moment — negative unity',
    clauses: [
      'objectiveUniversality.attained = completionInParticularization',
      'negativeUnity.retreated = intoUniversality',
      'negativeUnity.notDetermined = asThirdMoment',
      'thirdMoment = singularity',
      'result.is = negativeUnity',
      'result.is = singularity',
      'singularity.mustPosit = itsNegativity',
      'singularity.mustPart = intoExtremes',
      'development.concludes = inSyllogisticConclusion',
    ],
    predicates: [{ name: 'IsSingularityMoment', args: ['negativeUnity'] }],
    relations: [
      { predicate: 'determines', from: 'negativeUnity', to: 'singularity' },
    ],
  },
  {
    id: 'j-con-op-intro-8-proximate-diremption',
    chunkId: 'j-con-intro-8-proximate-diremption',
    label: 'Proximate diremption — unity as subject and predicate',
    clauses: [
      'proximateDiremption = judgment',
      'unity.positedAs = subject',
      'subject = immediateSingular',
      'unity.positedAs = predicate',
      'predicate = determinateConnectionOfMoments',
    ],
    predicates: [{ name: 'IsProximateDiremption', args: ['judgment'] }],
    relations: [
      { predicate: 'posits', from: 'unity', to: 'subjectPredicate' },
    ],
  },
  // ============================================================================
  // ASSERTORIC JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-con-op-1-assertoric-immediate',
    chunkId: 'j-con-1-assertoric-immediate',
    label: 'Assertoric judgment — immediate form',
    clauses: [
      'judgmentOfConcept.atFirst = immediate',
      'judgmentOfConcept = assertoricJudgment',
      'subject = concreteSingular',
      'predicate.expresses = connectionOfActualityToConcept',
      'predicate.expresses = {determinateness, constitution}',
      'examples = {"This house is bad", "this action is good"}',
    ],
    predicates: [{ name: 'IsAssertoricJudgment', args: [] }],
    relations: [
      { predicate: 'expresses', from: 'predicate', to: 'connectionToConcept' },
    ],
  },
  {
    id: 'j-con-op-2-assertoric-ought',
    chunkId: 'j-con-2-assertoric-ought',
    label: 'Subject ought to be — universal nature as self-subsistent concept',
    clauses: [
      'subject.oughtToBe = something',
      'universalNature.positedAs = selfSubsistentConcept',
    ],
    predicates: [{ name: 'HasOught', args: ['subject'] }],
    relations: [
      { predicate: 'posits', from: 'universalNature', to: 'selfSubsistentConcept' },
    ],
  },
  {
    id: 'j-con-op-3-assertoric-particularity',
    chunkId: 'j-con-3-assertoric-particularity',
    label: 'Particularity as constituted — may or may not conform',
    clauses: [
      'particularity = constituted',
      'particularity = externalConcreteExistence',
      'particularity.differs = fromSelfSubsistingUniversalNature',
      'externalConcreteExistence.indifferent = toUniversal',
      'externalConcreteExistence.mayConform = toUniversal',
      'externalConcreteExistence.mayNotConform = toUniversal',
    ],
    predicates: [{ name: 'IsConstitutedParticularity', args: ['particularity'] }],
    relations: [
      { predicate: 'mayConform', from: 'particularity', to: 'universal' },
    ],
  },
  {
    id: 'j-con-op-4-assertoric-constitution-singularity',
    chunkId: 'j-con-4-assertoric-constitution-singularity',
    label: 'Constitution as singularity — disjunctive link',
    clauses: [
      'constitution = singularity',
      'singularity.escapes = necessaryDeterminationOfUniversal',
      'determination.existsAs = particularizationOfSpecies',
      'determination.existsAs = negativePrincipleOfGenus',
      'concreteUniversality.divides = intoFormOfExtremes',
      'concept.unityConnecting = stillLacking',
    ],
    predicates: [{ name: 'IsSingularity', args: ['constitution'] }],
    relations: [
      { predicate: 'escapes', from: 'singularity', to: 'necessaryDetermination' },
    ],
  },
  {
    id: 'j-con-op-5-assertoric-subjective-assurance',
    chunkId: 'j-con-5-assertoric-subjective-assurance',
    label: 'Assertoric judgment — subjective assurance',
    clauses: [
      'judgment.only = assertoric',
      'credential = subjectiveAssurance',
      'goodBadRightSuitable.hangsOn = externalThird',
      'connectedness.externallyPosited = stillOnlyInItself',
      'connectedness = internal',
    ],
    predicates: [{ name: 'HasSubjectiveAssurance', args: ['assertoricJudgment'] }],
    relations: [
      { predicate: 'hangsOn', from: 'judgment', to: 'externalThird' },
    ],
  },
  {
    id: 'j-con-op-6-assertoric-not-merely-subjective',
    chunkId: 'j-con-6-assertoric-not-merely-subjective',
    label: 'Not merely subjective consciousness — predicates of object',
    clauses: [
      'goodBad.not = onlyInSubjectiveConsciousness',
      'goodBad.mayBe = badInItself',
      'predicates.are = predicatesOfObject',
      'subjectiveCharacter = connectednessNotPosited',
      'subjectiveCharacter = connectednessOnlyExternal',
      'copula.still = immediateAbstractBeing',
    ],
    predicates: [{ name: 'NotMerelySubjective', args: ['assertoricJudgment'] }],
    relations: [
      { predicate: 'are', from: 'predicates', to: 'objectPredicates' },
    ],
  },
  {
    id: 'j-con-op-7-assertoric-conflict',
    chunkId: 'j-con-7-assertoric-conflict',
    label: 'Assertoric judgment — conflict and problematic status',
    clauses: [
      'assertoricAssurance.confronted = byOpposing',
      'example: "this action is good" vs "this action is bad"',
      'opposite.hasEqualJustification = true',
      'subject.immediateSingular = true',
      'subject.notHas = determinatenessContainingConnection',
      'conformity = contingentMatter',
      'judgment.essentially = problematic',
    ],
    predicates: [{ name: 'IsProblematic', args: ['assertoricJudgment'] }],
    relations: [
      { predicate: 'confronts', from: 'assertoricAssurance', to: 'opposing' },
    ],
  },
  // ============================================================================
  // PROBLEMATIC JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-con-op-8-problematic-assertoric-both',
    chunkId: 'j-con-8-problematic-assertoric-both',
    label: 'Problematic judgment — assertoric taken positively and negatively',
    clauses: [
      'problematicJudgment = assertoricTakenPositivelyAndNegatively',
      'particularJudgment.likewise = problematic',
      'particularJudgment.has = positiveAndNegativeValue',
      'hypotheticalJudgment.being = problematic',
      'singularJudgment.still = merelySubjective',
      'categoricalJudgment.still = merelySubjective',
    ],
    predicates: [{ name: 'IsProblematicJudgment', args: [] }],
    relations: [
      { predicate: 'taken', from: 'problematicJudgment', to: 'bothPolarities' },
    ],
  },
  {
    id: 'j-con-op-9-problematic-immanent',
    chunkId: 'j-con-9-problematic-immanent',
    label: 'Problematic judgment — more immanent positing',
    clauses: [
      'problematicJudgment.positing = moreImmanent',
      'predicate.content = connectionOfSubjectToConcept',
      'immediate.determinedAs = contingent',
      'contingency.present = true',
    ],
    predicates: [{ name: 'IsImmanentPositing', args: ['problematicJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'problematicJudgment', to: 'contingency' },
    ],
  },
  {
    id: 'j-con-op-10-problematic-copula-indeterminacy',
    chunkId: 'j-con-10-problematic-copula-indeterminacy',
    label: 'Indeterminateness falls on copula — predicate already objective',
    clauses: [
      'predicateOughtToBeCoupled = problematic',
      'indeterminateness.fallsOn = copula',
      'predicate.already = objectiveConcreteUniversality',
      'problematicElement.fallsOn = immediacyOfSubject',
      'subject.determinedAs = contingency',
    ],
    predicates: [{ name: 'CopulaIndeterminate', args: ['problematicJudgment'] }],
    relations: [
      { predicate: 'fallsOn', from: 'indeterminateness', to: 'copula' },
    ],
  },
  {
    id: 'j-con-op-11-problematic-singularity-essential',
    chunkId: 'j-con-11-problematic-singularity-essential',
    label: 'Singularity essential — not to be abstracted',
    clauses: [
      'singularity.notToBeAbstracted = true',
      'if singularityPurified = then subjectOnlyUniversal',
      'predicate.entails = conceptPositedWithReferenceToSingularity',
      'notSay = "house or a house is good"',
      'say = "so indeed it is in the way it is made"',
    ],
    predicates: [{ name: 'SingularityEssential', args: ['problematicJudgment'] }],
    relations: [
      { predicate: 'entails', from: 'predicate', to: 'singularity' },
    ],
  },
  {
    id: 'j-con-op-12-problematic-subject-contingency',
    chunkId: 'j-con-12-problematic-subject-contingency',
    label: 'Problematic element in subject — moment of contingency',
    clauses: [
      'problematicElement.inSubject = momentOfContingency',
      'subjectivity = factExpressed',
      'subjectivity.contrastedWith = objectiveNature',
      'subjectivity.contrastedWith = concept',
      'subjectivity = mereModeAndManner',
      'subjectivity = constitution',
    ],
    predicates: [{ name: 'HasMomentOfContingency', args: ['subject'] }],
    relations: [
      { predicate: 'contains', from: 'subject', to: 'contingency' },
    ],
  },
  {
    id: 'j-con-op-13-problematic-subject-differentiated',
    chunkId: 'j-con-13-problematic-subject-differentiated',
    label: 'Subject differentiated into universality and constitution',
    clauses: [
      'subject.differentiated = {universality, constitution}',
      'universality = objectiveNature',
      'universality = ought',
      'constitution = particularizedConstitutionOfImmediateExistence',
      'subject.contains = groundForBeingOrNotBeing',
      'subject.equatedWith = predicate',
    ],
    predicates: [{ name: 'SubjectDifferentiated', args: ['problematicJudgment'] }],
    relations: [
      { predicate: 'differentiates', from: 'subject', to: 'universalityConstitution' },
    ],
  },
  {
    id: 'j-con-op-14-problematic-negativity-partition',
    chunkId: 'j-con-14-problematic-negativity-partition',
    label: 'Negativity of problematic character — original partition',
    clauses: [
      'negativity = problematicCharacter',
      'negativity.implicates = immediacyOfSubject',
      'negativity.amountsTo = originalPartition',
      'partition = momentsOfUniversalAndParticular',
      'subject.already = unity',
      'partition = judgmentItself',
    ],
    predicates: [{ name: 'IsOriginalPartition', args: ['negativity'] }],
    relations: [
      { predicate: 'partitions', from: 'negativity', to: 'subject' },
    ],
  },
  {
    id: 'j-con-op-15-problematic-subjectivity-duplicity',
    chunkId: 'j-con-15-problematic-subjectivity-duplicity',
    label: 'Duplicity of subjectivity — concept vs constitution',
    clauses: [
      'bothSidesOfSubject = subjectivity',
      'concept = universalEssenceOfFact',
      'concept = withdrawnIntoItself',
      'concept = negativeSelfUnity',
      'unity = factSubjectivity',
      'fact.essentiallyContingent = true',
      'fact.has = externalConstitution',
      'externalConstitution = mereSubjectivity',
      'mereSubjectivity.contrastedWith = objectivityOfConcept',
    ],
    predicates: [{ name: 'HasDuplicityOfSubjectivity', args: ['subject'] }],
    relations: [
      { predicate: 'contrasts', from: 'conceptSubjectivity', to: 'constitutionSubjectivity' },
    ],
  },
  {
    id: 'j-con-op-16-problematic-fact-duplicity',
    chunkId: 'j-con-16-problematic-fact-duplicity',
    label: 'Fact as duplicity — concept negates universality',
    clauses: [
      'fact.consistsIn = conceptAsSelfNegatingUnity',
      'concept.negates = itsUniversality',
      'concept.projects = intoExternalityOfSingularity',
      'subject.positedAs = duplicity',
      'truth = twoOppositeMeaningsInOne',
    ],
    predicates: [{ name: 'IsDuplicity', args: ['fact'] }],
    relations: [
      { predicate: 'negates', from: 'concept', to: 'universality' },
    ],
  },
  {
    id: 'j-con-op-17-problematic-subjective-meaning',
    chunkId: 'j-con-17-problematic-subjective-meaning',
    label: 'Subjective meaning becomes problematic',
    clauses: [
      'subjectiveMeaning.become = problematic',
      'subjectiveMeaning.lost = immediateDeterminateness',
      'immediateDeterminateness.had = inImmediateJudgment',
      'immediateDeterminateness.had = determinateOppositionToPredicate',
      'oppositeMeanings.surface = inOrdinaryReflection',
      'subjectivity.hasNoTruth = inOneAlone',
      'duplicityOfMeaning = manifestationOfOneSidedness',
    ],
    predicates: [{ name: 'SubjectiveMeaningProblematic', args: [] }],
    relations: [
      { predicate: 'becomes', from: 'subjectiveMeaning', to: 'problematic' },
    ],
  },
  {
    id: 'j-con-op-18-problematic-to-apodictic',
    chunkId: 'j-con-18-problematic-to-apodictic',
    label: 'Transition: problematic to apodictic',
    clauses: [
      'when problematicCharacter.positedAs = characterOfFact',
      'fact.withConstitution = true',
      'judgment.noLonger = problematic',
      'judgment.becomes = apodictic',
    ],
    predicates: [{ name: 'TransitionsToApodictic', args: ['problematicJudgment'] }],
    relations: [
      { predicate: 'transitions', from: 'problematicJudgment', to: 'apodicticJudgment' },
    ],
  },
  // ============================================================================
  // APODICTIC JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-con-op-19-apodictic-subject-structure',
    chunkId: 'j-con-19-apodictic-subject-structure',
    label: 'Apodictic judgment — subject includes universal and constitution',
    clauses: [
      'apodicticSubject.includes = {universal, constitution}',
      'universal = whatItOughtToBe',
      'constitution.contains = ground',
      'ground.explains = predicatePertainsOrNot',
      'ground.explains = subjectCorrespondsToConcept',
    ],
    predicates: [{ name: 'IsApodicticJudgment', args: [] }],
    relations: [
      { predicate: 'includes', from: 'apodicticSubject', to: 'universalConstitution' },
    ],
  },
  {
    id: 'j-con-op-20-apodictic-truly-objective',
    chunkId: 'j-con-20-apodictic-truly-objective',
    label: 'Apodictic judgment — truly objective, truth of judgment',
    clauses: [
      'apodicticJudgment.trulyObjective = true',
      'apodicticJudgment = truthOfJudgmentInGeneral',
      'subjectAndPredicate.correspond = toEachOther',
      'subjectAndPredicate.have = sameConcept',
      'content = positedConcreteUniversality',
      'content.contains = {objectiveUniversal, singularizedUniversal}',
      'objectiveUniversal = genus',
    ],
    predicates: [{ name: 'IsTrulyObjective', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'corresponds', from: 'subject', to: 'predicate' },
    ],
  },
  {
    id: 'j-con-op-21-apodictic-universal-through-opposite',
    chunkId: 'j-con-21-apodictic-universal-through-opposite',
    label: 'Universal through its opposite — correspondence',
    clauses: [
      'universal.isItself = true',
      'universal.continuesThrough = itsOpposite',
      'universal.isUniversal = onlyInUnityWithOpposite',
      'examples = {good, fitting, right}',
      'universal.hasOught = forGround',
      'universal.contains = correspondenceOfExistence',
      'universality.not = oughtOrGenusByItself',
      'universality = correspondence',
      'correspondence = predicateOfApodicticJudgment',
    ],
    predicates: [{ name: 'UniversalThroughOpposite', args: ['universal'] }],
    relations: [
      { predicate: 'continues', from: 'universal', to: 'opposite' },
    ],
  },
  {
    id: 'j-con-op-22-apodictic-subject-fact',
    chunkId: 'j-con-22-apodictic-subject-fact',
    label: 'Subject as fact — contains both moments',
    clauses: [
      'subject.contains = bothMoments',
      'moments.inImmediateUnity = asFact',
      'fact.truth = internallyFractured',
      'fracture = {ought, being}',
      'fracture = absoluteJudgmentOnAllActuality',
    ],
    predicates: [{ name: 'IsFact', args: ['subject'] }],
    relations: [
      { predicate: 'contains', from: 'subject', to: 'bothMoments' },
    ],
  },
  {
    id: 'j-con-op-23-apodictic-absolute-judgment',
    chunkId: 'j-con-23-apodictic-absolute-judgment',
    label: 'Absolute judgment — omnipotence of concept',
    clauses: [
      'originalPartition = omnipotenceOfConcept',
      'partition = turningBackIntoConceptUnity',
      'partition = absoluteConnectionOfOughtAndBeing',
      'connection.makes = actualIntoFact',
      'fact.innerConnection = concreteIdentity',
      'concreteIdentity = factSoul',
    ],
    predicates: [{ name: 'IsAbsoluteJudgment', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'makes', from: 'connection', to: 'fact' },
    ],
  },
  {
    id: 'j-con-op-24-apodictic-copula-ground',
    chunkId: 'j-con-24-apodictic-copula-ground',
    label: 'Copula determinateness — ground in constitution',
    clauses: [
      'transition = fromImmediateSimplicityToCorrespondence',
      'correspondence = determinateConnectionOfOughtAndBeing',
      'correspondence = copula',
      'transition.liesIn = particularDeterminatenessOfFact',
      'genus = universalExistingInAndForItself',
      'genus.appears = unconnected',
      'determinateness.reflected = intoItselfAndOther',
    ],
    predicates: [{ name: 'CopulaHasGround', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'liesIn', from: 'transition', to: 'particularDeterminateness' },
    ],
  },
  {
    id: 'j-con-op-25-apodictic-ground-constitution',
    chunkId: 'j-con-25-apodictic-ground-constitution',
    label: 'Judgment has ground in constitution — apodictic',
    clauses: [
      'judgment.hasGround = inConstitutionOfSubject',
      'judgment.thereby = apodictic',
      'copula.determinate = true',
      'copula.accomplished = true',
      'copula.hitherto = abstractIs',
      'copula.developed = intoGroundInGeneral',
    ],
    predicates: [{ name: 'HasGroundInConstitution', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'has', from: 'judgment', to: 'ground' },
    ],
  },
  {
    id: 'j-con-op-26-apodictic-copula-determinateness',
    chunkId: 'j-con-26-apodictic-copula-determinateness',
    label: 'Copula as determinate connection',
    clauses: [
      'copula.attachesTo = subjectAsImmediateDeterminateness',
      'copula.connectionTo = predicate',
      'predicate.content = correspondence',
      'predicate.content = connectionOfSubjectToUniversality',
    ],
    predicates: [{ name: 'CopulaAsDeterminateConnection', args: ['copula'] }],
    relations: [
      { predicate: 'connects', from: 'copula', to: 'subjectPredicate' },
    ],
  },
  {
    id: 'j-con-op-27-apodictic-form-passes-away',
    chunkId: 'j-con-27-apodictic-form-passes-away',
    label: 'Form of judgment passes away — three reasons',
    clauses: [
      'formOfJudgment.passedAway = true',
      'reason1 = subjectAndPredicateSameContent',
      'reason2 = subjectPointsBeyondAndConnects',
      'reason3 = connectingPassedOverIntoPredicate',
      'connecting = contentOfPredicate',
      'connecting = judgmentItself',
    ],
    predicates: [{ name: 'FormPassesAway', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'passesAway', from: 'formOfJudgment', to: 'syllogism' },
    ],
  },
  {
    id: 'j-con-op-28-apodictic-concrete-identity-recovered',
    chunkId: 'j-con-28-apodictic-concrete-identity-recovered',
    label: 'Concrete identity recovered — inner foundation',
    clauses: [
      'concreteIdentity = resultOfDisjunctiveJudgment',
      'concreteIdentity = innerFoundationOfJudgmentOfConcept',
      'identity.positedAtFirst = onlyInPredicate',
      'identity.recovered = inWhole',
    ],
    predicates: [{ name: 'ConcreteIdentityRecovered', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'recovers', from: 'apodicticJudgment', to: 'concreteIdentity' },
    ],
  },
  {
    id: 'j-con-op-29-apodictic-subject-predicate-whole-concept',
    chunkId: 'j-con-29-apodictic-subject-predicate-whole-concept',
    label: 'Subject and predicate each whole concept',
    clauses: [
      'positiveFactor = transitionOfJudgmentIntoAnotherForm',
      'subject = wholeConcept',
      'predicate = wholeConcept',
    ],
    predicates: [{ name: 'EachWholeConcept', args: ['subject', 'predicate'] }],
    relations: [
      { predicate: 'is', from: 'subject', to: 'wholeConcept' },
      { predicate: 'is', from: 'predicate', to: 'wholeConcept' },
    ],
  },
  {
    id: 'j-con-op-30-apodictic-copula-unity',
    chunkId: 'j-con-30-apodictic-copula-unity',
    label: 'Copula as unity of concept — distinct yet connecting',
    clauses: [
      'unityOfConcept = determinatenessConstitutingCopula',
      'unity.connects = subjectAndPredicate',
      'unity.distinct = fromSubjectAndPredicate',
      'unity.standsOn = otherSideOfSubject',
      'unity.as = immediateConstitution',
      'unity.essence = toConnect',
      'unity.notOnly = immediateConstitution',
      'unity = universalRunningThrough',
    ],
    predicates: [{ name: 'CopulaAsUnity', args: ['copula'] }],
    relations: [
      { predicate: 'connects', from: 'unity', to: 'subjectPredicate' },
    ],
  },
  {
    id: 'j-con-op-31-apodictic-form-connection',
    chunkId: 'j-con-31-apodictic-form-connection',
    label: 'Form of connection posited through copula determinateness',
    clauses: [
      'subjectAndPredicate.have = sameContent',
      'formOfConnection.posited = throughCopulaDeterminateness',
      'determinateness = universalOrParticularity',
    ],
    predicates: [{ name: 'FormOfConnectionPosited', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'copulaDeterminateness', to: 'formOfConnection' },
    ],
  },
  {
    id: 'j-con-op-32-apodictic-copula-replete',
    chunkId: 'j-con-32-apodictic-copula-replete',
    label: 'Copula replete — contains form determinations',
    clauses: [
      'copula.contains = formDeterminationsOfExtremes',
      'copula = determinateConnection',
      'copula = accomplishedCopula',
      'copula = repleteOfContent',
      'copula = unityOfConcept',
      'unity.reEmerges = fromJudgment',
      'unity.wasLost = inExtremes',
    ],
    predicates: [{ name: 'CopulaReplete', args: ['copula'] }],
    relations: [
      { predicate: 'contains', from: 'copula', to: 'formDeterminations' },
    ],
  },
  {
    id: 'j-con-op-33-apodictic-to-syllogism',
    chunkId: 'j-con-33-apodictic-to-syllogism',
    label: 'Transition: apodictic judgment becomes syllogism (Idea)',
    clauses: [
      'byVirtueOf = repletionOfCopula',
      'judgment.become = syllogism',
      'syllogism = Idea',
      'modality.become = Certainty',
    ],
    predicates: [{ name: 'BecomesSyllogism', args: ['apodicticJudgment'] }],
    relations: [
      { predicate: 'becomes', from: 'apodicticJudgment', to: 'syllogism' },
      { predicate: 'becomes', from: 'modality', to: 'certainty' },
    ],
  },
];

/* accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getLogicalOpsForChunk(oneBasedIndex: number) {
  const chunk = getChunk(oneBasedIndex);
  if (!chunk) return [];
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunk.id);
}
