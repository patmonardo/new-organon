import { z } from 'zod';
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';

// Core for Form docs (theory-bearing)
export const FormCore = BaseCore.extend({
  type: Type, // e.g., "system.Form"
  name: Label.optional(),
});
export type FormCore = z.infer<typeof FormCore>;

// Embedded realized shape (accept any object; detailed validation lives in FormShape)
const FormShapeEmbed = z.object({}).catchall(z.any());
type FormShapeEmbed = z.infer<typeof FormShapeEmbed>;

// Form document's shape
const FormShapeDoc = z.object({
  core: FormCore,
  state: BaseState.default({}), // BaseState applies defaults (status/tags/meta)
  fields: z.array(z.any()).default([]), // future: refine to field definitions
  form: FormShapeEmbed.optional(),
});

// Canonical Form schema
export const FormSchema = BaseSchema.extend({
  shape: FormShapeDoc,
});
export type Form = z.infer<typeof FormSchema>;

// Helpers
function genId() {
  return `form:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

// Accept UI-ish states but normalize to BaseState's enum
const VALID_BASE_STATUSES = new Set(['active', 'archived', 'deleted']);
function sanitizeState(input?: unknown): z.input<typeof BaseState> {
  if (!input || typeof input !== 'object' || Array.isArray(input)) return {};
  const i = input as Record<string, unknown>;
  const out: Record<string, unknown> = {};

  // Keep only valid enum; drop unknowns like 'idle' so BaseState default applies.
  if (typeof i.status === 'string' && VALID_BASE_STATUSES.has(i.status)) {
    out.status = i.status;
  }

  if (Array.isArray(i.tags)) {
    out.tags = i.tags.filter((t) => typeof t === 'string');
  }
  if (i.meta && typeof i.meta === 'object' && !Array.isArray(i.meta)) {
    out.meta = i.meta;
  }
  return out as z.input<typeof BaseState>;
}

type CreateFormInput = {
  id?: string;
  type: string;
  name?: string;
  fields?: unknown[];
  state?: z.input<typeof BaseState>;
  form?: FormShapeEmbed;
};

// Create a Form doc with sane defaults
export function createForm(input: CreateFormInput): Form {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: sanitizeState(input.state),
      fields: input.fields ?? [],
      form: input.form,
    },
  };
  return FormSchema.parse(draft);
}

type FormCoreOut = z.output<typeof FormCore>;
type BaseStateOut = z.output<typeof BaseState>;

type UpdateFormPatch = Partial<{
  core: Partial<FormCoreOut>;
  state: Partial<BaseStateOut>;
  fields: unknown[];
  form: FormShapeEmbed;
}>;

// Update with shallow merges where appropriate; increments revision
export function updateForm(doc: Form, patch: UpdateFormPatch): Form {
  const prevCore = doc.shape.core as FormCoreOut;
  const prevState = doc.shape.state as BaseStateOut;

  // Merge then sanitize state so invalid statuses (e.g., 'idle') are dropped,
  // allowing BaseState defaults to apply during parsing.
  const mergedState = { ...prevState, ...(patch.state ?? {}) };
  const cleanedState = sanitizeState(mergedState);

  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...prevCore, ...(patch.core ?? {}) },
      state: cleanedState,
      fields: patch.fields ?? doc.shape.fields,
      form: patch.form ?? doc.shape.form,
    },
    revision: (doc.revision ?? 0) + 1,
  };
  return FormSchema.parse(next);
}

// Ergonomics for embedded realized shape
export function getFormShape(doc: Form): Record<string, unknown> | undefined {
  return doc.shape.form as Record<string, unknown> | undefined;
}

export function setFormShape(doc: Form, form: Record<string, unknown>): Form {
  return updateForm(doc, { form: form as FormShapeEmbed });
}
