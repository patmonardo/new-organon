/**
 * TopicMap for Reflection.txt - The Syllogism of Reflection
 *
 * SOURCE ANALYSIS PHASE 1: Topics
 *
 * COGNITIVE SCIENCE: This is where the real cognitive work happens.
 * The skill in producing good chunks and topics is what makes everything else meaningful.
 * The TopicMap helps check and improve understanding of Hegel through step-by-step analysis.
 *
 * Architecture:
 *    Source Text → [Source Analysis: Cognitive Science] → Chunks + Topics
 *                                                              ↓
 *                    [Logical Op Generation: IR Translation] → Logical Operations (IR)
 *                                                              ↓
 *                    [Codegen: Backend] → Executable Code
 *
 * This TopicMap provides the structured plan for chunking the source text
 * into meaningful chunks. Good chunking/topic analysis makes Logical Operations meaningful
 * (not just jargon) and enables executable codegen (the backend).
 *
 * Each entry maps to:
 * - TopicMapEntry.id → Chunk.id
 * - TopicMapEntry.title → Chunk.title AND LogicalOperation.label (the "Title")
 * - TopicMapEntry.lineRange → Extract text → Chunk.text
 *
 * Reference:
 * - reflection-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 *
 * Note: All mainline text included, including examples, even though examples won't become Logical Operations.
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const REFLECTION_SYLLOGISM_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/concept/subject/syllogism/sources/reflection.txt',
  'Hegel\'s Science of Logic - The Syllogism',
  'The Syllogism of Reflection',
  [
    createTopicMapEntry(
      'syl-refl-1-introduction-sublated',
      'Introduction: Sublated Abstractness; Middle Term as Totality',
      [4, 30],
      'Course of qualitative syllogism has sublated abstractness. Term posited as determinateness in which other shines reflectively. Connection present (mediated and necessary). Each determinateness concrete (not singly but with reference to others). Middle was abstract particularity (isolated, middle only externally). Now = totality (posited unity of extremes). Unity = reflection embracing extremes (first sublating, not yet absolute identity).',
      [
        'sublated abstractness',
        'shines reflectively',
        'connection',
        'mediated and necessary',
        'concrete determinateness',
        'abstract particularity',
        'totality',
        'posited unity',
        'reflection embracing',
        'first sublating',
        'absolute identity',
      ],
      { section: 'The Syllogism of Reflection', order: 1 }
    ),

    createTopicMapEntry(
      'syl-refl-2-extremes-middle',
      'Extremes from Judgment of Reflection; Middle Contains Totality',
      [31, 59],
      'Extremes = determinations of judgment of reflection (singularity proper, universality as relation/reflection embracing manifold). Singular contains universality absolutely reflected (presupposed genus). Middle contains: singularity, expanded into universality as "all", universality at basis (genus). First to possess genuine determinateness (middle = totality). Immediate indeterminate (abstract particularity). = syllogism of allness.',
      [
        'extremes',
        'judgment of reflection',
        'singularity proper',
        'universality',
        'absolutely reflected',
        'presupposed genus',
        'totality',
        'genuine determinateness',
        'abstract particularity',
        'syllogism of allness',
      ],
      { section: 'The Syllogism of Reflection', order: 2 }
    ),

    createTopicMapEntry(
      'syl-refl-3-allness-understanding',
      'Allness: Understanding in Perfection; External Universality',
      [63, 82],
      'Allness = understanding in perfection (but more not yet). Middle developed into moments (concrete). But form gathers singular into universality only externally. Singular behaves as immediate subsisting. Negation of immediacy = only first negation (not negation of negation, absolute immanent reflection). Singular determinations still lie at basis. Allness = external universality of reflection (not universality of concept).',
      [
        'understanding in perfection',
        'developed into moments',
        'concrete',
        'externally',
        'immediate subsisting',
        'first negation',
        'negation of negation',
        'absolute immanent reflection',
        'at basis',
        'external universality',
      ],
      { section: 'The Syllogism of Allness', order: 3 }
    ),

    createTopicMapEntry(
      'syl-refl-4-allness-concrete',
      'Allness: Concrete Totality; Examples',
      [84, 116],
      'Existence contingent (one determinateness, multitude, opposite predicates). But middle contains singularity (concrete) → only predicate concretely belonging attached. Examples: "green" → pleasing (but may be ugly); "regularity" → beautiful (but may be ugly). When allness: contains as concreted term (not abstraction). Only predicates commensurate with concrete totality. "All things green" = all actual concrete things (concreted with all properties).',
      [
        'contingent',
        'one determinateness',
        'multitude',
        'opposite predicates',
        'contains singularity',
        'concrete',
        'concreted term',
        'abstraction',
        'commensurate',
        'concrete totality',
        'all actual concrete things',
      ],
      { section: 'The Syllogism of Allness', order: 4 }
    ),

    createTopicMapEntry(
      'syl-refl-5-allness-illusion',
      'Allness: Mere Illusion; Major Premise Presupposes Conclusion',
      [118, 135],
      'Reflective perfection makes mere illusion. Middle = "all" (immediately attached predicate). But "all" = "all singulars" → subject already possesses predicate immediately (does not first obtain by inference). Or: subject obtains as consequence, but major already contains conclusion. Major not correct on its own (presupposes conclusion of which should be ground).',
      [
        'reflective perfection',
        'mere illusion',
        'all singulars',
        'already possesses',
        'immediately',
        'does not first obtain',
        'already contains',
        'not correct on its own',
        'presupposes conclusion',
      ],
      { section: 'The Syllogism of Allness', order: 5 }
    ),

    createTopicMapEntry(
      'syl-refl-6-example-mortal',
      'Example: "All humans are mortal"',
      [137, 152],
      'All humans mortal, Gaius human, Therefore Gaius mortal. Major correct only because/to extent conclusion correct. Were Gaius not mortal, major would not be correct. Conclusion must be correct on its own immediately (otherwise major would not include all singulars). Before major accepted, antecedent question: whether conclusion counter-instance.',
      [
        'much cited syllogism',
        'correct only because',
        'to extent',
        'not mortal',
        'would not be correct',
        'correct on its own',
        'immediately',
        'counter-instance',
      ],
      { section: 'The Syllogism of Allness', order: 6 }
    ),

    createTopicMapEntry(
      'syl-refl-7-result-posited',
      'Result Posited in Syllogism Itself; Essence Rests on Singularity',
      [154, 185],
      'From concept (existence): premises contradicted (presupposed other). In reflection: result posited in syllogism itself (major presupposes conclusion). = only external empty reflective semblance. Essence rests on subjective singularity (constitutes middle, possessing universality only externally). Or: singular connected immediately (not by inference). Major mediated through singularity as allness. = syllogism of induction.',
      [
        'contradicted',
        'presupposed',
        'posited in syllogism itself',
        'external empty reflective semblance',
        'essence rests on subjective singularity',
        'immediately',
        'not by inference',
        'mediated through',
        'syllogism of induction',
      ],
      { section: 'The Syllogism of Allness', order: 7 }
    ),

    createTopicMapEntry(
      'syl-refl-8-induction-schema',
      'Induction: U-S-P; Singularity as Completed',
      [189, 215],
      'Allness = S-P-U. Induction = U-S-P (singularity as completed, posited with opposite = universality). One extreme = predicate common to all singulars (immediate premises). Other = immediate genus (exhausted in collection). Configuration: U -- P with s, s, s, s ad infinitum.',
      [
        'first figure',
        'second figure',
        'singularity as completed',
        'posited with opposite determination',
        'predicate common',
        'immediate genus',
        'exhausted',
        'ad infinitum',
      ],
      { section: 'The Syllogism of Induction', order: 8 }
    ),

    createTopicMapEntry(
      'syl-refl-9-induction-experience',
      'Induction: Deficiency Eliminated; Syllogism of Experience',
      [217, 252],
      'Second figure did not correspond (S did not subsume). In induction: deficiency eliminated ("all singulars"). U-S: subject = objective universal/genus, predicate = equal extension (identical for external reflection). Example: Lion, elephant = genus quadruped (same content, indifferent determination, equality of extension). = syllogism of experience (subjective gathering, conjoining genus). Objective significance: genus determined through totality as universal property. But only inner concept, not yet posited.',
      [
        'deficiency eliminated',
        'all singulars',
        'equal extension',
        'identical',
        'external reflection',
        'indifferent determination',
        'equality of extension',
        'syllogism of experience',
        'subjective gathering',
        'objective significance',
        'universal property',
        'inner concept',
        'not yet posited',
      ],
      { section: 'The Syllogism of Induction', order: 9 }
    ),

    createTopicMapEntry(
      'syl-refl-10-induction-subjective',
      'Induction: Subjective; Bad Infinity',
      [254, 275],
      'Essentially still subjective. Middle = singulars in immediacy (collecting = external reflection). Universality only completeness (remains task). Progression into bad infinity recurs. Singularity ought identical with universality, but singulars immediate → intended unity = perpetual ought (unity of likeness). Supposed identical at same time supposed not identical. a, b, c, d, e = genus only in infinite (do not yield complete experience). Conclusion problematic.',
      [
        'essentially subjective',
        'immediacy',
        'external reflection',
        'completeness',
        'task',
        'bad infinity',
        'perpetual ought',
        'unity of likeness',
        'supposed identical',
        'supposed not identical',
        'infinite',
        'complete experience',
        'problematic',
      ],
      { section: 'The Syllogism of Induction', order: 10 }
    ),

    createTopicMapEntry(
      'syl-refl-11-induction-presupposes',
      'Induction: Presupposes Genus; Truth is Analogy',
      [277, 320],
      'Expresses perception ought to infinity → presupposes genus in and for itself conjoined. Presupposes conclusion as immediate. Experience assumed valid though perception not complete (no counter-instance only if true in and for itself). Based on immediacy, but not supposed immediacy (not singularity existing immediately but in and for itself = universal). Fundamental character: syllogistic inference. Singularity can only be middle if immediately identical with universality (objective universality = genus). Or: universality external but essential (external = immediately opposite = internal). Truth = syllogism with middle singularity immediately in itself universality = analogy.',
      [
        'presupposes genus',
        'in and for itself',
        'presupposes conclusion',
        'assumed as valid',
        'true in and for itself',
        'immediacy',
        'in and for itself',
        'fundamental character',
        'fall apart',
        'immediately identical',
        'objective universality',
        'external but essential',
        'immediately its opposite',
        'truth',
        'syllogism of analogy',
      ],
      { section: 'The Syllogism of Induction', order: 11 }
    ),

    createTopicMapEntry(
      'syl-refl-12-analogy-schema',
      'Analogy: S-U-P; Singular in Universal Nature',
      [324, 344],
      'Third figure S-U-P. Middle no longer single quality but universality (immanent reflection of concreted term = its nature). Universality of concreted term = in itself this concreted term. Singular = middle (taken in universal nature). Another singular (extreme) has same universal nature. Example: earth has inhabitants, moon is earth, Therefore moon has inhabitants.',
      [
        'third figure',
        'S-U-P',
        'no longer single quality',
        'universality',
        'immanent reflection',
        'concreted term',
        'nature',
        'in itself',
        'singular in universal nature',
        'same universal nature',
        'example',
      ],
      { section: 'The Syllogism of Analogy', order: 12 }
    ),

    createTopicMapEntry(
      'syl-refl-13-analogy-superficial',
      'Analogy: Superficiality; Form vs Content',
      [346, 413],
      'More superficial the more universal = mere quality/distinctive mark (identity = similarity). Superficiality (debased to representation) should have no place. Unacceptable: major as "similar in one mark similar in other" (form as content). What matters = form (not empirical content). Analogy = form peculiarly its own. What tempts: middle/extremes more determined (take appearance of content determination). Form determines itself to content = necessary advance (touches nature essentially). Cannot be regarded as other empirical content (abstraction cannot be made).',
      [
        'superficial',
        'mere quality',
        'distinctive mark',
        'similarity',
        'debased',
        'representation',
        'form as content',
        'empirical content',
        'form peculiarly its own',
        'more determined',
        'appearance of content determination',
        'necessary advance',
        'cannot be regarded',
        'abstraction cannot be made',
      ],
      { section: 'The Syllogism of Analogy', order: 13 }
    ),

    createTopicMapEntry(
      'syl-refl-14-analogy-quaternio',
      'Analogy: Quaternio Terminorum; Essential Universality',
      [415, 473],
      'With major premise: may seem four terms (quaternio terminorum). Two singulars; third = property immediately assumed; fourth = other properties. Middle = singularity but immediately true universality. In induction: indeterminate number (infinite). In allness: external form determination. In analogy: essential universality. "Earth" = universal nature/genus as singular. From one aspect: would not make imperfect. From another: undetermined whether determinateness because nature in general or particularity. Still syllogism of reflection (united immediately). Externality still there (singular = genus only in itself/implicitly). Predicate not already predicate of other (though both belong to genus).',
      [
        'quaternio terminorum',
        'four terms',
        'two singulars',
        'property immediately assumed',
        'other properties',
        'true universality',
        'indeterminate number',
        'infinite',
        'external form determination',
        'essential universality',
        'universal nature',
        'genus',
        'undetermined',
        'nature in general',
        'particularity',
        'still syllogism of reflection',
        'immediately',
        'externality',
        'in itself implicitly',
        'not posited',
        'not already predicate',
      ],
      { section: 'The Syllogism of Analogy', order: 14 }
    ),

    createTopicMapEntry(
      'syl-refl-15-analogy-sublation',
      'Analogy: Demands Sublation; Passes to Necessity',
      [475, 512],
      'S-P conclusion; one premise likewise S-P. Entails requirement premise also S-P. = demand to counter immediacy (presupposes conclusion). Presupposition moved into them (syllogisms of reflection). = demand to be mediated. Demands sublation of moment of singularity. Remains: objective universal (genus purified). Genus = moment only as immediate presupposition. Demands sublation → negation no longer immediate but posited. First negation; second now come. External universality determined as existing in and for itself. Conclusion identical with premises (mediation rejoined presupposition). Identity becomes higher universality.',
      [
        'likewise S-P',
        'entails requirement',
        'demand to counter',
        'presupposes conclusion',
        'moved into them',
        'demand to be mediated',
        'sublation',
        'moment of singularity',
        'objective universal',
        'genus purified',
        'immediate presupposition',
        'no longer immediate but posited',
        'first negation',
        'second',
        'existing in and for itself',
        'identical with premises',
        'rejoined presupposition',
        'higher universality',
      ],
      { section: 'The Syllogism of Analogy', order: 15 }
    ),

    createTopicMapEntry(
      'syl-refl-16-review-necessity',
      'Review: Mediation as Concrete Unity; Passes to Necessity',
      [514, 543],
      'Reviewing: mediation = posited/concrete unity of form determinations. Reflection = positing one in other (middle = allness). Singularity = essential ground (universality external, completeness). Universality essential to singular if conjoining middle (implicitly existing). Singular not united positively but sublated (negative moment). Universal = genus posited. Singular = externality (extreme). In general = P-S-U (singular essential). But immediacy sublated → S-U-P. Passed over into syllogism of necessity.',
      [
        'reviewing course',
        'posited concrete unity',
        'form determinations',
        'positing one in other',
        'allness',
        'essential ground',
        'external determination',
        'completeness',
        'essential to singular',
        'implicitly existing',
        'not united positively',
        'sublated',
        'negative moment',
        'genus posited',
        'externality',
        'extreme',
        'P-S-U',
        'essential determination',
        'immediacy sublated',
        'S-U-P',
        'passed over',
        'syllogism of necessity',
      ],
      { section: 'The Syllogism of Reflection', order: 16 }
    ),
  ],
  {
    sectionDescription: 'The Syllogism of Reflection - Sublated abstractness, middle as totality. Allness (understanding in perfection, external universality, mere illusion). Induction (singularity as completed, experience, bad infinity, presupposes genus). Analogy (singular in universal nature, superficiality, quaternio terminorum, demands sublation). Passes to syllogism of necessity.',
  }
);

