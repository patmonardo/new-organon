import type { TopicMapEntry } from "@schema/topic";

/**
 * TopicMap for being.txt - A. BEING
 *
 * This TopicMap structures the logical development of Being as pure being,
 * the first moment of the Logic. Focuses on the Species (A. BEING) only,
 * ignoring top-level Genus introduction.
 *
 * Structure: Pure being as indeterminate immediacy → Pure being as emptiness/nothing
 */
export const beingTopicMap: TopicMapEntry[] = [
  {
    id: "being-1",
    title: "Pure being without determination — indeterminate immediacy",
    lineRange: { start: 32, end: 39 },
    description: "Being, pure being, without further determination. In its indeterminate immediacy it is equal only to itself and also not unequal with respect to another; it has no difference within it, nor any outwardly. If any determination or content were posited in it as distinct, or if it were posited by this determination or content as distinct from an other, it would thereby fail to hold fast to its purity. It is pure indeterminateness.",
    keyPoints: [
      "Being, pure being, without further determination",
      "In indeterminate immediacy equal only to itself",
      "Not unequal with respect to another",
      "No difference within it, nor any outwardly",
      "If any determination or content were posited in it as distinct, it would fail to hold fast to its purity",
      "Pure indeterminateness"
    ],
    status: "pending",
    section: "A. BEING",
    order: 1,
    relatedChunks: ["being-2"]
  },
  {
    id: "being-2",
    title: "Pure being as emptiness — nothing to intuit or think",
    lineRange: { start: 40, end: 47 },
    description: "It is pure indeterminateness and emptiness. There is nothing to be intuited in it, if one can speak here of intuiting; or, it is only this pure empty intuiting itself. Just as little is anything to be thought in it, or, it is equally only this empty thinking. Being, the indeterminate immediate is in fact nothing, and neither more nor less than nothing.",
    keyPoints: [
      "Pure indeterminateness and emptiness",
      "Nothing to be intuited in it",
      "Only pure empty intuiting itself",
      "Nothing to be thought in it",
      "Equally only empty thinking",
      "Being, the indeterminate immediate is in fact nothing",
      "Neither more nor less than nothing"
    ],
    status: "pending",
    section: "A. BEING",
    order: 2,
    relatedChunks: ["being-1"]
  }
];

