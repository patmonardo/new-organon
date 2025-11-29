import React from 'react';
import { ReactView, ReactViewOptions } from '../../src/sdsl/react-view';
import type { FormMode, DisplayDocument, DisplayElement, FormShape } from '../../src/sdsl/types';
import { FormModel } from '../../src/sdsl/form-model';

export class CustomerView extends ReactView<FormShape> {

  constructor(
    model: FormModel<FormShape>,
    mode: FormMode = 'view',
    options: ReactViewOptions = {}
  ) {
    super(model, mode, options);
  }

  /**
   * Override render to add custom view logic
   */
  render(): DisplayDocument {
    const baseDoc = super.render();
    const customerName = this._model.getField('name');
    const invoiceCount = this.asNumber(this._model.getField('invoiceCount'));
    const totalRevenue = this.asNumber(this._model.getField('totalRevenue'));
    const averageInvoice = this.asNumber(this._model.getField('averageInvoice'));

    const metricElements: DisplayElement[] = [
      this.metricElement('invoiceCount', 'Invoices', invoiceCount),
      this.metricElement('totalRevenue', 'Total Revenue', totalRevenue, 'USD'),
      this.metricElement('averageInvoice', 'Avg Invoice', averageInvoice, 'USD'),
    ].filter(Boolean) as DisplayElement[];

    const invoicesElement: DisplayElement = {
      type: 'card',
      props: {
        title: 'Recent Invoices',
      },
      children: [
        {
          type: 'table',
          props: {
            collection: 'invoices',
            rows: this.getInvoices(),
            columns: [
              { id: 'id', label: 'Invoice' },
              { id: 'amount', label: 'Amount' },
              { id: 'status', label: 'Status' },
              { id: 'date', label: 'Date' },
            ],
            emptyLabel: 'No invoices yet',
          },
        },
      ],
    };

    const profileElement: DisplayElement = {
      type: 'card',
      props: {
        title: 'Customer Profile',
        subtitle: customerName ? String(customerName) : undefined,
      },
      children: baseDoc.layout.children,
    };

    const layoutChildren: DisplayElement[] = [];
    if (metricElements.length) {
      layoutChildren.push({
        type: 'grid',
        props: { columns: metricElements.length },
        children: metricElements,
      });
    }
    layoutChildren.push(invoicesElement, profileElement);

    return {
      title: baseDoc.title || (customerName ? `Customer: ${customerName}` : 'Customer'),
      layout: {
        type: 'stack',
        gap: 6,
        children: layoutChildren,
      },
      meta: {
        ...baseDoc.meta,
        view: 'customer-dashboard',
      },
    };
  }

  /**
   * Custom render method for a specific "Invoice List" partial view
   */
  renderInvoiceList(): React.ReactNode {
    const invoices = this._model.getField('invoices') as any[] || [];

    if (invoices.length === 0) {
      return <div className="p-4 text-gray-500">No invoices found.</div>;
    }

    return (
      <div className="invoice-list">
        <h3>Recent Invoices</h3>
        <ul>
          {invoices.map((inv: any) => (
            <li key={inv.id}>
              {inv.date} - {inv.amount} cents - {inv.status}
            </li>
          ))}
        </ul>
      </div>
    );
  }

  private metricElement(metric: string, label: string, value: number, unit?: string): DisplayElement | null {
    if (!Number.isFinite(value)) {
      return null;
    }
    return {
      type: 'metric',
      props: {
        metric,
        label,
        value,
        unit,
      },
    };
  }

  private getInvoices(): Array<Record<string, unknown>> {
    const invoices = this._model.getField('invoices');
    return Array.isArray(invoices) ? invoices : [];
  }

  private asNumber(value: unknown): number {
    if (typeof value === 'number') {
      return value;
    }
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : NaN;
  }
}
