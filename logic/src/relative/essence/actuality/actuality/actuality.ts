import type { Chunk, LogicalOperation } from '../../../types';

/**
 * CHAPTER 2: ACTUALITY — Complete Structure
 * 
 * NOTE: This is Chapter 2 of Section III Actuality. Actuality is a Species
 * of the Genus Actuality. Each Chapter (Species) has three sub-species that make up
 * its Concept:
 * 
 * Structure:
 * Introduction: Actuality as reflected absoluteness, three moments
 * A. CONTINGENCY OR FORMAL ACTUALITY, POSSIBILITY, AND NECESSITY (Species 1)
 * B. RELATIVE NECESSITY OR REAL ACTUALITY, POSSIBILITY, AND NECESSITY (Species 2)
 * C. ABSOLUTE NECESSITY (Species 3)
 * 
 * Transition: Absolute necessity passes over to substance (absolute relation)
 */

// ============================================================================
// INTRODUCTION: CHAPTER 2 — ACTUALITY
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'act-ch2-intro-1-absolute-as-mode',
    title: 'Absolute as mode — reflected absoluteness',
    text: `The absolute is the unity of inner and outer
as a first implicitly existent unit.
The exposition appeared as an external reflection
which, for its part, has the immediate
as something it has found,
but it equally is its movement
and the reference connecting it to the absolute
and, as such, it leads it back to the latter,
determining it as a mere "way and manner."
But this "way and manner" is the
determination of the absolute itself,
namely its first identity
or its mere implicitly existent unity.
And through this reflection, not only is
that first in-itself posited as essenceless determination,
but, since the reflection is negative self-reference,
it is through it that the in-itself becomes
a mode in the first place.
It is this reflection that,
in sublating itself in its determinations
and as a movement which as such turns back upon itself,
is first truly absolute identity
and, at the same time, the determining of
the absolute or its modality.
The mode, therefore, is the externality of the absolute,
but equally so only its reflection into itself;
or again, it is the absolute's own manifestation,
so that this externalization is its immanent reflection
and therefore its being in-and-for-itself.

So, as the manifestation that it is nothing,
that it has no content, save to be
the manifestation of itself,
the absolute is absolute form.
Actuality is to be taken as
this reflected absoluteness.`,
    summary: 'Absolute = unity of inner and outer (first implicitly existent unit). Exposition = external reflection, movement, reference to absolute, determines as "way and manner" = determination of absolute itself (first identity). Reflection = negative self-reference, in-itself becomes mode. Reflection sublating itself = truly absolute identity, determining of absolute/modality. Mode = externality of absolute, reflection into itself, absolute\'s own manifestation, externalization = immanent reflection = being in-and-for-itself. Absolute = absolute form (manifestation of itself, no content). Actuality = reflected absoluteness.'
  },
  {
    id: 'act-ch2-intro-2-actuality-higher-than-being-existence',
    title: 'Actuality higher than being and concrete existence — manifestation',
    text: `Being is not yet actual;
it is the first immediacy;
its reflection is therefore becoming
and transition into an other;
or its immediacy is not being-in-and-for-itself.
Actuality also stands higher than concrete existence.
It is true that the latter is the immediacy
that has proceeded from ground and conditions,
or from essence and its reflection.
In itself or implicitly, it is therefore
what actuality is, real reflection;
but it is still not the posited unity of reflection and immediacy.
Hence concrete existence passes over into appearance
as it develops the reflection contained within it.
It is the ground that has foundered to the ground;
its determination, its vocation, is to restore this ground,
and therefore it becomes essential relation,
and its final reflection is that its
immediacy be posited as immanent reflection and conversely.
This unity, in which concrete existence
or immediacy and the in-itself,
the ground or the reflected, are simply moments,
is now actuality.
The actual is therefore manifestation.
It is not drawn into
the sphere of alteration by its externality,
nor is it the reflective shining of itself in an other.
It just manifests itself,
and this means that in its externality,
and only in it, it is itself, that is to say,
only as a self-differentiating and self-determining movement.`,
    summary: 'Being not yet actual (first immediacy, becoming, transition, not being-in-and-for-itself). Actuality higher than concrete existence. Concrete existence = immediacy from ground/conditions/essence/reflection, implicitly = real reflection, but not posited unity. Passes over into appearance, becomes essential relation, immediacy posited as immanent reflection. Unity (concrete existence/immediacy and in-itself/ground/reflected as moments) = actuality. Actual = manifestation (not drawn into alteration, not reflective shining in other). Manifests itself: in externality, only in it, itself = self-differentiating and self-determining movement.'
  },
  {
    id: 'act-ch2-intro-3-three-moments',
    title: 'Three moments — contingency, real actuality, absolute necessity',
    text: `Now in actuality as this absolute form,
the moments only are as sublated or formal, not yet realized;
their differentiation thus belongs at first to external reflection
and is not determined as content.

Actuality, as itself immediate form-unity of inner and outer,
is thus in the determination of immediacy
as against the determination of immanent reflection;
or it is an actuality as against a possibility.
The connection of the two to each other is the third,
the actual determined both as being reflected into itself
and as this being immediately existing.
This third is necessity.

But first, since the actual and the possible
are formal distinctions,
their connection is likewise only formal,
and consists only in this,
that the one just like the other
is a positedness, or in contingency.

Second, because in contingency
the actual as well as the possible
are a positedness,
because they have retained their determination,
real actuality now arises,
and with it also real possibility
and relative necessity.

Third, the reflection of relative necessity
into itself yields absolute necessity,
which is absolute possibility and actuality.`,
    summary: 'In actuality as absolute form: moments = sublated/formal, not realized; differentiation = external reflection, not determined as content. Actuality = immediate form-unity of inner and outer, determination of immediacy vs immanent reflection = actuality vs possibility. Connection = third = necessity (reflected into itself and immediately existing). First: formal distinctions = formal connection = contingency. Second: in contingency, actual/possible = positedness, retained determination = real actuality, real possibility, relative necessity. Third: reflection of relative necessity into itself = absolute necessity (absolute possibility and actuality).'
  },
];

// ============================================================================
// INTRODUCTION OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'act-ch2-op-intro-1-absolute-as-mode',
    chunkId: 'act-ch2-intro-1-absolute-as-mode',
    label: 'Absolute as mode — reflected absoluteness',
    clauses: [
      'absolute = unityOfInnerAndOuter',
      'absolute.as = firstImplicitlyExistentUnit',
      'exposition = externalReflection',
      'exposition.has = immediate',
      'exposition.is = movement',
      'exposition.reference = connectingToAbsolute',
      'exposition.leadsBack = toAbsolute',
      'exposition.determinesAs = "wayAndManner"',
      '"wayAndManner" = determinationOfAbsoluteItself',
      '"wayAndManner" = firstIdentity',
      '"wayAndManner" = implicitlyExistentUnity',
      'reflection = negativeSelfReference',
      'inItself.becomes = mode',
      'reflection.sublatingItself = trulyAbsoluteIdentity',
      'reflection = determiningOfAbsolute',
      'reflection = modality',
      'mode = externalityOfAbsolute',
      'mode = reflectionIntoItself',
      'mode = absolutesOwnManifestation',
      'externalization = immanentReflection',
      'externalization = beingInAndForItself',
      'absolute = absoluteForm',
      'absoluteForm.manifestation = ofItself',
      'absoluteForm.hasNoContent = true',
      'actuality = reflectedAbsoluteness',
    ],
    predicates: [
      { name: 'IsReflectedAbsoluteness', args: ['actuality'] },
    ],
    relations: [
      { predicate: 'is', from: 'actuality', to: 'reflectedAbsoluteness' },
    ],
  },
  {
    id: 'act-ch2-op-intro-2-actuality-higher-than-being-existence',
    chunkId: 'act-ch2-intro-2-actuality-higher-than-being-existence',
    label: 'Actuality higher than being and concrete existence — manifestation',
    clauses: [
      'being.notYet = actual',
      'being = firstImmediacy',
      'being.reflection = becoming',
      'being.reflection = transitionIntoOther',
      'being.immediacy.not = beingInAndForItself',
      'actuality.standsHigherThan = concreteExistence',
      'concreteExistence = immediacyFromGround',
      'concreteExistence.implicitly = realReflection',
      'concreteExistence.not = positedUnity',
      'concreteExistence.passesOverInto = appearance',
      'concreteExistence.becomes = essentialRelation',
      'unity = actuality',
      'unity.moments = {concreteExistence, inItself, ground, reflected}',
      'actual = manifestation',
      'actual.notDrawnInto = sphereOfAlteration',
      'actual.not = reflectiveShiningInOther',
      'actual.manifestsItself = true',
      'inExternality.actual = itself',
      'actual = selfDifferentiatingMovement',
      'actual = selfDeterminingMovement',
    ],
    predicates: [
      { name: 'IsManifestation', args: ['actual'] },
    ],
    relations: [
      { predicate: 'is', from: 'actual', to: 'manifestation' },
    ],
  },
  {
    id: 'act-ch2-op-intro-3-three-moments',
    chunkId: 'act-ch2-intro-3-three-moments',
    label: 'Three moments — contingency, real actuality, absolute necessity',
    clauses: [
      'inActuality.moments = {sublated, formal}',
      'moments.notYet = realized',
      'differentiation.belongsTo = externalReflection',
      'differentiation.notDeterminedAs = content',
      'actuality = immediateFormUnity',
      'actuality.determination = immediacy',
      'actuality.asAgainst = immanentReflection',
      'actuality.asAgainst = possibility',
      'connection = third',
      'third = necessity',
      'necessity.determinedAs = {reflectedIntoItself, immediatelyExisting}',
      'first = formalDistinctions',
      'formalDistinctions.connection = formal',
      'formalConnection = contingency',
      'second = inContingency',
      'inContingency.actualPossible = positedness',
      'inContingency.retainedDetermination = true',
      'arises = realActuality',
      'arises = realPossibility',
      'arises = relativeNecessity',
      'third = reflectionOfRelativeNecessity',
      'reflectionOfRelativeNecessity.intoItself = absoluteNecessity',
      'absoluteNecessity = {absolutePossibility, absoluteActuality}',
    ],
    predicates: [
      { name: 'HasThreeMoments', args: ['actuality'] },
    ],
    relations: [
      { predicate: 'has', from: 'actuality', to: 'threeMoments' },
    ],
  },
];

// Note: For aggregated exports, use index.ts
// Individual species modules are available for granular access

