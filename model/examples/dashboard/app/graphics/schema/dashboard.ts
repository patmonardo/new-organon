import { z } from 'zod'
import { FormShapeSchema } from './form'

// Dashboard field identifiers
const DashboardFieldId = z.enum(['title', 'layout', 'widgets'])

// Dashboard field types
const DashboardFieldType = z.enum(['text', 'select', 'widget-list'])

// Dashboard-specific form fields
export const DashboardFieldsShape = z.object({
  elements: z.array(
    z.object({
      id: DashboardFieldId,
      type: DashboardFieldType,
      label: z.string(),
      required: z.literal(true),
      defaultValue: z.string().optional()
    })
  )
})

// The complete dashboard form shape
export const DashboardFormShape = FormShapeSchema.extend({
  layout: z.object({
    title: z.string(),
    columns: z.literal('single'),
    sections: z.array(z.object({
      title: z.string(),
      fields: z.array(DashboardFieldId)
    }))
  }),
  fields: DashboardFieldsShape
})

export type DashboardFormShape = z.infer<typeof DashboardFormShape>
