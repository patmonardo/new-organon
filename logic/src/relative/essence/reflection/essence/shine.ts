/**
 * Logical Operations: Shine
 *
 * Shine (Semblance) is the reflection of essence within itself. It is the negative
 * posited as negative, the immediate non-existence that constitutes the determinateness
 * of essence.
 *
 * Dialectical Movement:
 * - Shine as remainder of being
 * - Shine in skepticism and idealism
 * - Shine as essence's own negativity
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// SHINE
// ============================================================================

export const shnOp1BeingIsShine: LogicalOperation = {
  id: 'shn-op-1-being-is-shine',
  chunkId: 'shn-1',
  label: 'Being is shine — negative posited as negative',
  clauses: [
    'beingOfShine = sublatednessOfBeing',
    'beingOfShine = beingsNothingness',
    'nothingness = hasInEssence',
    'shine = negativePositedAsNegative',
    'shine = doesNotExistApartFromEssence',
  ],
  predicates: [
    { name: 'sublatednessOfBeing', args: ['beingOfShine'] },
    { name: 'beingsNothingness', args: ['beingOfShine'] },
    { name: 'hasInEssence', args: ['nothingness'] },
    { name: 'negativePositedAsNegative', args: ['shine'] },
    { name: 'doesNotExistApartFromEssence', args: ['shine'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'negativePositedAsNegative' },
  ],
  candidateSummary: 'Being of shine consists solely in sublatedness of being, in being\'s nothingness. This nothingness it has in essence. Apart from essence, it does not exist. It is the negative posited as negative.',
  provenance: {
    sourceChunk: 'shn-1',
    sourceOp: 'shn-op-1-being-is-shine',
  },
};

export const shnOp2RemainderOfBeing: LogicalOperation = {
  id: 'shn-op-2-remainder-of-being',
  chunkId: 'shn-2',
  label: 'Shine as remainder of being — immediate non-existence',
  clauses: [
    'shine = remainderOfBeing',
    'shine = immediateNonExistence',
    'shine = nonSelfSubsistent',
    'shine = reflectedImmediacy',
    'shine = existsInNegation',
  ],
  predicates: [
    { name: 'remainderOfBeing', args: ['shine'] },
    { name: 'immediateNonExistence', args: ['shine'] },
    { name: 'nonSelfSubsistent', args: ['shine'] },
    { name: 'reflectedImmediacy', args: ['shine'] },
    { name: 'existsInNegation', args: ['shine'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'immediateNonExistence' },
  ],
  candidateSummary: 'Shine is all that remains of sphere of being. Seems to have immediate side independent of essence, other of essence. Other entails existence and non-existence. Since unessential no longer has being, only pure moment of non-existence remains. Shine is immediate non-existence, non-existence in determinateness of being. Exists only with reference to another, in its non-existence. Non-self-subsistent which exists only in its negation. Pure determinateness of immediacy, reflected immediacy. Only by virtue of mediation of its negation. Empty determination of immediacy of non-existence.',
  provenance: {
    sourceChunk: 'shn-2',
    sourceOp: 'shn-op-2-remainder-of-being',
  },
};

export const shnOp3SkepticismIdealism: LogicalOperation = {
  id: 'shn-op-3-skepticism-idealism',
  chunkId: 'shn-3',
  label: 'Shine in skepticism and idealism — manifold determinations',
  clauses: [
    'shine = phenomenonOfSkepticism',
    'shine = appearanceOfIdealism',
    'shine = manifoldOfDeterminations',
    'content = transposedFromBeing',
    'shine = immediatelyDetermined',
    'idealism = notAdvancedBeyondBeing',
  ],
  predicates: [
    { name: 'phenomenonOfSkepticism', args: ['shine'] },
    { name: 'appearanceOfIdealism', args: ['shine'] },
    { name: 'manifoldOfDeterminations', args: ['shine'] },
    { name: 'transposedFromBeing', args: ['content'] },
    { name: 'immediatelyDetermined', args: ['shine'] },
    { name: 'notAdvancedBeyondBeing', args: ['idealism'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'phenomenon' },
    { predicate: 'is', from: 'shine', to: 'appearance' },
  ],
  candidateSummary: 'Shine is \'phenomenon\' of skepticism, \'appearance\' of idealism. Immediacy which is not something nor thing in general. Not indifferent being apart from determinateness and connection with subject. Skepticism: shine supposed absolutely not to have foundation of being. Idealism: thing-in-itself not supposed to enter into cognitions. But both allow manifold of determinations, full richness of world. Content might have no being as foundation, no thing-in-itself. Content simply transposed from being into shine. Shine is immediately determined, can have this or that content. But whatever content, has not posited it but possesses it immediately. Idealism (Leibnizian, Kantian, Fichtean) not gone further than skepticism. Not advanced beyond being as determinateness.',
  provenance: {
    sourceChunk: 'shn-3',
    sourceOp: 'shn-op-3-skepticism-idealism',
  },
};

export const shnOp4ShineDeterminations: LogicalOperation = {
  id: 'shn-op-4-shine-determinations',
  chunkId: 'shn-4',
  label: 'Task: shine\'s determinations are essence\'s own',
  clauses: [
    'shine = immediatePresupposition',
    'shine = nullAsSuch',
    'determinationsOfShine = determinationsOfEssence',
    'determinateness = sublatedInEssence',
  ],
  predicates: [
    { name: 'immediatePresupposition', args: ['shine'] },
    { name: 'nullAsSuch', args: ['shine'] },
    { name: 'determinationsOfEssence', args: ['determinationsOfShine'] },
    { name: 'sublatedInEssence', args: ['determinateness'] },
  ],
  relations: [
    { predicate: 'are', from: 'determinationsOfShine', to: 'determinationsOfEssence' },
  ],
  candidateSummary: 'Shine contains immediate presupposition, independent side vis-à-vis essence. Task not to demonstrate shine sublates itself and returns into essence. Being has returned into essence in its totality; shine is null as such. Task: demonstrate determinations distinguishing shine from essence are determinations of essence itself. Further: this determinateness of essence (which shine is) is sublated in essence itself.',
  provenance: {
    sourceChunk: 'shn-4',
    sourceOp: 'shn-op-4-shine-determinations',
  },
};

export const shnOp5EssenceNegativity: LogicalOperation = {
  id: 'shn-op-5-essence-negativity',
  chunkId: 'shn-5',
  label: 'Shine as essence\'s negativity — immediacy of non-being',
  clauses: [
    'shine = immediacyOfNonBeing',
    'nonBeing = negativityOfEssence',
    'being = nonBeingInEssence',
    'immediacy = essencesOwnAbsoluteInItself',
    'shine = reflectiveImmediacy',
  ],
  predicates: [
    { name: 'immediacyOfNonBeing', args: ['shine'] },
    { name: 'negativityOfEssence', args: ['nonBeing'] },
    { name: 'nonBeingInEssence', args: ['being'] },
    { name: 'essencesOwnAbsoluteInItself', args: ['immediacy'] },
    { name: 'reflectiveImmediacy', args: ['shine'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'immediacyOfNonBeing' },
  ],
  candidateSummary: 'What constitutes shine is immediacy of non-being. This non-being is nothing else than negativity of essence within essence itself. In essence, being is non-being. Inherent nothingness is negative nature of essence itself. Immediacy or indifference which non-being contains is essence\'s own absolute in-itself. Negativity of essence is its self-equality, simple immediacy and indifference. Being preserved in essence through infinite negativity having equality with itself. Through this essence is itself being. Immediacy determinateness has in shine against essence is essence\'s own immediacy. Not immediacy of existent but absolutely mediated or reflective immediacy which is shine. Being, not as being, but only as determinateness of being as against mediation. Being as moment.',
  provenance: {
    sourceChunk: 'shn-5',
    sourceOp: 'shn-op-5-essence-negativity',
  },
};

export const shnOp6TwoMoments: LogicalOperation = {
  id: 'shn-op-6-two-moments',
  chunkId: 'shn-6',
  label: 'Two moments of shine are moments of essence',
  clauses: [
    'moments = nothingnessAndBeing',
    'nothingness = subsisting',
    'being = moment',
    'momentsOfShine = momentsOfEssence',
    'shine = shineOfEssenceItself',
  ],
  predicates: [
    { name: 'nothingnessAndBeing', args: ['moments'] },
    { name: 'subsisting', args: ['nothingness'] },
    { name: 'moment', args: ['being'] },
    { name: 'momentsOfEssence', args: ['momentsOfShine'] },
    { name: 'shineOfEssenceItself', args: ['shine'] },
  ],
  relations: [
    { predicate: 'are', from: 'momentsOfShine', to: 'momentsOfEssence' },
  ],
  candidateSummary: 'Two moments: nothingness but as subsisting, and being but as moment. Or: negativity existing in itself and reflected immediacy. These two moments of shine are moments of essence itself. Not shine of being in essence, or shine of essence in being. Shine in essence is not shine of other but shine as such, shine of essence itself.',
  provenance: {
    sourceChunk: 'shn-6',
    sourceOp: 'shn-op-6-two-moments',
  },
};

export const shnOp7EssenceInDeterminateness: LogicalOperation = {
  id: 'shn-op-7-essence-in-determinateness',
  chunkId: 'shn-7',
  label: 'Shine as essence itself in determinateness of being',
  clauses: [
    'shine = essenceInDeterminateness',
    'essence = determinedWithinItself',
    'determinateness = sublated',
    'essence = selfMediating',
    'negativity = selfRepelling',
  ],
  predicates: [
    { name: 'essenceInDeterminateness', args: ['shine'] },
    { name: 'determinedWithinItself', args: ['essence'] },
    { name: 'sublated', args: ['determinateness'] },
    { name: 'selfMediating', args: ['essence'] },
    { name: 'selfRepelling', args: ['negativity'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'essenceInDeterminateness' },
  ],
  candidateSummary: 'Shine is essence itself in determinateness of being. Essence has shine because determined within itself, distinguished from absolute unity. This determinateness absolutely sublated in it. Essence stands on its own, self-mediating through negation which it itself is. Identical unit of absolute negativity and immediacy. Negativity is negativity in itself, reference to itself, immediacy in itself. But negative reference to itself, self-repelling negating. Immediacy existing in itself is negative or determinate over against negativity. But determinateness itself is absolute negativity. Determining immediately sublates itself, turning back into itself.',
  provenance: {
    sourceChunk: 'shn-7',
    sourceOp: 'shn-op-7-essence-in-determinateness',
  },
};

export const shnOp8NegativeReturning: LogicalOperation = {
  id: 'shn-op-8-negative-returning',
  chunkId: 'shn-8',
  label: 'Shine as negative returning into itself',
  clauses: [
    'shine = negativeReturningIntoItself',
    'shine = internallyNonSubsistent',
    'referenceToSelf = immediacy',
    'negation = negativityReferringToItself',
    'sublation = absolute',
  ],
  predicates: [
    { name: 'negativeReturningIntoItself', args: ['shine'] },
    { name: 'internallyNonSubsistent', args: ['shine'] },
    { name: 'immediacy', args: ['referenceToSelf'] },
    { name: 'negativityReferringToItself', args: ['negation'] },
    { name: 'absolute', args: ['sublation'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'negativeReturningIntoItself' },
  ],
  candidateSummary: 'Shine is negative which has being, but in another, in its negation. Non-self-subsisting-being sublated within and null. Negative which returns into itself, non-subsistent as such, internally non-subsistent. Reference of negative or non-subsistent to itself is immediacy of this non-subsistent. It is other than it, determinateness over against it, negation over against negative. Negation standing over against negative is negativity referring solely to itself. Absolute sublation of determinateness itself.',
  provenance: {
    sourceChunk: 'shn-8',
    sourceOp: 'shn-op-8-negative-returning',
  },
};

export const shnOp9InfiniteDeterminateness: LogicalOperation = {
  id: 'shn-op-9-infinite-determinateness',
  chunkId: 'shn-9',
  label: 'Shine as infinite determinateness — essence',
  clauses: [
    'shine = infiniteDeterminateness',
    'negative = coincidesWithItself',
    'determinateness = selfSubsistence',
    'negativity = identicalWithImmediacy',
    'essence = shiningWithinItself',
  ],
  predicates: [
    { name: 'infiniteDeterminateness', args: ['shine'] },
    { name: 'coincidesWithItself', args: ['negative'] },
    { name: 'selfSubsistence', args: ['determinateness'] },
    { name: 'identicalWithImmediacy', args: ['negativity'] },
    { name: 'shiningWithinItself', args: ['essence'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'infiniteDeterminateness' },
  ],
  candidateSummary: 'Determinateness that shine is in essence is infinite determinateness. Only negative which coincides with itself. Determinateness that, as determinateness, is self-subsistence and not determined. Contrariwise, self-subsistence as self-referring immediacy equally just determinateness and moment. Negativity solely referring to itself. Negativity identical with immediacy, immediacy identical with negativity: essence. Shine is essence itself, but essence in determinateness. Determinateness only a moment. Essence is shining of itself within itself.',
  provenance: {
    sourceChunk: 'shn-9',
    sourceOp: 'shn-op-9-infinite-determinateness',
  },
};

export const shnOp10EssenceAsReflection: LogicalOperation = {
  id: 'shn-op-10-essence-as-reflection',
  chunkId: 'shn-10',
  label: 'Comparison with sphere of being — essence as reflection',
  clauses: [
    'essence = infiniteSelfContainedMovement',
    'essence = determinesImmediacyAsNegativity',
    'essence = determinesNegativityAsImmediacy',
    'essence = reflection',
    'shine = leftoverOfBeing',
  ],
  predicates: [
    { name: 'infiniteSelfContainedMovement', args: ['essence'] },
    { name: 'determinesImmediacyAsNegativity', args: ['essence'] },
    { name: 'determinesNegativityAsImmediacy', args: ['essence'] },
    { name: 'reflection', args: ['essence'] },
    { name: 'leftoverOfBeing', args: ['shine'] },
  ],
  relations: [
    { predicate: 'is', from: 'essence', to: 'reflection' },
  ],
  candidateSummary: 'In sphere of being, non-being arises over against being, each immediate, truth is becoming. In sphere of essence: contrast of essence and non-essential, then essence and shine. Non-essential and shine both leftover of being. Distinction consists in: essence taken at first as immediate, not as it is in itself. As immediacy which is immediacy as pure mediacy or absolute negativity. First immediacy is only determinateness of immediacy. Sublating determinateness: showing unessential is only shine, essence contains shine within itself. Essence is infinite self-contained movement. Determines immediacy as negativity and negativity as immediacy. Shining of itself within itself. In this self-movement, essence is reflection.',
  provenance: {
    sourceChunk: 'shn-10',
    sourceOp: 'shn-op-10-essence-as-reflection',
  },
};

export const shineOperations: LogicalOperation[] = [
  shnOp1BeingIsShine,
  shnOp2RemainderOfBeing,
  shnOp3SkepticismIdealism,
  shnOp4ShineDeterminations,
  shnOp5EssenceNegativity,
  shnOp6TwoMoments,
  shnOp7EssenceInDeterminateness,
  shnOp8NegativeReturning,
  shnOp9InfiniteDeterminateness,
  shnOp10EssenceAsReflection,
];
