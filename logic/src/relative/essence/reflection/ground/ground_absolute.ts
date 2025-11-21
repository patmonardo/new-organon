import type { Chunk, LogicalOperation } from './index'

/*
  Essence — A. ABSOLUTE GROUND

  This module consolidates the complete Absolute Ground section:
  - Part a: Form and Essence
  - Part b: Form and Matter
  - Part c: Form and Content

  THE LOGICALLY PROFOUND TRANSITION: FOUNDATION → GROUND
  This is where Identity-Difference (Foundation) becomes Form-Matter (Ground).
  This transition is one of the most profound logical moves in the System of Reflection:
  - Identity (Foundation) → Form (Ground): The self-identical becomes the determining
  - Difference (Foundation) → Matter (Ground): The differentiated becomes the substrate
  
  This is the systematic foundation that makes Reflection explicit and computable.
  It is not surprising that ML researchers struggle to codify "Reflection" — this
  structure requires the full systematic development from Foundation to Ground.

  PHILOSOPHICAL NOTES:

  1. **Dependent Origination (Pratītyasamutpāda)**:
     Ground is the Buddhist concept of Dependent Origination made explicit —
     the systematic structure of conditioned arising. Everything determinate
     arises through conditions (ground) and is grounded in conditions.
     The movement from Form → Matter → Content is the systematic unfolding
     of dependent origination: form conditions matter, matter conditions content,
     and content is the unity that conditions both.

  2. **Theory of Forms (Platonic/Aristotelian)**:
     This is also the Theory of Forms unified with Dependent Origination.
     Form is the active principle (Platonic Form, Aristotelian Formal Cause),
     Matter is the passive substrate (Aristotelian Material Cause),
     and Content is the informed unity (Aristotelian Final Cause).
     The synthesis of Form, Matter, and Content completes the four causes
     within a single systematic structure.

  3. **The Big Kahuna**:
     This is "the Big Kahuna" — Dependent Origination and Theory of Forms
     in one package. It is the systematic structure that shows how
     conditioned arising (Dependent Origination) and formal determination
     (Theory of Forms) are one and the same movement: the self-determination
     of essence through ground.

  4. **Ground as LogoGenesis: The Synthesis of Cit, Citi, and Citta**:
     Ground completes Pure Reason — Ground grounds the Syllogism. This is
     the profound synthesis where Logic (as Upadhi) meets Consciousness (Citta),
     resulting in Science itself.
     
     Ground is the Unity of Cit (Pure Consciousness) and Citi (Power of
     Consciousness) → Citta (Mind as Science/Laws of Science). The Cogito
     ("I think") is Citta — Mind as Science itself, or rather the Laws of Science.
     
     The Determinations of Reflection (Identity, Difference, Contradiction)
     are the Laws of Logic. Ground is the Synthesis of Mind (Citta) and Logic
     (Upadhi) — Logic as Upadhi applied to Consciousness results in Ground as
     Science. This is the LogoGenesis — the genesis/creation of Science itself.
     
     The truth of this LogoGenesis is found in the Syllogism of Necessity,
     which is the completion of Ground — where Science realizes itself as Truth.
*/

// ============================================================================
// PART A: FORM AND ESSENCE
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ag-chunk-1-intro',
    title: 'Absolute Ground — introduction',
    text: `The determination of reflection that returns into ground is the first immediate existence from which beginning is made. Existence here still means positedness and presupposes ground: the immediate is posited while the ground is the non-posited.`
  },
  {
    id: 'ag-chunk-2-reflection-and-positedness',
    title: 'Reflection, positing and sublation',
    text: `The positing rebounds on the positor; the positing is a sublating of itself. As sublated determinateness the ground is essence determined through itself, yet determined as indeterminate — essence identical with itself in its negativity.`
  },
  {
    id: 'ag-chunk-3-twofold-determinateness',
    title: 'Twofold determinateness: ground and grounded',
    text: `Determinateness of essence is twofold: (1) essence as ground (non-posited), and (2) the grounded, the immediate as positedness. Both are identical with themselves but in the identity of the negative with itself; the ground is the self-identity of positedness.`
  },
  {
    id: 'ag-chunk-4-mediation-unity-of-reflections',
    title: 'Mediation: unity of pure and determining reflection',
    text: `This mediation is neither pure reflection nor purely determining reflection but their unity: determinations have a posited self-subsistence and that subsistence is itself posited. Thus the determinations are distinguished from simple identity and constitute form as against essence.`
  },
  {
    id: 'ag-chunk-5-essence-form-and-determinations',
    title: 'Essence, form, and determinate immediacy',
    text: `Essence has form and determinations of form; only as ground does it have fixed immediacy. Essence is inseparable from its movement and is not a substrate preceding reflection; determinations of form rest on essence as an indeterminate that is indifferent to them.`
  },
  {
    id: 'ag-chunk-6-form-determinations-and-ground-connection',
    title: 'Form-determinations: identity, difference, ground-connection',
    text: `Form-determinations (identity, difference, diversity/opposition) include the ground-connection: through it essence is posited while remaining the substrate. The immanent identity with ground does not belong to form; ground posits subsistence for form.`
  },
  {
    id: 'ag-chunk-7-reciprocal-reference-form-and-essence',
    title: 'Reciprocal connecting reference of form and essence',
    text: `Essence is the simple unity of ground and grounded and thereby becomes itself a moment of form. In this absolute reciprocal reference the substrate (essence) and determinate form mutually constitute one another: form is grounded and ground becomes a moment of form.`
  },
  {
    id: 'ag-chunk-8-form-completed-whole',
    title: 'Form as the completed whole of reflection',
    text: `Form is the completed whole of reflection; it contains the determination that it is sublated and is therefore a unity of its determining moments while being referred to a sublatedness outside itself.`
  },
  {
    id: 'ag-chunk-9-form-vs-essence-and-positionality',
    title: 'Form (positing) vs simple essence (inert substrate)',
    text: `As self-referring negativity, form is positing and determining; simple essence is an indeterminate, inert substrate in which form's determinations have their subsistence and are reflected into themselves. The distinction of form and essence is necessary but their distinguishing is itself their unity.`
  },
  {
    id: 'ag-chunk-10-form-as-absolute-negativity',
    title: 'Form as absolute negativity; identity of form and essence',
    text: `Form is the negative absolute self-identity by virtue of which essence is essence; abstractly taken, this identity appears as essence against form, yet in truth the whole self-referring negativity is simple essence — form has essence within its identity and essence has form in its negative nature.`
  },
  {
    id: 'ag-chunk-11-form-makes-determination-positedness',
    title: 'Form's determining = making determination posited',
    text: `Form is the internal reflective shining of essence; in its determining, form makes determinations into positedness as positedness — the movement by which determinations acquire their positive, reflected status.`
  },
  {
    id: 'ag-chunk-12-form-sublates-distinguishing',
    title: 'Form sublates the distinguishing; determines essence by self-sublation',
    text: `Form does not stand prior to essence as an external presupposition; rather, by distinguishing it sublates that distinction and becomes the ground of its own determinations. Thus form determines essence by making the distinguishing itself into a sublated self-identity.`
  },
  {
    id: 'ag-chunk-13-form-as-groundful-essence',
    title: 'Form as groundful essence — self-identity in determination',
    text: `Form is the contradiction of being both posited (and sublated) and yet having subsistence in that sublatedness. It is ground as essence, self-identical through being determined or negated; the determination is the mode in which essence has subsistence.`
  },
  {
    id: 'ag-chunk-14-transition-form-essence-to-matter',
    title: 'Transition — form and essence as moments; essence determined as matter',
    text: `The distinctions of form and essence are moments of a single reference of form to itself. Determining form refers to itself as sublated positedness and thereby presupposes its identity; in this aspect essence appears as the indeterminate other, as formless identity — matter.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ag-op-1-intro',
    chunkId: 'ag-chunk-1-intro',
    label: 'Reflection returns → immediate positedness presupposes ground',
    clauses: [
      'reflection.returnsInto(ground)',
      'immediate.isPositedness && positedness.presupposes(ground)',
      'ground = nonPosited'
    ],
    predicates: [
      { name: 'ReturnsInto', args: ['reflection','ground'] },
      { name: 'IsPositedness', args: ['immediate'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'immediate', to: 'ground' },
      { predicate: 'is', from: 'ground', to: 'nonPosited' }
    ]
  },
  {
    id: 'ag-op-2-reflection-positedness-sublation',
    chunkId: 'ag-chunk-2-reflection-and-positedness',
    label: 'Positing rebounds → positing is self-sublation; ground = essence-in-negativity',
    clauses: [
      'positing.reboundsOn(positor)',
      'positing == sublatingOfItself',
      'ground = essence.determinedThroughItself(asIndeterminate)'
    ],
    predicates: [
      { name: 'ReboundsOn', args: ['positing','positor'] },
      { name: 'IsSublation', args: ['positing'] },
      { name: 'EssenceDetermined', args: ['ground'] }
    ],
    relations: [
      { predicate: 'reboundOn', from: 'positing', to: 'positor' },
      { predicate: 'isSublated', from: 'ground', to: 'essence' }
    ]
  },
  {
    id: 'ag-op-3-twofold-determinateness',
    chunkId: 'ag-chunk-3-twofold-determinateness',
    label: 'Twofold determinateness → ground vs grounded identity',
    clauses: [
      'essence.asGround = nonPositedness',
      'grounded = immediate.asPositedness',
      'identityOfNegative && identityOfPositive => singleIdentity'
    ],
    predicates: [
      { name: 'IsGround', args: ['essence'] },
      { name: 'IsGrounded', args: ['immediate'] },
      { name: 'NegativeIdentity', args: ['ground','grounded'] }
    ],
    relations: [
      { predicate: 'contrasts', from: 'ground', to: 'positedness' },
      { predicate: 'identicalWith', from: 'ground', to: 'grounded' }
    ]
  },
  {
    id: 'ag-op-4-mediation-unity',
    chunkId: 'ag-chunk-4-mediation-unity-of-reflections',
    label: 'Mediation = unity of pure & determining reflection; determinations self-subsist as posited',
    clauses: [
      'mediation = pureReflection ∪ determiningReflection',
      'determinations.havePositedSubsistence',
      'positedSubsistence => determinationsDistinguishedFromSimpleIdentity'
    ],
    predicates: [
      { name: 'IsMediation', args: ['mediation'] },
      { name: 'HasPositedSubsistence', args: ['determinations'] }
    ],
    relations: [
      { predicate: 'unites', from: 'mediation', to: 'pureReflection+determiningReflection' },
      { predicate: 'givesSubsistence', from: 'mediation', to: 'determinations' }
    ]
  },
  {
    id: 'ag-op-5-essence-form-determinations',
    chunkId: 'ag-chunk-5-essence-form-and-determinations',
    label: 'Essence inseparable from movement; form determinations rest on indeterminate essence',
    clauses: [
      'essence.isNotPriorToMovement',
      'form.determinations.haveFoundationIn(indeterminateEssence)',
      'determinations.reflectedIntoThemselves'
    ],
    predicates: [
      { name: 'NotPrior', args: ['essence','movement'] },
      { name: 'FoundationIn', args: ['form','essence'] }
    ],
    relations: [
      { predicate: 'restsOn', from: 'form', to: 'essence' },
      { predicate: 'reflectsInto', from: 'determinations', to: 'themselves' }
    ]
  },
  {
    id: 'ag-op-6-form-determinations-ground-connection',
    chunkId: 'ag-chunk-6-form-determinations-and-ground-connection',
    label: 'Form determinations (identity/difference) and ground-connection as form-determination',
    clauses: [
      'form.determinations = {identity, difference, diversity, opposition}',
      'groundConnection ∈ form.determinations',
      'ground.positsSubsistenceFor(form)'
    ],
    predicates: [
      { name: 'FormDeterminations', args: ['form'] },
      { name: 'Includes', args: ['form','groundConnection'] }
    ],
    relations: [
      { predicate: 'includes', from: 'form', to: 'groundConnection' },
      { predicate: 'grounds', from: 'ground', to: 'form' }
    ]
  },
  {
    id: 'ag-op-7-reciprocal-reference',
    chunkId: 'ag-chunk-7-reciprocal-reference-form-and-essence',
    label: 'Reciprocal reference: essence = unity of ground + grounded → becomes moment of form',
    clauses: [
      'essence = unity(ground, grounded)',
      'in unity essence.distinguishesItselfAs(substrate) from form',
      'simultaneously essence.becomes(momentOf(form))'
    ],
    predicates: [
      { name: 'UnityOf', args: ['essence','ground+grounded'] },
      { name: 'BecomesMoment', args: ['essence','form'] }
    ],
    relations: [
      { predicate: 'distinguishesAs', from: 'essence', to: 'substrate' },
      { predicate: 'becomes', from: 'essence', to: 'form' }
    ]
  },
  {
    id: 'ag-op-8-form-completed-whole',
    chunkId: 'ag-chunk-8-form-completed-whole',
    label: 'Form = completed reflection; unity includes reference to sublatedness',
    clauses: [
      'form.isCompletedWholeOf(reflection)',
      'form.contains(determinationOfSublation)',
      'form.unity = unity(ofDeterminingMoments) referredTo(sublatedness)'
    ],
    predicates: [
      { name: 'IsCompletedWhole', args: ['form','reflection'] },
      { name: 'ContainsSublation', args: ['form'] }
    ],
    relations: [
      { predicate: 'contains', from: 'form', to: 'determinationOfSublation' },
      { predicate: 'refersTo', from: 'form', to: 'sublatedness' }
    ]
  },
  {
    id: 'ag-op-9-form-vs-essence',
    chunkId: 'ag-chunk-9-form-vs-essence-and-positionality',
    label: 'Form (positing) vs essence (substrate); their distinguishing is unity',
    clauses: [
      'form = selfReferringNegativity (positing, determining)',
      'essence = indeterminateSubstrate (inert, foundationForDeterminations)',
      'distinction(form, essence) => unityOfDistinguishing'
    ],
    predicates: [
      { name: 'IsSelfReferringNegativity', args: ['form'] },
      { name: 'IsIndeterminateSubstrate', args: ['essence'] },
      { name: 'DistinctionYieldsUnity', args: ['form','essence'] }
    ],
    relations: [
      { predicate: 'grounds', from: 'essence', to: 'determinations' },
      { predicate: 'distinguishes', from: 'form', to: 'essence' }
    ]
  },
  {
    id: 'ag-op-10-form-absolute-negativity',
    chunkId: 'ag-chunk-10-form-as-absolute-negativity',
    label: 'Form as negative absolute self-identity; mutual immanence of form and essence',
    clauses: [
      'form.isNegativeAbsoluteSelfIdentity',
      'identityAbstract(form, essence) => appearsAs(essenceAgainstForm)',
      'inTruth: selfReferringNegativity == simpleEssence',
      'form.contains(essence) && essenceContains(form) (mutual immanence)'
    ],
    predicates: [
      { name: 'IsNegativeAbsoluteIdentity', args: ['form'] },
      { name: 'MutualImmanence', args: ['form','essence'] }
    ],
    relations: [
      { predicate: 'contains', from: 'form', to: 'essence' },
      { predicate: 'imbues', from: 'essence', to: 'form' }
    ]
  },
  {
    id: 'ag-op-11-form-posits-determination',
    chunkId: 'ag-chunk-11-form-makes-determination-positedness',
    label: 'Form's determining → makes determinations positedness',
    clauses: [
      'form = reflectiveShiningOf(essence)',
      'form.determining => make(determination, positedness)',
      'determination.positedness := determination.asPosited'
    ],
    predicates: [
      { name: 'ReflectiveShining', args: ['form','essence'] },
      { name: 'MakesPosited', args: ['form','determination'] }
    ],
    relations: [
      { predicate: 'makes', from: 'form', to: 'determination' },
      { predicate: 'posits', from: 'determination', to: 'positedness' }
    ]
  },
  {
    id: 'ag-op-12-form-sublates-distinguishing',
    chunkId: 'ag-chunk-12-form-sublates-distinguishing',
    label: 'Form sublates its distinguishing → determines essence by self-sublation',
    clauses: [
      'form.distinguishes(essence) => form.sublates(distinguishing)',
      'sublation => form.becomesGroundOf(itsDeterminations)',
      'determinationOfEssence := selfIdentityThroughSublation'
    ],
    predicates: [
      { name: 'SublatesDistinguishing', args: ['form'] },
      { name: 'BecomesGroundOf', args: ['form','determinations'] }
    ],
    relations: [
      { predicate: 'sublates', from: 'form', to: 'distinguishing' },
      { predicate: 'grounds', from: 'form', to: 'determinations' }
    ]
  },
  {
    id: 'ag-op-13-form-groundful-essence',
    chunkId: 'ag-chunk-13-form-as-groundful-essence',
    label: 'Form as groundful essence — self-identity within determination/negation',
    clauses: [
      'form.isPositedAndSublated && form.hasSubsistence',
      'form.asGround = essence.selfIdenticalInDetermination',
      'contradiction(positedness,susbistence) => formIsGround'
    ],
    predicates: [
      { name: 'IsPositedAndSublated', args: ['form'] },
      { name: 'SelfIdenticalInDetermination', args: ['essence'] }
    ],
    relations: [
      { predicate: 'isGroundAs', from: 'form', to: 'essence' },
      { predicate: 'sustains', from: 'form', to: 'subsistence' }
    ]
  },
  {
    id: 'ag-op-14-transition-form-essence-matter',
    chunkId: 'ag-chunk-14-transition-form-essence-to-matter',
    label: 'Transition: form refers to itself as sublated positedness → essence appears as matter',
    clauses: [
      'determiningForm.refersTo(selfAsSublatedPositedness)',
      'this.referring.presupposes(identity) => essence = indeterminateOther',
      'essence.asIndeterminate = matter (formless identity)'
    ],
    predicates: [
      { name: 'RefersToSelf', args: ['determiningForm'] },
      { name: 'EssenceAsMatter', args: ['essence'] }
    ],
    relations: [
      { predicate: 'refersTo', from: 'determiningForm', to: 'sublatedPositedness' },
      { predicate: 'appearsAs', from: 'essence', to: 'matter' }
    ]
  }
]

// ============================================================================
// PART B: FORM AND MATTER
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'agm-chunk-1-matter-as-identity',
    title: 'Matter as the simple identity (form's other)',
    text: `Essence becomes matter when its reflection relates itself to essence as the formless indeterminate. Matter is the simple identity void of distinction, the other of form, and the proper substrate for form's determinations — the self-subsistent term to which form refers for positive subsistence.`
  },
  {
    id: 'agm-chunk-2-abstraction-and-absolute-abstract',
    title: 'Abstraction → matter is the absolutely abstract',
    text: `If one abstracts from every determination and form, what remains is matter: the absolutely abstract. Determinate, perceptible things are always matter+form; matter emerges by the form's reduction of itself to simple identity, not by an external removal of form.`
  },
  {
    id: 'agm-chunk-3-mutual-presupposition',
    title: 'Mutual presupposition of form and matter',
    text: `Form presupposes a matter and matter presupposes form; they do not confront each other externally. Matter is indifferent to form yet is the determinateness of self-identity to which form returns as substrate. Each is not the ultimate ground of the other.`
  },
  {
    id: 'agm-chunk-4-matter-as-passive-form-as-active',
    title: 'Matter passive / Form active — implicit reciprocity',
    text: `Matter is the passive, indifferent substrate; form is the active, self-referring negative (contradictory, self-dissolving, self-determining). Matter implicitly contains form (sublated negativity) and is the absolute receptivity for form; form must inform matter and acquire subsistence there.`
  },
  {
    id: 'agm-chunk-5-form-and-matter-not-ground-each-other',
    title: 'Neither is ground of the other; indifference as common determination',
    text: `Form and matter are alike determined not to be posited by each other. Matter is the identity of ground and grounded as substrate facing form. Their shared determination of indifference constitutes their reciprocal reference rather than a unilateral grounding relation.`
  },
  {
    id: 'agm-chunk-6-informing-and-materializing',
    title: 'Informing and materializing: form must give itself subsistence in matter',
    text: `Form posits itself as sublated and thus presupposes matter; matter contains implicit form and must be informed. Form must materialize itself, giving itself self-identity or subsistence in matter so the determinations of form obtain concrete subsistence.`
  },
  {
    id: 'agm-chunk-7-mutual-self-mediation',
    title: 'Mutual self-mediation: form determines matter and vice versa',
    text: `Form determines matter and matter is determined by form: each implicitly contains the other. The activity of form on matter and matter's reception of form are the sublation of their apparent indifference — a self-mediation of each through its own non-being that restores their original identity.`
  },
  {
    id: 'agm-chunk-8-form-self-sublation-two-sides',
    title: 'Form's two-sided self-sublation and union with matter',
    text: `Form is self-subsisting yet self-sublating; its sublation has two sides: it sublates its self-subsistence (becoming posited in an other, i.e., matter) and sublates its reference to matter to give itself subsistence — achieving identity by passing into and uniting with matter.`
  },
  {
    id: 'agm-chunk-9-activity-of-form-as-negative-relating',
    title: 'Activity of form = negative relating to itself; matter shares movement',
    text: `The activity by which matter is determined is a negative relating of form to itself. This is also the movement that belongs to matter: matter contains implicit determination and absolute negativity, so form's activity and matter's internal movement are one and the same.`
  },
  {
    id: 'agm-chunk-10-matter-self-contradiction-and-subsistence',
    title: 'Matter's self-contradiction; subsistence through determination',
    text: `Matter is indeterminate self-identity and absolute negativity; it is self-contradictory and sublates itself within, allowing the negativity to obtain subsistence. Matter thereby attains determination by form while remaining the locus of the original unity-in-difference.`
  },
  {
    id: 'agm-chunk-11-externality-and-self-reference',
    title: 'Externality of relating and original unity; self-reference as reference-to-other',
    text: `The relating of form and matter appears as external because their original unity posits and presupposes itself; self-reference therefore manifests as reference to the self as sublated — a reference to its other — so that positing and presupposing are the same movement.`
  },
  {
    id: 'agm-chunk-12-restored-and-posited-unity',
    title: 'Restored original unity and its posited character',
    text: `Through the movement of form and matter the original unity is restored and becomes a posited unity. Matter is self-determining as much as it is determined by form; form determines itself while containing the matter it determines. The activity and movement are one, yielding the unity of in-itself and positedness: form is material, matter is necessarily formed.`
  },
  {
    id: 'agm-chunk-13-finitude-and-unity-as-truth',
    title: 'Finitude of form and matter; only their unity is truth',
    text: `Because form presupposes matter it is finite (an active factor, not a ground); matter presupposing form is finite as well. Neither finite form nor finite matter has truth alone — their unity is the truth. The determinations return to that unity and sublate their self-subsistence; the unity proves to be their ground.`
  },
  {
    id: 'agm-chunk-14-self-mediation-of-essence-as-ground',
    title: 'Self-mediation: matter determined by form through ground-unity',
    text: `The act by which matter is determined by form is the self-mediation of essence as ground: through itself and through the negation of itself. Matter becomes determined only insofar as it is the absolute unity of essence and form, and form grounds the subsistence of its determinations only insofar as it is that unity.`
  },
  {
    id: 'agm-chunk-15-informed-matter-and-negative-unity',
    title: 'Informed matter; the absolute ground as negative, posited unity',
    text: `Informed matter (form possessing subsistence) is the absolute unity of ground with itself, now also posited. The restored unity, having exhibited self-sublation, withdraws and repels itself and thus is negative unity — a determinate substrate: formed matter that nonetheless remains indifferent because sublated and unessential. This determinate substrate is the content.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'agm-op-1-matter-identity',
    chunkId: 'agm-chunk-1-matter-as-identity',
    label: 'Matter = simple identity; substrate for form',
    clauses: [
      'matter = essence.asSimpleIdentity',
      'matter.isOtherOf(form)',
      'matter.servesAs(substrateFor(form.determinations))'
    ],
    predicates: [
      { name: 'IsSimpleIdentity', args: ['matter'] },
      { name: 'IsOtherOf', args: ['matter','form'] },
      { name: 'IsSubstrateFor', args: ['matter','formDeterminations'] }
    ],
    relations: [
      { predicate: 'isOtherOf', from: 'matter', to: 'form' },
      { predicate: 'substrateFor', from: 'matter', to: 'form.determinations' }
    ]
  },
  {
    id: 'agm-op-2-abstraction-matter-abstract',
    chunkId: 'agm-chunk-2-abstraction-and-absolute-abstract',
    label: 'Abstraction from form → matter (absolutely abstract)',
    clauses: [
      'abstract(allFormDeterminations) => remainder == matter',
      'perceivedObjects = matter + form',
      'form.reductionToSimpleIdentity => matter.derivedBy(form)'
    ],
    predicates: [
      { name: 'AbstractsForm', args: ['process'] },
      { name: 'PerceivedIs', args: ['perceived','matter+form'] }
    ],
    relations: [
      { predicate: 'derives', from: 'matter', to: 'form.reduction' },
      { predicate: 'composedOf', from: 'perceivedObject', to: 'matter+form' }
    ]
  },
  {
    id: 'agm-op-3-mutual-presupposition',
    chunkId: 'agm-chunk-3-mutual-presupposition',
    label: 'Form ↔ Matter mutual presupposition; no external confrontation',
    clauses: [
      'form.presupposes(matter)',
      'matter.presupposes(form) (implicitly)',
      'neither.form.nor.matter = ultimateGroundOf(theOther)'
    ],
    predicates: [
      { name: 'Presupposes', args: ['form','matter'] },
      { name: 'MutualPresupposition', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'form', to: 'matter' },
      { predicate: 'implicitlyPresupposes', from: 'matter', to: 'form' }
    ]
  },
  {
    id: 'agm-op-4-passive-active-reciprocity',
    chunkId: 'agm-chunk-4-matter-as-passive-form-as-active',
    label: 'Matter passive, Form active; matter contains implicit form',
    clauses: [
      'matter.isPassiveAndIndifferent',
      'form.isActiveSelfReferringNegative',
      'matter.contains(sublatedNegativity) => implicitFormInside(matter)',
      'form.mustInform(matter) => form.materializes()'
    ],
    predicates: [
      { name: 'IsPassive', args: ['matter'] },
      { name: 'IsActiveNegative', args: ['form'] },
      { name: 'ContainsImplicitForm', args: ['matter'] }
    ],
    relations: [
      { predicate: 'contains', from: 'matter', to: 'implicitForm' },
      { predicate: 'informs', from: 'form', to: 'matter' }
    ]
  },
  {
    id: 'agm-op-5-indifference-and-reciprocal-reference',
    chunkId: 'agm-chunk-5-form-and-matter-not-ground-each-other',
    label: 'Indifference common determination → reciprocal reference',
    clauses: [
      'form.matterRelation = indifference',
      'indifference = determinationOf(matter)',
      'indifference => reciprocalReference(form,matter)'
    ],
    predicates: [
      { name: 'IsIndifference', args: ['relation'] },
      { name: 'ReciprocalReference', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'determines', from: 'indifference', to: 'matter' },
      { predicate: 'references', from: 'form', to: 'matter' }
    ]
  },
  {
    id: 'agm-op-6-informing-materializing',
    chunkId: 'agm-chunk-6-informing-and-materializing',
    label: 'Form must inform matter; form gains subsistence in matter',
    clauses: [
      'form.positsItselfAs(sublated) => presupposes(matter)',
      'matter.isAbsoluteReceptivityFor(form)',
      'form.materializes() => form.givesSelfSubsistenceIn(matter)'
    ],
    predicates: [
      { name: 'PresupposesMatter', args: ['form'] },
      { name: 'IsReceptivity', args: ['matter'] },
      { name: 'Materializes', args: ['form'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'form', to: 'matter' },
      { predicate: 'materializesIn', from: 'form', to: 'matter' }
    ]
  },
  {
    id: 'agm-op-7-mutual-self-mediation',
    chunkId: 'agm-chunk-7-mutual-self-mediation',
    label: 'Form ↔ Matter: mutual self-mediation restores original identity',
    clauses: [
      'form.containsImplicit(matter) && matter.containsImplicit(form)',
      'form.actOn(matter) && matter.receive(form) => sublationOfIndifference',
      'sublation => selfMediationEachThroughNonBeing && restorationOfIdentity'
    ],
    predicates: [
      { name: 'MutualContainment', args: ['form','matter'] },
      { name: 'SublationOfIndifference', args: ['relation'] },
      { name: 'RestoresIdentity', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'form', to: 'matter' },
      { predicate: 'restores', from: 'selfMediation', to: 'identity' }
    ]
  },
  {
    id: 'agm-op-8-form-self-sublation-two-sides',
    chunkId: 'agm-chunk-8-form-self-sublation-two-sides',
    label: 'Form self-sublates (two sides) → becomes posited and gives itself subsistence',
    clauses: [
      'form.isSelfSubsisting && form.isSelfSublating',
      'sideA: form.sublatesSelfSubsistence -> positednessIn(other=matter)',
      'sideB: form.sublatesReferenceTo(matter) -> givesSelfSubsistence',
      'union(form,matter) => formIsBothPositedAndSelfIdentical'
    ],
    predicates: [
      { name: 'IsSelfSubsisting', args: ['form'] },
      { name: 'SublatesToPositedness', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'sublatesInto', from: 'form', to: 'positedness' },
      { predicate: 'unitesWith', from: 'form', to: 'matter' }
    ]
  },
  {
    id: 'agm-op-9-activity-negative-relating',
    chunkId: 'agm-chunk-9-activity-of-form-as-negative-relating',
    label: 'Activity of form = negative self-relating; movement shared by matter',
    clauses: [
      'activityOfForm = negativeRelating(form, itself)',
      'matter.contains(implicitDetermination) && thusSharesMovement',
      'formActivity == matterMovement => singleContradictoryMovement'
    ],
    predicates: [
      { name: 'NegativeRelating', args: ['form'] },
      { name: 'SharesMovement', args: ['matter','form'] }
    ],
    relations: [
      { predicate: 'relatesNegatively', from: 'form', to: 'itself' },
      { predicate: 'sharesMovementWith', from: 'form', to: 'matter' }
    ]
  },
  {
    id: 'agm-op-10-matter-self-contradiction-subsistence',
    chunkId: 'agm-chunk-10-matter-self-contradiction-and-subsistence',
    label: 'Matter = indeterminate identity + absolute negativity → subsistence via determination',
    clauses: [
      'matter = indeterminateSelfIdentity && matter.hasAbsoluteNegativity',
      'matter.sublatesItselfWithin -> negativityObtainsSubsistence',
      'determinationBy(form) => matter.attainsDeterminationWhilePreservingUnity'
    ],
    predicates: [
      { name: 'IsIndeterminateIdentity', args: ['matter'] },
      { name: 'HasAbsoluteNegativity', args: ['matter'] }
    ],
    relations: [
      { predicate: 'sublatesWithin', from: 'matter', to: 'negativity' },
      { predicate: 'attainsDeterminationBy', from: 'matter', to: 'form' }
    ]
  },
  {
    id: 'agm-op-11-externality-self-reference',
    chunkId: 'agm-chunk-11-externality-and-self-reference',
    label: 'External relating = self-reference to sublated self; positing = presupposing',
    clauses: [
      'originalUnity.presupposesDifferentia -> relatingAppearsExternal',
      'selfReference -> referenceTo(selfAsSublated) == referenceToOther',
      'positing == presupposing (same movement)'
    ],
    predicates: [
      { name: 'AppearsExternal', args: ['relating'] },
      { name: 'SelfReferenceToSublated', args: ['self'] },
      { name: 'PositingEqualsPresupposing', args: ['movement'] }
    ],
    relations: [
      { predicate: 'appearsAs', from: 'relating', to: 'externality' },
      { predicate: 'isReferenceTo', from: 'selfReference', to: 'sublatedSelf' }
    ]
  },
  {
    id: 'agm-op-12-restored-posited-unity',
    chunkId: 'agm-chunk-12-restored-and-posited-unity',
    label: 'Movement of form & matter → restored unity & posited unity; matter/form interchange',
    clauses: [
      'movement(form, matter) => restore(originalUnity)',
      'restoredUnity.isPosited = true',
      'form.contains(matter) && matter.selfDetermines => unityOf(inItself, positedness)'
    ],
    predicates: [
      { name: 'RestoresUnity', args: ['movement'] },
      { name: 'IsPositedUnity', args: ['unity'] }
    ],
    relations: [
      { predicate: 'restores', from: 'movement', to: 'originalUnity' },
      { predicate: 'yields', from: 'restoredUnity', to: 'positedness' }
    ]
  },
  {
    id: 'agm-op-13-finitude-unity-truth',
    chunkId: 'agm-chunk-13-finitude-and-unity-as-truth',
    label: 'Form & matter finite; only their unity is truth; unity subsumes determinations',
    clauses: [
      'form.presupposes(matter) => form.isFinite',
      'matter.presupposes(form) => matter.isFinite',
      'truth := unity(form,matter)',
      'determinations.returnTo(unity) => sublationOfSelfSubsistence'
    ],
    predicates: [
      { name: 'IsFinite', args: ['formOrMatter'] },
      { name: 'UnityIsTruth', args: ['unity'] }
    ],
    relations: [
      { predicate: 'isFinite', from: 'form', to: 'true' },
      { predicate: 'isFinite', from: 'matter', to: 'true' },
      { predicate: 'subsume', from: 'unity', to: 'determinations' }
    ]
  },
  {
    id: 'agm-op-14-self-mediation-ground-unity',
    chunkId: 'agm-chunk-14-self-mediation-of-essence-as-ground',
    label: 'Determination of matter by form = self-mediation of essence as ground',
    clauses: [
      'act(determineMatterByForm) := selfMediation(essenceAsGround)',
      'thisMediation => determinationOnlyIf(essence.isAbsoluteUnity(form,matter))',
      'form.groundsSubsistenceOnlyIf(form.isThatUnity)'
    ],
    predicates: [
      { name: 'IsSelfMediation', args: ['act'] },
      { name: 'RequiresUnity', args: ['determination'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'essence', to: 'formMatterRelation' },
      { predicate: 'requires', from: 'determination', to: 'absoluteUnity' }
    ]
  },
  {
    id: 'agm-op-15-informed-matter-negative-unity',
    chunkId: 'agm-chunk-15-informed-matter-and-negative-unity',
    label: 'Informed matter/form = absolute ground posited; negative unity = formed matter as content',
    clauses: [
      'informedMatter := matterWith(form.subsistence)',
      'absoluteGround.exhibits(selfSublation) => withdrawnNegativeUnity',
      'resultingContent := formedMatter.indifferentBecauseSublated'
    ],
    predicates: [
      { name: 'InformedMatter', args: ['matter'] },
      { name: 'NegativeUnity', args: ['ground'] },
      { name: 'FormedMatterContent', args: ['content'] }
    ],
    relations: [
      { predicate: 'informs', from: 'form', to: 'matter' },
      { predicate: 'constitutes', from: 'negativeUnity', to: 'content' }
    ]
  }
)

// ============================================================================
// PART C: FORM AND CONTENT
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'agc-chunk-1-form-stance-over-essence',
    title: 'Form stands over against essence — ground-connection in general',
    text: `Form initially stands over against essence: it is the ground-connection in general and its determinations articulate ground and grounded. As such form also stands over against matter and finally against content, taking each formerly self-identical term under its dominion as one of its determinations.`
  },
  {
    id: 'agc-chunk-2-content-as-unity-of-form-and-matter',
    title: 'Content = unity of form and matter; content stands over against form',
    text: `Content is the unity of a form and a matter that belong to it essentially. Because this unity is determinate (posited), content stands over against form, making form appear unessential; content therefore contains both a form and matter as its substrate while regarding them as mere positedness.`
  },
  {
    id: 'agc-chunk-3-content-identity-and-ground-connection',
    title: 'Content identical in form & matter; ground-connection returns in content',
    text: `Content is what is identical in form and matter: their returned unity. This identity is both indifferent to form and, simultaneously, the identity of ground. The ground disappears into content while content is the negative reflection of form-determinations into themselves so that content bears the ground-connection as its essential form.`
  },
  {
    id: 'agc-chunk-4-content-of-ground-and-informed-identity',
    title: 'Content of ground = informed identity; content informs matter',
    text: `The content of the ground is the ground returned into unity with itself: informed identity. As such the formerly indeterminate matter becomes informed; the determinations of form acquire a material, indifferent subsistence within content. Content is thus both the self-identity of ground and a posited identity against the ground-connection.`
  },
  {
    id: 'agc-chunk-5-positedness-forms-and-total-positedness',
    title: 'Positedness and twofold form: total positedness vs immediate positedness',
    text: `Within content positedness appears as a determination of form that stands over against the form-as-total-connection. One form is the total positedness returning into itself; the other is mere immediate positedness or determinateness as such. Content thereby distinguishes forms while remaining their substrate.`
  },
  {
    id: 'agc-chunk-6-twofold-determinateness-and-conclusion',
    title: 'Twofold determinateness: form external to content and content's determinateness',
    text: `The ground has become a determinate ground in general and this determinateness is twofold: (1) form's determinateness as external to content (content remains indifferent), and (2) the determinateness of the content itself that the ground possesses — closing the synthesis of form, matter and content.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'agc-op-1-form-stance',
    chunkId: 'agc-chunk-1-form-stance-over-essence',
    label: 'Form as ground-connection; determinations = ground + grounded',
    clauses: [
      'form.standsOver(essence)',
      'form.isGroundConnection()',
      'form.determinations = {ground, grounded}'
    ],
    predicates: [
      { name: 'StandsOver', args: ['form', 'essence'] },
      { name: 'IsGroundConnection', args: ['form'] }
    ],
    relations: [
      { predicate: 'determines', from: 'form', to: 'ground+grounded' }
    ]
  },
  {
    id: 'agc-op-2-content-unity',
    chunkId: 'agc-chunk-2-content-as-unity-of-form-and-matter',
    label: 'Content = unity(form, matter) ; content stands over against form',
    clauses: [
      'content = unity(form, matter)',
      'content.isDeterminate => content.standsOver(form)',
      'form = unessentialAgainst(content)'
    ],
    predicates: [
      { name: 'IsUnityOf', args: ['content', 'form+matter'] },
      { name: 'StandsOver', args: ['content', 'form'] }
    ],
    relations: [
      { predicate: 'composedOf', from: 'content', to: 'form+matter' },
      { predicate: 'contrasts', from: 'content', to: 'form' }
    ]
  },
  {
    id: 'agc-op-3-content-identity-ground-connection',
    chunkId: 'agc-chunk-3-content-identity-and-ground-connection',
    label: 'Content identity = indifferent to form and identity of ground; ground-connection in content',
    clauses: [
      'content.isIdentityOf(form, matter)',
      'content.indifferentTo(form) && content.isGroundIdentity',
      'ground.disappearsInto(content) && content.reflects(form.determinations)'
    ],
    predicates: [
      { name: 'IsIdentityOf', args: ['content', 'form+matter'] },
      { name: 'IsGroundIdentity', args: ['content'] }
    ],
    relations: [
      { predicate: 'reflects', from: 'content', to: 'form.determinations' },
      { predicate: 'absorbs', from: 'content', to: 'ground' }
    ]
  },
  {
    id: 'agc-op-4-informed-content',
    chunkId: 'agc-chunk-4-content-of-ground-and-informed-identity',
    label: 'Content as informed matter; form determinations obtain material subsistence',
    clauses: [
      'content.informs(matter)',
      'form.determinations.obtainSubsistenceWithin(content)',
      'content = positedIdentityAgainst(groundConnection)'
    ],
    predicates: [
      { name: 'Informs', args: ['content', 'matter'] },
      { name: 'HasMaterialSubsistence', args: ['content', 'form.determinations'] }
    ],
    relations: [
      { predicate: 'informs', from: 'content', to: 'matter' },
      { predicate: 'sustains', from: 'content', to: 'form.determinations' }
    ]
  },
  {
    id: 'agc-op-5-positedness-forms',
    chunkId: 'agc-chunk-5-positedness-forms-and-total-positedness',
    label: 'Total positedness vs immediate positedness; two forms in content',
    clauses: [
      'form_total = totalPositednessReturningIntoItself',
      'form_immediate = immediatePositedness (determinateness)',
      'content.distinguishes(form_total, form_immediate)'
    ],
    predicates: [
      { name: 'IsTotalPositedness', args: ['form_total'] },
      { name: 'IsImmediatePositedness', args: ['form_immediate'] }
    ],
    relations: [
      { predicate: 'distinguishes', from: 'content', to: 'form_total+form_immediate' }
    ]
  },
  {
    id: 'agc-op-6-twofold-determinateness',
    chunkId: 'agc-chunk-6-twofold-determinateness-and-conclusion',
    label: 'Twofold determinateness: externality of form / determinateness of content',
    clauses: [
      'ground.becomesDeterminateGround()',
      'determinateness = {formExternalToContent, contentDeterminateness}',
      'synthesis.closes(form, matter, content)'
    ],
    predicates: [
      { name: 'IsDeterminateGround', args: ['ground'] },
      { name: 'HasTwofoldDeterminateness', args: ['determinateness'] }
    ],
    relations: [
      { predicate: 'yields', from: 'ground', to: 'determinateness' },
      { predicate: 'synthesizes', from: 'form+matter', to: 'content' }
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

