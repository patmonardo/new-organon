/**
 * Logical Operations: Determinate Ground
 *
 * Determinate Ground is the second determination of ground. It covers formal ground,
 * real ground, and complete ground.
 *
 * Dialectical Movement:
 * - Formal ground: determinate content
 * - Real ground: diverse content
 * - Complete ground: real ground returns to ground
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// DETERMINATE GROUND
// ============================================================================

export const detOp1FormalGround: LogicalOperation = {
  id: 'det-op-1-formal-ground',
  chunkId: 'det-1',
  label: 'a. Formal ground — determinate content',
  clauses: [
    'ground = hasDeterminateContent',
    'determinatenessOfContent = substrate',
    'determinatenessOfContent = simpleImmediate',
    'ground = negativelySelfReferringIdentity',
    'ground = makesItselfIntoPositedness',
    'identity = substrate',
    'identity = content',
    'identity = indifferentUnity',
  ],
  predicates: [
    { name: 'hasDeterminateContent', args: ['ground'] },
    { name: 'substrate', args: ['determinatenessOfContent'] },
    { name: 'simpleImmediate', args: ['determinatenessOfContent'] },
    { name: 'negativelySelfReferringIdentity', args: ['ground'] },
    { name: 'makesItselfIntoPositedness', args: ['ground'] },
    { name: 'substrate', args: ['identity'] },
    { name: 'content', args: ['identity'] },
    { name: 'indifferentUnity', args: ['identity'] },
  ],
  relations: [
    { predicate: 'has', from: 'ground', to: 'determinateContent' },
    { predicate: 'is', from: 'identity', to: 'content' },
  ],
  candidateSummary: 'Ground has determinate content. For form, determinateness of content is substrate, simple immediate as against mediation of form. Ground is negatively self-referring identity. Makes itself into positedness. Negatively refers to itself because in its negativity identical with itself. This identity is substrate or content. Constitutes indifferent or positive unity of ground-connection. In this connection, is mediating factor.',
  provenance: {
    sourceChunk: 'det-1',
    sourceOp: 'det-op-1-formal-ground',
  },
};

export const detOp2FormalMediation: LogicalOperation = {
  id: 'det-op-2-formal-mediation',
  chunkId: 'det-2',
  label: 'Formal mediation — negative and positive unity',
  clauses: [
    'mediation = negativeUnity',
    'negative = implicitInIndifferentSubstrate',
    'negative = substrateImmediateDeterminateness',
    'negative = negativeReferenceOfForm',
    'negativeMediation = formalMediation',
    'bothSides = positThemselvesIntoOneIdentity',
    'identity = determinateContent',
    'content = identicalElement',
    'content = subsistence',
  ],
  predicates: [
    { name: 'negativeUnity', args: ['mediation'] },
    { name: 'implicitInIndifferentSubstrate', args: ['negative'] },
    { name: 'substrateImmediateDeterminateness', args: ['negative'] },
    { name: 'negativeReferenceOfForm', args: ['negative'] },
    { name: 'formalMediation', args: ['negativeMediation'] },
    { name: 'positThemselvesIntoOneIdentity', args: ['bothSides'] },
    { name: 'determinateContent', args: ['identity'] },
    { name: 'identicalElement', args: ['content'] },
    { name: 'subsistence', args: ['content'] },
  ],
  relations: [
    { predicate: 'is', from: 'negativeMediation', to: 'formalMediation' },
  ],
  candidateSummary: 'In content, determinateness that ground and grounded have over against one another has at first disappeared. Mediation is also negative unity. Negative implicit in indifferent substrate is substrate\'s immediate determinateness. Through which ground has determinate content. Negative is negative reference of form to itself. Negative mediation of ground and grounded is formal mediation. Both sides of form mutually posit themselves into one identity as sublated. Presuppose identity. Latter is determinate content to which formal mediation refers itself as to positive mediating factor. Content is identical element of both, their subsistence, subsistence of each as whole itself.',
  provenance: {
    sourceChunk: 'det-2',
    sourceOp: 'det-op-2-formal-mediation',
  },
};

export const detOp3DeterminateGround: LogicalOperation = {
  id: 'det-op-3-determinate-ground',
  chunkId: 'det-3',
  label: 'Determinate ground — ground and grounded',
  clauses: [
    'determinateContent = consideredFromTwoSides',
    'content = indifferentToForms',
    'ground = momentOfForm',
    'grounded = sublatingOfItself',
    'grounded = positingOfGround',
    'movement = groundAsSuch',
    'eachSide = ground',
    'eachSide = posited',
    'eachSide = wholeMediation',
    'formAndContent = oneAndSameIdentity',
  ],
  predicates: [
    { name: 'consideredFromTwoSides', args: ['determinateContent'] },
    { name: 'indifferentToForms', args: ['content'] },
    { name: 'momentOfForm', args: ['ground'] },
    { name: 'sublatingOfItself', args: ['grounded'] },
    { name: 'positingOfGround', args: ['grounded'] },
    { name: 'groundAsSuch', args: ['movement'] },
    { name: 'ground', args: ['eachSide'] },
    { name: 'posited', args: ['eachSide'] },
    { name: 'wholeMediation', args: ['eachSide'] },
    { name: 'oneAndSameIdentity', args: ['formAndContent'] },
  ],
  relations: [
    { predicate: 'is', from: 'formAndContent', to: 'oneAndSameIdentity' },
  ],
  candidateSummary: 'In determinate ground: first, determinate content considered from two sides. Once as ground, then as grounded. Content indifferent to these forms, in each simply one determination. Second, ground is itself just as much moment of form as what is posited by it. Matter of indifference which determination is made first. Grounded is sublating of itself, makes itself into posited, and is positing of ground. Same movement is ground as such. Makes itself into posited, thereby becomes ground. Each side is just as much ground as posited, and each is whole mediation or whole form. Form and content are one and same identity.',
  provenance: {
    sourceChunk: 'det-3',
    sourceOp: 'det-op-3-determinate-ground',
  },
};

export const detOp4GroundIsSufficient: LogicalOperation = {
  id: 'det-op-4-ground-is-sufficient',
  chunkId: 'det-4',
  label: 'Ground is sufficient — formal ground',
  clauses: [
    'ground = sufficient',
    'nothingInGrounded = notInGround',
    'determination = doubled',
    'twoSides = doNotHaveRealDetermination',
    'determinateness = simpleDeterminateness',
    'determinateGround = formalGround',
    'content = selfIdenticalContent',
    'content = indifferentToForm',
    'form = externalToContent',
  ],
  predicates: [
    { name: 'sufficient', args: ['ground'] },
    { name: 'notInGround', args: ['nothingInGrounded'] },
    { name: 'doubled', args: ['determination'] },
    { name: 'doNotHaveRealDetermination', args: ['twoSides'] },
    { name: 'simpleDeterminateness', args: ['determinateness'] },
    { name: 'formalGround', args: ['determinateGround'] },
    { name: 'selfIdenticalContent', args: ['content'] },
    { name: 'indifferentToForm', args: ['content'] },
    { name: 'externalToContent', args: ['form'] },
  ],
  relations: [
    { predicate: 'is', from: 'ground', to: 'sufficient' },
  ],
  candidateSummary: 'Because of identity of ground and grounded, according both to content and form, ground is sufficient. Nothing in grounded which is not in ground. When one asks for ground, expects to see same determination doubled: once as posited, again as existence reflected into itself. In determined ground, ground and grounded are each whole form, content is one and same. Two sides do not have real determination, do not have different content. Determinateness is only one simple determinateness. Determinate ground present only in pure form, as formal ground. Content is self-identical content indifferent to form. Form is external to it, content is other than form.',
  provenance: {
    sourceChunk: 'det-4',
    sourceOp: 'det-op-4-ground-is-sufficient',
  },
};

export const detOp5RealGround: LogicalOperation = {
  id: 'det-op-5-real-ground',
  chunkId: 'det-5',
  label: 'b. Real ground — diverse content',
  clauses: [
    'determinatenessOfGround = determinatenessOfSubstrate',
    'determinatenessOfGround = othernessInGroundConnection',
    'connection = straysInContent',
    'content = indifferentToDeterminations',
    'content = identityOfGroundWithItself',
    'eachSide = identityOfWhole',
    'eachSide = diverseContent',
    'content = possessesDifferenceOfForm',
  ],
  predicates: [
    { name: 'determinatenessOfSubstrate', args: ['determinatenessOfGround'] },
    { name: 'othernessInGroundConnection', args: ['determinatenessOfGround'] },
    { name: 'straysInContent', args: ['connection'] },
    { name: 'indifferentToDeterminations', args: ['content'] },
    { name: 'identityOfGroundWithItself', args: ['content'] },
    { name: 'identityOfWhole', args: ['eachSide'] },
    { name: 'diverseContent', args: ['eachSide'] },
    { name: 'possessesDifferenceOfForm', args: ['content'] },
  ],
  relations: [
    { predicate: 'possesses', from: 'content', to: 'differenceOfForm' },
  ],
  candidateSummary: 'Determinateness of ground is determinateness of substrate or content determination, and otherness in ground-connection itself, distinctness of content and form. Connection strays in content as external form, content indifferent to determinations. But two are not external to each other. Content is: identity of ground with itself in grounded, and of grounded in ground. Each side is identity of whole within it. But since they belong to form and constitute its determinate difference, each is in its determinateness identity of whole with itself. Consequently, each has diverse content as against other. Content essentially possesses difference of form within. Is as ground something other than what it is as grounded.',
  provenance: {
    sourceChunk: 'det-5',
    sourceOp: 'det-op-5-real-ground',
  },
};

export const detOp6GroundRealized: LogicalOperation = {
  id: 'det-op-6-ground-realized',
  chunkId: 'det-6',
  label: 'Ground realized — no longer tautology',
  clauses: [
    'groundAndGrounded = diverseContent',
    'groundConnection = ceasedToBeFormal',
    'procession = noLongerTautology',
    'ground = realized',
    'demand = anotherContentDetermination',
  ],
  predicates: [
    { name: 'diverseContent', args: ['groundAndGrounded'] },
    { name: 'ceasedToBeFormal', args: ['groundConnection'] },
    { name: 'noLongerTautology', args: ['procession'] },
    { name: 'realized', args: ['ground'] },
    { name: 'anotherContentDetermination', args: ['demand'] },
  ],
  relations: [
    { predicate: 'is', from: 'ground', to: 'realized' },
  ],
  candidateSummary: 'Moment ground and grounded have diverse content, ground-connection has ceased to be formal one. Turning back to ground and procession forward from ground to posited is no longer tautology. Ground is realized. Henceforth, when we ask for ground, we actually demand another content determination for it than determination of content whose ground we are asking for.',
  provenance: {
    sourceChunk: 'det-6',
    sourceOp: 'det-op-6-ground-realized',
  },
};

export const detOp7ConnectionDeterminesItself: LogicalOperation = {
  id: 'det-op-7-connection-determines-itself',
  chunkId: 'det-7',
  label: 'Connection determines itself — twofold content',
  clauses: [
    'connection = determinesItself',
    'twoSides = differentContent',
    'twoSides = indifferent',
    'ground = reflectsItselfInOther',
    'grounded = hasSelfIdentityInGround',
    'grounded = possessesContentOfItsOwn',
    'grounded = unityOfTwofoldContent',
    'unity = negativeUnity',
    'unity = emptyReference',
    'unity = externalHoldingTogether',
  ],
  predicates: [
    { name: 'determinesItself', args: ['connection'] },
    { name: 'differentContent', args: ['twoSides'] },
    { name: 'indifferent', args: ['twoSides'] },
    { name: 'reflectsItselfInOther', args: ['ground'] },
    { name: 'hasSelfIdentityInGround', args: ['grounded'] },
    { name: 'possessesContentOfItsOwn', args: ['grounded'] },
    { name: 'unityOfTwofoldContent', args: ['grounded'] },
    { name: 'negativeUnity', args: ['unity'] },
    { name: 'emptyReference', args: ['unity'] },
    { name: 'externalHoldingTogether', args: ['unity'] },
  ],
  relations: [
    { predicate: 'possesses', from: 'grounded', to: 'contentOfItsOwn' },
  ],
  candidateSummary: 'Connection determines itself further. Two sides are of different content, indifferent to each other. Each is immediate, self-identical determination. Ground reflects itself in other, back to itself. Content on side of ground is equally in grounded. Latter has self-identity and subsistence only in ground. But grounded also possesses content of its own. Is unity of twofold content. Unity is negative unity, but since determinations are indifferent, unity is only empty reference to each other, void of content, not their mediation. It is something externally holding them together.',
  provenance: {
    sourceChunk: 'det-7',
    sourceOp: 'det-op-7-connection-determines-itself',
  },
};

export const detOp8RealGroundingConnection: LogicalOperation = {
  id: 'det-op-8-real-grounding-connection',
  chunkId: 'det-8',
  label: 'Real grounding connection — twofold',
  clauses: [
    'realGroundingConnection = twofold',
    'contentDetermination = extendsIntoPositedness',
    'grounded = containsGroundFully',
    'connection = undifferentiatedEssentialCompactness',
    'added = unessentialForm',
    'unessential = indifferentPositiveSubstrate',
    'connection = notGround',
    'connection = externalTie',
  ],
  predicates: [
    { name: 'twofold', args: ['realGroundingConnection'] },
    { name: 'extendsIntoPositedness', args: ['contentDetermination'] },
    { name: 'containsGroundFully', args: ['grounded'] },
    { name: 'undifferentiatedEssentialCompactness', args: ['connection'] },
    { name: 'unessentialForm', args: ['added'] },
    { name: 'indifferentPositiveSubstrate', args: ['unessential'] },
    { name: 'notGround', args: ['connection'] },
    { name: 'externalTie', args: ['connection'] },
  ],
  relations: [
    { predicate: 'contains', from: 'grounded', to: 'ground' },
  ],
  candidateSummary: 'In real grounding connection there is present a twofold. For one thing, content determination which is ground extends continuously into positedness. Constitutes simple identity of ground and grounded. Grounded contains ground fully within itself. Connection is undifferentiated essential compactness. Anything else in grounded added is only unessential form, external determinations free from ground, immediate manifold. Unessential is positively identical element, indifferent positive substrate. For another thing, that linked with substrate is indifferent content, but as unessential side. Main thing is connection of substrate and unessential manifold. But this connection is also not ground. One of something that constitutes connection is not reference of form but only external tie. It too is only substrate.',
  provenance: {
    sourceChunk: 'det-8',
    sourceOp: 'det-op-8-real-grounding-connection',
  },
};

export const detOp9GroundBreaksDown: LogicalOperation = {
  id: 'det-op-9-ground-breaks-down',
  chunkId: 'det-9',
  label: 'Ground breaks down — external ground',
  clauses: [
    'ground = breaksDown',
    'ground = externalDeterminations',
    'twoConnections = twoDifferentSubstrates',
    'formOfGround = vanished',
    'groundConnection = externalToItself',
    'externalGround = holdsTogetherDiversifiedContent',
    'realGround = referenceToAnother',
  ],
  predicates: [
    { name: 'breaksDown', args: ['ground'] },
    { name: 'externalDeterminations', args: ['ground'] },
    { name: 'twoDifferentSubstrates', args: ['twoConnections'] },
    { name: 'vanished', args: ['formOfGround'] },
    { name: 'externalToItself', args: ['groundConnection'] },
    { name: 'holdsTogetherDiversifiedContent', args: ['externalGround'] },
    { name: 'referenceToAnother', args: ['realGround'] },
  ],
  relations: [
    { predicate: 'is', from: 'realGround', to: 'referenceToAnother' },
  ],
  candidateSummary: 'Ground, in determining itself as real, because of diversity of content, breaks down into external determinations. Two connections: essential reality content as simple immediate identity, and something connecting distinct contents are two different substrates. Self-identical form of ground has vanished. Ground-connection has become external to itself. External ground holds together diversified content. Determines what is ground and what is posited by it. Determination not found in two-sided content itself. Real ground is reference to another: of content to another content, and of ground-connection (form) to immediate, to something not posited by it.',
  provenance: {
    sourceChunk: 'det-9',
    sourceOp: 'det-op-9-ground-breaks-down',
  },
};

export const detOp10CompleteGround: LogicalOperation = {
  id: 'det-op-10-complete-ground',
  chunkId: 'det-10',
  label: 'c. Complete ground — real ground returns to ground',
  clauses: [
    'groundAsContent = substrate',
    'groundAsConnection = substrate',
    'realGroundConnection = sublated',
    'ground = returnedToItsGround',
    'ground = grounded',
    'newGround = immanentReflectionOfLink',
    'reference = absoluteReference',
  ],
  predicates: [
    { name: 'substrate', args: ['groundAsContent'] },
    { name: 'substrate', args: ['groundAsConnection'] },
    { name: 'sublated', args: ['realGroundConnection'] },
    { name: 'returnedToItsGround', args: ['ground'] },
    { name: 'grounded', args: ['ground'] },
    { name: 'immanentReflectionOfLink', args: ['newGround'] },
    { name: 'absoluteReference', args: ['reference'] },
  ],
  relations: [
    { predicate: 'returnedTo', from: 'ground', to: 'ground' },
  ],
  candidateSummary: 'In real ground, ground as content and ground as connection are only substrates. Real ground-connection is ground rather as sublated. Makes up side of grounded or positedness. As positedness, ground itself has returned to its ground. Is now something grounded: it has another ground. This ground determined: first, identical with ground by which it is grounded, both sides have same content. Second, new ground is immanent reflection of link: absolute reference of two content determinations to each other.',
  provenance: {
    sourceChunk: 'det-10',
    sourceOp: 'det-op-10-complete-ground',
  },
};

export const detOp11CompleteGroundFormalAndReal: LogicalOperation = {
  id: 'det-op-11-complete-ground-formal-and-real',
  chunkId: 'det-11',
  label: 'Complete ground — formal and real',
  clauses: [
    'identity = reassertsItself',
    'groundConnection = complete',
    'groundConnection = containsFormalAndRealGround',
    'groundConnection = mediatesContentDeterminations',
  ],
  predicates: [
    { name: 'reassertsItself', args: ['identity'] },
    { name: 'complete', args: ['groundConnection'] },
    { name: 'containsFormalAndRealGround', args: ['groundConnection'] },
    { name: 'mediatesContentDeterminations', args: ['groundConnection'] },
  ],
  relations: [
    { predicate: 'contains', from: 'groundConnection', to: 'formalAndRealGround' },
  ],
  candidateSummary: 'Because real ground has returned to its ground, identity of ground and grounded or formality of ground reasserts itself. Newly arisen ground-connection is one which is complete. Contains formal and real ground in itself at same time. Mediates content determinations which in real ground confronted each other immediately.',
  provenance: {
    sourceChunk: 'det-11',
    sourceOp: 'det-op-11-complete-ground-formal-and-real',
  },
};

export const detOp12GroundConnectionDetermined: LogicalOperation = {
  id: 'det-op-12-ground-connection-determined',
  chunkId: 'det-12',
  label: 'Ground-connection determined — first and second',
  clauses: [
    'something = hasGround',
    'connection = sublatedInImmediacy',
    'connection = posited',
    'connection = hasGroundInAnother',
    'secondConnection = sameContent',
    'linking = immediate',
    'linking = notTrueAbsoluteConnection',
    'linking = relativeGround',
    'twoSomethings = distinctConnections',
  ],
  predicates: [
    { name: 'hasGround', args: ['something'] },
    { name: 'sublatedInImmediacy', args: ['connection'] },
    { name: 'posited', args: ['connection'] },
    { name: 'hasGroundInAnother', args: ['connection'] },
    { name: 'sameContent', args: ['secondConnection'] },
    { name: 'immediate', args: ['linking'] },
    { name: 'notTrueAbsoluteConnection', args: ['linking'] },
    { name: 'relativeGround', args: ['linking'] },
    { name: 'distinctConnections', args: ['twoSomethings'] },
  ],
  relations: [
    { predicate: 'has', from: 'something', to: 'ground' },
  ],
  candidateSummary: 'First, something has ground. Contains content determination which is ground and second determination as posited by ground. But because of indifference of content, one determination is not ground in itself. Connection sublated in immediacy of content, is posited, has its ground in another such connection. Second connection distinguished only according to form, has same content. Still has same two determinations but now their immediate linking together. Linking is not their true absolute connection. Two are supported by something, something connects them. But in connection which is not reflected, only immediate, only relative ground. Two somethings are two distinct connections. Stand in identical ground-connection of form. One and same whole content. Distinguished as ground and grounded only according to form.',
  provenance: {
    sourceChunk: 'det-12',
    sourceOp: 'det-op-12-ground-connection-determined',
  },
};

export const detOp13GroundConnectionReal: LogicalOperation = {
  id: 'det-op-13-ground-connection-real',
  chunkId: 'det-13',
  label: 'Ground-connection also real — inference',
  clauses: [
    'groundConnection = real',
    'formalGround = passesOverIntoRealGround',
    'content = twofold',
    'oneDetermination = identicalSubstrate',
    'groundConnection = mediated',
    'inference = linking',
  ],
  predicates: [
    { name: 'real', args: ['groundConnection'] },
    { name: 'passesOverIntoRealGround', args: ['formalGround'] },
    { name: 'twofold', args: ['content'] },
    { name: 'identicalSubstrate', args: ['oneDetermination'] },
    { name: 'mediated', args: ['groundConnection'] },
    { name: 'linking', args: ['inference'] },
  ],
  relations: [
    { predicate: 'passesOverInto', from: 'formalGround', to: 'realGround' },
  ],
  candidateSummary: 'Second, ground-connection is not only formal, but also real. Formal ground passes over into real ground. Moments of form reflect themselves into themselves, are self-subsistent content. Ground-connection contains one content with character of ground and another with that of grounded. Content is twofold content that behaves as ground and grounded. One content determination determined as identical substrate and foundation of connection. As against other determination, this determination is essential and is ground of other which is posited. In first something, second determination immediately linked with first. Other something only contains one determination as that in which immediately identical with first, but other as posited. Ground-connection in second something is mediated through connection in first something. Inference: since B implicitly linked with A in something, in second something to which only A belongs, also B is linked with it. This connection is ground of ground A. Whole ground-connection present in second something as posited or grounded.',
  provenance: {
    sourceChunk: 'det-13',
    sourceOp: 'det-op-13-ground-connection-real',
  },
};

export const detOp14CompleteMediation: LogicalOperation = {
  id: 'det-op-14-complete-mediation',
  chunkId: 'det-14',
  label: 'Complete mediation — conditioning',
  clauses: [
    'realGround = selfExternalReflection',
    'completeMediation = restorationOfIdentity',
    'groundConnection = mediatesItselfWithItself',
    'groundConnection = presupposingReflection',
    'totalGroundConnection = conditioningMediation',
  ],
  predicates: [
    { name: 'selfExternalReflection', args: ['realGround'] },
    { name: 'restorationOfIdentity', args: ['completeMediation'] },
    { name: 'mediatesItselfWithItself', args: ['groundConnection'] },
    { name: 'presupposingReflection', args: ['groundConnection'] },
    { name: 'conditioningMediation', args: ['totalGroundConnection'] },
  ],
  relations: [
    { predicate: 'is', from: 'totalGroundConnection', to: 'conditioningMediation' },
  ],
  candidateSummary: 'Real ground shows itself to be self-external reflection of ground. Its complete mediation is restoration of its identity with itself. But because identity has equally acquired externality of real ground, formal ground-connection is just as much self-positing as self-sublating ground. Ground-connection mediates itself with itself through its negation. Ground is connection of immediate content determinations. As form of immediate determinations, connects itself with itself as self-identical while connecting with their negation. Ground not in and for itself but as connected with sublated ground-connection. Sublated connection is real ground not in and for itself. Ground-connection is essentially presupposing reflection. Formal ground presupposes immediate content determination. Content presupposes form as real ground. Ground is form as immediate linkage but repels itself from itself, presupposes immediacy. Immediate is content determination, simple ground. But as ground, equally repelled from itself and refers itself to itself as to other. Total ground-connection has taken on determination of conditioning mediation.',
  provenance: {
    sourceChunk: 'det-14',
    sourceOp: 'det-op-14-complete-mediation',
  },
};

export const determinateGroundOperations: LogicalOperation[] = [
  detOp1FormalGround,
  detOp2FormalMediation,
  detOp3DeterminateGround,
  detOp4GroundIsSufficient,
  detOp5RealGround,
  detOp6GroundRealized,
  detOp7ConnectionDeterminesItself,
  detOp8RealGroundingConnection,
  detOp9GroundBreaksDown,
  detOp10CompleteGround,
  detOp11CompleteGroundFormalAndReal,
  detOp12GroundConnectionDetermined,
  detOp13GroundConnectionReal,
  detOp14CompleteMediation,
];
