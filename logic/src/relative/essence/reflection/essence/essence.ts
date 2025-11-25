/**
 * Logical Operations: Essence
 *
 * Essence is the truth of Being. It is the movement of reflection, the turning back
 * of being into itself. It covers the determination of essence as such, shine, and reflection.
 *
 * Dialectical Movement:
 * - Essence as such: simple self-relation
 * - Shine: essence's own positing
 * - Reflection: movement of nothing to nothing
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// ESSENCE
// ============================================================================

export const essOp1TruthOfBeing: LogicalOperation = {
  id: 'ess-op-1-truth-of-being',
  chunkId: 'ess-1',
  label: 'Truth of being is essence — mediated knowledge',
  clauses: [
    'being = immediate',
    'knowledge = seeksTruth',
    'essence = truthOfBeing',
    'essence = mediated',
    'knowledge = recollectsItself',
    'movement = beingsOwnNature',
  ],
  predicates: [
    { name: 'immediate', args: ['being'] },
    { name: 'seeksTruth', args: ['knowledge'] },
    { name: 'truthOfBeing', args: ['essence'] },
    { name: 'mediated', args: ['essence'] },
    { name: 'recollectsItself', args: ['knowledge'] },
    { name: 'beingsOwnNature', args: ['movement'] },
  ],
  relations: [
    { predicate: 'is', from: 'essence', to: 'truthOfBeing' },
    { predicate: 'foundThrough', from: 'essence', to: 'mediation' },
  ],
  candidateSummary: 'Being is the immediate. Knowledge seeks truth beyond immediacy. Essence is the truth of being, found through mediation. Knowledge recollects itself from immediate being to find essence. German etymology: Wesen (essence) from gewesen (past being). This movement is being\'s own nature, not external cognition.',
  provenance: {
    sourceChunk: 'ess-1',
    sourceOp: 'ess-op-1-truth-of-being',
  },
};

export const essOp2SublatedBeing: LogicalOperation = {
  id: 'ess-op-2-sublated-being',
  chunkId: 'ess-2',
  label: 'Essence as sublated being — external abstraction critique',
  clauses: [
    'absolute = determinedAsEssence',
    'pureBeing = presupposesRecollection',
    'externalAbstraction = emptyArtifact',
    'essence = neitherInItselfNorForItself',
    'essence = deadAndEmpty',
  ],
  predicates: [
    { name: 'determinedAsEssence', args: ['absolute'] },
    { name: 'presupposesRecollection', args: ['pureBeing'] },
    { name: 'emptyArtifact', args: ['externalAbstraction'] },
    { name: 'neitherInItselfNorForItself', args: ['essence'] },
    { name: 'deadAndEmpty', args: ['essence'] },
  ],
  relations: [
    { predicate: 'determinedAs', from: 'absolute', to: 'essence' },
  ],
  candidateSummary: 'Absolute determined first as being, now as essence. Pure being presupposes recollection and movement. External abstraction produces only empty artifact. Essence through external reflection is neither in-itself nor for-itself. Dead and empty absence of determinateness.',
  provenance: {
    sourceChunk: 'ess-2',
    sourceOp: 'ess-op-2-sublated-being',
  },
};

export const essOp3AbsoluteNegativity: LogicalOperation = {
  id: 'ess-op-3-absolute-negativity',
  chunkId: 'ess-3',
  label: 'Essence as absolute negativity — being-in-and-for-itself',
  clauses: [
    'essence = absoluteNegativity',
    'essence = infiniteMovement',
    'essence = beingInAndForItself',
    'essence = indifferentToDeterminateness',
    'essence = selfSublation',
  ],
  predicates: [
    { name: 'absoluteNegativity', args: ['essence'] },
    { name: 'infiniteMovement', args: ['essence'] },
    { name: 'beingInAndForItself', args: ['essence'] },
    { name: 'indifferentToDeterminateness', args: ['essence'] },
    { name: 'selfSublation', args: ['essence'] },
  ],
  relations: [
    { predicate: 'is', from: 'essence', to: 'absoluteNegativity' },
  ],
  candidateSummary: 'Essence through its own negativity, infinite movement of being. Being-in-and-for-itself, absolute in-itselfness. Indifferent to every determinateness of being. Not only in-itself but also being-for-itself. Self-sublation of otherness and determinateness.',
  provenance: {
    sourceChunk: 'ess-3',
    sourceOp: 'ess-op-3-absolute-negativity',
  },
};

export const essOp4IndeterminateEssence: LogicalOperation = {
  id: 'ess-op-4-indeterminate-essence',
  chunkId: 'ess-4',
  label: 'Indeterminate essence — must differentiate into existence',
  clauses: [
    'essence = turningBackOfBeing',
    'essence = indeterminate',
    'essence = holdsDeterminations',
    'essence = mustPassIntoExistence',
    'essence = selfRepelling',
  ],
  predicates: [
    { name: 'turningBackOfBeing', args: ['essence'] },
    { name: 'indeterminate', args: ['essence'] },
    { name: 'holdsDeterminations', args: ['essence'] },
    { name: 'mustPassIntoExistence', args: ['essence'] },
    { name: 'selfRepelling', args: ['essence'] },
  ],
  relations: [
    { predicate: 'passesInto', from: 'essence', to: 'existence' },
  ],
  candidateSummary: 'Essence as complete turning back of being into itself. At first indeterminate essence, holds determinations without positing. Must pass over into existence, differentiate itself. Repelling of itself from itself, negative self-reference. Determinations of essence different from determinations of being.',
  provenance: {
    sourceChunk: 'ess-4',
    sourceOp: 'ess-op-4-indeterminate-essence',
  },
};

export const essOp5EssenceReflection: LogicalOperation = {
  id: 'ess-op-5-essence-reflection',
  chunkId: 'ess-5',
  label: 'Essence compared to quality and quantity — reflection',
  clauses: [
    'essence = absoluteIndifference',
    'determinateness = positedByEssence',
    'negativity = reflection',
    'determinations = reflected',
    'determinations = sublated',
  ],
  predicates: [
    { name: 'absoluteIndifference', args: ['essence'] },
    { name: 'positedByEssence', args: ['determinateness'] },
    { name: 'reflection', args: ['negativity'] },
    { name: 'reflected', args: ['determinations'] },
    { name: 'sublated', args: ['determinations'] },
  ],
  relations: [
    { predicate: 'is', from: 'negativity', to: 'reflection' },
  ],
  candidateSummary: 'Essence is to whole what quality was to being: absolute indifference to limit. Quantity is indifference in immediate determination. In essence, determinateness does not exist but is posited by essence. Negativity of essence is reflection. Determinations are reflected, posited by essence, remain as sublated.',
  provenance: {
    sourceChunk: 'ess-5',
    sourceOp: 'ess-op-5-essence-reflection',
  },
};

export const essOp6EssenceMiddleTerm: LogicalOperation = {
  id: 'ess-op-6-essence-middle-term',
  chunkId: 'ess-6',
  label: 'Essence as middle term — transition to concept',
  clauses: [
    'essence = middleTerm',
    'essence = betweenBeingAndConcept',
    'essence = beingInAndForItself',
    'essence = firstNegationOfBeing',
    'movement = positingNegation',
  ],
  predicates: [
    { name: 'middleTerm', args: ['essence'] },
    { name: 'betweenBeingAndConcept', args: ['essence'] },
    { name: 'beingInAndForItself', args: ['essence'] },
    { name: 'firstNegationOfBeing', args: ['essence'] },
    { name: 'positingNegation', args: ['movement'] },
  ],
  relations: [
    { predicate: 'standsBetween', from: 'essence', to: 'beingAndConcept' },
  ],
  candidateSummary: 'Essence stands between being and concept, makes up their middle. Essence is being-in-and-for-itself in determination of being-in-itself. First negation of being. Movement: positing negation/determination, giving itself existence. Becomes concept when existence equals being-in-itself. But essence\'s existence still distinct from concept\'s existence.',
  provenance: {
    sourceChunk: 'ess-6',
    sourceOp: 'ess-op-6-essence-middle-term',
  },
};

export const essOp7ThreefoldStructure: LogicalOperation = {
  id: 'ess-op-7-threefold-structure',
  chunkId: 'ess-7',
  label: 'Threefold structure — shine, appearance, actuality',
  clauses: [
    'essence = shinesWithinItself',
    'essence = appears',
    'essence = revealsItself',
    'first = simpleEssence',
    'second = emergingIntoExistence',
    'third = actuality',
  ],
  predicates: [
    { name: 'shinesWithinItself', args: ['essence'] },
    { name: 'appears', args: ['essence'] },
    { name: 'revealsItself', args: ['essence'] },
    { name: 'simpleEssence', args: ['first'] },
    { name: 'emergingIntoExistence', args: ['second'] },
    { name: 'actuality', args: ['third'] },
  ],
  relations: [
    { predicate: 'has', from: 'essence', to: 'threeDeterminations' },
  ],
  candidateSummary: 'First: essence shines within itself (reflection). Second: it appears. Third: it reveals itself. Three determinations: simple essence in itself; emerging into existence; essence one with appearance (actuality).',
  provenance: {
    sourceChunk: 'ess-7',
    sourceOp: 'ess-op-7-threefold-structure',
  },
};

export const essOp8EssenceAsReflectionWithin: LogicalOperation = {
  id: 'ess-op-8-essence-as-reflection-within',
  chunkId: 'ess-8',
  label: 'Essence as reflection within',
  clauses: [
    'essence = issuesFromBeing',
    'essence = sublatedBeing',
    'shine = standsOverAgainstEssence',
    'shine = essencesOwnPositing',
    'moments = reflectionReflectiveDeterminationsFoundation',
  ],
  predicates: [
    { name: 'issuesFromBeing', args: ['essence'] },
    { name: 'sublatedBeing', args: ['essence'] },
    { name: 'standsOverAgainstEssence', args: ['shine'] },
    { name: 'essencesOwnPositing', args: ['shine'] },
    { name: 'reflectionReflectiveDeterminationsFoundation', args: ['moments'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'essencesOwnPositing' },
  ],
  candidateSummary: 'Essence issues from being, result of movement. At first immediate, essential existence vs. unessential. Essence is sublated being; what stands over against it is shine. Shine is essence\'s own positing. Three moments: reflection; reflective determinations; foundation.',
  provenance: {
    sourceChunk: 'ess-8',
    sourceOp: 'ess-op-8-essence-as-reflection-within',
  },
};

export const essOp9ShineIntroduction: LogicalOperation = {
  id: 'ess-op-9-shine-introduction',
  chunkId: 'ess-9',
  label: 'Shine — introduction',
  clauses: [
    'essence = standsOverAgainstBeing',
    'immediateBeing = unessential',
    'shine = beingVoidOfEssence',
    'shine = essencesOwnShining',
  ],
  predicates: [
    { name: 'standsOverAgainstBeing', args: ['essence'] },
    { name: 'unessential', args: ['immediateBeing'] },
    { name: 'beingVoidOfEssence', args: ['shine'] },
    { name: 'essencesOwnShining', args: ['shine'] },
  ],
  relations: [
    { predicate: 'is', from: 'shine', to: 'essencesOwnShining' },
  ],
  candidateSummary: 'Essence seems to stand over against being. Immediate being is unessential. More than unessential: being void of essence, shine. Shine is essence\'s own shining, reflection.',
  provenance: {
    sourceChunk: 'ess-9',
    sourceOp: 'ess-op-9-shine-introduction',
  },
};

export const essOp10EssentialAndUnessential: LogicalOperation = {
  id: 'ess-op-10-essential-and-unessential',
  chunkId: 'ess-10',
  label: 'Essential and unessential — determined negation',
  clauses: [
    'essence = sublatedBeing',
    'essence = simpleEquality',
    'essence = determinedNegation',
    'being = unessential',
    'essence = essentialExistence',
  ],
  predicates: [
    { name: 'sublatedBeing', args: ['essence'] },
    { name: 'simpleEquality', args: ['essence'] },
    { name: 'determinedNegation', args: ['essence'] },
    { name: 'unessential', args: ['being'] },
    { name: 'essentialExistence', args: ['essence'] },
  ],
  relations: [
    { predicate: 'relateAs', from: 'beingAndEssence', to: 'mutuallyIndifferentOthers' },
  ],
  candidateSummary: 'Essence is sublated being, simple equality with itself. Negation of sphere of being in general. Has immediacy over against it, preserved in sublating. Essence is determined negation. Being and essence relate as mutually indifferent others. Being is unessential, essence is essential existence.',
  provenance: {
    sourceChunk: 'ess-10',
    sourceOp: 'ess-op-10-essential-and-unessential',
  },
};

export const essOp11RelapseIntoExistence: LogicalOperation = {
  id: 'ess-op-11-relapse-into-existence',
  chunkId: 'ess-11',
  label: 'Distinction relapses into existence — external positing',
  clauses: [
    'distinction = makesEssenceRelapse',
    'essence = determinedAsExistent',
    'distinguishing = externalPositing',
    'separation = fallsOnThird',
    'content = essentialOrUnessential',
  ],
  predicates: [
    { name: 'makesEssenceRelapse', args: ['distinction'] },
    { name: 'determinedAsExistent', args: ['essence'] },
    { name: 'externalPositing', args: ['distinguishing'] },
    { name: 'fallsOnThird', args: ['separation'] },
    { name: 'essentialOrUnessential', args: ['content'] },
  ],
  relations: [
    { predicate: 'relapsesInto', from: 'essence', to: 'existence' },
  ],
  candidateSummary: 'Distinction of essential/unessential makes essence relapse into existence. Essence determined as existent, therefore as other. Distinguishing is external positing, separation falling on third. Dependent on external standpoint. Same content can be essential or unessential.',
  provenance: {
    sourceChunk: 'ess-11',
    sourceOp: 'ess-op-11-relapse-into-existence',
  },
};

export const essOp12EssenceAsAbsoluteNegativity: LogicalOperation = {
  id: 'ess-op-12-essence-as-absolute-negativity',
  chunkId: 'ess-12',
  label: 'Essence as absolute negativity — shine',
  clauses: [
    'essence = absoluteNegativity',
    'essence = sublatedItself',
    'being = nonEssence',
    'being = shine',
    'immediate = null',
  ],
  predicates: [
    { name: 'absoluteNegativity', args: ['essence'] },
    { name: 'sublatedItself', args: ['essence'] },
    { name: 'nonEssence', args: ['being'] },
    { name: 'shine', args: ['being'] },
    { name: 'null', args: ['immediate'] },
  ],
  relations: [
    { predicate: 'is', from: 'being', to: 'shine' },
  ],
  candidateSummary: 'Essence only essential as contrasted with unessential because taken as sublated being. First negation: determinateness through which being becomes existence. But essence is absolute negativity of being. Has sublated itself as immediate being and immediate negation. Being does not persist except as what essence is. Immediate differing from essence is null in and for itself: non-essence, shine.',
  provenance: {
    sourceChunk: 'ess-12',
    sourceOp: 'ess-op-12-essence-as-absolute-negativity',
  },
};

export const essenceOperations: LogicalOperation[] = [
  essOp1TruthOfBeing,
  essOp2SublatedBeing,
  essOp3AbsoluteNegativity,
  essOp4IndeterminateEssence,
  essOp5EssenceReflection,
  essOp6EssenceMiddleTerm,
  essOp7ThreefoldStructure,
  essOp8EssenceAsReflectionWithin,
  essOp9ShineIntroduction,
  essOp10EssentialAndUnessential,
  essOp11RelapseIntoExistence,
  essOp12EssenceAsAbsoluteNegativity,
];
