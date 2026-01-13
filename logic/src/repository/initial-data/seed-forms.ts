import { defaultConnection } from '../neo4j-client';
import { FormShapeRepository } from '../form';
import type { FormShapeRepo as FormShape } from '@schema/form';
import { v4 as uuidv4 } from 'uuid';

export async function seedForms() {
  console.log('Starting FormDB Seed...');

  // Ensure connection
  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error('Failed to connect to Neo4j. Exiting.');
    process.exit(1);
  }

  const repo = new FormShapeRepository(defaultConnection);

  const session = defaultConnection.getSession();
  try {
    // === 0. Wipe Database (as requested) ===
    console.log('Wiping database...');
    await session.run('MATCH (n) DETACH DELETE n');
    console.log('Database wiped.');

    // === 1. Invoice Form ===
    const invoiceFormId = 'invoice-form';
    const invoiceForm: FormShape = {
      id: invoiceFormId,
      name: 'InvoiceForm',
      title: 'Invoice',
      description: 'Manage invoices',
      fields: [
        {
          id: 'customerId',
          name: 'customerId',
          label: 'Customer',
          type: 'select',
          required: true,
          options: [], // Options will be populated dynamically by the View/Controller
          meta: { sectionHint: 'Invoice Details' },
        },
        {
          id: 'amount',
          name: 'amount',
          label: 'Amount ($)',
          title: 'Amount',
          type: 'number',
          required: true,
          meta: { sectionHint: 'Invoice Details' },
        },
        {
          id: 'status',
          name: 'status',
          label: 'Status',
          type: 'select',
          required: true,
          options: [
            { value: 'PAID', label: 'Paid' },
            { value: 'PENDING', label: 'Pending' },
            { value: 'OVERDUE', label: 'Overdue' },
            { value: 'DRAFT', label: 'Draft' },
          ],
          meta: { sectionHint: 'Invoice Details' },
        },
        {
          id: 'date',
          name: 'date',
          label: 'Date',
          type: 'date',
          required: true,
          meta: { sectionHint: 'Invoice Details' },
        },
      ],
      layout: {
        id: uuidv4(),
        title: 'Invoice Details',
        columns: 'single',
        sections: [
          {
            id: uuidv4(),
            title: 'Invoice Details',
            fields: ['customerId', 'amount', 'status', 'date'],
            columns: 1,
          },
        ],
        actions: [
          {
            id: 'submit',
            type: 'submit',
            label: 'Save Invoice',
            primary: true,
            position: 'bottom',
          },
          {
            id: 'cancel',
            type: 'button',
            label: 'Cancel',
            position: 'bottom',
          },
        ],
      },
    };

    console.log(`Seeding Invoice Form (${invoiceFormId})...`);
    await repo.saveForm(invoiceForm);
    console.log('Invoice Form seeded successfully.');

    // === 2. Customer Form ===
    const customerFormId = 'customer-form';
    const customerForm: FormShape = {
      id: customerFormId,
      name: 'CustomerForm',
      title: 'Customer',
      description: 'Manage customers',
      fields: [
        {
          id: 'name',
          name: 'name',
          label: 'Name',
          type: 'text',
          required: true,
        },
        {
          id: 'email',
          name: 'email',
          label: 'Email',
          type: 'email',
          required: true,
        },
        {
          id: 'imageUrl',
          name: 'imageUrl',
          label: 'Image URL',
          type: 'text',
        },
      ],
      layout: {
        id: uuidv4(),
        title: 'Customer Information',
        columns: 'single',
        sections: [
          {
            id: uuidv4(),
            title: 'Basic Info',
            fields: ['name', 'email', 'imageUrl'],
          },
        ],
        actions: [
          {
            id: 'submit',
            type: 'submit',
            label: 'Save Customer',
            primary: true,
          },
          {
            id: 'cancel',
            type: 'button',
            label: 'Cancel',
          },
        ],
      },
    };

    console.log(`Seeding Customer Form (${customerFormId})...`);
    await repo.saveForm(customerForm);
    console.log('Customer Form seeded successfully.');
  } catch (error) {
    console.error('Error seeding forms:', error);
  } finally {
    await defaultConnection.close();
  }
}
