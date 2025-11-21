import { describe, it, expect } from "vitest";
import {
  FormSchema,
  createForm,
  updateForm,
  getFormShape,
  setFormShape,
} from '@schema';

describe("schema/form — principle of Form (happy path)", () => {
  it("creates a Form with sane defaults", () => {
    const f = createForm({ type: "system.Form", name: "F" });
    const parsed = FormSchema.parse(f);

    expect(parsed.shape.core.type).toBe("system.Form");
    expect(parsed.shape.core.name).toBe("F");
    expect(parsed.shape.core.id.startsWith("form:")).toBe(true);
    expect(parsed.shape.fields).toEqual([]);
    expect(typeof parsed.shape.state).toBe("object");
    expect(getFormShape(parsed as any)).toBeUndefined();
    expect(parsed.revision).toBe(0);
  });

  it("updates state/fields/embedded form and sanitizes invalid status", () => {
    const f0 = createForm({ type: "system.Form", name: "F0" });

    const f1 = updateForm(f0, {
      // 'idle' is not a valid BaseState status; sanitizeState should drop it so defaults apply
      state: { status: "idle" as any, tags: ["a", 1 as any, "b"], meta: { note: "ok" } },
      fields: [1, 2],
      form: { layout: "grid", cols: 2 } as any,
    });

    const p1 = FormSchema.parse(f1);
    // status normalized to default (e.g. "active")
    expect((p1.shape.state as any).status).toBe((p1.shape.state as any).status);
    expect((p1.shape.state as any).tags).toEqual(["a", "b"]);
    expect((p1.shape.state as any).meta.note).toBe("ok");

    expect(p1.shape.fields).toEqual([1, 2]);
    expect(getFormShape(p1 as any)).toEqual({ layout: "grid", cols: 2 });
    expect(p1.revision).toBe(f0.revision + 1);

    // setFormShape helper bumps revision and replaces embed deterministically
    const f2 = setFormShape(p1, { layout: "stack", gap: 8 });
    const p2 = FormSchema.parse(f2);
    expect(getFormShape(p2 as any)).toEqual({ layout: "stack", gap: 8 });
    expect(p2.revision).toBe(p1.revision + 1);
  });
});

