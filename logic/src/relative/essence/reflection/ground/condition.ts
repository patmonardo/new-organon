/**
 * Logical Operations: Condition
 *
 * Condition is the third determination of ground. It covers the relatively unconditioned,
 * the absolutely unconditioned, and the procession of the fact into concrete existence.
 *
 * Dialectical Movement:
 * - Relatively unconditioned: condition as immediate
 * - Absolutely unconditioned: reflective shine
 * - Procession: absolutely unconditioned as absolute ground
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// CONDITION
// ============================================================================

export const conOp1RelativelyUnconditioned: LogicalOperation = {
  id: 'con-op-1-relatively-unconditioned',
  chunkId: 'con-1',
  label: 'a. The relatively unconditioned — condition as immediate',
  clauses: [
    'ground = immediate',
    'grounded = mediated',
    'ground = positingReflection',
    'ground = presupposingReflection',
    'mediation = groundsOwnDoing',
    'immediate = condition',
    'realGround = essentiallyConditioned',
  ],
  predicates: [
    { name: 'immediate', args: ['ground'] },
    { name: 'mediated', args: ['grounded'] },
    { name: 'positingReflection', args: ['ground'] },
    { name: 'presupposingReflection', args: ['ground'] },
    { name: 'groundsOwnDoing', args: ['mediation'] },
    { name: 'condition', args: ['immediate'] },
    { name: 'essentiallyConditioned', args: ['realGround'] },
  ],
  relations: [
    { predicate: 'refersTo', from: 'ground', to: 'condition' },
  ],
  candidateSummary: 'Ground is immediate, grounded is mediated. But ground is positing reflection, makes itself into positedness. Is presupposing reflection, refers itself to itself as to something sublated, to immediate through which it is itself mediated. Mediation, as advance from immediate to ground, is not external reflection but ground\'s own doing. Ground-connection as reflection into self-identity is just as essentially self-externalizing reflection. Immediate to which ground refers as to its essential presupposition is condition. Real ground is essentially conditioned.',
  provenance: {
    sourceChunk: 'con-1',
    sourceOp: 'con-op-1-relatively-unconditioned',
  },
};

export const conOp2ConditionThreeMoments: LogicalOperation = {
  id: 'con-op-2-condition-three-moments',
  chunkId: 'con-2',
  label: 'Condition — three moments',
  clauses: [
    'condition = immediateManifoldExistence',
    'condition = referredToOther',
    'condition = somethingPosited',
    'condition = presuppositionOfGround',
    'condition = contentOfGround',
    'condition = materialForGround',
    'condition = inItselfOfGround',
    'condition = unconditioned',
    'condition = externalToGround',
  ],
  predicates: [
    { name: 'immediateManifoldExistence', args: ['condition'] },
    { name: 'referredToOther', args: ['condition'] },
    { name: 'somethingPosited', args: ['condition'] },
    { name: 'presuppositionOfGround', args: ['condition'] },
    { name: 'contentOfGround', args: ['condition'] },
    { name: 'materialForGround', args: ['condition'] },
    { name: 'inItselfOfGround', args: ['condition'] },
    { name: 'unconditioned', args: ['condition'] },
    { name: 'externalToGround', args: ['condition'] },
  ],
  relations: [
    { predicate: 'constitutes', from: 'condition', to: 'presuppositionOfGround' },
  ],
  candidateSummary: 'Condition is, first, immediate, manifold existence. Second, existence referred to other, to something which is ground, not of this existence but in some other respect. According to reference, something posited. As condition, immediate existence supposed to be not for itself but for another. That it is for another is itself only positedness. Existence indifferent to being condition. Third, condition is immediate in sense that constitutes presupposition of ground. Form-connection of ground withdrawn into self-identity, hence content of ground. Condition is that in which ground-connection has its identity with itself, constitutes content of ground. But since content indifferent to form, only implicitly content of form, constitutes material for ground. Constitutes in-itself of ground, is for it unconditioned. Has presupposition in ground and is itself conditioned, but condition external to it.',
  provenance: {
    sourceChunk: 'con-2',
    sourceOp: 'con-op-2-condition-three-moments',
  },
};

export const conOp3SomethingHasConditionAndGround: LogicalOperation = {
  id: 'con-op-3-something-has-condition-and-ground',
  chunkId: 'con-3',
  label: 'Something has condition and ground',
  clauses: [
    'something = hasConditionAndGround',
    'ground = emptyMovementOfReflection',
    'ground = wholeForm',
    'groundConnection = selfSubsistingSelfReference',
    'groundContent = essentiallyInformed',
    'conditionContent = immediateMaterial',
    'reference = external',
  ],
  predicates: [
    { name: 'hasConditionAndGround', args: ['something'] },
    { name: 'emptyMovementOfReflection', args: ['ground'] },
    { name: 'wholeForm', args: ['ground'] },
    { name: 'selfSubsistingSelfReference', args: ['groundConnection'] },
    { name: 'essentiallyInformed', args: ['groundContent'] },
    { name: 'immediateMaterial', args: ['conditionContent'] },
    { name: 'external', args: ['reference'] },
  ],
  relations: [
    { predicate: 'has', from: 'something', to: 'conditionAndGround' },
  ],
  candidateSummary: 'Something is not through its condition, condition is not its ground. Condition is for ground moment of unconditioned immediacy. Over against condition stands ground-connection. Something has, besides condition, also ground. Ground is empty movement of reflection, whole form and self-subsistent process of mediation. Since mediating refers itself to itself as positing, equally is something immediate and unconditioned. Ground-connection is self-subsisting self-reference, has content peculiarly its own. One content is that of ground, essentially informed. Other content, that of condition, only immediate material. Connecting reference to ground, while constituting in-itself, also external to it.',
  provenance: {
    sourceChunk: 'con-3',
    sourceOp: 'con-op-3-something-has-condition-and-ground',
  },
};

export const conOp4TwoSides: LogicalOperation = {
  id: 'con-op-4-two-sides',
  chunkId: 'con-4',
  label: 'Two sides — indifferent and mediated',
  clauses: [
    'twoSides = indifferentAndUnconditioned',
    'twoSides = mediated',
    'condition = inItselfOfGround',
    'condition = simpleSelfIdentityOfGround',
    'groundConnection = hasInItselfOutsideItself',
    'eachSide = contradiction',
    'contradiction = independentSubsistenceAndMoments',
  ],
  predicates: [
    { name: 'indifferentAndUnconditioned', args: ['twoSides'] },
    { name: 'mediated', args: ['twoSides'] },
    { name: 'inItselfOfGround', args: ['condition'] },
    { name: 'simpleSelfIdentityOfGround', args: ['condition'] },
    { name: 'hasInItselfOutsideItself', args: ['groundConnection'] },
    { name: 'contradiction', args: ['eachSide'] },
    { name: 'independentSubsistenceAndMoments', args: ['contradiction'] },
  ],
  relations: [
    { predicate: 'is', from: 'eachSide', to: 'contradiction' },
  ],
  candidateSummary: 'Two sides of whole, condition and ground. On one hand, indifferent and unconditioned with respect to each other. One as non-referred-to side, other as connecting reference or form. On other hand, two sides also mediated. Condition is in-itself of ground, essential moment of ground-connection, simple self-identity of ground. But this also sublated, in-itself only something posited. Ground-connection has in self-subsistence also presupposition, has its in-itself outside itself. Each of two sides is contradiction: indifferent immediacy and essential mediation, both in one reference. Contradiction of independent subsistence and being determined as only moments.',
  provenance: {
    sourceChunk: 'con-4',
    sourceOp: 'con-op-4-two-sides',
  },
};

export const conOp5AbsolutelyUnconditioned: LogicalOperation = {
  id: 'con-op-5-absolutely-unconditioned',
  chunkId: 'con-5',
  label: 'b. The absolutely unconditioned — reflective shine',
  clauses: [
    'eachSide = reflectivelyShinesInOther',
    'condition = reflectedInFormConnection',
    'form = reflectedInImmediateExistence',
    'each = standsOutOnOwn',
    'each = hasContentOfOwn',
  ],
  predicates: [
    { name: 'reflectivelyShinesInOther', args: ['eachSide'] },
    { name: 'reflectedInFormConnection', args: ['condition'] },
    { name: 'reflectedInImmediateExistence', args: ['form'] },
    { name: 'standsOutOnOwn', args: ['each'] },
    { name: 'hasContentOfOwn', args: ['each'] },
  ],
  relations: [
    { predicate: 'shinesIn', from: 'eachSide', to: 'other' },
  ],
  candidateSummary: 'At first, each of two relatively unconditioned sides reflectively shines in other. Condition, as immediate, reflected in form connection of ground. Form in immediate existence as its positedness. But each, apart from reflective shine of other in it, stands out on own, has content of own.',
  provenance: {
    sourceChunk: 'con-5',
    sourceOp: 'con-op-5-absolutely-unconditioned',
  },
};

export const conOp6ConditionForm: LogicalOperation = {
  id: 'con-op-6-condition-form',
  chunkId: 'con-6',
  label: 'Condition\'s form — two moments',
  clauses: [
    'form = hasTwoMoments',
    'moments = positednessAndInItself',
    'existence = sublatesItself',
    'being = becomingOfEssence',
    'formDeterminations = notExternalToExistence',
    'existence = thisVeryReflection',
  ],
  predicates: [
    { name: 'hasTwoMoments', args: ['form'] },
    { name: 'positednessAndInItself', args: ['moments'] },
    { name: 'sublatesItself', args: ['existence'] },
    { name: 'becomingOfEssence', args: ['being'] },
    { name: 'notExternalToExistence', args: ['formDeterminations'] },
    { name: 'thisVeryReflection', args: ['existence'] },
  ],
  relations: [
    { predicate: 'is', from: 'being', to: 'becomingOfEssence' },
  ],
  candidateSummary: 'Condition is at first immediate existence. Form has two moments: positedness, according to which it is material and moment of ground; and in-itself, according to which constitutes essentiality of ground or simple reflection into itself. Both sides of form external to immediate existence. But existence is in it only this: sublate itself in immediacy and founder, going to ground. Being is as such only becoming of essence. Essential nature to make itself into positedness and identity which is immediacy through negation of itself. Form determinations are not external to existence, latter is this very reflection.',
  provenance: {
    sourceChunk: 'con-6',
    sourceOp: 'con-op-6-condition-form',
  },
};

export const conOp7ConditionAsWholeForm: LogicalOperation = {
  id: 'con-op-7-condition-as-whole-form',
  chunkId: 'con-7',
  label: 'Condition as whole form — ground',
  clauses: [
    'condition = momentOfOther',
    'inItself = throughNegationOfItself',
    'existence = immediateButMediated',
    'condition = wholeFormOfGroundConnection',
    'condition = foundersToGround',
    'condition = isGround',
    'ground = makesItselfIntoPositedness',
  ],
  predicates: [
    { name: 'momentOfOther', args: ['condition'] },
    { name: 'throughNegationOfItself', args: ['inItself'] },
    { name: 'immediateButMediated', args: ['existence'] },
    { name: 'wholeFormOfGroundConnection', args: ['condition'] },
    { name: 'foundersToGround', args: ['condition'] },
    { name: 'isGround', args: ['condition'] },
    { name: 'makesItselfIntoPositedness', args: ['ground'] },
  ],
  relations: [
    { predicate: 'is', from: 'condition', to: 'wholeFormOfGroundConnection' },
  ],
  candidateSummary: 'As condition, being posited as that which essentially is: moment and being of other, and in-itself of other. In itself but only through negation of itself, through ground and self-sublating presupposing reflection. In-itself of being only something posited. In-itself of condition has two sides: essentiality as essentiality of ground, immediacy of existence. Both sides same thing. Existence is immediate, but immediacy essentially mediated through self-sublating ground. Existence is at same time in-itself of ground and unconditioned side, but equally only moment or positedness. Condition is whole form of ground-connection. Presupposed in-itself, but consequently itself positedness. Both founders to ground and is ground. Ground that makes itself into positedness and thereby into grounded, both one and same.',
  provenance: {
    sourceChunk: 'con-7',
    sourceOp: 'con-op-7-condition-as-whole-form',
  },
};

export const conOp8ConditionedGround: LogicalOperation = {
  id: 'con-op-8-conditioned-ground',
  chunkId: 'con-8',
  label: 'Conditioned ground — whole itself',
  clauses: [
    'ground = selfSubsistent',
    'ground = selfReferringReflection',
    'ground = positsInItselfAsOther',
    'condition = groundConnectionsOwnMoment',
    'immediateExistence = essentiallyThroughGround',
    'ground = wholeItself',
  ],
  predicates: [
    { name: 'selfSubsistent', args: ['ground'] },
    { name: 'selfReferringReflection', args: ['ground'] },
    { name: 'positsInItselfAsOther', args: ['ground'] },
    { name: 'groundConnectionsOwnMoment', args: ['condition'] },
    { name: 'essentiallyThroughGround', args: ['immediateExistence'] },
    { name: 'wholeItself', args: ['ground'] },
  ],
  relations: [
    { predicate: 'is', from: 'ground', to: 'wholeItself' },
  ],
  candidateSummary: 'In conditioned ground, in-itself not just reflective shining of other in it. Ground is self-subsistent, self-referring reflection of positing, self-identical. In it its in-itself and content. But at same time presupposing reflection. Negatively refers to itself, posits in-itself as other opposite to it. Condition, according to both moment of in-itself and immediate existence, is ground-connection\'s own moment. Immediate existence essentially only through ground, moment of itself as presupposing. Ground is equally whole itself.',
  provenance: {
    sourceChunk: 'con-8',
    sourceOp: 'con-op-8-conditioned-ground',
  },
};

export const conOp9OneWhole: LogicalOperation = {
  id: 'con-op-9-one-whole',
  chunkId: 'con-9',
  label: 'One whole of form and content',
  clauses: [
    'oneWhole = formAndContent',
    'properContentOfCondition = selfIdentityOfReflection',
    'existence = informedMatter',
    'existence = content',
    'content = sameAsGround',
    'content = selfIdenticalInFormConnection',
  ],
  predicates: [
    { name: 'formAndContent', args: ['oneWhole'] },
    { name: 'selfIdentityOfReflection', args: ['properContentOfCondition'] },
    { name: 'informedMatter', args: ['existence'] },
    { name: 'content', args: ['existence'] },
    { name: 'sameAsGround', args: ['content'] },
    { name: 'selfIdenticalInFormConnection', args: ['content'] },
  ],
  relations: [
    { predicate: 'is', from: 'existence', to: 'content' },
  ],
  candidateSummary: 'Only one whole of form, equally only one whole of content. Proper content of condition is essential content only in so far as self-identity of reflection in form. Or ground-connection is in it immediate existence. Existence is condition only through presupposing reflection of ground. Ground\'s self-identity, or content, to which ground posits itself as opposite. Existence not formless material, because has form in it, informed matter. In identity with it at same time indifferent to it, is content. Same content as possessed by ground, content as self-identical in form connection.',
  provenance: {
    sourceChunk: 'con-9',
    sourceOp: 'con-op-9-one-whole',
  },
};

export const conOp10OneEssentialUnity: LogicalOperation = {
  id: 'con-op-10-one-essential-unity',
  chunkId: 'con-10',
  label: 'One essential unity — absolutely unconditioned',
  clauses: [
    'twoSides = oneEssentialUnity',
    'twoSides = presupposeOneIdentity',
    'substrate = trulyUnconditioned',
    'substrate = factInItself',
    'condition = relativelyUnconditioned',
    'condition = sublatedInAbsolutelyUnconditioned',
  ],
  predicates: [
    { name: 'oneEssentialUnity', args: ['twoSides'] },
    { name: 'presupposeOneIdentity', args: ['twoSides'] },
    { name: 'trulyUnconditioned', args: ['substrate'] },
    { name: 'factInItself', args: ['substrate'] },
    { name: 'relativelyUnconditioned', args: ['condition'] },
    { name: 'sublatedInAbsolutelyUnconditioned', args: ['condition'] },
  ],
  relations: [
    { predicate: 'is', from: 'substrate', to: 'trulyUnconditioned' },
  ],
  candidateSummary: 'Two sides of whole, condition and ground, one essential unity, as content as well as form. Pass into one another, posit themselves as sublated, reciprocally presuppose each other. But only one reflection of two, presupposing one presupposing only. Reciprocity amounts to both presuppose one identity for subsistence and substrate. Substrate, one content and unity of form of both, is truly unconditioned, fact in itself. Condition only relatively unconditioned. Usual to consider as itself conditioned, ask for new condition, progression ad infinitum. But condition as such conditioned solely because posited in-itselfness. Sublated in absolutely unconditioned.',
  provenance: {
    sourceChunk: 'con-10',
    sourceOp: 'con-op-10-one-essential-unity',
  },
};

export const conOp11AbsolutelyUnconditionedFact: LogicalOperation = {
  id: 'con-op-11-absolutely-unconditioned-fact',
  chunkId: 'con-11',
  label: 'Absolutely unconditioned — fact in itself',
  clauses: [
    'absolutelyUnconditioned = containsConditionAndGround',
    'unconditionedFact = conditionOfBoth',
    'unconditionedFact = itselfGround',
    'twoSides = presupposeTotality',
    'relation = disappeared',
    'relation = reducedToReflectiveShine',
    'fact = reflectionShiningInItself',
  ],
  predicates: [
    { name: 'containsConditionAndGround', args: ['absolutelyUnconditioned'] },
    { name: 'conditionOfBoth', args: ['unconditionedFact'] },
    { name: 'itselfGround', args: ['unconditionedFact'] },
    { name: 'presupposeTotality', args: ['twoSides'] },
    { name: 'disappeared', args: ['relation'] },
    { name: 'reducedToReflectiveShine', args: ['relation'] },
    { name: 'reflectionShiningInItself', args: ['fact'] },
  ],
  relations: [
    { predicate: 'is', from: 'unconditionedFact', to: 'conditionOfBoth' },
  ],
  candidateSummary: 'Absolutely unconditioned contains two sides, condition and ground, as moments. Unity to which they returned. Together constitute its form or positedness. Unconditioned fact is condition of both, but condition which is absolute, one which is itself ground. As ground, fact is negative identity repelled into two moments: sublated ground-connection, immediate manifold void of unity; and inner simple form which is ground. Two sides presuppose totality, presuppose it posits them. But since two sides identity, relation of condition and ground disappeared. Two reduced to mere reflective shine. Absolutely unconditioned in movement of positing and presupposing only movement in which shine sublates itself. Fact\'s own doing that conditions itself and places itself as ground. In connecting conditions and ground, fact reflection shining in itself, relation to them rejoining itself.',
  provenance: {
    sourceChunk: 'con-11',
    sourceOp: 'con-op-11-absolutely-unconditioned-fact',
  },
};

export const conOp12Procession: LogicalOperation = {
  id: 'con-op-12-procession',
  chunkId: 'con-12',
  label: 'c. Procession — absolutely unconditioned as absolute ground',
  clauses: [
    'absolutelyUnconditioned = absoluteGround',
    'absolutelyUnconditioned = identicalWithCondition',
    'ground = makesItselfIntoPositedness',
    'positedness = reflectionCompleteInBothSides',
    'positedness = sublatedGround',
  ],
  predicates: [
    { name: 'absoluteGround', args: ['absolutelyUnconditioned'] },
    { name: 'identicalWithCondition', args: ['absolutelyUnconditioned'] },
    { name: 'makesItselfIntoPositedness', args: ['ground'] },
    { name: 'reflectionCompleteInBothSides', args: ['positedness'] },
    { name: 'sublatedGround', args: ['positedness'] },
  ],
  relations: [
    { predicate: 'is', from: 'absolutelyUnconditioned', to: 'absoluteGround' },
  ],
  candidateSummary: 'Absolutely unconditioned is absolute ground identical with condition, immediate fact as truly essential. As ground, refers negatively to itself, makes itself into positedness. Positedness is reflection complete in both sides, self-identical form of connection. Positedness is first sublated ground, fact as immediacy void of reflection, side of conditions.',
  provenance: {
    sourceChunk: 'con-12',
    sourceOp: 'con-op-12-procession',
  },
};

export const conOp13Conditions: LogicalOperation = {
  id: 'con-op-13-conditions',
  chunkId: 'con-13',
  label: 'Conditions — totality of determinations',
  clauses: [
    'conditions = totalityOfDeterminations',
    'conditions = wholeContentOfFact',
    'sphereOfBeing = condition',
    'existence = makesItselfIntoMoment',
    'truthOfExistence = condition',
    'immediacy = momentOfForm',
  ],
  predicates: [
    { name: 'totalityOfDeterminations', args: ['conditions'] },
    { name: 'wholeContentOfFact', args: ['conditions'] },
    { name: 'condition', args: ['sphereOfBeing'] },
    { name: 'makesItselfIntoMoment', args: ['existence'] },
    { name: 'condition', args: ['truthOfExistence'] },
    { name: 'momentOfForm', args: ['immediacy'] },
  ],
  relations: [
    { predicate: 'is', from: 'truthOfExistence', to: 'condition' },
  ],
  candidateSummary: 'Totality of determinations of fact, fact as thrown into externality of being, restored circle of being. In condition, essence lets go unity of immanent reflection. Conditions are whole content of fact, unconditioned in form of formless being. Appear as manifold without unity, mingled with extra-essential elements. For absolute fact, sphere of being itself is condition. Ground posits sphere as first immediacy. Immediacy, as sublated reflection, is reflection in element of being. Form proliferates as determinateness of being. Existence that constitutes conditions not determined as condition by other, itself makes itself into moment of other. Becoming does not start from itself, immediacy only presupposed, movement doing of reflection itself. Truth of existence is that it is condition. Immediacy essentially only moment of form.',
  provenance: {
    sourceChunk: 'con-13',
    sourceOp: 'con-op-13-conditions',
  },
};

export const conOp14GroundConnectionForm: LogicalOperation = {
  id: 'con-op-14-ground-connection-form',
  chunkId: 'con-14',
  label: 'Ground-connection — form',
  clauses: [
    'groundConnection = determinedAsForm',
    'groundConnection = sublatesImmediacy',
    'positing = convertsIntoBecoming',
    'selfMediation = throughNegation',
    'mediation = groundless',
    'mediation = absoluteBecoming',
  ],
  predicates: [
    { name: 'determinedAsForm', args: ['groundConnection'] },
    { name: 'sublatesImmediacy', args: ['groundConnection'] },
    { name: 'convertsIntoBecoming', args: ['positing'] },
    { name: 'throughNegation', args: ['selfMediation'] },
    { name: 'groundless', args: ['mediation'] },
    { name: 'absoluteBecoming', args: ['mediation'] },
  ],
  relations: [
    { predicate: 'is', from: 'mediation', to: 'absoluteBecoming' },
  ],
  candidateSummary: 'Other side is ground-connection as such, determined as form. Form of absolute fact that possesses unity of form with itself or content within it. In determining content as condition, sublates diversity of content, reduces to moment. Reflection of ground sublates immediacy of conditions, connecting them, making moments within unity of fact. But conditions are that which unconditioned fact presupposes. Latter sublates own positing, positing converts immediately into becoming. Two one unity: internal movement is becoming, return into ground and positing of ground. Ground refers negatively to itself, makes itself into positedness, grounds conditions. Reflection is self-mediation of unconditioned fact through its negation. Reflection is presupposing, sublating of itself immediately positing. In positing, immediately sublating of presupposed. Mediation as turning back through negation has disappeared. Mediation is simple reflection reflectively shining within itself, groundless, absolute becoming. Process is coming forth, simple self-staging of fact in concrete existence, pure movement of fact to itself.',
  provenance: {
    sourceChunk: 'con-14',
    sourceOp: 'con-op-14-ground-connection-form',
  },
};

export const conOp15AllConditionsAtHand: LogicalOperation = {
  id: 'con-op-15-all-conditions-at-hand',
  chunkId: 'con-15',
  label: 'All conditions at hand — concrete existence',
  clauses: [
    'fact = stepsIntoConcreteExistence',
    'presupposedUnconditioned = groundlessImmediate',
    'scatteredManifold = recollectsItself',
    'ground = sublated',
    'ground = reflectiveShine',
    'comingForth = mediatedByDisappearingOfMediation',
  ],
  predicates: [
    { name: 'stepsIntoConcreteExistence', args: ['fact'] },
    { name: 'groundlessImmediate', args: ['presupposedUnconditioned'] },
    { name: 'recollectsItself', args: ['scatteredManifold'] },
    { name: 'sublated', args: ['ground'] },
    { name: 'reflectiveShine', args: ['ground'] },
    { name: 'mediatedByDisappearingOfMediation', args: ['comingForth'] },
  ],
  relations: [
    { predicate: 'stepsInto', from: 'fact', to: 'concreteExistence' },
  ],
  candidateSummary: 'When all conditions of fact at hand, fact steps into concrete existence. Fact is, before exists concretely: as essence or unconditioned, and immediate existence or determined. In former case, given itself form of external, groundless being. As absolute reflection, makes itself into presupposition. Presupposed unconditioned is groundless immediate whose being is to be there, without grounds. If all conditions at hand, scattered manifold internally recollects itself. Whole fact must be there within conditions. All conditions belong to concrete existence. Recollecting is foundering to ground and coming to be of ground. Ground is posited ground, to extent ground, to that extent sublated as ground, is immediate being. If all conditions at hand, sublate themselves, ground equally sublated. Latter proves only reflective shine that immediately disappears. Coming forth is tautological movement of fact to itself. Mediation through conditions and ground is disappearing of both. Coming forth so immediate, mediated only by disappearing of mediation.',
  provenance: {
    sourceChunk: 'con-15',
    sourceOp: 'con-op-15-all-conditions-at-hand',
  },
};

export const conOp16FactProceedsFromGround: LogicalOperation = {
  id: 'con-op-16-fact-proceeds-from-ground',
  chunkId: 'con-16',
  label: 'Fact proceeds from ground — concrete existence',
  clauses: [
    'fact = proceedsFromGround',
    'positing = disappearingOfGround',
    'ground = makesItselfIntoPositedness',
    'ground = rejoinsItself',
    'truthOfGrounding = groundUnitesWithItself',
    'fact = unconditioned',
    'fact = groundless',
    'immediacy = concreteExistence',
  ],
  predicates: [
    { name: 'proceedsFromGround', args: ['fact'] },
    { name: 'disappearingOfGround', args: ['positing'] },
    { name: 'makesItselfIntoPositedness', args: ['ground'] },
    { name: 'rejoinsItself', args: ['ground'] },
    { name: 'groundUnitesWithItself', args: ['truthOfGrounding'] },
    { name: 'unconditioned', args: ['fact'] },
    { name: 'groundless', args: ['fact'] },
    { name: 'concreteExistence', args: ['immediacy'] },
  ],
  relations: [
    { predicate: 'is', from: 'immediacy', to: 'concreteExistence' },
  ],
  candidateSummary: 'Fact proceeds from ground. Not grounded in manner ground stays underneath as substrate. Positing is outward movement of ground to itself and simple disappearing of it. Through union with conditions, obtains external immediacy and moment of being. As ground makes itself into positedness. Simple essentiality rejoins itself in positedness. In sublating itself, disappearing of difference from positedness, simple essential immediacy. Truth of grounding is in grounding ground unites with itself. Reflection into another is reflection into itself. Fact is unconditioned and, as such, groundless. Arises from ground only in so far as latter foundered and no longer ground. Rises up from groundless, from own essential negativity or pure form. Immediacy, mediated by ground and condition and self-identical through sublating of mediation, is concrete existence.',
  provenance: {
    sourceChunk: 'con-16',
    sourceOp: 'con-op-16-fact-proceeds-from-ground',
  },
};

export const conditionGroundOperations: LogicalOperation[] = [
  conOp1RelativelyUnconditioned,
  conOp2ConditionThreeMoments,
  conOp3SomethingHasConditionAndGround,
  conOp4TwoSides,
  conOp5AbsolutelyUnconditioned,
  conOp6ConditionForm,
  conOp7ConditionAsWholeForm,
  conOp8ConditionedGround,
  conOp9OneWhole,
  conOp10OneEssentialUnity,
  conOp11AbsolutelyUnconditionedFact,
  conOp12Procession,
  conOp13Conditions,
  conOp14GroundConnectionForm,
  conOp15AllConditionsAtHand,
  conOp16FactProceedsFromGround,
];
