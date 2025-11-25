/**
 * Logical Operations: Absolute Ground
 *
 * Absolute Ground is the first determination of ground. It covers form and essence,
 * form and matter, and form and content.
 *
 * Dialectical Movement:
 * - Form and essence: ground and grounded
 * - Form and matter: essence becomes matter
 * - Form and content: form stands over against content
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// ABSOLUTE GROUND
// ============================================================================

export const absOp1FormAndEssence: LogicalOperation = {
  id: 'abs-op-1-form-and-essence',
  chunkId: 'abs-1',
  label: 'a. Form and essence — ground and grounded',
  clauses: [
    'determinationOfReflection = returnsIntoGround',
    'existence = positedness',
    'existence = presupposesGround',
    'positing = sublatingOfItself',
    'immediate = posited',
    'ground = nonPosited',
    'ground = essenceDeterminedThroughItself',
    'ground = indeterminatePositedness',
    'essence = identicalWithItself',
    'determinatenessOfEssence = twofold',
    'determinateness = groundAndGrounded',
    'selfIdenticalNegative = selfIdenticalPositive',
  ],
  predicates: [
    { name: 'returnsIntoGround', args: ['determinationOfReflection'] },
    { name: 'positedness', args: ['existence'] },
    { name: 'presupposesGround', args: ['existence'] },
    { name: 'sublatingOfItself', args: ['positing'] },
    { name: 'posited', args: ['immediate'] },
    { name: 'nonPosited', args: ['ground'] },
    { name: 'essenceDeterminedThroughItself', args: ['ground'] },
    { name: 'indeterminatePositedness', args: ['ground'] },
    { name: 'identicalWithItself', args: ['essence'] },
    { name: 'twofold', args: ['determinatenessOfEssence'] },
    { name: 'groundAndGrounded', args: ['determinateness'] },
    { name: 'selfIdenticalPositive', args: ['selfIdenticalNegative'] },
  ],
  relations: [
    { predicate: 'presupposes', from: 'existence', to: 'ground' },
    { predicate: 'is', from: 'selfIdenticalNegative', to: 'selfIdenticalPositive' },
  ],
  candidateSummary: 'Determination of reflection returns into ground. First immediate existence from which beginning is made. Existence has meaning of positedness, presupposes ground. Positing is sublating of itself, immediate is posited, ground is non-posited. Ground is essence determined through itself, as indeterminate or sublated positedness. Essence in its negativity identical with itself. Determinateness of essence as ground is twofold: ground and grounded. Essence as ground: essence determined to be essence as against positedness, as non-positedness. Grounded: immediate that is not anything in and for itself, positedness as positedness. Self-identical negative and self-identical positive are one and same identity.',
  provenance: {
    sourceChunk: 'abs-1',
    sourceOp: 'abs-op-1-form-and-essence',
  },
};

export const absOp2MediationOfGround: LogicalOperation = {
  id: 'abs-op-2-mediation-of-ground',
  chunkId: 'abs-2',
  label: 'Mediation of ground — unity of pure and determining reflection',
  clauses: [
    'mediation = unityOfPureAndDeterminingReflection',
    'pureReflection = undistinguishedFromEssence',
    'determiningReflection = foundered',
    'determinations = positedDeterminations',
    'determinations = haveSelfSubsistence',
    'selfSubsistence = positedSubsistence',
    'determinations = constituteForm',
  ],
  predicates: [
    { name: 'unityOfPureAndDeterminingReflection', args: ['mediation'] },
    { name: 'undistinguishedFromEssence', args: ['pureReflection'] },
    { name: 'foundered', args: ['determiningReflection'] },
    { name: 'positedDeterminations', args: ['determinations'] },
    { name: 'haveSelfSubsistence', args: ['determinations'] },
    { name: 'positedSubsistence', args: ['selfSubsistence'] },
    { name: 'constituteForm', args: ['determinations'] },
  ],
  relations: [
    { predicate: 'is', from: 'mediation', to: 'unityOfPureAndDeterminingReflection' },
  ],
  candidateSummary: 'Mediation compared with preceding reflections. Not pure reflection: undistinguished from essence, no negative, no self-subsistence. Not determining reflection: determinations have essential self-subsistence, but reflection foundered, sunk to ground. In unity of ground, determinations are only posited determinations. Mediation of ground is unity of pure reflection and determining reflection. Determinations or posited have self-subsistence. Self-subsistence of determinations is posited subsistence. Since subsistence is itself posited or has determinateness, determinations distinguished from simple identity. They constitute form as against essence.',
  provenance: {
    sourceChunk: 'abs-2',
    sourceOp: 'abs-op-2-mediation-of-ground',
  },
};

export const absOp3EssenceHasForm: LogicalOperation = {
  id: 'abs-op-3-essence-has-form',
  chunkId: 'abs-3',
  label: 'Essence has form — substrate and movement',
  clauses: [
    'essence = hasForm',
    'essence = oneWithReflection',
    'essence = inseparableFromMovement',
    'essence = neitherBeforeNorInMovement',
    'movement = hasNoSubstrate',
    'termOfReference = arisesInGround',
    'essence = determinateEssence',
    'essence = hasFormAsEssence',
    'determinationsOfForm = determinationsInEssence',
    'essence = foundation',
    'essence = indeterminate',
  ],
  predicates: [
    { name: 'hasForm', args: ['essence'] },
    { name: 'oneWithReflection', args: ['essence'] },
    { name: 'inseparableFromMovement', args: ['essence'] },
    { name: 'neitherBeforeNorInMovement', args: ['essence'] },
    { name: 'hasNoSubstrate', args: ['movement'] },
    { name: 'arisesInGround', args: ['termOfReference'] },
    { name: 'determinateEssence', args: ['essence'] },
    { name: 'hasFormAsEssence', args: ['essence'] },
    { name: 'determinationsInEssence', args: ['determinationsOfForm'] },
    { name: 'foundation', args: ['essence'] },
    { name: 'indeterminate', args: ['essence'] },
  ],
  relations: [
    { predicate: 'has', from: 'essence', to: 'form' },
    { predicate: 'liesAt', from: 'essence', to: 'foundation' },
  ],
  candidateSummary: 'Essence has form and determinations of this form. Only as ground does it have fixed immediacy or is substrate. Essence as such is one with its reflection, inseparable from its movement. Not essence which movement runs its reflective course. Nor essence that from which movement begins. Cannot say essence returns into itself, essence shines in itself. Essence is neither before its movement nor in the movement. Movement has no substrate on which it runs its course. Term of reference arises in ground only following moment of sublated reflection. Essence as referred-to term is determinate essence. By virtue of positedness it has form as essence. Determinations of form are determinations in essence. Essence lies at their foundation as indeterminate, indifferent to them.',
  provenance: {
    sourceChunk: 'abs-3',
    sourceOp: 'abs-op-3-essence-has-form',
  },
};

export const absOp4FormDeterminations: LogicalOperation = {
  id: 'abs-op-4-form-determinations',
  chunkId: 'abs-4',
  label: 'Form determinations — everything determinate belongs to form',
  clauses: [
    'everythingDeterminate = belongsToForm',
    'formDetermination = posited',
    'formDeterminations = momentsOfReflections',
    'groundConnection = belongsToFormDeterminations',
    'identity = doesNotPertainToForm',
    'positedness = oneReflection',
    'reflection = constitutesEssence',
    'essence = simpleSubstrate',
    'essence = subsistenceOfForm',
    'essence = determinate',
    'essence = momentOfGroundConnection',
    'reference = reciprocal',
  ],
  predicates: [
    { name: 'belongsToForm', args: ['everythingDeterminate'] },
    { name: 'posited', args: ['formDetermination'] },
    { name: 'momentsOfReflections', args: ['formDeterminations'] },
    { name: 'belongsToFormDeterminations', args: ['groundConnection'] },
    { name: 'doesNotPertainToForm', args: ['identity'] },
    { name: 'oneReflection', args: ['positedness'] },
    { name: 'constitutesEssence', args: ['reflection'] },
    { name: 'simpleSubstrate', args: ['essence'] },
    { name: 'subsistenceOfForm', args: ['essence'] },
    { name: 'determinate', args: ['essence'] },
    { name: 'momentOfGroundConnection', args: ['essence'] },
    { name: 'reciprocal', args: ['reference'] },
  ],
  relations: [
    { predicate: 'belongsTo', from: 'everythingDeterminate', to: 'form' },
    { predicate: 'constitutes', from: 'reflection', to: 'essence' },
  ],
  candidateSummary: 'Everything determinate belongs in general to form. Form determination is something posited, distinguished from that of which it is form. Form determinations of essence are previously considered moments of reflections: identity and difference, latter as diversity and opposition. Ground-connection belongs among form determinations. Identity that has ground immanent in it does not pertain to form. Positedness as sublated (as ground and grounded) is one reflection. This reflection constitutes essence as simple substrate which is subsistence of form. But in ground this subsistence is posited. Essence is itself essentially as determinate, is moment of ground-connection and form. Absolute reciprocal connecting reference of form and essence.',
  provenance: {
    sourceChunk: 'abs-4',
    sourceOp: 'abs-op-4-form-determinations',
  },
};

export const absOp5FormAsCompletedWhole: LogicalOperation = {
  id: 'abs-op-5-form-as-completed-whole',
  chunkId: 'abs-5',
  label: 'Form as completed whole — absolute negativity',
  clauses: [
    'form = completedWholeOfReflection',
    'form = containsDeterminationOfReflection',
    'form = positingAndDetermining',
    'form = absoluteNegativity',
    'form = negativeAbsoluteSelfIdentity',
    'essence = notBeingButEssence',
    'form = internalReflectiveShiningOfEssence',
    'form = groundOfItsSublating',
    'form = contradiction',
  ],
  predicates: [
    { name: 'completedWholeOfReflection', args: ['form'] },
    { name: 'containsDeterminationOfReflection', args: ['form'] },
    { name: 'positingAndDetermining', args: ['form'] },
    { name: 'absoluteNegativity', args: ['form'] },
    { name: 'negativeAbsoluteSelfIdentity', args: ['form'] },
    { name: 'notBeingButEssence', args: ['essence'] },
    { name: 'internalReflectiveShiningOfEssence', args: ['form'] },
    { name: 'groundOfItsSublating', args: ['form'] },
    { name: 'contradiction', args: ['form'] },
  ],
  relations: [
    { predicate: 'is', from: 'form', to: 'absoluteNegativity' },
  ],
  candidateSummary: 'Form is completed whole of reflection. Contains determination of reflection that it is sublated. Referred to its sublatedness, to another that is not itself form but in which form is. As essential self-referring negativity, form is positing and determining. Simple essence is indeterminate and inert substrate. Form is absolute negativity itself or negative absolute self-identity. By virtue of which essence is not being but essence. Form has essence in its own identity, essence has absolute form in its negative nature. Cannot ask how form comes to essence. Form is only internal reflective shining of essence, its own reflection inhabiting it. Form does not determine essence as if truly presupposed, separate from essence. Form is itself ground of its sublating or identical reference of its determinations. Form is contradiction of being sublated in positedness and yet having subsistence in sublatedness.',
  provenance: {
    sourceChunk: 'abs-5',
    sourceOp: 'abs-op-5-form-as-completed-whole',
  },
};

export const absOp6FormDeterminesEssence: LogicalOperation = {
  id: 'abs-op-6-form-determines-essence',
  chunkId: 'abs-6',
  label: 'Form determines essence — matter',
  clauses: [
    'distinctions = momentsOfSimpleReference',
    'determiningForm = refersToItself',
    'determiningForm = positsItselfAsSublated',
    'essence = indeterminate',
    'form = otherToEssence',
    'essence = formlessIdentity',
    'essence = matter',
  ],
  predicates: [
    { name: 'momentsOfSimpleReference', args: ['distinctions'] },
    { name: 'refersToItself', args: ['determiningForm'] },
    { name: 'positsItselfAsSublated', args: ['determiningForm'] },
    { name: 'indeterminate', args: ['essence'] },
    { name: 'otherToEssence', args: ['form'] },
    { name: 'formlessIdentity', args: ['essence'] },
    { name: 'matter', args: ['essence'] },
  ],
  relations: [
    { predicate: 'is', from: 'essence', to: 'matter' },
  ],
  candidateSummary: 'Distinctions of form and essence are only moments of simple reference of form itself. Determining form refers itself to itself as sublated positedness. Refers itself to its identity as to another. Posits itself as sublated, therefore presupposes its identity. According to this moment, essence is indeterminate to which form is other. Not essence which is absolute reflection within. But essence determined as formless identity: it is matter.',
  provenance: {
    sourceChunk: 'abs-6',
    sourceOp: 'abs-op-6-form-determines-essence',
  },
};

export const absOp7FormAndMatter: LogicalOperation = {
  id: 'abs-op-7-form-and-matter',
  chunkId: 'abs-7',
  label: 'b. Form and matter — essence becomes matter',
  clauses: [
    'essence = becomesMatter',
    'matter = simpleIdentity',
    'matter = voidOfDistinction',
    'matter = otherOfForm',
    'matter = substrateOfForm',
    'matter = immanentReflectionOfDeterminations',
    'matter = absolutelyAbstract',
    'abstraction = notExternalRemoval',
    'form = reducesItselfToSimpleIdentity',
  ],
  predicates: [
    { name: 'becomesMatter', args: ['essence'] },
    { name: 'simpleIdentity', args: ['matter'] },
    { name: 'voidOfDistinction', args: ['matter'] },
    { name: 'otherOfForm', args: ['matter'] },
    { name: 'substrateOfForm', args: ['matter'] },
    { name: 'immanentReflectionOfDeterminations', args: ['matter'] },
    { name: 'absolutelyAbstract', args: ['matter'] },
    { name: 'notExternalRemoval', args: ['abstraction'] },
    { name: 'reducesItselfToSimpleIdentity', args: ['form'] },
  ],
  relations: [
    { predicate: 'becomes', from: 'essence', to: 'matter' },
  ],
  candidateSummary: 'Essence becomes matter in that its reflection is determined as relating itself to essence as to formless indeterminate. Matter is simple identity, void of distinction, that essence is. With determination that it is other of form. Proper base or substrate of form. Constitutes immanent reflection of determinations of form. Self-subsistent term to which determinations refer as to their positive subsistence. If abstraction made from every determination, from every form, matter is what is left over. Matter is absolutely abstract. Abstraction from which matter derives is not external removal and sublation of form. Form itself reduces itself by virtue of itself to this simple identity.',
  provenance: {
    sourceChunk: 'abs-7',
    sourceOp: 'abs-op-7-form-and-matter',
  },
};

export const absOp8FormAndMatterPresuppose: LogicalOperation = {
  id: 'abs-op-8-form-and-matter-presuppose',
  chunkId: 'abs-8',
  label: 'Form and matter presuppose each other — reciprocal reference',
  clauses: [
    'formAndMatter = presupposeEachOther',
    'neither = derivesFromItself',
    'matter = indifferentToForm',
    'form = presupposesMatter',
    'matter = notSimpleEssence',
    'matter = essenceDeterminedAsPositive',
    'matter = groundlessSubsistence',
    'matter = notGroundOfForm',
    'matter = identityOfGroundAndGrounded',
    'matter = passive',
    'form = active',
    'matter = mustBeInformed',
    'form = mustMaterializeItself',
  ],
  predicates: [
    { name: 'presupposeEachOther', args: ['formAndMatter'] },
    { name: 'derivesFromItself', args: ['neither'] },
    { name: 'indifferentToForm', args: ['matter'] },
    { name: 'presupposesMatter', args: ['form'] },
    { name: 'notSimpleEssence', args: ['matter'] },
    { name: 'essenceDeterminedAsPositive', args: ['matter'] },
    { name: 'groundlessSubsistence', args: ['matter'] },
    { name: 'notGroundOfForm', args: ['matter'] },
    { name: 'identityOfGroundAndGrounded', args: ['matter'] },
    { name: 'passive', args: ['matter'] },
    { name: 'active', args: ['form'] },
    { name: 'mustBeInformed', args: ['matter'] },
    { name: 'mustMaterializeItself', args: ['form'] },
  ],
  relations: [
    { predicate: 'presupposes', from: 'form', to: 'matter' },
  ],
  candidateSummary: 'Form presupposes matter to which it refers. Two do not find themselves confronting each other externally and accidentally. Neither matter nor form derives from itself, is a se, is eternal. Matter is indifferent with respect to form. Form presupposes matter because posits itself as sublated, refers to its identity as to something other. Matter is not simple essence but essence determined as something positive, only is as sublated negation. Matter is determined as groundless subsistence. Matter is not determined as ground of form. Form and matter alike determined as not to be posited each by other, each not to be ground of other. Matter is identity of ground and grounded, as substrate. Matter, determined as indifferent, is passive as contrasted to form, which is active. Form contains form locked up inside it. Matter must be informed, form must materialize itself.',
  provenance: {
    sourceChunk: 'abs-8',
    sourceOp: 'abs-op-8-form-and-matter-presuppose',
  },
};

export const absOp9FormDeterminesMatter: LogicalOperation = {
  id: 'abs-op-9-form-determines-matter',
  chunkId: 'abs-9',
  label: 'Form determines matter — self-mediation',
  clauses: [
    'form = determinesMatter',
    'matter = determinedByForm',
    'form = absoluteSelfIdentity',
    'form = containsMatter',
    'matter = possessesForm',
    'activity = sublatingOfSemblance',
    'determination = selfMediation',
    'twoMediations = oneMovement',
    'restoration = innerRecollection',
  ],
  predicates: [
    { name: 'determinesMatter', args: ['form'] },
    { name: 'determinedByForm', args: ['matter'] },
    { name: 'absoluteSelfIdentity', args: ['form'] },
    { name: 'containsMatter', args: ['form'] },
    { name: 'possessesForm', args: ['matter'] },
    { name: 'sublatingOfSemblance', args: ['activity'] },
    { name: 'selfMediation', args: ['determination'] },
    { name: 'oneMovement', args: ['twoMediations'] },
    { name: 'innerRecollection', args: ['restoration'] },
  ],
  relations: [
    { predicate: 'determines', from: 'form', to: 'matter' },
  ],
  candidateSummary: 'Form determines matter, matter is determined by form. Because form is absolute self-identity and implicitly contains matter. And equally because matter in pure abstraction or absolute negativity possesses form within it. Activity of form on matter and reception by matter is only sublating of semblance of their indifference and distinctness. Determination referring each to other is self-mediation of each through its own non-being. Two mediations are one movement. Restoration of their original identity is inner recollection of their exteriorization.',
  provenance: {
    sourceChunk: 'abs-9',
    sourceOp: 'abs-op-9-form-determines-matter',
  },
};

export const absOp10AbsoluteGround: LogicalOperation = {
  id: 'abs-op-10-absolute-ground',
  chunkId: 'abs-10',
  label: 'Form and matter presuppose each other — absolute ground',
  clauses: [
    'formAndMatter = presupposeEachOther',
    'essentialUnity = negativeSelfReference',
    'unity = splits',
    'substrate = indifferent',
    'form = determining',
    'unityOfEssenceAndForm = absoluteSelfDeterminingGround',
    'reference = reciprocalPresupposition',
  ],
  predicates: [
    { name: 'presupposeEachOther', args: ['formAndMatter'] },
    { name: 'negativeSelfReference', args: ['essentialUnity'] },
    { name: 'splits', args: ['unity'] },
    { name: 'indifferent', args: ['substrate'] },
    { name: 'determining', args: ['form'] },
    { name: 'absoluteSelfDeterminingGround', args: ['unityOfEssenceAndForm'] },
    { name: 'reciprocalPresupposition', args: ['reference'] },
  ],
  relations: [
    { predicate: 'is', from: 'unityOfEssenceAndForm', to: 'absoluteSelfDeterminingGround' },
  ],
  candidateSummary: 'Form and matter presuppose each other. This only means one essential unity is negative self-reference. Therefore splits, determined as indifferent substrate in essential identity. And as determining form in essential distinction or negativity. That unity of essence and form, two opposed as form and matter, is absolute self-determining ground. Inasmuch as unity differentiates itself, reference connecting two diverse terms becomes reference of reciprocal presupposition.',
  provenance: {
    sourceChunk: 'abs-10',
    sourceOp: 'abs-op-10-absolute-ground',
  },
};

export const absOp11FormSelfSublating: LogicalOperation = {
  id: 'abs-op-11-form-self-sublating',
  chunkId: 'abs-11',
  label: 'Form self-sublating — two sides',
  clauses: [
    'form = selfSublatingContradiction',
    'sublating = twoSided',
    'form = transformsIntoPosited',
    'posited = existsInOther',
    'other = matter',
    'form = sublatesDeterminateness',
    'form = givesItselfSubsistence',
    'reflection = unionWithMatter',
  ],
  predicates: [
    { name: 'selfSublatingContradiction', args: ['form'] },
    { name: 'twoSided', args: ['sublating'] },
    { name: 'transformsIntoPosited', args: ['form'] },
    { name: 'existsInOther', args: ['posited'] },
    { name: 'matter', args: ['other'] },
    { name: 'sublatesDeterminateness', args: ['form'] },
    { name: 'givesItselfSubsistence', args: ['form'] },
    { name: 'unionWithMatter', args: ['reflection'] },
  ],
  relations: [
    { predicate: 'transformsInto', from: 'form', to: 'posited' },
  ],
  candidateSummary: 'Form already is, as self-subsisting, self-sublating contradiction. Also posited as in this way self-sublating. Since two-sided, sublating also has two sides. For one, form sublates its self-subsistence and transforms itself into something posited, something that exists in other, and this other is matter. For other, form sublates its determinateness vis-à-vis matter, sublates its reference to it, consequently its positedness, thereby gives itself subsistence. Reflection in sublating positedness is its own identity into which it passes over. But since form at same time externalizes this identity and posits it over against itself as matter, reflection of positedness into itself is union with matter in which it obtains subsistence.',
  provenance: {
    sourceChunk: 'abs-11',
    sourceOp: 'abs-op-11-form-self-sublating',
  },
};

export const absOp12ActivityOfForm: LogicalOperation = {
  id: 'abs-op-12-activity-of-form',
  chunkId: 'abs-12',
  label: 'Activity of form — matter\'s movement',
  clauses: [
    'activityOfForm = negativeRelatingToItself',
    'movementOfMatter = formsOwnMovement',
    'form = freeOfMatter',
    'form = sublatesSelfSubsistence',
    'selfSubsistence = matterItself',
    'matter = sameContradictionAsForm',
    'matter = selfContradictory',
    'matter = sublatesItselfWithin',
  ],
  predicates: [
    { name: 'negativeRelatingToItself', args: ['activityOfForm'] },
    { name: 'formsOwnMovement', args: ['movementOfMatter'] },
    { name: 'freeOfMatter', args: ['form'] },
    { name: 'sublatesSelfSubsistence', args: ['form'] },
    { name: 'matterItself', args: ['selfSubsistence'] },
    { name: 'sameContradictionAsForm', args: ['matter'] },
    { name: 'selfContradictory', args: ['matter'] },
    { name: 'sublatesItselfWithin', args: ['matter'] },
  ],
  relations: [
    { predicate: 'is', from: 'movementOfMatter', to: 'formsOwnMovement' },
  ],
  candidateSummary: 'Activity of form by which matter is determined consists in negative relating of form to itself. Movement by which matter becomes determined is just as much form\'s own movement. Form is free of matter, but sublates its self-subsistence. Its self-subsistence is matter itself, for in matter it has its essential identity. Makes itself into positedness, but this is same as making matter into something determinate. What appears as activity of form is just as much movement that belongs to matter itself. Matter is in itself same contradiction that form contains. Matter is in itself self-contradictory because, as indeterminate self-identity, is at same time absolute negativity. Sublates itself within: identity disintegrates in negativity while latter obtains in it its subsistence.',
  provenance: {
    sourceChunk: 'abs-12',
    sourceOp: 'abs-op-12-activity-of-form',
  },
};

export const absOp13OriginalUnityRestored: LogicalOperation = {
  id: 'abs-op-13-original-unity-restored',
  chunkId: 'abs-13',
  label: 'Original unity restored — content',
  clauses: [
    'originalUnity = restored',
    'unity = positedUnity',
    'activityOfForm = movementOfMatter',
    'result = unityOfInItselfAndPositedness',
    'matter = determined',
    'form = material',
    'unity = ground',
    'informedMatter = content',
    'content = absoluteUnityOfGround',
  ],
  predicates: [
    { name: 'restored', args: ['originalUnity'] },
    { name: 'positedUnity', args: ['unity'] },
    { name: 'movementOfMatter', args: ['activityOfForm'] },
    { name: 'unityOfInItselfAndPositedness', args: ['result'] },
    { name: 'determined', args: ['matter'] },
    { name: 'material', args: ['form'] },
    { name: 'ground', args: ['unity'] },
    { name: 'content', args: ['informedMatter'] },
    { name: 'absoluteUnityOfGround', args: ['content'] },
  ],
  relations: [
    { predicate: 'is', from: 'informedMatter', to: 'content' },
  ],
  candidateSummary: 'Through movement of form and matter, original unity restored. On other hand, henceforth posited unity. Matter is just as much self-determining as determining is for it activity of form external to it. Activity of form and movement of matter are one and same thing. Result is unity of in-itself and positedness. Matter is as such determined or necessarily has form. Form is simply material, subsistent form. Inasmuch as form presupposes matter as its other, it is finite. Neither finite matter nor finite form have truth. Each refers to other, or only their unity is their truth. Unity proves to be their ground. Matter is ground only as absolute unity of essence and form. One unity, as absolute negativity, exclusive unity, is in its reflection a presupposing. Act by which matter determined by form is self-mediation of essence as ground, through itself and through negation of itself. Informed matter or form that possesses subsistence is absolute unity of ground with itself, but also unity as posited. Unity of form and matter, as substrate of both, but substrate which is determinate. Formed matter, but matter at same time indifferent to form and matter, because sublated and unessential. This is content.',
  provenance: {
    sourceChunk: 'abs-13',
    sourceOp: 'abs-op-13-original-unity-restored',
  },
};

export const absOp14FormAndContent: LogicalOperation = {
  id: 'abs-op-14-form-and-content',
  chunkId: 'abs-14',
  label: 'c. Form and content — form stands over against content',
  clauses: [
    'form = standsOverAgainstEssence',
    'form = standsOverAgainstMatter',
    'form = standsOverAgainstContent',
    'form = determiningReflection',
    'determinations = itselfAndMatter',
    'selfIdentical = passesUnderDominionOfForm',
  ],
  predicates: [
    { name: 'standsOverAgainstEssence', args: ['form'] },
    { name: 'standsOverAgainstMatter', args: ['form'] },
    { name: 'standsOverAgainstContent', args: ['form'] },
    { name: 'determiningReflection', args: ['form'] },
    { name: 'itselfAndMatter', args: ['determinations'] },
    { name: 'passesUnderDominionOfForm', args: ['selfIdentical'] },
  ],
  relations: [
    { predicate: 'standsOverAgainst', from: 'form', to: 'content' },
  ],
  candidateSummary: 'Form stands at first over against essence. It is then ground-connection in general, determinations are ground and grounded. It then stands over against matter. So it is determining reflection, determinations are determination of reflection itself and subsistence of latter. Finally, it stands over against content. Then its determinations are again itself and matter. What was previously self-identical: at first ground, then subsistence in general, finally matter. Now passes under dominion of form and is once more one of its determinations.',
  provenance: {
    sourceChunk: 'abs-14',
    sourceOp: 'abs-op-14-form-and-content',
  },
};

export const absOp15ContentHasFormAndMatter: LogicalOperation = {
  id: 'abs-op-15-content-has-form-and-matter',
  chunkId: 'abs-15',
  label: 'Content has form and matter — unity',
  clauses: [
    'content = hasFormAndMatter',
    'content = unityOfFormAndMatter',
    'content = standsOverAgainstForm',
    'content = indifferentTowardsForm',
    'content = identicalInFormAndMatter',
    'content = negativeReflectionIntoThemselves',
    'unity = formalUnity',
    'content = hasGroundConnection',
    'ground = hasContent',
  ],
  predicates: [
    { name: 'hasFormAndMatter', args: ['content'] },
    { name: 'unityOfFormAndMatter', args: ['content'] },
    { name: 'standsOverAgainstForm', args: ['content'] },
    { name: 'indifferentTowardsForm', args: ['content'] },
    { name: 'identicalInFormAndMatter', args: ['content'] },
    { name: 'negativeReflectionIntoThemselves', args: ['content'] },
    { name: 'formalUnity', args: ['unity'] },
    { name: 'hasGroundConnection', args: ['content'] },
    { name: 'hasContent', args: ['ground'] },
  ],
  relations: [
    { predicate: 'has', from: 'content', to: 'formAndMatter' },
  ],
  candidateSummary: 'Content has, first, form and matter that belong to it essentially; it is their unity. But because unity is at same time determinate or posited unity, content stands over against form. Latter constitutes positedness and is unessential over against content. Content indifferent towards form. Content is, second, what is identical in form and matter. So that these would be only indifferent external determinations. They are positedness in general, but positedness that has returned in content to its unity or its ground. Identity of content with itself is, in one respect, identity indifferent to form. But in another, identity of ground. Content is at same time negative reflection of form determinations into themselves. Its unity is therefore also formal unity or ground-connection as such. Content has this ground-connection as its essential form. Contrariwise, ground has a content.',
  provenance: {
    sourceChunk: 'abs-15',
    sourceOp: 'abs-op-15-content-has-form-and-matter',
  },
};

export const absOp16ContentOfGround: LogicalOperation = {
  id: 'abs-op-16-content-of-ground',
  chunkId: 'abs-16',
  label: 'Content of ground — determinate ground',
  clauses: [
    'contentOfGround = groundReturnedIntoUnity',
    'ground = indeterminateMatter',
    'ground = informedIdentity',
    'form = groundConnection',
    'content = determinedWithin',
    'content = essentialSelfIdentity',
    'content = positedIdentity',
    'ground = determinateGround',
    'determinateness = twofold',
  ],
  predicates: [
    { name: 'groundReturnedIntoUnity', args: ['contentOfGround'] },
    { name: 'indeterminateMatter', args: ['ground'] },
    { name: 'informedIdentity', args: ['ground'] },
    { name: 'groundConnection', args: ['form'] },
    { name: 'determinedWithin', args: ['content'] },
    { name: 'essentialSelfIdentity', args: ['content'] },
    { name: 'positedIdentity', args: ['content'] },
    { name: 'determinateGround', args: ['ground'] },
    { name: 'twofold', args: ['determinateness'] },
  ],
  relations: [
    { predicate: 'is', from: 'ground', to: 'determinateGround' },
  ],
  candidateSummary: 'Content of ground is ground that has returned into its unity with itself. Ground is at first essence that in its positedness is identical with itself. As diverse from and indifferent to its positedness, ground is indeterminate matter. But as content it is at same time informed identity. This form becomes ground-connection. Content is further determined within, not like matter as indifferent in general. But like informed matter, so that determinations of form have material, indifferent subsistence. On one hand, content is essential self-identity of ground in its positedness. On other hand, it is posited identity as against ground-connection. This positedness stands over against free positedness, form as whole connection of ground and grounded. Ground has made itself into determinate ground in general. Determinateness is itself twofold: of form first, and of content second.',
  provenance: {
    sourceChunk: 'abs-16',
    sourceOp: 'abs-op-16-content-of-ground',
  },
};

export const absoluteGroundOperations: LogicalOperation[] = [
  absOp1FormAndEssence,
  absOp2MediationOfGround,
  absOp3EssenceHasForm,
  absOp4FormDeterminations,
  absOp5FormAsCompletedWhole,
  absOp6FormDeterminesEssence,
  absOp7FormAndMatter,
  absOp8FormAndMatterPresuppose,
  absOp9FormDeterminesMatter,
  absOp10AbsoluteGround,
  absOp11FormSelfSublating,
  absOp12ActivityOfForm,
  absOp13OriginalUnityRestored,
  absOp14FormAndContent,
  absOp15ContentHasFormAndMatter,
  absOp16ContentOfGround,
];
