import { z } from 'zod';
import {
  FormShape,
  FormField,
  FormOption,
  FormSection
} from '../../src/schema/shape';

// =============================================================================
// DOMAIN TYPES (The "Real" Data)
// =============================================================================

export const InvoiceStatusSchema = z.enum(['PENDING', 'PAID', 'OVERDUE', 'DRAFT']);
export type InvoiceStatus = z.infer<typeof InvoiceStatusSchema>;

export const InvoiceSchema = z.object({
  id: z.string(),
  customerId: z.string(),
  amount: z.number(), // In cents
  status: InvoiceStatusSchema,
  date: z.string(), // ISO date
});
export type Invoice = z.infer<typeof InvoiceSchema>;

export const CustomerSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string().email(),
  imageUrl: z.string().optional(),
  region: z.string().optional(),
  createdAt: z.string().optional(),
  invoices: z.array(InvoiceSchema).optional(),
});
export type Customer = z.infer<typeof CustomerSchema>;

// =============================================================================
// SDSL SHAPES (The "Form" Definition)
// =============================================================================

// Invoice Status Options
const statusOptions: FormOption[] = [
  { value: 'PENDING', label: 'Pending' },
  { value: 'PAID', label: 'Paid' },
  { value: 'OVERDUE', label: 'Overdue' },
  { value: 'DRAFT', label: 'Draft' },
];

// Invoice Form Shape
export const InvoiceShape: FormShape = {
  id: 'invoice-form',
  name: 'Invoice',
  title: 'Invoice Details',
  fields: [
    {
      id: 'id',
      name: 'id',
      type: 'text',
      label: 'Invoice ID',
      required: true,
      readOnly: true,
      visible: false,
    },
    {
      id: 'customerId',
      name: 'customerId',
      type: 'text',
      label: 'Customer ID',
      required: true,
      visible: false,
    },
    {
      id: 'amount',
      name: 'amount',
      type: 'number',
      label: 'Amount (cents)',
      required: true,
      validation: {
        min: 0,
        message: 'Amount must be positive',
      },
    },
    {
      id: 'status',
      name: 'status',
      type: 'select',
      label: 'Status',
      required: true,
      options: statusOptions,
      validation: {
        required: true,
      }
    },
    {
      id: 'date',
      name: 'date',
      type: 'date',
      label: 'Date',
      required: true,
    },
  ],
};

// Customer Form Shape
export const CustomerShape: FormShape = {
  id: 'customer-form',
  name: 'Customer',
  title: 'Customer Profile',
  description: 'Manage customer details and invoices',
  fields: [
    {
      id: 'id',
      name: 'id',
      type: 'text',
      label: 'Customer ID',
      required: true,
      readOnly: true,
      visible: false, // Hidden ID field
    },
    {
      id: 'name',
      name: 'name',
      type: 'text',
      label: 'Full Name',
      placeholder: 'Enter customer name',
      required: true,
      validation: {
        required: true,
        minLength: 2,
        message: 'Name must be at least 2 characters',
      },
    },
    {
      id: 'email',
      name: 'email',
      type: 'email',
      label: 'Email Address',
      placeholder: 'customer@example.com',
      required: true,
      validation: {
        required: true,
        pattern: '^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$',
        message: 'Please enter a valid email address',
      },
    },
    {
      id: 'imageUrl',
      name: 'imageUrl',
      type: 'url',
      label: 'Profile Image URL',
      placeholder: 'https://...',
      required: false,
    },
    // Hidden fields populated by the semantic layer bridge
    {
      id: 'invoiceCount',
      name: 'invoiceCount',
      type: 'number',
      label: 'Invoice Count',
      readOnly: true,
      visible: false,
    },
    {
      id: 'totalRevenue',
      name: 'totalRevenue',
      type: 'number',
      label: 'Total Revenue (cents)',
      readOnly: true,
      visible: false,
    },
    {
      id: 'averageInvoice',
      name: 'averageInvoice',
      type: 'number',
      label: 'Average Invoice (cents)',
      readOnly: true,
      visible: false,
    },
    {
      id: 'invoices',
      name: 'invoices',
      type: 'json',
      label: 'Invoices',
      readOnly: true,
      visible: false,
    },
    {
      id: 'semanticPlan',
      name: 'semanticPlan',
      type: 'text',
      label: 'Semantic Plan',
      readOnly: true,
      visible: false,
    },
  ],
  layout: {
    id: 'customer-layout',
    sections: [
      {
        id: 'basic-info',
        title: 'Basic Information',
        fields: ['name', 'email', 'imageUrl'],
      },
    ],
    actions: [
      { id: 'submit', type: 'submit', label: 'Save Customer', primary: true },
      { id: 'cancel', type: 'button', label: 'Cancel' },
    ],
  },
};
