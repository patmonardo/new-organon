/**
 * Logical Operations: The Universal Concept
 *
 * The universal concept is the absolutely infinite, unconditioned and free.
 * It is the pure self-reference of the concept, the negation of negation,
 * and the creative principle that differentiates itself while remaining itself.
 *
 * Dialectical Movement:
 * - Genesis: Being → Essence → Concept via self-repulsion
 * - Universality as negation of negation
 * - Universal as simple yet rich, creative power
 * - Transition to particularity
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE UNIVERSAL CONCEPT
// ============================================================================

export const univOp1GenesisAbsoluteness: LogicalOperation = {
  id: 'univ-op-1-genesis-absoluteness',
  chunkId: 'univ-1-genesis-absoluteness',
  label: 'Pure Concept: Absolutely Infinite, Unconditioned; Genesis via Being → Essence → Concept',
  clauses: [
    'pureConcept = absolutelyInfinite',
    'pureConcept = unconditioned',
    'pureConcept = free',
    'genesis = being → essence → concept',
    'genesis = selfRepulsion',
    'becoming = selfRepulsion',
    'becoming = unconditional',
    'becoming = originative',
    'being → essence = reflectiveShine',
    'being → essence = positedness',
    'becoming = positing',
    'essence → being = positingSublated',
    'essence → being = restoredToOriginalBeing',
    'concept = mutualPenetration(moments)',
    'qualitativeExistent = positing + immanentTurningBack',
    'pureImmanentReflection = becomingOther',
    'pureImmanentReflection = determinateness',
    'determinateness = infinite',
    'determinateness = selfReferring',
  ],
  predicates: [
    { name: 'absolutelyInfinite', args: ['pureConcept'] },
    { name: 'unconditioned', args: ['pureConcept'] },
    { name: 'free', args: ['pureConcept'] },
    { name: 'selfRepulsion', args: ['genesis'] },
    { name: 'mutualPenetration', args: ['concept', 'moments'] },
    { name: 'infinite', args: ['determinateness'] },
    { name: 'selfReferring', args: ['determinateness'] },
  ],
  relations: [
    { predicate: 'is', from: 'pureConcept', to: 'absolutelyInfinite' },
    { predicate: 'genesis', from: 'being', to: 'essence' },
    { predicate: 'genesis', from: 'essence', to: 'concept' },
    { predicate: 'is', from: 'concept', to: 'mutualPenetration' },
  ],
  candidateSummary: 'The pure concept is absolutely infinite, unconditioned, and free. Its genesis proceeds from Being through Essence to Concept via self-repulsion, where becoming is the unconditional and originative. Being passes into Essence as reflective shine or positedness, and becoming becomes positing. Conversely, the positing of essence sublates itself and restores itself to original being. The concept is the mutual penetration of these moments, where the qualitative existent is only as positing and immanent turning back, and this pure immanent reflection is the becoming-other or determinateness, which is infinite and self-referring.',
  provenance: {
    sourceChunk: 'univ-1-genesis-absoluteness',
    sourceOp: 'univ-op-1-genesis-absoluteness',
  },
};

export const univOp2UniversalityNegationOfNegation: LogicalOperation = {
  id: 'univ-op-2-universality-negation-of-negation',
  chunkId: 'univ-2-universality-negation-of-negation',
  label: 'Concept Self-Identity; Universality as Negation of Negation',
  clauses: [
    'concept = absoluteSelfIdentity',
    'concept = negationOfNegation',
    'concept = infiniteUnity(negativity, itself)',
    'universality = pureSelfReference',
    'universality = viaNegativity',
    'universality = simplestDetermination',
    'universality = simple',
    'universality.contains = difference',
    'universality.contains = determinateness',
    'universality.contains = viaAbsoluteNegativity',
  ],
  predicates: [
    { name: 'absoluteSelfIdentity', args: ['concept'] },
    { name: 'negationOfNegation', args: ['concept'] },
    { name: 'infiniteUnity', args: ['negativity', 'itself'] },
    { name: 'pureSelfReference', args: ['universality'] },
    { name: 'simplestDetermination', args: ['universality'] },
  ],
  relations: [
    { predicate: 'is', from: 'concept', to: 'absoluteSelfIdentity' },
    { predicate: 'is', from: 'concept', to: 'negationOfNegation' },
    { predicate: 'is', from: 'universality', to: 'pureSelfReference' },
    { predicate: 'contains', from: 'universality', to: 'difference' },
    { predicate: 'contains', from: 'universality', to: 'determinateness' },
  ],
  candidateSummary: 'The concept is absolute self-identity by being the negation of negation or the infinite unity of negativity with itself. This pure self-reference of the concept, which is such by positing itself through negativity, is the universality of the concept. Universality seems incapable of explanation because it is the simplest determination, but it is precisely of the nature of the universal to be a simple that, by virtue of absolute negativity, contains difference and determinateness in itself in the highest degree.',
  provenance: {
    sourceChunk: 'univ-2-universality-negation-of-negation',
    sourceOp: 'univ-op-2-universality-negation-of-negation',
  },
};

export const univOp3ContrastBeingRichness: LogicalOperation = {
  id: 'univ-op-3-contrast-being-richness',
  chunkId: 'univ-3-contrast-being-richness',
  label: 'Contrast with Being; Universal as Simple Yet Rich, Identity as Absolute Mediation',
  clauses: [
    'being = simple',
    'being = immediate',
    'being.oneWith = nonBeing',
    'being → becoming',
    'conceptOfBeing = soSimpleItVanishes',
    'universal = simple',
    'universal = rich',
    'universal.richness = becauseConcept',
    'first = simpleSelfReference',
    'first = onlyInItself',
    'second = identity',
    'identity = absoluteMediation',
    'identity != anythingMediated',
  ],
  predicates: [
    { name: 'simple', args: ['being'] },
    { name: 'immediate', args: ['being'] },
    { name: 'simpleYetRich', args: ['universal'] },
    { name: 'absoluteMediation', args: ['identity'] },
  ],
  relations: [
    { predicate: 'is', from: 'being', to: 'simple' },
    { predicate: 'is', from: 'being', to: 'immediate' },
    { predicate: 'is', from: 'universal', to: 'simpleYetRich' },
    { predicate: 'is', from: 'identity', to: 'absoluteMediation' },
  ],
  candidateSummary: 'Being is simple as immediate, and we can only intend it without being able to say what it is. Being is immediately one with its other, non-being, and thus becomes becoming. The concept of being is so simple it vanishes into its opposite immediately. The universal, by contrast, is simple yet all the richer in itself because it is the concept. First, it is simple self-reference (only in itself). Second, identity is absolute mediation, but not anything mediated.',
  provenance: {
    sourceChunk: 'univ-3-contrast-being-richness',
    sourceOp: 'univ-op-3-contrast-being-richness',
  },
};

export const univOp4AbstractUniversalExternality: LogicalOperation = {
  id: 'univ-op-4-abstract-universal-externality',
  chunkId: 'univ-4-abstract-universal-externality',
  label: 'Abstract Universal, Double Negation, and Not-Yet Externality',
  clauses: [
    'abstractUniversal = opposedTo(particular, singular)',
    'abstractUniversal.requires = leavingAside(otherDeterminations)',
    'determinations = negations',
    'leavingAside = furtherNegating',
    'abstractUniversal.contains = negationOfNegation',
    'doubleNegation.misrepresented = asExternal',
    'universal.hasNotYet = determinationOfExternality',
    'universal = absoluteNegation',
    'universal = negationOfNegation',
    'universal = absoluteNegativity',
  ],
  predicates: [
    { name: 'abstractUniversal', args: ['universal'] },
    { name: 'negationOfNegation', args: ['abstractUniversal.contains'] },
    { name: 'absoluteNegativity', args: ['universal'] },
  ],
  relations: [
    { predicate: 'is', from: 'abstractUniversal', to: 'opposedTo' },
    { predicate: 'contains', from: 'abstractUniversal', to: 'negationOfNegation' },
    { predicate: 'is', from: 'universal', to: 'absoluteNegativity' },
  ],
  candidateSummary: 'The abstract universal, opposed to particular and singular, requires leaving aside other determinations of the concrete. Determinations are negations, and leaving aside is further negating. The abstract universal already contains negation of negation. Double negation is misrepresented as external: properties left out vs retained seem different, and the operation of leaving aside seems external. But the universal has not yet acquired the determination of externality. The universal is still absolute negation, negation of negation, absolute negativity.',
  provenance: {
    sourceChunk: 'univ-4-abstract-universal-externality',
    sourceOp: 'univ-op-4-abstract-universal-externality',
  },
};

export const univOp5PersistenceDetermination: LogicalOperation = {
  id: 'univ-op-5-persistence-determination',
  chunkId: 'univ-5-persistence-determination',
  label: 'Universal Maintains Itself in Its Determination; Persistence vs Qualitative Perishing',
  clauses: [
    'firstNegative = determination',
    'firstNegative != restrictionFor(universal)',
    'universal.maintainsItself = inDetermination',
    'selfIdentity = positive',
    'categoriesOfBeing = identitiesInRestriction',
    'categoriesOfBeing = identitiesInOtherness',
    'categoriesOfBeing = onlyImplicitlyConcept',
    'qualitativeDetermination.perished = inOther',
    'truth = diverseDetermination',
    'universal.whenPositing = remainsWhatItIs',
    'universal = soulOfConcrete',
    'universal = unhindered',
    'universal = equalToItself(manifoldness, diversity)',
    'universal.notSweptAway = inBecoming',
    'universal.persists = undisturbed',
    'universal = powerOfUnalterable',
    'universal = powerOfUndying',
    'universal = powerOfSelfPreservation',
  ],
  predicates: [
    { name: 'maintainsItself', args: ['universal', 'inDetermination'] },
    { name: 'soulOfConcrete', args: ['universal'] },
    { name: 'persists', args: ['universal', 'undisturbed'] },
    { name: 'powerOfSelfPreservation', args: ['universal'] },
  ],
  relations: [
    { predicate: 'maintainsItself', from: 'universal', to: 'inDetermination' },
    { predicate: 'is', from: 'universal', to: 'soulOfConcrete' },
    { predicate: 'persists', from: 'universal', to: 'undisturbed' },
  ],
  candidateSummary: 'The first negative (determination) is not a restriction for the universal. The universal maintains itself in its determination; self-identity is positive. Categories of being are identities in restriction and otherness, only implicitly concept, not manifest. Qualitative determination perished in its other; truth is diverse determination. The universal, when positing itself in determination, remains what it is. The universal is the soul of the concrete, inhabiting it, unhindered, equal to itself in manifoldness and diversity. It is not swept away in becoming but persists undisturbed. It is the power of unalterable, undying self-preservation.',
  provenance: {
    sourceChunk: 'univ-5-persistence-determination',
    sourceOp: 'univ-op-5-persistence-determination',
  },
};

export const univOp6EssenceCreativePrinciple: LogicalOperation = {
  id: 'univ-op-6-essence-creative-principle',
  chunkId: 'univ-6-essence-creative-principle',
  label: 'Not Mere Reflective Shine; Universal as Essence and Creative Principle',
  clauses: [
    'universal != mereReflectiveShine',
    'determinationOfReflection = relative',
    'determinationOfReflection = relating',
    'determinationOfReflection = shinesInOther',
    'universal = essenceOf(determination)',
    'universal = positiveNature(determination)',
    'determination = negativeOf(universal)',
    'determination = positedness',
    'determination = negativeOfNegative',
    'determination = selfIdentityOfNegative',
    'universal = substanceOf(determinations)',
    'accident = conceptSelfMediation',
    'accident = immanentReflection',
    'mediation.raises = accidentalToNecessity',
    'mediation = manifestedReference',
    'concept != abyssOfFormlessSubstance',
    'concept != innerIdentity(differentThings)',
    'concept = absoluteNegativity',
    'concept = informingPrinciple',
    'concept = creativePrinciple',
    'determination = notLimitation',
    'determination = sublatedAsDetermination',
    'determination = positedness',
    'reflectiveShine = appearanceAsAppearance(identical)',
  ],
  predicates: [
    { name: 'essenceOf', args: ['universal', 'determination'] },
    { name: 'substanceOf', args: ['universal', 'determinations'] },
    { name: 'absoluteNegativity', args: ['concept'] },
    { name: 'creativePrinciple', args: ['concept'] },
  ],
  relations: [
    { predicate: 'is', from: 'universal', to: 'essenceOf' },
    { predicate: 'is', from: 'universal', to: 'substanceOf' },
    { predicate: 'is', from: 'concept', to: 'absoluteNegativity' },
    { predicate: 'is', from: 'concept', to: 'creativePrinciple' },
  ],
  candidateSummary: 'The universal does not simply shine reflectively like determination of reflection. Determination of reflection is relative, relating, shines in other, external activity alongside self-subsistence. The universal is the essence of its determination, the determination\'s own positive nature. Determination (negative of universal) is in the concept simply positedness. Determination is negative of negative, self-identity of negative (which is universal). The universal is the substance of determinations. What for substance was accident is the concept\'s own self-mediation (immanent reflection). Mediation raises accidental to necessity, manifested reference. The concept is not the abyss of formless substance nor inner identity of different things. The concept is absolute negativity, the informing and creative principle. Determination is not limitation but sublated as determination, positedness. Reflective shine is appearance as appearance of identical.',
  provenance: {
    sourceChunk: 'univ-6-essence-creative-principle',
    sourceOp: 'univ-op-6-essence-creative-principle',
  },
};

export const univOp7FreePowerLove: LogicalOperation = {
  id: 'univ-op-7-free-power-love',
  chunkId: 'univ-7-free-power-love',
  label: 'Universal as Free Power/Love; Rest in Its Other',
  clauses: [
    'universal = freePower',
    'universal.itself = whileReachingOutToOther',
    'universal.embraces = other',
    'universal.without = violence',
    'universal.atRest = inOtherAsInOwn',
    'universal = freeLove',
    'universal = boundlessBlessedness',
    'universal.relates = toDistinctAsToItself',
    'universal.inDistinct = returnedToItself',
  ],
  predicates: [
    { name: 'freePower', args: ['universal'] },
    { name: 'freeLove', args: ['universal'] },
    { name: 'boundlessBlessedness', args: ['universal'] },
  ],
  relations: [
    { predicate: 'is', from: 'universal', to: 'freePower' },
    { predicate: 'is', from: 'universal', to: 'freeLove' },
    { predicate: 'relates', from: 'universal', to: 'toDistinctAsToItself' },
  ],
  candidateSummary: 'The universal is free power. It is itself while reaching out to other, embracing it, without violence. It is at rest in its other as in its own. It is also free love and boundless blessedness. It relates to distinct as to itself; in it, returned to itself.',
  provenance: {
    sourceChunk: 'univ-7-free-power-love',
    sourceOp: 'univ-op-7-free-power-love',
  },
};

export const univOp8DeterminatenessTotality: LogicalOperation = {
  id: 'univ-op-8-determinateness-totality',
  chunkId: 'univ-8-determinateness-totality',
  label: 'Determinateness Within the Universal; Totality vs Abstract Universal',
  clauses: [
    'cannotSpeakOf = universalApartFrom(determinateness)',
    'universal.contains = determinateness',
    'universal.contains = inAndForItself',
    'determinateness.notImported = fromOutside',
    'asNegativity = determinateness = particularity',
    'asSecondUniversal = absoluteDeterminateness = singularity',
    'asSecondUniversal = absoluteDeterminateness = concreteness',
    'universal = totalityOfConcept',
    'universal = concrete',
    'universal != empty',
    'universal.hasContent = byVirtueOfConcept',
    'content = universalOwn',
    'content = immanent',
    'abstractFromContent = abstractUniversal',
    'abstractUniversal = isolated',
    'abstractUniversal = imperfectMoment',
    'abstractUniversal = voidOfTruth',
  ],
  predicates: [
    { name: 'totalityOfConcept', args: ['universal'] },
    { name: 'abstractUniversal', args: ['universal'] },
  ],
  relations: [
    { predicate: 'contains', from: 'universal', to: 'determinateness' },
    { predicate: 'is', from: 'universal', to: 'totalityOfConcept' },
  ],
  candidateSummary: 'We cannot speak of universal apart from determinateness (particularity and singularity). The universal contains determinateness in and for itself (via absolute negativity). Determinateness is not imported from outside. As negativity (first immediate negation), determinateness is particularity. As second universal (negation of negation), absolute determinateness is singularity and concreteness. The universal is the totality of the concept, concrete, not empty, has content by virtue of the concept. Content in which universal preserves itself is universal\'s own, immanent. Abstract from content is abstract universal (isolated, imperfect moment, void of truth).',
  provenance: {
    sourceChunk: 'univ-8-determinateness-totality',
    sourceOp: 'univ-op-8-determinateness-totality',
  },
};

export const univOp9TotalReflectionInwardOutward: LogicalOperation = {
  id: 'univ-op-9-total-reflection-inward-outward',
  chunkId: 'univ-9-total-reflection-inward-outward',
  label: 'Universal as Total Reflection: Outward and Inward Shining; Determinate Concept as Immanent Character',
  clauses: [
    'universal.showsItself = asTotality',
    'universal.showsItself = viaDeterminateness',
    'determinateness = firstNegation + reflectionOfNegationIntoItself',
    'firstNegation = universal = particular',
    'otherDeterminateness = universalStillEssentiallyUniversal',
    'determinatenessInConcept = totalReflection',
    'totalReflection = doublyReflectiveShine',
    'outward = reflectionIntoOther',
    'outward = establishesDistinction',
    'outward = particularityResolvedInHigherUniversality',
    'inward = reflectionIntoItself',
    'relativeUniversal.preserves = universalityInDeterminateness',
    'relativeUniversal = viaInwardShining',
    'determinateness = determinateConcept',
    'determinateConcept = bentBackIntoItself',
    'determinateConcept = conceptOwnImmanentCharacter',
    'character.madeEssential = byTakenUpIntoUniversality',
    'character.pervaded = byUniversality',
    'character.pervades = universality',
    'character = equalInExtension',
    'character = identical',
    'character = genus',
    'determinateness.notSeparated = fromUniversal',
    'character != outwardlyDirectedLimitation',
    'character = positive',
    'character = freeSelfReference(byUniversality)',
    'determinateConcept = infinitelyFreeConcept',
  ],
  predicates: [
    { name: 'totalReflection', args: ['determinatenessInConcept'] },
    { name: 'doublyReflectiveShine', args: ['totalReflection'] },
    { name: 'infinitelyFreeConcept', args: ['determinateConcept'] },
  ],
  relations: [
    { predicate: 'is', from: 'determinatenessInConcept', to: 'totalReflection' },
    { predicate: 'is', from: 'determinateConcept', to: 'infinitelyFreeConcept' },
  ],
  candidateSummary: 'The universal shows itself as totality via determinateness. Determinateness is first negation plus reflection of negation into itself. First negation (by itself) makes universal particular (to be considered). Other determinateness makes universal still essentially universal. Determinateness in concept is total reflection, doubly reflective shine: outward (reflection into other, establishes distinction, particularity resolved in higher universality) and inward (reflection into itself). Relative universal preserves universality in determinateness (not indifferent, but via inward shining). Determinateness as determinate concept is bent back into itself, the concept\'s own immanent character. Character made essential by taken up into universality, pervaded by it (and pervades it). Equal in extension and identical. Character is genus (determinateness not separated from universal). Not outwardly directed limitation, but positive (free self-reference by universality). Determinate concept is infinitely free concept.',
  provenance: {
    sourceChunk: 'univ-9-total-reflection-inward-outward',
    sourceOp: 'univ-op-9-total-reflection-inward-outward',
  },
};

export const univOp10HigherUniversalConcretes: LogicalOperation = {
  id: 'univ-op-10-higher-universal-concretes',
  chunkId: 'univ-10-higher-universal-concretes',
  label: 'Higher Universal: Outward Turned Inward; Life, I, Spirit as Concretes; Idea of Infinite Spirit',
  clauses: [
    'otherSide = genusLimitedBy(determinateCharacter)',
    'genusLimitedBy = lowerGenus',
    'lowerGenus.resolution = inHigherUniversal',
    'higherUniversal = moreAbstractGenus',
    'higherUniversal = pertainsTo(outwardlyDirectedSide)',
    'trulyHigherUniversal = outwardlyDirectedSideRedirectedInwardly',
    'trulyHigherUniversal = secondNegation',
    'determinateness = positedness',
    'determinateness = reflectiveShine',
    'life = notUniversalOnlyAsHigherGenus',
    'life = concrete',
    'i = notUniversalOnlyAsHigherGenus',
    'i = concrete',
    'spirit = notUniversalOnlyAsHigherGenus',
    'spirit = concrete',
    'absoluteConcept = notUniversalOnlyAsHigherGenus',
    'absoluteConcept = concrete',
    'determinacies.notMereSpecies = lowerGenera',
    'determinacies = selfContained',
    'determinacies = selfComplete',
    'life = determinateConcept',
    'i = determinateConcept',
    'finiteSpirit = determinateConcept',
    'resolution = universalAsTrulyAbsoluteConcept',
    'resolution = ideaOfInfiniteSpirit',
    'infiniteSpirit = positedBeing',
    'infiniteSpirit = infinite',
    'infiniteSpirit = transparentReality',
    'infiniteSpirit.contemplates = itsCreation',
    'infiniteSpirit.inCreation = contemplatesItself',
  ],
  predicates: [
    { name: 'trulyHigherUniversal', args: ['universal'] },
    { name: 'concrete', args: ['life', 'i', 'spirit'] },
    { name: 'ideaOfInfiniteSpirit', args: ['resolution'] },
  ],
  relations: [
    { predicate: 'is', from: 'trulyHigherUniversal', to: 'secondNegation' },
    { predicate: 'is', from: 'resolution', to: 'ideaOfInfiniteSpirit' },
  ],
  candidateSummary: 'The other side: genus limited by determinate character is lower genus, resolution in higher universal. Higher universal can be grasped as more abstract genus (pertains to outwardly directed side). Truly higher universal is outwardly directed side redirected inwardly (second negation). Determinateness is positedness, reflective shine. Life, I, spirit, absolute concept are not universals only as higher genera. They are concretes whose determinacies are not mere species or lower genera. Determinacies in reality are self-contained and self-complete. Life, I, finite spirit are also determinate concepts. Their resolution is universal as truly absolute concept, the idea of infinite spirit. Infinite spirit is posited being, infinite, transparent reality. It contemplates its creation and, in creation, itself.',
  provenance: {
    sourceChunk: 'univ-10-higher-universal-concretes',
    sourceOp: 'univ-op-10-higher-universal-concretes',
  },
};

export const univOp11CreativeDifferentiation: LogicalOperation = {
  id: 'univ-op-11-creative-differentiation',
  chunkId: 'univ-11-creative-differentiation',
  label: 'True Infinite Universal as Particularity: Creative Self-Differentiation; Universal Differences',
  clauses: [
    'trueInfiniteUniversal = immediatelyInItself',
    'trueInfiniteUniversal = particularity',
    'trueInfiniteUniversal = singularity',
    'nowExamined = asParticularity',
    'determinesItself = freely',
    'processOfBecomingFinite != transition',
    'processOfBecomingFinite = creativePower',
    'processOfBecomingFinite = selfReferringAbsoluteNegativity',
    'differentiatesItself = internally',
    'differentiating = oneWithUniversality',
    'positsDifferences = thatAreThemselvesUniversals',
    'differences = selfReferring',
    'differences.become = fixed',
    'differences.become = isolated',
  ],
  predicates: [
    { name: 'creativePower', args: ['processOfBecomingFinite'] },
    { name: 'selfReferringAbsoluteNegativity', args: ['processOfBecomingFinite'] },
  ],
  relations: [
    { predicate: 'is', from: 'trueInfiniteUniversal', to: 'particularity' },
    { predicate: 'differentiatesItself', from: 'universal', to: 'internally' },
  ],
  candidateSummary: 'The true infinite universal is immediately in itself particularity and singularity. Now examined as particularity, it determines itself freely. The process of becoming finite is not transition (sphere of being). It is creative power as self-referring absolute negativity. It differentiates itself internally, determining (differentiating one with universality). It posits differences that are themselves universals (self-referring). Differences become fixed, isolated.',
  provenance: {
    sourceChunk: 'univ-11-creative-differentiation',
    sourceOp: 'univ-op-11-creative-differentiation',
  },
};

export const univOp12FiniteUniversality: LogicalOperation = {
  id: 'univ-op-12-finite-universality',
  chunkId: 'univ-12-finite-universality',
  label: 'Finite Subsistence as Universality; The Concept\'s Creativity',
  clauses: [
    'isolatedSubsistenceOfFinite = earlierDetermined',
    'isolatedSubsistenceOfFinite = beingForItself',
    'isolatedSubsistenceOfFinite = thinghood',
    'isolatedSubsistenceOfFinite = substance',
    'inTruth = universality',
    'form = withWhichInfiniteConceptClothes(differences)',
    'form = equallyItselfOneOf(differences)',
    'this = creativityOfConcept',
    'toBeComprehended = onlyInConceptInnermostCore',
  ],
  predicates: [
    { name: 'creativityOfConcept', args: ['this'] },
  ],
  relations: [
    { predicate: 'is', from: 'isolatedSubsistenceOfFinite', to: 'universality' },
    { predicate: 'is', from: 'this', to: 'creativityOfConcept' },
  ],
  candidateSummary: 'The isolated subsistence of finite was earlier determined as being-for-itself, thinghood, substance. In truth, it is universality. It is the form with which infinite concept clothes its differences. The form is equally itself one of its differences. This is the creativity of the concept. To be comprehended only in the concept\'s innermost core.',
  provenance: {
    sourceChunk: 'univ-12-finite-universality',
    sourceOp: 'univ-op-12-finite-universality',
  },
};

export const universalConceptOperations: LogicalOperation[] = [
  univOp1GenesisAbsoluteness,
  univOp2UniversalityNegationOfNegation,
  univOp3ContrastBeingRichness,
  univOp4AbstractUniversalExternality,
  univOp5PersistenceDetermination,
  univOp6EssenceCreativePrinciple,
  univOp7FreePowerLove,
  univOp8DeterminatenessTotality,
  univOp9TotalReflectionInwardOutward,
  univOp10HigherUniversalConcretes,
  univOp11CreativeDifferentiation,
  univOp12FiniteUniversality,
];

