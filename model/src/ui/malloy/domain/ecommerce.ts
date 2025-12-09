import { defineModel, dimension, sum, count, avg, DataModel } from '../../../data/sdsl';
import { PolarsDataset } from '../../../execution/polars-engine';

// =============================================================================
// MOCK DATA
// =============================================================================

export const ecommerceMockData: PolarsDataset = {
  customers: [
    { id: 'c1', name: 'Acme Corp', email: 'contact@acme.com', region: 'NA', createdAt: '2023-01-10', imageUrl: 'https://ui-avatars.com/api/?name=Acme+Corp' },
    { id: 'c2', name: 'Globex', email: 'info@globex.com', region: 'EU', createdAt: '2023-02-15', imageUrl: 'https://ui-avatars.com/api/?name=Globex' },
    { id: 'c3', name: 'Soylent', email: 'sales@soylent.com', region: 'NA', createdAt: '2023-03-20', imageUrl: 'https://ui-avatars.com/api/?name=Soylent' },
    { id: 'c4', name: 'Umbrella', email: 'secure@umbrella.com', region: 'AS', createdAt: '2023-01-05', imageUrl: 'https://ui-avatars.com/api/?name=Umbrella' },
    { id: 'c5', name: 'Stark Ind', email: 'tony@stark.com', region: 'NA', createdAt: '2023-04-12', imageUrl: 'https://ui-avatars.com/api/?name=Stark+Ind' },
  ],
  invoices: [
    { id: 'i101', customerId: 'c1', amount: 5000, status: 'paid', date: '2023-01-15' },
    { id: 'i102', customerId: 'c1', amount: 2500, status: 'pending', date: '2023-02-01' },
    { id: 'i103', customerId: 'c2', amount: 10000, status: 'paid', date: '2023-02-20' },
    { id: 'i104', customerId: 'c2', amount: 800, status: 'void', date: '2023-02-25' },
    { id: 'i105', customerId: 'c3', amount: 1200, status: 'paid', date: '2023-03-22' },
    { id: 'i106', customerId: 'c4', amount: 45000, status: 'paid', date: '2023-01-08' },
    { id: 'i107', customerId: 'c5', amount: 100000, status: 'draft', date: '2023-04-15' },
  ],
};

// =============================================================================
// DOMAIN MODELS
// =============================================================================

/**
 * Customers Model
 * Represents the base customer entity.
 */
export const CustomerModel = defineModel({
  name: 'customers',
  source: 'ecommerce.customers',
  dimensions: {
    id: dimension('id'),
    name: dimension('name'),
    email: dimension('email'),
    region: dimension('region'),
    joined_at: dimension('createdAt', 'day'),
  },
  measures: {
    count: count(),
  },
});

/**
 * Invoices Model
 * Represents invoice transactions.
 */
export const InvoiceModel = defineModel({
  name: 'invoices',
  source: 'ecommerce.invoices',
  dimensions: {
    id: dimension('id'),
    customer_id: dimension('customerId'),
    status: dimension('status'),
    date: dimension('date', 'day'),
  },
  measures: {
    total_amount: sum('amount'),
    avg_amount: avg('amount'),
    count: count(),
  },
  joins: {
    // In a real SDSL, we'd define the relationship here
    // customer: { model: CustomerModel, on: 'customerId = customer.id', type: 'left' }
  }
});

/**
 * Customer360 Model
 * A unified view of Customers enriched with Invoice metrics.
 * This simulates a 'source' that has already joined data.
 */
export const Customer360Model = defineModel({
  name: 'customer_360',
  source: 'ecommerce.customers_with_metrics', // Joined view
  dimensions: {
    customer_name: dimension('name'),
    region: dimension('region'),
    email: dimension('email'),
  },
  measures: {
    lifetime_value: { type: 'sum', field: 'totalRevenue', label: 'LTV' },
    invoice_count: { type: 'sum', field: 'invoiceCount', label: 'Orders' }, // projected metric
    avg_invoice: { type: 'avg', field: 'averageInvoice', label: 'Avg Order' },
  }
});
