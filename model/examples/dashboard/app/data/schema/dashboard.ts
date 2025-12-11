//@schema/dashboard.ts
import { z } from 'zod'

// Widget types we support
export const WidgetType = z.enum([
  'revenue-chart',
  'customer-table',
  'invoice-list',
  'metrics-card'
])

// Individual widget configuration
export const WidgetSchema = z.object({
  id: z.string(),
  type: WidgetType,
  title: z.string(),
  position: z.object({
    x: z.number(),
    y: z.number(),
    w: z.number(),
    h: z.number()
  }),
  config: z.record(z.string(), z.unknown()).optional()
})

// Layout configuration
export const LayoutSchema = z.object({
  type: z.enum(['grid', 'fixed', 'auto']),
  columns: z.number().default(12),
  gap: z.number().default(4)
})

// The complete dashboard schema
export const DashboardSchema = z.object({
  id: z.uuid(),
  title: z.string().min(1, 'Title is required'),
  description: z.string().optional(),
  layout: LayoutSchema,
  widgets: z.array(WidgetSchema),
  createdAt: z.date(),
  updatedAt: z.date(),
  userId: z.string()
})

export const CreateDashboardSchema = DashboardSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true
})

export type Widget = z.infer<typeof WidgetSchema>
export type Layout = z.infer<typeof LayoutSchema>
export type Dashboard = z.infer<typeof DashboardSchema>
export type CreateDashboard = z.infer<typeof CreateDashboardSchema>
