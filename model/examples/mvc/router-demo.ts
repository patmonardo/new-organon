import { ControllerRegistry } from './registry';
import { CustomerController } from './customer-controller';
import UniversalPage from './universal-page';

// 1. Register Routes
console.log('Registering /customer -> CustomerController');
ControllerRegistry.register('/customer', CustomerController);

// 2. Simulate Request
async function demo() {
  console.log('\n--- Simulating Request to /customer ---');

  // Simulate Next.js Page Props
  const props = {
    params: { slug: ['customer'] },
    searchParams: {}
  };

  try {
    const result = await UniversalPage(props);

    console.log('UniversalPage returned successfully.');

    // In a real environment, 'result' would be a React Element tree.
    // Since we are running in Node/TSX without a full React renderer,
    // we can inspect the structure if needed, or just trust that it returned.

    // Let's try to inspect it a bit if possible, or just log success.
    if (result) {
      console.log('Result type:', typeof result);
      // @ts-ignore - inspecting internal props for demo
      if (result.props) {
         // @ts-ignore
        console.log('Root element type:', result.type);
      }
    }

  } catch (e: any) {
    console.error('Error rendering page:', e);
  }
}

demo().catch(console.error);
