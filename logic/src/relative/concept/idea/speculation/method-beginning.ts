import type { Chunk, LogicalOperation } from '../../../types';

/**
 * METHOD SECTION 1: THE BEGINNING
 * 
 * The beginning as simple and universal, the absolute as implicit.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'spec-method-1-beginning',
    title: 'The beginning — simple and universal',
    text: `Accordingly, what constitutes the method are
the determinations of the concept itself and their connections,
and these we must now examine in the significance
that they have as determinations of the method.
In this, we must begin from the beginning.
We spoke of this beginning at the very beginning of the Logic,
and also in connection with subjective cognition,
and we showed that, when not performed arbitrarily
and in the absence of categorial sensitivity,
though it may seem to present many difficulties,
it is nevertheless of an extremely simple nature.
Because it is the beginning, its content is an immediate,
but one that has the meaning and the form of abstract universality.
Or be it a content of being, or of essence or of the concept,
inasmuch as it is something immediate, it is assumed,
found in advance, assertoric.
But first of all it is not an immediate of
sense-intuition or of representation,
but of thought, which because of its immediacy can
also be called a supersensuous, inner intuiting.
The immediate of sense-intuition is a manifold and a singular.
Cognition, on the contrary, is a thinking that conceptualizes;
its beginning, therefore, is also only in the element of thought,
a simple and a universal.
We spoke of this form earlier, in connection with definition.
At the beginning of finite cognition universality is likewise
recognized as an essential determination,
but only as thought and concept
determination in opposition to being.
In fact this first universality is an immediate universality,
and for that reason it has equally the significance of being,
for being is precisely this abstract self-reference.
Being has no need of further derivation,
as if it came to the abstract element of definition
only because taken from the intuition of the senses or elsewhere,
and in so far as it can be pointed at.
This pointing and deriving involve a mediation
that is more than a mere beginning,
and is a mediation of a kind that does not
belong to the comprehension of thought,
but is rather the elevation of representation,
of empirical and ratiocinative consciousness,
to the standpoint of thinking.
According to the currently accepted opposition
of thought, or concept, and being,
it passes as a very important truth that
no being belongs as yet to thought as thought,
and that being has a ground of its own independent of thought.
But the simple determination of being is in itself so poor that,
if for that reason alone, not much fuss ought to be made about it;
the universal is immediately itself this immediate
because, as abstract, it is also the
abstract self-reference which is being.
In fact, the demand that being should be exhibited has
a further, inner meaning in which more is at issue
than just this abstract determination;
implied in it is the demand for the realization of the concept,
a realization that is missing at the beginning itself
but is rather the goal and the business of the
entire subsequent development of cognition.

The beginning, therefore, has for the method no other determinateness
than that of being the simple and universal;
this is precisely the determinateness that makes it deficient.
Universality is the pure, simple concept,
and the method, as the consciousness of this concept,
is aware that universality is only a moment
and that in it the concept is still not determined in and for itself.
But with this consciousness that would want to carry
the beginning further only for the sake of method,
the method is only a formal procedure
posited in external reflection.
Where the method, however,
is the objective and immanent form,
the immediate character of the beginning must be
a lack inherent in the beginning itself,
which must be endowed with the
impulse to carry itself further.
But in the absolute method the universal
has the value not of a mere abstraction
but of the objective universal, that is,
the universal that is in itself the concrete totality,
but a totality as yet not posited, not yet for itself.
Even the abstract universal is as such,
when considered conceptually, that is, in its truth,
not just anything simple, but is, as abstract,
already posited afflicted by a negation.
For this reason also there is nothing so simple and so abstract,
be it in actuality or in thought, as is commonly imagined.
Anything as simple as that is a mere presumption
that has its ground solely in the lack of
awareness of what is actually there.
We said earlier that the beginning is
made with the immediate;
the immediacy of the universal is the same as
what is here expressed as the in-itself
that is without being-for-itself.
One may well say, therefore, that every
beginning must be made with the absolute,
just as every advance is only the exposition of it,
in so far as implicit in existence is the concept.
But because the absolute exists
at first only implicitly, in itself,
it equally is not the absolute
nor the posited concept,
and also not the idea,
for the in-itself is only
an abstract, one-sided moment,
and this is what they are.
The advance is not, therefore, a kind of superfluity;
this is what it would be if that which is
at the beginning were already the absolute;
the advance consists rather in this,
that the universal determines itself
and is the universal for itself,
that is, equally a singular and a subject.
Only in its consummation is it the absolute.`,
    summary: 'Method = determinations of concept and connections. Beginning = simple nature. Content = immediate = abstract universality. Not sense-intuition/representation, but thought = supersensuous inner intuiting. Beginning = simple/universal in element of thought. Universality = immediate universality = being (abstract self-reference). Being needs no derivation. Universal = immediate = abstract self-reference = being. Demand for being exhibited = demand for realization of concept (goal of development). Beginning = simple/universal (deficient). Universality = pure simple concept, only moment, concept not determined. Method objective/immanent: beginning has lack, impulse to carry further. Absolute method: universal = objective universal = concrete totality (not posited/for itself). Abstract universal = posited afflicted by negation. Nothing so simple/abstract. Beginning = immediate = in-itself without being-for-itself. Every beginning = absolute (implicit in existence = concept). Absolute implicit = not absolute/not posited concept/not idea. Advance = universal determines itself = universal for itself = singular/subject. Consummation = absolute.'
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'spec-op-method-1-beginning',
    chunkId: 'spec-method-1-beginning',
    label: 'The beginning',
    clauses: [
      'method = determinationsOfConcept',
      'beginning = simpleNature',
      'content = immediate',
      'content = abstractUniversality',
      'beginning = thought',
      'beginning = simpleUniversal',
      'universality = immediateUniversality',
      'universality = being',
      'being = abstractSelfReference',
      'beginning = simpleUniversal',
      'beginning.deficient = true',
      'universality = pureSimpleConcept',
      'universality = onlyMoment',
      'method.objectiveImmanent = true',
      'beginning.hasLack = true',
      'beginning.hasImpulse = true',
      'absoluteMethod.universal = objectiveUniversal',
      'objectiveUniversal = concreteTotality',
      'objectiveUniversal.notPosited = true',
      'abstractUniversal = positedAfflictedByNegation',
      'beginning = immediate',
      'immediacy = inItselfWithoutBeingForItself',
      'everyBeginning = absolute',
      'absolute.implicit = true',
      'absolute.notAbsolute = true',
      'advance = universalDeterminesItself',
      'universalDeterminesItself = universalForItself',
      'universalForItself = singularSubject',
      'consummation = absolute',
    ],
    predicates: [
      { name: 'IsBeginning', args: ['simpleUniversal'] },
      { name: 'IsAbsolute', args: ['consummation'] },
    ],
    relations: [
      { predicate: 'is', from: 'beginning', to: 'simpleUniversal' },
      { predicate: 'is', from: 'consummation', to: 'absolute' },
    ],
  },
];

