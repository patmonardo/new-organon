import type { TopicMapEntry } from "@schema/topic";

/**
 * TopicMap for constitution.txt - B. FINITUDE (b) Determination, constitution, and limit
 *
 * This TopicMap structures the logical development of determination and constitution
 * as moments that pass over into each other, leading to the emergence of limit
 * as the immanent determination that makes something finite.
 *
 * Structure:
 * - Introduction: In-itself mediated through being-for-other; determinateness existent-in-itself
 * - 1. Determination: Affirmative determinateness; example (human being); still only in itself as ought
 * - 2. Constitution: External existence; alteration; determination and constitution pass over into each other; alteration posited
 * - Transition: Sublation → two somethings; negation immanent; limit emerges
 * - 3. Limit: (a) Non-being of other and of something; mediation
 *   - (b) Existence outside/inside limit; limit as middle point
 *   - (c) Something outside limit = existence in general; limit as principle; unrest/contradiction
 * - Conclusion: Something with immanent limit = finite
 */
export const constitutionTopicMap: TopicMapEntry[] = [
  {
    id: "constitution-1",
    title: "Introduction: in-itself mediated through being-for-other; determinateness existent-in-itself",
    lineRange: { start: 4, end: 25 },
    description: "The in-itself, in which the something is reflected into itself from its being-for-other, no longer is an abstract in-itself but, as the negation of its being-for-other, is mediated through this latter, which is thus its moment. It is not only the immediate identity of the something with itself, but the identity by virtue of which the something also has present in it what it is in itself; the being-for-other is present in it because the in-itself is the sublation of it, is in itself from it; but, because it is still abstract, and therefore essentially affected with negation, it is equally affected with being-for-other. We have here not only quality and reality, existent determinateness, but determinateness existent-in-itself; and the development consists in positing such determinateness as thus immanently reflected.",
    keyPoints: [
      "In-itself no longer abstract but mediated through being-for-other",
      "Being-for-other is its moment",
      "Identity by virtue of which something has present in it what it is in itself",
      "Being-for-other present because in-itself is sublation of it",
      "Still abstract, affected with negation, equally affected with being-for-other",
      "Determinateness existent-in-itself",
      "Development: positing such determinateness as immanently reflected"
    ],
    status: "pending",
    section: "b. Determination, constitution, and limit",
    order: 1,
    relatedChunks: ["constitution-2"]
  },
  {
    id: "constitution-2",
    title: "Determination I: definition (affirmative determinateness; in-itself by which something abides)",
    lineRange: { start: 27, end: 52 },
    description: "The quality which in the simple something is an in-itself essentially in unity with the something's other moment, its being-in-it, can be named its determination, provided that this word is distinguished, in a more precise signification, from determinateness in general. Determination is affirmative determinateness; it is the in-itself by which a something abides in its existence while involved with an other that would determine it, by which it preserves itself in its self-equality, holding on to it in its being-for-other. Something fulfills its determination to the extent that the further determinateness, which variously accrues to it in the measure of its being-in-itself as it relates to an other, becomes its filling. Determination implies that what something is in itself is also present in it.",
    keyPoints: [
      "Quality in simple something is in-itself in unity with being-in-it",
      "Can be named determination (distinguished from determinateness in general)",
      "Determination = affirmative determinateness",
      "In-itself by which something abides in existence while involved with other",
      "Preserves itself in self-equality, holding on to it in being-for-other",
      "Something fulfills determination: further determinateness becomes filling",
      "Determination implies: what something is in itself also present in it"
    ],
    status: "pending",
    section: "1. Determination",
    order: 2,
    relatedChunks: ["constitution-1", "constitution-3"]
  },
  {
    id: "constitution-3",
    title: "Determination II: example (human being; rational thought; still only in itself as ought)",
    lineRange: { start: 54, end: 78 },
    description: "The determination of the human being, its vocation, is rational thought: thinking in general is his simple determinateness; by it the human being is distinguished from the brute; he is thinking in himself, in so far as this thinking is distinguished also from his being-for-other, from his own natural and sensuous being that brings him in immediate association with the other. But thinking is also in him; the human being is himself thinking, he exists as thinking, thought is his concrete existence and actuality; and, further, since thinking is in his existence and his existence is in his thinking, thinking is concrete, must be taken as having content and filling; it is rational thought and as such the determination of the human being. But even this determination is again only in itself, as an ought, that is to say, it is, together with the filling embodied in its in-itself, in the form of an in-itself in general as against the existence which is not embodied in it but still lies outside confronting it, immediate sensibility and nature.",
    keyPoints: [
      "Human being's determination: rational thought (vocation)",
      "Thinking: simple determinateness; distinguishes from brute",
      "Thinking in himself (distinguished from being-for-other, natural/sensuous being)",
      "But thinking also in him; he exists as thinking; thought is concrete existence",
      "Thinking in existence and existence in thinking: thinking concrete, has content/filling",
      "Rational thought = determination of human being",
      "But determination still only in itself, as ought",
      "In form of in-itself as against existence not embodied (immediate sensibility, nature)"
    ],
    status: "pending",
    section: "1. Determination",
    order: 3,
    relatedChunks: ["constitution-2", "constitution-4"]
  },
  {
    id: "constitution-4",
    title: "Constitution I: external existence (not belonging to being-in-itself)",
    lineRange: { start: 80, end: 91 },
    description: "The filling of the being-in-itself with determinateness is also distinct from the determinateness which is only being-for-other and remains outside the determination. For in the sphere of the qualitative, the distinguished terms are left, in their sublated being, also with an immediate, qualitative being contrasting them. That which the something has in it thus separates itself and is from this side the external existence of the something and also its existence, but not as belonging to its being-in-itself. Determinateness is thus constitution.",
    keyPoints: [
      "Filling of being-in-itself with determinateness distinct from determinateness only being-for-other",
      "In qualitative sphere, distinguished terms left with immediate qualitative being contrasting",
      "What something has in it separates itself",
      "External existence of something, also its existence, but not belonging to being-in-itself",
      "Determinateness = constitution"
    ],
    status: "pending",
    section: "2. Constitution",
    order: 4,
    relatedChunks: ["constitution-3", "constitution-5"]
  },
  {
    id: "constitution-5",
    title: "Constitution II: external influences; alteration falls on constitution",
    lineRange: { start: 93, end: 108 },
    description: "Constituted in this or that way, the something is caught up in external influences and in external relationships. This external connection on which the constitution depends, and the being determined through an other, appear as something accidental. But it is the quality of the something to be given over to this externality and to have a constitution. In so far as something alters, the alteration falls on the side of its constitution; the latter is that in the something which becomes an other. The something itself preserves itself in the alteration; the latter affects only this unstable surface of the something's otherness, not its determination.",
    keyPoints: [
      "Constituted: caught up in external influences, relationships",
      "External connection appears accidental",
      "But quality of something to be given over to externality, have constitution",
      "In alteration, alteration falls on constitution",
      "Constitution = that in something which becomes other",
      "Something preserves itself; alteration affects only unstable surface of otherness, not determination"
    ],
    status: "pending",
    section: "2. Constitution",
    order: 5,
    relatedChunks: ["constitution-4", "constitution-6"]
  },
  {
    id: "constitution-6",
    title: "Determination and constitution: distinct but connected; pass over into each other",
    lineRange: { start: 110, end: 137 },
    description: "Determination and constitution are thus distinct from each other; something, according to its determination, is indifferent to its constitution. But that which the something has in it is the middle term of this syllogism connecting the two, determination and constitution. Or, rather, the being-in-the-something showed itself to fall apart into these two extremes. The simple middle term is determinateness as such; its identity belongs to determination just as well as to constitution. But determination passes over into constitution on its own, and constitution into determination. This is implied in what has been said. The connection, upon closer consideration, is this: in so far as that which something is in itself is also in it, the something is affected with being-for-other; determination is therefore open, as such, to the relation with other. Determinateness is at the same time moment, but it contains at the same time the qualitative distinction of being different from being-in-itself, of being the negative of the something, another existence. This determinateness which thus holds the other in itself, united with the being-in-itself, introduces otherness in the latter or in determination, and determination is thereby reduced to constitution.",
    keyPoints: [
      "Determination and constitution distinct; something indifferent to constitution according to determination",
      "What something has in it = middle term connecting determination and constitution",
      "Being-in-something falls apart into two extremes",
      "Simple middle term = determinateness as such (identity belongs to both)",
      "Determination passes over into constitution; constitution into determination",
      "Connection: what something is in itself also in it → affected with being-for-other",
      "Determination open to relation with other",
      "Determinateness holds other in itself, united with being-in-itself",
      "Introduces otherness in determination → reduced to constitution"
    ],
    status: "pending",
    section: "2. Constitution",
    order: 6,
    relatedChunks: ["constitution-5", "constitution-7"]
  },
  {
    id: "constitution-7",
    title: "Constitution passes over into determination; constitution depends on determination",
    lineRange: { start: 138, end: 151 },
    description: "Conversely, the being-for-other, isolated as constitution and posited on its own, is in it the same as what the other as such is, the other in it, that is, the other of itself; but it consequently is self-referring existence, thus being-in-itself with a determinateness, therefore determination. Consequently, inasmuch as the two are also to be held apart, constitution, which appears to be grounded in something external, in an other in general, also depends on determination, and the determining from outside is at the same time determined by the something's own immanent determination. And further, constitution belongs to that which something is in itself: something alters along with its constitution.",
    keyPoints: [
      "Being-for-other isolated as constitution: same as other in it, other of itself",
      "Consequently self-referring existence = being-in-itself with determinateness = determination",
      "Constitution appears grounded in external, other, but also depends on determination",
      "Determining from outside determined by something's immanent determination",
      "Constitution belongs to what something is in itself",
      "Something alters along with constitution"
    ],
    status: "pending",
    section: "2. Constitution",
    order: 7,
    relatedChunks: ["constitution-6", "constitution-8"]
  },
  {
    id: "constitution-8",
    title: "Alteration posited in something; negation immanent",
    lineRange: { start: 153, end: 161 },
    description: "This altering of something is no longer the first alteration of something merely in accordance with its being-for-other. The first was an alteration only implicitly present, one that belonged to the inner concept; now the alteration is also posited in the something. The something itself is further determined, and negation is posited as immanent to it, as its developed being-in-itself.",
    keyPoints: [
      "Alteration no longer first alteration (merely in accordance with being-for-other)",
      "First was only implicitly present (inner concept)",
      "Now alteration also posited in something",
      "Something further determined",
      "Negation posited as immanent to it, as developed being-in-itself"
    ],
    status: "pending",
    section: "2. Constitution",
    order: 8,
    relatedChunks: ["constitution-7", "constitution-9"]
  },
  {
    id: "constitution-9",
    title: "Transition I: sublation → two somethings; negation immanent",
    lineRange: { start: 163, end: 179 },
    description: "The transition of determination and constitution into each other is at first the sublation of their distinction, and existence or something in general is thereby posited; moreover, since this something in general results from a distinction that also includes qualitative otherness within it, there are two somethings. But these are, with respect to each other, not just others in general, so that this negation would still be abstract and would occur only in the comparison of the two; rather the negation now is immanent to the somethings. As existing, they are indifferent to each other, but this, their affirmation, is no longer immediate: each refers itself to itself through the intermediary of the sublation of the otherness which in determination is reflected into the in-itselfness.",
    keyPoints: [
      "Transition of determination and constitution: sublation of distinction",
      "Existence or something in general posited",
      "Since distinction includes qualitative otherness: two somethings",
      "Not just others in general (negation abstract, only in comparison)",
      "Negation now immanent to somethings",
      "As existing, indifferent to each other",
      "But affirmation no longer immediate: each refers to itself through sublation of otherness"
    ],
    status: "pending",
    section: "Transition",
    order: 9,
    relatedChunks: ["constitution-8", "constitution-10"]
  },
  {
    id: "constitution-10",
    title: "Transition II: something behaves to other through itself; negation of other = quality; limit",
    lineRange: { start: 181, end: 227 },
    description: "Something behaves in this way in relation to the other through itself; since otherness is posited in it as its own moment, its in-itselfness holds negation in itself, and it now has its affirmative existence through its intermediary alone. But the other is also qualitatively distinguished from this affirmative existence and is thus posited outside the something. The negation of its other is only the quality of the something, for it is in this sublation of its other that it is something. The other, for its part, truly confronts an existence only with this sublation; it confronts the first something only externally, or, since the two are in fact inherently joined together, that is, according to their concept, their connectedness consists in this, that existence has passed over into otherness, something into other; that something is just as much an other as the other is. Now in so far as the in-itselfness is the non-being of the otherness that is contained in it but is at the same time also distinct as existent, something is itself negation, the ceasing to be of an other in it; it is posited as behaving negatively in relation to the other and in so doing preserving itself. This other, the in-itselfness of the something as negation of the negation, is the something's being-in-itself, and this sublation is as simple negation at the same time in it, namely, as its negation of the other something external to it. It is one determinateness of the two somethings that, on the one hand, as negation of the negation, is identical with the in-itselfness of the somethings, and also, on the other hand, since these negations are to each other as other somethings, joins them together of their own accord and, since each negation negates the other, equally separates them. This determinateness is limit.",
    keyPoints: [
      "Something behaves to other through itself",
      "Otherness posited in it as own moment",
      "In-itselfness holds negation in itself",
      "Affirmative existence through intermediary alone",
      "Other qualitatively distinguished, posited outside",
      "Negation of other = quality of something (in sublation of other that it is something)",
      "Other confronts existence only with this sublation",
      "Connectedness: existence passed over into otherness, something into other",
      "Something just as much other as other is",
      "In-itselfness = non-being of otherness contained but distinct as existent",
      "Something = negation, ceasing to be of other in it",
      "Behaves negatively to other, preserving itself",
      "This other (in-itselfness as negation of negation) = something's being-in-itself",
      "Sublation as simple negation = negation of other something external",
      "One determinateness: as negation of negation identical with in-itselfness; as negations to each other, joins and separates",
      "This determinateness = limit"
    ],
    status: "pending",
    section: "Transition",
    order: 10,
    relatedChunks: ["constitution-9", "constitution-11"]
  },
  {
    id: "constitution-11",
    title: "Limit I: non-being of other and of something; mediation",
    lineRange: { start: 245, end: 260 },
    description: "Something is therefore immediate, self-referring existence and at first it has a limit with respect to an other; limit is the non-being of the other, not of the something itself; in limit, something marks the boundary of its other. But other is itself a something in general. The limit that something has with respect to an other is, therefore, also the limit of the other as a something; it is the limit of this something in virtue of which the something holds the first something as its other away from itself, or is a non-being of that something. The limit is thus not only the non-being of the other, but of the one something just as of the other, and consequently of the something in general.",
    keyPoints: [
      "Something immediate, self-referring existence",
      "At first has limit with respect to other",
      "Limit = non-being of other, not of something itself",
      "In limit, something marks boundary of its other",
      "But other is itself something",
      "Limit of something = also limit of other",
      "Limit of other in virtue of which it holds first something as other away",
      "Limit = non-being of other and of one something and of other = non-being of something in general"
    ],
    status: "pending",
    section: "3. Limit (a)",
    order: 11,
    relatedChunks: ["constitution-10", "constitution-12"]
  },
  {
    id: "constitution-12",
    title: "Limit II: through limit something is; limit = mediation",
    lineRange: { start: 262, end: 285 },
    description: "But the limit is equally, essentially, the non-being of the other; thus, through its limit, something at the same time is. In limiting, something is of course thereby reduced to being limited itself; but, as the ceasing of the other in it, its limit is at the same time itself only the being of the something; this something is what it is by virtue of it, has its quality in it. This relation is the external appearance of the fact that limit is simple negation or the first negation, whereas the other is, at the same time, the negation of the negation, the in-itselfness of the something. Something, as an immediate existence, is therefore the limit with respect to another something; but it has this limit in it and is something through the mediation of that limit, which is just as much its non-being. The limit is the mediation in virtue of which something and other each both is and is not.",
    keyPoints: [
      "Limit essentially non-being of other",
      "Through limit, something at same time is",
      "In limiting, something reduced to being limited",
      "But as ceasing of other in it, limit = being of something",
      "Something is what it is by virtue of limit, has quality in it",
      "External appearance: limit = simple negation (first negation)",
      "Other = negation of negation, in-itselfness",
      "Something immediate existence = limit with respect to another something",
      "Has limit in it, is something through mediation of limit (which is also its non-being)",
      "Limit = mediation in virtue of which something and other each both is and is not"
    ],
    status: "pending",
    section: "3. Limit (a)",
    order: 12,
    relatedChunks: ["constitution-11", "constitution-13"]
  },
  {
    id: "constitution-13",
    title: "Limit III: existence outside/inside limit; limit as middle point",
    lineRange: { start: 287, end: 312 },
    description: "Now in so far as something in its limit both is and is not, and these moments are an immediate, qualitative distinction, the non-existence and the existence of the something fall outside each other. Something has its existence outside its limit (or, as representation would also have it, inside it); in the same way the other, too, since it is something, has it outside it. The limit is the middle point between the two at which they leave off. They have existence beyond each other, beyond their limit; the limit, as the non-being of each, is the other of both. It is in accordance with this difference of the something from its limit that the line appears as line outside its limit, the point; the plane as plane outside the line; the solid as solid only outside its limiting plane. This is the aspect of limit that first occurs to figurative representation (the self-external-being of the concept) and is also most commonly assumed in the context of spatial objects.",
    keyPoints: [
      "Something in limit both is and is not",
      "Moments immediate, qualitative distinction",
      "Non-existence and existence fall outside each other",
      "Something has existence outside limit (or inside, as representation)",
      "Other also has existence outside limit",
      "Limit = middle point at which they leave off",
      "They have existence beyond each other, beyond limit",
      "Limit as non-being of each = other of both",
      "Line outside point, plane outside line, solid outside plane (figurative representation)"
    ],
    status: "pending",
    section: "3. Limit (b)",
    order: 13,
    relatedChunks: ["constitution-12", "constitution-14"]
  },
  {
    id: "constitution-14",
    title: "Limit IV: something outside limit = existence in general; limit as principle",
    lineRange: { start: 314, end: 359 },
    description: "But further, something as it is outside the limit, as the unlimited something, is only existence in general. As such, it is not distinguished from its other; it is only existence and, therefore, it and its other have the same determination; each is only something in general or each is other; and so both are the same. But this, their at first immediate existence, is now posited in them as limit: in it both are what they are, distinct from each other. But it is also equally their common distinguishedness, the unity and the distinguishedness of both, just like existence. This double identity of the two, existence and limit, contains this: that something has existence only in limit, and that, since limit and immediate existence are each at the same time the negative of each other, the something, which is now only in its limit, equally separates itself from itself, points beyond itself to its non-being and declares it to be its being, and so it passes over into it. To apply this to the preceding example, the one determination is this: that something is what it is only in its limit. Therefore, the point is the limit of line, not because the latter just ceases at the point and has existence outside it; the line is the limit of plane, not because the plane just ceases at it; and the same goes for the plane as the limit of solid. Rather, at the point the line also begins; the point is its absolute beginning, and if the line is represented as unlimited on both its two sides, or, as is said, as extended to infinity, the point still constitutes its element, just as the line constitutes the element of the plane, and the plane that of the solid. These limits are the principle of that which they delimit; just as one, for instance, is as hundredth the limit, but also the element, of the whole hundred.",
    keyPoints: [
      "Something outside limit = unlimited something = only existence in general",
      "Not distinguished from other; same determination",
      "Both same",
      "Immediate existence now posited in them as limit",
      "In limit both distinct from each other",
      "Limit = common distinguishedness, unity and distinguishedness of both",
      "Double identity: something has existence only in limit",
      "Limit and immediate existence negative of each other",
      "Something only in limit separates from itself, points beyond to non-being, declares it its being, passes over",
      "Something is what it is only in limit",
      "Point = limit of line (not because line ceases, but point is beginning)",
      "Point = absolute beginning; even if line unlimited, point still its element",
      "Line = element of plane, plane = element of solid",
      "Limits = principle of that which they delimit",
      "One as hundredth = limit and element of whole hundred"
    ],
    status: "pending",
    section: "3. Limit (c)",
    order: 14,
    relatedChunks: ["constitution-13", "constitution-15"]
  },
  {
    id: "constitution-15",
    title: "Limit V: unrest/contradiction; dialectic of point/line/plane",
    lineRange: { start: 361, end: 405 },
    description: "The other determination is the unrest of the something in its limit in which it is immanent, the contradiction that propels it beyond itself. Thus the point is this dialectic of itself becoming line; the line, the dialectic of becoming plane; the plane, of becoming total space. A second definition is given of line, plane, and whole space which has the line come to be through the movement of the point; the plane through the movement of the line, and so forth. This movement of the point, the line, and so forth, is however viewed as something accidental, or as movement only in figurative representation. In fact, however, this view is taken back by supposing that the determinations from which the line, and so forth, originate are their elements and principles, and these are, at the same time, nothing else but their limits; the coming to be is not considered as accidental or only as represented. That the point, the line, the plane, are per se self-contradictory beginnings which on their own repel themselves from themselves, and consequently that the point passes over from itself into the line through its concept, moves in itself and makes the line come to be, and so on all this lies in the concept of the limit which is immanent in the something. The application itself, however, belongs to the treatment of space; as an indication of it here, we can say that the point is the totally abstract limit, but in a determinate existence; this existence is still taken in total abstraction, it is the so-called absolute, that is, abstract space, the absolutely continuous being-outside-one-another. Inasmuch as the limit is not abstract negation, but is rather in this existence, inasmuch as it is spatial determinateness, the point is spatial, is the contradiction of abstract negation and continuity and is, for that reason, the transition as it occurs and has already occurred into the line, and so forth. And so there is no point, just as there is no line or plane.",
    keyPoints: [
      "Other determination: unrest of something in limit (immanent)",
      "Contradiction propels it beyond itself",
      "Point = dialectic of itself becoming line",
      "Line = dialectic becoming plane",
      "Plane = dialectic becoming total space",
      "Second definition: line through movement of point, plane through movement of line",
      "Movement viewed as accidental or only in representation",
      "But view taken back: determinations from which line originates = elements/principles = limits",
      "Coming to be not accidental or only represented",
      "Point, line, plane per se self-contradictory beginnings",
      "Repel themselves from themselves",
      "Point passes over into line through concept, moves in itself",
      "Lies in concept of limit immanent in something",
      "Point = totally abstract limit in determinate existence",
      "Existence = abstract space, absolutely continuous being-outside-one-another",
      "Limit not abstract negation but in existence, spatial determinateness",
      "Point = contradiction of abstract negation and continuity",
      "Transition into line, etc.",
      "There is no point, just as no line or plane"
    ],
    status: "pending",
    section: "3. Limit (c)",
    order: 15,
    relatedChunks: ["constitution-14", "constitution-16"]
  },
  {
    id: "constitution-16",
    title: "Conclusion: something with immanent limit = finite",
    lineRange: { start: 407, end: 410 },
    description: "The something, posited with its immanent limit as the contradiction of itself by virtue of which it is directed and driven out and beyond itself, the finite.",
    keyPoints: [
      "Something posited with immanent limit",
      "As contradiction of itself",
      "By virtue of which directed and driven out and beyond itself",
      "= the finite"
    ],
    status: "pending",
    section: "Conclusion",
    order: 16,
    relatedChunks: ["constitution-15"]
  }
];

