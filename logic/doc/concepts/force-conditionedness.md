# Force — Part 1: Conditionedness (study notes)

Key ideas to operationalize cheaply:

- Negative unity: force is the unity that sublates the earlier whole/parts opposition; we model it as a computed score and provenance, not a new entity.
- Presupposition: force presupposes immediacy (a thing/matter). We record this as `presupposesThing`.
- Positedness: force appears as positedness (reflection) rather than being a property of the thing. We record `appearsAsPositedness`.
- Transience: immediacy is a transient, self‑sublating moment; when both immediacy and reflection are present, we flag `immediacyTransient`.
- “Designated as matter”: we infer tags from evidence (magnetic, electric, ether, etc.) to reflect the text without baking physics into schema.

Contract sketch

- Input: ForceCandidate { id, immediacy?, reflection?, externalityTags?, thingId?, hints? }
- Output: ForceConditionReport { id, presupposesThing, appearsAsPositedness, immediacyTransient, designatedAsMatter[], negativeUnityScore, signature, evidence[] }

This stays a projection over existing data and can later feed Part 2 (solicitation/reciprocity/infinity) and Action/World-of-Forces helpers.
