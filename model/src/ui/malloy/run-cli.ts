import { PolarsExecutionEngine } from '../../execution/polars-engine';
import { PolarsDataService } from '../../execution/polars-data-service';
import { Customer360Model, ecommerceMockData } from './domain/ecommerce';

async function main() {
  console.log('---------------------------------------------------------');
  console.log('malloy-cli: Starting "Customer/Invoice" Data Flow Trace');
  console.log('---------------------------------------------------------');

  // 1. Initialize "Server" Components
  console.log('[1] Initializing Engine with Mock Data...');
  const engine = new PolarsExecutionEngine(ecommerceMockData);
  const service = new PolarsDataService(engine);
  console.log('    -> Engine Ready (In-Memory Polars)');
  console.log('    -> Service Ready (Aquifer Adapter)');

  // 2. Construct Query (Simulating UI State)
  console.log('\n[2] Constructing Query from UI State...');
  const query = {
    group_by: ['customer_name', 'region'],
    aggregate: ['lifetime_value', 'invoice_count'],
    order_by: [{ field: 'lifetime_value', direction: 'desc' as const }],
    limit: 5,
  };
  console.log('    -> Query:', JSON.stringify(query, null, 2));

  // 3. Prepare View (SDSL)
  console.log('\n[3] Preparing SDSL View...');
  const view = Customer360Model.view(query);
  const plan = view.toPlan();
  console.log('    -> Generated Execution Plan:');
  console.log(plan);

  // 4. Execute Service
  console.log('\n[4] Executing via SemanticDataService...');
  const startTime = Date.now();
  const result = await service.execute(view);
  const duration = Date.now() - startTime;
  console.log(`    -> Execution Complete in ${duration}ms`);

  // 5. Display Results
  console.log('\n[5] Results (Top 5):');
  console.table(result.rows);

  if (result.meta) {
      console.log('\n[6] Meta Trace:');
      console.log(JSON.stringify(result.meta, null, 2));
  }

  console.log('\n---------------------------------------------------------');
  console.log('Trace Complete.');
  console.log('---------------------------------------------------------');
}

main().catch(err => {
  console.error('Fatal Error:', err);
  process.exit(1);
});
