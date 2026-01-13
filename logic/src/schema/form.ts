import { z } from 'zod';
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';
import { FormDialecticSchema } from './rules';

// ==========================================
// FORM SHAPE DEFINITIONS
// ==========================================

export const FormDataSchema = z
  .object({
    source: z
      .object({
        type: z.enum([
          'entity',
          'context',
          'api',
          'function',
          'localStorage',
          'composite',
        ]),
        entityRef: z
          .object({
            entity: z.string(),
            id: z.string(),
          })
          .optional(),
        contextRef: z
          .object({
            entityRef: z.object({
              entity: z.string(),
              id: z.string(),
            }),
            type: z.string(),
          })
          .optional(),
        apiConfig: z
          .object({
            endpoint: z.string(),
            method: z.enum(['GET', 'POST', 'PUT', 'DELETE', 'PATCH']),
            headers: z.record(z.string(), z.string()).optional(),
            params: z.record(z.string(), z.any()).optional(),
          })
          .optional(),
        functionRef: z
          .object({
            name: z.string(),
            args: z.array(z.any()).optional(),
          })
          .optional(),
        localStorageKey: z.string().optional(),
        compositeSources: z.array(z.any()).optional(),
      })
      .optional(),
    access: z
      .object({
        read: z
          .object({
            path: z.string().optional(),
            transform: z.function().optional(),
            default: z.any().optional(),
            cache: z.boolean().optional().default(false),
          })
          .optional(),
        write: z
          .object({
            path: z.string().optional(),
            transform: z.function().optional(),
            merge: z.boolean().optional().default(true),
            validation: z.function().optional(),
          })
          .optional(),
        subscribe: z
          .object({
            path: z.string().optional(),
            debounce: z.number().optional(),
            throttle: z.number().optional(),
          })
          .optional(),
      })
      .optional(),
    schema: z
      .object({
        type: z.enum(['zod', 'json-schema', 'typescript', 'custom']).optional(),
        definition: z.any().optional(),
      })
      .optional(),
    hooks: z
      .object({
        beforeLoad: z.function().optional(),
        afterLoad: z.function().optional(),
        beforeSubmit: z.function().optional(),
        afterSubmit: z.function().optional(),
        onValidate: z.function().optional(),
      })
      .optional(),
    meta: z
      .object({
        createdAt: z.date().optional(),
        updatedAt: z.date().optional(),
        version: z.string().optional(),
        owner: z.string().optional(),
        permissions: z.array(z.string()).optional(),
      })
      .optional(),
  })
  .catchall(z.any()) // Allow dialectical and other custom fields
  .optional();

export const FormModeSchema = z
  .enum(['create', 'edit', 'view'])
  .default('create');

export const FormContentSchema = z
  .enum(['jsx', 'html', 'json', 'xml'])
  .default('jsx');

export const FormTagSchema = z.object({
  value: z.any(),
  label: z.string(),
});

export const FormOptionSchema = z.object({
  value: z.any(),
  label: z.string(),
});

export const FormFieldValidationSchema = z.object({
  required: z.boolean().optional(),
  min: z.number().optional(),
  max: z.number().optional(),
  minLength: z.number().optional(),
  maxLength: z.number().optional(),
  pattern: z.string().optional(),
  custom: z.function().optional(),
  message: z.string().optional(),
});

export const FormFieldMetaSchema = z.object({
  sectionHint: z.string().optional(),
  validation: z
    .object({
      performed: z.boolean().optional(),
      timestamp: z.number().optional(),
      level: z.string().optional(),
    })
    .optional(),
  accessibility: z
    .object({
      enhanced: z.boolean().optional(),
      level: z.string().optional(),
      guideline: z.string().optional(),
    })
    .optional(),
  localization: z
    .object({
      applied: z.boolean().optional(),
      locale: z.string().optional(),
    })
    .optional(),
});

export const FormFieldSchema = z.object({
  id: z.string(),
  type: z.string().optional(),
  name: z.string().optional(),
  title: z.string().optional(),
  description: z.string().optional(),
  label: z.string().optional(),
  placeholder: z.string().optional(),
  required: z.boolean().optional().default(false).optional(),
  disabled: z.boolean().optional().default(false).optional(),
  createOnly: z.boolean().optional(),
  editOnly: z.boolean().optional(),
  readOnly: z.boolean().optional().default(false).optional(),
  visible: z.boolean().optional().default(true).optional(),
  validation: FormFieldValidationSchema.optional(),
  options: z.array(FormOptionSchema).optional(),
  inputType: z.string().optional(),
  format: z.string().optional(),
  meta: FormFieldMetaSchema.optional(),
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});

export const FormSectionSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  title: z.string().optional(),
  description: z.string().optional(),
  fields: z.array(z.string()).optional(),
  columns: z.number().optional().default(1).optional(),
  priority: z.number().optional().default(1).optional(),
  collapsible: z.boolean().optional().default(false).optional(),
  collapsed: z.boolean().optional().default(false).optional(),
  className: z.string().optional(),
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});

export const FormHandlerSchema = z.object({
  submit: z.function(),
  reset: z.function().optional(),
  cancel: z.function().optional(),
  delete: z.function().optional(),
});

export const FormActionSchema = z.object({
  id: z.string(),
  type: z.enum(['submit', 'reset', 'button']).optional(),
  label: z.string().optional(),
  primary: z.boolean().optional().default(false).optional(),
  disabled: z.boolean().optional().default(false).optional(),
  position: z
    .enum(['top', 'bottom', 'both'])
    .optional()
    .default('bottom')
    .optional(),
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});

export const FormLayoutSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  title: z.string().optional(),
  description: z.string().optional(),
  columns: z.enum(['single', 'double']).optional(),
  sections: z.array(FormSectionSchema).optional(),
  actions: z.array(FormActionSchema).optional(),
  responsive: z
    .object({
      sectionBreakpoints: z
        .record(z.string(), z.enum(['stack', 'grid', 'tabs']))
        .optional(),
      fieldArrangement: z
        .enum(['natural', 'importance', 'groupRelated'])
        .optional(),
    })
    .optional(),
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});

export const FormStateSchema = z.object({
  status: z.enum(['idle', 'submitting', 'success', 'error']),
  errors: z.record(z.string(), z.array(z.string())).optional(),
  message: z.string().optional(),
});

export const FormMetaSchema = z.object({
  validation: z
    .object({
      performed: z.boolean().optional(),
      timestamp: z.number().optional(),
      fieldErrors: z.number().optional(),
    })
    .optional(),
  layout: z
    .object({
      source: z.string().optional(),
      timestamp: z.number().optional(),
      generated: z.boolean().optional(),
    })
    .optional(),
  accessibility: z
    .object({
      enhanced: z.boolean().optional(),
      timestamp: z.number().optional(),
      level: z.string().optional(),
    })
    .optional(),
  localization: z
    .object({
      applied: z.boolean().optional(),
      locale: z.string().optional(),
      timestamp: z.number().optional(),
    })
    .optional(),
});

export const FormShapeSchema = z.object({
  id: z.string(),
  name: z.string(),
  title: z.string().optional(),
  description: z.string().optional(),
  schemaId: z.string().optional(),
  data: FormDataSchema.optional(), // Data binding configuration (how to get/set data)
  fields: z.array(FormFieldSchema),
  options: z.array(FormOptionSchema).optional(),
  tags: z.array(FormTagSchema).optional(),
  dialectic: FormDialecticSchema.optional(),
  isValid: z.boolean().optional(),
  layout: FormLayoutSchema.optional(),
  // state and meta removed - those belong in Entity (Empirical)
  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});

// ==========================================
// CORE FORM DOCUMENT (Envelope)
// ==========================================

// Core for Form docs (theory-bearing)
export const FormCore = BaseCore.extend({
  type: Type, // e.g., "system.Form"
  name: Label.optional(),
});
export type FormCore = z.infer<typeof FormCore>;

// Form document's shape
const FormDocShape = z.object({
  core: FormCore,
  state: BaseState.default({}), // BaseState applies defaults (status/tags/meta)
  fields: z.array(z.any()).default([]), // future: refine to field definitions
  form: FormShapeSchema.optional(), // EMBED THE FORM SHAPE HERE
});

// Canonical Form schema
export const FormSchema = BaseSchema.extend({
  shape: FormDocShape,
});
export type Form = z.infer<typeof FormSchema>;

// Type exports
export type FormData = z.infer<typeof FormDataSchema>;
export type FormMode = z.infer<typeof FormModeSchema>;
export type FormContent = z.infer<typeof FormContentSchema>;
export type FormFieldValidation = z.infer<typeof FormFieldValidationSchema>;
export type FormOption = z.infer<typeof FormOptionSchema>;
export type FormTag = z.infer<typeof FormTagSchema>;
export type FormField = z.infer<typeof FormFieldSchema>;
export type FormHandler = z.infer<typeof FormHandlerSchema>;
export type FormAction = z.infer<typeof FormActionSchema>;
export type FormFieldMeta = z.infer<typeof FormFieldMetaSchema>;
export type FormSection = z.infer<typeof FormSectionSchema>;
export type FormLayout = z.infer<typeof FormLayoutSchema>;
export type FormState = z.infer<typeof FormStateSchema>;
export type FormMeta = z.infer<typeof FormMetaSchema>;
export type FormShapeRepo = z.infer<typeof FormShapeSchema>;
