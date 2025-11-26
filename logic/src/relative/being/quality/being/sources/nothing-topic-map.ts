import type { TopicMapEntry } from "@schema/topic";

/**
 * TopicMap for nothing.txt - B. NOTHING
 *
 * This TopicMap structures the logical development of Nothing as pure nothingness,
 * the second moment of the Logic. Focuses on the Species (B. NOTHING) only,
 * ignoring top-level Genus introduction.
 *
 * Structure: Pure nothingness as emptiness → Nothing in intuiting/thinking → Same as pure being
 */
export const nothingTopicMap: TopicMapEntry[] = [
  {
    id: "nothing-1",
    title: "Pure nothingness — simple equality and complete emptiness",
    lineRange: { start: 12, end: 16 },
    description: "Nothing, pure nothingness; it is simple equality with itself, complete emptiness, complete absence of determination and content; lack of all distinction within.",
    keyPoints: [
      "Nothing, pure nothingness",
      "Simple equality with itself",
      "Complete emptiness",
      "Complete absence of determination and content",
      "Lack of all distinction within"
    ],
    status: "pending",
    section: "B. NOTHING",
    order: 1,
    relatedChunks: ["nothing-2"]
  },
  {
    id: "nothing-2",
    title: "Nothing in intuiting and thinking — same as pure being",
    lineRange: { start: 17, end: 28 },
    description: "In so far as mention can be made here of intuiting and thinking, it makes a difference whether something or nothing is being intuited or thought. To intuit or to think nothing has therefore a meaning; the two are distinguished and so nothing is (concretely exists) in our intuiting or thinking; or rather it is the empty intuiting and thinking itself, like pure being. Nothing is therefore the same determination or rather absence of determination, and thus altogether the same as what pure being is.",
    keyPoints: [
      "Difference whether something or nothing is intuited or thought",
      "To intuit or think nothing has meaning",
      "Nothing is (concretely exists) in our intuiting or thinking",
      "Empty intuiting and thinking itself, like pure being",
      "Nothing is the same determination or absence of determination",
      "Altogether the same as what pure being is"
    ],
    status: "pending",
    section: "B. NOTHING",
    order: 2,
    relatedChunks: ["nothing-1"]
  }
];

