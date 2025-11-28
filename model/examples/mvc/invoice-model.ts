import { z } from 'zod';
import { defineModel, sum, count, avg, dimension } from '../../src/data/sdsl';
import { InvoiceStatusSchema } from './customer';

export const InvoiceModel = defineModel({
  name: 'Invoice',
  source: 'invoices',
  fields: {
    id: z.string(),
    customerId: z.string(),
    amount: z.number(),
    status: InvoiceStatusSchema,
    date: z.string(), // ISO date string for now
  },
  measures: {
    count: count(),
    totalAmount: sum('amount'),
    averageAmount: avg('amount'),
  },
  dimensions: {
    status: 'status',
    date: dimension('date', 'day'),
    month: dimension('date', 'month'),
    customerId: 'customerId',
  },
});
