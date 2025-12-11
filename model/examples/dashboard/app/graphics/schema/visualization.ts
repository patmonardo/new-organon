//@graphics/schema/visualization.ts
import { z } from 'zod'
import { FormShapeSchema } from './form'

// Data representation
export const DataSchema = z.object({
  variables: z.array(z.object({
    name: z.string(),
    type: z.enum(['quantitative', 'ordinal', 'nominal', 'temporal']),
    format: z.string().optional()
  })),
  values: z.array(z.record(z.string(), z.unknown()))
})

// Visual encoding primitives
export const VisualSchema = z.object({
  // Spatial properties
  scale: z.object({
    type: z.enum(['linear', 'log', 'time', 'ordinal']),
    domain: z.array(z.unknown()),
    range: z.array(z.unknown()),
    padding: z.number().default(0.1)
  }),

  // Mark properties
  mark: z.object({
    type: z.enum(['bar', 'line', 'point', 'area']),
    color: z.string(),
    opacity: z.number().min(0).max(1),
    size: z.number()
  }),

  // Coordinate system
  coordinates: z.object({
    system: z.enum(['cartesian', 'polar']),
    orientation: z.enum(['vertical', 'horizontal'])
  }).default({
    system: 'cartesian',
    orientation: 'vertical'
  })
})

export type Data = z.infer<typeof DataSchema>
export type Visual = z.infer<typeof VisualSchema>
