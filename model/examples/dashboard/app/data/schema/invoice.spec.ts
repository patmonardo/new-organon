import { describe, it, expect } from 'vitest';
import { v4 as uuidv4 } from 'uuid';
import { InvoiceSchema } from './invoice';

describe('Invoice Schema', () => {
  it('should validate a valid invoice with enum status', () => {
    const validInvoice = {
      id: uuidv4(),
      customerId: uuidv4(), // Provide a valid UUID
      amount: 100,
      status: 'PENDING',
      date: new Date(),
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = InvoiceSchema.safeParse(validInvoice);
    expect(result.success).toBe(true);
  });

  it('should invalidate an invoice with invalid enum status', () => {
    const invalidInvoice = {
      id: uuidv4(),
      customerId: uuidv4(),
      amount: 100,
      status: 'INVALID',
      date: new Date(),
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = InvoiceSchema.safeParse(invalidInvoice);
    expect(result.success).toBe(false);
  });

  it('should invalidate an invoice with negative amount', () => {
    const invalidInvoice = {
      id: uuidv4(),
      customerId: uuidv4(),
      amount: -100,
      status: 'PENDING',
      date: new Date(),
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = InvoiceSchema.safeParse(invalidInvoice);
    expect(result.success).toBe(false);
  });

  it('should invalidate an invoice with invalid customerId', () => {
    const invalidInvoice = {
      id: uuidv4(),
      customerId: 10, // Provide an invalid customerId
      amount: 100,
      status: 'PENDING',
      date: new Date(),
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = InvoiceSchema.safeParse(invalidInvoice);
    expect(result.success).toBe(false);
  });
});
