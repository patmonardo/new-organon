import type { Chunk, LogicalOperation } from './index';

/**
 * SYLLOGISM OF EXISTENCE — Complete Structure
 * 
 * NOTE: The syllogism in its immediate form has for its moments the determinations
 * of the concept as immediate. These are abstract determinacies of form that have
 * not yet been developed by mediation into concretion but are only singular determinacies.
 * The first syllogism is strictly formal. The formalism of syllogistic inference consists
 * in stopping short at the form of this first syllogism.
 * 
 * Structure:
 * Introduction: Immediate form, abstract determinacies, particularity as middle
 * a. First figure (S-P-U) — three sections
 * b. Second figure (P-S-U) — three sections  
 * c. Third figure (S-U-P) — three sections
 * d. Fourth figure (U-U-U, mathematical syllogism) — transition to reflection
 * 
 * Transition: Syllogism of Existence passes over into Syllogism of Reflection
 */

// ============================================================================
// INTRODUCTION: SYLLOGISM OF EXISTENCE — IMMEDIATE FORM
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'syll-ex-intro-1-immediate-form',
    title: 'Syllogism in immediate form — abstract determinacies',
    text: `The syllogism in its immediate form has for its moments the determinations of the concept as immediate. Accordingly, these are the abstract determinacies of form, such as have not yet been developed by mediation into concretion but are only singular determinacies. The first syllogism is thus the one which is strictly formal.`,
  },
  {
    id: 'syll-ex-intro-2-formalism',
    title: 'Formalism consists in stopping short at first syllogism',
    text: `The formalism of syllogistic inference consists in stopping short at the form of this first syllogism. The concept, when partitioned into its abstract moments, has singularity and universality for its extremes, and itself appears as the particularity that stands between them.`,
  },
  {
    id: 'syll-ex-intro-3-particularity-middle',
    title: 'Particularity as middle term — double-sidedness',
    text: `Because of their immediacy, these determinacies only refer to themselves, one and all a single content. Particularity constitutes at first the middle term by uniting within itself, immediately, the two moments of singularity and universality. Because of its determinateness, on the one hand it is subsumed under the universal; on the other hand, the singular with respect to which it possesses universality is subsumed under it.`,
  },
  {
    id: 'syll-ex-intro-4-mediation-not-posited',
    title: 'Concretion as double-sidedness — mediation not yet posited',
    text: `This concretion is at first, however, only a double-sidedness; the middle term, because of the immediacy that affects it in the immediate syllogism, is a simple determinateness, and the mediation which it constitutes is not as yet posited. Now the dialectical movement of the syllogism of existence consists in positing the moments of the mediation that alone constitutes the syllogism.`,
  },
];

// ============================================================================
// a. THE FIRST FIGURE: S-P-U
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex1-1-overview',
    title: 'First figure — overview',
    text: `S-P-U is the general schema: singularity (S) connects to universality (U) through particularity (P). The extreme terms stand over against each other and are joined in a distinct third term (the particular).`
  },
  {
    id: 'syll-ex1-2-schema-and-relations',
    title: 'Schema and immediate relations',
    text: `Singularity is not universal immediately but via particularity; universality lowers itself to the singular through particularity. Extremes are determinateness and share universal determinateness in the particular.`
  },
  {
    id: 'syll-ex1-3-general-meaning-emergence',
    title: 'General meaning: emergence into existence',
    text: `The singular, an inward self-reference, emerges into existence via particularity into a universality where it stands in external conjunction; conversely, particularity concretizes singularity into a self-referring universal.`
  },
  {
    id: 'syll-ex1-4-objective-significance-not-yet-positied',
    title: 'Objective significance is initially superficial',
    text: `In the first figure the objective significance is only superficially present: determinations are not yet posited as the unity constituting syllogistic essence, so the syllogism remains subjective (abstract terms lack being-for-themselves).`
  },
  {
    id: 'syll-ex1-5-deficiency-and-form-relation',
    title: 'Deficiency not in form but in content richness',
    text: `The form-relation (singularity, particularity, universality) is essential; the deficiency is that each determination is not concurrently richer under the others — terms lack the fuller content required for true objectivity.`
  },
  {
    id: 'syll-ex1-6-aristotle-and-inherence',
    title: 'Aristotle’s inherence account (formalistic view)',
    text: `Aristotle formulates syllogism as repeated inherence relations (one extreme in middle; middle in other extreme) — a repetition of equal inherence rather than the determinateness of the three terms to one another.`
  },
  {
    id: 'syll-ex1-7-relation-of-figures-to-first',
    title: 'Other figures reduce to the first or develop it',
    text: `Other figure-relations are valid only insofar as they reduce to the original relation; when they deviate, they are transformations of the first abstract form, which then determines itself further and becomes totality.`
  },
  {
    id: 'syll-ex1-8-schema-restated-and-example',
    title: 'Restatement of S-P-U and example',
    text: `S is subsumed under P, P under U; thus S under U. The middle can be subject or predicate depending on perspective. The syllogism is not merely three judgments (e.g., 'All humans are mortal; Gaius is human; therefore Gaius is mortal').`
  },
  {
    id: 'syll-ex1-9-critique-of-formalism-and-conclusion',
    title: 'Critique of formalism; unity in fact',
    text: `Treating the inference as separate propositions is a subjective expedient that hides the unity of determinations. The syllogistic inference is the truth of judgment: the determinate relations are already a unity — "All things are a syllogism."`
  }
]

// appended: First Figure — part 2 (qualitative side, contingency of middles)
CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex1-10-qualitative-side',
    title: 'Qualitative side — terms as content',
    text: `Consider the syllogism where terms are concrete content: the terms are singular determinacies (properties, relations). The singular is an immediate concrete subject; particularity is one of its determinacies; universality is a more abstract determinateness within the particular.`
  },
  {
    id: 'syll-ex1-11-manifoldness-and-middles',
    title: 'Manifoldness of determinations; many possible middles',
    text: `A concrete subject has an indeterminate manifold of determinacies; any of these may serve as middle term and attach the subject to different universals. The same middle can link to several predicates; choice of middle is accidental.`
  },
  {
    id: 'syll-ex1-12-contingency-and-contradiction-examples',
    title: 'Contingency leads to contradictory yet correct inferences (examples)',
    text: `Different middle terms yield different, equally correct inferences that can contradict: e.g., a painted wall inferred blue from one middle, yellow from another; senses vs spirituality for human predicates; gravitation vs centrifugal force for celestial motion; sociability vs individuality for political ends.`
  },
  {
    id: 'syll-ex1-13-formal-syllogism-unsatisfactory',
    title: 'Why the formal syllogism is unsatisfactory',
    text: `The formal syllogism is unsatisfactory because the form's abstractness forces a one-sidedness: it treats a single quality of a concept as if it were exhaustive. Kant's antinomies are similar: picking different determinations as ground yields opposing yet formally necessary results.`
  },
  {
    id: 'syll-ex1-14-conclusion-contingency-in-form',
    title: 'Conclusion: contingency rooted in form, not content',
    text: `The insufficiency is not merely contental; it arises from the abstract form that permits only one-side determinateness. Thus when a subject is given, it is contingent which determinateness the formal syllogism will infer from it.`
  }
)

// appended: First Figure — part 3 (form-connections, regress, mediation-shift, singular-as-mediator)
CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex1-15-content-vs-form',
    title: 'Determinations: contentual immediacy vs determinations of form',
    text: `The determinations appear as immediate, contentual qualities, but their essence is formal: they are connections. These connections are (1) extremes→middle (the two premises) and (2) extremes→extremes (the mediated conclusion).`
  },
  {
    id: 'syll-ex1-16-premises-and-conclusion',
    title: 'Premises (immediate connections) vs mediated conclusion',
    text: `The immediate connections (propositio major and minor) are judgments and thus contradict the syllogism's demand that unity be posited; the true connection is the mediated conclusion, the syllogistic truth of the judgment.`
  },
  {
    id: 'syll-ex1-17-infinite-regress-of-proving-premises',
    title: 'Proof-regress: premises would need proving → bad infinity',
    text: `If premises must be proved, each proof yields new premises, producing a geometric progression to infinity. This regress repeats the original deficiency and indicates the need to sublate the infinite progression.`
  },
  {
    id: 'syll-ex1-18-mediation-reshaped',
    title: 'Mediation must change form: P-S-U and S-U-P',
    text: `Mediation cannot simply replicate S-P-U; to mediate P-U use S (P-S-U), to mediate S-P use U (S-U-P). The mediation form must shift so it does not reproduce the same deficient immediacy.`
  },
  {
    id: 'syll-ex1-19-singular-becomes-mediator',
    title: 'Singular elevated: conclusion posits S as universal mediator',
    text: `When the first syllogism concludes S-U, the singular is posited as a universal (it becomes mediating). The singular thus unites particularity and universality by being both particular and, through conclusion, a universal.`
  }
)

// ============================================================================
// b. THE SECOND FIGURE: P-S-U
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex2-1-overview',
    title: 'Second figure — overview (P-S-U)',
    text: `P-S-U: the particular mediates between singularity and universality. The truth of the first qualitative syllogism is that something is not in and for itself united to a qualitative determinateness which is a universal, but is united to it by means of a contingency or in a singularity. The subject of the syllogism has not returned in such a quality to its concept but is conceived only in its externality; the immediacy constitutes the basis of the connection and hence the mediation; to this extent, the singular is in truth the middle.`,
  },
  {
    id: 'syll-ex2-2-mediation-negative',
    title: 'Mediation as sublation of immediacy — negative unity',
    text: `But further, the syllogistic connection is the sublation of the immediacy; the conclusion is a connection drawn not immediately but through a third term; therefore, it contains a negative unity; therefore, the mediation is now determined as containing a negative moment within it.`,
  },
  {
    id: 'syll-ex2-3-premises-status',
    title: 'Premises: one immediate, one mediated',
    text: `In this second syllogism, the premises are: P-S and S-U; only the first of these premises is still an immediate one; the second, S-U, is already mediated, namely through the first syllogism; the second syllogism thus presupposes the first just as, conversely, the first presupposes the second.`,
  },
  {
    id: 'syll-ex2-4-exchange-of-places',
    title: 'Exchange of places: particular ↔ singular ↔ universal',
    text: `The two extremes are here determined, the one as against the other, as particular and universal. The latter thus retains its place; it is predicate. But the particular has exchanged places; it is subject or is posited in the determination of the extreme of singularity, just as the singular is posited with the determination of the middle term or of particularity. The two no longer are, therefore, the abstract immediacies which they were in the first syllogism.`,
  },
  {
    id: 'syll-ex2-5-determinate-meaning',
    title: 'Determinate meaning: universal as species via singularity',
    text: `The determinate and objective meaning of this syllogism is that the universal is not in and for itself a determinate particular (it is rather the totality of its particulars) but that it is one of its species through the mediation of singularity; the rest of its species are excluded from it by the immediacy of externality. Likewise the particular is not for its part immediately, and in and for itself, the universal; the negative unity is rather what removes the determinateness from it and thereby raises it to universality.`,
  },
  {
    id: 'syll-ex2-6-immediate-determinacies',
    title: 'Terms still immediate determinacies; form external',
    text: `But the terms are at first still immediate determinacies; they have not advanced of their own to any objective signification; the positions which two of them have exchanged and now occupy is the form, and this is as yet only external to them. Therefore they are still, as in the first syllogism, each a content indifferent as such to the other, two qualities linked together, not in and for themselves, but through the mediation of an accidental singularity.`,
  },
  {
    id: 'syll-ex2-7-beginning-realization',
    title: 'Transition: beginning of realization; alteration of pure form',
    text: `The syllogism of the first figure was the immediate syllogism, or again, the syllogism in so far as its concept is an abstract form that has not yet realized itself in all its determinations. The transition of this pure form into another figure is on the one hand the beginning of the realization of the concept, in that the negative moment of the mediation, and thereby one further determinateness of the form, is posited in the originally immediate, qualitative determinateness of the terms. But, on the other hand, this is at the same time an alteration of the pure form of the syllogism; the latter no longer conforms to it fully.`,
  },
  {
    id: 'syll-ex2-8-subjective-species',
    title: 'Second figure as subjective species; relation to first',
    text: `In so far as it is regarded as only a subjective syllogism that runs its course in external reflection, we can then take it as a species of syllogistic inference that should conform to the genus, namely the general schema S-P-U. But it does not at the moment conform to it. The true meaning of this syllogism's lack of conformity to the general form of the syllogism is that the latter has passed over into it, for its truth consists in being a subjective, contingent conjoining of terms.`,
  },
  {
    id: 'syll-ex2-9-particularity-conclusion',
    title: 'Conclusion restricted to particular judgment',
    text: `If the conclusion in this second figure is correct (that is, without recurring to the restriction, to which we shall presently turn, that makes of it something indeterminate), then it is correct because it is so on its own, not because it is the conclusion of this syllogism. But the same is the case for the conclusion of the first figure; it is this, the truth of that first figure, which is posited by the second. Consequently, the conclusion in this figure can only be particular. But the particular judgment, as we remarked above, is positive as well as negative, a conclusion, therefore, to which no great value can be ascribed.`,
  },
  {
    id: 'syll-ex2-10-indifference-interchangeability',
    title: 'Indifference of extremes; interchangeability of premises',
    text: `Since the particular and universal are also the extremes, and are immediate determinacies indifferent to each other, their relation itself is indifferent; each can be the major or the minor term, indifferently the one or the other, and consequently either premise can also be taken as major or minor.`,
  },
  {
    id: 'syll-ex2-11-universal-connection',
    title: 'Conclusion as universal connection (positive & negative)',
    text: `Since the conclusion is positive as well as negative, it is a connection which for that reason is indifferent to these determinacies, hence a universal connection. More precisely, the mediation of the first syllogism was implicitly a contingent one; in the second syllogism, this contingency is posited.`,
  },
  {
    id: 'syll-ex2-12-mediation-self-sublating',
    title: 'Mediation self-sublating; singularity as immediate manifold',
    text: `Consequently, the mediation is self-sublating; it has the determination of singularity and immediacy; what this syllogism joins together must, on the contrary, be in itself and immediately identical, for that mediating middle, the immediate singularity, is an infinitely manifold and external determining. Posited in it, therefore, is rather the self-external mediation.`,
  },
  {
    id: 'syll-ex2-13-immediacy-abstract-universal',
    title: 'Immediacy points to abstract universal',
    text: `The externality of singularity, however, is universality; that mediation by means of the immediate singular points beyond itself to the mediation which is the other than it, one which therefore occurs by means of the universal. In other words, what is supposed to be united by means of the second syllogism, must be immediately conjoined; the immediacy on which it is based does not allow any definite conclusion. The immediacy to which this syllogism points is the opposite of its own: it is the sublated first immediacy of being, therefore the immediacy reflected into itself or the abstract universal existing in itself.`,
  },
  {
    id: 'syll-ex2-14-transition-new-form',
    title: 'Transition: qualitative base → another form of syllogism',
    text: `From the standpoint of the present consideration, the transition of this syllogism was like the transition of being an alteration, for its base is qualitative; it is the immediacy of singularity. But according to the concept, singularity conjoins the particular and the universal by sublating the determinateness of the particular and this is what presents itself as the contingency of this syllogistic inference. But inasmuch as the middle term is posited in this determination which is its truth, we have another form of the syllogism.`,
  },
);

// ============================================================================
// c. THE THIRD FIGURE: S-U-P
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex3-1-overview',
    title: 'Third figure — overview (S-U-P)',
    text: `This third syllogism no longer has any single immediate premise; the connection S-U has been mediated by the first syllogism; the connection P-U by the second. It thus presupposes both these syllogisms; but conversely it is presupposed by them, just as in general each presupposes the other two. In this third figure, therefore, it is the determination of the syllogism as such that is brought to completion.`,
  },
  {
    id: 'syll-ex3-2-reciprocal-mediation',
    title: 'Reciprocal mediation and incompleteness',
    text: `This reciprocal mediation means just this, that each syllogism, although for itself a mediation, does not possess the totality of mediation but is affected by an immediacy whose mediation lies outside it.`,
  },
  {
    id: 'syll-ex3-3-formal-truth',
    title: 'S-U-P as the truth of the formal syllogism',
    text: `Considered in itself, the syllogism S-U-P is the truth of the formal syllogism; it expresses the fact that its mediating middle is the abstract universal and that the extremes are not contained in it according to their essential determinateness but only according to their universality, that precisely that is not conjoined in it, which was supposed to be mediated.`,
  },
  {
    id: 'syll-ex3-4-terms-immediate-content',
    title: 'Terms have immediate content indifferent to form',
    text: `Posited here, therefore, is that wherein the formalism of the syllogism consists; that its terms have an immediate content which is indifferent towards the form, or, what amounts to the same, that they are such form determinations as have not yet reflected themselves into determinations of content.`,
  },
  {
    id: 'syll-ex3-5-middle-indeterminate-universal',
    title: 'Middle = indeterminate universal (abstraction from determinateness)',
    text: `The middle of this syllogism is indeed the unity of the extremes, but a unity in which abstraction is made from their determinateness, the indeterminate universal. But in so far as this universal is at the same time distinguished from the extremes as the abstract from the determinate, it is itself also a determinate as against them, and the whole is a syllogism whose relation to its concept needs examining.`,
  },
  {
    id: 'syll-ex3-6-legitimacy-negative-conclusion',
    title: 'Legitimacy requires negative judgment; conclusion is negative',
    text: `As the universal, the middle term is with respect to both its extremes the term that subsumes or the predicate, not a term for once also subsumed or the subject. Now as a species of syllogism, it ought to conform to the latter, and this can only happen on condition that, inasmuch as the one connection S-U already possesses the appropriate relation, the other connection P-U contains it too. This occurs in a judgment in which the relation of subject and predicate is an indifferent one, in a negative judgment. Thus does the syllogism become legitimate, but the conclusion is necessarily negative.`,
  },
  {
    id: 'syll-ex3-7-indifference-fourth-figure',
    title: 'Indifference of roles → fourth figure and its vacuity',
    text: `Consequently, also indifferent is now which of the two determinations of this proposition is taken as predicate or subject, and whether the determination is taken in the syllogism as the extreme of singularity or the extreme of particularity, hence as the minor or major term. Since on the usual assumption which of the premises is supposed to be the major or the minor depends on this distinction, this too has now become a matter of indifference. This is the ground of the customary fourth figure of the syllogism which was unknown to Aristotle and has to do with an entirely void and uninteresting distinction.`,
  },
  {
    id: 'syll-ex3-8-objective-significance-limited',
    title: 'Objective significance: universality as middle is qualitative/abstract',
    text: `The objective significance of the syllogism in which the universal is the middle is that the mediating term, as the unity of the extremes, is essentially a universal. But since the universality is at first only qualitative or abstract, the determinateness of the extremes is not contained in it; their being conjoined in the conclusion, if the conjunction is to take place, must likewise have its ground in a mediation that lies outside this syllogism and is, with respect to the latter, just as contingent as it is in the preceding forms of the syllogism.`,
  },
  {
    id: 'syll-ex3-9-relationless-figure-uuu',
    title: 'Relationless figure (U-U-U) as extreme abstraction',
    text: `But now, since the universal is determined as the middle term, and since the determinateness of the extremes is not contained in this middle, the latter is posited as one which is wholly indifferent and external. It is here, by virtue indeed of a bare abstraction, that a fourth figure of the syllogism arose in the first place, namely the figure of the relationless syllogism, U-U-U, which abstracts from the qualitative differentiation of the terms and therefore has their merely external unity, their equality, for its determination.`,
  },
);

// ============================================================================
// d. THE FOURTH FIGURE: U-U-U, OR THE MATHEMATICAL SYLLOGISM
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex4-1-mathematical-form',
    title: 'Mathematical syllogism — "if two things equal to a third, then equal"',
    text: `The mathematical syllogism goes like this: if two things or two determinations are equal to a third, then they are equal to each other. The relation of inherence or subsumption of terms is done away with. A "third" is in general the mediating term; but this third has absolutely no determination as against the extremes. Each of the three terms can therefore be the mediating term just as well as any other.`,
  },
  {
    id: 'syll-ex4-2-external-determination',
    title: 'Which term mediates depends on external circumstances',
    text: `Which is needed for the job, which of the three connections are therefore to be taken as immediate, and which as mediated, depends on external circumstances and other conditions, namely which two of the three are immediately given. But this determination does not concern the syllogism and is wholly external.`,
  },
  {
    id: 'syll-ex4-3-axiom-status',
    title: 'Mathematical syllogism as axiom — self-explanatory',
    text: `The mathematical syllogism ranks in mathematics as an axiom, as a first self-explanatory proposition which is neither capable nor in need of proof, i.e of any mediation which neither presupposes anything else nor can be derived from anything else.`,
  },
  {
    id: 'syll-ex4-4-formalism-abstraction',
    title: 'Self-evidence lies in formalism — abstraction from qualitative diversity',
    text: `If we take a closer look at this prerogative that the proposition claims, of being immediately self-evident, we find that it lies in its formalism, in the fact that it abstracts from every qualitative diversity of determinations and only admits their quantitative equality or inequality. But for this very reason it is not without presupposition or mediation; the quantitative determination, which alone comes into consideration in it, is only by virtue of the abstraction from qualitative differentiation and from the concept determinations.`,
  },
  {
    id: 'syll-ex4-5-examples-magnitude',
    title: 'Examples — equality only according to magnitude',
    text: `Lines, figures, posited as equal to each other, are understood only according to their magnitude. A triangle is posited as equal to a square, not however as triangle to square but only according to magnitude, etc. Nor does the concept and its determinations enter into this syllogism; there is in it, therefore, no conceptual comprehension at all; the understanding is also not faced here by even the formal, abstract determinations of the concept.`,
  },
  {
    id: 'syll-ex4-6-indigence-abstractness',
    title: 'Self-evidence rests on indigence and abstractness',
    text: `The self-evidence of this syllogism rests, therefore, solely on the indigence and abstractness of its mode of thought.`,
  },
  {
    id: 'syll-ex4-7-positive-result',
    title: 'Positive result — not just abstraction',
    text: `But the result of the syllogism of existence is not just this abstraction from all determinateness of the concept; the negativity of the immediate and abstract determinations that emerged from it has yet another positive side, namely that in the abstract determinateness its other has been posited and the determinateness has thereby become concrete.`,
  },
  {
    id: 'syll-ex4-8-reciprocal-presupposition',
    title: 'Reciprocal presupposition — mediation based on mediation',
    text: `In the first place, the syllogisms of existence all have one another for presupposition, and the extremes conjoined in the conclusion are truly conjoined, in and for themselves, only inasmuch as they are otherwise united by an identity grounded elsewhere; the middle term, as constituted in the syllogisms we have examined, ought to be the conceptual unity of these syllogisms but is in fact only a formal determinateness that is not posited as their concrete unity. But what is thus presupposed by each and every of these mediations is not merely a given immediacy in general, as is the case for the mathematical syllogism, but is itself a mediation, namely of each of the other two syllogisms.`,
  },
  {
    id: 'syll-ex4-9-mediation-of-reflection',
    title: 'Mediation based on mediation — mediation of reflection',
    text: `Therefore, what is truly present here is not a mediation based on a given immediacy, but a mediation based on mediation. And this mediation is not quantitative, not one that abstracts from the form of mediation, but is rather a self-referring mediation, or the mediation of reflection.`,
  },
  {
    id: 'syll-ex4-10-circle-totality',
    title: 'Circle of reciprocal presupposing forms totality',
    text: `The circle of reciprocal presupposing which these syllogisms bring to closure is the turning back of this presupposing into itself; a presupposing that in this turning back forms a totality, and has the other to which every single syllogism refers, not outside by virtue of abstraction, but included within the circle.`,
  },
  {
    id: 'syll-ex4-11-determinations-as-middle',
    title: 'Each determination has occupied place of middle term',
    text: `Further, from the side of the single determinations of form it has been shown that in this whole of formal syllogisms each single determination has in turn occupied the place of the middle term. As immediate, this term was determined as particularity; thereupon, through dialectical movement it determined itself as singularity and universality. Likewise did each of these determinations occupy the places of both of the two extremes.`,
  },
  {
    id: 'syll-ex4-12-positive-result-concrete-identity',
    title: 'Positive result: mediation through concrete identity',
    text: `The merely negative result is the dissolution of the qualitative determinations of form into the merely quantitative, mathematical syllogism. But what we truly have here is the positive result, namely that mediation occurs, not through any single qualitative determinateness of form, but through the concrete identity of the determinacies.`,
  },
  {
    id: 'syll-ex4-13-deficiency-formalism',
    title: 'Deficiency and formalism of three figures',
    text: `The deficiency and formalism of the three figures of the syllogism just considered consists precisely in this, that one such single determinateness was supposed to constitute the middle term in it.`,
  },
  {
    id: 'syll-ex4-14-transition-reflection',
    title: 'Transition: syllogism of existence to syllogism of reflection',
    text: `Mediation has thus determined itself as the indifference of the immediate or abstract determinations of form and the positive reflection of one into the other. The immediate syllogism of existence has thereby passed over into the syllogism of reflection.`,
  },
);

// ============================================================================
// LOGICAL OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // INTRODUCTION OPERATIONS
  // ============================================================================
  {
    id: 'syll-ex-op-intro-1-immediate-form',
    chunkId: 'syll-ex-intro-1-immediate-form',
    label: 'Declare syllogism in immediate form — abstract determinacies',
    clauses: [
      'syllogism.immediateForm.has = determinationsOfConceptAsImmediate',
      'determinations = abstractDeterminaciesOfForm',
      'determinations.notDeveloped = byMediationIntoConcretion',
      'determinations = onlySingularDeterminacies',
      'firstSyllogism = strictlyFormal',
    ],
    predicates: [{ name: 'IsImmediateForm', args: ['syllogism'] }],
    relations: [
      { predicate: 'has', from: 'syllogism', to: 'abstractDeterminacies' },
    ],
  },
  {
    id: 'syll-ex-op-intro-2-formalism',
    chunkId: 'syll-ex-intro-2-formalism',
    label: 'Formalism consists in stopping short at first syllogism',
    clauses: [
      'formalism = stoppingShortAtFirstSyllogism',
      'concept.partitioned = intoAbstractMoments',
      'extremes = {singularity, universality}',
      'concept.appearsAs = particularity',
      'particularity.standsBetween = extremes',
    ],
    predicates: [{ name: 'IsFormalism', args: ['syllogisticInference'] }],
    relations: [
      { predicate: 'stopsShort', from: 'formalism', to: 'firstSyllogism' },
    ],
  },
  {
    id: 'syll-ex-op-intro-3-particularity-middle',
    chunkId: 'syll-ex-intro-3-particularity-middle',
    label: 'Particularity as middle term — double-sidedness',
    clauses: [
      'determinacies.referTo = themselves',
      'determinacies = singleContent',
      'particularity.constitutes = middleTerm',
      'particularity.unites = {singularity, universality}',
      'particularity.subsumedUnder = universal',
      'singular.subsumedUnder = particularity',
    ],
    predicates: [{ name: 'IsMiddleTerm', args: ['particularity'] }],
    relations: [
      { predicate: 'unites', from: 'particularity', to: 'singularityUniversality' },
    ],
  },
  {
    id: 'syll-ex-op-intro-4-mediation-not-posited',
    chunkId: 'syll-ex-intro-4-mediation-not-posited',
    label: 'Concretion as double-sidedness — mediation not yet posited',
    clauses: [
      'concretion = doubleSidedness',
      'middleTerm.affectedBy = immediacy',
      'middleTerm = simpleDeterminateness',
      'mediation.notPosited = true',
      'dialecticalMovement.posits = momentsOfMediation',
    ],
    predicates: [{ name: 'MediationNotPosited', args: ['syllogism'] }],
    relations: [
      { predicate: 'posits', from: 'dialecticalMovement', to: 'mediationMoments' },
    ],
  },
  // ============================================================================
  // FIRST FIGURE OPERATIONS (S-P-U)
  // ============================================================================
  {
    id: 'syll-ex1-op-1-declare-schema',
    chunkId: 'syll-ex1-1-overview',
    label: 'Declare S-P-U schema and role of P',
    clauses: ['schema = S-P-U', 'P.isMiddleTerm()', 'S.connectsTo(U) via P'],
    predicates: [{ name: 'IsSchema', args: ['S-P-U'] }],
    relations: [{ predicate: 'mediates', from: 'P', to: 'S+U' }]
  },
  {
    id: 'syll-ex1-op-2-particularity-function',
    chunkId: 'syll-ex1-2-schema-and-relations',
    label: 'Particularity grounds the mediation between S and U',
    clauses: ['S.notUniversalImmediate', 'U.notSingularImmediate', 'P.enables(conjunction)'],
    predicates: [{ name: 'IsParticularity', args: ['P'] }],
    relations: [{ predicate: 'enables', from: 'P', to: 'S-U-conjunction' }]
  },
  {
    id: 'syll-ex1-op-3-emergence-into-existence',
    chunkId: 'syll-ex1-3-general-meaning-emergence',
    label: 'Singular emerges into universality through P; particularity concretes singular',
    clauses: ['singular.selfReference => via(P) -> universality', 'particularity.concretizes(singularity)'],
    predicates: [{ name: 'EmergesInto', args: ['singular','universality'] }],
    relations: [{ predicate: 'concretizes', from: 'P', to: 'S' }]
  },
  {
    id: 'syll-ex1-op-4-objective-significance',
    chunkId: 'syll-ex1-4-objective-significance-not-yet-positied',
    label: 'Objective significance not yet posited; syllogism remains subjective',
    clauses: ['determinations.notPositedAsUnity', 'syllogism.status = subjective'],
    predicates: [{ name: 'IsSubjective', args: ['syllogism'] }],
    relations: [{ predicate: 'lacks', from: 'terms', to: 'beingForThemselves' }]
  },
  {
    id: 'syll-ex1-op-5-deficiency',
    chunkId: 'syll-ex1-5-deficiency-and-form-relation',
    label: 'Form is correct; deficiency is lack of richer determinations',
    clauses: ['formRelation = valid', 'deficiency = singleDetermination.lacksRicherContent'],
    predicates: [{ name: 'HasDeficiency', args: ['term'] }],
    relations: [{ predicate: 'isDueTo', from: 'deficiency', to: 'insufficientContent' }]
  },
  {
    id: 'syll-ex1-op-6-aristotle-inherence',
    chunkId: 'syll-ex1-6-aristotle-and-inherence',
    label: 'Aristotle: inherence repetition versus determinateness relation',
    clauses: ['Aristotle.form = repeatedInherence', 'this.ignores = determinatenessBetweenThreeTerms'],
    predicates: [{ name: 'IsInherenceView', args: ['Aristotle'] }],
    relations: [{ predicate: 'contrastsWith', from: 'inherenceView', to: 'determinatenessRelation' }]
  },
  {
    id: 'syll-ex1-op-7-figures-reduction',
    chunkId: 'syll-ex1-7-relation-of-figures-to-first',
    label: 'Other figures reduce to or develop the first figure',
    clauses: ['otherFigures.validOnlyIf(reducibleToFirst)', 'deviation => developmentOfFirst'],
    predicates: [{ name: 'ReducesTo', args: ['figure','first'] }],
    relations: [{ predicate: 'developsInto', from: 'firstAbstractForm', to: 'totality' }]
  },
  {
    id: 'syll-ex1-op-8-schema-restated',
    chunkId: 'syll-ex1-8-schema-restated-and-example',
    label: 'Restate subsumption chain and warn against three-proposition formalism',
    clauses: ['S.subsumedUnder(P)', 'P.subsumedUnder(U)', 'therefore S.subsumedUnder(U)', 'example.formalism = misleading'],
    predicates: [{ name: 'IsSubsumptionChain', args: ['S','P','U'] }],
    relations: [{ predicate: 'misleads', from: 'formalisticPresentation', to: 'subjectiveIllusion' }]
  },
  {
    id: 'syll-ex1-op-9-unity-in-fact',
    chunkId: 'syll-ex1-9-critique-of-formalism-and-conclusion',
    label: 'Syllogistic inference is the truth of judgment; unity precedes propositional steps',
    clauses: ['inference.isTruthOfJudgment', 'unityOfDeterminations = priorToSeparatedPropositions'],
    predicates: [{ name: 'IsTruthOfJudgment', args: ['syllogism'] }],
    relations: [{ predicate: 'grounds', from: 'unityOfDeterminations', to: 'judgmentTruth' }]
  },
  {
    id: 'syll-ex1-op-10-declare-qualitative',
    chunkId: 'syll-ex1-10-qualitative-side',
    label: 'Terms as singular determinacies (qualitative reading)',
    clauses: ['terms.mode = qualitative', 'terms.are = singularDeterminacies', 'P := onePropertyOf(S)'],
    predicates: [{ name: 'IsQualitative', args: ['syllogism'] }],
    relations: [{ predicate: 'takesAsMiddle', from: 'P', to: 'S' }]
  },
  {
    id: 'syll-ex1-op-11-multiple-middles',
    chunkId: 'syll-ex1-11-manifoldness-and-middles',
    label: 'Multiple possible middles attach S to different universals',
    clauses: ['S.determinacies = indeterminateManifold', 'forEach(d in S.determinacies) => d.canBeMiddle', 'sameMiddle.canLeadTo(manyPredicates)'],
    predicates: [{ name: 'HasManifold', args: ['S'] }],
    relations: [{ predicate: 'attachesTo', from: 'middle', to: 'universalSet' }]
  },
  {
    id: 'syll-ex1-op-12-contingency-examples',
    chunkId: 'syll-ex1-12-contingency-and-contradiction-examples',
    label: 'Different middles yield different, possibly contradictory but formally correct conclusions',
    clauses: ['middle1 -> conclusionA (correct)', 'middle2 -> conclusionB (correct)', 'conclusionA may contradict conclusionB'],
    predicates: [{ name: 'CanContradict', args: ['conclusions'] }],
    relations: [{ predicate: 'illustrates', from: 'examples', to: 'contingencyIssue' }]
  },
  {
    id: 'syll-ex1-op-13-form-abstractness-fault',
    chunkId: 'syll-ex1-13-formal-syllogism-unsatisfactory',
    label: 'Form’s abstractness causes one-sidedness; Kant analogy',
    clauses: ['form.isAbstract = true', 'abstractForm => content.oneSidedness', 'KantAntinomies ~ choosingDifferentDeterminatesAsGround'],
    predicates: [{ name: 'IsAbstractForm', args: ['formalSyllogism'] }],
    relations: [{ predicate: 'resembles', from: 'formalSyllogism', to: 'KantAntinomies' }]
  },
  {
    id: 'syll-ex1-op-14-contingency-rooted-in-form',
    chunkId: 'syll-ex1-14-conclusion-contingency-in-form',
    label: 'Contingency is consequence of form, not a content defect',
    clauses: ['insufficiency.cause = form.abstractness', 'givenSubject => contingentWhichDeterminateIsInferred'],
    predicates: [{ name: 'IsContingencyOfForm', args: ['syllogism'] }],
    relations: [{ predicate: 'resultsFrom', from: 'contingency', to: 'abstractForm' }]
  },
  {
    id: 'syll-ex1-op-15-content-vs-form',
    chunkId: 'syll-ex1-15-content-vs-form',
    label: 'Distinguish contentual immediacy from formal connection (premises vs conclusion)',
    clauses: [
      'determinations.mode = immediateContent',
      'essence(determinations) = formConnections',
      'connections = {extremes->middle (premises), extremes->extremes (conclusion)}'
    ],
    predicates: [
      { name: 'IsImmediateContent', args: ['determination'] },
      { name: 'IsFormConnection', args: ['determination'] }
    ],
    relations: [
      { predicate: 'connects', from: 'extremeA', to: 'middle' },
      { predicate: 'mediates', from: 'middle', to: 'extremesPair' }
    ]
  },

  {
    id: 'syll-ex1-op-16-premises-are-judgments',
    chunkId: 'syll-ex1-16-premises-and-conclusion',
    label: 'Premises are immediate judgments and contradict syllogistic demand for posited unity',
    clauses: [
      'premises.type = propositions',
      'syllogism.requires = unityPositedInMiddle',
      'premises.asImmediate => contradictNatureOfSyllogism'
    ],
    predicates: [
      { name: 'IsProposition', args: ['premise'] },
      { name: 'RequiresUnity', args: ['syllogism'] }
    ],
    relations: [
      { predicate: 'contradicts', from: 'premises', to: 'syllogisticRequirement' }
    ]
  },

  {
    id: 'syll-ex1-op-17-infinite-regress',
    chunkId: 'syll-ex1-17-infinite-regress-of-proving-premises',
    label: 'Proving premises yields infinite regress (bad infinity) that must be sublated',
    clauses: [
      'prove(premise) => produces(newPremises)',
      'iteration => geometricProgressionToInfinity',
      'badInfinity => indicatesDeficiencyOfForm'
    ],
    predicates: [
      { name: 'IsBadInfinity', args: ['regress'] },
      { name: 'IndicatesDeficiency', args: ['form'] }
    ],
    relations: [
      { predicate: 'resultsIn', from: 'proofIteration', to: 'infiniteProgression' }
    ]
  },

  {
    id: 'syll-ex1-op-18-mediation-reshaped',
    chunkId: 'syll-ex1-18-mediation-reshaped',
    label: 'Mediation must change form (use P-S-U or S-U-P) to avoid replication',
    clauses: [
      'if mediation.repeats(S-P-U) then regressContinues',
      'mediate(P-U) via S => form = P-S-U',
      'mediate(S-P) via U => form = S-U-P'
    ],
    predicates: [
      { name: 'AvoidsReplication', args: ['mediation'] },
      { name: 'IsAlternativeMediation', args: ['P-S-U|S-U-P'] }
    ],
    relations: [
      { predicate: 'transforms', from: 'originalForm', to: 'alternativeForm' }
    ]
  },

  {
    id: 'syll-ex1-op-19-singular-as-mediator',
    chunkId: 'syll-ex1-19-singular-becomes-mediator',
    label: 'Conclusion elevates S to mediating universal (S-U) — singular unites determinations',
    clauses: [
      'conclusion(S-U) => S.positedAsUniversal',
      'S.inMinor = particularity',
      'S.inConclusion = unityOfExtremes',
    ],
    predicates: [
      { name: 'IsPositedAsUniversal', args: ['S'] },
      { name: 'UnitesDeterminations', args: ['S'] },
    ],
    relations: [
      { predicate: 'elevates', from: 'conclusion', to: 'singular' },
    ],
  },
  // ============================================================================
  // SECOND FIGURE OPERATIONS (P-S-U)
  // ============================================================================
  {
    id: 'syll-ex2-op-1-declare-schema',
    chunkId: 'syll-ex2-1-overview',
    label: 'Declare P-S-U schema and role of singular as middle-in-truth',
    clauses: [
      'schema = P-S-U',
      'subject.unionToUniversal = viaContingencyInSingularity',
      'singular.functionsAsMiddleInTruth = true',
    ],
    predicates: [{ name: 'IsSchema', args: ['P-S-U'] }],
    relations: [
      { predicate: 'mediates', from: 'singular', to: 'particular+universal' },
    ],
  },
  {
    id: 'syll-ex2-op-2-mediation-negative',
    chunkId: 'syll-ex2-2-mediation-negative',
    label: 'Mediation contains negative moment (sublation of immediacy)',
    clauses: [
      'syllogisticConnection = sublationOfImmediacy',
      'conclusion.isMediated = true',
      'conclusion.contains = negativeUnity',
      'mediation.determinedAs = containingNegativeMoment',
    ],
    predicates: [{ name: 'IsSublation', args: ['mediation'] }],
    relations: [
      { predicate: 'contains', from: 'mediation', to: 'negativeMoment' },
    ],
  },
  {
    id: 'syll-ex2-op-3-premises-status',
    chunkId: 'syll-ex2-3-premises-status',
    label: 'Premises: P-S immediate; S-U mediated (presupposes first figure)',
    clauses: [
      'premise1 = P-S (immediate)',
      'premise2 = S-U (mediated)',
      'secondSyllogism.presupposes = firstSyllogism',
      'firstSyllogism.presupposes = secondSyllogism',
    ],
    predicates: [
      { name: 'IsImmediate', args: ['P-S'] },
      { name: 'IsMediated', args: ['S-U'] },
    ],
    relations: [
      { predicate: 'presupposes', from: 'secondFigure', to: 'firstFigure' },
    ],
  },
  {
    id: 'syll-ex2-op-4-exchange-places',
    chunkId: 'syll-ex2-4-exchange-of-places',
    label: 'Particular and singular exchange functional places',
    clauses: [
      'universal.retainsPlace = predicate',
      'particular.exchangesPlace = becomesSubject',
      'particular.positedAs = extremeOfSingularity',
      'singular.positedAs = middleTermParticularity',
      'extremes.noLonger = abstractImmediacies',
    ],
    predicates: [{ name: 'ExchangesPlace', args: ['particular', 'singular'] }],
    relations: [
      { predicate: 'positsExternally', from: 'extreme', to: 'otherPlace' },
    ],
  },
  {
    id: 'syll-ex2-op-5-determinate-meaning',
    chunkId: 'syll-ex2-5-determinate-meaning',
    label: 'Universal as one species among particulars via singular mediation',
    clauses: [
      'universal.not = immediateDeterminateParticular',
      'universal = totalityOfParticulars',
      'universal = oneSpeciesViaSingularity',
      'otherSpecies.excluded = byImmediacyOfExternality',
      'particular.not = immediatelyUniversal',
      'negativeUnity.removes = determinateness',
      'negativeUnity.raises = toUniversality',
    ],
    predicates: [{ name: 'IsSpeciesVia', args: ['universal', 'singularity'] }],
    relations: [
      { predicate: 'excludes', from: 'singularity', to: 'otherSpecies' },
    ],
  },
  {
    id: 'syll-ex2-op-6-immediate-determinacies',
    chunkId: 'syll-ex2-6-immediate-determinacies',
    label: 'Terms are immediate; exchanged positions remain external form',
    clauses: [
      'terms.mode = immediateDeterminacies',
      'terms.notAdvanced = toObjectiveSignification',
      'exchangedPositions = externalForm',
      'terms.still = contentIndifferentToOther',
      'linkage.via = accidentalSingularity',
    ],
    predicates: [
      { name: 'IsImmediateDeterminate', args: ['term'] },
      { name: 'HasExternalForm', args: ['exchangedPositions'] },
    ],
    relations: [
      { predicate: 'linkedBy', from: 'termPair', to: 'accidentalSingularity' },
    ],
  },
  {
    id: 'syll-ex2-op-7-beginning-realization',
    chunkId: 'syll-ex2-7-beginning-realization',
    label: 'Transition posits negative moment; pure form altered',
    clauses: [
      'firstFigure = immediateSyllogism',
      'firstFigure.concept = abstractFormNotRealized',
      'transition = beginningOfRealization',
      'negativeMoment.posited = inImmediateDeterminateness',
      'pureForm.altered = true',
      'form.noLongerConforms = fully',
    ],
    predicates: [
      { name: 'PositsNegativeMoment', args: ['transition'] },
      { name: 'AltersForm', args: ['pureForm'] },
    ],
    relations: [
      { predicate: 'resultsIn', from: 'transition', to: 'nonconformity' },
    ],
  },
  {
    id: 'syll-ex2-op-8-subjective-species',
    chunkId: 'syll-ex2-8-subjective-species',
    label: 'Second figure as subjective species; truth not from form',
    clauses: [
      'secondFigure.regardedAs = subjectiveSyllogism',
      'secondFigure.shouldConform = toS-P-U',
      'secondFigure.notConforms = atMoment',
      'truth = subjectiveContingentConjoining',
      'conclusion.correct = onItsOwn',
      'conclusion.notCorrect = becauseOfSyllogisticForm',
    ],
    predicates: [{ name: 'IsSubjectiveSpecies', args: ['secondFigure'] }],
    relations: [
      { predicate: 'posits', from: 'secondFigure', to: 'truthOfFirst' },
    ],
  },
  {
    id: 'syll-ex2-op-9-particularity-conclusion',
    chunkId: 'syll-ex2-9-particularity-conclusion',
    label: 'Conclusion restricted to particular judgment',
    clauses: [
      'conclusion.canOnlyBe = particular',
      'particularJudgment = positiveAndNegative',
      'conclusion.value = limited',
    ],
    predicates: [{ name: 'YieldsParticularConclusion', args: ['figure'] }],
    relations: [
      { predicate: 'restricts', from: 'secondFigure', to: 'particularConclusion' },
    ],
  },
  {
    id: 'syll-ex2-op-10-indifference',
    chunkId: 'syll-ex2-10-indifference-interchangeability',
    label: 'Extremes are indifferent; premises interchangeable',
    clauses: [
      'extremes.indifferent = true',
      'eachCanBe = majorOrMinor',
      'premises.interchangeable = true',
    ],
    predicates: [{ name: 'IsInterchangeable', args: ['extreme'] }],
    relations: [
      { predicate: 'permits', from: 'form', to: 'roleInterchange' },
    ],
  },
  {
    id: 'syll-ex2-op-11-universal-connection',
    chunkId: 'syll-ex2-11-universal-connection',
    label: 'Conclusion indifferent to positive/negative → reads as universal',
    clauses: [
      'conclusion.isPositive = true',
      'conclusion.isNegative = true',
      'conclusion.indifferentTo = determinacies',
      'conclusion = universalConnection',
      'contingency.firstSyllogism = positedInSecond',
    ],
    predicates: [{ name: 'IsUniversalConnection', args: ['conclusion'] }],
    relations: [
      { predicate: 'makesExplicit', from: 'secondFigure', to: 'firstFigure.contingency' },
    ],
  },
  {
    id: 'syll-ex2-op-12-self-sublating',
    chunkId: 'syll-ex2-12-mediation-self-sublating',
    label: 'Mediation contains singularity/immediacy and thereby self-sublates',
    clauses: [
      'mediation.has = {singularity, immediacy}',
      'singularity.isInfinitelyManifold = true',
      'singularity.isExternal = true',
      'mediation.selfSublates = true',
      'mediation.pointsBeyondItself = true',
    ],
    predicates: [{ name: 'IsSelfSublating', args: ['mediation'] }],
    relations: [
      { predicate: 'pointsTo', from: 'mediation', to: 'higherMediation(universal)' },
    ],
  },
  {
    id: 'syll-ex2-op-13-immediacy-abstract-universal',
    chunkId: 'syll-ex2-13-immediacy-abstract-universal',
    label: 'Immediacy in this figure corresponds to reflected abstract universal',
    clauses: [
      'immediacy.thisFigure = reflectedFirstImmediacy',
      'reflectedImmediacy = abstractUniversal',
      'abstractUniversal = existingInItself',
    ],
    predicates: [{ name: 'IsReflectedImmediacy', args: ['immediacy'] }],
    relations: [
      { predicate: 'equates', from: 'reflectedImmediacy', to: 'abstractUniversal' },
    ],
  },
  {
    id: 'syll-ex2-op-14-transition-new-form',
    chunkId: 'syll-ex2-14-transition-new-form',
    label: 'When middle is posited in its truth the figure becomes another form',
    clauses: [
      'transition.like = alteration',
      'base = qualitative',
      'base = immediacyOfSingularity',
      'singularity.conjoins = bySublatingParticular',
      'contingency.presentsItself = asSyllogisticInference',
      'whenMiddle.positedInTruth = thenNewForm',
    ],
    predicates: [{ name: 'TransformsForm', args: ['figure', 'newForm'] }],
    relations: [
      { predicate: 'yields', from: 'positedMiddle', to: 'newSyllogisticForm' },
    ],
  },
  // ============================================================================
  // THIRD FIGURE OPERATIONS (S-U-P)
  // ============================================================================
  {
    id: 'syll-ex3-op-1-declare-schema-reciprocity',
    chunkId: 'syll-ex3-1-overview',
    label: 'Declare S-U-P and reciprocal presupposition',
    clauses: [
      'schema = S-U-P',
      'thirdSyllogism.noImmediatePremise = true',
      'S-U.mediatedBy = firstSyllogism',
      'P-U.mediatedBy = secondSyllogism',
      'thirdFigure.presupposes = {firstFigure, secondFigure}',
      'thirdFigure.presupposedBy = {firstFigure, secondFigure}',
      'determination.broughtToCompletion = true',
    ],
    predicates: [{ name: 'IsSchema', args: ['S-U-P'] }],
    relations: [
      { predicate: 'reciprocallyPresupposes', from: 'thirdFigure', to: 'firstAndSecond' },
    ],
  },
  {
    id: 'syll-ex3-op-2-reciprocal-mediation',
    chunkId: 'syll-ex3-2-reciprocal-mediation',
    label: 'Each mediation lacks totality; immediacy\'s mediation lies outside',
    clauses: [
      'eachSyllogism.isMediation = true',
      'eachSyllogism.lacksTotality = true',
      'immediacy.mediation.liesOutside = sameSyllogism',
    ],
    predicates: [{ name: 'LacksTotality', args: ['syllogism'] }],
    relations: [
      { predicate: 'isAffectedBy', from: 'syllogism', to: 'externalImmediacy' },
    ],
  },
  {
    id: 'syll-ex3-op-3-formal-truth',
    chunkId: 'syll-ex3-3-formal-truth',
    label: 'S-U-P as truth of formal syllogism — middle is abstract universal',
    clauses: [
      'S-U-P = truthOfFormalSyllogism',
      'middle.is = abstractUniversal',
      'extremes.notContained = accordingToEssentialDeterminateness',
      'extremes.contained = onlyAsUniversality',
      'whatSupposedToBeMediated = notConjoined',
    ],
    predicates: [{ name: 'IsAbstractUniversal', args: ['middle'] }],
    relations: [
      { predicate: 'containsOnlyAs', from: 'middle', to: 'extremes.universalAspect' },
    ],
  },
  {
    id: 'syll-ex3-op-4-terms-immediate-content',
    chunkId: 'syll-ex3-4-terms-immediate-content',
    label: 'Terms possess immediate content indifferent to form',
    clauses: [
      'terms.haveImmediateContent = true',
      'immediacy.isIndifferentTo = form',
      'terms = formDeterminations',
      'formDeterminations.notReflected = intoContent',
    ],
    predicates: [{ name: 'HasImmediateContent', args: ['term'] }],
    relations: [
      { predicate: 'isIndifferentTo', from: 'content', to: 'form' },
    ],
  },
  {
    id: 'syll-ex3-op-5-indeterminate-universal',
    chunkId: 'syll-ex3-5-middle-indeterminate-universal',
    label: 'Middle unifies by abstraction; lacks the contained determinateness',
    clauses: [
      'middle.unifies = byAbstraction',
      'abstraction.from = determinateness',
      'middle = indeterminateUniversal',
      'syllogism.relationToConcept = needsExamining',
    ],
    predicates: [{ name: 'IsIndeterminateUniversal', args: ['middle'] }],
    relations: [
      { predicate: 'abstractsFrom', from: 'middle', to: 'extremes.determinateness' },
    ],
  },
  {
    id: 'syll-ex3-op-6-legitimacy-negative',
    chunkId: 'syll-ex3-6-legitimacy-negative-conclusion',
    label: 'Legitimacy requires symmetrical relation → negative conclusion',
    clauses: [
      'middleTerm.subsumes = bothExtremes',
      'middleTerm.notSubsumed = asSubject',
      'toConform.requires = bothConnectionsShareRelation',
      'thisRequires = negativeJudgment',
      'conclusion.necessarily = negative',
    ],
    predicates: [{ name: 'RequiresNegative', args: ['figure'] }],
    relations: [
      { predicate: 'rendersIndifferent', from: 'negativeConclusion', to: 'roles' },
    ],
  },
  {
    id: 'syll-ex3-op-7-indifference-fourth-figure',
    chunkId: 'syll-ex3-7-indifference-fourth-figure',
    label: 'Role indifference motivates fourth figure; distinction is idle',
    clauses: [
      'whichDetermination = predicateOrSubject (indifferent)',
      'whichExtreme = singularityOrParticularity (indifferent)',
      'whichPremise = majorOrMinor (indifferent)',
      'fourthFigure.origin = thisIndifference',
      'fourthFigure.unknownTo = Aristotle',
      'fourthFigure.value = void',
    ],
    predicates: [{ name: 'IsIdleDistinction', args: ['fourthFigure'] }],
    relations: [
      { predicate: 'derivesFrom', from: 'fourthFigure', to: 'roleIndifference' },
    ],
  },
  {
    id: 'syll-ex3-op-8-objective-significance',
    chunkId: 'syll-ex3-8-objective-significance-limited',
    label: 'Universality-as-middle is qualitative; conjunction depends on external mediation',
    clauses: [
      'middle.asUniversal = essentiallyUniversal',
      'universality.only = qualitativeOrAbstract',
      'determinateness.notContained = inMiddle',
      'conjunction.requires = externalMediation',
      'externalMediation = contingent',
    ],
    predicates: [{ name: 'IsQualitativeUniversal', args: ['middle'] }],
    relations: [
      { predicate: 'dependsOn', from: 'conjunction', to: 'externalMediation' },
    ],
  },
  {
    id: 'syll-ex3-op-9-relationless-uuu',
    chunkId: 'syll-ex3-9-relationless-figure-uuu',
    label: 'Abstracting yields U-U-U: relationless equality, not concrete mediation',
    clauses: [
      'universal.determinedAs = middleTerm',
      'determinateness.notContained = inMiddle',
      'middle.positedAs = indifferentAndExternal',
      'abstraction.yields = U-U-U',
      'U-U-U = relationlessSyllogism',
      'U-U-U.unity = externalEquality',
    ],
    predicates: [{ name: 'IsRelationlessFigure', args: ['U-U-U'] }],
    relations: [
      { predicate: 'expresses', from: 'U-U-U', to: 'externalEquality' },
    ],
  },
  // ============================================================================
  // FOURTH FIGURE OPERATIONS (U-U-U, MATHEMATICAL SYLLOGISM)
  // ============================================================================
  {
    id: 'syll-ex4-op-1-mathematical-form',
    chunkId: 'syll-ex4-1-mathematical-form',
    label: 'Mathematical syllogism — "if two things equal to a third, then equal"',
    clauses: [
      'mathematicalSyllogism = "if two equal to third, then equal"',
      'inherenceOrSubsumption = doneAwayWith',
      'third = mediatingTerm',
      'third.hasNoDetermination = againstExtremes',
      'eachTerm.canBe = mediatingTerm',
    ],
    predicates: [{ name: 'IsMathematicalSyllogism', args: [] }],
    relations: [
      { predicate: 'mediates', from: 'third', to: 'extremes' },
    ],
  },
  {
    id: 'syll-ex4-op-2-external-determination',
    chunkId: 'syll-ex4-2-external-determination',
    label: 'Which term mediates depends on external circumstances',
    clauses: [
      'whichTermMediates = dependsOnExternalCircumstances',
      'whichConnections = immediateOrMediated (external)',
      'determination.whollyExternal = true',
    ],
    predicates: [{ name: 'DependsOnExternal', args: ['mathematicalSyllogism'] }],
    relations: [
      { predicate: 'dependsOn', from: 'mediation', to: 'externalCircumstances' },
    ],
  },
  {
    id: 'syll-ex4-op-3-axiom-status',
    chunkId: 'syll-ex4-3-axiom-status',
    label: 'Mathematical syllogism as axiom — self-explanatory',
    clauses: [
      'mathematicalSyllogism.ranksAs = axiom',
      'axiom = firstSelfExplanatoryProposition',
      'axiom.notCapableOf = proof',
      'axiom.notInNeedOf = proof',
      'axiom.notPresupposes = anythingElse',
      'axiom.notDerivedFrom = anythingElse',
    ],
    predicates: [{ name: 'IsAxiom', args: ['mathematicalSyllogism'] }],
    relations: [
      { predicate: 'is', from: 'mathematicalSyllogism', to: 'axiom' },
    ],
  },
  {
    id: 'syll-ex4-op-4-formalism-abstraction',
    chunkId: 'syll-ex4-4-formalism-abstraction',
    label: 'Self-evidence lies in formalism — abstraction from qualitative diversity',
    clauses: [
      'selfEvidence.liesIn = formalism',
      'formalism = abstractionFromQualitativeDiversity',
      'onlyAdmits = quantitativeEqualityOrInequality',
      'quantitativeDetermination = byAbstraction',
      'abstraction.from = {qualitativeDifferentiation, conceptDeterminations}',
    ],
    predicates: [{ name: 'IsFormalism', args: ['mathematicalSyllogism'] }],
    relations: [
      { predicate: 'abstractsFrom', from: 'formalism', to: 'qualitativeDiversity' },
    ],
  },
  {
    id: 'syll-ex4-op-5-examples-magnitude',
    chunkId: 'syll-ex4-5-examples-magnitude',
    label: 'Examples — equality only according to magnitude',
    clauses: [
      'linesFigures.equal = onlyAccordingToMagnitude',
      'triangle.equalToSquare = onlyByMagnitude',
      'concept.notEnters = intoSyllogism',
      'noConceptualComprehension = true',
      'understanding.notFaced = byConceptDeterminations',
    ],
    predicates: [{ name: 'EqualityOnlyByMagnitude', args: ['mathematicalSyllogism'] }],
    relations: [
      { predicate: 'understands', from: 'mathematicalSyllogism', to: 'magnitudeOnly' },
    ],
  },
  {
    id: 'syll-ex4-op-6-indigence-abstractness',
    chunkId: 'syll-ex4-6-indigence-abstractness',
    label: 'Self-evidence rests on indigence and abstractness',
    clauses: [
      'selfEvidence.restsOn = indigenceAndAbstractness',
      'modeOfThought = indigentAndAbstract',
    ],
    predicates: [{ name: 'RestsOnIndigence', args: ['selfEvidence'] }],
    relations: [
      { predicate: 'restsOn', from: 'selfEvidence', to: 'indigenceAbstractness' },
    ],
  },
  {
    id: 'syll-ex4-op-7-positive-result',
    chunkId: 'syll-ex4-7-positive-result',
    label: 'Positive result — not just abstraction',
    clauses: [
      'result.not = justAbstraction',
      'negativity.has = positiveSide',
      'abstractDeterminateness.other = posited',
      'determinateness.become = concrete',
    ],
    predicates: [{ name: 'HasPositiveResult', args: ['syllogismOfExistence'] }],
    relations: [
      { predicate: 'has', from: 'negativity', to: 'positiveSide' },
    ],
  },
  {
    id: 'syll-ex4-op-8-reciprocal-presupposition',
    chunkId: 'syll-ex4-8-reciprocal-presupposition',
    label: 'Reciprocal presupposition — mediation based on mediation',
    clauses: [
      'syllogismsOfExistence.presuppose = oneAnother',
      'extremes.trulyConjoined = onlyIfOtherwiseUnited',
      'middleTerm.oughtToBe = conceptualUnity',
      'middleTerm.is = formalDeterminateness',
      'middleTerm.notPosited = asConcreteUnity',
      'presupposed = mediationOfOtherTwo',
    ],
    predicates: [{ name: 'HasReciprocalPresupposition', args: ['syllogismsOfExistence'] }],
    relations: [
      { predicate: 'presupposes', from: 'syllogism', to: 'otherSyllogisms' },
    ],
  },
  {
    id: 'syll-ex4-op-9-mediation-reflection',
    chunkId: 'syll-ex4-9-mediation-of-reflection',
    label: 'Mediation based on mediation — mediation of reflection',
    clauses: [
      'mediation.not = basedOnGivenImmediacy',
      'mediation = basedOnMediation',
      'mediation.not = quantitative',
      'mediation.not = abstractsFromForm',
      'mediation = selfReferringMediation',
      'mediation = mediationOfReflection',
    ],
    predicates: [{ name: 'IsMediationOfReflection', args: ['mediation'] }],
    relations: [
      { predicate: 'is', from: 'mediation', to: 'selfReferring' },
    ],
  },
  {
    id: 'syll-ex4-op-10-circle-totality',
    chunkId: 'syll-ex4-10-circle-totality',
    label: 'Circle of reciprocal presupposing forms totality',
    clauses: [
      'circle = reciprocalPresupposing',
      'circle.bringsToClosure = turningBackIntoItself',
      'presupposing.forms = totality',
      'other.included = withinCircle',
      'other.notOutside = byAbstraction',
    ],
    predicates: [{ name: 'FormsTotality', args: ['circle'] }],
    relations: [
      { predicate: 'forms', from: 'circle', to: 'totality' },
    ],
  },
  {
    id: 'syll-ex4-op-11-determinations-middle',
    chunkId: 'syll-ex4-11-determinations-as-middle',
    label: 'Each determination has occupied place of middle term',
    clauses: [
      'eachDetermination.occupied = placeOfMiddleTerm',
      'middleTerm.asImmediate = particularity',
      'middleTerm.determinedAs = {singularity, universality}',
      'eachDetermination.occupied = placesOfExtremes',
    ],
    predicates: [{ name: 'OccupiedMiddlePlace', args: ['determination'] }],
    relations: [
      { predicate: 'occupied', from: 'determination', to: 'middlePlace' },
    ],
  },
  {
    id: 'syll-ex4-op-12-concrete-identity',
    chunkId: 'syll-ex4-12-positive-result-concrete-identity',
    label: 'Positive result: mediation through concrete identity',
    clauses: [
      'negativeResult = dissolutionIntoQuantitative',
      'positiveResult = mediationThroughConcreteIdentity',
      'mediation.notThrough = singleQualitativeDeterminateness',
      'mediation.through = concreteIdentityOfDeterminacies',
    ],
    predicates: [{ name: 'MediationThroughConcreteIdentity', args: ['positiveResult'] }],
    relations: [
      { predicate: 'occurs', from: 'mediation', to: 'concreteIdentity' },
    ],
  },
  {
    id: 'syll-ex4-op-13-deficiency-formalism',
    chunkId: 'syll-ex4-13-deficiency-formalism',
    label: 'Deficiency and formalism of three figures',
    clauses: [
      'deficiency = singleDeterminatenessAsMiddleTerm',
      'formalism = singleDeterminatenessAsMiddleTerm',
      'threeFigures.deficient = inThis',
    ],
    predicates: [{ name: 'HasDeficiency', args: ['threeFigures'] }],
    relations: [
      { predicate: 'consistsIn', from: 'deficiency', to: 'singleDeterminateness' },
    ],
  },
  {
    id: 'syll-ex4-op-14-transition-reflection',
    chunkId: 'syll-ex4-14-transition-reflection',
    label: 'Transition: syllogism of existence to syllogism of reflection',
    clauses: [
      'mediation.determinedAs = indifferenceOfAbstractDeterminations',
      'mediation.determinedAs = positiveReflection',
      'syllogismOfExistence.passedOver = intoSyllogismOfReflection',
    ],
    predicates: [{ name: 'TransitionsToReflection', args: ['syllogismOfExistence'] }],
    relations: [
      { predicate: 'transitions', from: 'syllogismOfExistence', to: 'syllogismOfReflection' },
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
