import { describe, it, expect } from "vitest";
import { createCustomerList } from './customer';
import { Customer } from '@/data/schema/customer';

describe('createCustomerList', () => {
  it('should create a properly structured list from customer data', () => {
    // Mock customer data
    const mockCustomers: Customer[] = [
      {
        id: '1',
        createdAt: new Date(),
        updatedAt: new Date(),
        name: 'John Doe',
        email: 'john@example.com',
        imageUrl: '/customers/john.png'
      },
      {
        id: '2',
        createdAt: new Date(),
        updatedAt: new Date(),
        name: 'Jane Smith',
        email: 'jane@example.com',
        imageUrl: '/customers/jane.png'
      }
    ];

    // Call the function
    const result = createCustomerList(mockCustomers);

    // Assertions
    expect(result).toBeDefined();
    expect(result.items.length).toBe(2);

    // Check first item mapping
    expect(result.items[0].id).toBe('1');
    expect(result.items[0].content).toEqual(mockCustomers[0]);

    // Check relations for first item
    expect(result.items[0].relations).toHaveLength(2);
    expect(result.items[0].relations?.[0]).toEqual({
      label: 'Edit',
      href: '/customers/1/edit',
      relation: 'action',
      icon: 'pencil'
    });
    expect(result.items[0].relations?.[1]).toEqual({
      label: 'Delete',
      href: '/customers/1/delete',
      relation: 'action',
      icon: 'trash'
    });

    // Check layout configuration
    expect(result.layout).toEqual({
      type: 'linear',
      spacing: 'normal'
    });

    // Check navigation options
    expect(result.navigation).toEqual({
      search: true,
      pagination: true,
      creation: true
    });

    // Check global relations
    expect(result.relations).toHaveLength(1);
    expect(result.relations?.[0]).toEqual({
      label: 'Add Customer',
      href: '/customers/create',
      relation: 'action',
      icon: 'plus'
    });
  });

  it('should handle empty customer list', () => {
    const result = createCustomerList([]);

    expect(result.items).toEqual([]);
    expect(result.layout).toBeDefined();
    expect(result.navigation).toBeDefined();
    expect(result.relations).toBeDefined();
  });
});
