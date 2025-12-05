import React from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { CustomerController } from './customer-controller';
import { CustomerModel } from './customer-model';
import { CustomerDataService } from './customer-data.service';
import { PolarsExecutionEngine } from '../../src/data/polars-engine';
import { SqlEngine } from '../../src/data/sql-engine';

export async function runDemo() {
  console.log('=== MVC RUNTIME DEMO ===\n');

  // 1. Initialize Controller (starts in 'create' mode)
  console.log('1. Initializing Controller...');
  const controller = new CustomerController('view');

  // 2. Hydrate from Semantic Layer
  console.log('2. Loading customer from semantic model...');
  const snapshot = await controller.loadCustomerProfile('cust_100');
  if (snapshot) {
    const row = snapshot.rows[0];
    console.log(`   Loaded ${row?.name} (${row?.email})`);
    console.log('   Metrics:', snapshot.metrics);
    console.log('   Plan:', snapshot.plan);
  } else {
    console.log('   Customer not found in semantic layer, continuing with empty model.');
  }

  // 3. Render View with hydrated data via Radix adapter
  console.log('\n3. Rendering hydrated view via Radix adapter...');
  const { document: doc, element: radixTree } = controller.renderRadixDashboard();

  console.log('\n--- Display Document ---');
  console.log(`Title: ${doc.title}`);
  console.log('Layout:', JSON.stringify(doc.layout, null, 2));

  const staticMarkup = renderToStaticMarkup(React.createElement(React.Fragment, null, radixTree));
  console.log('\n--- Radix Adapter Markup (static snapshot) ---');
  console.log(staticMarkup);

  // 4. Business Logic (Get Invoices)
  console.log('\n4. Fetching Invoices...');
  try {
    const customerId = controller.model.getField('id') as string;
    const invoices = await controller.getInvoices(customerId);
    console.log(`   Invoices for ${customerId}:`, invoices);
  } catch (e: any) {
    console.error('   Error:', e.message);
  }

  // 5. Switch to Edit Mode and submit a change to prove form flow still works
  console.log('\n5. Switching to Edit Mode and submitting updated data...');
  controller.setMode('edit');
  const updatedCustomerData = {
    id: controller.model.getField('id'),
    name: 'Acme Industries (Updated)',
    email: 'ops@acme.com',
    imageUrl: 'https://images.ctfassets.net/example/acme-avatar.png',
    invoices: (snapshot?.collections?.invoices as Record<string, unknown>[] | undefined) || [],
  };

  try {
    const result = await controller.executeAction('submit', updatedCustomerData);
    console.log('   Submit Result:', result);
  } catch (e: any) {
    console.error('   Error submitting update:', e.message);
  }

  console.log('\n=== DEMO COMPLETE ===');

  await runDataSdslDemo();
}

export default runDemo;

runDemo().catch(console.error);

async function runDataSdslDemo() {
  console.log('\n=== DATA SDSL DEMO ===');

  const regionalRevenueView = CustomerModel.view({
    group_by: ['region'],
    aggregate: ['totalRevenue', 'count'],
    limit: 5,
  });

  const sqlEngine = new SqlEngine();
  const sqlQuery = sqlEngine.toSelect(regionalRevenueView);

  console.log('\nGenerated SQL (stub):');
  console.log(sqlQuery.text);
  console.log('Params:', sqlQuery.params);

  const dataService = new CustomerDataService();
  const dataset = dataService.getDatasetSnapshot();
  const polarsEngine = new PolarsExecutionEngine(dataset);
  const result = await polarsEngine.execute(regionalRevenueView);

  console.log('\nPolars execution plan (stub):');
  console.log(result.plan);
  console.log('Meta:', result.meta);
}
