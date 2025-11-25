/**
 * Logical Operations: Identity
 *
 * Identity is the first determination of reflection. Essence is simple
 * self-identity, essential identity. Being's negativity is identity itself.
 *
 * Dialectical Movement:
 * - Chapter 2 introduction: Foundation, essentialities
 * - Three moments: identity, difference, contradiction
 * - Essence as simple self-identity
 * - Being's negativity is identity itself
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// IDENTITY
// ============================================================================

export const idnOp1ChapterIntroduction: LogicalOperation = {
  id: 'idn-op-1-chapter-introduction',
  chunkId: 'idn-1',
  label: 'Chapter 2 introduction — Foundation, essentialities',
  clauses: [
    'foundation = essentialities',
    'foundation = determinationsOfReflection',
    'reflection = determinedReflection',
    'essence = determinedEssence',
    'essence = essentiality',
    'reflection = shiningOfEssenceWithinItself',
    'essence = infiniteImmanentTurningBack',
    'essence != immediateSimplicity',
    'essence = negativeSimplicity',
    'movement = absoluteMediationWithItself',
    'moments = determinationsReflectedIntoThemselves',
  ],
  predicates: [
    { name: 'essentialities', args: ['foundation'] },
    { name: 'determinationsOfReflection', args: ['foundation'] },
    { name: 'determinedReflection', args: ['reflection'] },
    { name: 'determinedEssence', args: ['essence'] },
    { name: 'shiningOfEssenceWithinItself', args: ['reflection'] },
    { name: 'infiniteImmanentTurningBack', args: ['essence'] },
    { name: 'negativeSimplicity', args: ['essence'] },
    { name: 'absoluteMediationWithItself', args: ['movement'] },
    { name: 'determinationsReflectedIntoThemselves', args: ['moments'] },
  ],
  relations: [
    { predicate: 'is', from: 'foundation', to: 'essentialities' },
    { predicate: 'is', from: 'reflection', to: 'shiningOfEssenceWithinItself' },
  ],
  candidateSummary: 'Foundation: The essentialities or the determinations of reflection. Reflection is determined reflection; essence is determined essence, essentiality. Reflection is shining of essence within itself. Essence as infinite immanent turning back. Not immediate simplicity, but negative simplicity. Movement across moments that are distinct, absolute mediation with itself. In these moments it shines; moments are determinations reflected into themselves.',
  provenance: {
    sourceChunk: 'idn-1',
    sourceOp: 'idn-op-1-chapter-introduction',
  },
};

export const idnOp2ThreeMoments: LogicalOperation = {
  id: 'idn-op-2-three-moments',
  chunkId: 'idn-2',
  label: 'Three moments: identity, difference, contradiction',
  clauses: [
    'first = identity',
    'identity = simpleSelfReference',
    'thisDetermination = absenceOfDetermination',
    'second = difference',
    'difference = specifyingDetermination',
    'difference = externalOrIndefinite',
    'difference = diversity',
    'difference = opposition',
    'third = contradiction',
    'contradiction = oppositionReflectedIntoItself',
    'contradiction = returnToFoundation',
  ],
  predicates: [
    { name: 'simpleSelfReference', args: ['identity'] },
    { name: 'absenceOfDetermination', args: ['thisDetermination'] },
    { name: 'specifyingDetermination', args: ['difference'] },
    { name: 'externalOrIndefinite', args: ['difference'] },
    { name: 'diversity', args: ['difference'] },
    { name: 'opposition', args: ['difference'] },
    { name: 'oppositionReflectedIntoItself', args: ['contradiction'] },
    { name: 'returnToFoundation', args: ['contradiction'] },
  ],
  relations: [
    { predicate: 'is', from: 'first', to: 'identity' },
    { predicate: 'is', from: 'second', to: 'difference' },
    { predicate: 'is', from: 'third', to: 'contradiction' },
  ],
  candidateSummary: 'First: essence is simple self-reference, pure identity. This determination is absence of determination. Second: specifying determination is difference. Difference: external or indefinite, diversity in general, or opposed diversity or opposition. Third: as contradiction, opposition reflected into itself and returns to foundation.',
  provenance: {
    sourceChunk: 'idn-2',
    sourceOp: 'idn-op-2-three-moments',
  },
};

export const idnOp3EssenceAsSimpleSelfIdentity: LogicalOperation = {
  id: 'idn-op-3-essence-as-simple-self-identity',
  chunkId: 'idn-3',
  label: 'A. IDENTITY — essence as simple self-identity',
  clauses: [
    'essence = simpleImmediacy',
    'essence = sublatedImmediacy',
    'negativity = being',
    'essence = equalToItself',
    'equality = inAbsoluteNegativity',
    'otherness = disappeared',
    'referenceToOther = disappeared',
    'disappearance = intoPureSelfEquality',
    'essence = simpleSelfIdentity',
  ],
  predicates: [
    { name: 'simpleImmediacy', args: ['essence'] },
    { name: 'sublatedImmediacy', args: ['essence'] },
    { name: 'being', args: ['negativity'] },
    { name: 'equalToItself', args: ['essence'] },
    { name: 'inAbsoluteNegativity', args: ['equality'] },
    { name: 'disappeared', args: ['otherness'] },
    { name: 'simpleSelfIdentity', args: ['essence'] },
  ],
  relations: [
    { predicate: 'is', from: 'essence', to: 'simpleSelfIdentity' },
    { predicate: 'disappearedInto', from: 'otherness', to: 'pureSelfEquality' },
  ],
  candidateSummary: 'Essence is simple immediacy as sublated immediacy. Its negativity is its being. Equal to itself in absolute negativity. By virtue of which otherness and reference to other have disappeared into pure self-equality. Essence is therefore simple self-identity.',
  provenance: {
    sourceChunk: 'idn-3',
    sourceOp: 'idn-op-3-essence-as-simple-self-identity',
  },
};

export const idnOp4SelfIdentityAsImmediacy: LogicalOperation = {
  id: 'idn-op-4-self-identity-as-immediacy',
  chunkId: 'idn-4',
  label: 'Self-identity as immediacy of reflection — essential identity',
  clauses: [
    'selfIdentity = immediacyOfReflection',
    'selfIdentity != selfEqualityOfBeing',
    'selfIdentity != nothing',
    'selfEquality = pureProduction',
    'production = fromItself',
    'production = inItself',
    'selfEquality != productionAsFromAnother',
    'identity = essentialIdentity',
  ],
  predicates: [
    { name: 'immediacyOfReflection', args: ['selfIdentity'] },
    { name: 'pureProduction', args: ['selfEquality'] },
    { name: 'fromItself', args: ['production'] },
    { name: 'inItself', args: ['production'] },
    { name: 'essentialIdentity', args: ['identity'] },
  ],
  relations: [
    { predicate: 'is', from: 'selfIdentity', to: 'immediacyOfReflection' },
  ],
  candidateSummary: 'This self-identity is immediacy of reflection. Not self-equality which being is, or also nothing. Self-equality which, in producing itself as unity, does not produce itself over again, as from another. Pure production, from itself and in itself. Essential identity.',
  provenance: {
    sourceChunk: 'idn-4',
    sourceOp: 'idn-op-4-self-identity-as-immediacy',
  },
};

export const idnOp5NotAbstractIdentity: LogicalOperation = {
  id: 'idn-op-5-not-abstract-identity',
  chunkId: 'idn-5',
  label: 'Not abstract identity — relative negation',
  clauses: [
    'identity != abstractIdentity',
    'identity != resultOfRelativeNegation',
    'relativeNegation = separatesDistinguished',
    'relativeNegation = leavesExistingOutside',
    'distinguished = sameAfterAsBefore',
  ],
  predicates: [
    { name: 'abstractIdentity', args: ['identity'] },
    { name: 'resultOfRelativeNegation', args: ['identity'] },
    { name: 'separatesDistinguished', args: ['relativeNegation'] },
    { name: 'leavesExistingOutside', args: ['relativeNegation'] },
    { name: 'sameAfterAsBefore', args: ['distinguished'] },
  ],
  relations: [
    { predicate: 'isNot', from: 'identity', to: 'abstractIdentity' },
  ],
  candidateSummary: 'Not abstract identity. Not identity which is result of relative negation preceding it. Relative negation separates what it distinguishes but leaves it existing outside it. Same after as before.',
  provenance: {
    sourceChunk: 'idn-5',
    sourceOp: 'idn-op-5-not-abstract-identity',
  },
};

export const idnOp6BeingNegativityIsIdentity: LogicalOperation = {
  id: 'idn-op-6-being-negativity-is-identity',
  chunkId: 'idn-6',
  label: 'Being\'s negativity is identity itself',
  clauses: [
    'being = sublatedItself',
    'sublation = inItself',
    'sublation != relatively',
    'simpleNegativity = negativityOfBeing',
    'simpleNegativity = identityItself',
    'identity = sameAsEssence',
  ],
  predicates: [
    { name: 'sublatedItself', args: ['being'] },
    { name: 'inItself', args: ['sublation'] },
    { name: 'negativityOfBeing', args: ['simpleNegativity'] },
    { name: 'identityItself', args: ['simpleNegativity'] },
    { name: 'sameAsEssence', args: ['identity'] },
  ],
  relations: [
    { predicate: 'is', from: 'simpleNegativity', to: 'identityItself' },
  ],
  candidateSummary: 'Being, and every determinateness of being, has sublated itself not relatively, but in itself. This simple negativity, negativity of being in itself, is the identity itself. In general, therefore, it is still the same as essence.',
  provenance: {
    sourceChunk: 'idn-6',
    sourceOp: 'idn-op-6-being-negativity-is-identity',
  },
};

export const identityOperations: LogicalOperation[] = [
  idnOp1ChapterIntroduction,
  idnOp2ThreeMoments,
  idnOp3EssenceAsSimpleSelfIdentity,
  idnOp4SelfIdentityAsImmediacy,
  idnOp5NotAbstractIdentity,
  idnOp6BeingNegativityIsIdentity,
];
