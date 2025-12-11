//@schema/customer.ts
import { z } from 'zod'
import { BaseSchema, BaseStateSchema } from './base'

export const CustomerSchema = BaseSchema.extend({
  name: z.string(),
  email: z.string().email(),
  imageUrl: z.string().nullish().or(z.literal("")),
})

// State Schema - Runtime state and validation errors
// Extends BaseStateSchema for status, validation, message
export const CustomerStateSchema = BaseStateSchema.extend({
  errors: z.object({
    name: z.array(z.string()).optional(),
    email: z.array(z.string()).optional(),
    imageUrl: z.array(z.string()).optional()
  }).optional()
})

// Shape Schema - Aggregates data and state
export const CustomerShapeSchema = z.object({
  base: CustomerSchema,
  state: CustomerStateSchema
})

export const CreateCustomerSchema = CustomerSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true
})

export const UpdateCustomerSchema = CustomerSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true
}).partial()

export type Customer = z.infer<typeof CustomerSchema>
export type CustomerState = z.infer<typeof CustomerStateSchema>
export type CustomerShape = z.infer<typeof CustomerShapeSchema>
export type CreateCustomer = z.infer<typeof CreateCustomerSchema>
export type UpdateCustomer = z.infer<typeof UpdateCustomerSchema>
