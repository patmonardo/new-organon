/**
 * TopicMap for contingency.txt - Contingency or Formal Actuality, Possibility, and Necessity
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
 * - contingency-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const CONTINGENCY_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/essence/actuality/actuality/sources/contingency.txt',
  'Hegel\'s Science of Logic - Actuality',
  'Contingency or Formal Actuality, Possibility, and Necessity',
  [
    createTopicMapEntry(
      'act-a-1-formal-actuality',
      'Formal actuality — contains possibility',
      [3, 13],
      'Formal actuality = first actuality, immediate, unreflected, only form determination, not totality of form = being/concrete existence. But by essence = form-unity of in-itselfness/inwardness and externality, immediately contains possibility. What is actual is possible.',
      [
        'formal actuality',
        'first actuality',
        'immediate',
        'unreflected',
        'form determination',
        'totality of form',
        'being',
        'concrete existence',
        'form-unity',
        'in-itselfness',
        'inwardness',
        'externality',
        'possibility',
        'what is actual is possible',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 1 }
    ),

    createTopicMapEntry(
      'act-a-2-possibility-reflected',
      'Possibility as actuality reflected into itself — two moments',
      [15, 32],
      'Possibility = actuality reflected into itself. Reflectedness = formal, determination of self-identity/in-itself. But determination = totality of form, in-itself determined as sublated, essentially only with reference to actuality, negative of actuality. Possibility entails two moments: (1) positive = being-reflected-into-itself, (2) negative = deficient, points to actuality, completed in other.',
      [
        'possibility',
        'actuality reflected into itself',
        'reflectedness',
        'formal',
        'determination of self-identity',
        'in-itself',
        'totality of form',
        'sublated',
        'negative of actuality',
        'two moments',
        'positive moment',
        'being-reflected-into-itself',
        'negative moment',
        'deficient',
        'points to actuality',
        'completed in other',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 2 }
    ),

    createTopicMapEntry(
      'act-a-3-formal-possibility-contradiction',
      'Formal possibility — contradiction, impossibility',
      [34, 53],
      'First positive side: possibility = form determination of self-identity, form of essentiality, relationless indeterminate receptacle. Formal possibility: everything possible that does not contradict itself, limitless manifoldness. But every manifold determined, possesses negation, indifferent diversity passes into opposition = contradiction. All things contradictory = impossible.',
      [
        'first positive side',
        'form determination of self-identity',
        'form of essentiality',
        'relationless',
        'indeterminate receptacle',
        'formal possibility',
        'everything possible',
        'does not contradict itself',
        'limitless manifoldness',
        'manifold determined',
        'possesses negation',
        'indifferent diversity',
        'opposition',
        'contradiction',
        'all things contradictory',
        'impossible',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 3 }
    ),

    createTopicMapEntry(
      'act-a-4-possible-says-nothing',
      '"It is possible" says nothing — principle of identity',
      [55, 72],
      '"It is possible" = purely formal assertion, superficial/empty as principle of contradiction. "A is possible" says no more than "A is A." Content undeveloped = simplicity, resolved into determinations = difference emerges. Stopping at simple = self-identical = possible. But says nothing, as principle of identity.',
      [
        '"it is possible"',
        'purely formal assertion',
        'superficial',
        'empty',
        'principle of contradiction',
        '"A is possible"',
        '"A is A"',
        'content undeveloped',
        'simplicity',
        'resolved into determinations',
        'difference emerges',
        'stopping at simple',
        'self-identical',
        'possible',
        'says nothing',
        'principle of identity',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 4 }
    ),

    createTopicMapEntry(
      'act-a-5-possible-more-than-identity',
      'Possible more than identity — ought-to-be, contradiction',
      [74, 92],
      'Possible more than identity. Possible = reflected immanent reflectedness, identical as moment of totality, determined not to be in itself, second determination = only possible, ought-to-be of totality. Without ought-to-be = essentiality. Absolute form: essence = moment, no truth without being. Possibility = mere essentiality, only moment, disproportionate. In-itself = only posited, not to be in itself. Internally: possibility = contradiction = impossibility.',
      [
        'possible more than identity',
        'reflected immanent reflectedness',
        'identical as moment of totality',
        'determined not to be in itself',
        'second determination',
        'only possible',
        'ought-to-be',
        'totality of form',
        'essentiality',
        'absolute form',
        'essence only moment',
        'no truth without being',
        'mere essentiality',
        'disproportionate',
        'in-itself only posited',
        'not to be in itself',
        'contradiction',
        'impossibility',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 5 }
    ),

    createTopicMapEntry(
      'act-a-6-possibility-contains-other',
      'Possibility contains its other — becomes actuality',
      [94, 124],
      'Possibility as form determination posited as sublated possesses content. Content = in-itself, sublated, otherness. Because only possible, opposite equally possible ("A is A", "not-A is not-A"). Statements indifferent, other not posited. Possibility = connection comparing, implies opposite also possible, determines both as possible. Connection (one possible contains other) = contradiction that sublates itself. Determined to be reflective, reflectively self-sublating = immediate = becomes actuality.',
      [
        'possibility as form determination',
        'posited as sublated',
        'possesses content',
        'in-itself',
        'sublated',
        'otherness',
        'opposite equally possible',
        '"A is A"',
        '"not-A is not-A"',
        'statements indifferent',
        'connection comparing',
        'opposite also possible',
        'determines both as possible',
        'one possible contains other',
        'contradiction',
        'sublates itself',
        'reflective',
        'reflectively self-sublating',
        'immediate',
        'becomes actuality',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 6 }
    ),

    createTopicMapEntry(
      'act-a-7-reflected-actuality',
      'Reflected actuality — unity with possibility',
      [126, 149],
      'This actuality = reflected actuality (not first), posited as unity with possibility. Actual = possible, immediate positive identity. But possibility determined as only possibility, actual = only possible. Possibility in actuality = sublated, only possibility. Actuality in unity = sublated immediacy, only moment, only possibility. Possibility not yet all actuality (not real/absolute). Formal possibility determined as only possibility = formless actuality = being/concrete existence. Everything possible has being/concrete existence.',
      [
        'reflected actuality',
        'not first actuality',
        'unity with possibility',
        'actual = possible',
        'immediate positive identity',
        'only possibility',
        'only possible',
        'sublated',
        'sublated immediacy',
        'only moment',
        'not yet all actuality',
        'not real actuality',
        'not absolute actuality',
        'formal possibility',
        'formless actuality',
        'being',
        'concrete existence',
        'everything possible has being',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 7 }
    ),

    createTopicMapEntry(
      'act-a-8-contingency-two-sides',
      'Contingency — two sides, groundless and grounded',
      [151, 226],
      'Unity of possibility and actuality = contingency. Contingent = actual determined as only possible, actual whose opposite equally is = mere being/concrete existence, posited as positedness/possibility. Possibility = immanent reflection, in-itself as positedness, possible = actual with value of contingent actuality = contingent. Two sides: (1) possibility immediately in it/sublated = immediate actuality, no ground, groundless. (2) Actual as only possible/positedness, possible as positedness, not in and for themselves, have immanent reflection in other = have ground. Contingent has no ground because contingent; has ground because contingent.',
      [
        'unity of possibility and actuality',
        'contingency',
        'contingent',
        'actual determined as only possible',
        'opposite equally is',
        'mere being',
        'concrete existence',
        'positedness',
        'possibility',
        'immanent reflection',
        'in-itself as positedness',
        'contingent actuality',
        'two sides',
        'possibility immediately in it',
        'sublated',
        'immediate actuality',
        'no ground',
        'groundless',
        'only possible',
        'not in and for themselves',
        'immanent reflection in other',
        'have ground',
        'has no ground because contingent',
        'has ground because contingent',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 8 }
    ),

    createTopicMapEntry(
      'act-a-9-absolute-restlessness-necessity',
      'Absolute restlessness — necessity',
      [184, 216],
      'Posited immediate conversion of inner/outer, immanently-reflected-being/being, each into other (moments of absolute form). Actuality in unity = concrete existence, groundless, only posited/possible; or reflected/separated = only possible. Possibility as in-itself = immediate, existent; or opposed = in-itself without actuality, only possible = concrete existence. Absolute restlessness of becoming = contingency. Each determination immediately turns into opposite, in opposite rejoins itself, identity of two (each in other) = necessity.',
      [
        'posited immediate conversion',
        'inner',
        'outer',
        'immanently-reflected-being',
        'being',
        'each into other',
        'moments of absolute form',
        'actuality in unity',
        'concrete existence',
        'groundless',
        'only posited',
        'only possible',
        'reflected',
        'separated',
        'possibility as in-itself',
        'immediate',
        'existent',
        'opposed',
        'in-itself without actuality',
        'absolute restlessness',
        'becoming',
        'contingency',
        'each determination turns into opposite',
        'rejoins itself',
        'identity of two',
        'each in other',
        'necessity',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 9 }
    ),

    createTopicMapEntry(
      'act-a-10-necessity',
      'Necessity — contingent is necessary',
      [218, 242],
      'Necessary = actual, immediate, groundless; but has actuality through other/in ground, positedness of ground, reflection into itself; possibility = sublated. Contingent = necessary because actual determined as possible; immediacy sublated, repelled into ground/in-itself, possibility (ground-grounded-connection) sublated, posited as being. Necessary is, existent = necessary. In itself, immanent reflection = other than immediacy, necessity = other. Existent not necessary; in-itself = positedness, sublated, immediate. Actuality in possibility = identical with itself = necessity.',
      [
        'necessary',
        'actual',
        'immediate',
        'groundless',
        'actuality through other',
        'in ground',
        'positedness of ground',
        'reflection into itself',
        'possibility sublated',
        'contingent = necessary',
        'actual determined as possible',
        'immediacy sublated',
        'repelled into ground',
        'repelled into in-itself',
        'ground-grounded-connection',
        'posited as being',
        'necessary is',
        'existent = necessary',
        'in itself',
        'immanent reflection',
        'other than immediacy',
        'necessity = other',
        'existent not necessary',
        'in-itself = positedness',
        'sublated',
        'immediate',
        'actuality in possibility',
        'identical with itself',
        'necessity',
      ],
      { section: 'Contingency or Formal Actuality, Possibility, and Necessity', order: 10 }
    ),
  ],
  {
    sectionDescription: 'Contingency or Formal Actuality, Possibility, and Necessity - Formal actuality contains possibility. Possibility as actuality reflected into itself, two moments. Formal possibility, contradiction, impossibility. "It is possible" says nothing. Possible more than identity, ought-to-be, contradiction. Possibility contains its other, becomes actuality. Reflected actuality, unity with possibility. Contingency, two sides (groundless and grounded). Absolute restlessness, necessity. Necessity, contingent is necessary.',
  }
);

