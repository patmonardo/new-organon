import { z } from 'zod';
import { defineModel, sum, avg, count, dimension } from '../../src/data/sdsl';
import { InvoiceModel } from './invoice-model';

export const CustomerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  fields: {
    id: z.string(),
    name: z.string(),
    email: z.string().email(),
    region: z.string().optional(),
    createdAt: z.string(), // ISO date string for signup
  },
  measures: {
    count: count(),
    totalRevenue: sum('invoices.amount'),
    averageInvoice: avg('invoices.amount'),
  },
  dimensions: {
    region: 'region',
    signupMonth: dimension('createdAt', 'month'),
  },
  joins: {
    invoices: {
      model: InvoiceModel,
      on: 'customers.id = invoices.customerId',
      type: 'left',
    },
  },
});
