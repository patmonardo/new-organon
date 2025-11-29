import React from 'react';
import { describe, expect, it } from 'vitest';
import { renderToStaticMarkup } from 'react-dom/server';
import { CustomerController } from '../examples/customer/customer-controller';

describe('CustomerController + Radix adapter integration', () => {
  it('hydrates semantic data and renders Radix dashboard', async () => {
    const controller = new CustomerController('view');

    const snapshot = await controller.loadCustomerProfile('cust_100');
    expect(snapshot).toBeTruthy();
    expect(snapshot?.metrics?.invoiceCount).toBeGreaterThan(0);
    expect(Array.isArray(snapshot?.collections?.invoices)).toBe(true);

    const { document, element } = controller.renderRadixDashboard();
    expect(document.meta?.view).toBe('customer-dashboard');
    expect(document.layout.children.length).toBeGreaterThanOrEqual(2);

    const markup = renderToStaticMarkup(<>{element}</>);
    expect(markup).toContain('Recent Invoices');
    expect(markup).toContain('Customer Profile');
  });
});
