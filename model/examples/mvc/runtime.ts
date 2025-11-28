import { CustomerController } from './customer-controller';
import { CustomerView } from './customer-view';

async function main() {
  console.log('=== MVC RUNTIME DEMO ===\n');

  // 1. Initialize Controller (starts in 'create' mode)
  console.log('1. Initializing Controller...');
  const controller = new CustomerController('create');

  // 2. Simulate User Input (filling the form)
  console.log('2. User fills form...');
  const newCustomerData = {
    name: 'Alice Corp',
    email: 'alice@example.com',
    imageUrl: 'https://example.com/alice.png'
  };

  // 3. Submit Form
  console.log('3. Submitting form...');
  try {
    const result = await controller.executeAction('submit', newCustomerData);
    console.log('   Result:', result);
  } catch (e: any) {
    console.error('   Error:', e.message);
  }

  // 4. Switch to View Mode
  console.log('\n4. Switching to View Mode...');
  controller.setMode('view');

  // 5. Render View
  console.log('5. Rendering View...');
  const view = new CustomerView(controller.model, 'view');
  const doc = view.render();

  console.log('\n--- Display Document ---');
  console.log(`Title: ${doc.title}`);
  console.log('Layout:', JSON.stringify(doc.layout, null, 2));

  // 6. Business Logic (Get Invoices)
  console.log('\n6. Fetching Invoices...');
  try {
    const customerId = controller.model.getField('id') as string;
    const invoices = await controller.getInvoices(customerId);
    console.log(`   Invoices for ${customerId}:`, invoices);
  } catch (e: any) {
    console.error('   Error:', e.message);
  }

  console.log('\n=== DEMO COMPLETE ===');
}

main().catch(console.error);
