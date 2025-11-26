import type { TopicMapEntry } from "@schema/topic";

/**
 * TopicMap for process.txt - B. THE PROCESS
 *
 * This TopicMap structures the logical development of the chemical process.
 * The process shows how non-indifferent objects relate through affinity.
 * Three syllogisms emerge: formal neutrality, real neutrality, disjunctive syllogism.
 * The process shows transition from externality to self-determination, preparing transition to Teleology (purpose).
 *
 * Structure:
 * - 1. Presupposition: Objects in tension, affinity, middle term (implicit nature, element of communication)
 * - 2. Product: Neutral object, tension dissolved, but negativity outside object
 * - 3. Elemental objects: Liberated from tension, inner determinateness as contradiction, chemism goes back to beginning and sublates itself
 */
export const processTopicMap: TopicMapEntry[] = [
  {
    id: "process-1",
    title: "Presupposition - Objects in tension, affinity",
    lineRange: { start: 4, end: 18 },
    description: "It begins with the presupposition that the objects in tension, as much as they are tensed against themselves, just as much are they by that very fact at first tensed against each other, a relation which is called their affinity. Each stands through its concept in contradiction to its concrete existence's own one-sidedness and each consequently strives to sublate it, and in this there is immediately posited the striving to sublate the one-sidedness of the other and, through this reciprocal balancing and combining, to posit a reality conformable to the concept that contains both moments.",
    keyPoints: [
      "Begins with presupposition that objects in tension",
      "As much tensed against themselves, just as much tensed against each other",
      "Relation called their affinity",
      "Each stands through concept in contradiction to concrete existence's own one-sidedness",
      "Each strives to sublate it",
      "Immediately posited striving to sublate one-sidedness of other",
      "Through reciprocal balancing and combining",
      "Posit reality conformable to concept containing both moments"
    ],
    status: "pending",
    section: "B. THE PROCESS",
    order: 1,
    relatedChunks: ["process-2"]
  },
  {
    id: "process-2",
    title: "Middle term - Implicit nature and element of communication",
    lineRange: { start: 19, end: 45 },
    description: "Since each is posited within it as self-contradictory and self-sublating, they are held apart from each other and from their reciprocal complementation only by external violent force. The middle term whereby these extremes are now concluded into a unity is, first, the implicitly existent nature of both, the whole concept containing both within. But, second, since in concrete existence the two stand over against each other, their absolute unity is also a still formal element that concretely exists distinct from them, the element of communication wherein they enter into external community with each other. Since the real difference belongs to the extremes, this middle term is only the abstract neutrality, the real possibility of those extremes, the theoretical element, as it were, of the concrete existence of the chemical objects, of their process and its result. In the realm of bodies, water fulfills the function of this medium; in that of spirit, inasmuch as there is in it an analog of such a relation, the sign in general, and language more specifically, can be regarded as fulfilling it.",
    keyPoints: [
      "Since each posited within it as self-contradictory and self-sublating",
      "Held apart from each other and from reciprocal complementation only by external violent force",
      "Middle term whereby extremes concluded into unity",
      "First: Implicitly existent nature of both, whole concept containing both within",
      "Second: Since in concrete existence two stand over against each other",
      "Absolute unity also still formal element concretely existing distinct from them",
      "Element of communication wherein they enter into external community",
      "Since real difference belongs to extremes, middle term only abstract neutrality",
      "Real possibility of extremes, theoretical element",
      "In realm of bodies, water fulfills function of medium",
      "In spirit, sign in general, language more specifically, can be regarded as fulfilling it"
    ],
    status: "pending",
    section: "B. THE PROCESS",
    order: 2,
    relatedChunks: ["process-1", "process-3"]
  },
  {
    id: "process-3",
    title: "Communication - Tranquil coming-together and negative relating",
    lineRange: { start: 47, end: 60 },
    description: "The relation of the objects, as mere communication in this element, is on the one hand a tranquil coming-together, but on the other it is equally a negative relating, for in communication the concrete concept which is their nature is posited in reality, and the real differences of the object are thereby reduced to its unity. Their prior self-subsistent determinateness is thus sublated in the union that conforms to the concept, which is one and the same in both; their opposition and tension are thereby blunted, with the result that in this reciprocal complementation the striving attains its tranquil neutrality.",
    keyPoints: [
      "Relation of objects as mere communication in this element",
      "On one hand tranquil coming-together",
      "On other equally negative relating",
      "In communication concrete concept which is their nature posited in reality",
      "Real differences of object reduced to unity",
      "Prior self-subsistent determinateness sublated in union conforming to concept",
      "One and same in both",
      "Opposition and tension blunted",
      "In reciprocal complementation striving attains tranquil neutrality"
    ],
    status: "pending",
    section: "B. THE PROCESS",
    order: 3,
    relatedChunks: ["process-2", "process-4"]
  },
  {
    id: "process-4",
    title: "Process dissolved - Product as neutral",
    lineRange: { start: 62, end: 86 },
    description: "The process is in this way dissolved; since the contradiction between concept and reality has been resolved, the extremes of the syllogism have consequently lost their opposition and have ceased to be extremes as against each other and the middle term. The product is something neutral, that is, something in which the ingredients, which can no longer be called objects, are no longer in tension and therefore no longer have the properties that accrued to them in tension, though in the product the capacity for their prior self-subsistence and tension is retained. For the negative unity of the neutral product proceeds from a presupposed non-indifference; the determinateness of the chemical object is identical with its objectivity; it is original. Through the process just considered, this non-indifference is only immediately sublated; the determinateness, therefore, is not as yet absolutely reflected into itself, and consequently the product of the process is only a formal unity.",
    keyPoints: [
      "Process dissolved",
      "Contradiction between concept and reality resolved",
      "Extremes of syllogism lost opposition, ceased to be extremes",
      "Product is something neutral",
      "Ingredients no longer called objects",
      "No longer in tension, no longer have properties accruing in tension",
      "Capacity for prior self-subsistence and tension retained",
      "Negative unity of neutral product proceeds from presupposed non-indifference",
      "Determinateness of chemical object identical with objectivity, original",
      "Through process, non-indifference only immediately sublated",
      "Determinateness not yet absolutely reflected into itself",
      "Product only formal unity"
    ],
    status: "pending",
    section: "B. THE PROCESS",
    order: 4,
    relatedChunks: ["process-3", "process-5"]
  },
  {
    id: "process-5",
    title: "Negativity outside object",
    lineRange: { start: 88, end: 108 },
    description: "In this product the tension of opposition, and the negative unity which is the activity of the process, are now indeed dissolved. But since this unity is essential to the concept and has also itself come into concrete existence, it is still present but has stepped outside the neutral object. The process does not spontaneously re-start itself, for it had non-indifference only as its presupposition; it did not posit it. This self-subsistent negativity outside the object, the concrete existence of the abstract singularity whose being-for-itself has its reality in the non-indifferent object, is in itself now in tension with its abstraction, an inherently restless activity outwardly bent on consuming. It connects immediately with the object whose tranquil neutrality is the real possibility of an opposition to this neutrality; the same object is now the middle term of the prior formal neutrality, now concrete in itself and determined.",
    keyPoints: [
      "In product tension of opposition and negative unity dissolved",
      "But since unity essential to concept and come into concrete existence",
      "Still present but stepped outside neutral object",
      "Process does not spontaneously re-start itself",
      "Had non-indifference only as presupposition, did not posit it",
      "Self-subsistent negativity outside object",
      "Concrete existence of abstract singularity",
      "Being-for-itself has reality in non-indifferent object",
      "In itself in tension with abstraction",
      "Inherently restless activity outwardly bent on consuming",
      "Connects immediately with object whose tranquil neutrality is real possibility of opposition",
      "Same object now middle term of prior formal neutrality",
      "Now concrete in itself and determined"
    ],
    status: "pending",
    section: "B. THE PROCESS",
    order: 5,
    relatedChunks: ["process-4", "process-6"]
  },
  {
    id: "process-6",
    title: "Disruption and disjunctive syllogism",
    lineRange: { start: 110, end: 154 },
    description: "The more precise immediate connection of the extreme of negative unity with the object is in that the latter is determined by it and is thereby disrupted. This disruption may at first be regarded as the restoration of the opposition of the objects in tension with which chemism began. But this determination does not constitute the other extreme of the syllogism but belongs to the immediate connection of the differentiating principle with the middle in which this principle gives itself its immediate reality; it is the determinateness which the middle term, besides at the same time being the universal nature of the subject matter, possesses in the disjunctive syllogism, whereby that object is both objective universality and determinate particularity. The other extreme of the syllogism stands opposed to the external self-subsistent extreme of singularity; it is, therefore, the equally self-subsisting extreme of universality; hence the disruption that the real neutrality of the middle term undergoes in it is that it breaks up into moments that are not non-indifferent but, on the contrary, neutral. Accordingly these moments are, on the one side, the abstract and indifferent base, and, on the other, this base's activating principle which, separated from it, equally attains the form of indifferent objectivity. This disjunctive syllogism is the totality of chemism in which the same objective whole is exhibited as self-standing negative unity; then, in the middle term, as real unity; and finally as the chemical reality resolved into its abstract moments. In these moments the determinateness has not reached its immanent reflection in an other as in the neutral product, but has in itself returned into its abstraction, an originally determined element.",
    keyPoints: [
      "More precise immediate connection of extreme of negative unity with object",
      "Latter determined by it, thereby disrupted",
      "Disruption may be regarded as restoration of opposition of objects in tension",
      "But determination does not constitute other extreme of syllogism",
      "Belongs to immediate connection of differentiating principle with middle",
      "In which principle gives itself immediate reality",
      "Determinateness which middle term possesses in disjunctive syllogism",
      "Object both objective universality and determinate particularity",
      "Other extreme stands opposed to external self-subsistent extreme of singularity",
      "Equally self-subsisting extreme of universality",
      "Disruption that real neutrality undergoes: breaks up into moments not non-indifferent but neutral",
      "On one side abstract and indifferent base",
      "On other base's activating principle separated from it",
      "Equally attains form of indifferent objectivity",
      "Disjunctive syllogism is totality of chemism",
      "Same objective whole exhibited as self-standing negative unity",
      "Then in middle term as real unity",
      "Finally as chemical reality resolved into abstract moments",
      "In these moments determinateness not reached immanent reflection in other",
      "But has in itself returned into abstraction, originally determined element"
    ],
    status: "pending",
    section: "B. THE PROCESS",
    order: 6,
    relatedChunks: ["process-5", "process-7"]
  },
  {
    id: "process-7",
    title: "Elemental objects - Liberation and return",
    lineRange: { start: 156, end: 180 },
    description: "These elemental objects are therefore liberated from chemical tension; in them, the original basis of that presupposition with which chemism began has been posited through the real process. Now further, their inner determinateness is as such essentially the contradiction of their simple indifferent subsistence and themselves as determinateness, and is the outward impulse that disrupts itself and posits tension in its determined object and in an other, in order that the object may have something to which it can relate as non-indifferent, with which it can neutralize itself and give to its simple determinateness an existent reality. Consequently, on the one hand chemism has gone back to its beginning in which objects in a state of reciprocal tension seek one another and then combine in a neutral product by means of a formal and external middle term; and, on the other hand, by thus going back to its concept, chemism sublates itself and has gone over into a higher sphere.",
    keyPoints: [
      "Elemental objects liberated from chemical tension",
      "In them original basis of presupposition with which chemism began",
      "Posited through real process",
      "Inner determinateness essentially contradiction",
      "Simple indifferent subsistence and themselves as determinateness",
      "Outward impulse that disrupts itself",
      "Posits tension in determined object and in other",
      "Object may have something to which it can relate as non-indifferent",
      "With which can neutralize itself",
      "Give simple determinateness existent reality",
      "On one hand chemism gone back to beginning",
      "Objects in reciprocal tension seek one another",
      "Combine in neutral product by means of formal and external middle term",
      "On other hand by going back to concept, chemism sublates itself",
      "Gone over into higher sphere"
    ],
    status: "pending",
    section: "B. THE PROCESS",
    order: 7,
    relatedChunks: ["process-6"]
  }
];

