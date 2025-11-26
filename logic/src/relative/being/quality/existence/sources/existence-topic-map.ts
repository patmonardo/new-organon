import type { TopicMapEntry } from "@schema/topic";

/**
 * TopicMap for existence.txt - A. EXISTENCE AS SUCH
 *
 * This TopicMap structures the logical development of Existence as the first moment
 * of Qualitative Logic—the determinate being that emerges from Becoming.
 *
 * Structure:
 * - Outline: determinateness → quality (reality|negation) → something
 * - a. Existence in general: From becoming, da-sein, reflection vs posited
 * - b. Quality: Immediate unity, reality and negation, limit/restriction
 * - c. Something: Negation of negation, beginning of subject, mediation-with-itself, alteration
 *
 * Existence completes the foundational structure: Being → Nothing → Becoming → Existence
 */
export const existenceTopicMap: TopicMapEntry[] = [
  {
    id: "existence-1",
    title: "Outline: determinateness → quality (reality|negation) → something (existent)",
    lineRange: { start: 4, end: 13 },
    description: "In existence (a) as such, its determinateness is first (b) to be distinguished as quality. The latter, however, is to be taken in both the two determinations of existence as reality and negation. In these determinacies, however, existence is equally reflected into itself, and, as so reflected, it is posited as (c) something, an existent.",
    keyPoints: [
      "Existence as such: determinateness first",
      "Determinateness distinguished as quality",
      "Quality taken in both determinations: reality and negation",
      "Existence reflected into itself",
      "Posited as something, an existent"
    ],
    status: "pending",
    section: "A. EXISTENCE AS SUCH",
    order: 1,
    relatedChunks: ["existence-2", "existence-7", "existence-11"]
  },
  {
    id: "existence-2",
    title: "Existence from becoming: simple oneness; immediate (becoming behind it)",
    lineRange: { start: 17, end: 28 },
    description: "Existence proceeds from becoming. It is the simple oneness of being and nothing. On account of this simplicity, it has the form of an immediate. Its mediation, the becoming, lies behind it; it has sublated itself, and existence therefore appears as a first from which the forward move is made. It is at first in the one-sided determination of being; the other determination which it contains, nothing, will likewise come up in it, in contrast to the first.",
    keyPoints: [
      "Existence proceeds from becoming",
      "Simple oneness of being and nothing",
      "Form of an immediate",
      "Mediation (becoming) lies behind it, has sublated itself",
      "Appears as a first from which forward move is made",
      "Initially one-sided as being; nothing will appear in contrast"
    ],
    status: "pending",
    section: "a. Existence in general",
    order: 2,
    relatedChunks: ["existence-3", "existence-1"]
  },
  {
    id: "existence-3",
    title: "\"Da\"-sein: being with non-being; determinateness as such (in form of being)",
    lineRange: { start: 30, end: 43 },
    description: "It is not mere being but existence, or Dasein [in German]; according to its [German] etymology, it is being (Sein) in a certain place (da). But the representation of space does not belong here. As it follows upon becoming, existence is in general being with a non-being, so that this non-being is taken up into simple unity with being. Non-being thus taken up into being with the result that the concrete whole is in the form of being, of immediacy, constitutes determinateness as such.",
    keyPoints: [
      "Not mere being but existence, or Dasein",
      "Etymology: being (Sein) in a certain place (da) — but space representation excluded",
      "Being with non-being",
      "Non-being taken up into simple unity with being",
      "Concrete whole in form of being (immediacy)",
      "Constitutes determinateness as such"
    ],
    status: "pending",
    section: "a. Existence in general",
    order: 3,
    relatedChunks: ["existence-2", "existence-4"]
  },
  {
    id: "existence-4",
    title: "Reflection vs posited I: being-as-moment (for us) vs determinateness-posited (in it)",
    lineRange: { start: 45, end: 55 },
    description: "The whole is likewise in the form or determinateness of being, since in becoming being has likewise shown itself to be only a moment: something sublated, negatively determined. It is such, however, for us, in our reflection; not yet as posited in it. What is posited, however, is the determinateness as such of existence, as is also expressed by the da (or \"there\") of the Dasein. The two are always to be clearly distinguished.",
    keyPoints: [
      "Whole in form of being (since in becoming, being shown as only a moment)",
      "Being as moment: sublated, negatively determined",
      "For us, in our reflection — not yet posited in it",
      "What is posited: determinateness as such of existence (expressed by \"da\")",
      "Two always to be clearly distinguished"
    ],
    status: "pending",
    section: "a. Existence in general",
    order: 4,
    relatedChunks: ["existence-3", "existence-5"]
  },
  {
    id: "existence-5",
    title: "Reflection vs posited II: scope of commentary vs moments of the fact itself",
    lineRange: { start: 56, end: 92 },
    description: "Only that which is posited in a concept belongs in the course of the elaboration of the latter to its content. Any determinateness not yet posited in the concept itself belongs instead to our reflection, whether this reflection is directed to the nature of the concept itself or is a matter of external comparison. To remark on a determinateness of this last kind can only be for the clarification or anticipation of the whole that will transpire in the course of the development itself. That the whole, the unity of being and nothing, is in the one-sided determinateness of being is an external reflection; but in negation, in something and other, and so forth, it will become posited. It was necessary here to call attention to the distinction just given; but to comment on all that reflection can allow itself, to give an account of it, would lead to a long-winded anticipation of what must transpire in the fact itself. Although such reflections may serve to facilitate a general overview and thus facilitate understanding, they also bring the disadvantage of being seen as unjustified assertions, unjustified grounds and foundations, of what is to follow. They should be taken for no more than what they are supposed to be and should be distinguished from what constitutes a moment in the advance of the fact itself.",
    keyPoints: [
      "Only what is posited in concept belongs to its content",
      "Unposited determinateness belongs to our reflection (ancillary)",
      "Reflection: for clarification or anticipation",
      "External reflection: whole in one-sided determinateness of being",
      "Will become posited in negation, something, other, etc.",
      "Distinguish reflection from moments of the fact itself"
    ],
    status: "pending",
    section: "a. Existence in general",
    order: 5,
    relatedChunks: ["existence-4", "existence-6"]
  },
  {
    id: "existence-6",
    title: "Correspondence: being (indeterminate) vs existence (determinate being, concrete)",
    lineRange: { start: 94, end: 100 },
    description: "Existence corresponds to being in the preceding sphere. But being is the indeterminate; there are no determinations that therefore transpire in it. But existence is determinate being, something concrete; consequently, several determinations, several distinct relations of its moments, immediately emerge in it.",
    keyPoints: [
      "Existence corresponds to being in preceding sphere",
      "Being is indeterminate; no determinations transpire in it",
      "Existence is determinate being, something concrete",
      "Multiple determinations and distinct relations of moments emerge immediately"
    ],
    status: "pending",
    section: "a. Existence in general",
    order: 6,
    relatedChunks: ["existence-5", "existence-7"]
  },
  {
    id: "existence-7",
    title: "Quality I: immediacy of unity (existent ↔ non-being); no posited differentiation",
    lineRange: { start: 104, end: 119 },
    description: "On account of the immediacy with which being and nothing are one in existence, neither oversteps the other; to the extent that existence is existent, to that extent it is non-being; it is determined. Being is not the universal, determinateness not the particular. Determinateness has yet to detach itself from being; nor will it ever detach itself from it, since the now underlying truth is the unity of non-being with being; all further determinations will transpire on this basis. But the connection which determinateness now has with being is one of the immediate unity of the two, so that as yet no differentiation between the two is posited.",
    keyPoints: [
      "Immediacy: being and nothing one in existence",
      "Neither oversteps the other",
      "To the extent existent, to that extent non-being",
      "It is determined",
      "Being not universal, determinateness not particular",
      "Determinateness inseparable from being (unity basis)",
      "No differentiation between the two posited yet"
    ],
    status: "pending",
    section: "b. Quality",
    order: 7,
    relatedChunks: ["existence-6", "existence-8", "existence-1"]
  },
  {
    id: "existence-8",
    title: "Quality II: definition (existent determinateness; simple, immediate)",
    lineRange: { start: 121, end: 129 },
    description: "Determinateness thus isolated by itself, as existent determinateness, is quality: something totally simple, immediate. Determinateness in general is the more universal which, further determined, can be something quantitative as well. On account of this simplicity, there is nothing further to say about quality as such.",
    keyPoints: [
      "Determinateness isolated by itself, as existent determinateness, is quality",
      "Totally simple, immediate",
      "Determinateness in general more universal (can further determine as quantity)",
      "On account of simplicity, nothing further to say about quality as such"
    ],
    status: "pending",
    section: "b. Quality",
    order: 8,
    relatedChunks: ["existence-7", "existence-9"]
  },
  {
    id: "existence-9",
    title: "Quality III: reality and negation; reflection; limit/restriction",
    lineRange: { start: 131, end: 143 },
    description: "Existence, however, in which nothing and being are equally contained, is itself the measure of the one-sidedness of quality as an only immediate or existent determinateness. Quality is equally to be posited in the determination of nothing, and the result is that the immediate or existent determinateness is posited as distinct, reflected, and the nothing, as thus the determinate element of determinateness, will equally be something reflected, a negation. Quality, in the distinct value of existent, is reality; when affected by a negating, it is negation in general, still a quality but one that counts as a lack and is further determined as limit, restriction.",
    keyPoints: [
      "Existence measures one-sidedness of quality as mere immediacy",
      "Quality equally posited in determination of nothing",
      "Result: immediate determinateness posited as distinct, reflected",
      "Nothing as determinate element of determinateness becomes reflected negation",
      "Quality in distinct value of existent = reality",
      "Quality under negating = negation in general (lack, limit, restriction)"
    ],
    status: "pending",
    section: "b. Quality",
    order: 9,
    relatedChunks: ["existence-8", "existence-10"]
  },
  {
    id: "existence-10",
    title: "Quality IV: valuation of reality vs negation (both are existence)",
    lineRange: { start: 145, end: 155 },
    description: "Both are an existence, but in reality, as quality with the accent on being an existent, that it is determinateness and hence also negation is concealed; reality only has, therefore, the value of something positive from which negating, restriction, lack, are excluded. Negation, for its part, taken as mere lack, would be what nothing is; but it is an existence, a quality, only determined with a non-being.",
    keyPoints: [
      "Both are existence",
      "Reality: accent on being an existent — conceals its determinateness/negation",
      "Reality appears as pure positive (negating, restriction, lack excluded)",
      "Negation: if taken as mere lack, would be what nothing is",
      "But negation is existence, a quality — determined with non-being"
    ],
    status: "pending",
    section: "b. Quality",
    order: 10,
    relatedChunks: ["existence-9", "existence-11"]
  },
  {
    id: "existence-11",
    title: "Something I: reality ∧ negation as existences; quality unseparated from existence",
    lineRange: { start: 159, end: 171 },
    description: "In existence its determinateness has been distinguished as quality; in this quality as something existing, the distinction exists: the distinction of reality and negation. Now though these distinctions are present in existence, they are just as much null and sublated. Reality itself contains negation; it is existence, not indeterminate or abstract being. Negation is for its part equally existence, not the supposed abstract nothing but posited here as it is in itself, as existent, as belonging to existence. Thus quality is in general unseparated from existence, and the latter is only determinate, qualitative being.",
    keyPoints: [
      "Determinateness distinguished as quality",
      "Distinction exists: reality and negation",
      "Distinctions present in existence, yet null and sublated",
      "Reality contains negation (existence, not abstract being)",
      "Negation is existence (not abstract nothing, but existent, belonging to existence)",
      "Quality unseparated from existence",
      "Existence = determinate, qualitative being"
    ],
    status: "pending",
    section: "c. Something",
    order: 11,
    relatedChunks: ["existence-10", "existence-12", "existence-1"]
  },
  {
    id: "existence-12",
    title: "Something II: sublation (not omission) → simplicity mediated → being-in-itself",
    lineRange: { start: 173, end: 187 },
    description: "This sublating of the distinction is more than the mere retraction and external re-omission of it, or a simple return to the simple beginning, to existence as such. The distinction cannot be left out, for it is. Therefore, what de facto is at hand is this: existence in general, distinction in it, and the sublation of this distinction; the existence, not void of distinctions as at the beginning, but as again self-equal through the sublation of the distinction; the simplicity of existence mediated through this sublation. This state of sublation of the distinction is existence's own determinateness; existence is thus being-in-itself; it is existent, something.",
    keyPoints: [
      "Sublation more than retraction/external re-omission",
      "Not simple return to beginning",
      "Distinction cannot be left out, for it is",
      "What is at hand: existence + distinction + sublation of distinction",
      "Existence not void of distinctions, but self-equal through sublation",
      "Simplicity of existence mediated through sublation",
      "This state is existence's determinateness: being-in-itself",
      "It is existent, something"
    ],
    status: "pending",
    section: "c. Something",
    order: 12,
    relatedChunks: ["existence-11", "existence-13"]
  },
  {
    id: "existence-13",
    title: "Something III: negation of negation; beginning of the subject",
    lineRange: { start: 189, end: 222 },
    description: "Something is the first negation of negation, as simple existent self-reference. Existence, life, thought, and so forth, essentially take on the determination of an existent being, a living thing, a thinking mind (\"I\"), and so forth. This determination is of the highest importance if we do not wish to halt at existence, life, thought, and so forth, as generalities, also not at Godhood (instead of God). In common representation, something rightly carries the connotation of a real thing. Yet it still is a very superficial determination, just as reality and negation, existence and its determinateness, though no longer the empty being and nothing, still are quite abstract determinations. For this reason they also are the most common expressions, and a reflection that is still philosophically unschooled uses them the most; it casts its distinctions in them, fancying that in them it has something really well and firmly determined. As something, the negative of the negative is only the beginning of the subject; its in-itselfness is still quite indeterminate. It determines itself further on, at first as existent-for-itself and so on, until it finally obtains in the concept the intensity of the subject. At the base of all these determinations there lies the negative unity with itself. In all this, however, care must be taken to distinguish the first negation, negation as negation in general, from the second negation, the negation of negation which is concrete, absolute negativity, just as the first is on the contrary only abstract negativity.",
    keyPoints: [
      "Something = first negation of negation (simple existent self-reference)",
      "Existence, life, thought take on determination of existent being, living thing, thinking mind (\"I\")",
      "Important: not halt at generalities (Godhood vs God)",
      "Common representation: something = real thing",
      "Still superficial determination",
      "Beginning of subject (in-itselfness still indeterminate)",
      "Determines further: existent-for-itself → concept (intensity of subject)",
      "Base: negative unity with itself",
      "Distinguish: first negation (abstract) vs second negation (concrete, absolute negativity)"
    ],
    status: "pending",
    section: "c. Something",
    order: 13,
    relatedChunks: ["existence-12", "existence-14"]
  },
  {
    id: "existence-14",
    title: "Something IV: mediation-with-itself (vs alleged bare immediacy)",
    lineRange: { start: 224, end: 244 },
    description: "Something is an existent as the negation of negation, for such a negation is the restoration of the simple reference to itself; but the something is thereby equally the mediation of itself with itself. Present in the simplicity of something, and then with greater determinateness in being-for-itself, in the subject, and so forth, this mediation of itself with itself is also already present in becoming, but only as totally abstract mediation; mediation with itself is posited in the something in so far as the latter is determined as a simple identity. Attention can be drawn to the presence of mediation in general, as against the principle of the alleged bare immediacy of a knowledge from which mediation should be excluded. But there is no further need to draw particular attention to the moment of mediation, since it is to be found everywhere and on all sides, in every concept.",
    keyPoints: [
      "Something as negation of negation = restoration of self-reference",
      "But also mediation of itself with itself",
      "Present in simplicity of something, then in being-for-itself, subject, etc.",
      "Already present in becoming (but abstract)",
      "Mediation-with-itself posited in something as simple identity",
      "Against principle of alleged bare immediacy",
      "Mediation found everywhere, in every concept"
    ],
    status: "pending",
    section: "c. Something",
    order: 14,
    relatedChunks: ["existence-13", "existence-15"]
  },
  {
    id: "existence-15",
    title: "Something V: mediation-only collapses to unity; becoming with moments as existents (other)",
    lineRange: { start: 246, end: 257 },
    description: "This mediation with itself which something is in itself, when taken only as the negation of negation, has no concrete determinations for its sides; thus it collapses into the simple unity which is being. Something is, and is therefore also an existent. Further, it is in itself also becoming, but a becoming that no longer has only being and nothing for its moments. One of these moments, being, is now existence and further an existent. The other moment is equally an existent, but determined as the negative of something; an other.",
    keyPoints: [
      "Mediation-with-itself, taken only as negation of negation, has no concrete determinations for sides",
      "Collapses into simple unity which is being",
      "Something is, and is therefore also an existent",
      "In itself also becoming",
      "Becoming no longer has only being and nothing for moments",
      "One moment (being) = existence, further an existent",
      "Other moment = existent, but determined as negative of something: an other"
    ],
    status: "pending",
    section: "c. Something",
    order: 15,
    relatedChunks: ["existence-14", "existence-16"]
  },
  {
    id: "existence-16",
    title: "Something VI: alteration; initially only in concept; other as qualitative",
    lineRange: { start: 258, end: 267 },
    description: "As becoming, something is a transition, the moments of which are themselves something, and for that reason it is an alteration, a becoming that has already become concrete. At first, however, something alters only in its concept; it is not yet posited in this way, as mediated and mediating, but at first only as maintaining itself simply in its reference to itself; and its negative is posited as equally qualitative, as only an other in general.",
    keyPoints: [
      "As becoming, something is transition",
      "Moments are themselves something",
      "Therefore it is alteration (concrete becoming)",
      "At first, something alters only in its concept",
      "Not yet posited as mediated and mediating",
      "At first only maintaining itself simply in reference to itself",
      "Negative posited as equally qualitative, as only an other in general"
    ],
    status: "pending",
    section: "c. Something",
    order: 16,
    relatedChunks: ["existence-15"]
  }
];

