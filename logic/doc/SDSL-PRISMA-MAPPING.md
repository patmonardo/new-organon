SDSL → Prisma Mapping for `Entity.particulars`

## Overview

This document describes a minimal mapping from SDSL domain metadata to a Prisma schema and runtime pattern that implements `Entity.particulars` (sublated particulars). The mapping is intentionally minimal and conservative so it can be adapted to both join-table and FK-style representations.

## SDSL metadata sketch

When a Domain is defined in SDSL, include a `particulars` block that signals where particulars live and how they link:

```json
{
  "domain": "Person",
  "formId": "form:person:v1",
  "particulars": {
    "source": "particulars", // source table/model name
    "join": "joinTable", // "joinTable" or "fk"
    "joinTable": "person_particulars",
    "roleField": "role" // optional field name to record dialectical role
  }
}
```

## Prisma options

Option A — Join table (flexible, particulars reused across entities)

```prisma
model Person {
  id           String             @id @default(cuid())
  name         String
  particulars  PersonParticular[]
}

model Particular {
  id       String              @id @default(cuid())
  type     String
  data     Json
  persons  PersonParticular[]
}

model PersonParticular {
  id           String   @id @default(cuid())
  personId     String
  particularId String
  role         String?
  person       Person   @relation(fields: [personId], references: [id])
  particular   Particular @relation(fields: [particularId], references: [id])
  @@index([personId])
}
```

Option B — FK on Particular (simple, particular belongs to one entity)

```prisma
model Particular {
  id       String @id @default(cuid())
  type     String
  data     Json
  entityId String?
  role     String?
  entity   Person? @relation(fields:[entityId], references:[id])
}
```

## Zod / SDSL adaptation

- Extend `EntityRef` with optional `role` (already added to `logic/src/schema/entity.ts`).
- Validate incoming SDSL mapping and particulars using Zod before applying DB changes.

## Runtime procedure pattern (Procedure-First Controller)

1. Parse and validate SDSL mapping and incoming Entity shape via Zod.
2. Upsert Particular records (or accept provided ids).
3. Create or upsert Entity row (Form reference).
4. Create join-table rows or set `entityId` on Particulars.
5. Persist `Entity.facets.particulars` as an array of `{id,type,role}` for fast read.
6. Emit audit/event for teaching/sublation rounds.

## Example considerations

- Use join table when particulars are shared or reused across many entities.
- Use FK when particulars are unique to a single entity and simpler queries are desired.
- Keep role optional: roles are pedagogical metadata and can be used for audit, correction, or filtering.

## Next steps

- Add a PR that adds Prisma models to `model/prisma` or an examples schema in `logic/examples` demonstrating migrations.
- Add a small example transformer in `logic/examples` that reads SDSL metadata and performs the upsert/link sequence using Prisma.

This mapping provides a clear bridge between SDSL domain declarations, the database schema, and the `logic` runtime semantics for `particulars` and sublation tracing.
