import type { TopicMapEntry } from "@schema/topic";

/**
 * TopicMap for something-and-other.txt - B. FINITUDE (a) Something and other
 *
 * This TopicMap structures the logical development of the relationship between
 * something and other, establishing the structure of being-in-itself and being-for-other
 * that is essential for understanding finitude.
 *
 * Structure:
 * - Outline: (a) Something and other → (b) Determination/constitution/limit → (c) Finitude
 * - Introduction: Negative determination (vs affirmative in first division)
 * - a. Something and other:
 *   - 1. Something and other are: both existents, both others, "this" as subjective, other as 'to heteron', nature as other-of-itself
 *   - 2. Being-for-other and being-in-itself: something preserves itself in non-being, two moments, their relationship
 *   - 3. Identity of being-in-itself and being-for-other: both moments of something, thing-in-itself critique, positedness, transition vs positing, determination as quality
 */
export const somethingAndOtherTopicMap: TopicMapEntry[] = [
  {
    id: "something-and-other-1",
    title: "Outline: (a) Something and other → (b) Determination/constitution/limit → (c) Finitude",
    lineRange: { start: 4, end: 19 },
    description: "(a) Something and other: at first they are indifferent to one another; an other is also an immediate existent, a something; the negation thus falls outside both. Something is in itself in contrast to its being-for-other. But the determinateness belongs also to its in-itself, and (b) the determination of this in-itself in turn passes over into constitution, and this latter, as identical with determination, constitutes the immanent and at the same time negated being-for-another, the limit of something which (c) is the immanent determination of the something itself, and the something thus is the finite.",
    keyPoints: [
      "(a) Something and other: indifferent at first; negation falls outside both; determinateness belongs to in-itself",
      "(b) Determination passes over into constitution; limit of something",
      "(c) Limit is immanent determination; something is finite"
    ],
    status: "pending",
    section: "B. FINITUDE",
    order: 1,
    relatedChunks: ["something-and-other-2"]
  },
  {
    id: "something-and-other-2",
    title: "Introduction: negative determination (vs affirmative in first division)",
    lineRange: { start: 21, end: 30 },
    description: "In the first division where existence in general was considered, this existence had, as at first taken up, the determination of an existent. The moments of its development, quality and something, are therefore of equally affirmative determination. The present division, on the contrary, develops the negative determination which is present in existence and was there from the start only as negation in general. It was then the first negation but has now been determined to the point of the being-in-itself of the something, the point of the negation of negation.",
    keyPoints: [
      "First division: existence had determination of existent (affirmative)",
      "Present division: develops negative determination present in existence",
      "Was first negation; now determined to being-in-itself, negation of negation"
    ],
    status: "pending",
    section: "B. FINITUDE",
    order: 2,
    relatedChunks: ["something-and-other-1", "something-and-other-3"]
  },
  {
    id: "something-and-other-3",
    title: "Something and other I: both existents; both others; indifference",
    lineRange: { start: 34, end: 46 },
    description: "Something and other are, first, both existents or something. Second, each is equally an other. It is indifferent which is named first, and just for this reason it is named something (in Latin, when they occur in a proposition, both are aliud, or \"the one, the other,\" alius alium; in the case of an alternating relation, the analogous expression is alter alterum). If of two beings we call the one A and the other B, the B is the one which is first determined as other. But the A is just as much the other of the B. Both are other in the same way.",
    keyPoints: [
      "First: both existents or something",
      "Second: each equally an other",
      "Indifferent which named first",
      "Both other in same way"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 3,
    relatedChunks: ["something-and-other-4"]
  },
  {
    id: "something-and-other-4",
    title: "\"This\" as subjective designation; language expresses universal",
    lineRange: { start: 47, end: 63 },
    description: "\"This\" serves to fix the distinction and the something which is to be taken in the affirmative sense. But \"this\" also expresses the fact that the distinction, and the privileging of one something, is a subjective designation that falls outside the something itself. The whole determinateness falls on the side of this external pointing; also the expression \"this\" contains no distinctions; each and every something is just as good a \"this\" as any other. By \"this\" we mean to express something completely determinate, overlooking the fact that language, as a work of the understanding, only expresses the universal, albeit naming it as a single object. But an individual name is something meaningless in the sense that it does not express a universal. It appears as something merely posited and arbitrary for the same reason that proper names can also be arbitrarily picked, arbitrarily given as well as arbitrarily altered.",
    keyPoints: [
      "\"This\" fixes distinction, privileges one something",
      "Subjective designation falling outside something itself",
      "Determinateness falls on external pointing",
      "\"This\" contains no distinctions; each something equally \"this\"",
      "Language expresses universal, naming as single object",
      "Individual name meaningless (doesn't express universal)",
      "Proper names arbitrary"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 4,
    relatedChunks: ["something-and-other-3", "something-and-other-5"]
  },
  {
    id: "something-and-other-5",
    title: "Otherness as external; comparison by Third; every existence is other",
    lineRange: { start: 65, end: 79 },
    description: "Otherness thus appears as a determination alien to the existence thus pointed at, or the other existence as outside this one existence, partly because the one existence is determined as other only by being compared by a Third, and partly because it is so determined only on account of the other which is outside it, but is not an other for itself. At the same time, as has been remarked, even for ordinary thinking every existence equally determines itself as an other existence, so that there is no existence that remains determined simply as an existence, none which is not outside an existence and therefore is not itself an other.",
    keyPoints: [
      "Otherness appears as determination alien to existence",
      "Determined as other by comparison by Third",
      "Determined on account of other outside it",
      "Not other for itself",
      "Every existence determines itself as other existence",
      "No existence remains simply as existence",
      "All are outside an existence, therefore themselves others"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 5,
    relatedChunks: ["something-and-other-4", "something-and-other-6"]
  },
  {
    id: "something-and-other-6",
    title: "Sameness falls in external reflection; other for itself apart from something",
    lineRange: { start: 81, end: 89 },
    description: "Both are determined as something as well as other: thus they are the same and there is as yet no distinction present in them. But this sameness of determinations, too, falls only within external reflection, in the comparison of the two; but the other, as posited at first, though an other with reference to something, is other also for itself apart from the something.",
    keyPoints: [
      "Both determined as something and other: same, no distinction",
      "Sameness falls only in external reflection (comparison)",
      "Other, as posited, is other with reference to something",
      "But also other for itself apart from something"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 6,
    relatedChunks: ["something-and-other-5", "something-and-other-7"]
  },
  {
    id: "something-and-other-7",
    title: "Other as 'to heteron' (Plato); other-of-itself; nature as other of spirit",
    lineRange: { start: 91, end: 108 },
    description: "Third, the other is therefore to be taken in isolation, with reference to itself, has to be taken abstractly as the other, the 'to heteron' of Plato who opposes it to the one as a moment of totality, and in this way ascribes to the other a nature of its own. Thus the other, taken solely as such, is not the other of something, but is the other within, that is, the other of itself. Such an other, which is the other by its own determination, is physical nature; nature is the other of spirit; this, its determination, is at first a mere relativity expressing not a quality of nature itself but only a reference external to it. But since spirit is the true something, and hence nature is what it is within only in contrast to spirit, taken for itself the quality of nature is just this, to be the other within, that which-exists-outside-itself (in the determinations of space, time, matter).",
    keyPoints: [
      "Other taken in isolation, abstractly as the other",
      "'To heteron' of Plato, opposed to one as moment of totality",
      "Other ascribed nature of its own",
      "Other, taken solely, is other within, other of itself",
      "Nature is other of spirit (example)",
      "Nature's quality: to be other within, that-which-exists-outside-itself (space, time, matter)"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 7,
    relatedChunks: ["something-and-other-6", "something-and-other-8"]
  },
  {
    id: "something-and-other-8",
    title: "Other-of-itself: absolutely unequal, negates/alters itself, yet identical",
    lineRange: { start: 110, end: 124 },
    description: "The other which is such for itself is the other within it, hence the other of itself and so the other of the other; therefore, the absolutely unequal in itself, that which negates itself, alters itself. But it equally remains identical with itself, for that into which it alters is the other, and this other has no additional determination; but that which alters itself is not determined in any other way than in this, to be an other; in going over to this other, it only unites with itself. It is thus posited as reflected into itself with sublation of the otherness, a self-identical something from which the otherness, which is at the same time a moment of it, is therefore distinct, itself not appertaining to it as something.",
    keyPoints: [
      "Other for itself is other within it, other of itself, other of other",
      "Absolutely unequal in itself",
      "Negates itself, alters itself",
      "Equally remains identical (alters to other with no additional determination)",
      "In going over to other, unites with itself",
      "Posited as reflected into itself with sublation of otherness",
      "Self-identical something; otherness distinct, not appertaining as something"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 8,
    relatedChunks: ["something-and-other-7", "something-and-other-9"]
  },
  {
    id: "something-and-other-9",
    title: "Something preserves itself in non-being; being-for-other and being-in-itself",
    lineRange: { start: 126, end: 145 },
    description: "The something preserves itself in its non-being; it is essentially one with it, and essentially not one with it. It therefore stands in reference to an otherness without being just this otherness. The otherness is at once contained in it and yet separated from it; it is being-for-other. Existence as such is an immediate, bare of references; or, it is in the determination of being. However, as including non-being within itself, existence is determinate being, being negated within itself, and then in the first instance an other; but, since in being negated it preserves itself at the same time, it is only being-for-other. It preserves itself in its non-being and is being; not, however, being in general but being with reference to itself in contrast to its reference to the other, as self-equality in contrast to its inequality. Such a being is being-in-itself.",
    keyPoints: [
      "Something preserves itself in non-being",
      "Essentially one with it, essentially not one with it",
      "Stands in reference to otherness without being it",
      "Otherness at once contained and separated: being-for-other",
      "Existence: immediate, bare of references (being)",
      "Including non-being: determinate being, negated within itself",
      "First instance: other; but preserves itself: only being-for-other",
      "Preserves itself in non-being and is being",
      "Being with reference to itself (vs reference to other): self-equality vs inequality",
      "Such being = being-in-itself"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 9,
    relatedChunks: ["something-and-other-8", "something-and-other-10"]
  },
  {
    id: "something-and-other-10",
    title: "Two pairs of determinations; truth is connection",
    lineRange: { start: 147, end: 179 },
    description: "Being-for-other and being-in-itself constitute the two moments of something. There are here two pairs of determinations: (1) something and other; (2) being-for-other and being-in-itself. The former contain the non-connectedness of their determinateness; something and other fall apart. But their truth is their connection; being-for-other and being-in-itself are therefore the same determinations posited as moments of one and the same unity, as determinations which are connections and which, in their unity, remain in the unity of existence. Each thus itself contains within it, at the same time, also the moment diverse from it. Being and nothing in their unity, which is existence, are no longer being and nothing (these they are only outside their unity); so in their restless unity, in becoming, they are coming-to-be and ceasing-to-be. In the something, being is being-in-itself. Now, as self-reference, self-equality, being is no longer immediately, but is self-reference only as the non-being of otherness (as existence reflected into itself). The same goes for non-being: as the moment of something in this unity of being and non-being: it is not non-existence in general but is the other, and more determinedly, according as being is at the same time distinguished from it, it is reference to its non-existence, being-for-other.",
    keyPoints: [
      "Being-for-other and being-in-itself = two moments of something",
      "Two pairs: (1) something and other; (2) being-for-other and being-in-itself",
      "Former: non-connectedness; something and other fall apart",
      "Truth is connection",
      "Being-for-other and being-in-itself = same determinations as moments of unity",
      "Each contains moment diverse from it",
      "Being-in-itself: self-reference as non-being of otherness (existence reflected)",
      "Being-for-other: reference to non-existence (other)"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 10,
    relatedChunks: ["something-and-other-9", "something-and-other-11"]
  },
  {
    id: "something-and-other-11",
    title: "Being-in-itself: negative reference; has non-being in it",
    lineRange: { start: 181, end: 197 },
    description: "Hence being-in-itself is, first, negative reference to non-existence; it has otherness outside it and is opposed to it; in so far as something is in itself, it is withdrawn from being-other and being-for-other. But, second, it has non-being also right in it; for it is itself the non-being of being-for-other. But being-for-other is, first, the negation of the simple reference of being to itself which, in the first place, is supposed to be existence and something; in so far as something is in an other or for an other, it lacks a being of its own. But, second, it is not non-existence as pure nothing; it is non-existence that points to being-in-itself as its being reflected into itself, just as conversely the being-in-itself points to being-for-other.",
    keyPoints: [
      "Being-in-itself: first, negative reference to non-existence",
      "Has otherness outside, opposed to it",
      "Withdrawn from being-other and being-for-other",
      "Second: has non-being in it (non-being of being-for-other)",
      "Being-for-other: first, negation of simple self-reference",
      "In other or for other: lacks being of own",
      "Second: not pure nothing; points to being-in-itself as reflected",
      "Being-in-itself points to being-for-other"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 11,
    relatedChunks: ["something-and-other-10", "something-and-other-12"]
  },
  {
    id: "something-and-other-12",
    title: "Both moments of something; identity of being-in-itself and being-for-other",
    lineRange: { start: 199, end: 221 },
    description: "Both moments are determinations of one and the same, namely of something. Something is in-itself in so far as it has returned from the being-for-other back to itself. But something has also a determination or circumstance, whether in itself (here the accent is on the in) or in it; in so far as this circumstance is in it externally, it is a being-for-other. This leads to a further determination. Being-in-itself and being-for-other are different at first. But that something also has in it what it is in itself and conversely is in itself also what it is as being-for-other this is the identity of being-in-itself and being-for-other, in accordance with the determination that the something is itself one and the same something of both moments, and these are in it, therefore, undivided. This identity already occurs formally in the sphere of existence, but more explicitly in the treatment of essence and later of the relations of interiority and externality, and in the most determinate form in the treatment of the idea, as the unity of concept and actuality.",
    keyPoints: [
      "Both moments determinations of one and same: something",
      "Something in-itself: returned from being-for-other to itself",
      "Something has determination/circumstance (in itself or in it)",
      "If in it externally: being-for-other",
      "Being-in-itself and being-for-other different at first",
      "Identity: something has in it what it is in itself; in itself what it is as being-for-other",
      "Something is one and same of both moments; undivided",
      "Identity occurs formally in existence; more explicit in essence; most determinate in idea"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 12,
    relatedChunks: ["something-and-other-11", "something-and-other-13"]
  },
  {
    id: "something-and-other-13",
    title: "In-itself as abstract; thing-in-itself critique I",
    lineRange: { start: 222, end: 232 },
    description: "Opinion has it that with the in-itself something lofty is being said, as with the inner; but what something is only in itself, is also only in it; in-itself is a merely abstract, and hence itself external determination. The expressions: there is nothing in it, or there is something in it, imply, though somewhat obscurely, that what is in a thing also pertains to its in-itselfness, to its inner, true worth.",
    keyPoints: [
      "Opinion: in-itself something lofty (like inner)",
      "But what is only in itself is also only in it",
      "In-itself = merely abstract, hence external determination",
      "\"There is nothing/something in it\" implies what is in thing pertains to in-itselfness, inner worth"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 13,
    relatedChunks: ["something-and-other-12", "something-and-other-14"]
  },
  {
    id: "something-and-other-14",
    title: "Thing-in-itself critique II: empty abstraction",
    lineRange: { start: 234, end: 255 },
    description: "It may be observed that here we have the meaning of the thing-in-itself. It is a very simple abstraction, though it was for a while a very important determination, something sophisticated, as it were, just as the proposition that we know nothing of what things are in themselves was a much valued piece of wisdom. Things are called \"in-themselves\" in so far as abstraction is made from all being-for-other, which really means, in so far as they are thought without all determination, as nothing. In this sense, of course, it is impossible to know what the thing-in-itself is. For the question \"what?\" calls for determinations to be produced; but since the things of which the determinations are called for are at the same time presumed to be things-in-themselves, which means precisely without determination, the impossibility of an answer is thoughtlessly implanted in the question, or else a senseless answer is given. The thing-in-itself is the same as that absolute of which nothing is known except that in it all is one. What there is in these things-in-themselves is therefore very well known; they are as such nothing but empty abstractions void of truth.",
    keyPoints: [
      "Here meaning of thing-in-itself",
      "Very simple abstraction (was important, sophisticated)",
      "\"We know nothing of things-in-themselves\" = valued wisdom",
      "Things \"in-themselves\": abstraction from all being-for-other",
      "Really: thought without all determination, as nothing",
      "Impossible to know (question calls for determinations; but things presumed without determination)",
      "Thing-in-itself = absolute where all is one",
      "Things-in-themselves = empty abstractions void of truth"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 14,
    relatedChunks: ["something-and-other-13", "something-and-other-15"]
  },
  {
    id: "something-and-other-15",
    title: "Thing-in-itself in truth: concept (concrete, cognizable)",
    lineRange: { start: 256, end: 264 },
    description: "What, however, the thing-in-itself in truth is, what there basically is in it, of this the Logic is the exposition. But in this Logic something better is understood by the in-itself than an abstraction, namely, what something is in its concept; but this concept is in itself concrete: as concept, in principle conceptually graspable; and, as determined and as the connected whole of its determinations, inherently cognizable.",
    keyPoints: [
      "Thing-in-itself in truth: Logic is exposition",
      "In Logic, in-itself = what something is in its concept",
      "Concept is in itself concrete",
      "Concept: in principle conceptually graspable",
      "As determined and connected whole of determinations: inherently cognizable"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 15,
    relatedChunks: ["something-and-other-14", "something-and-other-16"]
  },
  {
    id: "something-and-other-16",
    title: "Positedness; transition vs positing; in-itself vs posited",
    lineRange: { start: 266, end: 315 },
    description: "Being-in-itself has at first the being-for-other as a moment standing over against it. But positedness also comes to be positioned over against it, and, although in this expression being-for-other is also included, the expression still contains the determination of the bending back, which has already occurred, of that which is not in itself into that wherein it is positive, and this is its being-in-itself. Being-in-itself is normally to be taken as an abstract way of expressing the concept; positing, strictly speaking, first occurs in the sphere of essence, of objective reflection; the ground posits that which is grounded through it; more strongly, the cause produces an effect, an existence whose subsistence is immediately negated and which carries the meaning that it has its substance, its being, in an other. In the sphere of being, existence only emerges out of becoming. Or again, with the something an other is posited; with the finite, an infinite; but the finite does not bring forth the infinite, does not posit it. In the sphere of being, the self-determining of the concept is at first only in itself or implicit, and for that reason it is called a transition or passing over. And the reflecting determinations of being, such as something and other, or finite and infinite, although they essentially point to one another, or are as being-for-other, also stand on their own qualitatively; the other exists; the finite, like the infinite, is equally to be regarded as an immediate existent that stands firm on its own; the meaning of each appears complete even without its other. The positive and the negative, on the contrary, cause and effect, however much they are taken in isolation, have at the same time no meaning each without the other; their reflective shining in each other, the shine in each of its other, is present right in them. In the different cycles of determination and especially in the progress of the exposition, or, more precisely, in the progress of the concept in the exposition of itself, it is of capital concern always to clearly distinguish what still is in itself or implicitly and what is posited, how determinations are in the concept and how they are as posited or as existing-for-other. This is a distinction that belongs only to the dialectical development and one unknown to metaphysical philosophizing (to which the critical also belongs); the definitions of metaphysics, like its presuppositions, distinctions, and conclusions, are meant to assert and produce only the existent and that, too, as existent-in-itself.",
    keyPoints: [
      "Being-in-itself has being-for-other as moment over against it",
      "Positedness also positioned over against it",
      "Positedness: bending back of what is not in itself into being-in-itself",
      "Being-in-itself = abstract expression of concept",
      "Positing: strictly in essence (objective reflection)",
      "Ground posits grounded; cause produces effect",
      "In being: existence emerges from becoming (transition)",
      "Something posits other; finite posits infinite (but doesn't bring forth/posit it)",
      "In being: self-determining only in itself (implicit) = transition/passing over",
      "Reflecting determinations (something/other, finite/infinite): point to one another but stand on own qualitatively",
      "Positive/negative, cause/effect: no meaning without other (reflective shining)",
      "Capital concern: distinguish in-itself (implicit) vs posited (existing-for-other)",
      "Distinction belongs to dialectical development; unknown to metaphysics"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 16,
    relatedChunks: ["something-and-other-15", "something-and-other-17"]
  },
  {
    id: "something-and-other-17",
    title: "Identity in something; determination as quality",
    lineRange: { start: 324, end: 329 },
    description: "In the unity of the something with itself, being-for-other is identical with its in-itself; the being-for-other is thus in the something. The determinateness thus reflected into itself is therefore again a simple existent and hence again a quality: determination.",
    keyPoints: [
      "In unity of something with itself, being-for-other identical with in-itself",
      "Being-for-other thus in the something",
      "Determinateness reflected into itself = simple existent",
      "Hence again quality: determination"
    ],
    status: "pending",
    section: "a. Something and other",
    order: 17,
    relatedChunks: ["something-and-other-16"]
  }
];

