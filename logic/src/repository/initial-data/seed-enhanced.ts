import { defaultConnection } from '../neo4j-client';
import { FormShapeRepository } from '../form';
import { EntityShapeRepository } from '../entity';
import type { FormShapeRepo as FormShape } from '@schema/form';
import type { EntityShapeRepo as EntityShape } from '@schema/entity';
import { v4 as uuidv4 } from 'uuid';
import neo4j from 'neo4j-driver';

/**
 * Helper to convert Neo4j Integer to number
 */
function toNumber(value: any): number {
  if (value === null || value === undefined) return 0;
  if (neo4j.isInt(value)) {
    return value.toNumber();
  }
  return Number(value) || 0;
}

/**
 * Enhanced seed script that creates a rich dataset
 *
 * This demonstrates:
 * - Form creation via FormShapeRepository
 * - Entity creation via EntityShapeRepository
 * - Relationships between entities
 * - Real-world data patterns
 */
export async function seedEnhanced() {
  console.log('🌱 Starting Enhanced FormDB Seed...');

  // Ensure connection
  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error('❌ Failed to connect to Neo4j. Exiting.');
    process.exit(1);
  }

  // Create repositories
  const formRepo = new FormShapeRepository(defaultConnection);
  const entityRepo = new EntityShapeRepository(defaultConnection);

  const session = defaultConnection.getSession();
  try {
    // === 0. Wipe Database ===
    console.log('🧹 Wiping database...');
    const wipeResult = await session.run(
      'MATCH (n) DETACH DELETE n RETURN count(n) as deleted',
    );
    const deletedCount = toNumber(wipeResult.records[0]?.get('deleted'));
    console.log(`   Deleted ${deletedCount} nodes`);

    // === 1. Create Forms ===
    console.log('\n📋 Creating Forms...');

    // Invoice Form
    const invoiceFormId = 'invoice-form';
    const invoiceForm: FormShape = {
      id: invoiceFormId,
      name: 'InvoiceForm',
      title: 'Invoice',
      description: 'Manage invoices and billing',
      fields: [
        {
          id: 'customerId',
          name: 'customerId',
          label: 'Customer',
          type: 'select',
          required: true,
          options: [],
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
        {
          id: 'notes',
          name: 'notes',
          label: 'Notes',
          type: 'textarea',
          meta: { sectionHint: 'Additional Info' },
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
          {
            id: uuidv4(),
            title: 'Additional Info',
            fields: ['notes'],
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
      tags: [
        { value: 'billing', label: 'Billing' },
        { value: 'financial', label: 'Financial' },
      ],
    };

    await formRepo.saveForm(invoiceForm);
    console.log(`   ✅ Created Invoice Form (${invoiceFormId})`);

    // Customer Form
    const customerFormId = 'customer-form';
    const customerForm: FormShape = {
      id: customerFormId,
      name: 'CustomerForm',
      title: 'Customer',
      description: 'Manage customer information',
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
          id: 'phone',
          name: 'phone',
          label: 'Phone',
          type: 'text',
        },
        {
          id: 'imageUrl',
          name: 'imageUrl',
          label: 'Image URL',
          type: 'text',
        },
        {
          id: 'company',
          name: 'company',
          label: 'Company',
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
            fields: ['name', 'email', 'phone'],
          },
          {
            id: uuidv4(),
            title: 'Additional',
            fields: ['company', 'imageUrl'],
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
      tags: [
        { value: 'contact', label: 'Contact' },
        { value: 'crm', label: 'CRM' },
      ],
    };

    await formRepo.saveForm(customerForm);
    console.log(`   ✅ Created Customer Form (${customerFormId})`);

    // Product Form
    const productFormId = 'product-form';
    const productForm: FormShape = {
      id: productFormId,
      name: 'ProductForm',
      title: 'Product',
      description: 'Manage products and inventory',
      fields: [
        {
          id: 'name',
          name: 'name',
          label: 'Product Name',
          type: 'text',
          required: true,
        },
        {
          id: 'description',
          name: 'description',
          label: 'Description',
          type: 'textarea',
        },
        {
          id: 'price',
          name: 'price',
          label: 'Price ($)',
          type: 'number',
          required: true,
        },
        {
          id: 'sku',
          name: 'sku',
          label: 'SKU',
          type: 'text',
          required: true,
        },
        {
          id: 'stock',
          name: 'stock',
          label: 'Stock Quantity',
          type: 'number',
          required: true,
        },
        {
          id: 'category',
          name: 'category',
          label: 'Category',
          type: 'select',
          options: [
            { value: 'electronics', label: 'Electronics' },
            { value: 'clothing', label: 'Clothing' },
            { value: 'books', label: 'Books' },
            { value: 'food', label: 'Food' },
          ],
        },
      ],
      layout: {
        id: uuidv4(),
        title: 'Product Information',
        columns: 'single',
        sections: [
          {
            id: uuidv4(),
            title: 'Basic Info',
            fields: ['name', 'description', 'sku'],
          },
          {
            id: uuidv4(),
            title: 'Pricing & Inventory',
            fields: ['price', 'stock', 'category'],
          },
        ],
        actions: [
          {
            id: 'submit',
            type: 'submit',
            label: 'Save Product',
            primary: true,
          },
        ],
      },
      tags: [
        { value: 'inventory', label: 'Inventory' },
        { value: 'catalog', label: 'Catalog' },
      ],
    };

    await formRepo.saveForm(productForm);
    console.log(`   ✅ Created Product Form (${productFormId})`);

    // === 2. Create Entities ===
    console.log('\n👥 Creating Entities...');

    // Create Customers
    const customers = [
      {
        id: 'customer-1',
        name: 'Acme Corporation',
        email: 'contact@acme.com',
        phone: '+1-555-0101',
        company: 'Acme Corp',
      },
      {
        id: 'customer-2',
        name: 'TechStart Inc',
        email: 'hello@techstart.io',
        phone: '+1-555-0102',
        company: 'TechStart',
      },
      {
        id: 'customer-3',
        name: 'Global Solutions',
        email: 'info@globalsolutions.com',
        phone: '+1-555-0103',
        company: 'Global Solutions LLC',
      },
    ];

    for (const customerData of customers) {
      const entity: EntityShape = {
        id: customerData.id,
        type: 'system.Entity',
        name: customerData.name,
        formId: customerFormId,
        values: {}, // Values stored in Model/Postgres, not FormDB
        tags: ['customer'],
      };
      await entityRepo.saveEntity(entity);
      console.log(`   ✅ Created Customer: ${customerData.name}`);
    }

    // Create Products
    const products = [
      {
        id: 'product-1',
        name: 'Laptop Pro 15',
        sku: 'LAP-001',
        price: 1299,
        stock: 45,
        category: 'electronics',
      },
      {
        id: 'product-2',
        name: 'Wireless Headphones',
        sku: 'AUD-002',
        price: 199,
        stock: 120,
        category: 'electronics',
      },
      {
        id: 'product-3',
        name: 'Design Patterns Book',
        sku: 'BOK-003',
        price: 49,
        stock: 200,
        category: 'books',
      },
      {
        id: 'product-4',
        name: 'Cotton T-Shirt',
        sku: 'CLO-004',
        price: 29,
        stock: 300,
        category: 'clothing',
      },
    ];

    for (const productData of products) {
      const entity: EntityShape = {
        id: productData.id,
        type: 'system.Entity',
        name: productData.name,
        formId: productFormId,
        values: {},
        tags: ['product', productData.category],
      };
      await entityRepo.saveEntity(entity);
      console.log(`   ✅ Created Product: ${productData.name}`);
    }

    // Create Invoices
    const invoices = [
      {
        id: 'invoice-1',
        customerId: 'customer-1',
        amount: 2500,
        status: 'PAID',
        date: new Date('2024-01-15'),
      },
      {
        id: 'invoice-2',
        customerId: 'customer-1',
        amount: 1800,
        status: 'PENDING',
        date: new Date('2024-02-01'),
      },
      {
        id: 'invoice-3',
        customerId: 'customer-2',
        amount: 3200,
        status: 'PAID',
        date: new Date('2024-01-20'),
      },
      {
        id: 'invoice-4',
        customerId: 'customer-3',
        amount: 950,
        status: 'OVERDUE',
        date: new Date('2023-12-10'),
      },
    ];

    for (const invoiceData of invoices) {
      const entity: EntityShape = {
        id: invoiceData.id,
        type: 'system.Entity',
        name: `Invoice ${invoiceData.id}`,
        formId: invoiceFormId,
        values: {},
        status: invoiceData.status,
        tags: ['invoice', invoiceData.status.toLowerCase()],
      };
      await entityRepo.saveEntity(entity);
      console.log(
        `   ✅ Created Invoice: ${invoiceData.id} (${invoiceData.status})`,
      );
    }

    // === 3. Create Relationships ===
    console.log('\n🔗 Creating Relationships...');

    // Link invoices to customers
    for (const invoice of invoices) {
      await session.run(
        `
        MATCH (inv:Entity {id: $invoiceId})
        MATCH (cust:Entity {id: $customerId})
        MERGE (inv)-[:BELONGS_TO]->(cust)
        RETURN inv.id as invoiceId, cust.id as customerId
        `,
        { invoiceId: invoice.id, customerId: invoice.customerId },
      );
      console.log(`   ✅ Linked ${invoice.id} → ${invoice.customerId}`);
    }

    // === 4. Summary ===
    console.log('\n📊 Seed Summary:');
    const summary = await session.run(`
      MATCH (n)
      RETURN labels(n)[0] as label, count(n) as count
      ORDER BY count DESC
    `);

    for (const record of summary.records) {
      const label = record.get('label');
      const count = toNumber(record.get('count'));
      console.log(`   ${label}: ${count} nodes`);
    }

    const relSummary = await session.run(`
      MATCH ()-[r]->()
      RETURN type(r) as type, count(r) as count
      ORDER BY count DESC
    `);

    if (relSummary.records.length > 0) {
      console.log('\n   Relationships:');
      for (const record of relSummary.records) {
        const type = record.get('type');
        const count = toNumber(record.get('count'));
        console.log(`   ${type}: ${count} relationships`);
      }
    }

    console.log('\n✨ Enhanced seed completed successfully!');
  } catch (error) {
    console.error('❌ Error during enhanced seed:', error);
    throw error;
  } finally {
    await session.close();
    await defaultConnection.close();
  }
}
