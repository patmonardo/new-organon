//@schema/revenue.ts
import { z } from 'zod'
import { BaseSchema, BaseStateSchema } from './base'

// Time intervals for reporting (kept here as it's domain knowledge)
export const TimeIntervalSchema = z.enum(['day', 'week', 'month', 'quarter', 'year'])

// Base Revenue Schema for database records
export const RevenueSchema = BaseSchema.extend({
  id: z.string().uuid(),
  month: z.date(),
  revenue: z.number().positive(),
  expenses: z.number().positive().optional().default(0)
})

// State Schema for runtime state and validation
export const RevenueStateSchema = BaseStateSchema.extend({
  errors: z.object({
    month: z.array(z.string()).optional(),
    revenue: z.array(z.string()).optional(),
    expenses: z.array(z.string()).optional()
  }).optional()
})

// Shape Schema - Aggregates data and state
export const RevenueShapeSchema = z.object({
  base: RevenueSchema,
  state: RevenueStateSchema
})

export const CreateRevenueSchema = RevenueSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true
})

export const UpdateRevenueSchema = RevenueSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true
}).partial()

// Type for revenue metrics
export const RevenueMetricsSchema = z.object({
  month: z.date(),
  revenue: z.number(),
  expenses: z.number(),
  profit: z.number(),
});

// Type exports
export type Revenue = z.infer<typeof RevenueSchema>
export type RevenueState = z.infer<typeof RevenueStateSchema>
export type RevenueShape = z.infer<typeof RevenueShapeSchema>
export type CreateRevenue = z.infer<typeof CreateRevenueSchema>
export type UpdateRevenue = z.infer<typeof UpdateRevenueSchema>
export type TimeInterval = z.infer<typeof TimeIntervalSchema>
export type RevenueMetrics = z.infer<typeof RevenueMetricsSchema>
