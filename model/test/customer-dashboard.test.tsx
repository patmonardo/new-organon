/**
 * CustomerDashboard Test
 *
 * Tests the CustomerDashboard Radix component renders correctly
 * with hydrated semantic data.
 */

import React from 'react';
import { describe, expect, it } from 'vitest';
import { renderToStaticMarkup } from 'react-dom/server';
import { CustomerDashboard } from '../examples/customer/customer-dashboard';
import { CustomerController } from '../examples/customer/customer-controller';
import type { Customer, Invoice } from '../examples/customer/customer';

describe('CustomerDashboard', () => {
  const mockCustomer: Customer = {
    id: 'cust_test',
    name: 'Test Corporation',
    email: 'test@example.com',
    imageUrl: 'https://example.com/avatar.png',
    region: 'west',
  };

  const mockInvoices: Invoice[] = [
    { id: 'inv_1', customerId: 'cust_test', amount: 100000, status: 'PAID', date: '2024-01-15' },
    { id: 'inv_2', customerId: 'cust_test', amount: 50000, status: 'PENDING', date: '2024-02-01' },
    { id: 'inv_3', customerId: 'cust_test', amount: 25000, status: 'OVERDUE', date: '2024-03-10' },
  ];

  const mockMetrics = {
    invoiceCount: 3,
    totalRevenue: 175000,
    averageInvoice: 58333,
  };

  it('renders customer profile header', () => {
    const markup = renderToStaticMarkup(
      <CustomerDashboard
        customer={mockCustomer}
        invoices={mockInvoices}
        metrics={mockMetrics}
      />
    );

    expect(markup).toContain('Test Corporation');
    expect(markup).toContain('test@example.com');
    expect(markup).toContain('Customer Dashboard');
  });

  it('renders metric cards', () => {
    const markup = renderToStaticMarkup(
      <CustomerDashboard
        customer={mockCustomer}
        invoices={mockInvoices}
        metrics={mockMetrics}
      />
    );

    expect(markup).toContain('Total Invoices');
    expect(markup).toContain('Total Revenue');
    expect(markup).toContain('Average Invoice');
    // Check formatted currency
    expect(markup).toContain('$1,750'); // $1,750.00 total revenue
  });

  it('renders invoices table', () => {
    const markup = renderToStaticMarkup(
      <CustomerDashboard
        customer={mockCustomer}
        invoices={mockInvoices}
        metrics={mockMetrics}
      />
    );

    expect(markup).toContain('Recent Invoices');
    expect(markup).toContain('inv_1');
    expect(markup).toContain('inv_2');
    expect(markup).toContain('inv_3');
    expect(markup).toContain('PAID');
    expect(markup).toContain('PENDING');
    expect(markup).toContain('OVERDUE');
  });

  it('renders empty state when no invoices', () => {
    const markup = renderToStaticMarkup(
      <CustomerDashboard
        customer={mockCustomer}
        invoices={[]}
        metrics={{ invoiceCount: 0, totalRevenue: 0, averageInvoice: 0 }}
      />
    );

    expect(markup).toContain('No invoices yet');
  });

  it('renders region badge', () => {
    const markup = renderToStaticMarkup(
      <CustomerDashboard
        customer={mockCustomer}
        invoices={mockInvoices}
        metrics={mockMetrics}
      />
    );

    expect(markup).toContain('WEST');
  });
});

describe('CustomerDashboard integration with Controller', () => {
  it('hydrates from semantic layer and renders dashboard', async () => {
    const controller = new CustomerController('view');

    // Load customer profile via semantic hydration
    const snapshot = await controller.loadCustomerProfile('cust_100');
    expect(snapshot).toBeTruthy();

    // Extract data from controller
    const model = controller.model;
    const customer: Customer = {
      id: model.getField('id') as string,
      name: model.getField('name') as string,
      email: model.getField('email') as string,
      imageUrl: model.getField('imageUrl') as string | undefined,
    };

    const invoices = (snapshot?.collections?.invoices ?? []) as Invoice[];
    const metrics = {
      invoiceCount: snapshot?.metrics?.invoiceCount as number ?? 0,
      totalRevenue: snapshot?.metrics?.totalRevenue as number ?? 0,
      averageInvoice: snapshot?.metrics?.averageInvoice as number ?? 0,
    };

    // Render dashboard
    const markup = renderToStaticMarkup(
      <CustomerDashboard
        customer={customer}
        invoices={invoices}
        metrics={metrics}
        snapshot={snapshot ?? undefined}
      />
    );

    // Verify hydrated data appears
    expect(markup).toContain('Acme Industries');
    expect(markup).toContain('ops@acme.com');
    expect(markup).toContain('inv_1001');
    expect(markup).toContain('Total Revenue');
  });
});

