//@graphics/schema/revenue.ts
import { z } from "zod";
import { TimeIntervalSchema } from "@/data/schema/revenue";
import { FormOptionSchema, FormActionSchema, FormShapeSchema } from "@graphics/schema/form";

// Revenue field identifiers
export const RevenueFieldId = z.enum(["month", "revenue", "expenses"]);

// Revenue field types
export const RevenueFieldType = z.enum(["date", "number", "currency"]);

// Revenue-specific form field
export const RevenueFieldShapeSchema = z.object({
  id: RevenueFieldId,
  type: RevenueFieldType,
  label: z.string(),
  required: z.boolean().default(false),
  defaultValue: z.string().optional(),
  options: z.array(FormOptionSchema).optional(),
});

// Revenue form schema
export const RevenueFormShapeSchema = FormShapeSchema.extend({
  layout: z.object({
    title: z.string(),
    columns: z.enum(["single", "double"]),
    sections: z.array(
      z.object({
        title: z.string(),
        fieldIds: z.array(RevenueFieldId),
      })
    ).optional(),
    actions: z.array(FormActionSchema).default([]),
  }),
  fields: z.array(RevenueFieldShapeSchema),
});

// Revenue data points for charts and reporting (moved from core schema)
export const RevenueDataPointDisplaySchema = z.object({
  date: z.string(),
  amount: z.number(),
});

// Revenue chart data structure (moved from core schema)
export const RevenueChartDisplaySchema = z.object({
  interval: TimeIntervalSchema,
  data: z.array(RevenueDataPointDisplaySchema),
  totalRevenue: z.number(),
  previousPeriodRevenue: z.number().optional(),
  growthRate: z.number().optional(),
});

// Export types
export type RevenueFieldShape = z.infer<typeof RevenueFieldShapeSchema>;
export type RevenueFormShape = z.infer<typeof RevenueFormShapeSchema>;
export type RevenueDataPointDisplay = z.infer<typeof RevenueDataPointDisplaySchema>;
export type RevenueChartDisplay = z.infer<typeof RevenueChartDisplaySchema>;
