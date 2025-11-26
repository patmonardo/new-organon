/**
 * TopicMap for specific-quantum.txt - The Specific Quantum
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
 * - specific-quantum-chunks.md for detailed planning notes
 * - tools/source-analysis/SOURCE-ANALYSIS.md for workflow documentation
 * - tools/source-analysis/ARCHITECTURE.md for architectural overview
 */

import type { TopicMap } from '@schema/topic';
import { createTopicMap, createTopicMapEntry } from '@schema/topic';

export const SPECIFIC_QUANTUM_TOPIC_MAP: TopicMap = createTopicMap(
  'logic/src/relative/being/measure/specific-quantity/sources/specific-quantum.txt',
  'Hegel\'s Science of Logic - Measure',
  'The Specific Quantum',
  [
    createTopicMapEntry(
      'sq-a-1-measure-as-self-reference',
      'Measure as simple self-reference of quantum',
      [3, 19],
      'Measure = simple self-reference of quantum, its determinateness in itself. Quantum is qualitative. Immediate measure = immediate quantum = specific quantum. Quantum as self-referring externality is itself quality. Quantum = determinateness returned to simple self-equality, at one with determinate existence.',
      [
        'measure',
        'simple self-reference',
        'quantum',
        'own determinateness in itself',
        'quantum is qualitative',
        'immediate measure',
        'immediate quantum',
        'specific quantum',
        'quality',
        'no longer indifferent limit',
        'self-referring externality',
        'itself quality',
        'distinguished from',
        'does not extend past',
        'determinateness',
        'returned into simple self-equality',
        'at one with determinate existence',
      ],
      { section: 'The Specific Quantum', order: 1 }
    ),

    createTopicMapEntry(
      'sq-a-2-proposition-whatever-is-has-measure',
      'Proposition: "Whatever is, has a measure"',
      [21, 33],
      'Proposition: "Whatever is, has a measure." Every existence has magnitude belonging to its nature, constituting determinate nature and in-itself. Something not indifferent to magnitude; alteration of magnitude alters quality. As measure, quantum = determination of thing; exceeding or falling short = perishing.',
      [
        'proposition',
        'whatever is has a measure',
        'every existence',
        'magnitude',
        'belongs to nature',
        'constitutes determinate nature',
        'in-itself',
        'something not indifferent',
        'alteration of magnitude',
        'alters quality',
        'quantum as measure',
        'ceased to be limit which is none',
        'determination of thing',
        'exceed',
        'fall short',
        'perish',
      ],
      { section: 'The Specific Quantum', order: 2 }
    ),

    createTopicMapEntry(
      'sq-a-3-measure-as-standard',
      'Measure as standard — external vs original measure',
      [35, 70],
      'Measure as standard = quantum arbitrarily assumed as unit determinate in itself vs external amount. Unit can be determinate in itself (foot, etc.), but as measuring standard for other things = external measure, not original. Choice of fraction arbitrary. Universal standard = external comparison, not fundamental measure. Absolute standard = common, universal by convention only.',
      [
        'measure as standard',
        'quantum arbitrarily assumed',
        'unit determinate in itself',
        'external amount',
        'foot',
        'original measure',
        'measuring standard',
        'external measure',
        'not original measure',
        'diameter of earth',
        'length of pendulum',
        'choice of fraction',
        'arbitrary',
        'degree of latitude',
        'universal specific quantum',
        'particular things',
        'natural standard',
        'foolish',
        'universal standard',
        'external comparison',
        'superficial sense',
        'matter of indifference',
        'fundamental measure',
        'natural measures',
        'specifications',
        'universal measure',
        'absolute standard',
        'something common',
        'universal by convention',
      ],
      { section: 'The Specific Quantum', order: 3 }
    ),

    createTopicMapEntry(
      'sq-a-4-immediate-measure-double-determination',
      'Immediate measure — double determination and demise',
      [72, 108],
      'Immediate measure = simple determination of magnitude. Concrete existent has required size. As quantum = indifferent magnitude, fluctuating. As measure = restriction on fluctuation. Quantitative determinateness twofold: quality tied to it, yet quantity can fluctuate. Demise through alteration of quantum. Demise appears unexpected (quantum can alter without quality altering) but grasped by gradualness. But alteration = transition of quality into another, not just gradualness.',
      [
        'immediate measure',
        'simple determination of magnitude',
        'size of organic beings',
        'limbs',
        'concrete existent',
        'required size',
        'existence',
        'quantum',
        'indifferent magnitude',
        'open to external determination',
        'fluctuating increases and decreases',
        'measure',
        'distinct from itself as quantum',
        'restriction on fluctuation',
        'limit',
        'quantitative determinateness',
        'twofold',
        'quality tied to it',
        'quantity can fluctuate',
        'without prejudice to quality',
        'demise',
        'alteration of quantum',
        'unexpected',
        'alteration in quantum',
        'without measure and quality altered',
        'gradualness',
        'visualizing',
        'explaining',
        'disappearance of quality',
        'disappearance of something',
        'external limit',
        'by nature alterable',
        'transition of one quality into another',
        'transition of existence into non-existence',
        'decrease or increase',
        'one-sided holding fast to magnitude',
      ],
      { section: 'The Specific Quantum', order: 4 }
    ),

    createTopicMapEntry(
      'sq-a-5-bald-and-heap',
      'The bald and the heap — quantitative to qualitative alteration',
      [110, 140],
      'Ancients noticed: quantitative alteration suddenly becomes qualitative. Examples: "the bald" and "the heap" (elenchi). Question: does one hair/grain removal produce baldness/cease heap? Answer conceded (merely quantitative, insignificant). Repeated removal. Qualitative alteration revealed: bald, heap vanished. Forgot: repetition and that insignificant quantities add up to qualitative whole, which vanishes.',
      [
        'ancients',
        'taken notice',
        'coincidence',
        'quantitative alteration',
        'suddenly changes',
        'qualitative alteration',
        'popular examples',
        'inconsistencies',
        'the bald',
        'the heap',
        'elenchi',
        'Aristotle',
        'compelled to say opposite',
        'plucking one hair',
        'head',
        'horse\'s tail',
        'produce baldness',
        'heap',
        'one grain removed',
        'expected answer',
        'conceded',
        'merely quantitative difference',
        'insignificant',
        'repeated',
        'qualitative alteration revealed',
        'bald',
        'heap vanished',
        'repetition forgotten',
        'individually insignificant quantities',
        'disbursements from patrimony',
        'add up',
        'sum',
        'qualitative whole',
        'vanished',
        'purse empty',
      ],
      { section: 'The Specific Quantum', order: 5 }
    ),

    createTopicMapEntry(
      'sq-a-6-cunning-of-concept',
      'The cunning of the concept — quantum as indifferent limit',
      [142, 167],
      'Contradiction not sophistic. Mistake: assuming quantity = only indifferent limit. Truth: quantity = moment of measure, linked to quality. Elenchi correct: attest to mind interested in phenomena of thinking. Quantum as indifferent limit = side from which existence attacked. Cunning of concept: seizes existence where quality seems not to play; aggrandizement appears good fortune but brings misfortune.',
      [
        'embarrassment',
        'contradiction',
        'not sophistic',
        'pretense',
        'mistake',
        'assumed interlocutor',
        'ordinary consciousness',
        'assuming quantity',
        'only indifferent limit',
        'narrowly defined sense',
        'assumption confounded',
        'truth',
        'quantity is moment of measure',
        'linked to quality',
        'refuted',
        'one-sided stubborn adherence',
        'abstract determinateness of quantum',
        'elenchi',
        'not frivolous',
        'not pedantic',
        'basically correct',
        'attest to mind',
        'interest',
        'phenomena',
        'thinking',
        'quantum as indifferent limit',
        'side from which existence attacked',
        'laid low',
        'cunning of concept',
        'seizes existence',
        'quality does not seem to come into play',
        'aggrandizement',
        'State',
        'patrimony',
        'misfortune',
        'appears good fortune',
      ],
      { section: 'The Specific Quantum', order: 6 }
    ),

    createTopicMapEntry(
      'sq-a-7-specifying-of-measure',
      'Specifying of measure — two sides',
      [169, 187],
      'Measure in immediacy = ordinary quality of specific magnitude. Distinction: (1) quantum as indifferent limit (fluctuates without quality altering), (2) quantum as qualitative and specific. Both sides = magnitude determinations of same thing. Distinction immediate. Two sides have diverse concrete existence. Measure (magnitude determinate in itself) sublates indifference of alterable external side = specifying of measure.',
      [
        'measure',
        'immediacy',
        'ordinary quality',
        'specific magnitude',
        'appropriate',
        'distinction',
        'quantum as indifferent limit',
        'can fluctuate',
        'without quality altering',
        'quantum as qualitative and specific',
        'both sides',
        'magnitude determinations',
        'one and the same thing',
        'original immediacy of measure',
        'distinction immediate',
        'diverse concrete existence',
        'concrete existence of measure',
        'magnitude determinate in itself',
        'alterable external side',
        'sublates',
        'indifference',
        'specifying of measure',
      ],
      { section: 'The Specific Quantum', order: 7 }
    ),
  ],
  {
    sectionDescription: 'The Specific Quantum - Measure as simple self-reference of quantum (quantum is qualitative). Proposition: "Whatever is, has a measure." Measure as standard (external vs original, arbitrary choice). Immediate measure (double determination, demise through quantum alteration). The bald and the heap (quantitative to qualitative alteration, elenchi). The cunning of the concept (quantum as indifferent limit attacks existence). Specifying of measure (two sides with diverse concrete existence).',
  }
);

