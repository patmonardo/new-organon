/**
 * Logical Operations: The Relation of Whole and Parts
 *
 * The Relation of Whole and Parts is the first essential relation, where reflected
 * and existent immediacy have self-subsistence but contain contradiction that leads
 * to the relation of force and its expression.
 *
 * Dialectical Movement:
 * - Whole and parts: reciprocal conditioning
 * - Contradiction: each subsistence in other
 * - Transition: force and expression
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// THE RELATION OF WHOLE AND PARTS
// ============================================================================

export const wlpOp1TwoSides: LogicalOperation = {
  id: 'wlp-op-1-two-sides',
  chunkId: 'wlp-1',
  label: 'Essential relation — two sides',
  clauses: [
    'essentialRelation = selfSubsistenceReflected',
    'whole = worldInItself',
    'parts = worldOfAppearance',
    'each = hasOtherReflectivelyShining',
    'whole = substrate',
  ],
  predicates: [
    { name: 'selfSubsistenceReflected', args: ['essentialRelation'] },
    { name: 'worldInItself', args: ['whole'] },
    { name: 'worldOfAppearance', args: ['parts'] },
    { name: 'hasOtherReflectivelyShining', args: ['each'] },
    { name: 'substrate', args: ['whole'] },
  ],
  relations: [
    { predicate: 'is', from: 'whole', to: 'worldInItself' },
  ],
  candidateSummary: 'Essential relation contains self-subsistence of concrete existence reflected into itself. Simple form whose determinations are concrete existences, posited at same time, moments held in unity. Self-subsistence reflected into itself is reflection into opposite, immediate self-subsistence. Subsistence is identity with opposite no less than own self-subsistence. Other side immediately posited: immediate self-subsistence determined as other, multifarious manifold. Manifold essentially has reference of other side, unity of reflected self-subsistence. Whole is self-subsistence that constitutes world existing in-and-for-itself. Parts is immediate concrete existence which was world of appearance. In relation, two sides are self-subsistences, each has other reflectively shining in it, only is as identity of both. Whole constitutes unity of two sides, substrate. Immediate concrete existence as positedness. On side of parts, immediate manifold is self-subsistent substrate, whole only external reference.',
  provenance: {
    sourceChunk: 'wlp-1',
    sourceOp: 'wlp-op-1-two-sides',
  },
};

export const wlpOp2Contradiction: LogicalOperation = {
  id: 'wlp-op-2-contradiction',
  chunkId: 'wlp-2',
  label: 'Relation contains contradiction',
  clauses: [
    'relation = selfSubsistenceAndSublatedness',
    'whole = selfSubsistent',
    'parts = selfSubsistent',
    'each = relativeOfOther',
    'relation = immediateContradiction',
  ],
  predicates: [
    { name: 'selfSubsistenceAndSublatedness', args: ['relation'] },
    { name: 'selfSubsistent', args: ['whole'] },
    { name: 'selfSubsistent', args: ['parts'] },
    { name: 'relativeOfOther', args: ['each'] },
    { name: 'immediateContradiction', args: ['relation'] },
  ],
  relations: [
    { predicate: 'contains', from: 'relation', to: 'contradiction' },
  ],
  candidateSummary: 'Relation contains self-subsistence of sides and their sublatedness no less, two simply in one reference. Whole is self-subsistent. Parts are only moments of unity, but also equally self-subsistent. Reflected unity only moment. Each, in self-subsistence, simply relative of other. Relation within it immediate contradiction, sublates itself.',
  provenance: {
    sourceChunk: 'wlp-2',
    sourceOp: 'wlp-op-2-contradiction',
  },
};

export const wlpOp3WholeSelfExternalized: LogicalOperation = {
  id: 'wlp-op-3-whole-self-externalized',
  chunkId: 'wlp-3',
  label: 'Whole — self-externalized',
  clauses: [
    'whole = selfExternalized',
    'whole = hasSubsistenceInParts',
    'whole = consistsOfParts',
    'whole = onlyRelative',
  ],
  predicates: [
    { name: 'selfExternalized', args: ['whole'] },
    { name: 'hasSubsistenceInParts', args: ['whole'] },
    { name: 'consistsOfParts', args: ['whole'] },
    { name: 'onlyRelative', args: ['whole'] },
  ],
  relations: [
    { predicate: 'consistsOf', from: 'whole', to: 'parts' },
  ],
  candidateSummary: 'Whole is reflected unity that stands independently on own. But subsistence equally repelled by it. Self-externalized. Has subsistence in opposite, manifold immediacy, parts. Whole consists of parts, apart from them not anything. Whole relation and self-subsistent totality. But for precisely this reason, only relative. What makes it totality is rather other, parts. Does not have subsistence within it but in other.',
  provenance: {
    sourceChunk: 'wlp-3',
    sourceOp: 'wlp-op-3-whole-self-externalized',
  },
};

export const wlpOp4PartsWholeRelation: LogicalOperation = {
  id: 'wlp-op-4-parts-whole-relation',
  chunkId: 'wlp-4',
  label: 'Parts — whole relation',
  clauses: [
    'parts = haveWholeWithin',
    'parts = withoutWholeNoParts',
    'parts = haveSelfSubsistenceOnlyInWhole',
    'whole = otherToParts',
  ],
  predicates: [
    { name: 'haveWholeWithin', args: ['parts'] },
    { name: 'withoutWholeNoParts', args: ['parts'] },
    { name: 'haveSelfSubsistenceOnlyInWhole', args: ['parts'] },
    { name: 'otherToParts', args: ['whole'] },
  ],
  relations: [
    { predicate: 'haveSelfSubsistenceIn', from: 'parts', to: 'whole' },
  ],
  candidateSummary: 'Parts likewise whole relation. Immediate as against reflected self-subsistence. Do not subsist in whole but for themselves. Have whole within them as moment. Whole constitutes connecting reference. Without whole no parts. But because self-subsistent, connection only external moment, with respect to which indifferent. But parts, as manifold concrete existence, collapse together. Concrete existence is reflectionless being. Have self-subsistence only in reflected unity, which is unity as well as concrete existent manifoldness. Have self-subsistence only in whole. But whole at same time self-subsistence which is other to parts.',
  provenance: {
    sourceChunk: 'wlp-4',
    sourceOp: 'wlp-op-4-parts-whole-relation',
  },
};

export const wlpOp5ReciprocalConditioning: LogicalOperation = {
  id: 'wlp-op-5-reciprocal-conditioning',
  chunkId: 'wlp-5',
  label: 'Reciprocal conditioning — unconditioned',
  clauses: [
    'wholeAndParts = reciprocallyCondition',
    'each = immediateAndMediated',
    'wholeRelation = turningBackIntoItself',
    'wholeRelation = unconditioned',
  ],
  predicates: [
    { name: 'reciprocallyCondition', args: ['wholeAndParts'] },
    { name: 'immediateAndMediated', args: ['each'] },
    { name: 'turningBackIntoItself', args: ['wholeRelation'] },
    { name: 'unconditioned', args: ['wholeRelation'] },
  ],
  relations: [
    { predicate: 'is', from: 'wholeRelation', to: 'unconditioned' },
  ],
  candidateSummary: 'Whole and parts reciprocally condition each other. Relation higher than reference of conditioned and condition. Reference realized: condition is essential self-subsistence of conditioned, presupposed by latter. Condition as such only immediate, implicitly presupposed. Whole, through condition of parts, entails it only in so far as has parts for presupposition. Both sides posited as conditioning each other reciprocally. Each immediate self-subsistence, but self-subsistence equally mediated or posited through other. Whole relation, because of reciprocity, is turning back of conditioning into itself, non-relative, unconditioned.',
  provenance: {
    sourceChunk: 'wlp-5',
    sourceOp: 'wlp-op-5-reciprocal-conditioning',
  },
};

export const wlpOp6TwoRespects: LogicalOperation = {
  id: 'wlp-op-6-two-respects',
  chunkId: 'wlp-6',
  label: 'Two respects — identity and indifference',
  clauses: [
    'oneIdentity = bothOnlyMoments',
    'two = selfSubsistentIndifferent',
  ],
  predicates: [
    { name: 'bothOnlyMoments', args: ['oneIdentity'] },
    { name: 'selfSubsistentIndifferent', args: ['two'] },
  ],
  relations: [
    { predicate: 'are', from: 'two', to: 'selfSubsistentIndifferent' },
  ],
  candidateSummary: 'Inasmuch as each side has self-subsistence not in it but in other, only one identity of two in which both only moments. But inasmuch as each self-subsistent on own, two are two self-subsistent concrete existences indifferent to each other.',
  provenance: {
    sourceChunk: 'wlp-6',
    sourceOp: 'wlp-op-6-two-respects',
  },
};

export const wlpOp7EssentialIdentity: LogicalOperation = {
  id: 'wlp-op-7-essential-identity',
  chunkId: 'wlp-7',
  label: 'Essential identity — equality',
  clauses: [
    'whole = equalToParts',
    'parts = equalToWhole',
    'nothingInWhole = notInParts',
    'relation = indivisibleIdentity',
  ],
  predicates: [
    { name: 'equalToParts', args: ['whole'] },
    { name: 'equalToWhole', args: ['parts'] },
    { name: 'notInParts', args: ['nothingInWhole'] },
    { name: 'indivisibleIdentity', args: ['relation'] },
  ],
  relations: [
    { predicate: 'equalTo', from: 'whole', to: 'parts' },
  ],
  candidateSummary: 'In first respect, essential identity of two sides. Whole equal to parts and parts equal to whole. Nothing in whole which is not in parts, nothing in parts which is not in whole. Whole not abstract unity but unity of diversified manifoldness. Unity within which manifold held together is determinateness by virtue of which latter is parts. Relation has indivisible identity and only one self-subsistence.',
  provenance: {
    sourceChunk: 'wlp-7',
    sourceOp: 'wlp-op-7-essential-identity',
  },
};

export const wlpOp8WholeEqualToParts: LogicalOperation = {
  id: 'wlp-op-8-whole-equal-to-parts',
  chunkId: 'wlp-8',
  label: 'Whole equal to parts — tautology',
  clauses: [
    'whole = equalToPartsButNotAsParts',
    'whole = equalToTogether',
    'equality = tautology',
  ],
  predicates: [
    { name: 'equalToPartsButNotAsParts', args: ['whole'] },
    { name: 'equalToTogether', args: ['whole'] },
    { name: 'tautology', args: ['equality'] },
  ],
  relations: [
    { predicate: 'is', from: 'equality', to: 'tautology' },
  ],
  candidateSummary: 'Whole equal to parts but not to them as parts. Whole is reflected unity. Parts constitute determinate moment or otherness of unity, diversified manifold. Whole not equal to them as self-subsistent diversity but to them together. Their \'together\' nothing else but unity, whole as such. In parts, whole only equal to itself. Equality expresses only tautology: whole as whole equal not to parts but to whole.',
  provenance: {
    sourceChunk: 'wlp-8',
    sourceOp: 'wlp-op-8-whole-equal-to-parts',
  },
};

export const wlpOp9PartsEqualToWhole: LogicalOperation = {
  id: 'wlp-op-9-parts-equal-to-whole',
  chunkId: 'wlp-9',
  label: 'Parts equal to whole — tautology',
  clauses: [
    'parts = equalToWholeAsManifold',
    'parts = equalAsApportionedWhole',
    'tautology = partsEqualToThemselves',
  ],
  predicates: [
    { name: 'equalToWholeAsManifold', args: ['parts'] },
    { name: 'equalAsApportionedWhole', args: ['parts'] },
    { name: 'partsEqualToThemselves', args: ['tautology'] },
  ],
  relations: [
    { predicate: 'is', from: 'equality', to: 'tautology' },
  ],
  candidateSummary: 'Parts equal to whole. But because, as parts, moment of otherness, not equal to it as unity. But one of whole\'s manifold determinations maps over part. Equal to whole as manifold, as apportioned whole, as parts. Same tautology: parts as parts equal not to whole as such but in whole, to themselves.',
  provenance: {
    sourceChunk: 'wlp-9',
    sourceOp: 'wlp-op-9-parts-equal-to-whole',
  },
};

export const wlpOp10FallApart: LogicalOperation = {
  id: 'wlp-op-10-fall-apart',
  chunkId: 'wlp-10',
  label: 'Fall apart — destroy themselves',
  clauses: [
    'wholeAndParts = fallApart',
    'each = destroysThemselves',
    'selfSubsistence = negationOfSelves',
    'each = hasSelfSubsistenceInOther',
    'first = notFirst',
  ],
  predicates: [
    { name: 'fallApart', args: ['wholeAndParts'] },
    { name: 'destroysThemselves', args: ['each'] },
    { name: 'negationOfSelves', args: ['selfSubsistence'] },
    { name: 'hasSelfSubsistenceInOther', args: ['each'] },
    { name: 'notFirst', args: ['first'] },
  ],
  relations: [
    { predicate: 'is', from: 'selfSubsistence', to: 'negationOfSelves' },
  ],
  candidateSummary: 'Whole and parts fall indifferently apart. Each side refers only to itself. As held apart, destroy themselves. Whole indifferent towards parts is abstract identity, undifferentiated in itself. Identity whole only inasmuch as differentiated in itself. Identity of reflection has shown through movement that it has reflection into other for truth. Parts, indifferent to unity, only unconnected manifold, inherently other which sublates itself. Self-reference of each side is self-subsistence. But self-subsistence each side has for itself is negation of respective selves. Each side has self-subsistence not within but in other side. Other constitutes subsistence, presupposed immediate, supposed to be first. But first of each side only first which is not first, has beginning in other.',
  provenance: {
    sourceChunk: 'wlp-10',
    sourceOp: 'wlp-op-10-fall-apart',
  },
};

export const wlpOp11TruthMediation: LogicalOperation = {
  id: 'wlp-op-11-truth-mediation',
  chunkId: 'wlp-11',
  label: 'Truth of relation — mediation',
  clauses: [
    'truth = mediation',
    'contradiction = returnsToGround',
    'selfSubsistence = onlyToDisappear',
    'existence = positedAndMediated',
  ],
  predicates: [
    { name: 'mediation', args: ['truth'] },
    { name: 'returnsToGround', args: ['contradiction'] },
    { name: 'onlyToDisappear', args: ['selfSubsistence'] },
    { name: 'positedAndMediated', args: ['existence'] },
  ],
  relations: [
    { predicate: 'is', from: 'truth', to: 'mediation' },
  ],
  candidateSummary: 'Truth of relation consists in mediation. Essence is negative unity in which both reflected and existent immediacy equally sublated. Relation is contradiction that returns to ground, into unity which, as turning back, is reflected unity. But since equally posited itself as sublated, refers to itself negatively, makes itself into existent immediacy. Unity\'s negative reference, in so far as first and immediate, only is as mediated by other and equally posited. Other, existent immediacy, equally only as sublated. Self-subsistence is first, but only in order to disappear. Has existence which is posited and mediated.',
  provenance: {
    sourceChunk: 'wlp-11',
    sourceOp: 'wlp-op-11-truth-mediation',
  },
};

export const wlpOp12TransitionToForce: LogicalOperation = {
  id: 'wlp-op-12-transition-to-force',
  chunkId: 'wlp-12',
  label: 'Transition — force and expressions',
  clauses: [
    'relation = noLongerWholeAndParts',
    'immediacy = passedIntoPositedness',
    'relation = passedIntoForceAndExpression',
  ],
  predicates: [
    { name: 'noLongerWholeAndParts', args: ['relation'] },
    { name: 'passedIntoPositedness', args: ['immediacy'] },
    { name: 'passedIntoForceAndExpression', args: ['relation'] },
  ],
  relations: [
    { predicate: 'passesInto', from: 'relation', to: 'forceAndExpression' },
  ],
  candidateSummary: 'Determined in this way, relation no longer one of whole and parts. Previous immediacy of sides passed over into positedness and mediation. Each side posited, in so far as immediate, as self-sublating and passing over into other. In so far as negative reference, posited as conditioned through other, through positive. Immediate transition of each equally mediation, sublating posited through other. Relation of whole and parts passed over into relation of force and its expressions.',
  provenance: {
    sourceChunk: 'wlp-12',
    sourceOp: 'wlp-op-12-transition-to-force',
  },
};

export const wholePartsOperations: LogicalOperation[] = [
  wlpOp1TwoSides,
  wlpOp2Contradiction,
  wlpOp3WholeSelfExternalized,
  wlpOp4PartsWholeRelation,
  wlpOp5ReciprocalConditioning,
  wlpOp6TwoRespects,
  wlpOp7EssentialIdentity,
  wlpOp8WholeEqualToParts,
  wlpOp9PartsEqualToWhole,
  wlpOp10FallApart,
  wlpOp11TruthMediation,
  wlpOp12TransitionToForce,
];
