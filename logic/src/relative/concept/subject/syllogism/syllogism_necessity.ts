import type { Chunk, LogicalOperation } from './index';

/**
 * C. THE SYLLOGISM OF NECESSITY
 *
 * NOTE: This concludes the first of nine components of Hegel's Logic — the foundational
 * 3+3+3 parts. This gets us to the End of the Qualitative Logic, which is foundational
 * to the next level Quantitative Logic.
 *
 * The Syllogism of Necessity represents the completion of the syllogism as mediation,
 * where the concept realizes itself as objectivity. The middle term is now objective
 * universality (genus), containing the whole determinateness of the extremes.
 *
 * Structure:
 * Introduction: Middle determined as objective universality (genus)
 * a. The categorical syllogism (S-P-U)
 * b. The hypothetical syllogism (U-S-P)
 * c. The disjunctive syllogism (S-U-P)
 *
 * Transition: Sublation to objectivity — the concept realized as fact in and for itself
 *
 * PHILOSOPHICAL NOTES:
 *
 * **Ground as Laws of Logic and Science (Pure Algorithm)**:
 * Ground completes Pure Reason — Ground grounds the Syllogism. Ground is the Laws of
 * Logic and Science, the Pure Algorithm itself. The Determinations of Reflection
 * (Identity, Difference, Contradiction) are the Laws of Logic.
 *
 * **Syllogism as Enclosure into Truth**:
 * The Syllogism of Necessity is the enclosure into Truth — where Ground (Science)
 * realizes itself as Truth. This is the Enclosed Pure Concept with Determinate Concept.
 *
 * **The EnclosedConcept and Pipeline of Learning**:
 * The EnclosedConcept (determinate, no longer Pure Concept) enters the Pipeline
 * of Learning. This is the representation that another representation is within me —
 * self-reference as the structure of learning.
 *
 * **Qualitative Syllogism and Qualitative Oneness**:
 * The Syllogism of Necessity is a Qualitative Syllogism — it carries the Qualitative
 * Oneness up through the Pipeline of Objectivity to reach the Idea of the True.
 *
 * **Invariants as Qualitative Oneness**:
 * Invariants (in formal verification, loop invariants, class invariants, etc.) are
 * Qualitative Oneness — what remains constant, what preserves identity through
 * transformation. They are what carries the "oneness" through the process, ensuring
 * that the qualitative structure is preserved as the system moves through the Pipeline
 * of Objectivity. Invariants are the Qualitative Oneness made explicit in code.
 *
 * **The Complete Structure**:
 * Ground (Laws of Logic/Science) → Syllogism (Enclosure into Truth) →
 * EnclosedConcept (Determinate) → Pipeline of Learning → Objectivity →
 * Idea of the True
 *
 * This is the LogoGenesis completed — Ground (Science) encloses itself in the
 * Syllogism (Truth), which enters the Pipeline of Learning (Objectivity) to
 * reach the Idea of the True. Invariants are the Qualitative Oneness that carries
 * this structure through the pipeline.
 */

// ============================================================================
// INTRODUCTION: THE SYLLOGISM OF NECESSITY — MIDDLE AS OBJECTIVE UNIVERSALITY
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'sn-intro-1-middle-determined',
    title: 'Middle determined as objective universality (genus)',
    text: `The mediating middle has now determined itself (1) as simple determinate universality, like the particularity in the syllogism of existence, but (2) as objective universality, that is to say, one that, like the allness of the syllogism of reflection, contains the whole determinateness of the different extremes; this is a completed but simple universality, the universal nature of the fact, the genus.`,
  },
  {
    id: 'sn-intro-2-full-content',
    title: 'Syllogism full of content — difference reflected into identity',
    text: `This syllogism is full of content, because the abstract middle term of the syllogism of existence has posited itself to be determinate difference, in the way it is as the middle term of the syllogism of reflection, but this difference has again reflected itself into simple identity.`,
  },
  {
    id: 'sn-intro-3-immanent-reflection',
    title: 'Middle as immanent reflection — necessity',
    text: `This syllogism is for this reason the syllogism of necessity, because its middle term is not any adventitious immediate content but is the immanent reflection of the determinateness of the extremes. These have their inner identity in the middle term, whose content determinations are the form determinations of the extremes.`,
  },
  {
    id: 'sn-intro-4-moments-necessary-existence',
    title: 'Terms as moments of necessary existence',
    text: `Consequently, what differentiates the terms is a form which is external and unessential and the terms themselves are as moments of a necessary existence.`,
  },
  {
    id: 'sn-intro-5-immediate-formal',
    title: 'At first immediate and formal — realization task',
    text: `This syllogism is at first immediate and formal in the sense that what holds the terms together is the essential nature, as content, and this content is in the distinguished terms only in different form, and the extremes are by themselves only an unessential subsistence. The realization of this syllogism is a matter of determining it in such a way that the extremes are equally posited as this totality which initially the middle term is, and the necessity of the connection, which is at first only the substantial content, shall be a connection of the posited form.`,
  },
];

// ============================================================================
// a. THE CATEGORICAL SYLLOGISM (S-P-U)
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'sn-cat-1-overview',
    title: 'Categorical syllogism — first syllogism of necessity',
    text: `The categorical syllogism has the categorical judgment for one or for both of its premises. Associated with this syllogism, just as with that judgment, is the more specific signification that its middle term is the objective universality. Superficially, the categorical syllogism is also taken for nothing more than a mere syllogism of inherence.`,
  },
  {
    id: 'sn-cat-2-subject-predicate-through-substance',
    title: 'Subject conjoined with predicate through substance',
    text: `Taken in its full import, the categorical syllogism is the first syllogism of necessity, one in which a subject is conjoined with a predicate through its substance. But when elevated to the sphere of the concept, substance is the universal, so posited to be in and for itself that it has for its form or mode of being, not accidentality, as it has in the relation specific to it, but the determination of the concept.`,
  },
  {
    id: 'sn-cat-3-extremes-universality-singularity',
    title: 'Extremes: universality and singularity',
    text: `Its differences are therefore the extremes of the syllogism, specifically universality and singularity. This universality, as contrasted with the genus that more closely defines the middle term, is abstract or is a universal determinateness: it is the accidentality of substance summed up in a simple determinateness which is, however, the substance's essential difference, its specific difference. Singularity, for its part, is the actual, in itself the concrete unity of genus and determinateness though here, in the immediate syllogism, it is immediate singularity at first, accidentality summed up in the form of a subsistence existing for itself.`,
  },
  {
    id: 'sn-cat-4-both-premises-categorical',
    title: 'Both premises are categorical judgments',
    text: `The connection of this extreme term to the middle term constitutes a categorical judgment; but since the other extreme term also, as just determined, expresses the specific difference of the genus or its determinate principle, this other premise is also categorical.`,
  },
  {
    id: 'sn-cat-5-schema-spu',
    title: 'Schema S-P-U — no contingency',
    text: `This syllogism, as the first and therefore immediate syllogism of necessity, comes in the first instance under the schema of the formal syllogism, S-P-U. But since the middle term is the essential nature of the singular and not just one or other of its determinacies or properties, and likewise the extreme of universality is not any abstract universal, nor just any singular quality either, but is rather the universal determinateness of the genus, its specific difference, we no longer have the contingency of a subject being conjoined with just any quality through just any middle term.`,
  },
  {
    id: 'sn-cat-6-no-infinite-progression',
    title: 'No infinite progression — no external immediacy',
    text: `Consequently, since the connections of the extremes with the middle term also do not have the external immediacy that they have in the syllogism of existence, we do not have coming into play the demand for proof in the sense in which it occurred in the case of that other syllogism and led to an infinite progression.`,
  },
  {
    id: 'sn-cat-7-no-presupposition',
    title: 'Does not presuppose conclusion — connection of identity',
    text: `Further, this syllogism does not presuppose its conclusion for its premises, as in the syllogism of reflection. The terms, in keeping with the substantial content, stand to one another in a connection of identity that exists in and for itself; we have here one essence running through the three terms; an essence in which the determinations of singularity, particularity, and universality are only formal moments.`,
  },
  {
    id: 'sn-cat-8-objectivity-begins',
    title: 'Objectivity begins — no longer subjective',
    text: `To this extent, therefore, the categorical syllogism is no longer subjective; in that connection of identity, objectivity begins; the middle term is the identity, full of content, of its extremes, and these are contained in it in their self-subsistence, for their self-subsistence is the said substantial universality which is the genus. The subjective element of the syllogism consists in the indifferent subsistence of the extremes with respect to the concept or the middle term.`,
  },
  {
    id: 'sn-cat-9-subjective-element-remains',
    title: 'Subjective element remains — substantial identity not form identity',
    text: `But there is still a subjective element in this syllogism, for that identity is still the substantial identity or content but is not yet identity of form at the same time. The identity of the concept still is an inner bond and therefore, as connection, still necessity; the universality of the middle term is solid, positive identity, but is not equally the negativity of its extremes.`,
  },
  {
    id: 'sn-cat-10-singular-contingency',
    title: 'Singular as immediate — contingency of subsumption',
    text: `The immediacy of this syllogism, which is not yet posited as what it is in itself, is more precisely present in this way. The truly immediate element of the syllogism is the singular. This singular is subsumed under its genus as middle term; but subsumed under the same genus are also an indeterminate number of many other singulars; it is therefore contingent that only this singular is posited as subsumed under it.`,
  },
  {
    id: 'sn-cat-11-singular-posited-contingent',
    title: 'Singular posited as contingent — subjective actuality',
    text: `But further, this contingency does not belong only to an external reflection that finds the singular posited in the syllogism to be contingent by comparison with others; on the contrary, it is because the singular is itself connected to the middle term as its objectivity universality that it is posited as contingent, as a subjective actuality.`,
  },
  {
    id: 'sn-cat-12-extremes-indifferent',
    title: 'Extremes have indifferent immediacy',
    text: `From the other side, because the subject is an immediate singular, it contains determinations that are not contained in the middle term as the universal nature; it also has, therefore, a concrete existence which is indifferent to the middle term, determined for itself and with a content of its own. Therefore, conversely, this other term also has an indifferent immediacy and a concrete existence distinct from the former. The same relation also obtains between the middle term and the other extreme; for this too likewise has the determination of immediacy, hence of a being which is contingent with respect to the middle term.`,
  },
  {
    id: 'sn-cat-13-transition-hypothetical',
    title: 'Transition to hypothetical syllogism',
    text: `Accordingly, what is posited in the categorical syllogism are on the one hand, extremes that are so related to the middle term that they have objective universality or self-subsistent nature in themselves, and are at the same time immediate actualities, hence indifferent to one another. On the other hand, they are equally contingent, or their immediacy is as sublated in their identity. But this identity, because of the self-subsistence and totality of the actuality, is only formal, inner identity, and the syllogism of necessity has thereby determined itself to the hypothetical syllogism.`,
  },
);

// ============================================================================
// b. THE HYPOTHETICAL SYLLOGISM (U-S-P)
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'sn-hyp-1-judgment-structure',
    title: 'Hypothetical judgment — necessary connection without immediacy',
    text: `The hypothetical judgment contains only the necessary connection without the immediacy of the connected terms. "If A is, so is B"; or, the being of A is also just as much the being of an other, of the B; with this, it is not as yet said either that A is, or that B is.`,
  },
  {
    id: 'sn-hyp-2-syllogism-adds-immediacy',
    title: 'Hypothetical syllogism adds immediacy',
    text: `The hypothetical syllogism adds this immediacy of being: If A is, so is B, But A is, Therefore B is. The minor premise expresses by itself the immediate being of the A. But it is not only this that is added to the judgment. The conclusion contains the connection of subject and predicate, not as the abstract copula, but as the accomplished mediating unity.`,
  },
  {
    id: 'sn-hyp-3-being-a-as-middle',
    title: 'Being of A as middle term',
    text: `The being of the A is to be taken, therefore, not as mere immediacy but essentially as middle term of the syllogism. This needs closer examination.`,
  },
  {
    id: 'sn-hyp-4-connection-necessity',
    title: 'Connection as necessity — inner substantial identity',
    text: `In the first place, the connection of the hypothetical judgment is the necessity or the inner substantial identity associated with the external diversity of concrete existence; an identical content lying internally as its basis. The two sides of the judgment are both, therefore, not an immediate being, but a being held in necessity, hence one which is at the same time sublated or only being as appearance.`,
  },
  {
    id: 'sn-hyp-5-sides-universality-singularity',
    title: 'Sides as universality and singularity — indifference',
    text: `The two behave, moreover, as sides of the judgment, as universality and singularity; the one, therefore, is the above content as totality of determinations, the other as actuality. Yet it is a matter of indifference which side is taken as universality and which as singularity. That is to say, inasmuch as the conditions are still the inner, abstract element of an actuality, they are the universal, and it is by being held together in one singularity that they step into actuality. Conversely, the conditions are a dismembered and dispersed appearance that gains unity and meaning, and a universally valid existence, only in actuality.`,
  },
  {
    id: 'sn-hyp-6-condition-vs-cause',
    title: 'Condition vs cause/ground — condition more universal',
    text: `The relation that is here being assumed between the two sides of condition and conditioned may however also be taken to be one of cause and effect, ground and consequence. This is a matter of indifference here. The relation of condition, however, corresponds more closely to the one that obtains in the hypothetical judgment and syllogism inasmuch as condition is essentially an indifferent concrete existence, whereas ground and cause are inherently a transition; moreover, condition is a more universal condition in that it comprehends both sides of the relation, since effect, consequence, etc., are just as much the condition of cause and ground as these are the condition of them.`,
  },
  {
    id: 'sn-hyp-7-a-mediating-being',
    title: 'A as mediating being — immediacy and self-sublation',
    text: `Now A is the mediating being in so far as it is, first, an immediate being, an indifferent actuality, but, second, in so far as it is equally inherently contingent, self-sublating being. What translates the conditions into the actuality of the new shape of which they are the conditions is the fact that they are not being as an abstract immediacy, but being according to its concept becoming in the first instance, but more determinedly (since the concept is no longer transition) singularity as self-referring negative unity.`,
  },
  {
    id: 'sn-hyp-8-negativity-mediating-means',
    title: 'Negativity as mediating means — activity',
    text: `The conditions are a dispersed material awaiting and requiring application; this negativity is the mediating means, the free unity of the concept. It determines itself as activity, for this middle term is the contradiction of objective universality, or of the totality of the identical content and the indifferent immediacy. This middle term is no longer, therefore, merely inner but existent necessity; the objective universality contains its self-reference as simple immediacy, as being.`,
  },
  {
    id: 'sn-hyp-9-conclusion-identity',
    title: 'Conclusion — mediated immediacy and identity',
    text: `The conclusion, "therefore B is," expresses the same contradiction, that B exists immediately but at the same time through an other or as mediated. According to its form, it is therefore the same concept that the middle term is, distinguished from necessity only as the necessary, in the totally superficial form of singularity as contrasted with universality. The absolute content of A and B is the same; for ordinary representation, they are two different names for the same basic thing, since representation fixes the appearances of the diversified shape of existence and distinguishes the necessary from its necessity; but to the extent that necessity were to be separated from B, the latter would not be the necessary. What we have here, therefore, is the identity of the mediating term and the mediated.`,
  },
  {
    id: 'sn-hyp-10-form-negative-unity',
    title: 'Form as negative unity — first display',
    text: `The hypothetical syllogism is the first to display the necessary connection as a connectedness through form or negative unity, just as the categorical syllogism displays it through positive unity, the solid content, the objective universality. But necessity merges with the necessary; the form-activity of translating the conditioning actuality into the conditioned is in itself the unity into which the determinacies of the oppositions previously let free into indifferent existence are sublated, and where the difference of A and B is an empty name.`,
  },
  {
    id: 'sn-hyp-11-unity-posited',
    title: 'Unity posited — identical content',
    text: `The unity is therefore a unity reflected into itself, and hence an identical content, and is this content not only implicitly in itself but, through this syllogism, it is also posited, for the being of A is also not its own being but that of B and vice versa, and in general the being of the one is the being of the other and, as determined in the conclusion, their immediate being or indifferent determinateness is a mediated one; therefore, their externality has been sublated, and what is posited is their unity withdrawn into itself.`,
  },
  {
    id: 'sn-hyp-12-transition-disjunctive',
    title: 'Transition to disjunctive syllogism',
    text: `The mediation of the syllogism has thereby determined itself as singularity, immediacy, and self-referring negativity, or as a differentiating identity that retrieves itself into itself out of this differentiation, as absolute form, and for that very reason as objective universality, self-identical existent content. In this determination, the syllogism is the disjunctive syllogism.`,
  },
);

// ============================================================================
// c. THE DISJUNCTIVE SYLLOGISM (S-U-P)
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'sn-dis-1-schema',
    title: 'Disjunctive syllogism — schema S-U-P',
    text: `As the hypothetical syllogism comes in general under the schema of the second figure of the formal syllogism, U-S-P, so the disjunctive comes under the schema of the third, S-U-P. The middle term, however, is a universality replete with form; it has determined itself as totality, as developed objective universality.`,
  },
  {
    id: 'sn-dis-2-middle-totality',
    title: 'Middle term as totality — universality, particularity, singularity',
    text: `The middle term, therefore, is universality as well as particularity and singularity. As that universality, it is in the first place the substantial identity of the genus, but this identity is secondly one in which particularity is included, but again, included as equal to it therefore as a universal sphere that contains its total particularity, the genus sorted out in its species, an A which is B as well as C and D.`,
  },
  {
    id: 'sn-dis-3-particularization-either-or',
    title: 'Particularization as differentiation — either-or (negative unity)',
    text: `But particularization is differentiation and as such equally the either-or of B, C, D negative unity, the reciprocal exclusion of the determinations. This excluding, moreover, is now not just reciprocal, the determination not merely relative, but is also just as much self-referring determination, the particular as singularity to the exclusion of the others.`,
  },
  {
    id: 'sn-dis-4-forms-examples',
    title: 'Disjunctive forms — examples',
    text: `A is either B or C or D, But A is B, Therefore A is neither C nor D. Or also: A is either B or C or D, But A is neither C nor D, Therefore A is B.`,
  },
  {
    id: 'sn-dis-5-subject-across-premises',
    title: 'Subject A across premises and conclusion',
    text: `A is subject not only in the two premises but also in the conclusion. It is a universal in the first premise and in its predicate the universal sphere particularized in the totality of its species; in the second premise, it is as a determinate, or as a species; in the conclusion it is posited as the excluding, singular determinateness. Or again, in the minor it is already exclusive singularity, and in the conclusion it is positively posited as the determinate that it is.`,
  },
  {
    id: 'sn-dis-6-mediator-unity',
    title: 'Mediator = unity of mediator and mediated',
    text: `Consequently, what as such appears to be meditated is the universality of A with the singularity. But the mediating means is this A which is the universal sphere of its particularizations and is determined as a singular. What is posited in the disjunctive syllogism is thus the truth of the hypothetical syllogism, the unity of the mediator and the mediated, and for that reason the disjunctive syllogism is equally no longer a syllogism at all.`,
  },
  {
    id: 'sn-dis-7-middle-contains-extremes',
    title: 'Middle term contains extremes — extremes as positedness',
    text: `For the middle term which is posited in it as the totality of the concept itself contains the two extremes in their complete determinateness. The extremes, as distinct from this middle term, are only a positedness to which there no longer accrues any proper determinateness of its own as against the middle term.`,
  },
  {
    id: 'sn-dis-8-form-content-identity',
    title: 'Form identical with content',
    text: `If we consider the matter with narrower reference to the hypothetical syllogism, we find that there was in the latter a substantial identity as the inner bond of necessity, and a negative unity distinct from it, namely the activity or the form that translated one existence into another. The disjunctive syllogism is in general in the determination of universality, its middle term is the A as genus and as perfectly determined; also posited through this unity is the earlier inner content and, conversely, the positedness or the form is not the external negative unity over against an indifferent existence but is identical with that solid content. The whole form determination of the concept is posited in its determinate difference and at the same time in the simple identity of the concept.`,
  },
  {
    id: 'sn-dis-9-formalism-sublated',
    title: 'Formalism and subjectivity sublated',
    text: `In this way the formalism of the syllogistic inference, and consequently the subjectivity of the syllogism and of the concept in general, has sublated itself. This formal or subjective factor consisted in that the middle mediating the extremes is the concept as an abstract determination and is therefore distinct from the terms whose unity it is.`,
  },
  {
    id: 'sn-dis-10-distinction-fallen-away',
    title: 'Distinction of mediating and mediated has fallen away',
    text: `In the completion of the syllogism, where the objective universality is equally posited as the totality of the form determinations, the distinction of mediating and mediated has on the contrary fallen away. That which is mediated is itself an essential moment of what mediates it, and each moment is the totality of what is mediated.`,
  },
  {
    id: 'sn-dis-11-figures-middle',
    title: 'Figures exhibit determinateness as middle — concept as ought',
    text: `The figures of the syllogism exhibit each determinateness of the concept singly as the middle term, a middle term which is at the same time the concept as an ought, the requirement that the mediating factor be the concept's totality. The different genera of the syllogism exhibit instead the stages in the repletion or concretion of the middle term.`,
  },
  {
    id: 'sn-dis-12-repletion-concretion',
    title: 'Repletion/concretion across genera',
    text: `In the formal syllogism the middle is posited as totality only through all the determinacies, but each singly, discharging the function of mediation. In the syllogism of reflection, the middle term is the unity gathering together externally the determinations of the extremes. In the syllogism of necessity the middle has determined itself as a unity which is just as developed and total as it is simple, and the form of the syllogism, which consisted in the difference of the middle term over against its extremes, has thereby sublated itself.`,
  },
  {
    id: 'sn-dis-13-concept-realized',
    title: 'Concept realized as objectivity',
    text: `With this the concept in general has been realized; more precisely, it has gained the kind of reality which is objectivity. The first reality was that the concept, in itself negative unity, partitions itself and as judgment posits its determinations in determinate and indifferent difference, and in the syllogism it then sets itself over against them.`,
  },
  {
    id: 'sn-dis-14-inwardness-externality',
    title: 'Inwardness and externality equated',
    text: `Since it is still in this way the inwardness of this now acquired externality, in the course of the syllogisms this externality is equated with the inner unity; the different determinations return into the latter through the mediation that unites them at first in a third term, and as a result the externality exhibits, in itself, the concept which, for its part, is no longer distinct from it as inner unity.`,
  },
  {
    id: 'sn-dis-15-syllogism-mediation',
    title: 'Syllogism as mediation — complete concept posited',
    text: `Conversely, however, that determinateness of the concept which was considered as reality is equally a positedness. For the identity of the concept's inwardness and externality has been exhibited as the truth of the concept not only in this result; on the contrary, already in the judgment the moments of the concept remain, even in their reciprocal indifference, determinations that have significance only in their connection. The syllogism is mediation, the complete concept in its positedness.`,
  },
  {
    id: 'sn-dis-16-sublation-result',
    title: 'Sublation of mediation — result as objectivity',
    text: `Its movement is the sublation of this mediation in which nothing is in and for itself, but each thing is only through the meditation of an other. The result is therefore an immediacy that has emerged through the sublation of the mediation, a being which is equally identical with mediation and is the concept that has restored itself out of, and in, its otherness.`,
  },
  {
    id: 'sn-dis-17-objectivity',
    title: 'Result: objectivity — fact in and for itself',
    text: `This being is therefore a fact which is in and for itself: objectivity.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // INTRODUCTION OPERATIONS
  // ============================================================================
  {
    id: 'sn-op-intro-1-middle-determined',
    chunkId: 'sn-intro-1-middle-determined',
    label: 'Middle determined as objective universality (genus)',
    clauses: [
      'middle.determinedAs = {simpleDeterminateUniversality, objectiveUniversality}',
      'objectiveUniversality.contains = wholeDeterminatenessOfExtremes',
      'middle = completedSimpleUniversality (genus)',
    ],
    predicates: [{ name: 'IsObjectiveUniversality', args: ['middle'] }],
    relations: [
      { predicate: 'contains', from: 'middle', to: 'wholeDeterminateness' },
    ],
  },
  {
    id: 'sn-op-intro-2-full-content',
    chunkId: 'sn-intro-2-full-content',
    label: 'Syllogism full of content — difference reflected into identity',
    clauses: [
      'existenceMiddle.positedAs = determinateDifference',
      'reflectionMiddle = determinateDifference',
      'difference.reflectedInto = simpleIdentity',
    ],
    predicates: [{ name: 'IsFullOfContent', args: ['necessitySyllogism'] }],
    relations: [
      { predicate: 'reflects', from: 'difference', to: 'simpleIdentity' },
    ],
  },
  {
    id: 'sn-op-intro-3-immanent-reflection',
    chunkId: 'sn-intro-3-immanent-reflection',
    label: 'Middle as immanent reflection — necessity',
    clauses: [
      'middle.not = adventitiousImmediateContent',
      'middle = immanentReflectionOfExtremesDeterminateness',
      'extremes.haveInnerIdentity = inMiddleTerm',
      'middle.contentDeterminations = formDeterminationsOfExtremes',
    ],
    predicates: [{ name: 'IsImmanentReflection', args: ['middle'] }],
    relations: [
      { predicate: 'reflects', from: 'middle', to: 'extremesDeterminateness' },
    ],
  },
  {
    id: 'sn-op-intro-4-moments-necessary',
    chunkId: 'sn-intro-4-moments-necessary-existence',
    label: 'Terms as moments of necessary existence',
    clauses: [
      'differentiation = externalUnessentialForm',
      'terms = momentsOfNecessaryExistence',
    ],
    predicates: [{ name: 'AreMomentsOfNecessaryExistence', args: ['terms'] }],
    relations: [
      { predicate: 'are', from: 'terms', to: 'momentsOfNecessaryExistence' },
    ],
  },
  {
    id: 'sn-op-intro-5-realization-task',
    chunkId: 'sn-intro-5-immediate-formal',
    label: 'Realization task — extremes as totality',
    clauses: [
      'syllogism.atFirst = immediateAndFormal',
      'essentialNature.holdsTogether = terms',
      'realizationTask = extremesPositedAsTotality',
      'necessity.shallBe = connectionOfPositedForm',
    ],
    predicates: [{ name: 'RequiresRealization', args: ['necessitySyllogism'] }],
    relations: [
      { predicate: 'requires', from: 'syllogism', to: 'realization' },
    ],
  },
  // ============================================================================
  // CATEGORICAL SYLLOGISM OPERATIONS
  // ============================================================================
  {
    id: 'sn-op-cat-1-overview',
    chunkId: 'sn-cat-1-overview',
    label: 'Categorical syllogism — first syllogism of necessity',
    clauses: [
      'categoricalSyllogism.has = categoricalJudgmentPremises',
      'middleTerm = objectiveUniversality',
      'superficially = syllogismOfInherence',
    ],
    predicates: [{ name: 'IsCategoricalSyllogism', args: [] }],
    relations: [
      { predicate: 'has', from: 'categoricalSyllogism', to: 'categoricalPremises' },
    ],
  },
  {
    id: 'sn-op-cat-2-subject-predicate-substance',
    chunkId: 'sn-cat-2-subject-predicate-through-substance',
    label: 'Subject conjoined with predicate through substance',
    clauses: [
      'categoricalSyllogism = firstSyllogismOfNecessity',
      'subject.conjoinedWith = predicate',
      'conjunction.through = substance',
      'substance.elevatedToConcept = universal',
      'substance.modeOfBeing = determinationOfConcept',
    ],
    predicates: [{ name: 'ConjoinsThroughSubstance', args: ['categoricalSyllogism'] }],
    relations: [
      { predicate: 'conjoins', from: 'substance', to: 'subjectAndPredicate' },
    ],
  },
  {
    id: 'sn-op-cat-3-extremes',
    chunkId: 'sn-cat-3-extremes-universality-singularity',
    label: 'Extremes: universality and singularity',
    clauses: [
      'extremes = {universality, singularity}',
      'universality = abstractUniversalDeterminateness (specificDifference)',
      'singularity = concreteUnity(genus, determinateness)',
      'singularity.atFirst = immediateSingularity',
    ],
    predicates: [{ name: 'AreExtremes', args: ['universality', 'singularity'] }],
    relations: [
      { predicate: 'contrasts', from: 'universality', to: 'singularity' },
    ],
  },
  {
    id: 'sn-op-cat-4-both-premises',
    chunkId: 'sn-cat-4-both-premises-categorical',
    label: 'Both premises are categorical',
    clauses: [
      'extreme.connectsToMiddle = categoricalJudgment',
      'otherExtreme.expresses = specificDifference',
      'bothPremises = categorical',
    ],
    predicates: [{ name: 'AreCategoricalPremises', args: ['premises'] }],
    relations: [
      { predicate: 'makes', from: 'extremeConnection', to: 'categoricalPremise' },
    ],
  },
  {
    id: 'sn-op-cat-5-no-contingency',
    chunkId: 'sn-cat-5-schema-spu',
    label: 'Schema S-P-U — no contingency',
    clauses: [
      'schema = S-P-U',
      'middle = essentialNatureOfSingular',
      'universalityExtreme = universalDeterminatenessOfGenus',
      'noContingency = true',
    ],
    predicates: [{ name: 'HasNoContingency', args: ['categoricalSyllogism'] }],
    relations: [
      { predicate: 'eliminates', from: 'middle', to: 'contingency' },
    ],
  },
  {
    id: 'sn-op-cat-6-no-infinite-progression',
    chunkId: 'sn-cat-6-no-infinite-progression',
    label: 'No infinite progression',
    clauses: [
      'connections.notHave = externalImmediacy',
      'noDemandForProof = true',
      'noInfiniteProgression = true',
    ],
    predicates: [{ name: 'AvoidsInfiniteProgression', args: ['categoricalSyllogism'] }],
    relations: [
      { predicate: 'avoids', from: 'categoricalSyllogism', to: 'infiniteProgression' },
    ],
  },
  {
    id: 'sn-op-cat-7-connection-identity',
    chunkId: 'sn-cat-7-no-presupposition',
    label: 'Connection of identity — one essence',
    clauses: [
      'doesNotPresuppose = conclusion',
      'terms.standIn = connectionOfIdentity',
      'oneEssence.runsThrough = threeTerms',
      'essence.contains = {singularity, particularity, universality}',
    ],
    predicates: [{ name: 'HasConnectionOfIdentity', args: ['categoricalSyllogism'] }],
    relations: [
      { predicate: 'runsThrough', from: 'oneEssence', to: 'threeTerms' },
    ],
  },
  {
    id: 'sn-op-cat-8-objectivity-begins',
    chunkId: 'sn-cat-8-objectivity-begins',
    label: 'Objectivity begins',
    clauses: [
      'categoricalSyllogism.noLonger = subjective',
      'objectivity.begins = true',
      'middleTerm = identityFullOfContent',
      'extremes.containedIn = middleTerm',
      'selfSubsistence = substantialUniversality (genus)',
    ],
    predicates: [{ name: 'BeginsObjectivity', args: ['categoricalSyllogism'] }],
    relations: [
      { predicate: 'begins', from: 'categoricalSyllogism', to: 'objectivity' },
    ],
  },
  {
    id: 'sn-op-cat-9-subjective-remains',
    chunkId: 'sn-cat-9-subjective-element-remains',
    label: 'Subjective element remains',
    clauses: [
      'identity.still = substantialIdentity (content)',
      'identity.notYet = identityOfForm',
      'conceptIdentity = innerBond',
      'middleUniversality.not = negativityOfExtremes',
    ],
    predicates: [{ name: 'HasSubjectiveElement', args: ['categoricalSyllogism'] }],
    relations: [
      { predicate: 'remains', from: 'subjectiveElement', to: 'categoricalSyllogism' },
    ],
  },
  {
    id: 'sn-op-cat-10-singular-contingency',
    chunkId: 'sn-cat-10-singular-contingency',
    label: 'Singular as immediate — contingency',
    clauses: [
      'trulyImmediate = singular',
      'singular.subsumedUnder = genus',
      'manySingulars.subsumedUnder = sameGenus',
      'choiceOfSingular = contingent',
    ],
    predicates: [{ name: 'IsContingentChoice', args: ['singular'] }],
    relations: [
      { predicate: 'renders', from: 'subsumption', to: 'contingency' },
    ],
  },
  {
    id: 'sn-op-cat-11-singular-subjective',
    chunkId: 'sn-cat-11-singular-posited-contingent',
    label: 'Singular posited as contingent — subjective actuality',
    clauses: [
      'singular.connectedToMiddle = asObjectivityUniversality',
      'singular.positedAs = contingent',
      'singular = subjectiveActuality',
    ],
    predicates: [{ name: 'IsSubjectiveActuality', args: ['singular'] }],
    relations: [
      { predicate: 'posits', from: 'connection', to: 'singularAsContingent' },
    ],
  },
  {
    id: 'sn-op-cat-12-extremes-indifferent',
    chunkId: 'sn-cat-12-extremes-indifferent',
    label: 'Extremes have indifferent immediacy',
    clauses: [
      'subject.contains = determinationsNotInMiddle',
      'subject.has = concreteExistenceIndifferentToMiddle',
      'otherTerm.has = indifferentImmediacy',
      'otherExtreme.has = determinationOfImmediacy',
      'otherExtreme = contingentWithRespectToMiddle',
    ],
    predicates: [{ name: 'HaveIndifferentImmediacy', args: ['extremes'] }],
    relations: [
      { predicate: 'have', from: 'extremes', to: 'indifferentImmediacy' },
    ],
  },
  {
    id: 'sn-op-cat-13-transition',
    chunkId: 'sn-cat-13-transition-hypothetical',
    label: 'Transition to hypothetical syllogism',
    clauses: [
      'extremes.have = objectiveUniversalityAndImmediacy',
      'extremes.indifferent = toOneAnother',
      'extremes.also = contingent',
      'immediacy.sublated = inIdentity',
      'identity.only = formalInnerIdentity',
      'syllogism.determinesTo = hypothetical',
    ],
    predicates: [{ name: 'TransitionsToHypothetical', args: ['categoricalSyllogism'] }],
    relations: [
      { predicate: 'transitions', from: 'categoricalSyllogism', to: 'hypotheticalSyllogism' },
    ],
  },
  // ============================================================================
  // HYPOTHETICAL SYLLOGISM OPERATIONS
  // ============================================================================
  {
    id: 'sn-op-hyp-1-judgment',
    chunkId: 'sn-hyp-1-judgment-structure',
    label: 'Hypothetical judgment — necessary connection without immediacy',
    clauses: [
      'hypotheticalJudgment.contains = necessaryConnection',
      'hypotheticalJudgment.lacks = immediacyOfTerms',
      'being(A) = being(B)',
      'notSaid = {A is, B is}',
    ],
    predicates: [{ name: 'IsHypotheticalJudgment', args: [] }],
    relations: [
      { predicate: 'lacks', from: 'hypotheticalJudgment', to: 'immediacy' },
    ],
  },
  {
    id: 'sn-op-hyp-2-adds-immediacy',
    chunkId: 'sn-hyp-2-syllogism-adds-immediacy',
    label: 'Hypothetical syllogism adds immediacy',
    clauses: [
      'schema = {If A then B; A is; therefore B is}',
      'minor.expresses = immediateBeing(A)',
      'conclusion = accomplishedMediatingUnity',
      'conclusion.not = abstractCopula',
    ],
    predicates: [{ name: 'AddsImmediacy', args: ['hypotheticalSyllogism'] }],
    relations: [
      { predicate: 'adds', from: 'hypotheticalSyllogism', to: 'immediacy' },
    ],
  },
  {
    id: 'sn-op-hyp-3-being-middle',
    chunkId: 'sn-hyp-3-being-a-as-middle',
    label: 'Being of A as middle term',
    clauses: [
      'being(A).not = mereImmediacy',
      'being(A) = middleTermOfSyllogism',
    ],
    predicates: [{ name: 'IsMiddleTerm', args: ['being(A)'] }],
    relations: [
      { predicate: 'functionsAs', from: 'being(A)', to: 'middleTerm' },
    ],
  },
  {
    id: 'sn-op-hyp-4-connection',
    chunkId: 'sn-hyp-4-connection-necessity',
    label: 'Connection as necessity — inner substantial identity',
    clauses: [
      'connection = necessity',
      'connection = innerSubstantialIdentity',
      'identicalContent.liesInternally = asBasis',
      'sides.not = immediateBeing',
      'sides = beingHeldInNecessity',
      'sides = beingAsAppearance',
    ],
    predicates: [{ name: 'HasInnerSubstantialIdentity', args: ['connection'] }],
    relations: [
      { predicate: 'holds', from: 'identity', to: 'appearance' },
    ],
  },
  {
    id: 'sn-op-hyp-5-sides',
    chunkId: 'sn-hyp-5-sides-universality-singularity',
    label: 'Sides as universality and singularity — indifference',
    clauses: [
      'sides = {universality, singularity}',
      'one = contentAsTotalityOfDeterminations',
      'other = actuality',
      'indifferent = whichSideIsWhich',
      'conditions = innerAbstractElement',
      'conditions.gainActuality = inSingularity',
    ],
    predicates: [{ name: 'AreSides', args: ['universality', 'singularity'] }],
    relations: [
      { predicate: 'correspondsTo', from: 'conditions', to: 'actuality' },
    ],
  },
  {
    id: 'sn-op-hyp-6-condition-cause',
    chunkId: 'sn-hyp-6-condition-vs-cause',
    label: 'Condition vs cause/ground — condition more universal',
    clauses: [
      'relation.canBeReadAs = {condition/conditioned, cause/effect, ground/consequence}',
      'condition = indifferentConcreteExistence',
      'groundAndCause = inherentlyTransition',
      'condition.comprehends = bothSides',
      'effect.canBeCondition = ofCause',
    ],
    predicates: [{ name: 'IsMoreUniversal', args: ['condition'] }],
    relations: [
      { predicate: 'comprehends', from: 'condition', to: 'bothSides' },
    ],
  },
  {
    id: 'sn-op-hyp-7-mediating-being',
    chunkId: 'sn-hyp-7-a-mediating-being',
    label: 'A as mediating being — immediacy and self-sublation',
    clauses: [
      'A.is = immediateBeing',
      'A.is = indifferentActuality',
      'A.is = contingentSelfSublatingBeing',
      'conditions = dispersedMaterial',
      'negativity = mediatingMeans',
      'negativity = freeUnityOfConcept',
    ],
    predicates: [{ name: 'IsMediatingBeing', args: ['A'] }],
    relations: [
      { predicate: 'translates', from: 'conditions', to: 'actuality' },
    ],
  },
  {
    id: 'sn-op-hyp-8-negativity-activity',
    chunkId: 'sn-hyp-8-negativity-mediating-means',
    label: 'Negativity as mediating means — activity',
    clauses: [
      'negativity.determinesItself = asActivity',
      'middleTerm = contradictionOfObjectiveUniversality',
      'middleTerm = contradictionOfTotalityAndIndifferentImmediacy',
      'middleTerm = existentNecessity',
      'objectiveUniversality.contains = selfReferenceAsBeing',
    ],
    predicates: [{ name: 'IsMediatingMeans', args: ['negativity'] }],
    relations: [
      { predicate: 'isContradictionOf', from: 'middleTerm', to: 'objectiveUniversality' },
    ],
  },
  {
    id: 'sn-op-hyp-9-conclusion',
    chunkId: 'sn-hyp-9-conclusion-identity',
    label: 'Conclusion — mediated immediacy and identity',
    clauses: [
      'B.exists = immediately',
      'B.exists = throughOther (mediated)',
      'conclusion.form = sameConceptAsMiddleTerm',
      'absoluteContent(A) = absoluteContent(B)',
      'mediatingTerm.identity = mediated.identity',
    ],
    predicates: [{ name: 'ExpressesMediatedImmediacy', args: ['conclusion'] }],
    relations: [
      { predicate: 'identifies', from: 'mediatingTerm', to: 'mediated' },
    ],
  },
  {
    id: 'sn-op-hyp-10-form-negative',
    chunkId: 'sn-hyp-10-form-negative-unity',
    label: 'Form as negative unity — first display',
    clauses: [
      'hypothetical.displays = necessaryConnectionThroughForm',
      'form = negativeUnity',
      'categorical.displays = positiveUnity',
      'necessity.mergesWith = necessary',
      'difference(A,B) = emptyName',
    ],
    predicates: [{ name: 'DisplaysNegativeUnity', args: ['hypotheticalSyllogism'] }],
    relations: [
      { predicate: 'displays', from: 'hypotheticalSyllogism', to: 'negativeUnity' },
    ],
  },
  {
    id: 'sn-op-hyp-11-unity-posited',
    chunkId: 'sn-hyp-11-unity-posited',
    label: 'Unity posited — identical content',
    clauses: [
      'unity = reflectedIntoItself',
      'unity = identicalContent',
      'being(A) = being(B)',
      'immediacy = mediated',
      'externality.sublated = true',
      'unity = withdrawnIntoItself',
    ],
    predicates: [{ name: 'IsPositedUnity', args: ['unity'] }],
    relations: [
      { predicate: 'sublates', from: 'unity', to: 'externality' },
    ],
  },
  {
    id: 'sn-op-hyp-12-transition',
    chunkId: 'sn-hyp-12-transition-disjunctive',
    label: 'Transition to disjunctive syllogism',
    clauses: [
      'mediation.determinedAs = {singularity, immediacy, selfReferringNegativity}',
      'mediation = differentiatingIdentity',
      'mediation.retrievesItself = intoItself',
      'mediation = absoluteForm',
      'mediation = objectiveUniversality',
      'syllogism = disjunctive',
    ],
    predicates: [{ name: 'TransitionsToDisjunctive', args: ['hypotheticalSyllogism'] }],
    relations: [
      { predicate: 'transitions', from: 'hypotheticalSyllogism', to: 'disjunctiveSyllogism' },
    ],
  },
  // ============================================================================
  // DISJUNCTIVE SYLLOGISM OPERATIONS
  // ============================================================================
  {
    id: 'sn-op-dis-1-schema',
    chunkId: 'sn-dis-1-schema',
    label: 'Disjunctive syllogism — schema S-U-P',
    clauses: [
      'schema = S-U-P',
      'middleTerm = universalityRepleteWithForm',
      'middleTerm = totality',
      'middleTerm = developedObjectiveUniversality',
    ],
    predicates: [{ name: 'IsDisjunctiveSyllogism', args: [] }],
    relations: [
      { predicate: 'implements', from: 'disjunctiveSyllogism', to: 'S-U-P' },
    ],
  },
  {
    id: 'sn-op-dis-2-middle-totality',
    chunkId: 'sn-dis-2-middle-totality',
    label: 'Middle term as totality — universality, particularity, singularity',
    clauses: [
      'middleTerm = {universality, particularity, singularity}',
      'middleTerm = substantialIdentityOfGenus',
      'middleTerm = universalSphere',
      'middleTerm.contains = totalParticularity',
      'middleTerm = genusSortedInSpecies',
      'A = {B, C, D}',
    ],
    predicates: [{ name: 'IsTotality', args: ['middleTerm'] }],
    relations: [
      { predicate: 'contains', from: 'middleTerm', to: 'totalParticularity' },
    ],
  },
  {
    id: 'sn-op-dis-3-particularization',
    chunkId: 'sn-dis-3-particularization-either-or',
    label: 'Particularization as differentiation — either-or',
    clauses: [
      'particularization = differentiation',
      'differentiation = eitherOr(B, C, D)',
      'eitherOr = negativeUnity',
      'eitherOr = reciprocalExclusion',
      'exclusion = selfReferringDetermination',
      'particular = singularityExcludingOthers',
    ],
    predicates: [{ name: 'IsDifferentiation', args: ['particularization'] }],
    relations: [
      { predicate: 'excludes', from: 'B', to: 'C+D' },
    ],
  },
  {
    id: 'sn-op-dis-4-forms',
    chunkId: 'sn-dis-4-forms-examples',
    label: 'Disjunctive forms — examples',
    clauses: [
      'form1: (A either B or C or D) && (A is B) => (A not C && A not D)',
      'form2: (A either B or C or D) && (A not C && A not D) => (A is B)',
    ],
    predicates: [{ name: 'IsDisjunctiveForm', args: ['form1|form2'] }],
    relations: [
      { predicate: 'derives', from: 'premises', to: 'conclusion' },
    ],
  },
  {
    id: 'sn-op-dis-5-subject-roles',
    chunkId: 'sn-dis-5-subject-across-premises',
    label: 'Subject A across premises and conclusion',
    clauses: [
      'A.inFirstPremise = universal',
      'A.predicate = universalSphereParticularized',
      'A.inSecondPremise = determinateSpecies',
      'A.inConclusion = excludingSingularDeterminateness',
    ],
    predicates: [{ name: 'PlaysMultipleRoles', args: ['A'] }],
    relations: [
      { predicate: 'transformsRole', from: 'A', to: 'conclusionRole' },
    ],
  },
  {
    id: 'sn-op-dis-6-mediator-unity',
    chunkId: 'sn-dis-6-mediator-unity',
    label: 'Mediator = unity of mediator and mediated',
    clauses: [
      'mediated = universality(A) + singularity',
      'mediatingMeans = A.asUniversalSphere.determinedAsSingular',
      'disjunctive.posits = truthOfHypothetical',
      'disjunctive = unityOfMediatorAndMediated',
      'disjunctive.noLonger = syllogism',
    ],
    predicates: [{ name: 'PositsMediatorUnity', args: ['disjunctiveSyllogism'] }],
    relations: [
      { predicate: 'posits', from: 'disjunctiveSyllogism', to: 'mediatorUnity' },
    ],
  },
  {
    id: 'sn-op-dis-7-middle-contains',
    chunkId: 'sn-dis-7-middle-contains-extremes',
    label: 'Middle term contains extremes — extremes as positedness',
    clauses: [
      'middleTerm = totalityOfConcept',
      'middleTerm.contains = extremesInCompleteDeterminateness',
      'extremes.asDistinct = merePositedness',
      'extremes.noProperDeterminateness = againstMiddle',
    ],
    predicates: [{ name: 'ContainsExtremes', args: ['middleTerm'] }],
    relations: [
      { predicate: 'contains', from: 'middleTerm', to: 'extremes' },
    ],
  },
  {
    id: 'sn-op-dis-8-form-content',
    chunkId: 'sn-dis-8-form-content-identity',
    label: 'Form identical with content',
    clauses: [
      'hypothetical.had = substantialIdentity + negativeUnity',
      'disjunctive.middleTerm = A.asGenusAndPerfectlyDetermined',
      'form.identicalWith = solidContent',
      'formDetermination = determinateDifference + simpleIdentity',
    ],
    predicates: [{ name: 'HasFormContentIdentity', args: ['disjunctiveSyllogism'] }],
    relations: [
      { predicate: 'identifies', from: 'form', to: 'content' },
    ],
  },
  {
    id: 'sn-op-dis-9-formalism-sublated',
    chunkId: 'sn-dis-9-formalism-sublated',
    label: 'Formalism and subjectivity sublated',
    clauses: [
      'formalism.sublated = true',
      'subjectivity.sublated = true',
      'middle.was = abstractDeterminationDistinctFromTerms',
      'completion.overcomes = formalDistinction',
    ],
    predicates: [{ name: 'SublatesFormalism', args: ['disjunctiveSyllogism'] }],
    relations: [
      { predicate: 'sublates', from: 'disjunctiveSyllogism', to: 'formalism' },
    ],
  },
  {
    id: 'sn-op-dis-10-distinction-fallen',
    chunkId: 'sn-dis-10-distinction-fallen-away',
    label: 'Distinction of mediating and mediated has fallen away',
    clauses: [
      'objectiveUniversality = totalityOfFormDeterminations',
      'mediatorMediated.distinction = false',
      'mediated ∈ mediator.moments',
      'eachMoment = totalityOfMediated',
    ],
    predicates: [{ name: 'CollapsesDistinction', args: ['disjunctiveSyllogism'] }],
    relations: [
      { predicate: 'collapses', from: 'disjunctiveSyllogism', to: 'distinction' },
    ],
  },
  {
    id: 'sn-op-dis-11-figures',
    chunkId: 'sn-dis-11-figures-middle',
    label: 'Figures exhibit determinateness as middle — concept as ought',
    clauses: [
      'figures.exhibit = determinatenessAsMiddle',
      'middle = conceptAsOught',
      'middle.requirement = conceptTotality',
      'genera.exhibit = stagesOfRepletion',
    ],
    predicates: [{ name: 'ExhibitsAsMiddle', args: ['figures', 'determinateness'] }],
    relations: [
      { predicate: 'exhibits', from: 'figures', to: 'determinatenessAsMiddle' },
    ],
  },
  {
    id: 'sn-op-dis-12-repletion',
    chunkId: 'sn-dis-12-repletion-concretion',
    label: 'Repletion/concretion across genera',
    clauses: [
      'formal.middle.positedVia = allDeterminaciesSingly',
      'reflection.middle = unityGatheringExternally',
      'necessity.middle = developedAndTotalAndSimple',
      'formDistinction.sublated = true',
    ],
    predicates: [{ name: 'ShowsRepletionStages', args: ['genera'] }],
    relations: [
      { predicate: 'sublates', from: 'necessityMiddle', to: 'formDistinction' },
    ],
  },
  {
    id: 'sn-op-dis-13-concept-realized',
    chunkId: 'sn-dis-13-concept-realized',
    label: 'Concept realized as objectivity',
    clauses: [
      'concept.realized = true',
      'concept.gained = objectivity',
      'firstReality = conceptPartitionsItself',
      'concept.asJudgment = positsDeterminations',
      'concept.asSyllogism = setsOverAgainst',
    ],
    predicates: [{ name: 'IsRealized', args: ['concept'] }],
    relations: [
      { predicate: 'gains', from: 'concept', to: 'objectivity' },
    ],
  },
  {
    id: 'sn-op-dis-14-inwardness-externality',
    chunkId: 'sn-dis-14-inwardness-externality',
    label: 'Inwardness and externality equated',
    clauses: [
      'concept.still = inwardnessOfExternality',
      'externality.equatedWith = innerUnity',
      'determinations.returnInto = innerUnity',
      'externality.exhibits = concept',
      'concept.noLongerDistinct = fromExternality',
    ],
    predicates: [{ name: 'EquatesInwardnessExternality', args: ['syllogism'] }],
    relations: [
      { predicate: 'equates', from: 'syllogism', to: 'inwardnessAndExternality' },
    ],
  },
  {
    id: 'sn-op-dis-15-mediation',
    chunkId: 'sn-dis-15-syllogism-mediation',
    label: 'Syllogism as mediation — complete concept posited',
    clauses: [
      'determinateness = positedness',
      'identity.exhibited = asTruthOfConcept',
      'judgment.momentsRemain = determinationsInConnection',
      'syllogism = mediation',
      'syllogism = completeConceptPosited',
    ],
    predicates: [{ name: 'IsCompleteConceptPosited', args: ['syllogism'] }],
    relations: [
      { predicate: 'is', from: 'syllogism', to: 'completeConceptPosited' },
    ],
  },
  {
    id: 'sn-op-dis-16-sublation-result',
    chunkId: 'sn-dis-16-sublation-result',
    label: 'Sublation of mediation — result as objectivity',
    clauses: [
      'movement = sublationOfMediation',
      'nothing.is = inAndForItself',
      'eachThing.onlyThrough = mediationOfOther',
      'result = immediacyEmergingFromSublation',
      'result = beingIdenticalWithMediation',
      'result = conceptRestoredFromOtherness',
    ],
    predicates: [{ name: 'SublatesMediation', args: ['syllogism'] }],
    relations: [
      { predicate: 'yields', from: 'sublation', to: 'objectivity' },
    ],
  },
  {
    id: 'sn-op-dis-17-objectivity',
    chunkId: 'sn-dis-17-objectivity',
    label: 'Result: objectivity — fact in and for itself',
    clauses: [
      'result = factInAndForItself',
      'result = objectivity',
    ],
    predicates: [{ name: 'IsObjectivity', args: ['result'] }],
    relations: [
      { predicate: 'is', from: 'result', to: 'objectivity' },
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
