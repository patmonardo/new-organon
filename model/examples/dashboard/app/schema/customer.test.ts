import { describe, it, expect } from 'vitest';
import { v4 as uuidv4 } from "uuid";
import { CustomerSchema } from './customer'; // Import your Customer schema

describe('Customer Schema', () => {
  it('should validate a valid customer', () => {
    const validCustomer = {
      id: uuidv4(),
      name: 'John Doe',
      email: 'john.doe@example.com',
      imageUrl: null,
      invoices: [],
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = CustomerSchema.safeParse(validCustomer);
    expect(result.success).toBe(true);
  });

  it('should invalidate a customer with invalid email', () => {
    const invalidCustomer = {
      id: uuidv4(),
      name: 'John Doe',
      email: 'invalid-email',
      imageUrl: null,
      invoices: [],
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = CustomerSchema.safeParse(invalidCustomer);
    expect(result.success).toBe(false);
  });

  it('should invalidate a customer with missing name', () => {
    const invalidCustomer = {
      id: uuidv4(),
      email: 'john.doe@example.com',
      imageUrl: null,
      invoices: [],
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = CustomerSchema.safeParse(invalidCustomer);
    expect(result.success).toBe(false);
  });

  it('should validate a customer with null imageUrl', () => {
    const validCustomer = {
      id: uuidv4(),
      name: 'John Doe',
      email: 'john.doe@example.com',
      imageUrl: null,
      invoices: [],
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    const result = CustomerSchema.safeParse(validCustomer);
    expect(result.success).toBe(true);
  });
});
