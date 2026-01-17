import type { PrismaClient } from '@prisma/client';
import { EntityShapeSchema } from '@schema';

/**
 * Illustrative helper: create an Entity and link particulars.
 * Adapt this to your Prisma models (Person, Particular, PersonParticular).
 */
export async function createEntityWithParticulars(
  prisma: PrismaClient,
  entityShape: unknown,
) {
  const validated = EntityShapeSchema.parse(entityShape);

  // Upsert particulars (simple strategy: create if missing)
  const particularIds = await Promise.all(
    (validated.particulars || []).map(async (p: any) => {
      const up = await prisma.particular.upsert({
        where: { id: p.id },
        create: { id: p.id, type: p.type, data: {} },
        update: {},
      });
      return { id: up.id, type: up.type, role: p.role };
    }),
  );

  // Create entity (example uses `person` table — adapt model name)
  const ent = await prisma.person.create({
    data: { id: validated.id, name: (validated as any).name || '' },
  });

  // Link via join table
  await Promise.all(
    particularIds.map((pid) =>
      prisma.personParticular.create({
        data: { personId: ent.id, particularId: pid.id, role: pid.role },
      }),
    ),
  );

  // Optionally update facets/shape in a repo column
  await prisma.person.update({
    where: { id: ent.id },
    data: {
      /* facets: JSON update if present */
    } as any,
  });

  return ent;
}
