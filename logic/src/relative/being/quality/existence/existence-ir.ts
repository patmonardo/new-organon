/**
 * Existence IR: Dialectic Pseudo-Code for the Determinateness of Being
 *
 * This module expresses the A. EXISTENCE section (Being → Nothing → Becoming → Existence)
 * as executable dialectic states built on the shared GDSL schema.
 *
 * Architecture: CPU (Quality / Being)
 * Scope: A. Existence as Such — Outline → Existence-in-general → Quality → Something
 */

import type {
  DialecticState,
  DialecticIR,
  Moment,
  Invariant,
  Force,
  Transition,
} from '@schema/dialectic';

/**
 * State 1: Outline — determinateness → quality → something
 */
const state1: DialecticState = {
  id: 'existence-1',
  title: 'Outline: determinateness → quality (reality|negation) → something (existent)',
  concept: 'ExistenceOutline',
  phase: 'quality',

  moments: [
    {
      name: 'determinateness',
      definition: 'The first content of existence as such',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'quality',
    },
    {
      name: 'quality',
      definition: 'Determinateness taken as reality and negation',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'reality',
    },
    {
      name: 'something',
      definition: 'Existence reflected into itself as an existent',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'exist-1-inv-1',
      constraint: 'determinateness → quality',
      predicate: 'implies(determinateness, quality)',
    },
    {
      id: 'exist-1-inv-2',
      constraint: 'quality.contains = {reality, negation}',
      predicate: 'contains(quality, {reality, negation})',
    },
    {
      id: 'exist-1-inv-3',
      constraint: 'existence.reflectedIntoItself = something',
      predicate: 'reflectedInto(existence, itself, something)',
    },
  ],

  forces: [
    {
      id: 'exist-1-force-1',
      description: 'Reflection into itself drives determinateness to appear as existence-in-general',
      type: 'mediation',
      trigger: 'quality.dualRealityNegation',
      effect: 'existence.posited = something',
      targetState: 'existence-2',
    },
  ],

  transitions: [
    {
      id: 'exist-1-trans-1',
      from: 'existence-1',
      to: 'existence-2',
      mechanism: 'mediation',
      description: 'Outline collapses into the immediate oneness that proceeds from becoming',
    },
  ],

  nextStates: ['existence-2'],
  previousStates: [],

  provenance: {
    topicMapId: 'existence-1',
    lineRange: { start: 4, end: 13 },
    section: 'A. EXISTENCE AS SUCH',
    order: 1,
  },

  description:
    'In existence as such, determinateness is first to be distinguished as quality. Quality must be taken in both determinations—reality and negation; as existence is reflected into itself, it is posited as something, an existent.',
  keyPoints: [
    'Existence as such: determinateness first',
    'Determinateness distinguished as quality',
    'Quality has determinations reality and negation',
    'Existence reflected into itself',
    'Reflection posits an existent something',
  ],
};

/**
 * State 2: Existence from becoming — simple oneness, immediacy
 */
const state2: DialecticState = {
  id: 'existence-2',
  title: 'Existence from becoming: simple oneness; immediate (becoming behind it)',
  concept: 'ExistenceFromBecoming',
  phase: 'quality',

  moments: [
    {
      name: 'being',
      definition: 'The one-sided determination that first appears in existence',
      type: 'determination',
    },
    {
      name: 'nothing',
      definition: 'The other determination contained within existence',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'being',
    },
    {
      name: 'becoming',
      definition: 'The mediation that has sublated itself and lies behind existence',
      type: 'process',
      relation: 'mediates',
      relatedTo: 'existence',
    },
  ],

  invariants: [
    {
      id: 'exist-2-inv-1',
      constraint: 'existence = unity(being, nothing)',
      predicate: 'unity(existence, being, nothing)',
    },
    {
      id: 'exist-2-inv-2',
      constraint: 'mediation(becoming) liesBehind existence',
      predicate: 'liesBehind(becoming, existence)',
    },
    {
      id: 'exist-2-inv-3',
      constraint: 'existence.appearsAs = firstImmediate',
      predicate: 'appearsAs(existence, firstImmediate)',
    },
  ],

  forces: [
    {
      id: 'exist-2-force-1',
      description: 'Latent nothing seeks expression, compelling existence toward Da-sein',
      type: 'contradiction',
      trigger: 'being.oneSided = true',
      effect: 'nonBeing.emerges = true',
      targetState: 'existence-3',
    },
  ],

  transitions: [
    {
      id: 'exist-2-trans-1',
      from: 'existence-2',
      to: 'existence-3',
      mechanism: 'passover',
      description: 'Simple immediacy passes over into Dasein: being with non-being',
    },
    {
      id: 'exist-2-trans-2',
      from: 'existence-2',
      to: 'existence-4',
      mechanism: 'reflection',
      description: 'Recognition that being is only a moment prepares reflection vs. posited distinction',
    },
  ],

  nextStates: ['existence-3', 'existence-4'],
  previousStates: ['existence-1'],

  provenance: {
    topicMapId: 'existence-2',
    lineRange: { start: 17, end: 28 },
    section: 'a. Existence in general',
    order: 2,
  },

  description:
    'Existence proceeds from becoming. It is the simple oneness of being and nothing, appears as an immediate first because mediation lies behind it, and begins in the one-sided determination of being while containing nothing implicitly.',
  keyPoints: [
    'Existence proceeds from becoming',
    'Simple unity of being and nothing',
    'Becoming is the sublated mediation behind existence',
    'Existence appears as first immediate',
    'Initially one-sided as being, with nothing latent',
  ],
};

/**
 * State 3: Dasein — being with non-being
 */
const state3: DialecticState = {
  id: 'existence-3',
  title: '"Da"-sein: being with non-being; determinateness as such (in form of being)',
  concept: 'DaseinUnity',
  phase: 'quality',

  moments: [
    {
      name: 'being',
      definition: 'Immediate presence (Sein)',
      type: 'determination',
    },
    {
      name: 'nonBeing',
      definition: 'Moment taken up into simple unity with being',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'being',
    },
    {
      name: 'unity',
      definition: 'Concrete whole in the form of being (immediacy)',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'exist-3-inv-1',
      constraint: 'dasein = being + nonBeing (simple unity)',
      predicate: 'unity(dasein, being, nonBeing)',
    },
    {
      id: 'exist-3-inv-2',
      constraint: 'determinateness = nonBeingWithinBeing',
      predicate: 'determinatenessFrom(nonBeing, being)',
    },
    {
      id: 'exist-3-inv-3',
      constraint: 'representationOfSpace ≠ applicable',
      predicate: 'excluded(representationOfSpace, dasein)',
    },
  ],

  forces: [
    {
      id: 'exist-3-force-1',
      description: 'Distinction between what is posited in reflection and in the thing pushes toward explicit differentiation',
      type: 'reflection',
      trigger: 'unity.perceivedByUs',
      effect: 'determinateness.posited = da',
      targetState: 'existence-4',
    },
  ],

  transitions: [
    {
      id: 'exist-3-trans-1',
      from: 'existence-3',
      to: 'existence-4',
      mechanism: 'reflection',
      description: 'From unity we distinguish being-as-moment (for us) versus determinateness posited in it',
    },
  ],

  nextStates: ['existence-4'],
  previousStates: ['existence-2'],

  provenance: {
    topicMapId: 'existence-3',
    lineRange: { start: 30, end: 43 },
    section: 'a. Existence in general',
    order: 3,
  },

  description:
    'Dasein is being with non-being such that non-being is taken up into simple unity with being, producing a concrete whole in the form of being and constituting determinateness as such.',
  keyPoints: [
    'Existence (Dasein) is being with non-being',
    'Non-being taken up into unity with being',
    'Concrete whole remains in form of being',
    'Determinateness is constituted through this unity',
  ],
};

/**
 * State 4: Reflection vs posited I — being-as-moment vs determinateness
 */
const state4: DialecticState = {
  id: 'existence-4',
  title: 'Reflection vs posited I: being-as-moment (for us) vs determinateness-posited (in it)',
  concept: 'ReflectionVsPosited',
  phase: 'quality',

  moments: [
    {
      name: 'beingAsMoment',
      definition: 'Being shown as sublated moment (for us)',
      type: 'negation',
    },
    {
      name: 'determinatenessInIt',
      definition: 'Determination that is posited within existence itself',
      type: 'determination',
    },
    {
      name: 'da',
      definition: 'The “there” that expresses determinateness as such',
      type: 'quality',
      relation: 'contains',
      relatedTo: 'determinatenessInIt',
    },
  ],

  invariants: [
    {
      id: 'exist-4-inv-1',
      constraint: 'beingAsMoment = forReflectionOnly',
      predicate: 'scope(beingAsMoment, reflection)',
    },
    {
      id: 'exist-4-inv-2',
      constraint: 'determinatenessInIt ≠ ourReflection',
      predicate: 'distinct(determinatenessInIt, reflection)',
    },
    {
      id: 'exist-4-inv-3',
      constraint: 'da expresses determinateness',
      predicate: 'expresses(da, determinateness)',
    },
  ],

  forces: [
    {
      id: 'exist-4-force-1',
      description: 'Need to separate commentary from the fact itself drives clarification',
      type: 'mediation',
      trigger: 'reflection.overreaches',
      effect: 'callAttention(distinction)',
      targetState: 'existence-5',
    },
  ],

  transitions: [
    {
      id: 'exist-4-trans-1',
      from: 'existence-4',
      to: 'existence-5',
      mechanism: 'reflection',
      description: 'From first distinction we articulate the scope of reflection relative to what is posited',
    },
  ],

  nextStates: ['existence-5'],
  previousStates: ['existence-3'],

  provenance: {
    topicMapId: 'existence-4',
    lineRange: { start: 45, end: 55 },
    section: 'a. Existence in general',
    order: 4,
  },

  description:
    'The whole is in the form of being, but being as a moment is only for us; what is posited in the thing is determinateness as such (“da”). These two viewpoints must be kept distinct.',
  keyPoints: [
    'Whole is in form of being since being proved a moment',
    'Being-as-moment is for reflection, not yet posited in the thing',
    'What is posited is determinateness as such (“da”)',
    'Two perspectives must remain distinct',
  ],
};

/**
 * State 5: Reflection vs posited II — scope of commentary
 */
const state5: DialecticState = {
  id: 'existence-5',
  title: 'Reflection vs posited II: scope of commentary vs moments of the fact itself',
  concept: 'ReflectionScope',
  phase: 'quality',

  moments: [
    {
      name: 'conceptContent',
      definition: 'What is posited in the concept and belongs to its development',
      type: 'determination',
    },
    {
      name: 'externalReflection',
      definition: 'Determinateness not yet posited that belongs to commentary/anticipation',
      type: 'mediation',
      relation: 'opposite',
      relatedTo: 'conceptContent',
    },
    {
      name: 'unityBeingNothing',
      definition: 'Whole that appears in one-sided determination of being',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'exist-5-inv-1',
      constraint: 'onlyPositedDeterminateness ∈ conceptContent',
      predicate: 'belongsTo(positedDeterminateness, conceptContent)',
    },
    {
      id: 'exist-5-inv-2',
      constraint: 'externalReflection = clarification/anticipation only',
      predicate: 'scope(externalReflection, clarification)',
    },
    {
      id: 'exist-5-inv-3',
      constraint: 'distinction(reflection, factMoments) maintained',
      predicate: 'distinct(externalReflection, factMoment)',
    },
  ],

  forces: [
    {
      id: 'exist-5-force-1',
      description: 'Clarifying the boundary prepares the move to determinate correspondence with being',
      type: 'mediation',
      trigger: 'reflection.lingers',
      effect: 'returnToConcept = true',
      targetState: 'existence-6',
    },
  ],

  transitions: [
    {
      id: 'exist-5-trans-1',
      from: 'existence-5',
      to: 'existence-6',
      mechanism: 'passover',
      description: 'Having delimited reflection, we compare existence with being itself',
    },
  ],

  nextStates: ['existence-6'],
  previousStates: ['existence-4'],

  provenance: {
    topicMapId: 'existence-5',
    lineRange: { start: 56, end: 92 },
    section: 'a. Existence in general',
    order: 5,
  },

  description:
    'Only what is posited in the concept belongs to its content; unposited determinations belong to our reflection for clarification or anticipation. Reflection should be distinguished from what constitutes a moment of the fact itself.',
  keyPoints: [
    'Posited content belongs to the concept’s development',
    'External reflection aids overview but remains ancillary',
    'Unity of being and nothing in one-sided being is external reflection',
    'Must distinguish reflection from moments of the fact itself',
  ],
};

/**
 * State 6: Correspondence — being vs existence
 */
const state6: DialecticState = {
  id: 'existence-6',
  title: 'Correspondence: being (indeterminate) vs existence (determinate being, concrete)',
  concept: 'BeingExistenceCorrespondence',
  phase: 'quality',

  moments: [
    {
      name: 'being',
      definition: 'Indeterminate immediacy without inner determinations',
      type: 'quality',
    },
    {
      name: 'existence',
      definition: 'Determinate being, concrete with internal relations',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'determinations',
    },
    {
      name: 'determinations',
      definition: 'Plural relations emerging immediately within existence',
      type: 'moment',
    },
  ],

  invariants: [
    {
      id: 'exist-6-inv-1',
      constraint: 'being.indeterminate = true',
      predicate: 'indeterminate(being)',
    },
    {
      id: 'exist-6-inv-2',
      constraint: 'existence = determinateBeing',
      predicate: 'equals(existence, determinateBeing)',
    },
    {
      id: 'exist-6-inv-3',
      constraint: 'determinations.emergeImmediatelyIn(existence)',
      predicate: 'emergesImmediately(determinations, existence)',
    },
  ],

  forces: [
    {
      id: 'exist-6-force-1',
      description: 'Multiplying determinations leads to the qualitative analysis proper',
      type: 'mediation',
      trigger: 'existence.concrete = true',
      effect: 'quality.sequenceStarts = true',
      targetState: 'existence-7',
    },
  ],

  transitions: [
    {
      id: 'exist-6-trans-1',
      from: 'existence-6',
      to: 'existence-7',
      mechanism: 'passover',
      description: 'Determinate being becomes explicit quality (immediate unity of being and non-being)',
    },
  ],

  nextStates: ['existence-7'],
  previousStates: ['existence-5'],

  provenance: {
    topicMapId: 'existence-6',
    lineRange: { start: 94, end: 100 },
    section: 'a. Existence in general',
    order: 6,
  },

  description:
    'Existence corresponds to being. Whereas being is indeterminate, existence is determinate being—a concrete something in which multiple relations of its moments immediately emerge.',
  keyPoints: [
    'Being is indeterminate and without internal determinations',
    'Existence is determinate being, concrete',
    'Multiple determinations and relations arise immediately in existence',
  ],
};

/**
 * State 7: Quality I — immediacy of unity
 */
const state7: DialecticState = {
  id: 'existence-7',
  title: 'Quality I: immediacy of unity (existent ↔ non-being); no posited differentiation',
  concept: 'ImmediateQuality',
  phase: 'quality',

  moments: [
    {
      name: 'existent',
      definition: 'Existence in its immediacy',
      type: 'determination',
    },
    {
      name: 'nonBeing',
      definition: 'Moment inseparable from the existent',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'existent',
    },
    {
      name: 'unity',
      definition: 'Immediate unity of being and nothing',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'exist-7-inv-1',
      constraint: 'existent ⇔ nonBeing (coextensive)',
      predicate: 'coextensive(existent, nonBeing)',
    },
    {
      id: 'exist-7-inv-2',
      constraint: 'determinateness inseparable from being',
      predicate: 'inseparable(determinateness, being)',
    },
    {
      id: 'exist-7-inv-3',
      constraint: 'noDifferentiationPosited',
      predicate: 'notPosited(differentiation, unity)',
    },
  ],

  forces: [
    {
      id: 'exist-7-force-1',
      description: 'Isolating determinateness as existent determinateness defines quality',
      type: 'mediation',
      trigger: 'needToIsolateDeterminateness',
      effect: 'quality.defined = true',
      targetState: 'existence-8',
    },
  ],

  transitions: [
    {
      id: 'exist-7-trans-1',
      from: 'existence-7',
      to: 'existence-8',
      mechanism: 'sublation',
      description: 'Immediate unity is isolated as pure quality',
    },
  ],

  nextStates: ['existence-8'],
  previousStates: ['existence-6', 'existence-1'],

  provenance: {
    topicMapId: 'existence-7',
    lineRange: { start: 104, end: 119 },
    section: 'b. Quality',
    order: 7,
  },

  description:
    'Being and nothing are one in existence such that neither oversteps the other; determinateness has yet to detach from being, so no differentiation is posited. Quality is this immediate unity.',
  keyPoints: [
    'Being and nothing immediately one in existence',
    'To the extent something exists it is equally non-being',
    'Determinateness inseparable from being',
    'No differentiation yet posited',
  ],
};

/**
 * State 8: Quality II — definition of quality
 */
const state8: DialecticState = {
  id: 'existence-8',
  title: 'Quality II: definition (existent determinateness; simple, immediate)',
  concept: 'QualityDefinition',
  phase: 'quality',

  moments: [
    {
      name: 'existentDeterminateness',
      definition: 'Determinateness isolated by itself',
      type: 'determination',
    },
    {
      name: 'simplicity',
      definition: 'Totally simple immediacy of quality',
      type: 'quality',
    },
    {
      name: 'quantitativePotential',
      definition: 'Further determination that could become quantity',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'exist-8-inv-1',
      constraint: 'quality = existentDeterminateness',
      predicate: 'equals(quality, existentDeterminateness)',
    },
    {
      id: 'exist-8-inv-2',
      constraint: 'quality.simple = true',
      predicate: 'simple(quality)',
    },
    {
      id: 'exist-8-inv-3',
      constraint: 'determinateness.general ≥ quality',
      predicate: 'moreUniversal(determinatenessGeneral, quality)',
    },
  ],

  forces: [
    {
      id: 'exist-8-force-1',
      description: 'Reflection of equality of being and nothing introduces reality/negation polarity',
      type: 'reflection',
      trigger: 'quality.reconsidered',
      effect: 'realityNegationPair = posited',
      targetState: 'existence-9',
    },
  ],

  transitions: [
    {
      id: 'exist-8-trans-1',
      from: 'existence-8',
      to: 'existence-9',
      mechanism: 'reflection',
      description: 'Quality becomes distinct as reality versus negation',
    },
  ],

  nextStates: ['existence-9'],
  previousStates: ['existence-7'],

  provenance: {
    topicMapId: 'existence-8',
    lineRange: { start: 121, end: 129 },
    section: 'b. Quality',
    order: 8,
  },

  description:
    'Determinateness isolated by itself, as existent determinateness, is quality: something simple and immediate. Determinateness in general is more universal and can become quantitative.',
  keyPoints: [
    'Quality = existent determinateness',
    'Quality is totally simple and immediate',
    'Determinateness in general is more universal and can become quantity',
  ],
};

/**
 * State 9: Quality III — reality and negation
 */
const state9: DialecticState = {
  id: 'existence-9',
  title: 'Quality III: reality and negation; reflection; limit/restriction',
  concept: 'RealityNegation',
  phase: 'quality',

  moments: [
    {
      name: 'reality',
      definition: 'Quality in the distinct value of the existent',
      type: 'quality',
    },
    {
      name: 'negation',
      definition: 'Quality affected by negating, counting as lack/limit',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'reality',
    },
    {
      name: 'measure',
      definition: 'Existence as the measure of one-sidedness',
      type: 'mediation',
    },
  ],

  invariants: [
    {
      id: 'exist-9-inv-1',
      constraint: 'existence.contains {being, nothing}',
      predicate: 'contains(existence, {being, nothing})',
    },
    {
      id: 'exist-9-inv-2',
      constraint: 'quality.positedInNothing = negation',
      predicate: 'positedIn(quality, nothing, negation)',
    },
    {
      id: 'exist-9-inv-3',
      constraint: 'negation = limit/restriction yet still quality',
      predicate: 'stillQuality(negation)',
    },
  ],

  forces: [
    {
      id: 'exist-9-force-1',
      description: 'Different valuations of reality and negation provoke explicit comparison',
      type: 'contradiction',
      trigger: 'reality.appearsPositive',
      effect: 'negation.assertsExistence = true',
      targetState: 'existence-10',
    },
  ],

  transitions: [
    {
      id: 'exist-9-trans-1',
      from: 'existence-9',
      to: 'existence-10',
      mechanism: 'reflection',
      description: 'Reality and negation are both existence but appear differently valued',
    },
  ],

  nextStates: ['existence-10'],
  previousStates: ['existence-8'],

  provenance: {
    topicMapId: 'existence-9',
    lineRange: { start: 131, end: 143 },
    section: 'b. Quality',
    order: 9,
  },

  description:
    'Quality must be posited equally in the determination of nothing, yielding reality (quality as existent) and negation (quality as lack/limit). Both are reflected determinations within existence.',
  keyPoints: [
    'Existence measures one-sided quality',
    'Quality posited in determination of nothing',
    'Reality = quality as existent',
    'Negation = quality as lack/limit but still a quality',
  ],
};

/**
 * State 10: Quality IV — valuation of reality vs negation
 */
const state10Quality: DialecticState = {
  id: 'existence-10',
  title: 'Quality IV: valuation of reality vs negation (both are existence)',
  concept: 'QualityValuation',
  phase: 'quality',

  moments: [
    {
      name: 'reality',
      definition: 'Quality accenting being-an-existent, concealing its negation',
      type: 'quality',
    },
    {
      name: 'negation',
      definition: 'Quality interpreted as lack yet still an existent determination',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'exist-10-inv-1',
      constraint: 'reality & negation ⊆ existence',
      predicate: 'subset({reality, negation}, existence)',
    },
    {
      id: 'exist-10-inv-2',
      constraint: 'reality.concealsDeterminateness',
      predicate: 'conceals(reality, determinateness)',
    },
    {
      id: 'exist-10-inv-3',
      constraint: 'negation = existenceWithNonBeing',
      predicate: 'determinedWith(negation, nonBeing)',
    },
  ],

  forces: [
    {
      id: 'exist-10-force-1',
      description: 'Recognizing both as existences ushers the move to “something” where reality/negation are moments',
      type: 'sublation',
      trigger: 'valuation.compared',
      effect: 'qualityUnseparatedFromExistence = true',
      targetState: 'existence-11',
    },
  ],

  transitions: [
    {
      id: 'exist-10-trans-1',
      from: 'existence-10',
      to: 'existence-11',
      mechanism: 'sublation',
      description: 'Reality and negation sublate into something where both are existences',
    },
  ],

  nextStates: ['existence-11'],
  previousStates: ['existence-9'],

  provenance: {
    topicMapId: 'existence-10',
    lineRange: { start: 145, end: 155 },
    section: 'b. Quality',
    order: 10,
  },

  description:
    'Reality, taken as sheer positivity, conceals that it is determinateness and thus negation; negation, if taken as mere lack, would be nothing, but it is a quality determined with non-being. Both are existences.',
  keyPoints: [
    'Reality and negation are both existence',
    'Reality hides its negation when accenting pure being',
    'Negation remains existence, not abstract nothing',
  ],
};

/**
 * State 11: Something I — reality and negation as existences
 */
const state11: DialecticState = {
  id: 'existence-11',
  title: 'Something I: reality ∧ negation as existences; quality unseparated from existence',
  concept: 'SomethingUnity',
  phase: 'quality',

  moments: [
    {
      name: 'reality',
      definition: 'Contains negation within itself as existence',
      type: 'quality',
    },
    {
      name: 'negation',
      definition: 'Equally existence belonging to the something',
      type: 'negation',
    },
    {
      name: 'quality',
      definition: 'Unseparated from existence; determinate being',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'exist-11-inv-1',
      constraint: 'reality.contains(negation)',
      predicate: 'contains(reality, negation)',
    },
    {
      id: 'exist-11-inv-2',
      constraint: 'negation.isExistence',
      predicate: 'isExistence(negation)',
    },
    {
      id: 'exist-11-inv-3',
      constraint: 'quality ≡ determinateExistence',
      predicate: 'equivalent(quality, determinateExistence)',
    },
  ],

  forces: [
    {
      id: 'exist-11-force-1',
      description: 'Presence of distinction + its nullity invites explicit sublation',
      type: 'sublation',
      trigger: 'distinction.presentAndNull',
      effect: 'sublation.process = engaged',
      targetState: 'existence-12',
    },
  ],

  transitions: [
    {
      id: 'exist-11-trans-1',
      from: 'existence-11',
      to: 'existence-12',
      mechanism: 'sublation',
      description: 'Reality/negation distinction is explicitly sublated into being-in-itself',
    },
  ],

  nextStates: ['existence-12'],
  previousStates: ['existence-10', 'existence-1'],

  provenance: {
    topicMapId: 'existence-11',
    lineRange: { start: 159, end: 171 },
    section: 'c. Something',
    order: 11,
  },

  description:
    'Reality and negation are present in existence but equally null and sublated. Reality contains negation; negation is existence. Quality is unseparated from existence, which is determinate qualitative being.',
  keyPoints: [
    'Reality contains negation',
    'Negation is an existent quality',
    'Quality remains inseparable from existence',
  ],
};

/**
 * State 12: Something II — sublation and being-in-itself
 */
const state12: DialecticState = {
  id: 'existence-12',
  title: 'Something II: sublation (not omission) → simplicity mediated → being-in-itself',
  concept: 'SublatedSomething',
  phase: 'quality',

  moments: [
    {
      name: 'distinction',
      definition: 'Difference within existence that cannot be omitted',
      type: 'determination',
    },
    {
      name: 'sublation',
      definition: 'Process that returns to simplicity through mediation',
      type: 'process',
    },
    {
      name: 'beingInItself',
      definition: 'Existence’s own determinateness after sublation',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'exist-12-inv-1',
      constraint: 'distinction.mustBeConsidered = true',
      predicate: 'necessary(distinction)',
    },
    {
      id: 'exist-12-inv-2',
      constraint: 'simplicity = mediatedThrough(sublation)',
      predicate: 'mediatedThrough(simplicity, sublation)',
    },
    {
      id: 'exist-12-inv-3',
      constraint: 'existence.state = beingInItself',
      predicate: 'state(existence, beingInItself)',
    },
  ],

  forces: [
    {
      id: 'exist-12-force-1',
      description: 'Being-in-itself as negation of negation initiates subjective trajectory',
      type: 'negation',
      trigger: 'sublation.completed',
      effect: 'negationOfNegation = posited',
      targetState: 'existence-13',
    },
  ],

  transitions: [
    {
      id: 'exist-12-trans-1',
      from: 'existence-12',
      to: 'existence-13',
      mechanism: 'negation',
      description: 'Being-in-itself becomes the first negation of negation (“something” proper)',
    },
  ],

  nextStates: ['existence-13'],
  previousStates: ['existence-11'],

  provenance: {
    topicMapId: 'existence-12',
    lineRange: { start: 173, end: 187 },
    section: 'c. Something',
    order: 12,
  },

  description:
    'Sublation of distinction is more than mere omission. Existence, distinction, and their sublation are all present. Simplicity is mediated through sublation; existence is thus being-in-itself, an existent something.',
  keyPoints: [
    'Sublation exceeds mere retraction',
    'Distinction must remain acknowledged',
    'Simplicity is mediated; existence becomes being-in-itself',
  ],
};

/**
 * State 13: Something III — negation of negation, beginning of subject
 */
const state13: DialecticState = {
  id: 'existence-13',
  title: 'Something III: negation of negation; beginning of the subject',
  concept: 'NegationOfNegation',
  phase: 'quality',

  moments: [
    {
      name: 'firstNegation',
      definition: 'Simple existent self-reference',
      type: 'negation',
    },
    {
      name: 'secondNegation',
      definition: 'Negation of negation, absolute negativity',
      type: 'negation',
    },
    {
      name: 'subjectSeed',
      definition: 'Indeterminate in-itself-ness that will become the subject',
      type: 'determination',
    },
  ],

  invariants: [
    {
      id: 'exist-13-inv-1',
      constraint: 'something = firstNegationOfNegation',
      predicate: 'equals(something, firstNegationOfNegation)',
    },
    {
      id: 'exist-13-inv-2',
      constraint: 'subjectSeed.indeterminate = true',
      predicate: 'indeterminate(subjectSeed)',
    },
    {
      id: 'exist-13-inv-3',
      constraint: 'mustDistinguish(firstNegation, secondNegation)',
      predicate: 'distinguish(firstNegation, secondNegation)',
    },
  ],

  forces: [
    {
      id: 'exist-13-force-1',
      description: 'Need to exhibit mediation-with-itself pushes toward explicit mediation',
      type: 'mediation',
      trigger: 'subjectSeed.seeksDetermination',
      effect: 'mediationWithItself = posited',
      targetState: 'existence-14',
    },
  ],

  transitions: [
    {
      id: 'exist-13-trans-1',
      from: 'existence-13',
      to: 'existence-14',
      mechanism: 'mediation',
      description: 'Negation of negation becomes mediation of the something with itself',
    },
  ],

  nextStates: ['existence-14'],
  previousStates: ['existence-12'],

  provenance: {
    topicMapId: 'existence-13',
    lineRange: { start: 189, end: 222 },
    section: 'c. Something',
    order: 13,
  },

  description:
    'Something is the first negation of negation—simple self-reference—but only the beginning of the subject. It must determine itself further. Absolute negativity lies at the base, and we must distinguish first from second negation.',
  keyPoints: [
    'Something = first negation of negation',
    'Determination of existent beings (life, thought, etc.)',
    'Beginning of subject; in-itself still indeterminate',
    'Distinguish abstract vs concrete negation',
  ],
};

/**
 * State 14: Something IV — mediation-with-itself
 */
const state14: DialecticState = {
  id: 'existence-14',
  title: 'Something IV: mediation-with-itself (vs alleged bare immediacy)',
  concept: 'SelfMediation',
  phase: 'quality',

  moments: [
    {
      name: 'selfReference',
      definition: 'Restoration of simple reference through negation of negation',
      type: 'determination',
    },
    {
      name: 'mediation',
      definition: 'Process whereby something relates itself to itself',
      type: 'mediation',
    },
    {
      name: 'immediacyPrinciple',
      definition: 'Alleged bare immediacy from which mediation is wrongly excluded',
      type: 'quality',
    },
  ],

  invariants: [
    {
      id: 'exist-14-inv-1',
      constraint: 'mediationPresentInEveryConcept = true',
      predicate: 'ubiquitous(mediation)',
    },
    {
      id: 'exist-14-inv-2',
      constraint: 'selfReference = mediationWithItself',
      predicate: 'equals(selfReference, mediationWithItself)',
    },
    {
      id: 'exist-14-inv-3',
      constraint: 'principleOfBareImmediacy refuted',
      predicate: 'refuted(immediacyPrinciple)',
    },
  ],

  forces: [
    {
      id: 'exist-14-force-1',
      description: 'Abstract mediation collapses into unity unless concrete sides emerge (otherness)',
      type: 'contradiction',
      trigger: 'mediation.takenAsPure',
      effect: 'seekConcreteSides = true',
      targetState: 'existence-15',
    },
  ],

  transitions: [
    {
      id: 'exist-14-trans-1',
      from: 'existence-14',
      to: 'existence-15',
      mechanism: 'negation',
      description: 'Pure mediation, lacking sides, collapses into unity and demands “other”',
    },
  ],

  nextStates: ['existence-15'],
  previousStates: ['existence-13'],

  provenance: {
    topicMapId: 'existence-14',
    lineRange: { start: 224, end: 244 },
    section: 'c. Something',
    order: 14,
  },

  description:
    'Something as negation of negation is mediation of itself with itself. This mediation already appeared abstractly in becoming; now it is posited in something as simple identity, refuting any principle of bare immediacy.',
  keyPoints: [
    'Something is mediation with itself',
    'Mediation found everywhere, even in concepts',
    'Refutes doctrines of pure immediacy',
  ],
};

/**
 * State 15: Something V — mediation-only collapses, introduces other
 */
const state15: DialecticState = {
  id: 'existence-15',
  title: 'Something V: mediation-only collapses to unity; becoming with moments as existents (other)',
  concept: 'MediatedBecoming',
  phase: 'quality',

  moments: [
    {
      name: 'mediation',
      definition: 'Negation of negation lacking concrete sides',
      type: 'process',
    },
    {
      name: 'existenceMoment',
      definition: 'One moment now determined as existent being',
      type: 'determination',
    },
    {
      name: 'other',
      definition: 'Moment determined as negative of the something',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'exist-15-inv-1',
      constraint: 'mediationWithoutSides → collapseIntoBeing',
      predicate: 'collapsesInto(mediation, being)',
    },
    {
      id: 'exist-15-inv-2',
      constraint: 'something.isExistent = true',
      predicate: 'isExistent(something)',
    },
    {
      id: 'exist-15-inv-3',
      constraint: 'becoming.moments = {existenceMoment, other}',
      predicate: 'momentsOf(becoming, {existenceMoment, other})',
    },
  ],

  forces: [
    {
      id: 'exist-15-force-1',
      description: 'Having established the other, the system turns to alteration',
      type: 'passover',
      trigger: 'becoming.concrete = true',
      effect: 'alteration.concept = activated',
      targetState: 'existence-16',
    },
  ],

  transitions: [
    {
      id: 'exist-15-trans-1',
      from: 'existence-15',
      to: 'existence-16',
      mechanism: 'passover',
      description: 'Concrete becoming with two existent moments yields alteration',
    },
  ],

  nextStates: ['existence-16'],
  previousStates: ['existence-14'],

  provenance: {
    topicMapId: 'existence-15',
    lineRange: { start: 246, end: 257 },
    section: 'c. Something',
    order: 15,
  },

  description:
    'Pure mediation collapses into unity unless its sides are determined. Something is and is therefore existent; it is also becoming whose moments are now existents—one being, the other the negative (other).',
  keyPoints: [
    'Pure mediation collapses into simple being',
    'Something is an existent and also becoming',
    'Becoming now has two existent moments: the something and its other',
  ],
};

/**
 * State 16: Something VI — alteration
 */
const state16: DialecticState = {
  id: 'existence-16',
  title: 'Something VI: alteration; initially only in concept; other as qualitative',
  concept: 'Alteration',
  phase: 'quality',

  moments: [
    {
      name: 'transition',
      definition: 'Becoming whose moments are somethings',
      type: 'process',
    },
    {
      name: 'selfMaintenance',
      definition: 'Something maintaining itself in reference to itself',
      type: 'determination',
    },
    {
      name: 'qualitativeOther',
      definition: 'Negative posited as another something',
      type: 'negation',
    },
  ],

  invariants: [
    {
      id: 'exist-16-inv-1',
      constraint: 'alteration.initiallyConceptual',
      predicate: 'initiallyConceptual(alteration)',
    },
    {
      id: 'exist-16-inv-2',
      constraint: 'something.maintainsSelf = true',
      predicate: 'maintainsSelf(something)',
    },
    {
      id: 'exist-16-inv-3',
      constraint: 'other.positedAsQualitative',
      predicate: 'positedAs(qualitativeOther, qualitative)',
    },
  ],

  forces: [
    {
      id: 'exist-16-force-1',
      description: 'Conceptual alteration foreshadows the move to Something and Other dialectics (next module)',
      type: 'sublation',
      trigger: 'qualitativeOther.articulated',
      effect: 'transitionToNextSection = true',
      targetState: 'something-and-other-1',
    },
  ],

  transitions: [
    {
      id: 'exist-16-trans-1',
      from: 'existence-16',
      to: 'something-and-other-1',
      mechanism: 'sublation',
      description: 'Alteration matures into the explicit Something/Other dialectic',
    },
  ],

  nextStates: ['something-and-other-1'],
  previousStates: ['existence-15'],

  provenance: {
    topicMapId: 'existence-16',
    lineRange: { start: 258, end: 267 },
    section: 'c. Something',
    order: 16,
  },

  description:
    'Something, as becoming whose moments are themselves somethings, is alteration. Initially this alteration exists only in the concept: the something maintains itself, and its negative is posited as a qualitative other in general.',
  keyPoints: [
    'Something as transition with somethings for moments',
    'Alteration at first only conceptual',
    'Something maintains itself while its negative is a qualitative other',
  ],
};

/**
 * Existence IR Document
 */
export const existenceIR: DialecticIR = {
  id: 'existence-ir',
  title: 'Existence IR: Determinateness of Being',
  section: 'A. Existence as Such',
  states: [
    state1,
    state2,
    state3,
    state4,
    state5,
    state6,
    state7,
    state8,
    state9,
    state10Quality,
    state11,
    state12,
    state13,
    state14,
    state15,
    state16,
  ],
  metadata: {
    sourceFile: 'existence.txt',
    totalStates: 16,
    cpuGpuMapping: {
      'existence-1': 'quality',
      'existence-2': 'quality',
      'existence-3': 'quality',
      'existence-4': 'quality',
      'existence-5': 'quality',
      'existence-6': 'quality',
      'existence-7': 'quality',
      'existence-8': 'quality',
      'existence-9': 'quality',
      'existence-10': 'quality',
      'existence-11': 'quality',
      'existence-12': 'quality',
      'existence-13': 'quality',
      'existence-14': 'quality',
      'existence-15': 'quality',
      'existence-16': 'quality',
    },
  },
};

export const existenceStates = {
  'existence-1': state1,
  'existence-2': state2,
  'existence-3': state3,
  'existence-4': state4,
  'existence-5': state5,
  'existence-6': state6,
  'existence-7': state7,
  'existence-8': state8,
  'existence-9': state9,
  'existence-10': state10Quality,
  'existence-11': state11,
  'existence-12': state12,
  'existence-13': state13,
  'existence-14': state14,
  'existence-15': state15,
  'existence-16': state16,
};


