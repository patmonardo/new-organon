import type { Chunk, LogicalOperation } from './index'

/*
  Essence — B. DETERMINATE GROUND

  This module consolidates the complete Determinate Ground section:
  - Part a: Formal Ground
  - Part b: Real Ground
  - Part c: Complete Ground

  PHILOSOPHICAL NOTES:

  1. **Dependent Origination as Determinate Ground**:
     Determinate Ground is Dependent Origination made determinate — the
     systematic structure of conditioned arising with specific content.
     Formal Ground shows the structure of conditioning (tautological),
     Real Ground shows the diversity of conditions (external reference),
     and Complete Ground shows the unity of both (conditioning mediation).

  2. **Theory of Forms as Determinate Ground**:
     Determinate Ground is the Theory of Forms made determinate — formal
     cause (Formal Ground), material cause (Real Ground), and final cause
     (Complete Ground) unified in a single systematic structure. The movement
     from Formal → Real → Complete Ground is the systematic determination
     of the four causes within Dependent Origination.

  3. **The Big Kahuna Continued**:
     Determinate Ground continues "the Big Kahuna" — showing how Dependent
     Origination and Theory of Forms are systematically determined through
     the threefold structure of Formal, Real, and Complete Ground.

  4. **Ground as LogoGenesis**:
     Determinate Ground is Ground made determinate — the systematic structure
     where Logic (Upadhi) structures Consciousness (Citta) into Science with
     specific content. Complete Ground is the LogoGenesis completed — where
     Science realizes itself as Truth, culminating in the Syllogism of Necessity.
*/

// ============================================================================
// PART A: FORMAL GROUND
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'dg-chunk-1-intro',
    title: 'Formal ground — introduction',
    text: `The determinate ground is a ground with a determinate content. Form treats that determinateness as substrate: the simple immediate that stands against form's mediation. Formal ground is the negative self-referring identity that posits itself and thereby constitutes the mediating factor.`
  },
  {
    id: 'dg-chunk-2-content-as-substrate',
    title: 'Content as substrate and indifferent unity',
    text: `In the determinate ground the content is the identical element common to both ground and grounded. The content is indifferent to their distinction; it functions as the substrate or positive unity through which formal mediation operates.`
  },
  {
    id: 'dg-chunk-3-negative-self-reference',
    title: 'Negative self-reference and positedness',
    text: `The ground refers negatively to itself: in its negativity it is self-identical and makes itself into a positedness. That negative reference is the immediate determinateness by which the ground acquires determinate content.`
  },
  {
    id: 'dg-chunk-4-formal-mediation-unity',
    title: 'Formal mediation: unity of the two sides',
    text: `Formal mediation is the negative unity in which both sides (ground and grounded) pass into each other and mutually posit themselves into one sublated identity. This mediation is the form proper: unity of pure reflection and determining reference applied to the determinate content.`
  },
  {
    id: 'dg-chunk-5-two-sided-perspective',
    title: 'Two-sided consideration: ground vs grounded',
    text: `A determinate content can be considered from two perspectives: as ground and as grounded. Each side is equally the whole mediation; the transition may be taken from either side because the content itself remains identical and indifferent to form.`
  },
  {
    id: 'dg-chunk-6-sufficiency-and-limitation',
    title: 'Sufficiency of the ground (limited) and formal ground',
    text: `Because ground and grounded share identity of content and form, the ground is sufficient (relative to this relation): nothing in the grounded is not in the ground. But this sufficiency is limited: when determinateness is still simple and undifferentiated we have only a formal ground, with content external to form.`
  },
  {
    id: 'dg-chunk-7-formal-ground-conclusion',
    title: 'Conclusion: formal ground vs real/complete ground',
    text: `Formal ground is the present stage where determinateness is one simple content indifferent to form. It is the formal moment in the development toward real and complete ground — the basis for subsequent analyses (real ground, complete ground) and for syllogisms of reflection.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'dg-op-1-intro',
    chunkId: 'dg-chunk-1-intro',
    label: 'DeterminateGround = ground + determinateContent (formal)',
    clauses: [
      'determinateGround.has(content)',
      'form.treats(content) as substrate',
      'ground = negativeSelfReferringIdentity -> positedness'
    ],
    predicates: [
      { name: 'HasContent', args: ['determinateGround','content'] },
      { name: 'TreatsAsSubstrate', args: ['form','content'] }
    ],
    relations: [
      { predicate: 'has', from: 'determinateGround', to: 'content' },
      { predicate: 'treatsAs', from: 'form', to: 'substrate' }
    ]
  },
  {
    id: 'dg-op-2-content-substrate',
    chunkId: 'dg-chunk-2-content-as-substrate',
    label: 'Content = identical element; substrate indifferent to form',
    clauses: [
      'content.isIdenticalElementOf(ground, grounded)',
      'content.indifferentTo(form)',
      'content.constitutes(positiveUnity)'
    ],
    predicates: [
      { name: 'IsIdenticalElement', args: ['content','ground+grounded'] },
      { name: 'IsIndifferentTo', args: ['content','form'] }
    ],
    relations: [
      { predicate: 'constitutes', from: 'content', to: 'positiveUnity' }
    ]
  },
  {
    id: 'dg-op-3-negative-self-reference',
    chunkId: 'dg-chunk-3-negative-self-reference',
    label: 'Ground refers negatively to itself → positedness',
    clauses: [
      'ground.refersNegativelyTo(ground)',
      'negativity => selfIdentity && positedness',
      'posited.content = determinateContent'
    ],
    predicates: [
      { name: 'RefersNegativelyTo', args: ['ground','itself'] },
      { name: 'IsPositedness', args: ['ground'] }
    ],
    relations: [
      { predicate: 'refersNegatively', from: 'ground', to: 'itself' },
      { predicate: 'yields', from: 'negativity', to: 'positedness' }
    ]
  },
  {
    id: 'dg-op-4-formal-mediation',
    chunkId: 'dg-chunk-4-formal-mediation-unity',
    label: 'Formal mediation = negative unity; mutual positing',
    clauses: [
      'formalMediation = negativeUnity(form)',
      'sides.passIntoEachOther() => mutualPositing',
      'formalMediation.refersTo(determinateContent) as mediatingFactor'
    ],
    predicates: [
      { name: 'IsFormalMediation', args: ['formalMediation'] },
      { name: 'MutualPositing', args: ['sides'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'formalMediation', to: 'determinateContent' }
    ]
  },
  {
    id: 'dg-op-5-two-sided-perspective',
    chunkId: 'dg-chunk-5-two-sided-perspective',
    label: 'Ground and grounded are both whole form; perspective reversible',
    clauses: [
      'content.consideredAs(ground) || content.consideredAs(grounded)',
      'eitherTransition.valid -> bothSides.wholeMediation',
      'form.isExternalTo(simpleDeterminateness)'
    ],
    predicates: [
      { name: 'IsEitherPerspective', args: ['content'] },
      { name: 'WholeMediation', args: ['sides'] }
    ],
    relations: [
      { predicate: 'reversibleFrom', from: 'transition', to: 'eitherSide' }
    ]
  },
  {
    id: 'dg-op-6-sufficiency-and-formal-limit',
    chunkId: 'dg-chunk-6-sufficiency-and-limitation',
    label: 'Ground sufficiency (relative) and formal ground limitation',
    clauses: [
      'ground.isSufficientRelativeTo(relation) => nothingIn(grounded) notIn(ground)',
      'if content.simpleDeterminateness then ground = formalGround',
      'formalGround.content.externalToForm'
    ],
    predicates: [
      { name: 'IsSufficientRelative', args: ['ground'] },
      { name: 'IsFormalGround', args: ['ground'] }
    ],
    relations: [
      { predicate: 'isSufficientFor', from: 'ground', to: 'grounded' },
      { predicate: 'limits', from: 'formalGround', to: 'form' }
    ]
  },
  {
    id: 'dg-op-7-conclusion-formal-ground',
    chunkId: 'dg-chunk-7-formal-ground-conclusion',
    label: 'Formal ground = stage before real/complete ground; basis for syllogisms of reflection',
    clauses: [
      'formalGround.present => nextAnalyse(realGround, completeGround)',
      'formalGround = basisFor(syllogismsOfReflection)'
    ],
    predicates: [
      { name: 'IsBasisFor', args: ['formalGround','syllogismsOfReflection'] }
    ],
    relations: [
      { predicate: 'prepares', from: 'formalGround', to: 'realCompleteGroundAnalysis' }
    ]
  }
]

// ============================================================================
// PART B: REAL GROUND
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'dg1-chunk-1-intro-real-ground',
    title: 'Real ground — diversity of content and realization',
    text: `When the ground and the grounded possess diverse contents the ground-connection ceases to be merely formal: asking for a ground now demands a different content-determination. This diversity realizes the ground — the turning-back is no longer tautological.`
  },
  {
    id: 'dg1-chunk-2-two-immediates-and-indifference',
    title: 'Two immediates, mutual indifference, and empty unity',
    text: `As the two sides acquire different contents they become immediate and self-identical, indifferent to one another. Their unity is a negative unity that, being an empty external reference, holds them together without being a genuine mediation.`
  },
  {
    id: 'dg1-chunk-3-continuous-extension-and-essential-compactness',
    title: 'Ground-content extends into positedness — essential compactness',
    text: `In one respect the ground-content extends into the positedness so fully that the grounded contains the ground within itself — a compact essential identity. Anything beyond this essential element in the grounded is unessential, an external manifold not of the ground.`
  },
  {
    id: 'dg1-chunk-4-unessential-manifold-and-substrate',
    title: 'Unessential manifold as indifferent substrate',
    text: `The unessential manifold present in the grounded is a positive, indifferent substrate. It resides within the grounded but does not posit itself as grounded content; it is external to the ground-connection and therefore not grounded by it.`
  },
  {
    id: 'dg1-chunk-5-connection-as-external-tie',
    title: 'Connection becomes external tie not true mediation',
    text: `The tie that links distinct contents is not a form-reference holding positedness in place; rather it is an external bond that fails to make the unessential manifold truly posited. The real ground thus breaks into different substrates and external determinations.`
  },
  {
    id: 'dg1-chunk-6-loss-of-self-identity-of-ground-connection',
    title: 'Self-identity of ground vanishes; ground-connection externalized',
    text: `Because of content diversity the self-identical form of the ground (one thing alternating essential and posited) disappears. The ground-connection becomes external to itself and no longer contains its mediation immanently.`
  },
  {
    id: 'dg1-chunk-7-conclusion-real-ground-as-reference-to-another',
    title: 'Conclusion: real ground = reference to another content',
    text: `The real ground is an external reference: a content-determination refers to another content, and the form (ground-connection) itself refers to something immediate not posited by it. Real ground is thus grounded by a further, distinct content.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'dg1-op-1-intro-real-ground',
    chunkId: 'dg1-chunk-1-intro-real-ground',
    label: 'Diverse contents → ground realized; demand another content as ground',
    clauses: [
      'if ground.content != grounded.content then ground.isRealized',
      'askingForGround => demand(differentContent)'
    ],
    predicates: [
      { name: 'IsRealized', args: ['ground'] },
      { name: 'DemandsDifferentContent', args: ['query'] }
    ],
    relations: [
      { predicate: 'requires', from: 'requestForGround', to: 'differentContent' }
    ]
  },
  {
    id: 'dg1-op-2-indifferent-immediates-empty-unity',
    chunkId: 'dg1-chunk-2-two-immediates-and-indifference',
    label: 'Different contents → immediates indifferent; unity empty reference',
    clauses: [
      'sideA.isImmediate && sideB.isImmediate',
      'sideA.indifferentTo(sideB)',
      'unity = emptyReference(sideA, sideB)'
    ],
    predicates: [
      { name: 'IsImmediate', args: ['side'] },
      { name: 'IsEmptyUnity', args: ['unity'] }
    ],
    relations: [
      { predicate: 'holdsExternally', from: 'unity', to: 'sides' }
    ]
  },
  {
    id: 'dg1-op-3-compactness-and-unessential',
    chunkId: 'dg1-chunk-3-continuous-extension-and-essential-compactness',
    label: 'Ground-content extends into positedness; unessential manifold external',
    clauses: [
      'ground.content.extendsInto(positedness)',
      'grounded.contains(groundContentFully)',
      'otherElementsInGrounded = unessentialManifold (external)'
    ],
    predicates: [
      { name: 'ExtendsInto', args: ['groundContent', 'positedness'] },
      { name: 'IsUnessentialManifold', args: ['elements'] }
    ],
    relations: [
      { predicate: 'containsFully', from: 'grounded', to: 'groundContent' },
      { predicate: 'isExternalTo', from: 'unessentialManifold', to: 'groundConnection' }
    ]
  },
  {
    id: 'dg1-op-4-unessential-substrate',
    chunkId: 'dg1-chunk-4-unessential-manifold-and-substrate',
    label: 'Unessential manifold = indifferent substrate, not posited by ground',
    clauses: [
      'unessential.isPositiveIndifferentElement',
      'unessential.doesNotPositItselfAsGrounded',
      'unessential.remaining => substrateOnly'
    ],
    predicates: [
      { name: 'IsIndifferentSubstrate', args: ['unessential'] },
      { name: 'NotPositedByGround', args: ['unessential'] }
    ],
    relations: [
      { predicate: 'residesIn', from: 'unessential', to: 'grounded' },
      { predicate: 'notPositedBy', from: 'unessential', to: 'ground' }
    ]
  },
  {
    id: 'dg1-op-5-connection-external-tie',
    chunkId: 'dg1-chunk-5-connection-as-external-tie',
    label: 'Connection becomes external tie; real ground breaks into substrates',
    clauses: [
      'connection = externalTie(distinctContents)',
      'externalTie.doesNotPosit(unessentialManifold)',
      'result => differentSubstratesExist'
    ],
    predicates: [
      { name: 'IsExternalTie', args: ['connection'] },
      { name: 'BreaksIntoSubstrates', args: ['ground'] }
    ],
    relations: [
      { predicate: 'bindsExternally', from: 'connection', to: 'distinctContents' },
      { predicate: 'yields', from: 'externalTie', to: 'multipleSubstrates' }
    ]
  },
  {
    id: 'dg1-op-6-loss-self-identity',
    chunkId: 'dg1-chunk-6-loss-of-self-identity-of-ground-connection',
    label: 'Diverse content → loss of self-identity; ground-connection externalized',
    clauses: [
      'if contents.diverse then selfIdenticalForm.vanishes',
      'groundConnection.externalized = true'
    ],
    predicates: [
      { name: 'Vanishes', args: ['selfIdenticalForm'] },
      { name: 'IsExternalized', args: ['groundConnection'] }
    ],
    relations: [
      { predicate: 'becomes', from: 'selfIdenticalForm', to: 'vanishedState' },
      { predicate: 'externalizes', from: 'groundConnection', to: 'itself' }
    ]
  },
  {
    id: 'dg1-op-7-real-ground-is-reference',
    chunkId: 'dg1-chunk-7-conclusion-real-ground-as-reference-to-another',
    label: 'Real ground = reference to another content or immediate',
    clauses: [
      'realGround = reference(contentA -> contentB)',
      'formRefersTo(immediateNotPositedByIt) => groundIsExternalReference'
    ],
    predicates: [
      { name: 'IsReferenceToAnother', args: ['realGround'] },
      { name: 'RefersToImmediate', args: ['form'] }
    ],
    relations: [
      { predicate: 'refers', from: 'realGround', to: 'otherContent' },
      { predicate: 'groundsByReference', from: 'form', to: 'immediate' }
    ]
  }
)

// ============================================================================
// PART C: COMPLETE GROUND
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'dg2-chunk-1-complete-ground-intro',
    title: 'Complete ground — introduction',
    text: `Real ground returns to its own ground: the previously posited, external link is sublated into a new ground that contains both formal and real moments. The complete ground unifies the formal and the real, mediating content determinations that formerly confronted each other immediately.`
  },
  {
    id: 'dg2-chunk-2-two-connections-and-new-ground',
    title: 'Two connections; new ground contains same contents and linkages',
    text: `When a posited link is grounded, the new ground is determined so that it is identical with the ground it grounds: both sides share the same content determinations and their linkage is present in the new ground as the immanent reflection of that link.`
  },
  {
    id: 'dg2-chunk-3-formality-reasserts-and-completeness',
    title: 'Formality reasserts; complete ground contains formal + real',
    text: `Because real ground returns to its ground, the formality of ground reasserts itself; the newly arisen ground-connection is complete, containing within itself both the formal and the real ground and mediating the previously immediate confrontations.`
  },
  {
    id: 'dg2-chunk-4-something-has-a-ground-two-determinations',
    title: 'Something has a ground — contains ground-determination and posited determination',
    text: `A something that has a ground contains the content-determination that is ground and a second determination posited by that ground. Due to content indifference, neither determination alone constitutes ground in itself; the connection is posited and has its ground in another connection.`
  },
  {
    id: 'dg2-chunk-5-general-linking-and-relative-ground',
    title: 'General linking; relative ground vs true absolute connection',
    text: `The second connection is distinguished only by form and shares the same contents; the linking is general and produces content diversification indifferent to each other. This linking is not the true absolute connection but a relative ground supported by a something that connects them immediately.`
  },
  {
    id: 'dg2-chunk-6-two-somethings-andKindsOfConnection',
    title: 'Two somethings: distinct connections of content with identical whole',
    text: `There are two distinct somethings — two connections of content — that form one whole content (two determinations + their connection). They differ solely by the kind of connection (immediate vs posited); thus ground and grounded are distinguished only according to form.`
  },
  {
    id: 'dg2-chunk-7-formal-to-real-transition-and-self-subsistence',
    title: 'Formal → Real transition; one content becomes self-subsistent and grounded',
    text: `Formal ground passes into real ground as the moments reflect into themselves and become self-subsistent content. One content determination becomes the essential, identical substrate and ground of the other, which is posited within the second something.`
  },
  {
    id: 'dg2-chunk-8-mediation-through-first-something',
    title: 'Mediation through the first something; ground-of-ground inference',
    text: `Because B is implicitly linked with A in a first something, in a second something where only A is immediate, B is nonetheless linked to it via the original connection. Thus the connection in the first something mediates and becomes the ground of the ground A in the second something.`
  },
  {
    id: 'dg2-chunk-9-complete-ground-as-self-external-reflection',
    title: 'Complete ground = self-external reflection; identity restored via negation',
    text: `Real ground is the self-external reflection of ground; its complete mediation restores identity with itself. The ground-connection thus mediates itself through negation: it is simultaneously self-positing and self-sublating, connecting immediate determinations with their negations.`
  },
  {
    id: 'dg2-chunk-10-conditioning-mediation-and-conclusion',
    title: 'Ground as conditioning mediation; conclusion',
    text: `The total ground-connection presupposes reflection: formal ground presupposes immediate content and that content presupposes form as real ground. Ground is a linkage that refers to immediacy while also referring itself as to another — in short, the complete ground is conditioning mediation.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'dg2-op-1-complete-intro',
    chunkId: 'dg2-chunk-1-complete-ground-intro',
    label: 'CompleteGround := formal + real moments united; mediates confronted contents',
    clauses: [
      'completeGround.contains(formalGround, realGround)',
      'completeGround.mediates(contentDeterminations)'
    ],
    predicates: [
      { name: 'ContainsMoments', args: ['completeGround', 'formal+real'] },
      { name: 'Mediates', args: ['completeGround', 'contents'] }
    ],
    relations: [
      { predicate: 'contains', from: 'completeGround', to: 'formal+real' },
      { predicate: 'mediates', from: 'completeGround', to: 'contentDeterminations' }
    ]
  },
  {
    id: 'dg2-op-2-two-connections-new-ground',
    chunkId: 'dg2-chunk-2-two-connections-and-new-ground',
    label: 'New ground mirrors contents and linkage; immanent reflection',
    clauses: [
      'newGround.identicalWith(previousGround).inContentAndLinkage',
      'newGround.linkage = immanentReflection(previousLink)'
    ],
    predicates: [
      { name: 'IsIdenticalWith', args: ['newGround', 'previousGround'] },
      { name: 'IsImmanentReflection', args: ['linkage'] }
    ],
    relations: [
      { predicate: 'mirrors', from: 'newGround', to: 'previousContents+link' }
    ]
  },
  {
    id: 'dg2-op-3-formality-reasserts',
    chunkId: 'dg2-chunk-3-formality-reasserts-and-completeness',
    label: 'Return to ground => formality reasserts; complete ground contains both forms',
    clauses: [
      'if realGround.returnsToGround then formality.reasserts',
      'completeGround.includes(formal, real)'
    ],
    predicates: [
      { name: 'ReassertsFormality', args: ['process'] },
      { name: 'Includes', args: ['completeGround', 'moments'] }
    ],
    relations: [
      { predicate: 'includes', from: 'completeGround', to: 'formal+real' }
    ]
  },
  {
    id: 'dg2-op-4-something-has-ground',
    chunkId: 'dg2-chunk-4-something-has-a-ground-two-determinations',
    label: 'Something.hasGround => contains groundDetermination + positedDetermination; ground in another connection',
    clauses: [
      'something.contains(groundDetermination, positedDetermination)',
      'connectionOfThese.presupposes(anotherConnection)'
    ],
    predicates: [
      { name: 'ContainsDeterminations', args: ['something'] },
      { name: 'PresupposesAnotherConnection', args: ['connection'] }
    ],
    relations: [
      { predicate: 'contains', from: 'something', to: 'determinations' },
      { predicate: 'presupposes', from: 'connection', to: 'anotherConnection' }
    ]
  },
  {
    id: 'dg2-op-5-general-linking-relative-ground',
    chunkId: 'dg2-chunk-5-general-linking-and-relative-ground',
    label: 'General linking yields relative ground; contents remain indifferent',
    clauses: [
      'linking.isGeneral && contents.indifferent => ground.isRelative',
      'relativeGround.supportedBy(something) not trueAbsoluteConnection'
    ],
    predicates: [
      { name: 'IsRelativeGround', args: ['link'] },
      { name: 'IsGeneralLinking', args: ['link'] }
    ],
    relations: [
      { predicate: 'supports', from: 'relativeGround', to: 'contents' }
    ]
  },
  {
    id: 'dg2-op-6-two-somethings',
    chunkId: 'dg2-chunk-6-two-somethings-andKindsOfConnection',
    label: 'Two somethings: same whole content, different connection kinds (immediate vs posited)',
    clauses: [
      'twoSomethings.share(wholeContent)',
      'distinction := kindOfConnection (immediate | posited)'
    ],
    predicates: [
      { name: 'SharesWholeContent', args: ['somethings'] },
      { name: 'KindOfConnection', args: ['connection'] }
    ],
    relations: [
      { predicate: 'distinguishedBy', from: 'somethings', to: 'connectionKind' }
    ]
  },
  {
    id: 'dg2-op-7-formal-to-real-transition',
    chunkId: 'dg2-chunk-7-formal-to-real-transition-and-self-subsistence',
    label: 'Formal → Real: moments reflect into self-subsistence; one content becomes essential ground',
    clauses: [
      'formalGround.transformsInto(realGround) via reflectionIntoSelf',
      'oneContent.becomes(essentialGround) for the other'
    ],
    predicates: [
      { name: 'TransformsInto', args: ['formal', 'real'] },
      { name: 'BecomesEssentialGround', args: ['content'] }
    ],
    relations: [
      { predicate: 'grounds', from: 'essentialContent', to: 'otherContent' }
    ]
  },
  {
    id: 'dg2-op-8-mediation-through-first',
    chunkId: 'dg2-chunk-8-mediation-through-first-something',
    label: 'Mediation: original connection in first something grounds the second',
    clauses: [
      'if B.linkedWith(A) in firstSomething then B.linkedWith(A) in secondSomething',
      'firstConnection.mediates(secondConnection) => groundOfGroundInference'
    ],
    predicates: [
      { name: 'IsMediatedBy', args: ['second', 'first'] },
      { name: 'GroundOfGround', args: ['inference'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'firstConnection', to: 'secondConnection' }
    ]
  },
  {
    id: 'dg2-op-9-complete-as-self-external',
    chunkId: 'dg2-chunk-9-complete-ground-as-self-external-reflection',
    label: 'CompleteGround = self-external reflection; self-positing & self-sublating',
    clauses: [
      'realGround = selfExternalReflection(ground)',
      'completeConnection.selfPosits && completeConnection.selfSublates'
    ],
    predicates: [
      { name: 'IsSelfExternalReflection', args: ['realGround'] },
      { name: 'SelfPositsAndSublates', args: ['connection'] }
    ],
    relations: [
      { predicate: 'reflects', from: 'completeGround', to: 'itself' }
    ]
  },
  {
    id: 'dg2-op-10-conditioning-mediation',
    chunkId: 'dg2-chunk-10-conditioning-mediation-and-conclusion',
    label: 'Ground as conditioning mediation: formal presupposes immediate and vice versa',
    clauses: [
      'formalGround.presupposes(immediateContent)',
      'immediateContent.presupposes(form.asRealGround)',
      'groundConnection := conditioningMediation'
    ],
    predicates: [
      { name: 'PresupposesImmediate', args: ['formalGround'] },
      { name: 'IsConditioningMediation', args: ['ground'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'formalGround', to: 'immediateContent' },
      { predicate: 'conditions', from: 'groundConnection', to: 'contentDeterminations' }
    ]
  }
)

// Accessors
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS
}

export function getLogicalOpsForChunk(oneBasedIndex: number): LogicalOperation | null {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return null
  return LOGICAL_OPERATIONS.find(op => op.chunkId === chunk.id) ?? null
}

