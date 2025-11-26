import type { TopicMapEntry } from "@schema/topic";

/**
 * TopicMap for becoming.txt - C. BECOMING
 *
 * This TopicMap structures the logical development of Becoming as the unity
 * of Being and Nothing, the third moment of the Logic. Focuses on the Species
 * (C. BECOMING) only, ignoring top-level Genus introduction.
 *
 * This is the most intense moment - the synthesis where Being and Nothing
 * unite in their vanishing movement, ultimately collapsing into existence.
 *
 * Structure:
 * - 1. Unity of being and nothing (same yet distinct)
 * - 2. The moments of becoming (unseparatedness, two unities, interpenetration)
 * - 3. Sublation of becoming (equilibrium, contradiction, transition to existence)
 */
export const becomingTopicMap: TopicMapEntry[] = [
  {
    id: "becoming-1",
    title: "C.1. Unity of being and nothing — same yet distinct",
    lineRange: { start: 11, end: 30 },
    description: "Pure being and pure nothing are therefore the same. The truth is neither being nor nothing, but rather that being has passed over into nothing and nothing into being; 'has passed over,' not passes over. But the truth is just as much that they are not without distinction; it is rather that they are not the same, that they are absolutely distinct yet equally unseparated and inseparable, and that each immediately vanishes in its opposite. Their truth is therefore this movement of the immediate vanishing of the one into the other: becoming, a movement in which the two are distinguished, but by a distinction which has just as immediately dissolved itself.",
    keyPoints: [
      "Pure being and pure nothing are the same",
      "Truth is neither being nor nothing",
      "Being has passed over into nothing and nothing into being",
      "'Has passed over,' not passes over",
      "They are not without distinction",
      "They are not the same, absolutely distinct",
      "Yet equally unseparated and inseparable",
      "Each immediately vanishes in its opposite",
      "Truth is movement of immediate vanishing of one into the other",
      "Becoming: movement in which two are distinguished",
      "Distinction has just as immediately dissolved itself"
    ],
    status: "pending",
    section: "C.1. Unity of being and nothing",
    order: 1,
    relatedChunks: ["becoming-2"]
  },
  {
    id: "becoming-2",
    title: "C.2. Becoming as unseparatedness — determinate unity",
    lineRange: { start: 33, end: 44 },
    description: "Becoming is the unseparatedness of being and nothing, not the unity that abstracts from being and nothing; as the unity of being and nothing it is rather this determinate unity, or one in which being and nothing equally are. However, inasmuch as being and nothing are each unseparated from its other, each is not. In this unity, therefore, they are, but as vanishing, only as sublated. They sink from their initially represented self-subsistence into moments which are still distinguished but at the same time sublated.",
    keyPoints: [
      "Becoming is unseparatedness of being and nothing",
      "Not unity that abstracts from being and nothing",
      "Unity of being and nothing is determinate unity",
      "One in which being and nothing equally are",
      "Each unseparated from its other, each is not",
      "In unity they are, but as vanishing, only as sublated",
      "Sink from self-subsistence into moments still distinguished but sublated"
    ],
    status: "pending",
    section: "C.2. The moments of becoming",
    order: 1,
    relatedChunks: ["becoming-1", "becoming-3"]
  },
  {
    id: "becoming-3",
    title: "C.2. Two unities — being and nothing as moments",
    lineRange: { start: 46, end: 60 },
    description: "Grasped as thus distinguished, each is in their distinguishedness a unity with the other. Becoming thus contains being and nothing as two such unities, each of which is itself unity of being and nothing; the one is being as immediate and as reference to nothing; the other is nothing as immediate and as reference to being; in these unities the determinations are of unequal value. Becoming is in this way doubly determined. In one determination, nothing is the immediate, that is, the determination begins with nothing and this refers to being; that is to say, it passes over into it. In the other determination, being is the immediate, that is, the determination begins with being and this passes over into nothing: coming-to-be and ceasing-to-be.",
    keyPoints: [
      "Each in distinguishedness is unity with the other",
      "Becoming contains being and nothing as two unities",
      "Each unity is itself unity of being and nothing",
      "One: being as immediate and reference to nothing",
      "Other: nothing as immediate and reference to being",
      "Determinations of unequal value",
      "Becoming doubly determined: coming-to-be and ceasing-to-be"
    ],
    status: "pending",
    section: "C.2. The moments of becoming",
    order: 2,
    relatedChunks: ["becoming-2", "becoming-4"]
  },
  {
    id: "becoming-4",
    title: "C.2. Coming-to-be and ceasing-to-be — interpenetration",
    lineRange: { start: 62, end: 82 },
    description: "Both are the same, becoming, and even as directions that are so different they interpenetrate and paralyze each other. The one is ceasing-to-be; being passes over into nothing, but nothing is just as much the opposite of itself, the passing-over into being, coming-to-be. This coming-to-be is the other direction; nothing goes over into being, but being equally sublates itself and is rather the passing-over into nothing; it is ceasing-to-be. They do not sublate themselves reciprocally [the one sublating the other externally] but each rather sublates itself in itself and is within it the opposite of itself.",
    keyPoints: [
      "Both are the same, becoming",
      "Directions interpenetrate and paralyze each other",
      "One is ceasing-to-be: being passes over into nothing",
      "Nothing is opposite of itself: passing-over into being, coming-to-be",
      "Coming-to-be is other direction: nothing goes over into being",
      "Being equally sublates itself: passing-over into nothing, ceasing-to-be",
      "They do not sublate themselves reciprocally",
      "Each rather sublates itself in itself",
      "Each is within it the opposite of itself"
    ],
    status: "pending",
    section: "C.2. The moments of becoming",
    order: 3,
    relatedChunks: ["becoming-3", "becoming-5"]
  },
  {
    id: "becoming-5",
    title: "C.3. Equilibrium and quiescent unity — vanishing of becoming",
    lineRange: { start: 85, end: 91 },
    description: "The equilibrium in which coming-to-be and ceasing-to-be are poised is in the first place becoming itself. But this becoming equally collects itself in quiescent unity. Being and nothing are in it only as vanishing; becoming itself, however, is only by virtue of their being distinguished. Their vanishing is therefore the vanishing of becoming, or the vanishing of the vanishing itself. Becoming is a ceaseless unrest that collapses into a quiescent result.",
    keyPoints: [
      "Equilibrium of coming-to-be and ceasing-to-be is becoming itself",
      "Becoming collects itself in quiescent unity",
      "Being and nothing are in it only as vanishing",
      "Becoming only by virtue of their being distinguished",
      "Vanishing is vanishing of becoming, vanishing of vanishing itself",
      "Becoming is ceaseless unrest that collapses into quiescent result"
    ],
    status: "pending",
    section: "C.3. Sublation of becoming",
    order: 1,
    relatedChunks: ["becoming-4", "becoming-6"]
  },
  {
    id: "becoming-6",
    title: "C.3. Contradiction and vanishedness — not nothing",
    lineRange: { start: 93, end: 99 },
    description: "This can also be expressed thus: becoming is the vanishing of being into nothing, and of nothing into being, and the vanishing of being and nothing in general; but at the same time it rests on their being distinct. It therefore contradicts itself in itself, because what it unites within itself is self-opposed; but such a union destroys itself. This result is a vanishedness, but it is not nothing; as such, it would be only a relapse into one of the already sublated determinations and not the result of nothing and of being.",
    keyPoints: [
      "Becoming is vanishing of being into nothing, nothing into being",
      "Vanishing of being and nothing in general",
      "Rests on their being distinct",
      "Contradicts itself in itself",
      "Union destroys itself",
      "Result is vanishedness, but not nothing",
      "Would be relapse into already sublated determinations",
      "Not result of nothing and of being"
    ],
    status: "pending",
    section: "C.3. Sublation of becoming",
    order: 2,
    relatedChunks: ["becoming-5", "becoming-7"]
  },
  {
    id: "becoming-7",
    title: "C.3. Transition to existence — quiescent simplicity",
    lineRange: { start: 101, end: 108 },
    description: "It is the unity of being and nothing that has become quiescent simplicity. But this quiescent simplicity is being, yet no longer for itself but as determination of the whole. Becoming, as transition into the unity of being and nothing, a unity which is as existent or has the shape of the one-sided immediate unity of these moments, is existence.",
    keyPoints: [
      "Unity of being and nothing that has become quiescent simplicity",
      "Quiescent simplicity is being",
      "No longer for itself but as determination of whole",
      "Becoming as transition into unity of being and nothing",
      "Unity which is as existent",
      "Has shape of one-sided immediate unity of these moments",
      "Is existence"
    ],
    status: "pending",
    section: "C.3. Sublation of becoming",
    order: 3,
    relatedChunks: ["becoming-6"]
  }
];

