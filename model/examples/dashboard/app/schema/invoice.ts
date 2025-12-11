//@schema/invoice.ts
import { z } from 'zod';
import { BaseSchema, BaseStateSchema } from './base';
import { CustomerSchema } from './customer';

// Define status enum
export const InvoiceStatusSchema = z.enum(["PENDING", "PAID", "OVERDUE", "DRAFT"]);

// Core Invoice Schema
export const InvoiceSchema = BaseSchema.extend({
  customerId: z.string().uuid(),
  amount: z.number().positive(),
  date: z.date(),
  status: InvoiceStatusSchema,
});

// Join with Customer for display purposes
export const InvoiceWithCustomerSchema = InvoiceSchema.extend({
  customer: CustomerSchema
});

// State Schema - Runtime state and validation errors
export const InvoiceStateSchema = BaseStateSchema.extend({
  errors: z.object({
    customerId: z.array(z.string()).optional(),
    amount: z.array(z.string()).optional(),
    date: z.array(z.string()).optional(),
    status: z.array(z.string()).optional()
  }).optional()
});

// Shape Schema - Aggregates data and state
export const InvoiceShapeSchema = z.object({
  base: InvoiceSchema,
  state: InvoiceStateSchema
});

// For data entry/updates
export const CreateInvoiceSchema = InvoiceSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
});

export const UpdateInvoiceSchema = InvoiceSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
}).partial();

// Type exports
export type Invoice = z.infer<typeof InvoiceSchema>;
export type InvoiceWithCustomer = z.infer<typeof InvoiceWithCustomerSchema>;
export type InvoiceState = z.infer<typeof InvoiceStateSchema>;
export type InvoiceShape = z.infer<typeof InvoiceShapeSchema>;
export type CreateInvoice = z.infer<typeof CreateInvoiceSchema>;
export type UpdateInvoice = z.infer<typeof UpdateInvoiceSchema>;
export type InvoiceStatus = z.infer<typeof InvoiceStatusSchema>;
