//@view/customer.test.ts
import { describe, it, expect } from 'vitest'
import { FormHandler } from '@graphics/schema/form'
import { CustomerView } from './customer'

describe('CustomerView', () => {
  it('should render an edit form', () => {
    const customerView = new CustomerView();

    // Add console.log to see what's happening
    console.log('About to render customer view');

    const result = customerView.render('create', 'jsx', {} as FormHandler);

  });
  it('should render a create form', () => {
    const customerView = new CustomerView();

    // Add console.log to see what's happening
    console.log('About to render customer view');

    const result = customerView.render('create', 'jsx', {} as FormHandler);
  });
})
