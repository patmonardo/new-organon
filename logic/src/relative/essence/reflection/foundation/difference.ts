import type { Chunk, LogicalOperation } from './index'

/*
  Foundation — B. DIFFERENCE

  This module consolidates the complete Difference section:
  - Part 1: Absolute difference
  - Part 2: Diversity
  - Part 3: Opposition

  STRUCTURE: Essence → Foundation → Ground
  - Essence: The Essential and the Unessential
  - Foundation: The Determinations of Reflection (Identity, Difference, Contradiction)
  - Ground: The resolved contradiction (Absolute Ground, Determinate Ground, Condition)

  PHILOSOPHICAL NOTES:
  
  1. **Cogito as Instrumental Cause (Prayogikam)**:
     In Yoga philosophy, the Cogito (the "I think") functions as an Instrumental Cause.
     Difference, as the hard machinery of reflection, operates as this instrumental cause —
     it is the mechanism through which reflection determines itself.
  
  2. **Determination Power as Essential Cause**:
     The Determination Power (the power to determine, to differentiate) is an Essential Cause
     in the Aristotelian sense. Difference is this power made explicit — it is not merely
     a determination but the power of determination itself.
  
  3. **Difference as the Hard Machinery of Reflection**:
     Difference is the most complex and foundational machinery of reflection. It contains:
     - Absolute difference (the simple "not")
     - Diversity (indifferent multiplicity)
     - Opposition (mutual inclusion and exclusion)
     This machinery drives the entire movement from Identity through Difference to Contradiction.
*/

// ============================================================================
// PART 1: ABSOLUTE DIFFERENCE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'diff-1-absolute-overview',
    title: 'Absolute difference — overview',
    text: `Difference is the negativity that reflection possesses in itself: the nothing invoked in identity discourse. It is the essential moment of identity that determines itself and is thus differentiated from difference.`
  },
  {
    id: 'diff-2-difference-in-for-itself',
    title: 'Difference in and for itself (self-referring difference)',
    text: `This difference is absolute: not difference through something external but self‑referring and simple. The simple "not" constitutes the difference of reflection — a determinateness in itself rather than otherness of mere existence.`
  },
  {
    id: 'diff-3-contrast-existence-reflection',
    title: 'Contrast: reflection vs existence',
    text: `In existence, otherness lies outside and each existence remains immediate for itself. In reflection, by contrast, the other is other in and for itself — difference is posited as internal determinateness and not external otherness.`
  },
  {
    id: 'diff-4-moments-unity',
    title: 'Difference includes identity and difference as posited moments',
    text: `Difference contains both moments, identity and difference, each a posited determinateness. In this positedness each refers to itself: identity functions as the moment of immanent reflection while difference is itself reflected.`
  },
  {
    id: 'diff-5-positedness-self-reference',
    title: 'Positedness and self-reference of the moments',
    text: `Both identity and difference are reflections into themselves; each retains its referential character within the posited whole. Their mutual self-reference is the structural condition for diversity.`
  },
  {
    id: 'diff-6-diversity-emerges',
    title: 'Diversity as the result of dual reflective moments',
    text: `Because difference has these two reflective moments that are themselves self-referring, difference becomes diversity: a determinate multiplicity grounded in the unity of difference and identity.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'diff-op-1-declare-absolute',
    chunkId: 'diff-1-absolute-overview',
    label: 'Declare difference as reflective negativity',
    clauses: ['difference = negativityOfReflection', 'identity.includes = differenceAsMoment'],
    predicates: [{ name: 'IsReflectiveDifference', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'difference', to: 'identityMoment' }]
  },
  {
    id: 'diff-op-2-self-referring',
    chunkId: 'diff-2-difference-in-for-itself',
    label: 'Formalize self-referring / simple not',
    clauses: ['difference.selfReferring = true', 'difference.simpleNot = true', 'notExternalMediation = true'],
    predicates: [{ name: 'IsAbsoluteDifference', args: [] }],
    relations: [{ predicate: 'distinguishes', from: 'absoluteDifference', to: 'externalOtherness' }]
  },
  {
    id: 'diff-op-3-contrast-existence',
    chunkId: 'diff-3-contrast-existence-reflection',
    label: 'Contrast existential otherness with reflective difference',
    clauses: ['existence.otherness = external', 'reflection.otherness = internal', 'determinateness = inItself'],
    predicates: [{ name: 'ContrastsExistenceReflection', args: [] }],
    relations: [{ predicate: 'contrastsWith', from: 'reflectionDifference', to: 'existentialOtherness' }]
  },
  {
    id: 'diff-op-4-moments-declare',
    chunkId: 'diff-4-moments-unity',
    label: 'Declare identity + difference as mutual posited moments',
    clauses: [
      'difference.includes = {identity, difference}',
      'each.moment = positedDeterminateness',
      'each.refersTo = itself'
    ],
    predicates: [{ name: 'HasPositedMoments', args: ['difference'] }],
    relations: [{ predicate: 'composes', from: 'difference', to: 'moments(identity,difference)' }]
  },
  {
    id: 'diff-op-5-self-reference-formalize',
    chunkId: 'diff-5-positedness-self-reference',
    label: 'Formalize self-reference of moments',
    clauses: [
      'identity.refersTo = identity',
      'difference.refersTo = difference',
      'mutualSelfReference => structuralCondition(diversity)'
    ],
    predicates: [{ name: 'IsMutualSelfReference', args: [] }],
    relations: [{ predicate: 'grounds', from: 'mutualSelfReference', to: 'diversity' }]
  },
  {
    id: 'diff-op-6-diversity-result',
    chunkId: 'diff-6-diversity-emerges',
    label: 'Encode diversity emergence from internal duality',
    clauses: [
      'if difference.hasDualReflectiveMoments then result = diversity',
      'diversity = determinateMultiplicityGroundedInUnity'
    ],
    predicates: [{ name: 'ProducesDiversity', args: ['difference'] }],
    relations: [{ predicate: 'yields', from: 'internalDuality', to: 'diversity' }]
  }
]

// ============================================================================
// PART 2: DIVERSITY
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'div-1-overview',
    title: 'Diversity — overview',
    text: `Identity internally breaks apart into diversity because absolute difference posits itself as the negative of itself. The two moments (itself and its negative) are reflections into themselves and remain identical with themselves, yielding diversity.`
  },
  {
    id: 'div-2-substance-of-diversity',
    title: 'Diversity as subsistence of the negative',
    text: `The different subsists as diverse, indifferent to others, because identity constitutes its base. The diverse remains itself even in its opposite; diversity constitutes the otherness proper to reflection rather than the otherness of mere existence.`
  },
  {
    id: 'div-3-moments-self-referring',
    title: 'Moments of difference are self‑referring',
    text: `The moments of difference — identity and difference itself — are diverse when each is reflected into itself and refers only to itself. In this structure the two moments are not differentiated with respect to each other; their difference becomes external.`
  },
  {
    id: 'div-4-externality-and-indifference',
    title: 'Externality, indifference, and mutual conduct',
    text: `Because the moments are self‑referring and indifferent, they conduct themselves as moments different in general. They are indifferent to each other and to their determinateness, and difference thus appears as external in relation to them.`
  },
  {
    id: 'div-5-positedness-and-externality',
    title: 'Positedness and externality in diversity',
    text: `In diversity, reflection becomes generally external: difference is a positedness and yet encompasses the whole reflection. Identity and difference are reflections that are themselves wholes; determinateness as only identity or only difference is a sublated positedness.`
  },
  {
    id: 'div-6-duplicity-immanent-negation',
    title: 'Duplicity: immanent reflection and determinateness as negation',
    text: `There is a duplicity: immanent reflection as such, and determinateness as negation or positedness. Positedness is reflection external to itself — self‑referring implicitly, but referring to something external in its positedness.`
  },
  {
    id: 'div-7-immanent-vs-external-reflection',
    title: 'Immanent versus external reflection (moments of difference)',
    text: `Reflection-in-itself (immanent) and external reflection are the two determinations in which identity and difference are posited. Immanent reflection is identity made indifferent to difference (diversity); external reflection is determinate difference and likeness/unlikeness as posited relations dependent on a third.`
  },
  {
    id: 'div-8-likeness-and-unlikeness',
    title: 'Likeness and unlikeness as external determinations',
    text: `External identity is likeness and external difference is unlikeness: identity/unlikeness are posited relations, not properties in and for themselves. Whether something is like or unlike another depends on a third external standpoint.`
  },
  {
    id: 'div-9-external-connecting',
    title: 'External reflection connects diversity to likeness/unlikeness',
    text: `External reflection refers diversity to likeness and unlikeness by comparison. This comparing moves back and forth between likeness and unlikeness, but that movement is external to the determinations themselves: each is referred only to a third standpoint, not to each other.`
  },
  {
    id: 'div-10-oscillation-comparing',
    title: 'Oscillation of comparing (likeness ↔ unlikeness)',
    text: `The alternation of likeness and unlikeness makes each stand out independently; external reflection is external to itself. Determinate difference is negated absolute difference and has its reflection outside itself, so the moments come apart and refer externally to the immanent reflection confronting them.`
  },
  {
    id: 'div-11-separation-and-destruction',
    title: 'Separation of likeness/unlikeness and their self‑destruction',
    text: `When likeness and unlikeness are kept apart as external viewpoints, the very keeping-apart undermines them: each is meant to be a reference to the other, but their indifference makes them self-referential and so identical with themselves, dissolving real difference.`
  },
  {
    id: 'div-12-negative-unity',
    title: 'Negative unity of the comparing subject',
    text: `The comparing subject that oscillates between likeness and unlikeness is the negative unity of both; by letting one disappear into the other it is the subjective negative that transcends what is compared, and this negative unity becomes the nature of likeness/unlikeness themselves.`
  },
  {
    id: 'div-13-implicit-reflection-return',
    title: 'Implicit reflection and return into negative unity → opposition',
    text: `Likeness and unlikeness are implicitly reflected (self-reference without negation) and thus collapse back into a negative unity. The diverse, through this indifference and implicit reflection, passes into negative reflection: the result is opposition.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'div-op-1-declare-breakup',
    chunkId: 'div-1-overview',
    label: 'Declare identity breaking into diversity',
    clauses: [
      'identity.sublates -> diversityOccurs',
      'diversity = negativeOfSelf ∧ selfReflected'
    ],
    predicates: [{ name: 'IsDiversityFromIdentity', args: [] }],
    relations: [{ predicate: 'emergesFrom', from: 'diversity', to: 'identitySublation' }]
  },
  {
    id: 'div-op-2-substance-formalize',
    chunkId: 'div-2-substance-of-diversity',
    label: 'Formalize diversity as subsistence of the negative with base in identity',
    clauses: [
      'diverse.subsists = true',
      'base(identity) => diverse.identityPreserved',
      'otherness.reflection != otherness.existence'
    ],
    predicates: [{ name: 'IsDiverseSubsistence', args: [] }],
    relations: [{ predicate: 'contrasts', from: 'reflectiveOtherness', to: 'existentialOtherness' }]
  },
  {
    id: 'div-op-3-self-referring-moments',
    chunkId: 'div-3-moments-self-referring',
    label: 'Encode the self‑referring nature of the moments (identity, difference)',
    clauses: [
      'for each moment in {identity,difference}: moment.refersTo = itself',
      'not(determinedWithRespectToOther) => externalDifference'
    ],
    predicates: [{ name: 'HasSelfReferringMoments', args: [] }],
    relations: [{ predicate: 'yields', from: 'selfReferringMoments', to: 'externalityOfDifference' }]
  },
  {
    id: 'div-op-4-externality-behavior',
    chunkId: 'div-4-externality-and-indifference',
    label: 'State behavior: indifference leads to external relation between moments',
    clauses: [
      'moments.indifferentToEachOther = true',
      'conduct(asGeneralMoments) => noInternalDifferentiation',
      'externalDifference => momentsInteractAsDiverse'
    ],
    predicates: [{ name: 'IsExternalDifferenceBehavior', args: [] }],
    relations: [{ predicate: 'explains', from: 'indifference', to: 'diverseConduct' }]
  },
  {
    id: 'div-op-5-positedness',
    chunkId: 'div-5-positedness-and-externality',
    label: 'Formalize positedness and external reflection in diversity',
    clauses: [
      'diversity.reflection = externalInGeneral',
      'difference = positedness ∧ wholeReflection',
      'determinateness(identity|difference) = sublatedPositedness'
    ],
    predicates: [{ name: 'IsPositedDiversity', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'positedness', to: 'diversity' }]
  },
  {
    id: 'div-op-6-duplicity',
    chunkId: 'div-6-duplicity-immanent-negation',
    label: 'Encode duplicity: immanent reflection vs determinateness as negation',
    clauses: [
      'duplicity = {immanentReflection, determinatenessAsNegation}',
      'positedness.refersTo = externalSomething',
      'immanentReflection.implicitSelfReference = true'
    ],
    predicates: [{ name: 'HasDuplicity', args: [] }],
    relations: [{ predicate: 'distinguishes', from: 'immanentReflection', to: 'positedness' }]
  },
  {
    id: 'div-op-7-immanent-vs-external',
    chunkId: 'div-7-immanent-vs-external-reflection',
    label: 'Differentiate immanent and external reflection and their roles',
    clauses: [
      'reflection.inItself = immanentIdentity',
      'externalReflection = determinateDifferenceOrLikeness',
      'identity.asOneReflection => containsDifferenceAsIndifferent'
    ],
    predicates: [{ name: 'DistinguishReflections', args: [] }],
    relations: [{ predicate: 'maps', from: 'immanentReflection', to: 'identityRole' }]
  },
  {
    id: 'div-op-8-likeness-unlikeness',
    chunkId: 'div-8-likeness-and-unlikeness',
    label: 'Formalize likeness/unlikeness as external, standpoint-relative relations',
    clauses: [
      'likeness = positedIdentity (not identityInAndForItself)',
      'unlikeness = positedDifference (not differenceInAndForItself)',
      'relationalTruth(like|unlike) => dependsOn(thirdStandpoint)'
    ],
    predicates: [{ name: 'IsStandpointRelativeRelation', args: ['likeness','unlikeness'] }],
    relations: [{ predicate: 'dependsOn', from: 'likeness/unlikeness', to: 'externalStandpoint' }]
  },
  {
    id: 'div-op-9-external-connect',
    chunkId: 'div-9-external-connecting',
    label: 'Formalize external comparison linking diversity to likeness/unlikeness',
    clauses: [
      'externalReflection.compare(diverse) => {likeness, unlikeness}',
      'compare.operation = oscillation(likeness,unlikeness)',
      'likeness.refersTo = thirdStandpoint',
      'unlikeness.refersTo = thirdStandpoint'
    ],
    predicates: [{ name: 'IsExternalComparison', args: [] }],
    relations: [{ predicate: 'refersTo', from: 'likeness/unlikeness', to: 'thirdStandpoint' }]
  },
  {
    id: 'div-op-10-oscillation',
    chunkId: 'div-10-oscillation-comparing',
    label: 'Encode oscillation and externality of reflection',
    clauses: [
      'oscillation = move(likeness -> unlikeness -> likeness)',
      'oscillation.externalTo(determinations) = true',
      'determinateDifference = negatedAbsoluteDifference'
    ],
    predicates: [{ name: 'IsOscillatoryComparing', args: [] }],
    relations: [{ predicate: 'externalizes', from: 'oscillation', to: 'determinations' }]
  },
  {
    id: 'div-op-11-separation-sublates',
    chunkId: 'div-11-separation-and-destruction',
    label: 'Model separation causing self-referential collapse and sublation',
    clauses: [
      'if likeness.indifferentTo(unlikeness) then likeness -> selfReference',
      'selfReference => lossOfOpposition',
      'separation.subsumes -> sublationOfBoth'
    ],
    predicates: [{ name: 'SeparationCausesCollapse', args: [] }],
    relations: [{ predicate: 'yields', from: 'selfReference', to: 'lossOfOpposition' }]
  },
  {
    id: 'div-op-12-negative-unity-formalize',
    chunkId: 'div-12-negative-unity',
    label: 'Formalize comparing-subject as negative unity that transcends moments',
    clauses: [
      'comparingSubject.oscillates => negativeUnity',
      'negativeUnity.transforms(likeness,unlikeness) => mutualIdentity',
      'negativeUnity.initially = subjectiveOperationOutsideMoments'
    ],
    predicates: [{ name: 'IsNegativeUnity', args: [] }],
    relations: [{ predicate: 'transforms', from: 'negativeUnity', to: 'likeness/unlikeness' }]
  },
  {
    id: 'div-op-13-implicit-reflection-return',
    chunkId: 'div-13-implicit-reflection-return',
    label: 'Encode implicit reflection collapsing into negative unity → opposition',
    clauses: [
      'implicitReflection = selfReferenceWithoutNegation',
      'implicitReflection + indifference => collapseIntoNegativeUnity',
      'result = opposition (diversity as negative unity)'
    ],
    predicates: [{ name: 'IsImplicitReflectionCollapse', args: [] }],
    relations: [{ predicate: 'produces', from: 'implicitReflection', to: 'opposition' }]
  }
)

// ============================================================================
// PART 3: OPPOSITION
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'opp-1-overview',
    title: 'Opposition — overview',
    text: `Opposition completes determinate reflection (difference). It is the unity of identity and diversity: moments are diverse within one identity and thus stand as opposites.`
  },
  {
    id: 'opp-2-positedness-reflection',
    title: 'Positedness reflected into itself',
    text: `The moments of opposition are positedness reflected into itself. Likeness and unlikeness (external reflection) become determinations of opposition when reflected into themselves, each containing the other implicitly.`
  },
  {
    id: 'opp-3-positive-negative-definitions',
    title: 'Positive and negative as determinations',
    text: `Self‑likeness that contains reference to unlikeness is the positive; self‑unlikeness that contains reference to likeness is the negative. Both are positedness reflected in opposite modes (likeness vs unlikeness).`
  },
  {
    id: 'opp-4-mutual-inclusion',
    title: 'Mutual inclusion of opposites',
    text: `Each moment contains its other: positedness reflected into self‑likeness includes unlikeness, and positedness reflected into self‑unlikeness includes likeness. This mutual inclusion is essential to the determination of opposition.`
  },
  {
    id: 'opp-5-reflection-as-negation-of-negation',
    title: 'Negation as negation and immanent reflection',
    text: `What is reflected is positedness — negation as negation — so immanent reflection has reference to the other as its determination. Opposition thus rests on negation that refers to and includes its other.`
  },
  {
    id: 'opp-6-self-subsisting-sides',
    title: 'Self‑subsisting sides of opposition',
    text: `The positive and the negative become self‑subsisting because they are reflections of the whole into itself; as such the opposition they constitute is implicitly determinate. Each side is itself and its other: determinateness now lies within, not in an external other.`
  },
  {
    id: 'opp-7-mutual-dependence-and-positedness',
    title: 'Mutual dependence: each exists by virtue of the other',
    text: `The positive and negative are absolute moments of opposition; each is what it is by virtue of the other's non‑being. They are opposites in the simple sense: each is negative relative to the other and is posited only through that relation — posited moments of one mediation.`
  },
  {
    id: 'opp-8-indifference-and-interchangeability',
    title: 'Indifference, interchangeability, and reflected positedness',
    text: `As posited beings reflected into themselves, the sides show indifference to their first identity: their determinateness is general and interchangeable. Either side can be taken as positive or negative; the determinateness is not internally specific until further reflection.`
  },
  {
    id: 'opp-9-internalization-and-completion',
    title: 'Internalization: each contains positive and negative within itself',
    text: `The posited reference to the other is taken back into each side: the positive contains the negative within, and the negative contains the positive. Each becomes a self‑subsistent unity in which positedness is sublated — the opposition is completed by internalization of its other.`
  },
  {
    id: 'opp-10-negative-sublated',
    title: 'Negative as sublated positedness',
    text: `The negative is not mere immediate negation but sublated positedness: the negative in and for itself positively rests upon itself. As immanent reflection it negates its reference to the other, excluding the positive from itself.`
  },
  {
    id: 'opp-11-independent-opposites',
    title: 'Independent opposite: negative vs positive',
    text: `The negative stands as an independently existing opposite over against the positive, which is the determination of the sublated opposition. The opposition rests upon itself and the negative excludes the positive as its non-being.`
  },
  {
    id: 'opp-12-in-itselfness-and-abstraction',
    title: 'In‑itselfness of sides; abstraction and determination',
    text: `Positive and negative can be considered in themselves (abstracted from excluding reference). This in‑itselfness is an immediate being/non‑being but remains a moment of opposition: their in‑itself is the form of their immanent reflectedness.`
  },
  {
    id: 'opp-13-exclusion-constitutes-in-itselfness',
    title: 'Exclusion constitutes in‑itselfness and mutual determination',
    text: `To be positive or negative in themselves implies that exclusion-reference to the other constitutes their determination. Their being in and for themselves is thus bound up with the exclusive reference that defines opposition.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'opp-op-1-declare-opposition',
    chunkId: 'opp-1-overview',
    label: 'Declare opposition as unity of identity + diversity',
    clauses: [
      'opposition = unity(identity, diversity)',
      'moments(opposition) = diverseWithinIdentity',
      'opposition.completes = determinateReflection'
    ],
    predicates: [{ name: 'IsOpposition', args: [] }],
    relations: [{ predicate: 'completes', from: 'opposition', to: 'determinateReflection' }]
  },
  {
    id: 'opp-op-2-positedness-reflects',
    chunkId: 'opp-2-positedness-reflection',
    label: 'Formalize positedness reflected into itself (likeness/unlikeness)',
    clauses: [
      'positedness.reflectedIntoItself = true',
      'likeness = positedIdentity',
      'unlikeness = positedDifference'
    ],
    predicates: [{ name: 'IsPositedReflection', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'positedness', to: 'likeness/unlikeness' }]
  },
  {
    id: 'opp-op-3-define-pos-neg',
    chunkId: 'opp-3-positive-negative-definitions',
    label: 'Define positive and negative via internal reference to other',
    clauses: [
      'positive.contains = referenceTo(unlikeness)',
      'negative.contains = referenceTo(likeness)',
      'positive = self-likenessWithOther',
      'negative = self-unlikenessWithOther'
    ],
    predicates: [{ name: 'IsPositiveNegativePair', args: [] }],
    relations: [{ predicate: 'mutuallyIncludes', from: 'positive', to: 'negative' }]
  },
  {
    id: 'opp-op-4-mutual-inclusion-formalize',
    chunkId: 'opp-4-mutual-inclusion',
    label: 'Encode mutual inclusion: each moment contains the other',
    clauses: [
      'for each m in {positive, negative}: m.includes = other(m)',
      'mutualInclusion => whole(m) = true',
      'opposition.definedBy = mutualInclusion'
    ],
    predicates: [{ name: 'HasMutualInclusion', args: [] }],
    relations: [{ predicate: 'grounds', from: 'mutualInclusion', to: 'opposition' }]
  },
  {
    id: 'opp-op-5-negation-as-determination',
    chunkId: 'opp-5-reflection-as-negation-of-negation',
    label: 'Characterize negation-as-negation and its role',
    clauses: [
      'negation.asNegation => positedness',
      'immanentReflection.references = otherAsDetermination',
      'opposition.requires = negationReferentialStructure'
    ],
    predicates: [{ name: 'IsNegationAsNegation', args: [] }],
    relations: [{ predicate: 'enables', from: 'negation', to: 'opposition' }]
  },
  {
    id: 'opp-op-6-self-subsist-formalize',
    chunkId: 'opp-6-self-subsisting-sides',
    label: 'Formalize self-subsistence of positive/negative',
    clauses: [
      'positive.selfSubsists = true',
      'negative.selfSubsists = true',
      'opposition.implicitDeterminacy = true',
      'side.determinateness.located = within(side)'
    ],
    predicates: [{ name: 'IsSelfSubsistingSide', args: [] }],
    relations: [{ predicate: 'grounds', from: 'selfSubsistence', to: 'implicitDeterminacy' }]
  },
  {
    id: 'opp-op-7-mutual-dependence-formalize',
    chunkId: 'opp-7-mutual-dependence-and-positedness',
    label: 'Encode mutual dependence: each defined by the other\'s non-being',
    clauses: [
      'for each s in {positive,negative}: s.isByVirtueOf(other.nonBeing)',
      'both = oneMediationOfOpposition',
      'each.isNegativeWithRespectTo(other)'
    ],
    predicates: [{ name: 'IsMutualDependence', args: [] }],
    relations: [{ predicate: 'dependsOn', from: 'side', to: 'other.nonBeing' }]
  },
  {
    id: 'opp-op-8-interchangeability',
    chunkId: 'opp-8-indifference-and-interchangeability',
    label: 'Capture indifference and interchangeability of determinateness',
    clauses: [
      'positedness.reflected => sides.indifferentToFirstIdentity',
      'determinateness = general (canInterchangePositiveNegative)',
      'noInternalSpecificity => interchangeability = true'
    ],
    predicates: [{ name: 'IsInterchangeableDeterminateness', args: [] }],
    relations: [{ predicate: 'permits', from: 'interchangeability', to: 'roleSwap(positive,negative)' }]
  },
  {
    id: 'opp-op-9-internalization',
    chunkId: 'opp-9-internalization-and-completion',
    label: 'Formalize internalization: each side contains its other and sublation completes opposition',
    clauses: [
      'positive.includes = negative (internally)',
      'negative.includes = positive (internally)',
      'positedness.sublated => side.positedBeing = sublatedBeing',
      'result = opposition.completedByInternalization'
    ],
    predicates: [{ name: 'IsInternalizedOpposition', args: [] }],
    relations: [{ predicate: 'sublates', from: 'positedness', to: 'completedOpposition' }]
  },
  {
    id: 'opp-op-10-negative-sublation',
    chunkId: 'opp-10-negative-sublated',
    label: 'Formalize negative as sublated positedness and exclusion',
    clauses: [
      'negative = sublatedPositedness',
      'negative.restsUponItself = true',
      'immanentReflection(negative) => excludes(positive)'
    ],
    predicates: [{ name: 'IsSublatedNegative', args: [] }],
    relations: [{ predicate: 'excludes', from: 'negative', to: 'positive' }]
  },
  {
    id: 'opp-op-11-independent-opposites',
    chunkId: 'opp-11-independent-opposites',
    label: 'Encode independent opposite relation and opposition self‑grounding',
    clauses: [
      'negative.independentExistence = true',
      'positive = determinationOfSublatedOpposition',
      'opposition.selfGrounded = true'
    ],
    predicates: [{ name: 'IsIndependentOppositePair', args: [] }],
    relations: [{ predicate: 'grounds', from: 'opposition', to: 'selfGrounding' }]
  },
  {
    id: 'opp-op-12-in-itselfness',
    chunkId: 'opp-12-in-itselfness-and-abstraction',
    label: 'Capture in‑itself abstraction and its status as a moment',
    clauses: [
      'inItself(positive|negative) = abstractImmediateBeingOrNonBeing',
      'inItself.form = immanentReflectedness',
      'inItselfness != removalOfOpposition (it remains a moment)'
    ],
    predicates: [{ name: 'IsInItselfMoment', args: [] }],
    relations: [{ predicate: 'qualifies', from: 'inItselfness', to: 'immanentReflectedness' }]
  },
  {
    id: 'opp-op-13-exclusion-constitutes',
    chunkId: 'opp-13-exclusion-constitutes-in-itselfness',
    label: 'Formalize that exclusive reference constitutes determination in and for itself',
    clauses: [
      'toBePositiveInItself => notMerelyContrastWithOther',
      'exclusiveReferenceToOther => constitutes(inItselfness)',
      'inAndForItself = determinationByExclusion'
    ],
    predicates: [{ name: 'ExclusionConstitutesDetermination', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'exclusiveReference', to: 'inAndForItself' }]
  }
)
