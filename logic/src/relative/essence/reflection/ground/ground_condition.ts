import type { Chunk, LogicalOperation } from './index'

/*
  Essence — C. CONDITION

  This module consolidates the complete Condition section:
  - Part a: The Relatively Unconditioned
  - Part b: The Absolutely Unconditioned
  - Part c: Procession of the Fact into Concrete Existence

  PHILOSOPHICAL NOTES:

  1. **Dependent Origination as Condition**:
     Condition is Dependent Origination made explicit — the immediate
     presupposition that grounds all conditioned arising. The movement
     from Relatively Unconditioned → Absolutely Unconditioned → Concrete
     Existence is the systematic structure of Dependent Origination:
     conditions arise, become unconditioned, and proceed into concrete
     existence — the complete cycle of conditioned arising.

  2. **Theory of Forms as Condition**:
     Condition is the Theory of Forms made explicit — the immediate
     presupposition that grounds all formal determination. The fact
     (the unconditioned) conditions itself, places itself as ground
     over its conditions, and proceeds into concrete existence —
     the complete cycle of formal determination within Dependent Origination.

  3. **The Big Kahuna Completed**:
     Condition completes "the Big Kahuna" — showing how Dependent Origination
     and Theory of Forms culminate in Concrete Existence. The fact conditions
     itself, grounds itself, and proceeds into existence — the systematic
     structure that unifies Dependent Origination and Theory of Forms in
     the actualization of the unconditioned fact.

  4. **Ground as LogoGenesis — Condition as Completion**:
     Condition is the completion of Ground as LogoGenesis. The Absolutely
     Unconditioned (the fact-in-itself) is Ground as Science — the synthesis
     of Cit (Pure Consciousness) and Citi (Power of Consciousness) in Citta
     (Mind as Science), structured by Logic (Upadhi). The Procession into
     Concrete Existence is where Ground (Science) realizes itself as Truth
     in the Syllogism of Necessity — the LogoGenesis completed and actualized.
*/

// ============================================================================
// PART A: THE RELATIVELY UNCONDITIONED
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'chunk-1-ground-self-positing',
    title: 'Ground — self-positing & mediation',
    text: `Ground is the immediate,
and the grounded the mediated.
But ground is positing reflection;
as such, it makes itself into positedness
and is presupposing reflection;
as such it refers itself to itself
as to something sublated,
to an immediate through which
it is itself mediated.
This mediation, as an advance
from the immediate to the ground,
is not an external reflection
but, as we have seen, the ground's own doing
or, what is the same, the ground-connection,
as reflection into its self-identity,
is just as essentially self-externalizing reflection.
The immediate to which ground refers as
to its essential presupposition is condition;
real ground is accordingly essentially conditioned.
The determinateness that it contains is
the otherness of itself.`
  },
  {
    id: 'chunk-2-condition-conclusion',
    title: 'Conclusion — what condition is',
    text: `Condition is therefore,
first, an immediate, manifold existence.
Second, it is this existence referred to an other,
to something which is ground,
not of this existence but in some other respect,
for existence itself is immediate and without ground.
According to this reference, it is something posited;
as condition, the immediate existence is supposed to be
not for itself but for another.
But this, that it thus is for another, is at the same time
itself only a positedness;
that it is posited is sublated in its immediacy:
an existence is indifferent to being a condition.
Third, condition is something immediate in the sense
that it constitutes the presupposition of ground.`
  },
  {
    id: 'chunk-3-condition-content-form',
    title: 'Expansion — condition as content and form-connection',
    text: `In this determination, it is the form-connection of ground
withdrawn into self-identity, hence the content of ground.
But content is as such only the indifferent unity of ground,
as in the form: without form, no content.
It nevertheless frees itself
from this indifferent unity
in that the ground-connection,
in the complete ground,
becomes a connection external to its identity,
whereby content acquires immediacy.

In so far, therefore, as condition is
that in which the ground-connection has
its identity with itself,
it constitutes the content of ground;
but since this content is indifferent to form,
it is only implicitly the content of form,
is something which has yet to become content
and hence constitutes the material for the ground.`
  },
  {
    id: 'chunk-4-condition-becomes-unconditioned',
    title: 'Return — condition becomes relative unconditioned for ground',
    text: `Posited as condition,
and in accordance with the second moment,
existence is determined to lose its indifferent immediacy
and to become the moment of another.
By virtue of its immediacy, it is indifferent to this connection;
inasmuch as it enters into it, however,
it constitutes the in-itself of the ground
and is for it the unconditioned.
In order to be condition,
it has its presupposition in the ground
and is itself conditioned;
but this condition is external to it.`
  },
  {
    id: 'chunk-5-condition-essence-distinction',
    title: 'Essence — condition is not the ground; distinction of roles',
    text: `Something is not through its condition; its condition is not its ground.
Condition is for the ground the moment of unconditioned immediacy, but is not itself the positing-movement that refers itself to itself and makes itself into positedness.
Over against condition there stands the ground-connection: something has, besides its condition, also a ground.
The ground is the empty movement of reflection which nonetheless has immediacy as its presupposition outside it.`
  },
  {
    id: 'chunk-6-ground-connection-content',
    title: 'Ground-connection — self-subsisting reflection and content distinction',
    text: `The ground-connection is the whole form and the self-subsistent process of mediation; since this mediating refers itself to itself as positing it is also something immediate and unconditioned on that side.
As self-subsisting self-reference it has a content peculiar to itself (the informed content of ground) distinct from the immediate material content of the condition.
The condition's content is immediate material which, while external, constitutes the in-itself of the ground and should become a moment of it (material-for-form).`
  },
  {
    id: 'chunk-7-two-sides-indifference',
    title: 'The two sides — mutual indifference and unconditionedness',
    text: `The two sides of the whole, condition and ground, are thus, on the one hand, indifferent and unconditioned with respect to each other:
the one as the non-referred-to side, to which the connecting reference in which it is the condition is external;
the other as the connecting reference, or form, for which the determinate existence of the condition is only a material, something passive whose form, such as it possesses on its own account, is unessential.`
  },
  {
    id: 'chunk-8-mediation-contradiction',
    title: 'Mediation — condition as in-itself of ground and the internal contradiction',
    text: `On the other hand, the two sides are also mediated.
Condition is the in-itself of the ground; so much is it the essential moment of the ground-connection, that it is the simple self-identity of the ground.
But this is sublated; this in-itself is only something posited; immediate existence is indifferent to being a condition.
Likewise, the ground-connection has in its self-subsistence also a presupposition; it has its in-itself outside itself.
Consequently, each side is this contradiction: indifferent immediacy and essential mediation, both in one reference — independent subsistence and determination as only moments.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'op-1-ground-self-positing',
    chunkId: 'chunk-1-ground-self-positing',
    label: 'Ground self-positing / mediation',
    clauses: [
      'ground.immediate = true',
      'ground.posits(self) => ground.positedness',
      'mediation := ground.connection (self-activity), not external reflection',
      'condition = immediate presupposition referenced by ground',
      'ground.determinateness = othernessOf(self)'
    ],
    predicates: [
      { name: 'Immediate', args: ['ground'] },
      { name: 'Posits', args: ['ground'] },
      { name: 'MediationIsSelfDoing', args: ['ground'] }
    ],
    relations: [{ predicate: 'presupposes', from: 'ground', to: 'condition' }]
  },
  {
    id: 'op-2-condition-conclusion',
    chunkId: 'chunk-2-condition-conclusion',
    label: 'Condition — immediacy, for-another, positedness, presupposition',
    clauses: [
      'condition.isImmediate = true',
      'condition.referredTo = ground (for-another)',
      'condition.isPosited = true',
      'positedness.sublatedBy(immediacy) => existence.indifferentToBeingCondition',
      'condition.constitutesPresuppositionOf(ground)'
    ],
    predicates: [
      { name: 'Immediate', args: ['condition'] },
      { name: 'ForAnother', args: ['condition', 'ground'] },
      { name: 'Posited', args: ['condition'] }
    ],
    relations: [
      { predicate: 'for', from: 'condition', to: 'ground' },
      { predicate: 'constitutes', from: 'condition', to: 'ground.inItself' }
    ]
  },
  {
    id: 'op-3-condition-content-form',
    chunkId: 'chunk-3-condition-content-form',
    label: 'Form-connection withdrawn into self-identity → content of ground',
    clauses: [
      'formConnection(ground).withdrawnInto(selfIdentity) => content(ground)',
      'content.initial = indifferentUnity(ground)',
      'without form => no content',
      'completeGround: groundConnection.becomesExternal => content.acquiresImmediacy',
      'implicitContent => materialFor(groundForm)'
    ],
    predicates: [
      { name: 'WithdrawnInto', args: ['formConnection','selfIdentity'] },
      { name: 'IndifferentUnity', args: ['content','ground'] },
      { name: 'BecomesExternal', args: ['groundConnection'] }
    ],
    relations: [
      { predicate: 'yields', from: 'formConnection', to: 'ground.content' },
      { predicate: 'materialFor', from: 'ground.content', to: 'ground.form' }
    ]
  },
  {
    id: 'op-4-condition-becomes-unconditioned',
    chunkId: 'chunk-4-condition-becomes-unconditioned',
    label: 'Condition → relative unconditioned for ground',
    clauses: [
      'condition.posited && condition.entersConnection => condition.losesIndifferentImmediacy',
      'condition.entersConnection => condition.constitutes(ground.inItself)',
      'condition.isFor(ground) && condition.isConditioned(externalToItself)',
      'through entry, condition.functionsFor(ground) as relativeUnconditioned'
    ],
    predicates: [
      { name: 'LosesImmediacy', args: ['condition'] },
      { name: 'ConstitutesInItself', args: ['condition','ground'] },
      { name: 'IsFor', args: ['condition','ground'] }
    ],
    relations: [
      { predicate: 'constitutes', from: 'condition', to: 'ground.inItself' },
      { predicate: 'functionsAs', from: 'condition', to: 'ground.relativeUnconditioned' }
    ]
  },
  {
    id: 'op-5-condition-essence-distinction',
    chunkId: 'chunk-5-condition-essence-distinction',
    label: 'Distinction: condition ≠ ground; condition as moment of immediacy',
    clauses: [
      'forAny(x): x.notIdenticalTo(x.condition)',
      'condition.roleFor(ground) = momentOf(unconditionedImmediacy)',
      'condition.isNot(positingMovement)',
      'groundConnection.existsOppositeTo(condition)'
    ],
    predicates: [
      { name: 'NotThroughCondition', args: ['something','condition'] },
      { name: 'MomentOfImmediacy', args: ['condition','ground'] },
      { name: 'IsNotPositing', args: ['condition'] }
    ],
    relations: [
      { predicate: 'opposes', from: 'ground.connection', to: 'condition' },
      { predicate: 'hasMoment', from: 'ground', to: 'condition' }
    ]
  },
  {
    id: 'op-6-ground-connection-content',
    chunkId: 'chunk-6-ground-connection-content',
    label: 'Ground-connection as self-subsisting reflection; content vs material distinction',
    clauses: [
      'groundConnection.isSelfSubsistent = true',
      'groundConnection.hasIdentityOfReflection = true',
      'ground.content = informedContent (distinct)',
      'condition.content = immediateMaterial (external-to-ground)',
      'condition.entersConnection => condition.materialBecomesMomentOf(ground)'
    ],
    predicates: [
      { name: 'SelfSubsistent', args: ['groundConnection'] },
      { name: 'HasReflectionIdentity', args: ['groundConnection'] },
      { name: 'InformedContent', args: ['ground'] },
      { name: 'ImmediateMaterial', args: ['condition'] }
    ],
    relations: [
      { predicate: 'hasContent', from: 'groundConnection', to: 'ground.content' },
      { predicate: 'materialFor', from: 'condition.content', to: 'ground' },
      { predicate: 'constitutes', from: 'condition', to: 'ground.inItself' }
    ]
  },
  {
    id: 'op-7-two-sides-indifference',
    chunkId: 'chunk-7-two-sides-indifference',
    label: 'Two sides: mutual indifference; externality of reference',
    clauses: [
      'condition.indifferentTo(ground) && ground.indifferentTo(condition)',
      'condition.isNonReferredSide = true',
      'ground.isConnectingReference = true',
      'condition.content = immediateMaterial (externalToReference)'
    ],
    predicates: [
      { name: 'Indifferent', args: ['condition','ground'] },
      { name: 'NonReferredSide', args: ['condition'] },
      { name: 'ConnectingReference', args: ['ground'] },
      { name: 'ImmediateMaterial', args: ['condition'] }
    ],
    relations: [
      { predicate: 'externalTo', from: 'condition.content', to: 'ground.connection' },
      { predicate: 'isFormFor', from: 'ground', to: 'condition' }
    ]
  },
  {
    id: 'op-8-mediation-contradiction',
    chunkId: 'chunk-8-mediation-contradiction',
    label: 'Mediation & contradiction: in-itself posited; mutual presupposition',
    clauses: [
      'condition.isInItselfOf(ground) = true',
      'condition.inItselfIsPosited = true',
      'groundConnection.hasPresuppositionOutsideItself = true',
      'eachSide = (indifferentImmediacy && essentialMediation)'
    ],
    predicates: [
      { name: 'InItselfOf', args: ['condition','ground'] },
      { name: 'Posited', args: ['condition'] },
      { name: 'HasPresuppositionOutside', args: ['groundConnection'] },
      { name: 'ContradictoryDouble', args: ['side'] }
    ],
    relations: [
      { predicate: 'constitutesInItself', from: 'condition', to: 'ground.inItself' },
      { predicate: 'presupposesOutside', from: 'groundConnection', to: 'ground.inItself' }
    ]
  }
]

// ============================================================================
// PART B: THE ABSOLUTELY UNCONDITIONED
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'a-chunk-1-reflective-shine',
    title: 'Reflective shine & independence',
    text: `At first, each of the two relatively unconditioned sides reflectively shines in the other; condition, as an immediate, is reflected in the form-connection of the ground, and this form in the immediate existence as its positedness; but each, apart from this reflective shine of its other in it, stands out on its own and has a content of its own.`
  },
  {
    id: 'a-chunk-2-condition-form-moments',
    title: 'Condition as immediate existence; two form-moments',
    text: `Condition is at first immediate existence; its form has these two moments: that of positedness, according to which it is, as condition, material and moment of the ground; and that of the in-itself, according to which it constitutes the essentiality of ground or its simple reflection into itself. Both sides of the form are external to immediate existence, for the latter is the sublated ground-connection.`
  },
  {
    id: 'a-chunk-3-existence-sublation',
    title: 'Existence sublates itself — being becomes essence',
    text: `But, first, existence is in it only this: to sublate itself in its immediacy and to founder, going to the ground. Being is as such only the becoming of essence; it is its essential nature to make itself into a positedness and into an identity which is an immediacy through the negation of itself. The form determinations of positedness and of self-identical in-itself, the form through which immediate existence is condition, are not, therefore, external to that existence; the latter is, rather, this very reflection.`
  },
  {
    id: 'a-chunk-4-being-as-condition-posited',
    title: 'Being as condition — positedness and in‑itself mediated',
    text: `Second: as condition, being is posited as what it essentially is — a moment and the being of another, and at the same time the in-itself of the other. It is in-itself only through its negation (the ground) and by self-sublation; the in-itself of being is thus itself posited and contains both essentiality-for-ground and immediate existence.`
  },
  {
    id: 'a-chunk-5-ground-self-subsistence',
    title: 'Ground as self-subsistent reflection and presupposing relation',
    text: `Likewise the ground is the self-subsistent, self-referring reflection of positing and thus self-identical; yet it is also presupposing reflection that posits its in-itself as an other. Condition, in both its in-itself and immediacy, is the ground-connection's own moment; the immediate existence is only through the ground and is a moment of the presupposing.`
  },
  {
    id: 'a-chunk-6-unity-of-form-and-content',
    title: 'Unity — one whole of form and content; existence as informed matter',
    text: `The result is one whole of form and one whole of content: the condition's content is essential only insofar as it is the self-identity of reflection in the form; existence is not mere formless material but informed matter that is identical in form-connection with the ground's content.`
  },
  {
    id: 'a-chunk-7-unity-and-unconditioned-fact',
    title: 'Unity of condition & ground — the unconditioned fact',
    text: `The two sides of the whole, condition and ground, are one essential unity as content and form. They pass into one another, posit themselves as sublated, and reciprocally presuppose each other. Their mutual presupposing amounts to presupposing one identity — the single substrate that is the truly unconditioned: the fact-in-itself.`
  },
  {
    id: 'a-chunk-8-relativity-and-sublation-of-condition',
    title: 'Relativity of condition; finite determinate and sublation',
    text: `Condition remains only relatively unconditioned: because any given condition is a finite determinate existence, one asks for another condition and so sets the regress in motion. This finitude is external to the concept of condition and is therefore sublated in the absolutely unconditioned (the fact in itself).`
  },
  {
    id: 'a-chunk-9-absolutely-unconditioned',
    title: 'Absolutely unconditioned — fact contains its moments',
    text: `The absolutely unconditioned (the fact-in-itself) contains within itself the two sides, condition and ground, as its moments; it is the unity to which they have returned. Together they constitute its form or positedness. The unconditioned fact is itself ground: it both conditions and grounds the two moments, and so its positing is the return and unity of what was divided.`
  },
  {
    id: 'a-chunk-10-two-shapes-and-rejoining',
    title: 'Two shapes (sublated manifold & inner ground) — reflective rejoining',
    text: `These two moments appear in two shapes: (1) as the sublated ground-connection — an immediate manifold void of unity that refers outward to the ground and constitutes its in-itself; (2) as an inner, simple form which is itself ground and which refers to the self-identical immediate as other, determining it as condition. They presuppose the totality that posits them, yet because they are identical, the relation of condition and ground dissolves into mere reflective shine. The absolutely unconditioned thus posits and presupposes in a movement where that shine sublates itself: the fact conditions itself, places itself as ground over its conditions, and in so doing rejoins them in itself.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'a-op-1-reflective-shine',
    chunkId: 'a-chunk-1-reflective-shine',
    label: 'Mutual reflective shine; independent content',
    clauses: [
      'condition.reflectsIn(ground) && ground.formReflectsIn(condition)',
      'condition.hasIndependentContent = true',
      'ground.hasIndependentContent = true'
    ],
    predicates: [
      { name: 'ReflectsIn', args: ['condition','ground'] },
      { name: 'HasIndependentContent', args: ['entity'] }
    ],
    relations: [
      { predicate: 'reflectsIn', from: 'condition', to: 'ground' },
      { predicate: 'reflectsIn', from: 'ground.form', to: 'condition' }
    ]
  },
  {
    id: 'a-op-2-condition-form-moments',
    chunkId: 'a-chunk-2-condition-form-moments',
    label: 'Condition form-moments: positedness & in-itself',
    clauses: [
      'condition.isImmediate = true',
      'condition.form.moments = [positedness, inItself]',
      'positedness => condition.materialFor(ground)',
      'inItSelf => constitutesEssentialityOf(ground)',
      'formMoments.externalTo(condition.immediacy) = true'
    ],
    predicates: [
      { name: 'IsImmediate', args: ['condition'] },
      { name: 'Positedness', args: ['condition'] },
      { name: 'InItself', args: ['condition'] }
    ],
    relations: [
      { predicate: 'materialFor', from: 'condition', to: 'ground' },
      { predicate: 'constitutes', from: 'condition.inItSelf', to: 'ground.essentiality' }
    ]
  },
  {
    id: 'a-op-3-existence-sublation',
    chunkId: 'a-chunk-3-existence-sublation',
    label: 'Existence sublates itself; being → becoming of essence',
    clauses: [
      'existence.sublatesSelfInImmediacy => goesTo(ground)',
      'being = becomingOf(essence)',
      'existence.makesItself.positedness = true',
      'identity.immediacy.throughNegation = true',
      'formDeterminations.areInternalTo(existence) = true'
    ],
    predicates: [
      { name: 'SublatesSelfInImmediacy', args: ['existence'] },
      { name: 'BecomingOfEssence', args: ['being'] },
      { name: 'MakesPositedness', args: ['existence'] }
    ],
    relations: [
      { predicate: 'sublatesTo', from: 'existence', to: 'ground' },
      { predicate: 'becomes', from: 'being', to: 'essence' }
    ]
  },
  {
    id: 'a-op-4-being-as-condition-posited',
    chunkId: 'a-chunk-4-being-as-condition-posited',
    label: 'Being as condition: posited in negation; mediated in-itself',
    clauses: [
      'forAny(x): if x.isCondition then x.isMomentOf(another)',
      'x.inItself = true onlyIf x.negatesSelfVia(ground)',
      'inItSelf(posited) && immediacy(coexist) -> bothSidesPresent'
    ],
    predicates: [
      { name: 'IsCondition', args: ['x'] },
      { name: 'MomentOf', args: ['x','other'] },
      { name: 'NegatesSelfVia', args: ['x','ground'] }
    ],
    relations: [
      { predicate: 'isMomentOf', from: 'condition', to: 'ground' },
      { predicate: 'inItSelfNegatedBy', from: 'condition', to: 'ground' }
    ]
  },
  {
    id: 'a-op-5-ground-self-subsistence',
    chunkId: 'a-chunk-5-ground-self-subsistence',
    label: 'Ground self-subsistent reflection; presupposing relation',
    clauses: [
      'ground.isSelfSubsistent = true',
      'ground.isSelfReferring = true',
      'ground.presupposesOutsideItself = true',
      'condition.isMomentOf(ground) && immediate.existsOnlyThrough(ground)'
    ],
    predicates: [
      { name: 'SelfSubsistent', args: ['ground'] },
      { name: 'SelfReferring', args: ['ground'] },
      { name: 'PresupposesOutside', args: ['ground'] }
    ],
    relations: [
      { predicate: 'selfSubsists', from: 'ground', to: 'ground' },
      { predicate: 'presupposes', from: 'ground', to: 'condition' }
    ]
  },
  {
    id: 'a-op-6-unity-of-form-and-content',
    chunkId: 'a-chunk-6-unity-of-form-and-content',
    label: 'Unity: one whole of form and content; existence as informed matter',
    clauses: [
      'whole.form = true && whole.content = true',
      'condition.content.isEssential iff condition.isSelfIdentityInForm',
      'existence.isInformedMatter = true (not mere formless material)',
      'condition.content == ground.content (identity-in-form)'
    ],
    predicates: [
      { name: 'WholeForm', args: ['whole'] },
      { name: 'WholeContent', args: ['whole'] },
      { name: 'IsInformedMatter', args: ['existence'] }
    ],
    relations: [
      { predicate: 'identityInForm', from: 'condition.content', to: 'ground.content' },
      { predicate: 'informs', from: 'form', to: 'content' }
    ]
  },
  {
    id: 'a-op-7-unity-unconditioned-fact',
    chunkId: 'a-chunk-7-unity-and-unconditioned-fact',
    label: 'Condition ⇄ Ground as one unity → Unconditioned substrate (Fact-in-itself)',
    clauses: [
      'condition.presupposes(ground) && ground.presupposes(condition)',
      'mutualPresupposition => singleIdentity(substrate)',
      'singleIdentity.isUnconditioned = true',
      'substrate == factInItself'
    ],
    predicates: [
      { name: 'MutualPresupposes', args: ['condition','ground'] },
      { name: 'SingleIdentity', args: ['substrate'] },
      { name: 'IsUnconditioned', args: ['substrate'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'condition', to: 'ground' },
      { predicate: 'presupposes', from: 'ground', to: 'condition' },
      { predicate: 'substrateOf', from: 'substrate', to: 'condition+ground' }
    ]
  },
  {
    id: 'a-op-8-relative-condition-sublation',
    chunkId: 'a-chunk-8-relativity-and-sublation-of-condition',
    label: 'Relatively unconditioned → regress; sublation in absolute',
    clauses: [
      'if condition.isFiniteDeterminate then askFor(newCondition)',
      'askingFor(newCondition) => regressAdInfinitum',
      'regressAdInfinitum is explainedBy condition.finitude',
      'absolutelyUnconditioned.sublates(condition) => terminationOfRegress'
    ],
    predicates: [
      { name: 'RelativelyUnconditioned', args: ['condition'] },
      { name: 'FiniteDeterminate', args: ['condition'] },
      { name: 'SublatedIn', args: ['absolutelyUnconditioned','condition'] }
    ],
    relations: [
      { predicate: 'requests', from: 'condition', to: 'furtherCondition' },
      { predicate: 'sublates', from: 'absolutelyUnconditioned', to: 'condition' }
    ]
  },
  {
    id: 'a-op-9-absolutely-unconditioned',
    chunkId: 'a-chunk-9-absolutely-unconditioned',
    label: 'Absolutely unconditioned contains moments; is itself ground',
    clauses: [
      'fact.contains(condition) && fact.contains(ground)',
      'fact.moments = [condition, ground]',
      'fact.isGround = true',
      'fact.posits(condition, ground) && fact.unifies(condition, ground)'
    ],
    predicates: [
      { name: 'ContainsAsMoment', args: ['fact','moment'] },
      { name: 'IsGround', args: ['fact'] },
      { name: 'Unifies', args: ['fact','moments'] }
    ],
    relations: [
      { predicate: 'contains', from: 'fact', to: 'condition' },
      { predicate: 'contains', from: 'fact', to: 'ground' },
      { predicate: 'grounds', from: 'fact', to: 'condition+ground' }
    ]
  },
  {
    id: 'a-op-10-two-shapes-rejoining',
    chunkId: 'a-chunk-10-two-shapes-and-rejoining',
    label: 'Two shapes (sublated manifold & inner ground) → reflective rejoining',
    clauses: [
      'shapeA = sublatedGroundConnection (immediateManifold, external, refersTo(ground))',
      'shapeB = innerSimpleForm (ground, selfIdentical, refersTo(immediate) as other)',
      'shapeA.presupposes(totality) && shapeB.presupposes(totality)',
      'if shapeA.identityWith(shapeB) then relation(condition,ground) -> reflectiveShine',
      'fact.positsAndPresupposes = movementWhere(shinethenSublatesItself)',
      'fact.selfConditions() && fact.placesItselfAs(ground) && fact.rejoins(condition,ground)'
    ],
    predicates: [
      { name: 'SublatedConnection', args: ['shapeA'] },
      { name: 'InnerSimpleGround', args: ['shapeB'] },
      { name: 'PresupposesTotality', args: ['shape'] },
      { name: 'ReflectiveShine', args: ['shape'] }
    ],
    relations: [
      { predicate: 'refersTo', from: 'shapeA', to: 'ground' },
      { predicate: 'refersTo', from: 'shapeB', to: 'immediate' },
      { predicate: 'identityWith', from: 'shapeA', to: 'shapeB' },
      { predicate: 'rejoins', from: 'fact', to: 'condition+ground' }
    ]
  }
)

// ============================================================================
// PART C: PROCESSION OF THE FACT INTO CONCRETE EXISTENCE
// ============================================================================

CANONICAL_CHUNKS.push(
  {
    id: 'c3-chunk-1-absolute-ground-identity',
    title: 'Absolute ground identical with its condition',
    text: `The absolutely unconditioned is the absolute ground identical with its condition — the immediate fact as truly essential. As ground it refers negatively to itself and makes itself into a positedness that is a complete, self-identical reflection.`
  },
  {
    id: 'c3-chunk-2-positedness-sublated-immediacy',
    title: 'Positedness as sublated ground; immediacy as side of conditions',
    text: `This positedness is first the sublated ground: the fact as immediacy void of reflection, the side of the conditions. It is the totality of the determinations of the fact, the fact thrown into the externality of being — the restored circle of being.`
  },
  {
    id: 'c3-chunk-3-conditions-as-content-manifold',
    title: 'Conditions as whole content; manifold without unity',
    text: `Essence lets go of immanent unity and becomes immediacy that functions as a conditioning presupposition, constituting one side only. Thus conditions are the whole content of the fact (the unconditioned as formless being) yet appear as a manifold without unity, mingled with extra-essential elements and circumstances.`
  },
  {
    id: 'c3-chunk-4-sphere-of-being-and-sublated-reflection',
    title: 'Sphere of being as condition; reflection in being',
    text: `For the absolute fact the sphere of being itself is the condition. The ground returning posits that sphere as first immediacy. This immediacy, as sublated reflection, is reflection in the element of being; form proliferates as determinateness of being distinct from the determination of reflection.`
  },
  {
    id: 'c3-chunk-5-becoming-and-return',
    title: 'Becoming of being; return of essence to ground',
    text: `The unity of form sinks into immediacy as a passing-over of determinateness. The becoming of being is the coming-to-be of essence and a return to ground. Existence that constitutes the conditions makes itself into the moment of an other; its becoming is the doing of reflection.`
  },
  {
    id: 'c3-chunk-6-truth-existence-reflective-shine',
    title: 'Truth: existence is condition; immediacy/becoming are reflective moments',
    text: `The truth of existence is that it is condition; its immediacy is presupposed by the ground-connection's reflection (sublated). Immediacy and becoming are the reflective shine of the unconditioned and are only moments of form.`
  },
  {
    id: 'c3-chunk-7-ground-connection-form-vs-immediacy',
    title: 'Ground-connection: form as against immediacy of conditions',
    text: `The other side of the reflective shine is the ground-connection determined as form, opposed to the immediacy of the conditions and content. This side is the form of the absolute fact that unifies form with content, and by positing itself as form it sublates the diversity of the content, reducing it to a moment while granting itself immediacy of subsistence.`
  },
  {
    id: 'c3-chunk-8-reflection-sublates-immediacy',
    title: 'Reflection of ground sublates immediacy; conditions become moments',
    text: `The reflection of the ground sublates the immediacy of the conditions by connecting them and making them moments within the fact's unity. The conditions are presupposed by the unconditioned fact, and the fact thereby sublates its own positing so that positing immediately converts into becoming.`
  },
  {
    id: 'c3-chunk-9-positing-sublating-cycle-and-becoming',
    title: 'Positing ↔ Sublating ↔ Determination from within',
    text: `The reflection proceeds as presupposing, then immediately sublating itself into a positing that determines; that positing in turn sublates the presupposed and determines from within. The cycle is thus a becoming within itself — the mediation by negation has vanished into simple self-reflection and immanent becoming.`
  },
  {
    id: 'c3-chunk-10-mediation-disappears-fact-self-staging',
    title: 'Mediation disappears; the fact self-stages in concrete existence',
    text: `Mediation has disappeared into reflective shining; the fact's movement of being posited through conditions and ground is the vanishing of mediation's shine. The process of positing is a coming-forth, the simple self-staging of the fact in concrete existence — the pure movement of the fact to itself.`
  },
  {
    id: 'c3-chunk-11-all-conditions-at-hand-recollection',
    title: 'All conditions at hand → internal recollection → fact steps into existence',
    text: `When all the conditions of a fact are at hand the scattered manifold recollects itself: the whole fact is present within its conditions and thereby steps into concrete existence. The totality of conditions constitutes the reflection of the fact and effects its internal recollection.`
  },
  {
    id: 'c3-chunk-12-essence-immediacy-twofold-presence',
    title: 'Fact antecedently: essence (unconditioned) and immediate existence (determinate)',
    text: `The fact is prior to concrete existence as essence (the unconditioned) and as immediate existence (determined). Immediate existence appears twofold — in the conditions and in the ground — and its determinations are determinations of reflection.`
  },
  {
    id: 'c3-chunk-13-posited-ground-sublated',
    title: 'Posited ground becomes sublated; groundless immediacy and foundering',
    text: `As absolute reflection the fact makes itself its presupposition, producing a groundless immediacy whose being is simply to be there. The recollecting of conditions is the foundering of immediate existence to ground and the coming-to-be of a posited ground, which, insofar as it is ground, is immediately sublated.`
  },
  {
    id: 'c3-chunk-14-coming-forth-disappearance-mediation',
    title: 'Coming-forth into concrete existence = disappearance of mediation',
    text: `If all conditions are present they sublate themselves as immediacy and presupposition and the ground is sublated; the reflective shine disappears. The coming-forth is thus the tautological movement of the fact to itself: its mediation through conditions and ground vanishes, and the fact's entry into concrete existence is immediate except for that disappearance.`
  },
  {
    id: 'c3-chunk-15-transition-concrete-existence',
    title: 'Transition — Idea of Concrete Existence',
    text: `The fact proceeds from the ground; the positing is the outward movement of the ground to itself and the simple disappearing of it. Through union with the conditions the fact obtains external immediacy and the moment of being, not as alien addition but because the ground makes itself into a positedness and, by sublating its difference from that positedness, becomes simple essential immediacy. Thus the fact is both unconditioned and groundless: it arises only insofar as the ground has foundered and ceased to be a substrate. This immediacy — mediated by ground and condition but rendered self-identical through sublation — is concrete existence.`
  }
)

LOGICAL_OPERATIONS.push(
  {
    id: 'c3-op-1-ground-identity',
    chunkId: 'c3-chunk-1-absolute-ground-identity',
    label: 'AbsoluteGround ≡ Condition → positedness as self-identical reflection',
    clauses: [
      'absoluteGround.isIdenticalWith(condition)',
      'ground.refersNegativelyTo(ground)',
      'ground.positsItselfAs(positedness)',
      'positedness.isSelfIdentical(bothSides=true)'
    ],
    predicates: [
      { name: 'IsIdenticalWith', args: ['absoluteGround', 'condition'] },
      { name: 'RefersNegativelyTo', args: ['ground', 'itself'] },
      { name: 'PositsAs', args: ['ground', 'positedness'] }
    ],
    relations: [
      { predicate: 'identicalWith', from: 'absoluteGround', to: 'condition' },
      { predicate: 'posits', from: 'absoluteGround', to: 'positedness' }
    ]
  },
  {
    id: 'c3-op-2-positedness-sublation',
    chunkId: 'c3-chunk-2-positedness-sublated-immediacy',
    label: 'Positedness → sublated ground → immediacy (side of conditions)',
    clauses: [
      'positedness == sublatedGround',
      'sublatedGround.appearsAs(immediacyVoidOfReflection)',
      'immediacy.servesAs(sideOfConditions)',
      'fact.asTotality = determinations.thrownInto(externalBeing)'
    ],
    predicates: [
      { name: 'IsSublatedGround', args: ['positedness'] },
      { name: 'IsImmediacy', args: ['sublatedGround'] },
      { name: 'TotalityDeterminations', args: ['fact'] }
    ],
    relations: [
      { predicate: 'appearsAs', from: 'sublatedGround', to: 'immediacy' },
      { predicate: 'constitutes', from: 'immediacy', to: 'conditions' }
    ]
  },
  {
    id: 'c3-op-3-conditions-content-and-manifold',
    chunkId: 'c3-chunk-3-conditions-as-content-manifold',
    label: 'Conditions = whole content; also manifold w/o unity and extra-essential admixture',
    clauses: [
      'conditions.areWholeContentOf(fact)',
      'conditions.presentAs(formlessBeing)',
      'conditions.include(extraEssentialElements, circumstances)',
      'manifoldWithoutUnity != unityOfForm'
    ],
    predicates: [
      { name: 'WholeContent', args: ['conditions', 'fact'] },
      { name: 'FormlessBeing', args: ['conditions'] },
      { name: 'Includes', args: ['conditions', 'extraEssentialElements'] }
    ],
    relations: [
      { predicate: 'constitutes', from: 'conditions', to: 'fact.content' },
      { predicate: 'contains', from: 'conditions', to: 'extraEssentialElements' }
    ]
  },
  {
    id: 'c3-op-4-sphere-being-reflection',
    chunkId: 'c3-chunk-4-sphere-of-being-and-sublated-reflection',
    label: 'SphereOfBeing as condition; reflection-in-being creates determinateness',
    clauses: [
      'forAbsoluteFact: sphereOfBeing.isCondition = true',
      'ground.returning.posits(sphereOfBeing) as firstImmediacy',
      'sublatedReflection -> reflectionIn(elementOfBeing)',
      'form.proliferates => determinatenessOfBeing (distinctFromReflection)'
    ],
    predicates: [
      { name: 'SphereIsCondition', args: ['sphereOfBeing'] },
      { name: 'PositsAsImmediacy', args: ['ground', 'sphereOfBeing'] },
      { name: 'DeterminatenessOfBeing', args: ['form'] }
    ],
    relations: [
      { predicate: 'isCondition', from: 'sphereOfBeing', to: 'absoluteFact' },
      { predicate: 'posits', from: 'ground', to: 'sphereOfBeing' }
    ]
  },
  {
    id: 'c3-op-5-becoming-and-return',
    chunkId: 'c3-chunk-5-becoming-and-return',
    label: 'Becoming of being → coming-to-be of essence → return to ground',
    clauses: [
      'unityOfForm.sinksInto(immediacy) as passingOver',
      'becomingOfBeing => comingToBeOf(essence)',
      'existence.constitutingConditions makesItselfMomentOf(other)',
      'movementOfBecoming.isDoingOf(reflection)'
    ],
    predicates: [
      { name: 'PassingOver', args: ['unityOfForm', 'immediacy'] },
      { name: 'ComingToBe', args: ['essence'] },
      { name: 'DoesMovement', args: ['reflection'] }
    ],
    relations: [
      { predicate: 'sinksInto', from: 'unityOfForm', to: 'immediacy' },
      { predicate: 'returnsTo', from: 'essence', to: 'ground' }
    ]
  },
  {
    id: 'c3-op-6-truth-existence-reflective-shine',
    chunkId: 'c3-chunk-6-truth-existence-reflective-shine',
    label: 'Truth: existence is condition; immediacy & becoming are reflective moments',
    clauses: [
      'truthOfExistence := existence.isCondition',
      'existence.immediacy.isPresupposedBy(groundConnectionReflection)',
      'immediacy.isMomentOf(form)',
      'becoming.isReflectiveShineOf(unconditioned)'
    ],
    predicates: [
      { name: 'IsConditionTruth', args: ['existence'] },
      { name: 'PresupposedImmediacy', args: ['immediacy', 'groundConnection'] },
      { name: 'ReflectiveShine', args: ['becoming'] }
    ],
    relations: [
      { predicate: 'isCondition', from: 'existence', to: 'truth' },
      { predicate: 'momentOf', from: 'immediacy', to: 'form' }
    ]
  },
  {
    id: 'c3-op-7-ground-connection-form-vs-immediacy',
    chunkId: 'c3-chunk-7-ground-connection-form-vs-immediacy',
    label: 'Ground-connection (form) vs immediacy (conditions): unifies and sublates content',
    clauses: [
      'groundConnection.isForm && conditions.areImmediacy',
      'groundConnection.unifies(form, content)',
      'groundConnection.sublates(diversityOfContent) => contentReducedTo(moment)',
      'form.grantsItself(immediacyOfSubsistence)'
    ],
    predicates: [
      { name: 'IsForm', args: ['groundConnection'] },
      { name: 'Unifies', args: ['groundConnection', 'content'] },
      { name: 'Sublates', args: ['groundConnection', 'content'] }
    ],
    relations: [
      { predicate: 'opposes', from: 'groundConnection', to: 'immediacy' },
      { predicate: 'reducesToMoment', from: 'groundConnection', to: 'content' }
    ]
  },
  {
    id: 'c3-op-8-reflection-sublates-immediacy',
    chunkId: 'c3-chunk-8-reflection-sublates-immediacy',
    label: 'Reflection sublates immediacy; conditions become moments of unity',
    clauses: [
      'reflectionOfGround.connects(conditions) => conditions.become(moments)',
      'fact.presupposes(conditions) && fact.sublates(itsPositing)',
      'positedness.convertsImmediatelyTo(becoming)'
    ],
    predicates: [
      { name: 'Connects', args: ['reflectionOfGround', 'conditions'] },
      { name: 'Presupposes', args: ['fact', 'conditions'] },
      { name: 'ConvertsTo', args: ['positedness', 'becoming'] }
    ],
    relations: [
      { predicate: 'connects', from: 'reflectionOfGround', to: 'conditions' },
      { predicate: 'sublates', from: 'fact', to: 'itsPositing' }
    ]
  },
  {
    id: 'c3-op-9-positing-sublating-cycle',
    chunkId: 'c3-chunk-9-positing-sublating-cycle-and-becoming',
    label: 'Presupposing → Positing → Sublating → Becoming (immanent cycle)',
    clauses: [
      'reflection.presupposes() -> presupposing',
      'presupposing.sublatesItselfImmediateAs(positing)',
      'positing.subsumes(presupposed) => determiningFromWithin',
      'cycle => becomingWithinItself (mediation disappears)'
    ],
    predicates: [
      { name: 'Presupposing', args: ['reflection'] },
      { name: 'Positing', args: ['reflection'] },
      { name: 'DeterminesFromWithin', args: ['positing'] }
    ],
    relations: [
      { predicate: 'sublatesInto', from: 'presupposing', to: 'positing' },
      { predicate: 'determines', from: 'positing', to: 'presupposed' }
    ]
  },
  {
    id: 'c3-op-10-mediation-disappears-self-staging',
    chunkId: 'c3-chunk-10-mediation-disappears-fact-self-staging',
    label: 'Disappearance of mediation → fact's self-staging in concrete existence',
    clauses: [
      'mediation.asTurningBack.disappears -> mediation = simpleReflection',
      'fact.movement(positedThroughConditions, positedThroughGround) => disappearanceOfReflectiveShine',
      'processOfPositing == comingForth == fact.selfStaging()'
    ],
    predicates: [
      { name: 'Disappears', args: ['mediation'] },
      { name: 'SelfStaging', args: ['fact'] },
      { name: 'ComingForth', args: ['processOfPositing'] }
    ],
    relations: [
      { predicate: 'becomes', from: 'mediation', to: 'simpleReflection' },
      { predicate: 'selfStages', from: 'fact', to: 'concreteExistence' }
    ]
  },
  {
    id: 'c3-op-11-all-conditions-recollect',
    chunkId: 'c3-chunk-11-all-conditions-at-hand-recollection',
    label: 'All conditions at hand → recollection → fact steps into concrete existence',
    clauses: [
      'if conditions.totality.isPresent then manifold.recollectsInternally()',
      'recollection => fact.isPositedWithin(conditions) && fact.stepsInto(existence)',
      'totalityOfConditions == reflectionOfFact'
    ],
    predicates: [
      { name: 'TotalityPresent', args: ['conditions'] },
      { name: 'RecollectsInternally', args: ['manifold'] },
      { name: 'IsPositedWithin', args: ['fact','conditions'] }
    ],
    relations: [
      { predicate: 'recollects', from: 'conditions', to: 'fact' },
      { predicate: 'stepsInto', from: 'fact', to: 'concreteExistence' }
    ]
  },
  {
    id: 'c3-op-12-essence-and-immediacy-twofold',
    chunkId: 'c3-chunk-12-essence-immediacy-twofold-presence',
    label: 'Fact = essence (unconditioned) + immediate existence (determinate) — twofold determination',
    clauses: [
      'fact.existsAs(essence, immediateExistence)',
      'immediateExistence.appearsIn(conditions, ground)',
      'determinationsOfImmediate := determinationsOfReflection'
    ],
    predicates: [
      { name: 'ExistsAs', args: ['fact','modes'] },
      { name: 'AppearsIn', args: ['immediacy','locations'] },
      { name: 'DeterminationsOfReflection', args: ['determinations'] }
    ],
    relations: [
      { predicate: 'appearsAs', from: 'fact', to: 'essence' },
      { predicate: 'appearsAs', from: 'fact', to: 'immediateExistence' }
    ]
  },
  {
    id: 'c3-op-13-posited-ground-sublated',
    chunkId: 'c3-chunk-13-posited-ground-sublated',
    label: 'Posited ground (groundless immediacy) → sublation of ground when conditions total',
    clauses: [
      'fact.asAbsoluteReflection => presupposedUnconditioned = groundlessImmediacy',
      'if conditions.totality.present then immediate.recollected -> ground.positedButSublated',
      'ground.asPosited.isSublated -> reflectiveShine.disappears'
    ],
    predicates: [
      { name: 'GroundlessImmediacy', args: ['presupposedUnconditioned'] },
      { name: 'PositedButSublated', args: ['ground'] },
      { name: 'ReflectiveShineDisappears', args: ['ground'] }
    ],
    relations: [
      { predicate: 'isPresupposition', from: 'groundlessImmediacy', to: 'fact' },
      { predicate: 'sublates', from: 'ground', to: 'immediacy' }
    ]
  },
  {
    id: 'c3-op-14-coming-forth-disappearance-mediation',
    chunkId: 'c3-chunk-14-coming-forth-disappearance-mediation',
    label: 'Coming-forth into concrete existence = disappearance of mediation',
    clauses: [
      'conditions.totality.present => conditions.sublateAs(immediacy,presupposition)',
      'ground.sublated && conditions.sublated => reflectiveShine.disappears',
      'comingForth := tautologicalMovement(factToItself) = immediateEntryExceptFor(disappearanceOfMediation)'
    ],
    predicates: [
      { name: 'SublatesAs', args: ['conditions','immediacy'] },
      { name: 'ReflectiveShineVanishes', args: ['process'] },
      { name: 'TautologicalMovement', args: ['comingForth'] }
    ],
    relations: [
      { predicate: 'sublates', from: 'conditions', to: 'immediacy' },
      { predicate: 'vanishes', from: 'reflectiveShine', to: 'nothing' }
    ]
  },
  {
    id: 'c3-op-15-transition-concrete-existence',
    chunkId: 'c3-chunk-15-transition-concrete-existence',
    label: 'Fact proceeds from ground → positing = outward movement & disappearance → concrete existence',
    clauses: [
      'fact.proceedsFrom(ground)',
      'ground.positsItself() => outwardMovement && ground.disappearsAs(substrate)',
      'unionWith(conditions) => fact.obtains(externalImmediacy, momentOfBeing)',
      'ground.sublatesDifferenceWith(positedness) => essentialImmediacy',
      'fact.isUnconditioned && fact.isGroundless',
      'concreteExistence := immediacy.mediatedBy(ground,conditions) && immediacy.selfIdenticalBy(sublation)'
    ],
    predicates: [
      { name: 'ProceedsFrom', args: ['fact','ground'] },
      { name: 'OutwardMovement', args: ['ground'] },
      { name: 'EssentialImmediacy', args: ['ground','positedness'] },
      { name: 'ConcreteExistence', args: ['immediacy'] }
    ],
    relations: [
      { predicate: 'proceedsFrom', from: 'fact', to: 'ground' },
      { predicate: 'obtains', from: 'fact', to: 'externalImmediacy' },
      { predicate: 'sublates', from: 'ground', to: 'positedness' },
      { predicate: 'constitutes', from: 'immediacy', to: 'concreteExistence' }
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

