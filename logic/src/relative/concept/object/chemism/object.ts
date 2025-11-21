import type { Chunk, LogicalOperation } from '../../../types';

/*
  Chemical Object — A. THE CHEMICAL OBJECT

  This module translates the Chemical Object section:
  - Distinction from mechanical object: non-indifference to determinateness
  - Determinateness as universal principle
  - Distinction: inner totality vs external determinateness
  - Contradiction: immediate positedness vs immanent concept
  - Striving to sublate and actualize objective totality

  PHILOSOPHICAL NOTES:

  1. **Distinction from Mechanical Object**:
     Chemical object distinguished from mechanical in that latter is totality indifferent
     to determinateness, whereas in chemical object determinateness, reference to other,
     and mode and manner of this reference belong to its nature. Determinateness is
     essentially particularization - taken up into universality, thus a principle:
     determinateness which is universal, not only of one singular object but also of other.
     In chemical object there is distinction in concept between inner totality of two
     determinacies and determinateness constituting nature of singular object in externality
     and concrete existence. Since object is implicitly whole concept, it has necessity
     and impulse to sublate opposed one-sided subsistence, bring itself in existence to
     real whole which it is according to concept.

  2. **Chemism as Universal Relation**:
     Expression "chemism" for relation of non-indifference of objectivity not to be
     understood as only found in elemental nature strictly going by that name. Meteorological
     relation must be regarded as process whose parts have more nature of physical than
     chemical elements. In animate things, sex relation falls under this schema. Schema
     also constitutes formal basis for spiritual relations of love, friendship, and the like.
     Chemism is universal pattern of non-indifference - objects determinedly related,
     striving towards each other.

  3. **Chemical Object as Basis**:
     On closer examination, chemical object is at first self-subsistent totality in general,
     reflected into itself, distinct from reflectedness outwards - indifferent basis, individual
     not yet determined as non-indifferent. Person too is in first instance basis of this kind,
     refers only to itself. But immanent determinateness constituting object's non-indifference
     is, first, reflected into itself such that retraction of reference outwards is only formal
     abstract universality. Outwards reference is determination of object's immediacy and concrete
     existence. From this side object does not return within it to individual totality - negative
     unity has two moments of opposition in two particular objects. Accordingly, chemical object
     is not comprehensible from itself - being of one object is being of another.

  4. **Determinateness as Real Genus**:
     Second, determinateness is absolutely reflected into itself and is concrete moment of
     individual concept of whole which is universal essence, real genus of particular objects.
     Chemical object is contradiction of immediate positedness and immanent individual concept -
     striving to sublate immediate determinateness of existence and give concrete existence to
     objective totality of concept. Hence it still remains non-self-subsistent object, but in
     such way that it is by nature in tension with lack of self-subsistence and initiates process
     as self-determining.

  5. **Transition from Mechanism**:
     Chemical Object marks transition from Mechanism where objects were indifferent, externally
     related. In Chemism, objects are determinedly opposed, non-indifferent - reference to other
     belongs to nature. This is first negation of indifferent objectivity and externality of
     determinateness. Objects strive towards each other, initiate process as self-determining -
     preparing for Chemical Process where they unite and separate.
*/

// ============================================================================
// SECTION III: OBJECTIVITY
// ============================================================================

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'chem-obj-distinction',
    title: 'Distinction from mechanical object: non-indifference',
    text: `A. THE CHEMICAL OBJECT

The chemical object is distinguished from
the mechanical in that the latter is
a totality indifferent to determinateness,
whereas in the chemical object the determinateness,
and hence the reference to other,
and the mode and manner of this reference,
belong to its nature.
This determinateness is at the same time
essentially a particularization,
that is, it is taken up into universality;
thus it is a principle:
a determinateness which is universal,
not only the determinateness
of the one singular object
but also of the other.
In the chemical object there is now, therefore,
a distinction in its concept,
between the inner totality of the two determinacies
and the determinateness that constitutes
the nature of the singular object
in its externality and concrete existence.
Since in this way the object is implicitly the whole concept,
it has within it the necessity and the impulse
to sublate its opposed, one-sided subsistence,
and to bring itself in existence to the real whole
which it is according to its concept.`,
    summary:
      'Chemical object distinguished from mechanical: latter is totality indifferent to determinateness, whereas in chemical object determinateness, reference to other, mode and manner belong to nature. Determinateness is particularization taken up into universality - universal principle, not only of one but also of other. Distinction in concept: inner totality of two determinacies vs determinateness constituting nature in externality. Object implicitly whole concept - has necessity and impulse to sublate opposed one-sided subsistence, bring itself to real whole.'
  },

  {
    id: 'chem-obj-universal',
    title: 'Chemism as universal relation',
    text: `Regarding the expression "chemism" for the said
relation of the non-indifference of objectivity,
it may be further remarked that the expression is
not to be understood here as though the relation were
only to be found in that form of elemental nature
that strictly goes by that name.
Already the meteorological relation must be regarded
as a process whose parts have more the nature
of physical than chemical elements.
In animate things, the sex relation falls under this schema,
and the schema also constitutes the formal basis
for the spiritual relations of love, friendship, and the like.`,
    summary:
      'Chemism: Expression for relation of non-indifference of objectivity not to be understood as only found in elemental nature. Meteorological relation regarded as process with physical elements. In animate things, sex relation falls under schema. Schema constitutes formal basis for spiritual relations of love, friendship, and the like.'
  },

  {
    id: 'chem-obj-basis',
    title: 'Chemical object as basis: contradiction',
    text: `On closer examination, the chemical object is
at first a self-subsistent totality in general,
one reflected into itself and therefore distinct
from its reflectedness outwards, an indifferent basis,
the individual not yet determined as non-indifferent;
the person, too, is in the first instance a basis
of this kind, one that refers only to itself.
But the immanent determinateness
that constitutes the object's non-indifference is,
first, reflected into itself in such a manner
that this retraction of the reference outwards is
only a formal abstract universality;
the outwards reference is thus a determination
of the object's immediacy and concrete existence.
From this side the object does not return,
within it, to individual totality:
the negative unity has its two moments
of opposition in two particular objects.
Accordingly, a chemical object is
not comprehensible from itself,
and the being of one object is
the being of another.
But, second, the determinateness is
absolutely reflected into itself
and is the concrete moment of
the individual concept of the whole
which is the universal essence,
the real genus of the particular objects.
The chemical object,
which is thus the contradiction
of its immediate positedness
and its immanent individual concept,
is a striving to sublate
the immediate determinateness of its existence
and to give concrete existence to
the objective totality of the concept.
Hence it does still remain a non-self-subsistent object,
but in such a way that it is by nature in tension
with this lack of self-subsistence
and initiates the process as a self-determining.`,
    summary:
      'Chemical object: At first self-subsistent totality, reflected into itself, distinct from reflectedness outwards - indifferent basis, individual not yet non-indifferent. Person too is basis referring only to itself. But immanent determinateness constituting non-indifference is, first, reflected into itself - retraction is only formal abstract universality. Outwards reference is determination of immediacy and concrete existence. Object does not return to individual totality - negative unity has two moments in two particular objects. Chemical object not comprehensible from itself - being of one is being of another. Second, determinateness absolutely reflected into itself - concrete moment of individual concept of whole, universal essence, real genus. Chemical object is contradiction of immediate positedness and immanent individual concept - striving to sublate immediate determinateness, give concrete existence to objective totality. Remains non-self-subsistent but in tension with lack, initiates process as self-determining.'
  }
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'op-chem-obj-1',
    chunkId: 'chem-obj-distinction',
    label: 'Distinction: non-indifference to determinateness',
    clauses: [
      'Chemical object distinguished from mechanical',
      'Mechanical is totality indifferent to determinateness',
      'In chemical object determinateness belongs to nature',
      'Reference to other belongs to nature',
      'Mode and manner of reference belong to nature',
      'Determinateness is essentially particularization',
      'Taken up into universality',
      'Thus a principle: universal determinateness',
      'Not only of one singular object but also of other',
      'Distinction in concept: inner totality vs external determinateness',
      'Object implicitly whole concept',
      'Has necessity and impulse to sublate opposed one-sided subsistence',
      'Bring itself to real whole according to concept'
    ],
    predicates: [
      { name: 'is', args: ['mechanical', 'indifferent'] },
      { name: 'belongs', args: ['determinateness', 'toNature'] },
      { name: 'belongs', args: ['referenceToOther', 'toNature'] },
      { name: 'is', args: ['determinateness', 'particularization'] },
      { name: 'is', args: ['determinateness', 'takenUpIntoUniversality'] },
      { name: 'is', args: ['determinateness', 'principle'] },
      { name: 'is', args: ['determinateness', 'universal'] },
      { name: 'is', args: ['object', 'wholeConcept'] },
      { name: 'has', args: ['object', 'necessity'] },
      { name: 'has', args: ['object', 'impulse'] },
      { name: 'toSublate', args: ['object', 'opposedSubsistence'] },
      { name: 'toBring', args: ['object', 'toRealWhole'] }
    ],
    relations: [
      { predicate: 'belongs', from: 'determinateness', to: 'toNature' },
      { predicate: 'is', from: 'object', to: 'wholeConcept' }
    ],
    candidateSummary:
      'Chemical object: Determinateness belongs to nature - reference to other, mode and manner. Determinateness is particularization, universal principle. Object implicitly whole concept - has necessity and impulse to sublate opposed subsistence, bring to real whole.'
  },

  {
    id: 'op-chem-obj-2',
    chunkId: 'chem-obj-universal',
    label: 'Chemism as universal relation',
    clauses: [
      'Chemism: expression for relation of non-indifference',
      'Not only found in elemental nature',
      'Meteorological relation regarded as process',
      'Parts have more nature of physical than chemical elements',
      'In animate things, sex relation falls under schema',
      'Schema constitutes formal basis for spiritual relations',
      'Love, friendship, and the like'
    ],
    predicates: [
      { name: 'is', args: ['chemism', 'expression'] },
      { name: 'is', args: ['chemism', 'nonIndifference'] },
      { name: 'isNot', args: ['chemism', 'onlyElemental'] },
      { name: 'is', args: ['meteorologicalRelation', 'process'] },
      { name: 'have', args: ['parts', 'physicalNature'] },
      { name: 'falls', args: ['sexRelation', 'underSchema'] },
      { name: 'constitutes', args: ['schema', 'formalBasis'] },
      { name: 'are', args: ['love', 'spiritualRelation'] },
      { name: 'are', args: ['friendship', 'spiritualRelation'] }
    ],
    relations: [
      { predicate: 'falls', from: 'sexRelation', to: 'underSchema' },
      { predicate: 'constitutes', from: 'schema', to: 'formalBasis' }
    ],
    candidateSummary:
      'Chemism: Universal relation of non-indifference, not only elemental. Meteorological, sexual, spiritual relations (love, friendship) fall under schema.'
  },

  {
    id: 'op-chem-obj-3',
    chunkId: 'chem-obj-basis',
    label: 'Chemical object: contradiction and striving',
    clauses: [
      'At first self-subsistent totality',
      'Reflected into itself, distinct from reflectedness outwards',
      'Indifferent basis, individual not yet non-indifferent',
      'Person too is basis referring only to itself',
      'Immanent determinateness constituting non-indifference',
      'First: reflected into itself - retraction is formal abstract universality',
      'Outwards reference is determination of immediacy',
      'Object does not return to individual totality',
      'Negative unity has two moments in two particular objects',
      'Chemical object not comprehensible from itself',
      'Being of one object is being of another',
      'Second: determinateness absolutely reflected into itself',
      'Concrete moment of individual concept of whole',
      'Universal essence, real genus',
      'Chemical object is contradiction',
      'Immediate positedness vs immanent individual concept',
      'Striving to sublate immediate determinateness',
      'Give concrete existence to objective totality',
      'Remains non-self-subsistent',
      'But in tension with lack of self-subsistence',
      'Initiates process as self-determining'
    ],
    predicates: [
      { name: 'is', args: ['chemicalObject', 'selfSubsistentTotality'] },
      { name: 'is', args: ['chemicalObject', 'reflectedIntoItself'] },
      { name: 'is', args: ['chemicalObject', 'indifferentBasis'] },
      { name: 'is', args: ['individual', 'notYetNonIndifferent'] },
      { name: 'is', args: ['person', 'basis'] },
      { name: 'refers', args: ['person', 'onlyToItself'] },
      { name: 'is', args: ['determinateness', 'reflectedIntoItself'] },
      { name: 'is', args: ['retraction', 'formalAbstractUniversality'] },
      { name: 'is', args: ['outwardsReference', 'determination'] },
      { name: 'doesNot', args: ['object', 'return'] },
      { name: 'has', args: ['negativeUnity', 'twoMoments'] },
      { name: 'isNot', args: ['chemicalObject', 'comprehensibleFromItself'] },
      { name: 'is', args: ['beingOfOne', 'beingOfAnother'] },
      { name: 'is', args: ['determinateness', 'absolutelyReflected'] },
      { name: 'is', args: ['determinateness', 'concreteMoment'] },
      { name: 'is', args: ['concept', 'universalEssence'] },
      { name: 'is', args: ['concept', 'realGenus'] },
      { name: 'is', args: ['chemicalObject', 'contradiction'] },
      { name: 'is', args: ['striving', 'toSublate'] },
      { name: 'is', args: ['striving', 'toGiveExistence'] },
      { name: 'remains', args: ['chemicalObject', 'nonSelfSubsistent'] },
      { name: 'is', args: ['chemicalObject', 'inTension'] },
      { name: 'initiates', args: ['chemicalObject', 'process'] },
      { name: 'is', args: ['chemicalObject', 'selfDetermining'] }
    ],
    relations: [
      { predicate: 'is', from: 'chemicalObject', to: 'contradiction' },
      { predicate: 'initiates', from: 'chemicalObject', to: 'process' }
    ],
    candidateSummary:
      'Chemical object: At first self-subsistent, indifferent basis. But immanent determinateness - first reflected into itself (formal universality), outwards reference. Object not comprehensible from itself - being of one is being of another. Second, determinateness absolutely reflected - universal essence, real genus. Contradiction: immediate positedness vs immanent concept. Striving to sublate, give existence to totality. In tension, initiates process as self-determining.'
  }
];

/* minimal, stable accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getChunkById(id: string): Chunk | null {
  return CANONICAL_CHUNKS.find((c) => c.id === id) ?? null;
}

export function getLogicalOpsForChunkId(chunkId: string): LogicalOperation[] {
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunkId);
}

export function getAllChunks(): Chunk[] {
  return CANONICAL_CHUNKS;
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS;
}
