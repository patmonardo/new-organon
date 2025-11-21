import type { Chunk, LogicalOperation } from './index';

/**
 * B. THE SYLLOGISM OF REFLECTION
 *
 * NOTE: Hegel uses "Inference" (Schluss) as the broader term encompassing
 * all syllogistic forms. "Syllogism" is one species of inference.
 *
 * ALGORITHM AS SYLLOGISTIC INFERENCE:
 * The Algorithm here is Syllogistic Inference — but really of Mere Validity
 * and not Science (in the cultic sense). This is the inference of the understanding,
 * not the absolute concept. It establishes validity through external reflection,
 * not through immanent necessity.
 *
 * EMPIRICISTS AS ABSOLUTISTS:
 * So-called "empiricists" (Newton, Einstein) are really Absolutists — they
 * presuppose universal laws as immediately given, treating induction and analogy
 * as if they yield absolute truth rather than mere validity. They mistake
 * the syllogism of reflection (mere validity) for the syllogism of necessity (science).
 *
 * Structure:
 * Introduction: Transition from qualitative syllogism to reflection
 * a. The Syllogism of Allness (S-P-U)
 * b. The Syllogism of Induction (U-S-P)
 * c. The Syllogism of Analogy (S-U-P)
 *
 * Transition: Reflection passes over into Necessity (S-U-P schema)
 */

// ============================================================================
// INTRODUCTION: THE SYLLOGISM OF REFLECTION — TRANSITION FROM QUALITATIVE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'sr-intro-1-qualitative-sublation',
    title: 'Qualitative syllogism sublated — terms become concrete',
    text: `The course of the qualitative syllogism has sublated the abstractness of its terms; the syllogistic term has thus posited itself as a determinateness in which also the other determinateness shines reflectively. Besides the abstract terms, there is also present in the syllogism the connection of the terms, and in the conclusion this connection is posited as one which is mediated and necessary; in truth, therefore, each determinateness is posited, not singly by itself, but with reference to the others, as concrete determinateness.`,
  },
  {
    id: 'sr-intro-2-middle-as-totality',
    title: 'Middle term posited as totality of determinations',
    text: `The middle term was the abstract particularity, an isolated simple determinateness, and was a middle only externally and relative to the self-subsisting extremes. This term is now posited as the totality of the determinations; thus it is the posited unity of the extremes; but this unity is at first that of a reflection embracing the extremes within itself; an embracing which, as a first sublating of immediacy and a first connecting of the determinations, is not yet the absolute identity of the concept.`,
  },
  {
    id: 'sr-intro-3-extremes-as-reflection-determinations',
    title: 'Extremes as determinations of judgment of reflection',
    text: `The extremes are the determinations of the judgment of reflection, singularity proper, and universality as a determination of relation, or a reflection that embraces a manifold within itself. But, as was shown in connection with the judgment of reflection, the singular subject also contains, besides the mere singularity that belongs to form, determinateness as universality absolutely reflected into itself, as presupposed, that is, here still immediately assumed, genus.`,
  },
  {
    id: 'sr-intro-4-middle-content-from-extremes',
    title: 'Middle content results from extremes — threefold structure',
    text: `From this determinateness of the extremes, which belongs to the course of the determination of the judgment, there results the more precise content of the middle, which is what counts most in the syllogism, for it is the middle that distinguishes the syllogism from judgment. The middle contains (1) singularity; (2) but singularity expanded into universality, as an "all"; (3) the universality that lies at the basis, uniting singularity and abstract universality in itself, the genus.`,
  },
  {
    id: 'sr-intro-5-first-genuine-determinateness',
    title: 'First syllogism with genuine determinateness of form',
    text: `The syllogism of reflection is thus the first to possess genuine determinateness of form, for the middle is posited as the totality of determinations; the immediate syllogism is by contrast indeterminate because the middle is still only abstract particularity in which the moments of its concept are not yet posited. This first syllogism of reflection may be called the syllogism of allness.`,
  },
];

// ============================================================================
// a. THE SYLLOGISM OF ALLNESS (S-P-U)
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'sr-all-1-overview',
    title: 'Allness — understanding in its perfection',
    text: `The syllogism of allness is the syllogism of the understanding in its perfection but more than that it is not yet. That the middle in it is not abstract particularity but is developed into its moments and is therefore concrete, is indeed an essential requirement of the concept. But at first the form of the allness gathers the singular into universality only externally, and conversely the singular behaves in the universality still as an immediate that subsists on its own.`,
  },
  {
    id: 'sr-all-2-first-negation-only',
    title: 'First negation only — not negation of negation',
    text: `The negation of the immediacy of the determinations which was the result of the syllogism of existence is only the first negation, not yet the negation of the negation, or absolute immanent reflection. The singular determinations that the universality of reflection holds within still lie, therefore, at the basis of that universality; in other words, allness is not yet the universality of the concept, but the external universality of reflection.`,
  },
  {
    id: 'sr-all-3-contingency-resolved',
    title: 'Contingency resolved by concrete middle',
    text: `The syllogism of existence was contingent because its middle term was one single determinateness of the concrete subject and as such admitted of a multitude of other such middle terms, and consequently the subject could be conjoined in conclusion with an indeterminate number of other predicates, with opposite predicates as well. But since the middle term now contains singularity and is thereby itself concrete, only a predicate that concretely belongs to the subject can be attached to the latter by means of it.`,
  },
  {
    id: 'sr-all-4-example-green-regularity',
    title: 'Example: green/regularity — abstract vs concrete',
    text: `For instance, if from the middle term "green" the conclusion is made to follow that a painting is pleasing, because green is pleasing to the eye, or if a poem, a building, etc., is said to be beautiful because it possesses regularity, the painting, the poem, the building, etc., may nonetheless still be ugly on account of other determinations from which this predicate "ugly" might be deduced. By contrast, when the middle term has the determination of allness, it contains the green, the regularity, as a concreted term which for that very reason is not the abstraction of a mere green, a mere regular, etc.; only predicates commensurate with concrete totality may now be attached to this concreted term.`,
  },
  {
    id: 'sr-all-5-judgment-vs-proposition',
    title: 'Judgment vs proposition — abstraction vs concrete totality',
    text: `In the judgment, "what is green or regular is pleasing," the subject is only the abstraction of green, regularity; in the proposition, "all things green or regular are pleasing," the subject is on the contrary all actual concrete things that are green or regular things, therefore, that are intended as concreted with all the properties that they may also have besides the green or the regularity.`,
  },
  {
    id: 'sr-all-6-reflective-perfection-illusion',
    title: 'Reflective perfection as illusion',
    text: `However, this very reflective perfection of the syllogism makes of it a mere illusion. The middle term has the determinateness of "all," to which there is immediately attached in the major the predicate which in the conclusion is then conjoined with the subject. But the "all" is "all singulars"; in it, therefore, the subject already possesses that predicate immediately; it does not first obtain it by means of the syllogistic inference.`,
  },
  {
    id: 'sr-all-7-major-presupposes-conclusion',
    title: 'Major premise presupposes its conclusion',
    text: `Or again, the subject obtains a predicate as a consequence through the conclusion; but the major premise already contains this conclusion in it; therefore the major premise is not correct on its own account, or is not an immediately presupposed judgment, but itself already presupposes the conclusion of which it should be the ground.`,
  },
  {
    id: 'sr-all-8-example-gaius',
    title: 'Example — Gaius and mortality',
    text: `In the much cited syllogism: All humans are mortal, Now Gaius is a human, Therefore Gaius is mortal, the major premise is correct only because and to the extent that the conclusion is correct; were Gaius by chance not mortal, the major premise would not be correct. The proposition which was supposed to be the conclusion must be correct on its own, immediately, for otherwise the major premise would not include all singulars; before the major premise can be accepted as correct, the antecedent question is whether the conclusion may not be a counter-instance of it.`,
  },
  {
    id: 'sr-all-9-presupposition-posited',
    title: 'Presupposition posited in syllogism itself',
    text: `It followed from the concept of the syllogism, with regard to the syllogism of existence, that the premises, as immediate, contradicted the conclusion, that is to say, contradicted the mediation that the concept of the syllogism requires; that the first syllogism thus presupposed other syllogisms, and conversely these presupposed the first. In the syllogism of reflection this result is posited in the syllogism itself: the major premise presupposes its conclusion, for it contains the union of the singular with a predicate that would have to be a conclusion first.`,
  },
  {
    id: 'sr-all-10-empty-semblance',
    title: 'Empty reflective semblance of inference',
    text: `What we have here in fact can therefore be expressed by saying that the syllogism of reflection is only an external, empty reflective semblance of syllogistic inference; that therefore the essence of the inference rests on subjective singularity; this singularity thus constitutes the middle term and is to be posited as such: singularity which is singularity as such and possesses universality only externally.`,
  },
  {
    id: 'sr-all-11-singular-immediate-connection',
    title: 'Singular connected immediately to predicate',
    text: `Or what has been shown on closer inspection of the content of the syllogism of reflection is that the singular stands connected to its predicate immediately, not by way of an inference, and that the major premise, the union of a particular with a universal, or more precisely of a formal universal with a universal in itself, is mediated through the connection of the singularity that is present in the formal universal, of singularity as allness. But this is the syllogism of induction.`,
  },
);

// ============================================================================
// b. THE SYLLOGISM OF INDUCTION (U-S-P)
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'sr-ind-1-overview',
    title: 'Induction — overview (schema U‑S‑P / second-figure reflection)',
    text: `The syllogism of allness comes under the schema of the first figure, S-P-U; the syllogism of induction under that of the second, U-S-P, because it again has singularity for its middle term, not abstract singularity but singularity as completed, that is to say, posited with its opposite determination, that of universality. The one extreme is some predicate or other which is common to all these singulars; its connection with them makes up the kind of immediate premises, of which one was supposed to be the conclusion in the preceding syllogism.`,
  },
  {
    id: 'sr-ind-2-genus-exhausted',
    title: 'Other extreme: immediate genus exhausted by singulars',
    text: `The other extreme may be the immediate genus, as it is in the middle term of the preceding syllogism, or in the subject of the universal judgment, and which is exhausted in the collection of singulars or also species of the middle term. Accordingly, the syllogism has this configuration: s, s, U -- P, s, s, ad infinitum.`,
  },
  {
    id: 'sr-ind-3-psu-mismatch',
    title: 'P-S-U mismatch and completed middle',
    text: `The second figure of the formal syllogism, P-S-U, does not correspond to this schema, because the S that constitutes the middle term did not subsume or was not a predicate. In induction this deficiency is eliminated; here the middle term is "all singulars"; the proposition, U-S, which contains as the subject the objective universal or the genus set apart as an extreme, has a predicate which is of at least equal extension as the subject and is consequently identical with it for external reflection.`,
  },
  {
    id: 'sr-ind-4-equality-extension',
    title: 'Equality of extension — indifferent formal determination',
    text: `Lion, elephant, etc., constitute the genus of quadruped; the difference, that the same content is posited once in singularity and again in universality, is thus just an indifferent determination of form an indifference which in the syllogism of reflection is the posited result of the formal syllogism and is posited here through the equality of extension.`,
  },
  {
    id: 'sr-ind-5-experience-not-contingent',
    title: 'Induction as experience (not mere perception)',
    text: `Induction, therefore, is not the syllogism of mere perception or of contingent existence, like the second figure corresponding to it, but the syllogism of experience; of the subjective gathering together of singulars in the genus, and of the conjoining of the genus with a universal determinateness on the ground that the latter is found in all singulars.`,
  },
  {
    id: 'sr-ind-6-objective-significance-inner',
    title: 'Objective significance: inner concept not yet posited',
    text: `It also has the objective significance that the immediate genus has determined itself through the totality of singularity as a universal property and possesses its existence in a universal relation or mark. But the objective significance of this syllogism, as it was of the others, is at first only its inner concept, and is not as yet posited in it.`,
  },
  {
    id: 'sr-ind-7-subjective-nature',
    title: 'Induction remains essentially subjective',
    text: `On the contrary, induction is essentially still a subjective syllogism. The middle terms are the singulars in their immediacy, the collecting of them into a genus through the allness is an external reflection. Because of the persisting immediacy of the singulars and because of the externality that derives from it, the universality is only completeness, or rather, it remains a task.`,
  },
  {
    id: 'sr-ind-8-bad-infinity',
    title: 'Bad infinity and problematic conclusion',
    text: `In induction, therefore, there recurs the progression into the bad infinity; singularity ought to be posited as identical with universality, but since the singulars are equally posited as immediate, the intended unity remains only a perpetual ought; it is a unity of likeness; the terms which are supposed to be identical are at the same time supposed not to be identical. The a, b, c, d, e, constitute the genus only further on, in the infinite; they do not yield a complete experience. The conclusion of induction thus remains problematic.`,
  },
  {
    id: 'sr-ind-9-presupposition-genus',
    title: 'Induction presupposes genus conjoined with determinateness',
    text: `But induction, by expressing that perception, in order to become experience, ought to be carried on to infinity, presupposes that the genus is in and for itself conjoined with its determinateness. In this, it in fact rather presupposes its conclusion as something immediate, just as the syllogism of allness presupposes the conclusion for one of its premises.`,
  },
  {
    id: 'sr-ind-10-experience-validity',
    title: 'Experience validity — true in and for itself',
    text: `An experience that rests on induction is assumed as valid even though the perception is admittedly not complete; it may be assumed, however, that there is no counter-instance to the experience only if the latter is true in and for itself. Inference by induction, therefore, is based indeed on an immediacy, but not on the immediacy on which it is supposed to be based, not on a singularity that exists immediately, but on one that exists in and for itself, on the universal.`,
  },
  {
    id: 'sr-ind-11-middle-splits',
    title: 'Middle term splits if singularity essential, universality external',
    text: `The fundamental character of induction is that it is a syllogistic inference; if singularity is taken as the essential determination of the middle term, but universality as only the external determination, then the middle term would fall apart into two disjoined parts, and there would be no inference; this externality belongs rather to the extremes.`,
  },
  {
    id: 'sr-ind-12-analogy-as-truth',
    title: 'Analogy: singularity immediately identical with universality',
    text: `Singularity can only be a middle term if immediately identical with the universality; such a universality is in truth objective universality, the genus. The matter can also be viewed in this way: universality is external but essential to the determination of the singularity which is at the basis of the middle term of induction; such an external is just as much immediately its opposite, the internal. The truth of the syllogism of induction is therefore a syllogism that has for its middle term a singularity which is immediately in itself universality. This is the syllogism of analogy.`,
  },
);

// ============================================================================
// c. THE SYLLOGISM OF ANALOGY (S-U-P)
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'sr-ana-1-overview',
    title: 'Analogy — overview (schema S-U-P)',
    text: `This syllogism has the third figure of the immediate syllogism, S-U-P, for its abstract schema. But its middle term is no longer some single quality or other but a universality which is the immanent reflection of a concreted term and is therefore its nature; and conversely, since it is thus the universality of a concreted term, it is at the same time in itself this concreted term.`,
  },
  {
    id: 'sr-ana-2-middle-singular-universal',
    title: 'Middle: singular taken in its universal nature',
    text: `Here, therefore, a singular is the middle term, but a singular taken in its universal nature; there is moreover another singular, an extreme term, which has the same universal nature as the other which is the middle term.`,
  },
  {
    id: 'sr-ana-3-example-earth-moon',
    title: 'Example: earth / moon inhabitants',
    text: `For example: The earth has inhabitants, The moon is an earth, Therefore the moon has inhabitants.`,
  },
  {
    id: 'sr-ana-4-superficiality-quality',
    title: 'Superficiality when universal is mere quality',
    text: `Analogy is all the more superficial, the more the universal in which the two extremes are united, and in accordance with which the one extreme becomes the predicate of the other, is a mere quality or, since quality is a matter of subjectivity, is some distinctive mark or other and the identity of the extremes is therein taken as just a similarity. But this kind of superficiality to which a form of understanding or of reason is reduced debased to the sphere of mere representation should have no place in logic.`,
  },
  {
    id: 'sr-ana-5-form-as-content-fallacy',
    title: 'Fallacy: making form the content',
    text: `Also unacceptable is to present the major premise of this syllogism as though it should run: "That which is similar to an object in one distinctive mark is similar to it in other such marks as well." On this formulation, the form of the syllogism is expressed in the shape of a content while the empirical content, the content properly so called, is together relegated to the minor premise.`,
  },
  {
    id: 'sr-ana-6-form-determines-content',
    title: 'Form determining content is necessary advance',
    text: `What might tempt one to this view in regard to the syllogism of analogy, and perhaps in regard to the syllogism of induction too, is that the middle term in them, and also the extremes, are more determined than they are in the merely formal syllogism, and therefore the determinations of form, since they are no longer simple and abstract, must also take on the appearance of a content determination. But that the form determines itself to content is first of all a necessary advance on the part of the formal side, and therefore an advance that touches the nature of the syllogism essentially; secondly, such a content determination cannot, therefore, be regarded as any other empirical content, and abstraction cannot be made from it.`,
  },
  {
    id: 'sr-ana-7-quaternio-terminorum',
    title: 'Quaternio terminorum — apparent four-term problem',
    text: `When we consider the syllogism of analogy with its major premise expressed as above, namely, "if two subject matters agree in one or more properties, then a further property of one also belongs to the other," it may seem that this syllogism contains four terms, the quaternio terminorum, a circumstance that brings with it the difficulty of how to bring analogy into the form of a formal syllogism. There are two singulars; for a third, a property immediately assumed as common, and, for a fourth, the other properties that one singular possesses immediately but the other first comes to possess only by means of the syllogism.`,
  },
  {
    id: 'sr-ana-8-middle-dual-nature',
    title: 'Middle dual nature: singularity + essential universality',
    text: `This is so because, as we have seen, in the syllogism of analogy the middle term is posited as singularity but immediately also as the true universality of the singularity. In induction, the middle term is, apart from the extremes, an indeterminate number of singulars; this syllogism, therefore, required the enumeration of an infinite number of terms. In the syllogism of allness the universality in the middle term is still only the external form determination of the allness; in the syllogism of analogy, on the contrary, it is as essential universality.`,
  },
  {
    id: 'sr-ana-9-earth-as-concrete-universal',
    title: 'Earth as concrete universal — genus and singular',
    text: `In the above example, the middle term, "the earth," is taken as something concrete which, in truth, is just as much a universal nature or genus as it is a singular. From this aspect, the quaternio terminorum would not make analogy an imperfect syllogism.`,
  },
  {
    id: 'sr-ana-10-indeterminacy-general-vs-particular',
    title: 'Indeterminacy: general nature vs particularity',
    text: `But it would make it so from another aspect; for although the one subject has the same universal nature as the other, it is undetermined whether the determinateness, which is inferred to pertain also to the second subject, pertains to the first because of its nature in general or because of its particularity; for example, whether the earth has inhabitants as a heavenly body in general or only as this particular heavenly body.`,
  },
  {
    id: 'sr-ana-11-analogy-still-reflection',
    title: 'Analogy remains reflective — externality persists',
    text: `Analogy is still a syllogism of reflection inasmuch as singularity and universality are united in its middle term immediately. Because of this immediacy, the externality of the unity of reflection is still there; the singular is the genus only in itself, implicitly; it is not posited in this negativity by which its determinateness would be the genus's own determinateness. For this reason the predicate that belongs to the singular of the middle term is not already the predicate of the other singular, even though the two singulars both belong to the one genus.`,
  },
  {
    id: 'sr-ana-12-presupposition-demand',
    title: 'Analogy presupposes its conclusion — demand for mediation',
    text: `S-P ("the moon is inhabited") is the conclusion; but the one premise ("the earth is inhabited") is likewise S-P; in so far as S-P is supposed to be a conclusion, it entails the requirement that that premise also be S-P. This syllogism is thus in itself the demand to counter the immediacy that it contains; or again, it presupposes its conclusion.`,
  },
  {
    id: 'sr-ana-13-sublation-singularity',
    title: 'Demand for sublation of singularity — genus purified',
    text: `Since the syllogism of analogy is therefore the demand that it be mediated as against the immediacy with which its mediation is burdened, what it demands is the sublation of the moment of singularity. Thus there remains for the middle term the objective universal, the genus purified of immediacy. In the syllogism of analogy the genus was a moment of the middle term only as immediate presupposition; since the syllogism itself demands the sublation of the presupposed immediacy, the negation of singularity and hence the universal is no longer immediate but posited.`,
  },
  {
    id: 'sr-ana-14-second-negation',
    title: 'Second negation — universality in and for itself',
    text: `The syllogism of reflection contained the first negation of immediacy; the second has now come on the scene, and with it the external universality of reflection is determined as existing in and for itself. Regarded from the positive side, the conclusion shows itself to be identical with the premises, the mediation to have rejoined its presupposition, and what we have is thus an identity of the universality of reflection by virtue of which it becomes a higher universality.`,
  },
  {
    id: 'sr-ana-15-review-course',
    title: 'Review: mediation as concrete unity — transition to necessity',
    text: `Reviewing the course of the syllogism of reflection, we find that mediation is in general the posited or concrete unity of the form determinations of the extremes; reflection consists in this positing of the one determination in the other; the mediating middle is thus allness. But it is singularity that proves to be the essential ground of mediation while universality is only as an external determination in it, as completeness. But universality is essential to the singular if the latter is to be the conjoining middle term; is therefore to be taken as an implicitly existing universal. But the singular is not united with it in just this positive manner but is sublated in it and is a negative moment; thus the universal is the genus posited as existing in and for itself, and the singular as immediate is rather the externality of the genus, or it is an extreme.`,
  },
  {
    id: 'sr-ana-16-transition-necessity',
    title: 'Transition to syllogism of necessity',
    text: `The syllogism of reflection, taken in general, comes under the schema P-S-U in which the singular is still as such the essential determination of the middle term; but since its immediacy has been sublated, the syllogism has entered under the formal schema S-U-P, and the syllogism of reflection has thus passed over into the syllogism of necessity.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // INTRODUCTION OPERATIONS
  // ============================================================================
  {
    id: 'sr-op-intro-1-qualitative-sublation',
    chunkId: 'sr-intro-1-qualitative-sublation',
    label: 'Qualitative syllogism sublated — terms become concrete',
    clauses: [
      'qualitativeSyllogism.sublated = abstractness',
      'term.positedAs = determinatenessWithReflectiveShine',
      'connection.positedAs = mediatedAndNecessary',
      'eachDeterminateness = concrete (with reference to others)',
    ],
    predicates: [{ name: 'IsSublatedQualitative', args: [] }],
    relations: [
      { predicate: 'transforms', from: 'qualitativeSyllogism', to: 'reflectionSyllogism' },
    ],
  },
  {
    id: 'sr-op-intro-2-middle-totality',
    chunkId: 'sr-intro-2-middle-as-totality',
    label: 'Middle term as totality of determinations',
    clauses: [
      'middle.was = abstractParticularity',
      'middle.now = totalityOfDeterminations',
      'middle.unity = reflectionEmbracingExtremes',
      'unity.notYet = absoluteIdentityOfConcept',
    ],
    predicates: [{ name: 'IsTotalityMiddle', args: ['middle'] }],
    relations: [
      { predicate: 'posits', from: 'middle', to: 'unityOfExtremes' },
    ],
  },
  {
    id: 'sr-op-intro-3-extremes-reflection',
    chunkId: 'sr-intro-3-extremes-as-reflection-determinations',
    label: 'Extremes as reflection determinations',
    clauses: [
      'extremes = {singularity, universalityAsRelation}',
      'singularSubject.contains = {singularity(form), universalityReflectedIntoItself}',
      'universality = presupposedGenus (immediately assumed)',
    ],
    predicates: [{ name: 'AreReflectionDeterminations', args: ['extremes'] }],
    relations: [
      { predicate: 'contains', from: 'singularSubject', to: 'presupposedGenus' },
    ],
  },
  {
    id: 'sr-op-intro-4-middle-threefold',
    chunkId: 'sr-intro-4-middle-content-from-extremes',
    label: 'Middle threefold structure: singularity, all, genus',
    clauses: [
      'middle.contains = {singularity, singularityExpandedToAll, genus}',
      'middle.distinguishes = syllogismFromJudgment',
      'genus.unites = {singularity, abstractUniversality}',
    ],
    predicates: [{ name: 'HasThreefoldStructure', args: ['middle'] }],
    relations: [
      { predicate: 'unites', from: 'genus', to: 'singularityAndAbstractUniversality' },
    ],
  },
  {
    id: 'sr-op-intro-5-genuine-determinateness',
    chunkId: 'sr-intro-5-first-genuine-determinateness',
    label: 'First syllogism with genuine determinateness',
    clauses: [
      'reflectionSyllogism.has = genuineDeterminatenessOfForm',
      'immediateSyllogism.is = indeterminate',
      'firstReflectionSyllogism = allness',
    ],
    predicates: [{ name: 'HasGenuineDeterminateness', args: ['reflectionSyllogism'] }],
    relations: [
      { predicate: 'contrasts', from: 'reflectionSyllogism', to: 'immediateSyllogism' },
    ],
  },
  // ============================================================================
  // ALLNESS OPERATIONS
  // ============================================================================
  {
    id: 'sr-op-1-allness-overview',
    chunkId: 'sr-all-1-overview',
    label: 'Allness: understanding in perfection',
    clauses: [
      'allness = understandingInPerfection',
      'middle.developed = true',
      'middle.isConcrete = true',
      'universality.gathersSingulars = externally',
      'singular.behavesAs = immediate (subsists on own)',
    ],
    predicates: [{ name: 'IsAllness', args: [] }],
    relations: [
      { predicate: 'gathers', from: 'universality', to: 'singulars' },
    ],
  },
  {
    id: 'sr-op-2-first-negation',
    chunkId: 'sr-all-2-first-negation-only',
    label: 'First negation only — not negation of negation',
    clauses: [
      'negationOfImmediacy = firstNegation',
      'notYet = negationOfNegation',
      'notYet = absoluteImmanentReflection',
      'singularDeterminations.lieAtBasis = universality',
      'allness = externalUniversalityOfReflection',
    ],
    predicates: [{ name: 'IsFirstNegation', args: ['allness'] }],
    relations: [
      { predicate: 'underlies', from: 'singularDeterminations', to: 'universality' },
    ],
  },
  {
    id: 'sr-op-3-contingency-resolved',
    chunkId: 'sr-all-3-contingency-resolved',
    label: 'Concrete middle resolves contingency',
    clauses: [
      'existenceSyllogism.was = contingent',
      'allness.middle.contains = singularity',
      'allness.middle.isConcrete = true',
      'onlyConcretePredicates.allowed = true',
    ],
    predicates: [{ name: 'ResolvesContingency', args: ['allness'] }],
    relations: [
      { predicate: 'blocks', from: 'concreteMiddle', to: 'arbitraryPredicates' },
    ],
  },
  {
    id: 'sr-op-4-example-green',
    chunkId: 'sr-all-4-example-green-regularity',
    label: 'Example: green/regularity — abstract vs concrete',
    clauses: [
      'abstractMiddle(green) -> oppositePredicatesPossible',
      'allnessMiddle -> onlyConcreteTotalityPredicates',
      'judgment.subject = abstraction',
      'proposition.subject = concreteTotality',
    ],
    predicates: [{ name: 'IllustratesConcreteness', args: ['example'] }],
    relations: [
      { predicate: 'contrasts', from: 'abstractMiddle', to: 'allnessMiddle' },
    ],
  },
  {
    id: 'sr-op-5-judgment-vs-proposition',
    chunkId: 'sr-all-5-judgment-vs-proposition',
    label: 'Judgment vs proposition contrast',
    clauses: [
      'judgment.subject = abstraction',
      'proposition.subject = concreteThingsWithAllProperties',
    ],
    predicates: [{ name: 'ContrastsAbstraction', args: ['judgment', 'proposition'] }],
    relations: [
      { predicate: 'contrasts', from: 'judgment', to: 'proposition' },
    ],
  },
  {
    id: 'sr-op-6-illusion',
    chunkId: 'sr-all-6-reflective-perfection-illusion',
    label: 'Reflective perfection as illusion',
    clauses: [
      'middle = allSingulars',
      'predicate.attachedImmediately = true',
      'subject.possessesPredicate = immediately',
      'inference.notNecessary = true',
    ],
    predicates: [{ name: 'IsReflectiveIllusion', args: ['allness'] }],
    relations: [
      { predicate: 'containsImmediately', from: 'middle', to: 'predicate' },
    ],
  },
  {
    id: 'sr-op-7-major-presupposes',
    chunkId: 'sr-all-7-major-presupposes-conclusion',
    label: 'Major premise presupposes conclusion',
    clauses: [
      'major.contains = conclusion',
      'major.notCorrectOnOwn = true',
      'major.presupposes = conclusion',
    ],
    predicates: [{ name: 'MajorPresupposesConclusion', args: ['major'] }],
    relations: [
      { predicate: 'presupposes', from: 'major', to: 'conclusion' },
    ],
  },
  {
    id: 'sr-op-8-gaius-example',
    chunkId: 'sr-all-8-example-gaius',
    label: 'Gaius example: major validity depends on conclusion',
    clauses: [
      'major.validity ⇔ conclusion.holds',
      'counterInstance => major.invalid',
      'conclusion.mustBeSecure = independently',
    ],
    predicates: [{ name: 'IllustratesPresupposition', args: ['GaiusExample'] }],
    relations: [
      { predicate: 'dependsOn', from: 'major', to: 'conclusion' },
    ],
  },
  {
    id: 'sr-op-9-presupposition-posited',
    chunkId: 'sr-all-9-presupposition-posited',
    label: 'Presupposition posited in syllogism itself',
    clauses: [
      'existenceSyllogism.presupposed = otherSyllogisms',
      'reflectionSyllogism.presupposition = positedInItself',
      'major.contains = unionOfSingularWithPredicate',
    ],
    predicates: [{ name: 'HasPositedPresupposition', args: ['reflectionSyllogism'] }],
    relations: [
      { predicate: 'posits', from: 'reflectionSyllogism', to: 'presupposition' },
    ],
  },
  {
    id: 'sr-op-10-empty-semblance',
    chunkId: 'sr-all-10-empty-semblance',
    label: 'Empty reflective semblance',
    clauses: [
      'reflectionSyllogism = externalEmptySemblance',
      'essenceOfInference = subjectiveSingularity',
      'singularity.possessesUniversality = externally',
    ],
    predicates: [{ name: 'IsEmptySemblance', args: ['reflectionSyllogism'] }],
    relations: [
      { predicate: 'restsOn', from: 'inference', to: 'subjectiveSingularity' },
    ],
  },
  {
    id: 'sr-op-11-singular-immediate',
    chunkId: 'sr-all-11-singular-immediate-connection',
    label: 'Singular connected immediately — transition to induction',
    clauses: [
      'singular.connectedToPredicate = immediately',
      'notByWayOf = inference',
      'major.mediatedThrough = singularityAsAllness',
      'this = syllogismOfInduction',
    ],
    predicates: [{ name: 'IsImmediateConnection', args: ['singular', 'predicate'] }],
    relations: [
      { predicate: 'transitions', from: 'allness', to: 'induction' },
    ],
  },
  // ============================================================================
  // INDUCTION OPERATIONS
  // ============================================================================
  {
    id: 'sr-op-12-induction-overview',
    chunkId: 'sr-ind-1-overview',
    label: 'Induction: U-S-P schema',
    clauses: [
      'allness.schema = S-P-U',
      'induction.schema = U-S-P',
      'middle = singularityCompleted',
      'singularity.positedWith = universality',
    ],
    predicates: [{ name: 'IsInduction', args: [] }],
    relations: [
      { predicate: 'implements', from: 'induction', to: 'U-S-P' },
    ],
  },
  {
    id: 'sr-op-13-genus-exhausted',
    chunkId: 'sr-ind-2-genus-exhausted',
    label: 'Genus exhausted by singulars',
    clauses: [
      'otherExtreme = immediateGenus',
      'genus.exhaustedBy = collectionOfSingulars',
      'configuration = s, s, U-P, s, s, adInfinitum',
    ],
    predicates: [{ name: 'IsExhaustedGenus', args: ['genus', 'singulars'] }],
    relations: [
      { predicate: 'exhaustedBy', from: 'genus', to: 'singulars' },
    ],
  },
  {
    id: 'sr-op-14-psu-mismatch',
    chunkId: 'sr-ind-3-psu-mismatch',
    label: 'P-S-U mismatch resolved',
    clauses: [
      'P-S-U.S.didNotSubsume = true',
      'induction.middle = allSingulars',
      'U-S.predicate.extension >= subject.extension',
      'relation = externalIdentity',
    ],
    predicates: [{ name: 'ResolvesPSUMismatch', args: ['induction'] }],
    relations: [
      { predicate: 'resolves', from: 'induction', to: 'PSUMismatch' },
    ],
  },
  {
    id: 'sr-op-15-equality-extension',
    chunkId: 'sr-ind-4-equality-extension',
    label: 'Equality of extension — indifferent form',
    clauses: [
      'sameContent.positedAs = {singularity, universality}',
      'difference = indifferentFormalDetermination',
      'equalityOfExtension = positedResult',
    ],
    predicates: [{ name: 'HasEqualExtension', args: ['subject', 'predicate'] }],
    relations: [
      { predicate: 'grounds', from: 'equalityOfExtension', to: 'formalResult' },
    ],
  },
  {
    id: 'sr-op-16-experience',
    chunkId: 'sr-ind-5-experience-not-contingent',
    label: 'Induction as experience',
    clauses: [
      'induction.is = syllogismOfExperience',
      'not = merePerception',
      'not = contingentExistence',
      'subjectiveGathering = singularsIntoGenus',
    ],
    predicates: [{ name: 'IsSyllogismOfExperience', args: ['induction'] }],
    relations: [
      { predicate: 'gathers', from: 'induction', to: 'singularsIntoGenus' },
    ],
  },
  {
    id: 'sr-op-17-objective-inner',
    chunkId: 'sr-ind-6-objective-significance-inner',
    label: 'Objective significance as inner concept',
    clauses: [
      'genus.determinesItselfVia = totalityOfSingularity',
      'objectiveSignificance = innerConcept',
      'notYetPosited = true',
    ],
    predicates: [{ name: 'IsInnerObjectiveSignificance', args: ['genus'] }],
    relations: [
      { predicate: 'awaits', from: 'innerConcept', to: 'positedRealization' },
    ],
  },
  {
    id: 'sr-op-18-subjective',
    chunkId: 'sr-ind-7-subjective-nature',
    label: 'Induction remains subjective',
    clauses: [
      'middleTerms = singularsInImmediacy',
      'collectingIntoGenus = externalReflection',
      'universality = completenessTask',
    ],
    predicates: [{ name: 'IsSubjectiveSyllogism', args: ['induction'] }],
    relations: [
      { predicate: 'remains', from: 'induction', to: 'subjective' },
    ],
  },
  {
    id: 'sr-op-19-bad-infinity',
    chunkId: 'sr-ind-8-bad-infinity',
    label: 'Bad infinity and problematic conclusion',
    clauses: [
      'singularity.oughtToBe = identicalWithUniversality',
      'singulars.positedAs = immediate',
      'unity.remains = perpetualOught',
      'unity = unityOfLikeness',
      'genus.formedOnlyIn = infinite',
      'conclusion = problematic',
    ],
    predicates: [{ name: 'ExhibitsBadInfinity', args: ['induction'] }],
    relations: [
      { predicate: 'renders', from: 'finiteInstances', to: 'insufficientGround' },
    ],
  },
  {
    id: 'sr-op-20-presupposition-genus',
    chunkId: 'sr-ind-9-presupposition-genus',
    label: 'Induction presupposes genus conjoined',
    clauses: [
      'induction.presupposes = genusConjoinedWithDeterminateness',
      'presupposesConclusion = asImmediate',
      'sameAs = allnessPresupposition',
    ],
    predicates: [{ name: 'PresupposesConclusion', args: ['induction'] }],
    relations: [
      { predicate: 'presupposes', from: 'induction', to: 'genusConjoined' },
    ],
  },
  {
    id: 'sr-op-21-experience-validity',
    chunkId: 'sr-ind-10-experience-validity',
    label: 'Experience validity — true in and for itself',
    clauses: [
      'experience.assumedValid = despiteIncompletePerception',
      'noCounterInstance => if trueInAndForItself',
      'immediacy.basedOn = universal (not isolated singular)',
    ],
    predicates: [{ name: 'IsValidExperience', args: ['induction'] }],
    relations: [
      { predicate: 'dependsOn', from: 'inference', to: 'universalImmediacy' },
    ],
  },
  {
    id: 'sr-op-22-middle-splits',
    chunkId: 'sr-ind-11-middle-splits',
    label: 'Middle splits if singularity essential, universality external',
    clauses: [
      'if singularity.essential ∧ universality.external then middle.splits',
      'if middle.splits then noInference',
      'externality.belongsTo = extremes',
    ],
    predicates: [{ name: 'WouldSplit', args: ['middle'] }],
    relations: [
      { predicate: 'prevents', from: 'splitMiddle', to: 'inference' },
    ],
  },
  {
    id: 'sr-op-23-analogy-truth',
    chunkId: 'sr-ind-12-analogy-as-truth',
    label: 'Analogy: singularity immediately identical with universality',
    clauses: [
      'singularity.middleTerm => immediatelyIdenticalWithUniversality',
      'universality = objectiveUniversality (genus)',
      'external.is = immediatelyInternal',
      'truthOfInduction = analogy',
    ],
    predicates: [{ name: 'IsAnalogy', args: ['syllogism'] }],
    relations: [
      { predicate: 'transforms', from: 'induction', to: 'analogy' },
    ],
  },
  // ============================================================================
  // ANALOGY OPERATIONS
  // ============================================================================
  {
    id: 'sr-op-24-analogy-overview',
    chunkId: 'sr-ana-1-overview',
    label: 'Analogy: S-U-P schema',
    clauses: [
      'schema = S-U-P',
      'middle.not = singleQuality',
      'middle = universalityAsImmanentReflection',
      'universality = natureOfConcretedTerm',
      'universality = concretedTermItself',
    ],
    predicates: [{ name: 'IsAnalogySchema', args: [] }],
    relations: [
      { predicate: 'implements', from: 'analogy', to: 'S-U-P' },
    ],
  },
  {
    id: 'sr-op-25-middle-singular-universal',
    chunkId: 'sr-ana-2-middle-singular-universal',
    label: 'Middle: singular in universal nature',
    clauses: [
      'middle = singularInUniversalNature',
      'otherExtreme = singularWithSameUniversalNature',
    ],
    predicates: [{ name: 'IsSingularUniversalMiddle', args: ['middle'] }],
    relations: [
      { predicate: 'shares', from: 'otherExtreme', to: 'universalNature' },
    ],
  },
  {
    id: 'sr-op-26-earth-moon',
    chunkId: 'sr-ana-3-example-earth-moon',
    label: 'Earth/moon example',
    clauses: [
      'premise1: earth.has(inhabitants)',
      'premise2: moon.is(earth)',
      'conclusion: moon.has(inhabitants)',
    ],
    predicates: [{ name: 'IllustratesAnalogy', args: ['earthMoonExample'] }],
    relations: [
      { predicate: 'derives', from: 'premises', to: 'conclusion' },
    ],
  },
  {
    id: 'sr-op-27-superficiality',
    chunkId: 'sr-ana-4-superficiality-quality',
    label: 'Superficiality when universal is quality',
    clauses: [
      'if universal.isQuality then analogy.isSuperficial',
      'identity.takenAs = similarity',
      'reducedTo = mereRepresentation',
    ],
    predicates: [{ name: 'IsSuperficialAnalogy', args: ['analogy'] }],
    relations: [
      { predicate: 'reducesTo', from: 'analogy', to: 'representation' },
    ],
  },
  {
    id: 'sr-op-28-form-content-fallacy',
    chunkId: 'sr-ana-5-form-as-content-fallacy',
    label: 'Form as content fallacy',
    clauses: [
      'major.asFormContent => empiricalContent.relegatedToMinor',
      'form.expressedAs = content',
    ],
    predicates: [{ name: 'IsFormContentFallacy', args: [] }],
    relations: [
      { predicate: 'violates', from: 'formAsContent', to: 'logicalRequirement' },
    ],
  },
  {
    id: 'sr-op-29-form-determines',
    chunkId: 'sr-ana-6-form-determines-content',
    label: 'Form determining content is necessary advance',
    clauses: [
      'form.determinesContent = necessaryAdvance',
      'contentDetermination.not = arbitraryEmpirical',
      'abstraction.cannotBeMade = fromContentDetermination',
    ],
    predicates: [{ name: 'IsFormDeterminedContent', args: ['form'] }],
    relations: [
      { predicate: 'qualifies', from: 'form', to: 'contentDetermination' },
    ],
  },
  {
    id: 'sr-op-30-quaternio',
    chunkId: 'sr-ana-7-quaternio-terminorum',
    label: 'Quaternio terminorum — four terms',
    clauses: [
      'terms = {s1, s2, commonProperty, inferredProperties}',
      'appearsAs = fourTerms',
      'difficulty = reduceToFormalSyllogism',
    ],
    predicates: [{ name: 'HasQuaternioAspect', args: ['analogy'] }],
    relations: [
      { predicate: 'appearsAs', from: 'analogy', to: 'fourTerms' },
    ],
  },
  {
    id: 'sr-op-31-middle-dual',
    chunkId: 'sr-ana-8-middle-dual-nature',
    label: 'Middle dual nature: singularity + essential universality',
    clauses: [
      'analogy.middle = singularity ∧ essentialUniversality',
      'induction.middle = indeterminateSingulars',
      'allness.middle = externalFormUniversality',
      'analogy.middle = essentialUniversality',
    ],
    predicates: [{ name: 'HasDualNature', args: ['middle'] }],
    relations: [
      { predicate: 'combines', from: 'middle', to: 'singularityAndUniversality' },
    ],
  },
  {
    id: 'sr-op-32-earth-concrete',
    chunkId: 'sr-ana-9-earth-as-concrete-universal',
    label: 'Earth as concrete universal',
    clauses: [
      'earth.is = universalNature (genus)',
      'earth.is = singular',
      'quaternio.not = imperfectSyllogism (from this aspect)',
    ],
    predicates: [{ name: 'IsConcreteUniversal', args: ['earth'] }],
    relations: [
      { predicate: 'is', from: 'earth', to: 'genusAndSingular' },
    ],
  },
  {
    id: 'sr-op-33-indeterminacy',
    chunkId: 'sr-ana-10-indeterminacy-general-vs-particular',
    label: 'Indeterminacy: general vs particular',
    clauses: [
      'propertySource = {generalNature | particularity}',
      'undetermined = whichSource',
      'quaternio.makesImperfect = from this aspect',
    ],
    predicates: [{ name: 'IsSourceIndeterminate', args: ['inferredProperty'] }],
    relations: [
      { predicate: 'weakens', from: 'sourceIndeterminacy', to: 'conclusion' },
    ],
  },
  {
    id: 'sr-op-34-analogy-reflection',
    chunkId: 'sr-ana-11-analogy-still-reflection',
    label: 'Analogy remains reflective',
    clauses: [
      'singularity.unitedWithUniversality = immediately',
      'externality.persists = true',
      'singular.isGenus = implicitly',
      'predicate.notAutomatic = for other singular',
    ],
    predicates: [{ name: 'IsReflectiveRemnant', args: ['analogy'] }],
    relations: [
      { predicate: 'warns', from: 'sharedGenus', to: 'predicateNotGuaranteed' },
    ],
  },
  {
    id: 'sr-op-35-presupposition-demand',
    chunkId: 'sr-ana-12-presupposition-demand',
    label: 'Analogy presupposes conclusion — demand for mediation',
    clauses: [
      'conclusion = S-P',
      'premise = S-P',
      'syllogism.demands = counterImmediacy',
      'syllogism.presupposes = conclusion',
    ],
    predicates: [{ name: 'PresupposesConclusion', args: ['analogy'] }],
    relations: [
      { predicate: 'demands', from: 'analogy', to: 'mediation' },
    ],
  },
  {
    id: 'sr-op-36-sublation-singularity',
    chunkId: 'sr-ana-13-sublation-singularity',
    label: 'Demand for sublation of singularity',
    clauses: [
      'demand = sublateSingularity',
      'middleTerm.remains = objectiveUniversal',
      'genus = purifiedOfImmediacy',
      'universal = posited (not immediate)',
    ],
    predicates: [{ name: 'DemandsSublation', args: ['analogy'] }],
    relations: [
      { predicate: 'sublates', from: 'analogy', to: 'singularity' },
    ],
  },
  {
    id: 'sr-op-37-second-negation',
    chunkId: 'sr-ana-14-second-negation',
    label: 'Second negation — universality in and for itself',
    clauses: [
      'reflectionSyllogism.contained = firstNegation',
      'secondNegation.now = present',
      'universality.determinedAs = existingInAndForItself',
      'conclusion.identicalWith = premises',
      'universality.becomes = higherUniversality',
    ],
    predicates: [{ name: 'IsSecondNegation', args: ['analogy'] }],
    relations: [
      { predicate: 'transforms', from: 'secondNegation', to: 'higherUniversality' },
    ],
  },
  {
    id: 'sr-op-38-review-course',
    chunkId: 'sr-ana-15-review-course',
    label: 'Review: mediation as concrete unity',
    clauses: [
      'mediation = concreteUnityOfFormDeterminations',
      'reflection = positingOneDeterminationInOther',
      'mediatingMiddle = allness',
      'essentialGround = singularity',
      'universality = externalDetermination (completeness)',
      'singular.sublatedIn = universal',
      'universal = genusInAndForItself',
    ],
    predicates: [{ name: 'IsConcreteUnity', args: ['mediation'] }],
    relations: [
      { predicate: 'grounds', from: 'singularity', to: 'mediation' },
    ],
  },
  {
    id: 'sr-op-39-transition-necessity',
    chunkId: 'sr-ana-16-transition-necessity',
    label: 'Transition to syllogism of necessity',
    clauses: [
      'reflectionSyllogism.schema = P-S-U',
      'singularity.immediacy.sublated = true',
      'syllogism.enteredUnder = S-U-P',
      'reflection.passedOver = necessity',
    ],
    predicates: [{ name: 'TransitionsToNecessity', args: ['reflectionSyllogism'] }],
    relations: [
      { predicate: 'transitions', from: 'reflectionSyllogism', to: 'necessitySyllogism' },
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
