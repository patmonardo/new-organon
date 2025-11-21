import type { Chunk, LogicalOperation } from '../syllogism/index';

/**
 * JUDGMENT OF NECESSITY — Complete Structure
 * 
 * NOTE: The judgment of necessity posits objective universality—the universality
 * that exists in and for itself, corresponding to substantiality in the sphere of
 * essence, but distinguished by belonging to the concept and having immanent
 * necessity. This judgment determines universality as genus and species.
 * 
 * Structure:
 * Introduction: Objective universality and genus/species determination
 * a. The categorical judgment (genus/species structure, necessity)
 * b. The hypothetical judgment (conditional necessity, ground/consequence)
 * c. The disjunctive judgment (genus disjoined into species, concept as copula)
 * 
 * Transition: Judgment of Necessity rises to Judgment of the Concept
 */

// ============================================================================
// INTRODUCTION: JUDGMENT OF NECESSITY — OBJECTIVE UNIVERSALITY
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-nes-intro-1-objective-universality',
    title: 'Objective universality — substantiality in the concept',
    text: `The determination to which universality has advanced is, as we have seen, the universality that exists in and for itself or the objective universality that in the sphere of essence corresponds to substantiality. It is distinguished from the latter because it belongs to the concept and for this reason is not only the inner but also the posited necessity of its determinations, or in other words, the distinction is immanent to it, whereas substance has its distinction only in its accidents, does not have it as a principle within it.`,
  },
  {
    id: 'j-nes-intro-2-genus-and-species',
    title: 'Universality determined as genus and species',
    text: `In the judgment now, this objective universality is posited, first, posited with this determinateness as essential to it, immanent to it; second, posited with it as diverse from it, a particularity for which the said universality constitutes the substantial basis. In this way the universality is determined as genus and species.`,
  },
];

// ============================================================================
// a. THE CATEGORICAL JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-nes-1-categorical-genus-species',
    title: 'Categorical judgment — genus divides into species',
    text: `The genus essentially divides or repels itself into species; it is genus only in so far as it comprehends the species under it; the species is a species only in so far as, on the one side, it exists in singulars, and, on the other side, it possesses in the genus a higher universality. Now the categorical judgment has for predicate such a universality as in it the subject possesses its immanent nature.`,
  },
  {
    id: 'j-nes-2-categorical-immediate',
    title: 'Categorical judgment as immediate judgment of necessity',
    text: `But the categorical judgment is itself the first or the immediate judgment of necessity; consequently, the determinateness of the subject, by virtue of which the latter is a singular as contrasted to the genus or the species, belongs to the immediacy of external concrete existence. But objective universality also has here only its first immediate particularization; on the one hand, therefore, it is itself a determinate genus with respect to which there are higher genera; on the other hand, it is not the most proximate genus, that is, its determinateness is not directly the principle of the specific particularity of the subject.`,
  },
  {
    id: 'j-nes-3-categorical-substantial-identity',
    title: 'Substantial identity of subject and predicate',
    text: `But what is necessary in it is the substantial identity of subject and predicate, in view of which the distinguishing mark of each is only an unessential positedness or even only a name; in its predicate, the subject is reflected into its being-in-and-for-itself. Such a predicate ought not to be classed with the predicates of the preceding judgments.`,
  },
  {
    id: 'j-nes-4-categorical-examples',
    title: 'Examples distinguishing categorical from accidental predicates',
    text: `For example, to throw together into one class these judgments: The rose is red, The rose is a plant, or This ring is yellow, It is gold, and thus to take such an external property as the color of a flower as a predicate equal to its vegetable nature, is to overlook a difference which the dullest mind would not miss. The categorical judgment, therefore, is definitely to be distinguished from the positive and the negative judgment; in these, what is said of the subject is a singular accidental content; in the former, the content is the totality of the form reflected into itself.`,
  },
  {
    id: 'j-nes-5-categorical-copula-necessity',
    title: 'Copula expresses necessity vs abstract being',
    text: `In this content, therefore, the copula has the meaning of necessity, whereas in that of the other two it has only the meaning of abstract, immediate being.`,
  },
  {
    id: 'j-nes-6-categorical-inner-necessity',
    title: 'Determinateness, contingency, and inner necessity',
    text: `The determinateness of the subject, which makes it a particular with respect to the predicate, is at first still something contingent; subject and predicate are not connected with necessity by the form or the determinateness; the necessity is therefore still an inner one. The subject is subject, however, only as a particular, and to the extent that it possesses objective universality, it has to possess it essentially in accordance with that at first immediate determinateness.`,
  },
  {
    id: 'j-nes-7-categorical-to-hypothetical',
    title: 'Transition: categorical judgment passing to hypothetical',
    text: `The objective universal, in determining itself, that is, in positing itself in a judgment, is in a connection of identity with this repelled determinateness as such, essentially, that is, this determinateness is not to be posited as merely accidental. Only through this necessity of its immediate being does the categorical judgment conform to its objective universality and, in this way, has passed over into the hypothetical judgment.`,
  },
);

// ============================================================================
// b. THE HYPOTHETICAL JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-nes-8-hypothetical-form',
    title: 'Hypothetical judgment — "If A is, then B is"',
    text: `"If A is, then B is"; or "The being of A is not its own being but the being of an other, of B." What is posited in this judgment is the necessary connectedness of immediate determinacies, a connectedness which in the categorical judgment is not yet posited.`,
  },
  {
    id: 'j-nes-9-hypothetical-two-immediates',
    title: 'Two immediate, externally contingent concrete existences',
    text: `There are here two immediate, or externally contingent concrete existences, of which in the categorical judgment there is at first only one, the subject; but since one is external to the other, this other is immediately also external with respect to the first. On account of this immediacy, the contents of both sides are still indifferent to each other; at first, therefore, this judgment is a proposition of empty form.`,
  },
  {
    id: 'j-nes-10-hypothetical-connection-essential',
    title: 'Connection essential — being as mere possibility',
    text: `Now, first, the immediacy is as such indeed self-subsistent, a concrete being; but, second, what is essential is its connection; this being is therefore just as much mere possibility; the hypothetical judgment does not say either that A is, or that B is, but only that if the one is, then the other is; only the connectedness of the extremes is posited as existing, not the extremes themselves.`,
  },
  {
    id: 'j-nes-11-hypothetical-being-of-other',
    title: 'Each extreme as being of an other',
    text: `Indeed, each extreme is posited in this necessity as equally the being of an other. The principle of identity asserts that A is only A, not B; and B is only B, not A. In the hypothetical judgment, on the contrary, the being of finite things is posited through the concept in accordance with their formal truth, namely that the finite is its own being, but equally is not its own being but is the being of an other.`,
  },
  {
    id: 'j-nes-12-hypothetical-spheres-being-essence',
    title: 'Finite in sphere of being vs essence vs concept',
    text: `In the sphere of being, the finite alters and comes to be an other. In the sphere of essence, it is appearance; its being is posited to consist in the reflective shining of an other in it, and the necessity is the inner connection not yet posited as such. But the concept is this: that this identity is posited; that the existent is not abstract self-identity but concrete self-identity and is, immediately within it, the being of an other.`,
  },
  {
    id: 'j-nes-13-hypothetical-relations-reflection',
    title: 'Relations of reflection as moments of one identity',
    text: `The hypothetical judgment can be more closely determined in terms of the relations of reflection as a relation of ground and consequence, condition and conditioned, causality etc. Just as substantiality is present in the categorical judgment in the form of its concept, so is the connectedness of causality in the hypothetical judgment. This and the other relations all recur in it, but they are there essentially only as moments of one and the same identity.`,
  },
  {
    id: 'j-nes-14-hypothetical-indeterminate-form',
    title: 'Hypothetical as proposition — indeterminate form',
    text: `However, in it they are as yet not opposed as singular or particular and universal according to the determinations of the concept, but are only as moments in general at first. The hypothetical judgment, therefore, has a shape which is more that of a proposition; just as the particular judgment is of indeterminate content, so is the hypothetical of indeterminate form, for the determination of its content does not conform to the relation of subject and predicate.`,
  },
  {
    id: 'j-nes-15-hypothetical-concrete-universality',
    title: 'Being as unity of self and other — concrete universality',
    text: `Yet the being, since it is the being of the other, is for that very reason in itself the unity of itself and the other, and therefore universality; by the same token it is in fact only a particular, for it is a determinate being and does not refer in its determinateness merely to itself. But it is not the simple, abstract particularity that is posited; on the contrary, through the immediacy which the determinacies possess, the moments of particularity are differentiated; at the same time, through the unity of these moments as constituted by their connection, the particularity is also their totality.`,
  },
  {
    id: 'j-nes-16-hypothetical-to-disjunctive',
    title: 'Transition: hypothetical to disjunctive judgment',
    text: `In truth, therefore, what is posited in this judgment is universality as the concrete identity of the concept whose determinations do not have any subsistence of their own but are only particularities posited in that identity. So it is the disjunctive judgment.`,
  },
);

// ============================================================================
// c. THE DISJUNCTIVE JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-nes-17-disjunctive-overview',
    title: 'Disjunctive judgment — objective universality in union with form',
    text: `In the categorical judgment, the concept is objective universality and an external singularity. In the hypothetical, the concept manifests its presence in this externality, in its negative identity. Through this identity, the objective universality and the external singularity obtain the determinateness, now posited in the disjunctive judgment, which in the hypothetical they possess immediately. Hence the disjunctive judgment is objective universality at the same time posited in union with the form.`,
  },
  {
    id: 'j-nes-18-disjunctive-structure',
    title: 'Disjunctive structure — genus and differentiated determinations',
    text: `It thus contains, first, the concrete universality or the genus in simple form, as the subject; second, the same universality but as the totality of its differentiated determinations. "A is either B or C." This is the necessity of the concept in which, first, the self-identity of the two extremes is of the same extent, content, and universality.`,
  },
  {
    id: 'j-nes-19-disjunctive-differentiation-form',
    title: 'Differentiation according to form — mere form',
    text: `Second, they are differentiated according to the form of conceptual determination, but, because of that identity, this determination is a mere form. Third, the identical objective universality appears for that reason reflected into itself as against the non-essential form, as a content which however has the determinateness of form in it, once as the simple determination of genus; then again, as this determinateness developed in its difference, and in this way it is the particularity of the species and their totality, the universality of the genus.`,
  },
  {
    id: 'j-nes-20-disjunctive-particularity-predicate',
    title: 'Particularity as predicate — greater universal',
    text: `The particularity constitutes in its development the predicate, because, in containing the whole universal sphere of the subject, and in containing it, however, also in the articulation of particularity, it is to that extent the greater universal.`,
  },
  {
    id: 'j-nes-21-disjunctive-positive-identity',
    title: 'Genus as substantial universality — positive identity',
    text: `Upon closer consideration of this particularization, it is the genus that constitutes first of all the substantial universality of the species; the subject is thus B as well as C; this "as well as" indicates the positive identity of the particular with the universal; this objective universal maintains itself fully in its particularity.`,
  },
  {
    id: 'j-nes-22-disjunctive-negative-connection',
    title: 'Species mutually exclude — negative connection',
    text: `Secondly, the species mutually exclude one another; "A is either B or C"; for they are the specific difference of the universal sphere. This "either or" is their negative connection. In this negative connection they are just as identical as in the positive; the genus is their unity as a unity of determinate particulars.`,
  },
  {
    id: 'j-nes-23-disjunctive-concrete-vs-abstract',
    title: 'Concrete universality vs abstract universality',
    text: `If the genus were an abstract universality, as in the judgments of existence, then the species would also have to be taken as diverse and mutually indifferent; this universality, however, is not the external one that arises only through comparison and abstraction but is, on the contrary, the universality which is immanent to the genus and concrete.`,
  },
  {
    id: 'j-nes-24-disjunctive-empirical-vs-necessary',
    title: 'Empirical disjunction vs necessary disjunction',
    text: `An empirical disjunctive judgment is without necessity; A is either B or C or D, etc., because the species B, C, D, etc., are found beforehand; strictly speaking, therefore, there is no question here of an "either or," for the completeness of these species is only a subjective one; of course, one species excludes the other, but the "either or" excludes every other species and excludes within itself an entire sphere.`,
  },
  {
    id: 'j-nes-25-disjunctive-negative-unity',
    title: 'Necessity in negative unity of objective universal',
    text: `This totality has its necessity in the negative unity of the objective universal which has dissolved singularity within itself and possesses, immanent in it, the simple principle of differentiation by which the species are determined and connected.`,
  },
  {
    id: 'j-nes-26-disjunctive-empirical-species',
    title: 'Empirical species lack immanent determinateness',
    text: `The empirical species, on the contrary, have their differences in some accidentality or other which is a principle external to them and is not, therefore, their principle, and consequently also not the immanent determinateness of the genus; for this reason, they are also not reciprocally connected according to their determinateness. Yet it is by virtue of their determinateness that the species constitute the universality of the predicate.`,
  },
  {
    id: 'j-nes-27-disjunctive-contrary-contradictory',
    title: 'Contrary and contradictory concepts — proper place',
    text: `Here is where the so-called contrary and contradictory concepts should find their proper place, for the disjunctive judgment is where the essential difference of the concept is posited; but here they also equally find their truth, namely that contrariness and contradictoriness are themselves differentiated both as contraries and as contradictory.`,
  },
  {
    id: 'j-nes-28-disjunctive-contrary-meaning',
    title: 'Species as contrary — diverse yet subsisting',
    text: `Species are contrary inasmuch as they are merely diverse, that is to say, inasmuch as they possess an immediate existence as subsisting in and for themselves by virtue of the genus which is their nature. They are contradictory, inasmuch as they exclude one another. But each of these determinations is by itself one-sided and void of truth.`,
  },
  {
    id: 'j-nes-29-disjunctive-unity-truth',
    title: 'Unity of contrariness and contradictoriness as truth',
    text: `In the "either or" of the disjunctive judgment, their unity is posited as their truth, which is that the independent subsistence of the species as concrete universality is itself also the principle of the negative unity by which they mutually exclude one another.`,
  },
  {
    id: 'j-nes-30-disjunctive-proximate-genus',
    title: 'Proximate genus — quantitative difference',
    text: `Through the identity just demonstrated of subject and predicate in accordance with the negative unity, the genus is determined in the disjunctive judgment as the proximate genus. This expression indicates at first the mere quantitative difference of the more or less determinations which a universal contains as contrasted to a particularity coming under it. On this account, which is the truly proximate genus remains contingent.`,
  },
  {
    id: 'j-nes-31-disjunctive-abstract-genus',
    title: 'Abstract genus cannot form disjunctive judgment',
    text: `But then, if the genus is taken as a universal arrived at by the mere abstraction of determinations, it cannot strictly speaking form a disjunctive judgment; for it is contingent whether, as it were, there is still left in it the determinateness that constitutes the principle of the "either or"; the genus would not be displayed in the species according to its determinateness, and these would only be capable of contingent completeness.`,
  },
  {
    id: 'j-nes-32-disjunctive-categorical-external',
    title: 'Categorical judgment — genus external to subject',
    text: `In the categorical judgment, the genus stands at first over against the subject only in this abstract form, is not, therefore, necessarily its proximate genus and, to this extent, is external to it.`,
  },
  {
    id: 'j-nes-33-disjunctive-concrete-genus',
    title: 'Concrete genus as proximate genus',
    text: `But when the genus is a concrete, essentially determined universality, then, as simple determinateness, it is the unity of the moments of the concept; moments that, only sublated in that simplicity, have their real difference in the species. Hence the genus is the proximate genus of a species, for the latter possesses its specific difference in the essential determinateness of the genus, and the species have as such the determination differentiating them in the nature of the genus.`,
  },
  {
    id: 'j-nes-34-disjunctive-determinateness-aspect',
    title: 'Identity from aspect of determinateness',
    text: `What we have just considered constitutes the identity of subject and predicate from the aspect of determinateness in general. This is an aspect that was posited by the hypothetical judgment, the necessity of which is an identity of immediate and diverse things and is, therefore, essentially a negative unity.`,
  },
  {
    id: 'j-nes-35-disjunctive-negative-unity-differentiated',
    title: 'Negative unity differentiated — subject and predicate',
    text: `It is this negative unity that in principle separates subject and predicate but is now posited as itself differentiated – in the subject, as simple determinateness; in the predicate, as totality. That parting of subject and predicate is the difference of the concept; the totality of the species in the predicate can then be none other than this difference.`,
  },
  {
    id: 'j-nes-36-disjunctive-concept-disjoins',
    title: 'Concept disjoins itself — negative unity',
    text: `The reciprocal determination of the disjunctive terms is therefore hereby given. It reduces to the difference of the concept, for it is the concept alone that disjoins itself and manifests in its determination its negative unity.`,
  },
  {
    id: 'j-nes-37-disjunctive-species-conceptual-determinateness',
    title: 'Species as simple conceptual determinateness',
    text: `Of course, the species comes up for consideration here only under the aspect of its simple conceptual determinateness, not according to the shape in which, proceeding from the idea, it steps into a further self-subsistent reality. This reality is of course dropped in the simple principle of the genus; but the essential differentiation must be a moment of the concept.`,
  },
  {
    id: 'j-nes-38-disjunctive-concept-progressive-determination',
    title: 'Concept\'s own progressive determination',
    text: `In the judgment here considered, it is really now the concept's own progressive determination that itself posits its disjunction, just as was the case for the concept itself, as we saw when it was determined in and for itself and was differentiated into determinate concepts.`,
  },
  {
    id: 'j-nes-39-disjunctive-concept-as-member',
    title: 'Concept as universal is itself a disjunctive member',
    text: `Now because the concept is the universal, the positive as well as the negative totality of the particulars, for that reason it is immediately itself also one of its disjunctive members; the other member, however, is this universality resolved into its particularity, or the determinateness of the concept as determinateness, in which the very universality displays itself as totality.`,
  },
  {
    id: 'j-nes-40-disjunctive-form-validation',
    title: 'Disjunction form validation — concept determinateness',
    text: `If the disjunction of a genus into species has not yet attained this form, this is proof that the disjunction has not risen to the determinateness of the concept and has not proceeded from it.`,
  },
  {
    id: 'j-nes-41-disjunctive-color-example',
    title: 'Color example — empirical disjunction as barbarism',
    text: `Color is either violet, indigo, blue, green, yellow, orange, or red; even empirically, the confusion and impurity of such a disjunction are at once apparent; it is a barbarism even from this standpoint.`,
  },
  {
    id: 'j-nes-42-disjunctive-color-principle',
    title: 'Color as concrete unity — principle of differentiation',
    text: `If color is conceived as the concrete unity of light and darkness, then this genus has within it the determinateness that constitutes the principle of its particularization into species. Of these, however, one must be the utterly simple color that holds the opposition in balance, contained and negated in the color's intensity; the relation of the opposition of light and darkness must then take its place over against it, and, since this relation is a natural phenomenon, the indifferent neutrality of the opposition must be further added to it.`,
  },
  {
    id: 'j-nes-43-disjunctive-warning-empirical',
    title: 'Warning against improper genera and empirical mixtures',
    text: `Taking for genus such mixtures as violet, and orange, or shades of difference like indigo blue and light blue, betrays a totally inconsiderate procedure that shows too little reflection even for empiricism. But this is not the place to discuss the different and more finely determined forms that disjunction may indeed assume in the element of nature or spirit.`,
  },
  {
    id: 'j-nes-44-disjunctive-subject-predicate-members',
    title: 'Subject and predicate as members of disjunction',
    text: `In the first instance, the disjunctive judgment has the members of the disjunction in the predicate. But the judgment is itself equally disjoined; its subject and predicate are the members of the disjunction; they are the moments of the concept posited in their determinateness but at the same time as identical; identical, (a) in the objective universality which is in the subject as the simple genus, and in the predicate as the universal sphere and totality of the moments of the concept; and (b) in the negative unity, the developed connectedness of necessity, in accordance with which the simple determinateness in the subject has fallen apart into the difference of the species and these, in this very difference, have their essential connection and self-identity.`,
  },
  {
    id: 'j-nes-45-disjunctive-copula-concept',
    title: 'Copula becomes the concept — judgment of the concept',
    text: `This unity, the copula of this judgment in which the extremes have come together through their identity, is thus the concept itself, indeed the concept as posited; the mere judgment of necessity has thereby risen to the judgment of the concept.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // INTRODUCTION OPERATIONS
  // ============================================================================
  {
    id: 'j-nes-op-intro-1-objective-universality',
    chunkId: 'j-nes-intro-1-objective-universality',
    label: 'Declare objective universality as substantiality in the concept',
    clauses: [
      'universality.advanced = objectiveUniversality',
      'objectiveUniversality = existsInAndForItself',
      'objectiveUniversality.correspondsTo = substantiality (inEssence)',
      'objectiveUniversality.belongsTo = concept',
      'objectiveUniversality.has = positedNecessity',
      'distinction.immanent = true',
      'substance.hasDistinction = onlyInAccidents',
      'substance.notHas = principleWithin',
    ],
    predicates: [{ name: 'IsObjectiveUniversality', args: [] }],
    relations: [
      { predicate: 'corresponds', from: 'objectiveUniversality', to: 'substantiality' },
    ],
  },
  {
    id: 'j-nes-op-intro-2-genus-species',
    chunkId: 'j-nes-intro-2-genus-and-species',
    label: 'Universality determined as genus and species',
    clauses: [
      'objectiveUniversality.posited = withDeterminateness',
      'determinateness.essential = true',
      'determinateness.immanent = true',
      'universality.diverseFrom = particularity',
      'particularity.hasBasis = substantialUniversality',
      'universality.determinedAs = genusAndSpecies',
    ],
    predicates: [{ name: 'DeterminesGenusSpecies', args: ['universality'] }],
    relations: [
      { predicate: 'determines', from: 'universality', to: 'genusSpecies' },
    ],
  },
  // ============================================================================
  // CATEGORICAL JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-nes-op-1-categorical-genus-species',
    chunkId: 'j-nes-1-categorical-genus-species',
    label: 'Declare genus divides into species — categorical structure',
    clauses: [
      'genus.divides = intoSpecies',
      'genus.repelsItself = intoSpecies',
      'genus.isGenus = onlyInComprehendingSpecies',
      'species.isSpecies = {existsInSingulars, possessesHigherUniversality}',
      'categoricalJudgment.predicate = universality',
      'subject.possessesInPredicate = immanentNature',
    ],
    predicates: [{ name: 'IsCategoricalJudgment', args: [] }],
    relations: [
      { predicate: 'divides', from: 'genus', to: 'species' },
    ],
  },
  {
    id: 'j-nes-op-2-categorical-immediate',
    chunkId: 'j-nes-2-categorical-immediate',
    label: 'Categorical as immediate judgment of necessity',
    clauses: [
      'categoricalJudgment = firstImmediateJudgmentOfNecessity',
      'subject.determinateness = singular (contrastedToGenusSpecies)',
      'determinateness.belongsTo = immediacyOfExternalConcreteExistence',
      'objectiveUniversality.has = firstImmediateParticularization',
      'genus.isDeterminateGenus = withRespectToHigherGenera',
      'genus.not = mostProximateGenus',
      'determinateness.not = principleOfSpecificParticularity',
    ],
    predicates: [{ name: 'IsImmediateNecessity', args: ['categoricalJudgment'] }],
    relations: [
      { predicate: 'belongsTo', from: 'determinateness', to: 'immediacy' },
    ],
  },
  {
    id: 'j-nes-op-3-categorical-substantial-identity',
    chunkId: 'j-nes-3-categorical-substantial-identity',
    label: 'Substantial identity of subject and predicate',
    clauses: [
      'necessary = substantialIdentityOfSubjectAndPredicate',
      'distinguishingMark = unessentialPositedness',
      'distinguishingMark = onlyName',
      'subject.reflectedInto = beingInAndForItself',
      'categoricalPredicate.notClassed = withPrecedingJudgments',
    ],
    predicates: [{ name: 'HasSubstantialIdentity', args: ['categoricalJudgment'] }],
    relations: [
      { predicate: 'reflects', from: 'subject', to: 'beingInAndForItself' },
    ],
  },
  {
    id: 'j-nes-op-4-categorical-examples',
    chunkId: 'j-nes-4-categorical-examples',
    label: 'Distinguish categorical from accidental predicates',
    clauses: [
      'example1 = "rose is red" (accidental)',
      'example2 = "rose is plant" (categorical)',
      'example3 = "ring is yellow" (accidental)',
      'example4 = "ring is gold" (categorical)',
      'externalProperty.notEqual = vegetableNature',
      'categoricalJudgment.distinguishedFrom = {positiveJudgment, negativeJudgment}',
      'precedingJudgments.content = singularAccidental',
      'categoricalJudgment.content = totalityOfFormReflectedIntoItself',
    ],
    predicates: [{ name: 'DistinguishesCategorical', args: ['examples'] }],
    relations: [
      { predicate: 'distinguishes', from: 'categoricalJudgment', to: 'precedingJudgments' },
    ],
  },
  {
    id: 'j-nes-op-5-categorical-copula-necessity',
    chunkId: 'j-nes-5-categorical-copula-necessity',
    label: 'Copula expresses necessity vs abstract being',
    clauses: [
      'categoricalCopula.meaning = necessity',
      'precedingCopula.meaning = abstractImmediateBeing',
    ],
    predicates: [{ name: 'CopulaExpressesNecessity', args: ['categoricalJudgment'] }],
    relations: [
      { predicate: 'expresses', from: 'copula', to: 'necessity' },
    ],
  },
  {
    id: 'j-nes-op-6-categorical-inner-necessity',
    chunkId: 'j-nes-6-categorical-inner-necessity',
    label: 'Determinateness contingent externally, necessary internally',
    clauses: [
      'subject.determinateness = particular (withRespectToPredicate)',
      'determinateness.atFirst = contingent',
      'subjectAndPredicate.notConnected = byFormOrDeterminateness',
      'necessity = inner',
      'subject.isSubject = onlyAsParticular',
      'subject.possessesObjectiveUniversality = essentially',
      'objectiveUniversality.possessed = inAccordanceWithImmediateDeterminateness',
    ],
    predicates: [{ name: 'HasInnerNecessity', args: ['categoricalJudgment'] }],
    relations: [
      { predicate: 'possesses', from: 'subject', to: 'objectiveUniversality' },
    ],
  },
  {
    id: 'j-nes-op-7-categorical-to-hypothetical',
    chunkId: 'j-nes-7-categorical-to-hypothetical',
    label: 'Transition: categorical judgment passing to hypothetical',
    clauses: [
      'objectiveUniversal.determinesItself = inJudgment',
      'objectiveUniversal.inConnection = identityWithRepelledDeterminateness',
      'determinateness.not = merelyAccidental',
      'necessityOfImmediateBeing = conformsToObjectiveUniversality',
      'categoricalJudgment.passedOver = intoHypotheticalJudgment',
    ],
    predicates: [{ name: 'TransitionsToHypothetical', args: ['categoricalJudgment'] }],
    relations: [
      { predicate: 'transitions', from: 'categoricalJudgment', to: 'hypotheticalJudgment' },
    ],
  },
  // ============================================================================
  // HYPOTHETICAL JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-nes-op-8-hypothetical-form',
    chunkId: 'j-nes-8-hypothetical-form',
    label: 'Hypothetical judgment — "If A is, then B is"',
    clauses: [
      'hypotheticalForm = "if A is, then B is"',
      'alternativeForm = "being of A is being of B"',
      'posited = necessaryConnectednessOfImmediateDeterminacies',
      'connectedness.notYetPosited = inCategoricalJudgment',
    ],
    predicates: [{ name: 'IsHypotheticalJudgment', args: [] }],
    relations: [
      { predicate: 'posits', from: 'hypotheticalJudgment', to: 'connectedness' },
    ],
  },
  {
    id: 'j-nes-op-9-hypothetical-two-immediates',
    chunkId: 'j-nes-9-hypothetical-two-immediates',
    label: 'Two immediate, externally contingent concrete existences',
    clauses: [
      'thereAre = twoImmediateConcreteExistences',
      'existences = externallyContingent',
      'categoricalJudgment.has = onlyOneSubject',
      'one.externalTo = other',
      'other.externalTo = first',
      'contents.indifferent = toEachOther',
      'judgment.atFirst = propositionOfEmptyForm',
    ],
    predicates: [{ name: 'HasTwoImmediates', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'external', from: 'one', to: 'other' },
    ],
  },
  {
    id: 'j-nes-op-10-hypothetical-connection-essential',
    chunkId: 'j-nes-10-hypothetical-connection-essential',
    label: 'Connection essential — being as mere possibility',
    clauses: [
      'immediacy.selfSubsistent = concreteBeing',
      'essential = connection',
      'being = merePossibility',
      'hypotheticalJudgment.notSays = {A is, B is}',
      'hypotheticalJudgment.says = ifOneThenOther',
      'positedAsExisting = connectednessOfExtremes',
      'notPosited = extremesThemselves',
    ],
    predicates: [{ name: 'ConnectionEssential', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'hypotheticalJudgment', to: 'connectedness' },
    ],
  },
  {
    id: 'j-nes-op-11-hypothetical-being-of-other',
    chunkId: 'j-nes-11-hypothetical-being-of-other',
    label: 'Each extreme as being of an other',
    clauses: [
      'eachExtreme.posited = asBeingOfOther',
      'principleOfIdentity.asserts = {A is only A, B is only B}',
      'hypotheticalJudgment.posits = finiteThroughConcept',
      'formalTruth = finiteIsOwnBeingAndNotOwnBeing',
      'finite.isOwnBeing = true',
      'finite.notOwnBeing = true',
      'finite.isBeingOfOther = true',
    ],
    predicates: [{ name: 'PositsBeingOfOther', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'hypotheticalJudgment', to: 'beingOfOther' },
    ],
  },
  {
    id: 'j-nes-op-12-hypothetical-spheres-being-essence',
    chunkId: 'j-nes-12-hypothetical-spheres-being-essence',
    label: 'Finite in sphere of being vs essence vs concept',
    clauses: [
      'inSphereOfBeing.finite = altersAndComesToBeOther',
      'inSphereOfEssence.finite = appearance',
      'being.consistsIn = reflectiveShiningOfOther',
      'necessity = innerConnection (notYetPosited)',
      'concept = identityPosited',
      'existent.not = abstractSelfIdentity',
      'existent = concreteSelfIdentity',
      'existent.immediatelyWithin = beingOfOther',
    ],
    predicates: [{ name: 'PositsConceptualIdentity', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'concept', to: 'identity' },
    ],
  },
  {
    id: 'j-nes-op-13-hypothetical-relations-reflection',
    chunkId: 'j-nes-13-hypothetical-relations-reflection',
    label: 'Relations of reflection as moments of one identity',
    clauses: [
      'hypotheticalJudgment.determinedAs = relationsOfReflection',
      'relations = {groundConsequence, conditionConditioned, causality}',
      'substantiality.presentIn = categoricalJudgment (asConcept)',
      'connectednessOfCausality.presentIn = hypotheticalJudgment',
      'relations.recur = asMomentsOfOneIdentity',
      'relations.notOpposed = asSingularParticularUniversal',
      'relations.are = momentsInGeneral',
    ],
    predicates: [{ name: 'MapsReflectiveRelations', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'unifies', from: 'hypotheticalJudgment', to: 'reflectiveRelations' },
    ],
  },
  {
    id: 'j-nes-op-14-hypothetical-indeterminate-form',
    chunkId: 'j-nes-14-hypothetical-indeterminate-form',
    label: 'Hypothetical as proposition — indeterminate form',
    clauses: [
      'hypotheticalJudgment.shape = moreProposition',
      'particularJudgment = indeterminateContent',
      'hypotheticalJudgment = indeterminateForm',
      'content.determination.notConforms = toSubjectPredicateRelation',
    ],
    predicates: [{ name: 'IsIndeterminateForm', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'has', from: 'hypotheticalJudgment', to: 'indeterminateForm' },
    ],
  },
  {
    id: 'j-nes-op-15-hypothetical-concrete-universality',
    chunkId: 'j-nes-15-hypothetical-concrete-universality',
    label: 'Being as unity of self and other — concrete universality',
    clauses: [
      'being.isBeingOfOther = unityOfSelfAndOther',
      'being = universality',
      'being = particular',
      'being = determinateBeing',
      'determinateness.notRefers = merelyToItself',
      'particularity.not = simpleAbstract',
      'momentsOfParticularity.differentiated = throughImmediacy',
      'particularity = totalityOfMoments',
    ],
    predicates: [{ name: 'PositsConcreteUniversality', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'hypotheticalJudgment', to: 'concreteUniversality' },
    ],
  },
  {
    id: 'j-nes-op-16-hypothetical-to-disjunctive',
    chunkId: 'j-nes-16-hypothetical-to-disjunctive',
    label: 'Transition: hypothetical to disjunctive judgment',
    clauses: [
      'posited = universalityAsConcreteIdentity',
      'concreteIdentity = concept',
      'determinations.notHave = independentSubsistence',
      'determinations = particularitiesPositedInIdentity',
      'result = disjunctiveJudgment',
    ],
    predicates: [{ name: 'TransitionsToDisjunctive', args: ['hypotheticalJudgment'] }],
    relations: [
      { predicate: 'transitions', from: 'hypotheticalJudgment', to: 'disjunctiveJudgment' },
    ],
  },
  // ============================================================================
  // DISJUNCTIVE JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-nes-op-17-disjunctive-overview',
    chunkId: 'j-nes-17-disjunctive-overview',
    label: 'Disjunctive judgment — objective universality in union with form',
    clauses: [
      'categoricalJudgment.concept = {objectiveUniversality, externalSingularity}',
      'hypotheticalJudgment.concept = manifestsInExternality',
      'manifestation = negativeIdentity',
      'disjunctiveJudgment = objectiveUniversalityInUnionWithForm',
    ],
    predicates: [{ name: 'IsDisjunctiveJudgment', args: [] }],
    relations: [
      { predicate: 'unites', from: 'disjunctiveJudgment', to: 'objectiveUniversality' },
    ],
  },
  {
    id: 'j-nes-op-18-disjunctive-structure',
    chunkId: 'j-nes-18-disjunctive-structure',
    label: 'Disjunctive structure — genus and differentiated determinations',
    clauses: [
      'disjunctiveJudgment.contains = {concreteUniversality, genusAsSubject}',
      'disjunctiveJudgment.contains = {sameUniversality, totalityOfDifferentiatedDeterminations}',
      'form = "A is either B or C"',
      'necessity = necessityOfConcept',
      'extremes.selfIdentity = {sameExtent, sameContent, sameUniversality}',
    ],
    predicates: [{ name: 'HasDisjunctiveStructure', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'contains', from: 'disjunctiveJudgment', to: 'genusAndSpecies' },
    ],
  },
  {
    id: 'j-nes-op-19-disjunctive-differentiation-form',
    chunkId: 'j-nes-19-disjunctive-differentiation-form',
    label: 'Differentiation according to form — mere form',
    clauses: [
      'extremes.differentiated = accordingToFormOfConceptualDetermination',
      'becauseOfIdentity.determination = mereForm',
      'objectiveUniversality.reflected = intoItself',
      'content.hasDeterminateness = ofForm',
      'content = {simpleDeterminationOfGenus, developedInDifference}',
      'developed = {particularityOfSpecies, totality, universalityOfGenus}',
    ],
    predicates: [{ name: 'DifferentiatesByForm', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'differentiates', from: 'disjunctiveJudgment', to: 'species' },
    ],
  },
  {
    id: 'j-nes-op-20-disjunctive-particularity-predicate',
    chunkId: 'j-nes-20-disjunctive-particularity-predicate',
    label: 'Particularity as predicate — greater universal',
    clauses: [
      'particularity.constitutes = predicate',
      'particularity.contains = wholeUniversalSphereOfSubject',
      'particularity.contains = articulationOfParticularity',
      'particularity = greaterUniversal',
    ],
    predicates: [{ name: 'ParticularityAsPredicate', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'constitutes', from: 'particularity', to: 'predicate' },
    ],
  },
  {
    id: 'j-nes-op-21-disjunctive-positive-identity',
    chunkId: 'j-nes-21-disjunctive-positive-identity',
    label: 'Genus as substantial universality — positive identity',
    clauses: [
      'genus.constitutes = substantialUniversalityOfSpecies',
      'subject = B as well as C',
      '"as well as" = positiveIdentity',
      'positiveIdentity = particularWithUniversal',
      'objectiveUniversal.maintainsItself = fullyInParticularity',
    ],
    predicates: [{ name: 'HasPositiveIdentity', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'maintains', from: 'objectiveUniversal', to: 'particularity' },
    ],
  },
  {
    id: 'j-nes-op-22-disjunctive-negative-connection',
    chunkId: 'j-nes-22-disjunctive-negative-connection',
    label: 'Species mutually exclude — negative connection',
    clauses: [
      'species.mutuallyExclude = oneAnother',
      'form = "A is either B or C"',
      'species = specificDifferenceOfUniversalSphere',
      '"either or" = negativeConnection',
      'negativeConnection.species = justAsIdenticalAsPositive',
      'genus = unityOfDeterminateParticulars',
    ],
    predicates: [{ name: 'HasNegativeConnection', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'excludes', from: 'species', to: 'species' },
    ],
  },
  {
    id: 'j-nes-op-23-disjunctive-concrete-vs-abstract',
    chunkId: 'j-nes-23-disjunctive-concrete-vs-abstract',
    label: 'Concrete universality vs abstract universality',
    clauses: [
      'if genus = abstractUniversality then species = diverseAndMutuallyIndifferent',
      'genus.not = externalUniversality (throughComparisonAbstraction)',
      'genus = immanentToGenusAndConcrete',
    ],
    predicates: [{ name: 'IsConcreteUniversality', args: ['genus'] }],
    relations: [
      { predicate: 'distinguishes', from: 'concreteUniversality', to: 'abstractUniversality' },
    ],
  },
  {
    id: 'j-nes-op-24-disjunctive-empirical-vs-necessary',
    chunkId: 'j-nes-24-disjunctive-empirical-vs-necessary',
    label: 'Empirical disjunction vs necessary disjunction',
    clauses: [
      'empiricalDisjunctiveJudgment.without = necessity',
      'form = "A is either B or C or D, etc."',
      'species.found = beforehand',
      'completeness = onlySubjective',
      '"either or".excludes = everyOtherSpecies',
      '"either or".excludes = entireSphere',
    ],
    predicates: [{ name: 'IsEmpiricalDisjunction', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'distinguishes', from: 'necessaryDisjunction', to: 'empiricalDisjunction' },
    ],
  },
  {
    id: 'j-nes-op-25-disjunctive-negative-unity',
    chunkId: 'j-nes-25-disjunctive-negative-unity',
    label: 'Necessity in negative unity of objective universal',
    clauses: [
      'totality.hasNecessity = inNegativeUnity',
      'negativeUnity = objectiveUniversal',
      'objectiveUniversal.dissolved = singularityWithinItself',
      'objectiveUniversal.possesses = simplePrincipleOfDifferentiation',
      'principle.determines = species',
      'principle.connects = species',
    ],
    predicates: [{ name: 'HasNegativeUnity', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'grounds', from: 'negativeUnity', to: 'necessity' },
    ],
  },
  {
    id: 'j-nes-op-26-disjunctive-empirical-species',
    chunkId: 'j-nes-26-disjunctive-empirical-species',
    label: 'Empirical species lack immanent determinateness',
    clauses: [
      'empiricalSpecies.differences = inAccidentality',
      'principle = externalToSpecies',
      'principle.not = theirPrinciple',
      'principle.not = immanentDeterminatenessOfGenus',
      'species.notReciprocallyConnected = accordingToDeterminateness',
      'species.constituteUniversality = byVirtueOfDeterminateness',
    ],
    predicates: [{ name: 'LacksImmanentDeterminateness', args: ['empiricalSpecies'] }],
    relations: [
      { predicate: 'lacks', from: 'empiricalSpecies', to: 'immanentDeterminateness' },
    ],
  },
  {
    id: 'j-nes-op-27-disjunctive-contrary-contradictory',
    chunkId: 'j-nes-27-disjunctive-contrary-contradictory',
    label: 'Contrary and contradictory concepts — proper place',
    clauses: [
      'disjunctiveJudgment = properPlaceForContraryContradictory',
      'disjunctiveJudgment.posits = essentialDifferenceOfConcept',
      'contrarinessContradictoriness.findTruth = inDisjunctiveJudgment',
      'truth = contrarinessContradictorinessDifferentiated',
    ],
    predicates: [{ name: 'IsProperPlaceForContraryContradictory', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'disjunctiveJudgment', to: 'essentialDifference' },
    ],
  },
  {
    id: 'j-nes-op-28-disjunctive-contrary-meaning',
    chunkId: 'j-nes-28-disjunctive-contrary-meaning',
    label: 'Species as contrary — diverse yet subsisting',
    clauses: [
      'species.contrary = asMerelyDiverse',
      'species.possess = immediateExistence',
      'species.subsist = inAndForThemselves',
      'subsistence.byVirtueOf = genus',
      'species.contradictory = asExcludingOneAnother',
      'eachDetermination.oneSided = voidOfTruth',
    ],
    predicates: [{ name: 'SpeciesAsContrary', args: ['species'] }],
    relations: [
      { predicate: 'excludes', from: 'species', to: 'species' },
    ],
  },
  {
    id: 'j-nes-op-29-disjunctive-unity-truth',
    chunkId: 'j-nes-29-disjunctive-unity-truth',
    label: 'Unity of contrariness and contradictoriness as truth',
    clauses: [
      '"either or".posits = unityAsTruth',
      'truth = independentSubsistenceIsPrinciple',
      'independentSubsistence = speciesAsConcreteUniversality',
      'principle = negativeUnity',
      'negativeUnity.mutuallyExcludes = species',
    ],
    predicates: [{ name: 'UnityAsTruth', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'disjunctiveJudgment', to: 'unity' },
    ],
  },
  {
    id: 'j-nes-op-30-disjunctive-proximate-genus',
    chunkId: 'j-nes-30-disjunctive-proximate-genus',
    label: 'Proximate genus — quantitative difference',
    clauses: [
      'genus.determined = asProximateGenus',
      'determination.through = identityOfSubjectAndPredicate',
      'identity.inAccordanceWith = negativeUnity',
      'proximateGenus.indicates = quantitativeDifference',
      'quantitativeDifference = moreOrLessDeterminations',
      'whichGenus = trulyProximate (remainsContingent)',
    ],
    predicates: [{ name: 'IsProximateGenus', args: ['genus'] }],
    relations: [
      { predicate: 'determines', from: 'disjunctiveJudgment', to: 'proximateGenus' },
    ],
  },
  {
    id: 'j-nes-op-31-disjunctive-abstract-genus',
    chunkId: 'j-nes-31-disjunctive-abstract-genus',
    label: 'Abstract genus cannot form disjunctive judgment',
    clauses: [
      'if genus = abstractUniversal then cannotForm = disjunctiveJudgment',
      'reason = determinatenessMayBeMissing',
      'determinateness = principleOfEitherOr',
      'genus.notDisplayed = inSpeciesAccordingToDeterminateness',
      'species.capable = onlyContingentCompleteness',
    ],
    predicates: [{ name: 'CannotFormDisjunctive', args: ['abstractGenus'] }],
    relations: [
      { predicate: 'prevents', from: 'abstractGenus', to: 'disjunctiveJudgment' },
    ],
  },
  {
    id: 'j-nes-op-32-disjunctive-categorical-external',
    chunkId: 'j-nes-32-disjunctive-categorical-external',
    label: 'Categorical judgment — genus external to subject',
    clauses: [
      'categoricalJudgment.genus = abstractForm',
      'genus.notNecessarily = proximateGenus',
      'genus.external = toSubject',
    ],
    predicates: [{ name: 'GenusExternal', args: ['categoricalJudgment'] }],
    relations: [
      { predicate: 'external', from: 'genus', to: 'subject' },
    ],
  },
  {
    id: 'j-nes-op-33-disjunctive-concrete-genus',
    chunkId: 'j-nes-33-disjunctive-concrete-genus',
    label: 'Concrete genus as proximate genus',
    clauses: [
      'when genus = concreteEssentiallyDeterminedUniversality',
      'genus = simpleDeterminateness',
      'genus = unityOfMomentsOfConcept',
      'moments.sublated = inSimplicity',
      'moments.realDifference = inSpecies',
      'genus = proximateGenusOfSpecies',
      'species.possesses = specificDifferenceInEssentialDeterminateness',
      'species.determination = differentiatingInNatureOfGenus',
    ],
    predicates: [{ name: 'IsConcreteProximateGenus', args: ['genus'] }],
    relations: [
      { predicate: 'is', from: 'concreteGenus', to: 'proximateGenus' },
    ],
  },
  {
    id: 'j-nes-op-34-disjunctive-determinateness-aspect',
    chunkId: 'j-nes-34-disjunctive-determinateness-aspect',
    label: 'Identity from aspect of determinateness',
    clauses: [
      'identity = ofSubjectAndPredicate',
      'aspect = determinatenessInGeneral',
      'aspect.positedBy = hypotheticalJudgment',
      'necessity = identityOfImmediateAndDiverse',
      'necessity = negativeUnity',
    ],
    predicates: [{ name: 'IdentityFromDeterminateness', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'hypotheticalJudgment', to: 'aspect' },
    ],
  },
  {
    id: 'j-nes-op-35-disjunctive-negative-unity-differentiated',
    chunkId: 'j-nes-35-disjunctive-negative-unity-differentiated',
    label: 'Negative unity differentiated — subject and predicate',
    clauses: [
      'negativeUnity.separates = subjectAndPredicate',
      'negativeUnity.posited = asDifferentiated',
      'differentiatedIn = {subject:simpleDeterminateness, predicate:totality}',
      'parting = differenceOfConcept',
      'totalityOfSpecies = difference',
    ],
    predicates: [{ name: 'NegativeUnityDifferentiated', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'differentiates', from: 'negativeUnity', to: 'subjectPredicate' },
    ],
  },
  {
    id: 'j-nes-op-36-disjunctive-concept-disjoins',
    chunkId: 'j-nes-36-disjunctive-concept-disjoins',
    label: 'Concept disjoins itself — negative unity',
    clauses: [
      'reciprocalDetermination = given',
      'reducesTo = differenceOfConcept',
      'concept.disjoins = itself',
      'concept.manifests = negativeUnityInDetermination',
    ],
    predicates: [{ name: 'ConceptDisjoinsItself', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'disjoins', from: 'concept', to: 'itself' },
    ],
  },
  {
    id: 'j-nes-op-37-disjunctive-species-conceptual-determinateness',
    chunkId: 'j-nes-37-disjunctive-species-conceptual-determinateness',
    label: 'Species as simple conceptual determinateness',
    clauses: [
      'species.considered = underAspectOfSimpleConceptualDeterminateness',
      'species.notConsidered = accordingToShapeOfFurtherReality',
      'reality.dropped = inSimplePrincipleOfGenus',
      'essentialDifferentiation = momentOfConcept',
    ],
    predicates: [{ name: 'SpeciesAsConceptualDeterminateness', args: ['species'] }],
    relations: [
      { predicate: 'is', from: 'species', to: 'conceptualDeterminateness' },
    ],
  },
  {
    id: 'j-nes-op-38-disjunctive-concept-progressive-determination',
    chunkId: 'j-nes-38-disjunctive-concept-progressive-determination',
    label: 'Concept\'s own progressive determination',
    clauses: [
      'disjunction = conceptOwnProgressiveDetermination',
      'concept.posits = itsDisjunction',
      'justAs = conceptDeterminedInAndForItself',
      'concept.differentiated = intoDeterminateConcepts',
    ],
    predicates: [{ name: 'ConceptProgressiveDetermination', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'posits', from: 'concept', to: 'disjunction' },
    ],
  },
  {
    id: 'j-nes-op-39-disjunctive-concept-as-member',
    chunkId: 'j-nes-39-disjunctive-concept-as-member',
    label: 'Concept as universal is itself a disjunctive member',
    clauses: [
      'concept = universal',
      'concept = positiveTotalityOfParticulars',
      'concept = negativeTotalityOfParticulars',
      'concept = oneOfDisjunctiveMembers',
      'otherMember = universalityResolvedIntoParticularity',
      'otherMember = determinatenessAsDeterminateness',
      'universality.displays = asTotality',
    ],
    predicates: [{ name: 'ConceptAsDisjunctiveMember', args: ['concept'] }],
    relations: [
      { predicate: 'is', from: 'concept', to: 'disjunctiveMember' },
    ],
  },
  {
    id: 'j-nes-op-40-disjunctive-form-validation',
    chunkId: 'j-nes-40-disjunctive-form-validation',
    label: 'Disjunction form validation — concept determinateness',
    clauses: [
      'if disjunction.notAttained = thisForm',
      'then proof = disjunctionNotRisenToConceptDeterminateness',
      'then proof = disjunctionNotProceededFromConcept',
    ],
    predicates: [{ name: 'ValidatesDisjunctionForm', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'validates', from: 'form', to: 'conceptDeterminateness' },
    ],
  },
  {
    id: 'j-nes-op-41-disjunctive-color-example',
    chunkId: 'j-nes-41-disjunctive-color-example',
    label: 'Color example — empirical disjunction as barbarism',
    clauses: [
      'example = "Color is either violet, indigo, blue, green, yellow, orange, or red"',
      'confusionAndImpurity = atOnceApparent',
      'disjunction = barbarism',
    ],
    predicates: [{ name: 'IllustratesEmpiricalDisjunction', args: ['colorExample'] }],
    relations: [
      { predicate: 'illustrates', from: 'colorExample', to: 'empiricalDisjunction' },
    ],
  },
  {
    id: 'j-nes-op-42-disjunctive-color-principle',
    chunkId: 'j-nes-42-disjunctive-color-principle',
    label: 'Color as concrete unity — principle of differentiation',
    clauses: [
      'if color = concreteUnityOfLightAndDarkness',
      'then genus.has = determinateness',
      'determinateness = principleOfParticularization',
      'oneSpecies = utterlySimpleColor',
      'simpleColor.holds = oppositionInBalance',
      'simpleColor.containedNegated = inColorIntensity',
      'relation = oppositionOfLightAndDarkness',
      'relation = naturalPhenomenon',
      'neutrality = indifferentNeutralityOfOpposition',
    ],
    predicates: [{ name: 'ColorAsConcreteUnity', args: ['color'] }],
    relations: [
      { predicate: 'has', from: 'genus', to: 'principleOfDifferentiation' },
    ],
  },
  {
    id: 'j-nes-op-43-disjunctive-warning-empirical',
    chunkId: 'j-nes-43-disjunctive-warning-empirical',
    label: 'Warning against improper genera and empirical mixtures',
    clauses: [
      'improperGenus = {mixtures, shadesOfDifference}',
      'examples = {violet, orange, indigoBlue, lightBlue}',
      'procedure = totallyInconsiderate',
      'procedure.shows = tooLittleReflection',
    ],
    predicates: [{ name: 'WarnsAgainstImproperGenera', args: [] }],
    relations: [
      { predicate: 'warns', from: 'system', to: 'improperGenera' },
    ],
  },
  {
    id: 'j-nes-op-44-disjunctive-subject-predicate-members',
    chunkId: 'j-nes-44-disjunctive-subject-predicate-members',
    label: 'Subject and predicate as members of disjunction',
    clauses: [
      'disjunctiveJudgment.hasMembers = inPredicate',
      'judgment.itselfDisjoined = true',
      'subjectAndPredicate = membersOfDisjunction',
      'subjectAndPredicate = momentsOfConcept',
      'moments.posited = inDeterminateness',
      'moments.identical = true',
      'identity = {objectiveUniversality, negativeUnity}',
      'objectiveUniversality = {subject:simpleGenus, predicate:universalSphereTotality}',
      'negativeUnity = developedConnectednessOfNecessity',
      'simpleDeterminateness.fallenApart = intoDifferenceOfSpecies',
      'species.have = essentialConnectionAndSelfIdentity',
    ],
    predicates: [{ name: 'SubjectPredicateAsMembers', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'are', from: 'subjectPredicate', to: 'disjunctiveMembers' },
    ],
  },
  {
    id: 'j-nes-op-45-disjunctive-copula-concept',
    chunkId: 'j-nes-45-disjunctive-copula-concept',
    label: 'Copula becomes the concept — judgment of the concept',
    clauses: [
      'unity = copulaOfJudgment',
      'extremes.comeTogether = throughIdentity',
      'unity = conceptItself',
      'unity = conceptAsPosited',
      'judgmentOfNecessity.risen = toJudgmentOfConcept',
    ],
    predicates: [{ name: 'CopulaBecomesConcept', args: ['disjunctiveJudgment'] }],
    relations: [
      { predicate: 'becomes', from: 'copula', to: 'concept' },
      { predicate: 'rises', from: 'judgmentOfNecessity', to: 'judgmentOfConcept' },
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
