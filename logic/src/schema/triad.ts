import { z } from "zod";
import { Id, Label, Type, Timestamps, stamp, touch } from "./base";

// UPS roles (Universal–Particular–Singular)
export const Axis = z.enum(["universal", "particular", "singular"]);
export type Axis = z.infer<typeof Axis>;

// Reference to a schema-level entity (pure form)
export const TriadRef = z.object({
  id: Id,
  type: Type,            // e.g., "system.Entity" | "form.Shape" | domain type
  label: Label.optional()
});
export type TriadRef = z.infer<typeof TriadRef>;

// Canonical Triad (recursive triple, not a pair)
export const Triad = z
  .object({
    id: Id,
    name: Label,
    description: z.string().optional(),

    universal: TriadRef,
    particular: TriadRef,
    singular: TriadRef,

    tags: z.array(Label).default([])
  })
  .and(Timestamps)
  .superRefine((t, ctx) => {
    const ids = [t.universal.id, t.particular.id, t.singular.id];
    if (new Set(ids).size !== 3) {
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message: "Triad members (universal, particular, singular) must reference distinct ids"
      });
    }
  });

export type Triad = z.infer<typeof Triad>;

// Helpers (schema-as-canon: preserve timestamps/invariants)
export function createTriad(input: z.input<typeof Triad>): Triad {
  return Triad.parse(stamp({ tags: [], ...(input as any) }));
}

export function updateTriad(current: Triad, patch: Partial<z.input<typeof Triad>>): Triad {
  return Triad.parse(touch({ ...current, ...(patch as any) }));
}

// Small ergonomic helpers
export function replaceSingular(current: Triad, singular: z.input<typeof TriadRef>): Triad {
  return updateTriad(current, { singular: TriadRef.parse(singular) });
}

export function toUPS<T extends Triad>(t: T) {
  return {
    universal: t.universal,
    particular: t.particular,
    singular: t.singular
  };
}
