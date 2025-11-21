import type { Chunk, LogicalOperation } from '../../../types';

/**
 * CHAPTER 3: THE ABSOLUTE RELATION — Complete Structure
 * 
 * NOTE: This is Chapter 3 of Section III Actuality. The absolute relation is a Species
 * of the Genus Actuality. Each Chapter (Species) has three sub-species that make up
 * its Concept:
 * 
 * Structure:
 * Introduction: Absolute necessity as absolute relation, three moments
 * A. THE RELATION OF SUBSTANTIALITY (Species 1)
 * B. THE RELATION OF CAUSALITY (Species 2)
 * C. RECIPROCITY OF ACTION (Species 3)
 * 
 * Transition: Reciprocity passes over to the concept (subjectivity/freedom)
 */

// ============================================================================
// INTRODUCTION: CHAPTER 3 — THE ABSOLUTE RELATION
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'sub-ch3-intro-1-absolute-necessity-relation',
    title: 'Absolute necessity as absolute relation — reflective shine',
    text: `Absolute necessity is not so much the necessary,
even less a necessary, but necessity:
being simply as reflection.
It is relation because it is a distinguishing
whose moments are themselves
the whole totality of necessity,
and therefore subsist absolutely,
but do so in such a way that
their subsisting is one subsistence,
and the difference only the reflective shine of
the movement of exposition,
and this reflective shine is the absolute itself.
Essence as such is reflection or a shining;
as absolute relation, however, essence is the
reflective shine posited as reflective shine,
one which, as such self-referring, is absolute actuality.
The absolute, first expounded by external reflection,
as absolute form or as necessity now expounds itself;
this self-exposition is its self-positing,
and is only this self-positing.
Just as the light of nature is not a something,
nor is it a thing, but its being is rather only its shining,
so manifestation is self-identical absolute actuality.`,
    summary: 'Absolute necessity = necessity (not the necessary, not a necessary), being as reflection. Relation = distinguishing whose moments = whole totality of necessity, subsist absolutely, one subsistence, difference = reflective shine of movement of exposition, reflective shine = absolute itself. Essence = reflection/shining; as absolute relation = reflective shine posited as reflective shine, self-referring = absolute actuality. Absolute expounded by external reflection, now expounds itself = self-positing. Light of nature = only shining; manifestation = self-identical absolute actuality.'
  },
  {
    id: 'sub-ch3-intro-2-sides-totalities',
    title: 'Sides are totalities — identical positing',
    text: `The sides of the absolute relation
are not, therefore, attributes.
In the attribute the absolute reflectively shines
only in one of its moments,
as in a presupposition that
external reflection has simply assumed.
But the expositor of the absolute is the absolute necessity
which, as self-determining, is identical with itself.
Since this necessity is the reflective shining
posited as reflective shining, the sides of this relation,
because they are as shine, are totalities;
for as shine, the differences are
themselves and their opposite,
that is, they are the whole;
and, conversely, they thus are only shine
because they are totalities.
Thus this distinguishing,
this reflecting shining of the absolute,
is only the identical positing of itself.`,
    summary: 'Sides of absolute relation not attributes. In attribute: absolute shines only in one moment (presupposition of external reflection). Expositor = absolute necessity (self-determining, identical with itself). Necessity = reflective shining posited as reflective shining, sides = totalities (as shine, differences = themselves and opposite = whole). Only shine because totalities. Distinguishing/reflecting shining = identical positing of itself.'
  },
  {
    id: 'sub-ch3-intro-3-three-relations',
    title: 'Three relations — substance, causality, concept',
    text: `This relation in its immediate concept is
the relation of substance and accidents,
the immediate internal disappearing and becoming
of the absolute reflective shine.
If substance determines itself as a being-for-itself over
against an other or is absolute relation as something real,
then we have the relation of causality.
Finally, when this last relation passes over into
reciprocal causality by referring itself to itself,
we then have the absolute relation also posited
in accordance with the determination it contains;
this posited unity of itself in its determinations,
which are posited as the whole itself
and consequently equally as determinations,
is then the concept.`,
    summary: 'Relation in immediate concept = substance and accidents (immediate internal disappearing/becoming of absolute reflective shine). If substance determines as being-for-itself over against other = absolute relation as real = causality. When causality passes into reciprocal causality (referring itself to itself) = absolute relation posited, posited unity of itself in determinations (posited as whole, equally as determinations) = concept.'
  },
];

// ============================================================================
// INTRODUCTION OPERATIONS
// ============================================================================

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'sub-ch3-op-intro-1-absolute-necessity-relation',
    chunkId: 'sub-ch3-intro-1-absolute-necessity-relation',
    label: 'Absolute necessity as absolute relation — reflective shine',
    clauses: [
      'absoluteNecessity.not = theNecessary',
      'absoluteNecessity.not = aNecessary',
      'absoluteNecessity = necessity',
      'absoluteNecessity = beingAsReflection',
      'relation = distinguishing',
      'distinguishing.moments = wholeTotalityOfNecessity',
      'moments.subsistAbsolutely = true',
      'moments.subsisting = oneSubsistence',
      'difference = reflectiveShine',
      'reflectiveShine.of = movementOfExposition',
      'reflectiveShine = absoluteItself',
      'essence = reflection',
      'essence = shining',
      'asAbsoluteRelation.essence = reflectiveShinePosited',
      'reflectiveShinePosited = selfReferring',
      'selfReferring = absoluteActuality',
      'absolute.expoundedBy = externalReflection',
      'absolute.nowExpoundsItself = true',
      'selfExposition = selfPositing',
      'lightOfNature.not = something',
      'lightOfNature.not = thing',
      'lightOfNature.being = onlyShining',
      'manifestation = selfIdenticalAbsoluteActuality',
    ],
    predicates: [
      { name: 'IsAbsoluteRelation', args: ['absoluteNecessity'] },
    ],
    relations: [
      { predicate: 'is', from: 'absoluteNecessity', to: 'absoluteRelation' },
    ],
  },
  {
    id: 'sub-ch3-op-intro-2-sides-totalities',
    chunkId: 'sub-ch3-intro-2-sides-totalities',
    label: 'Sides are totalities — identical positing',
    clauses: [
      'sidesOfAbsoluteRelation.not = attributes',
      'inAttribute.absoluteShinesOnlyIn = oneMoment',
      'oneMoment = presupposition',
      'presupposition.of = externalReflection',
      'expositor = absoluteNecessity',
      'absoluteNecessity = selfDetermining',
      'absoluteNecessity = identicalWithItself',
      'necessity = reflectiveShiningPosited',
      'sides = totalities',
      'becauseAsShine = true',
      'differences = themselvesAndOpposite',
      'differences = whole',
      'onlyShineBecause = totalities',
      'distinguishing = identicalPositing',
      'reflectingShining = identicalPositing',
    ],
    predicates: [
      { name: 'AreTotalities', args: ['sides'] },
    ],
    relations: [
      { predicate: 'are', from: 'sides', to: 'totalities' },
    ],
  },
  {
    id: 'sub-ch3-op-intro-3-three-relations',
    chunkId: 'sub-ch3-intro-3-three-relations',
    label: 'Three relations — substance, causality, concept',
    clauses: [
      'relationInImmediateConcept = substanceAndAccidents',
      'substanceAndAccidents = immediateInternalDisappearing',
      'substanceAndAccidents = becoming',
      'becoming.of = absoluteReflectiveShine',
      'ifSubstanceDeterminesAs = beingForItself',
      'beingForItself.overAgainst = other',
      'absoluteRelationAsReal = causality',
      'whenCausalityPassesInto = reciprocalCausality',
      'reciprocalCausality.referringItselfTo = itself',
      'absoluteRelationPosited = true',
      'positedUnity = itselfInDeterminations',
      'determinations.positedAs = wholeItself',
      'determinations.equallyAs = determinations',
      'this = concept',
    ],
    predicates: [
      { name: 'HasThreeMoments', args: ['relation'] },
    ],
    relations: [
      { predicate: 'has', from: 'relation', to: 'threeMoments' },
    ],
  },
];

// Note: For aggregated exports, use index.ts
// Individual species modules are available for granular access

