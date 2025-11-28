import React from 'react';
import { ReactView, ReactViewOptions } from '../../src/sdsl/react-view';
import { FormMode, DisplayDocument } from '../../src/schema/shape';
import { FormModel } from '../../src/sdsl/form-model';
import { CustomerShape } from './customer';

export class CustomerView extends ReactView<typeof CustomerShape> {

  constructor(
    model: FormModel<typeof CustomerShape>,
    mode: FormMode = 'view',
    options: ReactViewOptions = {}
  ) {
    super(model, mode, options);
  }

  /**
   * Override render to add custom view logic
   */
  render(): DisplayDocument {
    const doc = super.render();

    // Example: Add a custom header or summary in 'view' mode
    if (this._mode === 'view') {
      const customerName = this._model.getField('name');
      doc.title = `Customer: ${customerName}`;

      // We could inject extra display elements here if we wanted
      // doc.layout.children.unshift({ ... })
    }

    return doc;
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
}
