import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Customer } from '@/data/schema/customer';
import { TableShape } from '@graphics/schema/table';
import { CustomerTable } from './customer';

/**
 * TABLE SYSTEM ARCHITECTURE
 * -------------------------
 * The table system follows our enhanced architecture pattern which
 * can be understood through a Kantian philosophical framework:
 *
 * ┌───────────────────────────┐       ┌───────────────────────────┐
 * │      SERVER SIDE          │       │       CLIENT SIDE         │
 * │    (The Noumenal)         │       │     (The Phenomenal)      │
 * │                           │       │                           │
 * │  "The thing in itself"    │       │  "The thing in appearance"│
 * │                           │       │                           │
 * │ - Raw data                │       │ - Visual representation   │
 * │ - Business logic          │       │ - User interactions       │
 * │ - Unobservable processes  │       │ - Perceptible elements    │
 * └─────────────┬─────────────┘       └─────────────┬─────────────┘
 *               │                                   │
 *               │           ADAPTER                 │
 *               └───────────────────────────────────┘
 *                       "The Transcendental"
 *
 * 1. SCHEMA FIRST
 *    - TableShape defines the structure using Zod
 *    - Creates type-safe, validated data structures
 *    - Represents the "pure concept" of a table
 *
 * 2. GENERIC BASE OBJECT
 *    - Table<T> abstract class provides the foundation
 *    - Handles common functionality and state
 *    - Represents the "categorical imperative" - consistent rules for all tables
 *
 * 3. EXTENDED OBJECT
 *    - CustomerTable implements domain-specific configurations
 *    - Adapts abstract concept to concrete domain
 *    - Bridges business domain with presentation
 *
 * 4. ADAPTER-RENDERER PATTERN
 *    - TableShapeAdapter (server): The noumenal - processes raw data out of sight
 *    - TableRenderer (client): The phenomenal - presents data in perceptible form
 *    - Adapter: The transcendental bridge connecting the two worlds
 */

// Mock the renderer component since we're testing the table logic, not the rendering
vi.mock('./renderer', () => ({
  TableRenderer: ({ shape, data }: { shape: TableShape, data: any[] }) => (
    <div data-testid="table-renderer" data-shape={JSON.stringify(shape)} data-items={JSON.stringify(data)}>
      {shape.layout.title}
    </div>
  )
}));

describe('CustomerTable', () => {
  const mockCustomers: Customer[] = [
    {
      id: '1',
      name: 'John Doe',
      email: 'john@example.com',
      imageUrl: '/images/customers/john.jpg',
      createdAt: new Date(),
      updatedAt: new Date()
    },
    {
      id: '2',
      name: 'Jane Smith',
      email: 'jane@example.com',
      imageUrl: '/images/customers/jane.jpg',
      createdAt: new Date(),
      updatedAt: new Date()
    }
  ];

  it('configures table with correct properties', () => {
    const table = new CustomerTable(mockCustomers);
    const { container } = render(<>{table.render()}</>);

    // Get the rendered component with shape data
    const rendererEl = screen.getByTestId('table-renderer');
    const shape = JSON.parse(rendererEl.getAttribute('data-shape') || '{}');

    // Verify the shape configuration
    expect(shape.layout.title).toBe('Customers');
    expect(shape.columns.length).toBeGreaterThan(0);
    expect(shape.columns[0].key).toBe('name');
  });

  it('transforms customer data correctly', () => {
    const table = new CustomerTable(mockCustomers);
    const { container } = render(<>{table.render()}</>);

    // Get the rendered items data
    const rendererEl = screen.getByTestId('table-renderer');
    const items = JSON.parse(rendererEl.getAttribute('data-items') || '[]');

    // Verify data transformation
    expect(items.length).toBe(2);
    expect(items[0].id).toBe('1');
    expect(items[0].name).toBe('John Doe');
  });

  it('includes action buttons in configuration', () => {
    const table = new CustomerTable(mockCustomers);
    const { container } = render(<>{table.render()}</>);

    // Get the rendered shape data
    const rendererEl = screen.getByTestId('table-renderer');
    const shape = JSON.parse(rendererEl.getAttribute('data-shape') || '{}');

    // Verify actions are present
    expect(shape.actions).toBeDefined();
    expect(shape.actions.length).toBeGreaterThan(0);
  });
});

/**
 * USAGE EXAMPLE
 * -------------
 * The CustomerTable can be used as follows:
 *
 * ```tsx
 * function CustomerListPage({ customers }: { customers: Customer[] }) {
 *   const customerTable = new CustomerTable(customers);
 *   return (
 *     <div className="container">
 *       {customerTable.render()}
 *     </div>
 *   );
 * }
 * ```
 */
