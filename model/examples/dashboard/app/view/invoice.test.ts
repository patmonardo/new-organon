import { describe, it, expect } from 'vitest'
import type { FormHandler } from '@/ui/graphics/schema/form'
import { InvoiceView } from './invoice'

describe('InvoiceView', () => {
  it('should render a create form', () => {
    const invoiceView = new InvoiceView();

    // Add console.log to see what's happening
    console.log('About to render invoice view');

    const result = invoiceView.render('create', 'jsx', {} as FormHandler);

  });
});
