
import React from 'react';
import { CustomerController } from './customer-controller';
import { renderToStaticMarkup } from 'react-dom/server';

async function runDemo() {
  console.log('--- Starting Customer Demo ---');

  // 1. Initialize Controller
  console.log('1. Initializing Controller...');
  const controller = new CustomerController('view');

  // 2. Load Data (Hydration)
  console.log('2. Loading Customer Profile (cust_100)...');
  const snapshot = await controller.loadCustomerProfile('cust_100');

  if (!snapshot) {
    console.error('Failed to load customer profile.');
    return;
  }

  console.log('   Loaded Snapshot:', {
    rows: snapshot.rows.length,
    metrics: snapshot.metrics,
  });

  // 3. Render Dashboard
  console.log('3. Rendering Radix Dashboard...');
  const { element, document } = controller.renderRadixDashboard();

  // 4. Output Structure
  console.log('   Document Title:', document.title);
  console.log('   Layout Type:', document.layout.type);

  // 5. Simulate React Rendering (Server-Side)
  console.log('4. Generating Static Markup...');
  // Note: key props might cause hydration mismatches in real SSR, but fine for static demo
  const html = renderToStaticMarkup(element as React.ReactElement);

  // Pretty print a snippet of the HTML
  console.log('   Output HTML (Snippet):');
  console.log(html.slice(0, 500) + '...');

  console.log('--- Demo Complete ---');
}

runDemo().catch(console.error);
