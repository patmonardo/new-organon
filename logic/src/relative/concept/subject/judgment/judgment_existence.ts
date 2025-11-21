import type { Chunk, LogicalOperation } from '../syllogism/index';

/**
 * JUDGMENT OF EXISTENCE — Complete Structure
 *
 * NOTE: The judgment of existence is immediate—the first judgment where no reflection
 * and no movement of determinations has been found. It is also called the judgment of
 * inherence because the subject is the immediate and essential term, and the predicate
 * has its foundation in the subject.
 *
 * Structure:
 * Introduction: Judgment of Existence overview
 * a. The positive judgment (3 parts)
 * b. The negative judgment (3 parts)
 * c. The infinite judgment (with transition to reflection)
 *
 * Transition: Judgment of Existence passes over into Judgment of Reflection
 */

// ============================================================================
// INTRODUCTION: JUDGMENT OF EXISTENCE — OVERVIEW
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-exist-intro-1-truth-and-immediacy',
    title: 'Judgment of existence — truth and immediacy',
    text: `In the subjective judgment we expect to see one and the same object double, once in its singular actuality, and again in its essential identity or in its concept: the singular raised into its universality or, what is the same thing, the universal made singular into its actuality. The judgment is thus truth, for it is the agreement of the concept and reality. But it is not at first constituted in this way, for at first the judgment is immediate, since as yet no reflection and no movement of the determinations has been found in it. This immediacy renders the first judgment a judgment of immediate existence; we can also call it a qualitative judgment, but only in so far as quality does not apply to the determinateness of being alone but also extends to the universality which, because of its simplicity, likewise has the form of immediacy.`,
  },
  {
    id: 'j-exist-intro-2-judgment-of-inherence',
    title: 'Judgment of inherence — subject as immediate basis',
    text: `The judgment of existence is also the judgment of inherence because, though immediacy is its determination, it is the subject that in the distinction between subject and predicate is the immediate and hence the first and the essential term in the judgment, and the predicate consequently takes on the form of something that does not subsist on its own but has its foundation in the subject.`,
  },
];

// ============================================================================
// a. THE POSITIVE JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-exist-pos-1-subject-predicate-immediacy',
    title: 'Subject and predicate as immediate determinations',
    text: `The subject and predicate, as we have just said, are names at first that receive their actual determination only as the judgment runs its course. However, as sides of the judgment, the judgment being the posited determinate concept, they have the determination of moments of the concept, but, on account of their immediacy, this determination is as yet quite simple, still not enriched by mediation and also still caught up in the abstract opposition of abstract singularity and abstract universality. The predicate, to speak of it first, is the abstract universal; this abstract is conditioned by mediation, by the sublation of singularity and particularity, but so far such a mediation is here only a presupposition. In the sphere of the concept there can be no other immediacy than the one that contains mediation in and for itself and has arisen only through its sublation; this is the immediacy of the universal. Thus qualitative being also is in its concept a universal; as being, however, the immediacy is not yet posited as such; it is only as universality that immediacy is the concept determination in which it is posited that negativity essentially belongs to it. This connection is given in the judgment in which universality is the predicate of a subject. Similarly the subject is an abstract singular, or the immediate which is supposed to be such and therefore the singular as a something in general. The subject constitutes, therefore, the abstract side of the judgment, the side in it according to which the concept has passed over into externality. As these two concept determinations are determined, so is also their connection, the "is" or the copula; it too can have no other meaning than that of an immediate, abstract being. It is because of this connection, which still does not contain any mediation or negation, that this judgment is called "positive."`,
  },
  {
    id: 'j-exist-pos-2-proposition-singular-universal',
    title: 'Proposition: the singular is universal',
    text: `The first pure expression of the positive judgment is, therefore, the proposition: the singular is universal. This expression must not be put in the form of "A is B," for A and B are totally formless and hence meaningless names, whereas judgment in general, and therefore already the judgment of existence, has determinations of the concept for its extremes. "A is B" can stand just as well for any mere proposition as for a judgment. But what is asserted in every judgment, even one more richly determined in form, is the proposition that has this determined content, namely, "the singular is universal," for every judgment is in principle also an abstract judgment. However, if no thought is given to the fact that with every judgment, the positive at least, the assertion is made that the singular is universal, this happens either because no attention is given to the determinate form differentiating subject and object (for it is taken for granted that the judgment is nothing but the connecting of two concepts) or also likely because the further content of the judgment, "Gaius is learned," or "the rose is red," comes drifting in before the mind, and the latter, busy with the picture of Gaius etc., fails to reflect on the form; even though, at least, such a content as "Gaius," which is the one that usually has to be dragged in as an example, is much less interesting than the form, and is indeed chosen because it is uninteresting, not to divert attention from the form to itself.`,
  },
  {
    id: 'j-exist-pos-3-objective-meaning-perishability',
    title: 'Objective meaning: perishability and universal subsistence',
    text: `The objective meaning of the proposition stating that the singular is universal conveys, as already incidentally noted, both the perishableness of singular things and their positive subsistence in the concept in general. The concept itself is imperishable, but that which emerges from it in its division is subjected to alteration and to falling back into its universal nature. But the universal, conversely, gives itself a determinate existence. Just as essence goes out into reflective shine in its determinations; or ground into concrete existence in appearance; and substance into manifestation in its accidents, so does the universal resolve itself into the singular; judgment is this resolution of the universal, the development of the negativity which, implicitly, it already is. This last circumstance is expressed by the converse proposition, "the universal is singular," which is also equally spoken in the positive judgment.`,
  },
  {
    id: 'j-exist-pos-4-subject-as-concrete',
    title: 'Subject as concrete — manifold properties',
    text: `The subject, the immediate singular at first, is in the judgment itself referred to its other, namely the universal; it is thereby posited as the concrete according to the category of being, as a something of many qualities; or as the concrete of reflection, a thing of manifold properties, an actual of manifold possibilities, a substance of precisely such accidents. Because these manifolds here belong to the subject of the judgment, the something, the thing, etc., is in its qualities, properties, or accidents, reflected into itself, or continues across them, maintaining itself in them and them in itself. Positedness or determinateness belongs to being which is in and for itself. The subject is therefore inherently the universal. The predicate, on the contrary, being this universality not as real or concrete, but as abstract, is in contrast to the subject the determinateness; it contains only one moment of the subject's totality to the exclusion of the others. On account of this negativity, which as an extreme of the judgment is at the same time self-referring, the predicate is an abstract singular. For instance, in the proposition, "the rose is fragrant," the predicate expresses only one of the many properties of the rose; it isolates it, whereas in the subject the property is joined with the others; likewise in the dissolution of the thing, the manifold properties that inhere in it become isolated in acquiring self-subsistence as materials. From this side, then, the proposition of the judgment says: the universal is singular.`,
  },
  {
    id: 'j-exist-pos-5-reciprocity-twofold-result',
    title: 'Reciprocity of determination: twofold result',
    text: `By juxtaposing this reciprocal determination of subject and predicate in the judgment, we thus obtain this twofold result. (1) Immediately, the subject is indeed an existent or the singular, while the predicate is the universal. But because the judgment connects the two, and the subject is determined as universal by the predicate, the subject is then the universal. (2) The predicate is determined in the subject, for it is not a determination in general but the determination rather of the subject. "The rose is fragrant." This fragrance is not some indeterminate fragrance or other, but the fragrance of the rose. The predicate is therefore a singular. Now since the subject and predicate stand related in the judgment, they should retain the opposition of concept determinations; likewise, in the reciprocity of causality, before the latter attains its truth, the two sides are still supposed to remain self-subsistent and mutually opposed as against the equality of their determination. Therefore, when the subject is determined as universal, the predicate should not also be taken in its determination of universality, for then we would have no judgment; it must rather be taken only in its determination of singularity. And if the subject is determined as singular, then the predicate is to be taken as universal.`,
  },
  {
    id: 'j-exist-pos-6-empty-identical-propositions',
    title: 'Empty identical propositions — judgment sublated',
    text: `If we reflect on the mere identity above, then we have these two identical propositions, "the singular is singular," "the universal is universal," in which the sides of the judgment would have completely fallen apart; only the self-reference of each is expressed while the reference connecting them to each other is dissolved; and thus the judgment would be sublated.`,
  },
  {
    id: 'j-exist-pos-7-form-vs-content',
    title: 'Form vs content — two propositions in one',
    text: `Of the two propositions we drew, the first, "the universal is singular," expresses the judgment according to its content, as an isolated determination in the predicate and as the totality of determinations in the subject. The other, "the singular is universal," expresses it according to form as immediately given through the judgment itself. In the immediate positive judgment, the extremes are still simple: form and content are therefore still united. Or, in other words, it does not consist of two propositions; the twofold connection that it yielded immediately constitutes the one positive judgment. For its extremes are (a) the self-subsisting abstract determinations of judgment, and (b) each side of the determination is determined through the other by virtue of the copula connecting them. Implicitly, however, the difference of form and content is for this reason present in it, as we have seen; and indeed, what the first proposition contains, that the singular is universal, belongs to form, for the proposition expresses the immediate determinateness of the judgment. The relation, on the contrary, which the other proposition expresses, that the universal is singular or that the subject is determined as universal whereas the predicate is determined as particular or singular, concerns the content for its determinations are only the result of an immanent reflection by virtue of which the immediate determinacies of the judgment are sublated and the form is thereby converted into an identity that has withdrawn into itself and persists over against the distinction of form: it converts itself into content.`,
  },
  {
    id: 'j-exist-pos-8-union-particularity-empty',
    title: 'Union into particularity — empty identical proposition',
    text: `If now the two propositions, the one of form and the other of content, (Subject) (Predicate) The singular is universal, The universal is singular, were to be united because they are contained in the one positive judgment, so that both, the subject as well as the predicate, were determined as the unity of singularity and universality, then both the subject and predicate would be the particular, and this must be recognized as implicitly their inner determination. However, this combination would be arrived at only through an external reflection; moreover, the proposition that results from it, "the particular is the particular," would no longer be a judgment but an empty identical proposition as were the two propositions already found in the positive judgment, "the singular is singular," and "the universal is universal." Singularity and universality cannot yet be united into particularity, because in the positive judgment they are still posited as immediate. Or again, the judgment must still be distinguished according to its form and its content, because the subject and predicate are themselves still distinguished as immediacy and mediated, or because the judgment, according to its connection, is both the self-subsistence of the connected terms and their reciprocal determination or mediation.`,
  },
  {
    id: 'j-exist-pos-9-inadequacy-of-form',
    title: 'Inadequacy of form — singular not universal',
    text: `In first place, then, the meaning of the judgment when considered according to its form is that the singular is universal. But in fact such an immediate singular is definitely not universal; its predicate is of wider extension, does not correspond to it. The subject is a being existing immediately for itself, and hence the opposite of that abstraction, of that universality posited through mediation that was supposed to be predicated of it.`,
  },
  {
    id: 'j-exist-pos-10-inadequacy-of-content',
    title: 'Inadequacy of content — universal not singular',
    text: `In second place, if the judgment is considered according to its content, or as the proposition, "the universal is singular," then the subject is a universe of qualities, an infinitely determined concrete universe, and since its determinacies are as yet qualitites, properties, or accidents, its totality is the bad infinite plurality of them. Such a subject, therefore, is not at all the one single property that its predicate declares. Consequently, both propositions must be united, and the positive judgment must be posited as negative instead.`,
  },
);

// ============================================================================
// b. THE NEGATIVE JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-exist-neg-1-positive-not-true',
    title: 'Positive judgment not true — truth in negative',
    text: `We spoke earlier of the common notion that whether the content of a judgment is true or false depends solely on the content itself, since logical truth concerns only the form and its only requirement is that such content shall not contradict itself. Nothing else is reckoned as the form of judgment except that the latter is a connection of two concepts. But we have seen that these two concepts are not just the relationless determination of a sum, but that they relate to each other as singular and universal. These are the determinations that constitute the truly logical content and also, abstracted in that way, the content of the positive judgment; whatever other content is in a judgment ("the sun is round," "Cicero was a great Roman orator," "it is daytime now," etc.) does not concern the judgment as such; the judgment only says that the subject is predicate, or, since these are only names, that the singular is universal and vice versa. It is because of this purely logical content that the positive judgment is not true but has its truth in the negative judgment. In judgment, so it is required, the content simply ought not to contradict itself; but it does contradict itself in the positive judgment, as we have just seen.`,
  },
  {
    id: 'j-exist-neg-2-singular-is-particular',
    title: 'Singular is particular — mediated determination',
    text: `The positive judgment first attains its truth in the negative judgment: the singular is not abstractly universal, but rather, the predicate of the singular, because it is such a predicate, or because, if considered by itself without reference to the subject, it is an abstract universal, is for that very reason itself something determinate; from the start, therefore, the singular is a particular. Furthermore, with respect to the other proposition that the positive judgment contains, the meaning of the negative judgment is that the universal is not abstractly singular but that this predicate, "singular," by the very fact that it is a predicate, or because it refers to a universal subject, is more than just mere singularity, and the universal, accordingly, is from the start equally a particular. Since this universal, as subject, is itself in the judgment determination of singularity, the two propositions both reduce to one: "the singular is a particular."`,
  },
  {
    id: 'j-exist-neg-3-particularity-mediation',
    title: 'Particularity as mediated — not external reflection',
    text: `We may remark that (a) the particularity that here comes to the predicate has already come up for consideration before; here, however, it is not posited by external reflection but has arisen rather as mediated by the negative connection indicated in the judgment. (b) This determination results here only for the predicate. In the immediate judgment, the judgment of existence, the subject is the underlying basis; the determination seems at first, therefore, to occur in the predicate. But in fact this first negation cannot as yet be a determination, or cannot truly be the positing of the singular, for such a positing is only a second moment, the negative of the negative.`,
  },
  {
    id: 'j-exist-neg-4-positive-expression',
    title: 'Positive expression of negative judgment',
    text: `The singular is a particular: this is the positive expression of the negative judgment. This expression, therefore, is not the positive judgment itself, for the latter, because of its immediacy, has an abstraction for its extremes, while the particular, precisely through the positing of the judgment connection, results as the first mediated determination. But this determination is not to be taken only as a moment of the extremes, but also as the determination of the connection, as it truly is from the start; in other words, the judgment is also to be considered as negative.`,
  },
  {
    id: 'j-exist-neg-5-connection-separation',
    title: 'Connection as separation — positive as negative',
    text: `This transition is founded on the relation of the extremes and on their connection in the judgment as such. The positive judgment is the connection of the singular and the universal which are such immediately and each, therefore, is not at the same time what the other is. The connection is therefore just as essentially separation, or negative; for this reason the positive judgment was to be posited as negative. There was no need, therefore, for the logicians to make such a fuss about the not of the negative judgment being attached to the copula. In the judgment, the determination of the extremes is equally a determinate connection. The judgment determination, or the extreme, is not the purely qualitative one of immediate being that only stands over against an other outside it. Nor is it the determination of reflection, which, in accordance with its general form, behaves positively and negatively, posited in either case as exclusive, only implicitly identical with the other. The judgment determination, as the determination of the concept, is a universal within, posited as extending continuously in its other. Conversely, the judgment connection is the same determination as the extremes have; for it is precisely this universality and continuous extension of each into the other; in so far as these are distinguished, the connection also has negativity in it.`,
  },
  {
    id: 'j-exist-neg-6-not-attached-to-predicate',
    title: 'Not attached to predicate — not-universal as particular',
    text: `The just stated transition from the form of the connection to the form of the determination has the immediate consequence that the not of the copula must just as equally be attached to the predicate and that the latter must be determined as the not-universal. But, through a no less immediate consequence, the not-universal is the particular. If the focus is on the negative according to the totally abstract determination of immediate non-being, then the predicate is the totally indeterminate not-universal. This is the determination which is normally treated in logic in connection with the contradictory concepts, and the further point is made, a point considered important that in the negative of a concept one should only focus on the negative, taking it as the mere indeterminate extent of the other of the positive concept. Thus the mere not-white would be just as much red, yellow, blue, etc. as black.`,
  },
  {
    id: 'j-exist-neg-7-conceptualized-negation',
    title: 'Conceptualized negation vs unconceptualized not',
    text: `White, however, is an unconceptualized determination of intuition; the not of white is equally, then, unconceptualized not-being, the abstraction that came in for consideration at the very beginning of the Logic where becoming was recognized to be its closest truth. To use as an example, in the consideration of judgment determinations, an unconceptualized content of this sort, drawn from intuition and the imagination, and to take the determinations of being, and of reflection, as such judgment determinations, is the same uncritical practice as when Kant applies the concepts of the understanding to the infinite idea of reason, the so-called thing-in-itself; the concept, to which the judgment proceeding from it also belongs, is the true thing-in-itself or the rational; those other determinations belong to being and essence; they are not yet forms developed into the shape where they are in their truth, in the concept. If we stop at white, red, as representations of the senses, then we call concept what is only a determination of pictorial representation. This is common practice. But then, surely, the not-white, the not-red, will be nothing positive, just as the not-triangular will be something totally indeterminate, for a determination based as such on number and quantum is essentially something indifferent, void of concept. Yet, like non-being itself, such a sensuous content ought to be conceptualized; ought to shed that indifference and abstract immediacy with which it is affected in the blind immobility of pictorial representation. Already in the sphere of immediate existence, the non-being which is otherwise void of thought becomes limit, and by virtue of this limit the something refers to an other despite itself. In the sphere of reflection, on the other hand, it is the negative that refers essentially to a positive, and is thereby determined; a negative is no longer that indeterminate non-being, for it is posited to be only to the extent that the positive stands over against it, and as third comes their ground; the negative is thus held circumscribed in a sphere within which the non-being of one is something determinate. But it is all the more in the absolutely fluid continuity of the concept that the not is immediately a positive, and the negation is not just determinateness but is taken up into universality and is posited as identical with it. The non-universal is therefore directly the particular.`,
  },
  {
    id: 'j-exist-neg-8-negation-preserves-subject',
    title: 'Negation preserves subject — denies determinateness',
    text: `Since negation has to do with the connection of judgment, and we are considering the negative judgment still as such, the latter is in the first instance still a judgment; we thus have the relation of subject and predicate, or of singularity and universality, and their connection, the form of the judgment. The subject, as the immediate underlying basis, remains untouched by the negation; it retains, therefore, its determination of having a predicate, or its reference to the universality. Consequently, what is negated in the predicate is not the universality as such, but the abstraction or the determinateness of the predicate that appeared as content in contrast to that universality. The negative judgment is not, therefore, total negation; the universal sphere which contains the predicate remains standing; the connection of subject and the predicate is therefore still essentially positive; the yet remaining determination of the predicate is no less connection. When it is said that, for instance, the rose is not red, only the determinateness of the predicate is thereby denied and thus separated from the universality which equally attaches to it; the universal sphere, color, is retained; if the rose is not red, it is nonetheless assumed that it has a color, though another color. From the side of this universal sphere, the judgment is still positive.`,
  },
  {
    id: 'j-exist-neg-9-particularity-retained',
    title: 'Particularity retained — transformed determinateness',
    text: `"The singular is a particular." This positive form of the negative judgment immediately expresses that the particular contains universality. In addition, it also expresses that the predicate is not just a universal but also one which is still determinate. The negative form contains the same, for although the rose, for instance, is not red, it is supposed, nevertheless, not only still to retain the universal sphere of color as predicate, but to have some other determinate color as well; the singularity of determinateness of the rose is therefore only sublated; and not only is the universal sphere left standing but determinateness too is retained, although transformed into an indeterminate determinateness, a universal determinateness, that is to say, into particularity.`,
  },
  {
    id: 'j-exist-neg-10-particularity-mediates',
    title: 'Particularity mediates singularity and universality',
    text: `The particularity that has resulted as the positive determination of the negative judgment is the term mediating singularity and universality; so the negative judgment is now that which provides in general the mediation for the third step, that of the reflection of the judgment of existence into itself. This judgment is according to its objective meaning only the moment of the alteration of accidents, or, in the sphere of existence, of the singularized properties of the concrete. Through this alteration, the full determinateness of the predicate, or the concrete, emerges as posited.`,
  },
  {
    id: 'j-exist-neg-11-singular-not-particular',
    title: 'Singular not particular — requires further negation',
    text: `"The singular is particular" is what the positive expression of the negative judgment says. But the singular is also not particular, for particularity is of wider extension than singularity; it is a predicate, therefore, that does not correspond to the subject, one in which the latter, therefore, does not as yet have its truth. "The singular is only a singular": this is a negativity that refers to nothing else, be it positive or negative, except itself. The rose is not a thing of some color or other, but one that only has the one determinate color which is the rose-color. The singular is not an indeterminate determinate but the determinate determinate.`,
  },
  {
    id: 'j-exist-neg-12-negation-of-negation',
    title: 'Negation of negation — restoration of concrete',
    text: `This negation of the negative judgment appears, when one starts from its positive form, to be again a first negation. But this is not what it is. The negative judgment is again, in and for itself, already the second negation or the negation of negation, and this, what it is in and for itself, is to be posited. To wit: the judgment negates the determinateness of the predicate of the positive judgment, its abstract universality, or, considered as content, the singular quality that it possesses of the subject. But the negation of the determinateness is already the second negation, hence the infinite turning back of the singularity into itself. With this, therefore, the restoration of the concrete totality of the subject has taken place, or rather, the subject is now for the first time posited as singular, for through the negation and the sublation of that negation it is mediated with itself. The predicate, for its part, has thereby passed over from the first universality to absolute determinateness and made itself equal to the subject. Thus the judgment says: "the singular is singular." From the other side, since the subject was equally to be taken as a universal, and since in the negative judgment the predicate, which as against that subject is the singular, expanded into particularity; moreover, since now the negation of this determinateness is equally the purification of the universality contained in the predicate, this judgment also says: "the universal is the universal."`,
  },
  {
    id: 'j-exist-neg-13-infinite-judgment-emergence',
    title: 'Infinite judgment emerges — whole extent negated',
    text: `In these two judgments, which were earlier obtained through external reflection, the predicate is already expressed in its positivity. But the negation of the negative judgment must itself first appear in the form of a negative judgment. It has just been shown that there still remained in this judgment a positive connection of subject and predicate as well the universal sphere of the latter. From this side, the negative judgment thus contains a universality which is more purified of limitation than was contained by the positive judgment and is for this reason all the more to be negated of the subject as a singular. In this manner, the whole extent of the predicate is negated, and there is no longer any positive connection between it and the subject. This is the infinite judgment.`,
  },
);

// ============================================================================
// c. THE INFINITE JUDGMENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'j-exist-inf-1-negative-infinite-nonsensical',
    title: 'Negative infinite — nonsensical judgment',
    text: `The negative judgment is as little of a true judgment as the positive. But the infinite judgment which is supposed to be its truth is, according to its negative expression, the negative infinite, a judgment in which even the form of judgment is sublated. But this is a nonsensical judgment. It ought to be a judgment, and hence contains a connection of subject and predicate; but any such connection ought not at the same time to be there. The name of the infinite judgment does indeed occur in the common textbooks of logic, but without any clarification as to its meaning. Examples of negatively infinite judgments are easy to come by. It is a matter of picking determinations, one of which does not contain not just the determinateness of the other but its universal sphere as well, and of combining them negatively as subject and predicate, as when we say, for example, that spirit is not red, yellow, etc., is not acid, not alkali, etc., or that the rose is not an elephant, the understanding is not a table, and the like. These judgments are correct or true, as it is said, and yet, any such truth notwithstanding, nonsensical and fatuous. Or, more to the point, they are not judgments at all.`,
  },
  {
    id: 'j-exist-inf-2-crime-as-infinite-judgment',
    title: 'Crime as infinite judgment — negating universal sphere',
    text: `A more realistic example of the infinite judgment is the evil action. In civil litigation, when a thing is negated as the property of another party, it is still conceded that the same thing would indeed belong to that party if the latter had a right to it. It is only under the title of right that the possession of it is challenged; in the negative judgment, therefore, the universal sphere, "right," is still acknowledged and maintained. But crime is the infinite judgment that negates, not only the particular right, but the universal sphere, the right as right. It has correctness, in the sense that it is an effective action, but since it stands in a thoroughly negative fashion with respect to the morality that constitutes its sphere, it is nonsensical.`,
  },
  {
    id: 'j-exist-inf-3-positive-element-negation-of-negation',
    title: 'Positive element — negation of negation',
    text: `The positive element of the infinite judgment, the negation of the negation, is the reflection of singularity into itself by virtue of which the singularity is first posited as the determinate determinate. "The singular is singular" is what the infinite judgment said according to that reflection. In the judgment of existence, the subject is as the immediate singular, hence more of just a something in general. Through the mediation of the negative and infinite judgment, it is posited as singular for the first time. The singular is thus posited as expanding into its predicate, which is identical with it; to the same extent, therefore, universality is also no longer anything immediate but a summing of distincts. The positively infinite judgment equally says, "the universal is universal," and in this the universal is posited also as a turning back into itself.`,
  },
  {
    id: 'j-exist-inf-4-judgment-sublates-itself',
    title: 'Judgment sublates itself — negative and positive infinite',
    text: `Now through the reflection of the judgment determinations into themselves, the judgment has sublated itself; in the negatively infinite judgment, the difference is, so to speak, too great for it still to remain a judgment; subject and predicate have no positive connection whatsoever to each other; in the positively infinite judgment, on the contrary, only identity is present, and because of this total lack of difference there is no longer a judgment.`,
  },
  {
    id: 'j-exist-inf-5-transition-to-reflection',
    title: 'Transition to judgment of reflection',
    text: `More precisely, it is the judgment of existence that has sublated itself and, consequently, there is posited what the copula of the judgment contains, namely that in its identity the qualitative extremes are sublated. But since this unity is the concept, it is immediately torn apart and is a judgment, but one whose terms are no longer immediately determined but are reflected into themselves. The judgment of existence has passed over into the judgment of reflection.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ============================================================================
  // INTRODUCTION OPERATIONS
  // ============================================================================
  {
    id: 'j-exist-op-intro-1-truth-and-immediacy',
    chunkId: 'j-exist-intro-1-truth-and-immediacy',
    label: 'Declare judgment of existence as immediate truth',
    clauses: [
      'judgment = truth (agreementOfConceptAndReality)',
      'judgment.atFirst = immediate',
      'judgment.hasNo = reflectionOrMovement',
      'judgment = judgmentOfImmediateExistence',
      'judgment = qualitativeJudgment',
    ],
    predicates: [{ name: 'IsJudgmentOfExistence', args: [] }],
    relations: [
      { predicate: 'expresses', from: 'judgment', to: 'truth' },
    ],
  },
  {
    id: 'j-exist-op-intro-2-judgment-of-inherence',
    chunkId: 'j-exist-intro-2-judgment-of-inherence',
    label: 'Declare judgment of inherence — subject as basis',
    clauses: [
      'judgmentOfExistence = judgmentOfInherence',
      'subject = immediateAndEssential',
      'predicate.hasFoundation = inSubject',
      'predicate.notSubsists = onItsOwn',
    ],
    predicates: [{ name: 'IsJudgmentOfInherence', args: [] }],
    relations: [
      { predicate: 'grounds', from: 'subject', to: 'predicate' },
    ],
  },
  // ============================================================================
  // POSITIVE JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-exist-op-pos-1-subject-predicate-immediacy',
    chunkId: 'j-exist-pos-1-subject-predicate-immediacy',
    label: 'Declare subject and predicate as immediate determinations',
    clauses: [
      'subject = abstractSingular',
      'predicate = abstractUniversal',
      'mediation.presupposed = true',
      'mediation.notEnacted = true',
      'copula = immediateAbstractBeing',
      'judgment.kind = positive',
    ],
    predicates: [{ name: 'IsPositiveJudgment', args: [] }],
    relations: [
      { predicate: 'connects', from: 'copula', to: 'subjectPredicate' },
    ],
  },
  {
    id: 'j-exist-op-pos-2-proposition-singular-universal',
    chunkId: 'j-exist-pos-2-proposition-singular-universal',
    label: 'Declare proposition: the singular is universal',
    clauses: [
      'proposition.form = "singular is universal"',
      'proposition.not = "A is B" (formless)',
      'everyJudgment.asserts = singularIsUniversal',
    ],
    predicates: [{ name: 'IsPropositionSingularUniversal', args: [] }],
    relations: [
      { predicate: 'expresses', from: 'proposition', to: 'positiveJudgmentForm' },
    ],
  },
  {
    id: 'j-exist-op-pos-3-objective-meaning',
    chunkId: 'j-exist-pos-3-objective-meaning-perishability',
    label: 'Encode objective meaning: perishability and universal subsistence',
    clauses: [
      'singular.perishable = true',
      'singular.subsistsInConcept = true',
      'concept.imperishable = true',
      'universal.resolvesInto = singular',
      'judgment = resolutionOfUniversal',
      'converseProposition = "universal is singular"',
    ],
    predicates: [{ name: 'FormalizesObjectiveMeaning', args: [] }],
    relations: [
      { predicate: 'resolves', from: 'universal', to: 'singular' },
    ],
  },
  {
    id: 'j-exist-op-pos-4-subject-concrete',
    chunkId: 'j-exist-pos-4-subject-as-concrete',
    label: 'Encode subject as concrete with manifold properties',
    clauses: [
      'subject.positedAs = concrete',
      'subject.contains = manifoldProperties',
      'subject.inherently = universal',
      'predicate = abstractSingular (isolatedProperty)',
      'predicate.contains = oneMomentOfSubjectTotality',
    ],
    predicates: [{ name: 'IsSubjectConcrete', args: [] }],
    relations: [
      { predicate: 'contains', from: 'subject', to: 'manifoldProperties' },
    ],
  },
  {
    id: 'j-exist-op-pos-5-reciprocity',
    chunkId: 'j-exist-pos-5-reciprocity-twofold-result',
    label: 'Encode reciprocity: twofold result',
    clauses: [
      'result1: subject.determinedAs = universal',
      'result2: predicate.determinedAs = singular',
      'subjectAndPredicate.retain = oppositionOfConceptDeterminations',
    ],
    predicates: [{ name: 'IsReciprocalDetermination', args: [] }],
    relations: [
      { predicate: 'determines', from: 'predicate', to: 'subject' },
      { predicate: 'determines', from: 'subject', to: 'predicate' },
    ],
  },
  {
    id: 'j-exist-op-pos-6-empty-identical',
    chunkId: 'j-exist-pos-6-empty-identical-propositions',
    label: 'Flag empty identical propositions — judgment sublated',
    clauses: [
      'if identityOnly then propositions = {"singular is singular", "universal is universal"}',
      'judgment.sublated = true',
      'connection.dissolved = true',
    ],
    predicates: [{ name: 'IsEmptyIdentity', args: [] }],
    relations: [
      { predicate: 'signals', from: 'emptyIdentity', to: 'judgmentSublation' },
    ],
  },
  {
    id: 'j-exist-op-pos-7-form-vs-content',
    chunkId: 'j-exist-pos-7-form-vs-content',
    label: 'Encode form vs content — two propositions in one',
    clauses: [
      'formProposition = "singular is universal"',
      'contentProposition = "universal is singular"',
      'positiveJudgment.unites = formAndContent',
      'difference.implicitlyPresent = true',
    ],
    predicates: [{ name: 'FlagsFormContentUnity', args: [] }],
    relations: [
      { predicate: 'containsImplicitly', from: 'positiveJudgment', to: 'formAndContent' },
    ],
  },
  {
    id: 'j-exist-op-pos-8-union-particularity',
    chunkId: 'j-exist-pos-8-union-particularity-empty',
    label: 'Flag union into particularity — empty identical',
    clauses: [
      'if formAndContentUnited then bothSides = particular',
      'resultingProposition = "particular is particular" (empty)',
      'emptyIdentical = true',
      'singularityAndUniversality.cannotUnite = intoParticularity (yet)',
    ],
    predicates: [{ name: 'IsEmptyParticularity', args: [] }],
    relations: [
      { predicate: 'signals', from: 'unionAttempt', to: 'judgmentFailure' },
    ],
  },
  {
    id: 'j-exist-op-pos-9-inadequacy-form',
    chunkId: 'j-exist-pos-9-inadequacy-of-form',
    label: 'Flag inadequacy: singular not universal',
    clauses: [
      'immediateSingular.not = universal',
      'predicate.widerExtension = thanSubject',
      'subject.oppositeOf = abstractUniversality',
    ],
    predicates: [{ name: 'FormInadequate', args: [] }],
    relations: [
      { predicate: 'contradicts', from: 'immediateSingular', to: 'universality' },
    ],
  },
  {
    id: 'j-exist-op-pos-10-inadequacy-content',
    chunkId: 'j-exist-pos-10-inadequacy-of-content',
    label: 'Flag inadequacy: universal not singular',
    clauses: [
      'subject = badInfinitePlurality',
      'subject.not = singleProperty',
      'positiveJudgment.mustBePositedAs = negative',
    ],
    predicates: [{ name: 'ContentInadequate', args: [] }],
    relations: [
      { predicate: 'transitionsTo', from: 'positiveJudgment', to: 'negativeJudgment' },
    ],
  },
  // ============================================================================
  // NEGATIVE JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-exist-op-neg-1-positive-not-true',
    chunkId: 'j-exist-neg-1-positive-not-true',
    label: 'Declare positive judgment not true — truth in negative',
    clauses: [
      'positiveJudgment.notTrue = true',
      'positiveJudgment.contradictsItself = true',
      'truthFoundIn = negativeJudgment',
    ],
    predicates: [{ name: 'PositiveRequiresNegative', args: [] }],
    relations: [
      { predicate: 'requires', from: 'positiveJudgment', to: 'negativeMediation' },
    ],
  },
  {
    id: 'j-exist-op-neg-2-singular-is-particular',
    chunkId: 'j-exist-neg-2-singular-is-particular',
    label: 'Encode singular is particular — mediated determination',
    clauses: [
      'singular.not = abstractlyUniversal',
      'singular.is = particular',
      'universal.is = particular',
      'bothPropositions.reduceTo = "singular is particular"',
    ],
    predicates: [{ name: 'MediatesToParticular', args: [] }],
    relations: [
      { predicate: 'yields', from: 'negativeJudgment', to: 'particularity' },
    ],
  },
  {
    id: 'j-exist-op-neg-3-particularity-mediation',
    chunkId: 'j-exist-neg-3-particularity-mediation',
    label: 'Encode particularity as mediated — not external reflection',
    clauses: [
      'particularity.not = externalReflection',
      'particularity.arises = throughNegativeConnection',
      'determination.resultsIn = predicate',
    ],
    predicates: [{ name: 'ParticularityMediated', args: [] }],
    relations: [
      { predicate: 'arisesThrough', from: 'particularity', to: 'negativeConnection' },
    ],
  },
  {
    id: 'j-exist-op-neg-4-positive-expression',
    chunkId: 'j-exist-neg-4-positive-expression',
    label: 'Encode positive expression of negative judgment',
    clauses: [
      'positiveExpression = "singular is particular"',
      'particular = firstMediatedDetermination',
      'judgment.also = negative',
    ],
    predicates: [{ name: 'ArticulatesNegativeAsPositiveForm', args: [] }],
    relations: [
      { predicate: 'expresses', from: 'negativeJudgment', to: 'mediatedDetermination' },
    ],
  },
  {
    id: 'j-exist-op-neg-5-connection-separation',
    chunkId: 'j-exist-neg-5-connection-separation',
    label: 'Encode connection as separation — positive as negative',
    clauses: [
      'positiveJudgment.connection = alsoSeparation',
      'connection.essentially = negative',
      'positiveJudgment.toBePositedAs = negative',
    ],
    predicates: [{ name: 'ConnectionAsSeparation', args: [] }],
    relations: [
      { predicate: 'contains', from: 'connection', to: 'negativity' },
    ],
  },
  {
    id: 'j-exist-op-neg-6-not-attached-predicate',
    chunkId: 'j-exist-neg-6-not-attached-to-predicate',
    label: 'Encode not attached to predicate — not-universal as particular',
    clauses: [
      'not.attachTo = predicate',
      'predicate.determinedAs = notUniversal',
      'notUniversal = particular',
      'ifAbstractFocus then predicate = indeterminateNotUniversal',
    ],
    predicates: [{ name: 'NegationAsDeterminate', args: [] }],
    relations: [
      { predicate: 'transforms', from: 'negation', to: 'particular' },
    ],
  },
  {
    id: 'j-exist-op-neg-7-conceptualized-negation',
    chunkId: 'j-exist-neg-7-conceptualized-negation',
    label: 'Encode conceptualized negation vs unconceptualized not',
    clauses: [
      'unconceptualizedNot = indeterminateNonBeing',
      'inConcept: not = immediatelyPositive',
      'nonUniversal = directlyParticular',
    ],
    predicates: [{ name: 'ConceptualizesNegation', args: [] }],
    relations: [
      { predicate: 'transforms', from: 'conceptualNegation', to: 'particular' },
    ],
  },
  {
    id: 'j-exist-op-neg-8-negation-preserves-subject',
    chunkId: 'j-exist-neg-8-negation-preserves-subject',
    label: 'Encode negation preserves subject — denies determinateness',
    clauses: [
      'subject.untouchedBy = negation',
      'subject.retains = referenceToUniversality',
      'negated = determinatenessOfPredicate',
      'universalSphere.retained = true',
      'judgment.stillPositive = fromUniversalSphere',
    ],
    predicates: [{ name: 'NegationPreservesSubject', args: [] }],
    relations: [
      { predicate: 'maintains', from: 'negation', to: 'subjectReference' },
    ],
  },
  {
    id: 'j-exist-op-neg-9-particularity-retained',
    chunkId: 'j-exist-neg-9-particularity-retained',
    label: 'Encode particularity retained — transformed determinateness',
    clauses: [
      'particular.contains = universality',
      'determinateness.retained = true',
      'determinateness.transformed = intoParticularity',
    ],
    predicates: [{ name: 'UniversalRetainedParticularArises', args: [] }],
    relations: [
      { predicate: 'yields', from: 'negativeJudgment', to: 'particularity' },
    ],
  },
  {
    id: 'j-exist-op-neg-10-particularity-mediates',
    chunkId: 'j-exist-neg-10-particularity-mediates',
    label: 'Encode particularity mediates singularity and universality',
    clauses: [
      'particularity = mediator(singularity, universality)',
      'negativeJudgment.provides = mediationForReflection',
      'judgment.meaning = alterationOfAccidents',
    ],
    predicates: [{ name: 'ParticularityMediates', args: [] }],
    relations: [
      { predicate: 'enables', from: 'particularity', to: 'selfReflection' },
    ],
  },
  {
    id: 'j-exist-op-neg-11-singular-not-particular',
    chunkId: 'j-exist-neg-11-singular-not-particular',
    label: 'Encode singular not particular — requires further negation',
    clauses: [
      'singular.alsoNot = particular',
      'particularity.widerExtension = thanSingularity',
      'singular.is = determinateDeterminate',
    ],
    predicates: [{ name: 'RequiresFurtherNegation', args: [] }],
    relations: [
      { predicate: 'requires', from: 'singular', to: 'furtherNegation' },
    ],
  },
  {
    id: 'j-exist-op-neg-12-negation-of-negation',
    chunkId: 'j-exist-neg-12-negation-of-negation',
    label: 'Encode negation of negation — restoration of concrete',
    clauses: [
      'negativeJudgment = secondNegation',
      'negationOfDeterminateness = restorationOfConcrete',
      'subject.positedAs = singular (firstTime)',
      'judgmentSays = {"singular is singular", "universal is universal"}',
    ],
    predicates: [{ name: 'NegationOfNegationRestores', args: [] }],
    relations: [
      { predicate: 'yields', from: 'sublation', to: 'concreteRestoration' },
    ],
  },
  {
    id: 'j-exist-op-neg-13-infinite-emergence',
    chunkId: 'j-exist-neg-13-infinite-judgment-emergence',
    label: 'Encode infinite judgment emergence — whole extent negated',
    clauses: [
      'wholeExtentOfPredicate.negated = true',
      'positiveConnection.absent = true',
      'this = infiniteJudgment',
    ],
    predicates: [{ name: 'YieldsInfiniteJudgment', args: [] }],
    relations: [
      { predicate: 'yields', from: 'totalNegation', to: 'infiniteJudgment' },
    ],
  },
  // ============================================================================
  // INFINITE JUDGMENT OPERATIONS
  // ============================================================================
  {
    id: 'j-exist-op-inf-1-negative-infinite',
    chunkId: 'j-exist-inf-1-negative-infinite-nonsensical',
    label: 'Declare negative infinite — nonsensical judgment',
    clauses: [
      'infiniteJudgment = negativeInfinite',
      'formOfJudgment.sublated = true',
      'judgment.nonsensical = true',
      'examples = {"spirit is not red", "rose is not elephant"}',
      'judgment.notJudgment = true',
    ],
    predicates: [{ name: 'IsNegativeInfinite', args: [] }],
    relations: [
      { predicate: 'flags', from: 'infiniteJudgment', to: 'paradoxStatus' },
    ],
  },
  {
    id: 'j-exist-op-inf-2-crime-example',
    chunkId: 'j-exist-inf-2-crime-as-infinite-judgment',
    label: 'Encode crime as infinite judgment — negating universal sphere',
    clauses: [
      'crime = infiniteJudgment',
      'crime.negates = universalSphere (right as right)',
      'crime.effective = true',
      'crime.nonsensical = true',
    ],
    predicates: [{ name: 'CrimeAsInfiniteJudgment', args: [] }],
    relations: [
      { predicate: 'negates', from: 'crime', to: 'universalSphere' },
    ],
  },
  {
    id: 'j-exist-op-inf-3-positive-element',
    chunkId: 'j-exist-inf-3-positive-element-negation-of-negation',
    label: 'Encode positive element — negation of negation',
    clauses: [
      'positiveElement = negationOfNegation',
      'singularity.reflectedInto = itself',
      'singularity.positedAs = determinateDeterminate',
      'judgmentSays = {"singular is singular", "universal is universal"}',
    ],
    predicates: [{ name: 'IsNegationOfNegation', args: [] }],
    relations: [
      { predicate: 'expresses', from: 'positiveElement', to: 'singularIsSingular' },
    ],
  },
  {
    id: 'j-exist-op-inf-4-judgment-sublates',
    chunkId: 'j-exist-inf-4-judgment-sublates-itself',
    label: 'Encode judgment sublates itself — negative and positive infinite',
    clauses: [
      'judgment.sublated = true',
      'negativeInfinite: difference.tooGreat = forJudgment',
      'positiveInfinite: identity.total = noJudgment',
    ],
    predicates: [{ name: 'DetectsSublationStatus', args: [] }],
    relations: [
      { predicate: 'marks', from: 'detector', to: 'sublationState' },
    ],
  },
  {
    id: 'j-exist-op-inf-5-transition-reflection',
    chunkId: 'j-exist-inf-5-transition-to-reflection',
    label: 'Encode transition to judgment of reflection',
    clauses: [
      'judgmentOfExistence.sublated = true',
      'copula.contains = unity (qualitativeExtremesSublated)',
      'unity = concept',
      'concept.tornApart = judgment',
      'terms.reflectedInto = themselves',
      'judgmentOfExistence.passedOver = intoJudgmentOfReflection',
    ],
    predicates: [{ name: 'TransitionsToReflection', args: [] }],
    relations: [
      { predicate: 'transitions', from: 'judgmentOfExistence', to: 'judgmentOfReflection' },
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
