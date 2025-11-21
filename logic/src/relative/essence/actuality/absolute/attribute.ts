import type { Chunk, LogicalOperation } from '../../../types';

/**
 * B. THE ABSOLUTE ATTRIBUTE — Species 2 of The Absolute
 *
 * This module covers the second species: the absolute attribute,
 * relative absolute, and the form determination.
 */

// ============================================================================
// B. THE ABSOLUTE ATTRIBUTE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'abs-b-1-absolute-absolute-vs-attribute',
    title: 'Absolute absolute vs attribute — relative absolute',
    text: `The expression which we have used, "the absolute absolute,"
denotes the absolute which in its form
has returned back into itself
or whose form is equal to its content.
The attribute is just the relative absolute,
a combination which only signifies the absolute
in a form determination.
For at first, before its complete exposition,
the form is only internally
or, which is the same, only externally;
it is at first determinate form in general
or negation in general.
But because form is at the same time
as the form of the absolute,
the attribute is the whole content of the absolute;
it is the totality which earlier appeared as a world,
or as one of the sides of the essential relation,
each of which is itself the whole.`,
    summary: '"Absolute absolute" = absolute whose form returned into itself, form = content. Attribute = relative absolute, absolute in form determination. Form at first = internally/externally, determinate form/negation. But form = form of absolute, attribute = whole content, totality (world, side of essential relation, each = whole).'
  },
  {
    id: 'abs-b-2-worlds-reduced-to-shine',
    title: 'Worlds reduced to reflective shine — true subsistence',
    text: `But both worlds, the phenomenal world
and the world that exists in and for itself,
were supposed to be opposed
to each other in their essence.
Each side of the essential relation was
indeed equal to the other:
the whole as much as the parts,
the expression of force the same content
as force itself,
and the outer everywhere the same as the inner.
But these sides were at the same time
supposed each to have still
an immediate subsistence of its own,
the one side as existent immediacy
and the other as reflected immediacy.
In the absolute, on the contrary,
these different immediacies have been
reduced to a reflective shine,
and the totality that the attribute is
is posited as its true and single subsistence,
while the determination in which it is
is posited as unessential subsistence.`,
    summary: 'Both worlds opposed in essence. Each side equal to other (whole/parts, expression/force, outer/inner). But each had immediate subsistence (existent immediacy, reflected immediacy). In absolute: different immediacies reduced to reflective shine. Totality (attribute) = true single subsistence, determination = unessential subsistence.'
  },
  {
    id: 'abs-b-3-attribute-as-identity-determination',
    title: 'Attribute as identity determination — absolute totality',
    text: `The absolute is attribute because,
as simple absolute identity,
it is in the determination of identity;
now to the determination as such
other determinations can be attached,
for instance, also that there are several attributes.
But because absolute identity has only this meaning,
that not only all determinations have been sublated
but that reflection itself has also sublated itself,
all determinations are thus posited in it as sublated.
Or the totality is posited as absolute totality.
Or again, the attribute has the absolute
for its content and subsistence
and, consequently, its form determination
by which it is attribute is also posited,
posited immediately as mere reflective shine;
the negative is posited as negative.`,
    summary: 'Absolute = attribute because simple absolute identity in determination of identity. Other determinations can be attached (several attributes). But absolute identity = all determinations sublated, reflection sublated itself, determinations posited as sublated. Totality = absolute totality. Attribute has absolute for content/subsistence, form determination posited as mere reflective shine, negative posited as negative.'
  },
  {
    id: 'abs-b-4-positive-reflective-shine-sublates',
    title: 'Positive reflective shine sublates attribute',
    text: `The positive reflective shine that
the exposition gives itself through the attribute
in that it does not take the finite in its limitation as
something that exists in and for itself
but dissolves its subsistence into the absolute
and expands it into attribute;
sublates precisely this, that the attribute is attribute;
it sinks it and its differentiating act
into the simple absolute.`,
    summary: 'Positive reflective shine (exposition through attribute): does not take finite in limitation as existing in and for itself, dissolves subsistence into absolute, expands into attribute. Sublates that attribute is attribute, sinks it and differentiating act into simple absolute.'
  },
  {
    id: 'abs-b-5-reflection-reverts-to-identity',
    title: 'Reflection reverts to identity — not true absolute',
    text: `But since reflection thus reverts
from its differentiating act
only to the identity of the absolute,
it has not at the same time
left its externality behind
and has not arrived at the true absolute.
It has only reached the
indeterminate, abstract identity,
which is to say, the identity
in the determinateness of identity.
Or, since reflection determines the
absolute into attribute as inner form,
this determining is something
still distinct from externality;
the inner determination does not
penetrate the absolute;
the attribute's expression,
as something merely posited,
is to disappear into the absolute.`,
    summary: 'Reflection reverts from differentiating act only to identity of absolute, has not left externality behind, not arrived at true absolute. Only reached indeterminate abstract identity (identity in determinateness of identity). Reflection determines absolute into attribute as inner form, determining distinct from externality, inner determination does not penetrate, attribute\'s expression (merely posited) to disappear into absolute.'
  },
  {
    id: 'abs-b-6-form-nullity',
    title: 'Form as nullity — external reflective shine',
    text: `The form by virtue of which
the absolute would be attribute,
whether it is taken as outer or inner,
is therefore posited as something null in itself,
an external reflective shine,
or a mere way and manner.`,
    summary: 'Form by which absolute would be attribute (outer or inner) = null in itself, external reflective shine, mere way and manner.'
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'abs-b-op-1-absolute-absolute-vs-attribute',
    chunkId: 'abs-b-1-absolute-absolute-vs-attribute',
    label: 'Absolute absolute vs attribute — relative absolute',
    clauses: [
      '"absoluteAbsolute" = absolute',
      'absoluteAbsolute.form.returnedInto = itself',
      'absoluteAbsolute.form = content',
      'attribute = relativeAbsolute',
      'attribute = absoluteInFormDetermination',
      'form.atFirst = {internally, externally}',
      'form.atFirst = {determinateForm, negation}',
      'form = formOfAbsolute',
      'attribute = wholeContent',
      'attribute = totality',
      'totality.appearedAs = {world, sideOfEssentialRelation}',
      'each = whole',
    ],
    predicates: [
      { name: 'IsRelativeAbsolute', args: ['attribute'] },
    ],
    relations: [
      { predicate: 'is', from: 'attribute', to: 'relativeAbsolute' },
    ],
  },
  {
    id: 'abs-b-op-2-worlds-reduced-to-shine',
    chunkId: 'abs-b-2-worlds-reduced-to-shine',
    label: 'Worlds reduced to reflective shine — true subsistence',
    clauses: [
      'bothWorlds.opposedIn = essence',
      'eachSide.equalTo = other',
      'sides.had = immediateSubsistence',
      'oneSide = existentImmediacy',
      'otherSide = reflectedImmediacy',
      'inAbsolute.immediacies.reducedTo = reflectiveShine',
      'totality.positedAs = trueSingleSubsistence',
      'determination.positedAs = unessentialSubsistence',
    ],
    predicates: [
      { name: 'IsReducedToShine', args: ['immediacies'] },
    ],
    relations: [
      { predicate: 'reducedTo', from: 'immediacies', to: 'reflectiveShine' },
    ],
  },
  {
    id: 'abs-b-op-3-attribute-as-identity-determination',
    chunkId: 'abs-b-3-attribute-as-identity-determination',
    label: 'Attribute as identity determination — absolute totality',
    clauses: [
      'absolute = attribute',
      'absolute.as = simpleAbsoluteIdentity',
      'absolute.in = determinationOfIdentity',
      'otherDeterminations.canBeAttached = true',
      'absoluteIdentity = allDeterminationsSublated',
      'absoluteIdentity = reflectionSublatedItself',
      'determinations.positedAs = sublated',
      'totality = absoluteTotality',
      'attribute.hasAbsoluteFor = {content, subsistence}',
      'formDetermination.positedAs = mereReflectiveShine',
      'negative.positedAs = negative',
    ],
    predicates: [
      { name: 'IsAbsoluteTotality', args: ['totality'] },
    ],
    relations: [
      { predicate: 'is', from: 'totality', to: 'absoluteTotality' },
    ],
  },
  {
    id: 'abs-b-op-4-positive-reflective-shine-sublates',
    chunkId: 'abs-b-4-positive-reflective-shine-sublates',
    label: 'Positive reflective shine sublates attribute',
    clauses: [
      'positiveReflectiveShine.through = attribute',
      'doesNotTakeFiniteInLimitation = asExistingInAndForItself',
      'dissolvesSubsistenceInto = absolute',
      'expandsInto = attribute',
      'sublates = attributeIsAttribute',
      'sinksInto = simpleAbsolute',
    ],
    predicates: [
      { name: 'SublatesAttribute', args: ['positiveReflectiveShine'] },
    ],
    relations: [
      { predicate: 'sublates', from: 'positiveReflectiveShine', to: 'attribute' },
    ],
  },
  {
    id: 'abs-b-op-5-reflection-reverts-to-identity',
    chunkId: 'abs-b-5-reflection-reverts-to-identity',
    label: 'Reflection reverts to identity — not true absolute',
    clauses: [
      'reflection.revertsFrom = differentiatingAct',
      'revertsTo = identityOfAbsolute',
      'hasNotLeft = externality',
      'hasNotArrivedAt = trueAbsolute',
      'onlyReached = indeterminateAbstractIdentity',
      'indeterminateAbstractIdentity = identityInDeterminateness',
      'reflection.determinesAbsoluteInto = attribute',
      'attribute.as = innerForm',
      'determining.distinctFrom = externality',
      'innerDetermination.notPenetrates = absolute',
      'attributesExpression = merelyPosited',
      'attributesExpression.toDisappearInto = absolute',
    ],
    predicates: [
      { name: 'IsNotTrueAbsolute', args: ['identity'] },
    ],
    relations: [
      { predicate: 'revertsTo', from: 'reflection', to: 'identity' },
    ],
  },
  {
    id: 'abs-b-op-6-form-nullity',
    chunkId: 'abs-b-6-form-nullity',
    label: 'Form as nullity — external reflective shine',
    clauses: [
      'form.byWhichAbsoluteWouldBe = attribute',
      'form.as = {outer, inner}',
      'form.positedAs = nullInItself',
      'form = externalReflectiveShine',
      'form = mereWayAndManner',
    ],
    predicates: [
      { name: 'IsNullity', args: ['form'] },
    ],
    relations: [
      { predicate: 'is', from: 'form', to: 'nullity' },
    ],
  },
];

