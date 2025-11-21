import type { Chunk, LogicalOperation } from './index'

/*
  Foundation — A. IDENTITY

  This module covers the Identity section of the Determinations of Reflection.

  STRUCTURE: Essence → Foundation → Ground
  - Essence: The Essential and the Unessential
  - Foundation: The Determinations of Reflection (Identity, Difference, Contradiction)
  - Ground: The resolved contradiction (Absolute Ground, Determinate Ground, Condition)

  PHILOSOPHICAL NOTES:

  1. **Identity as the First Determination**:
     Identity is the first determination of reflection — essence as simple self-reference,
     pure identity. It is the immediacy of reflection, not abstract identity but
     essential identity produced from itself.

  2. **Identity = Essence**:
     Identity is not separate from essence; it is essence itself. The negativity of being
     in itself is identity itself. This establishes the fundamental unity: identity
     remains the same as essence.

  3. **Foundation as the Shining of Essence**:
     Reflection is the shining of essence within itself. Identity is the first moment
     of this Foundation — the pure self-reference that forms the basis for all further
     determinations (Difference, Contradiction) which resolve to Ground.
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'id-1-essence-immediacy',
    title: 'Essence as simple sublated immediacy',
    text: `Essence is simple immediacy as sublated immediacy. Its negativity is its being; through this self‑negation otherness and reference disappear into pure self‑equality. Essence is therefore simple self‑identity.`
  },
  {
    id: 'id-2-self-identity-of-reflection',
    title: 'Self‑identity as immediacy of reflection',
    text: `This self‑identity is the immediacy of reflection: a self‑equality that produces itself from itself, not by being reproduced from another source, but as essential, internal identity.`
  },
  {
    id: 'id-3-not-abstract-identity',
    title: 'Not abstract or relative identity',
    text: `Identity here is not abstract nor the product of a prior relative negation that leaves the otherness existing outside it; instead being and its determinateness sublate themselves in themselves, yielding identity as internal negativity.`
  },
  {
    id: 'id-4-identity-equivalence-essence',
    title: 'Identity equals essence',
    text: `Because being has sublated itself in itself, this simple negativity — the negativity of being in itself — is identity itself. In general, identity remains the same as essence.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'id-op-1-sublation-declares',
    chunkId: 'id-1-essence-immediacy',
    label: 'Declare essence as sublated immediacy → self‑identity',
    clauses: ['essence = sublatedImmediacy', 'negativityOfBeing => disappearanceOfOtherness', 'result = selfIdentity'],
    predicates: [{ name: 'IsSublatedImmediacy', args: ['essence'] }],
    relations: [{ predicate: 'yields', from: 'negation', to: 'selfIdentity' }]
  },
  {
    id: 'id-op-2-self-production',
    chunkId: 'id-2-self-identity-of-reflection',
    label: 'Formalize self‑production: identity produced from itself',
    clauses: ['selfEquality.producesItself = true', 'production.source = self (not other)'],
    predicates: [{ name: 'IsSelfProducing', args: ['identity'] }],
    relations: [{ predicate: 'originatesFrom', from: 'identity', to: 'itself' }]
  },
  {
    id: 'id-op-3-not-abstract',
    chunkId: 'id-3-not-abstract-identity',
    label: 'Contrast with abstract/relative identity',
    clauses: ['not = abstractIdentity', 'not = relativeNegationResult', 'being.sublatesInItself = true'],
    predicates: [{ name: 'IsInternalIdentity', args: [] }],
    relations: [{ predicate: 'contrastsWith', from: 'internalIdentity', to: 'abstractIdentity' }]
  },
  {
    id: 'id-op-4-equate-essence-identity',
    chunkId: 'id-4-identity-equivalence-essence',
    label: 'Equate identity with essence (summary move)',
    clauses: ['identity == essence', 'negativityOfBeing => identityInItself'],
    predicates: [{ name: 'EssenceEqualsIdentity', args: [] }],
    relations: [{ predicate: 'summarizes', from: 'identity', to: 'essence' }]
  }
]
