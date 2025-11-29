import { describe, expect, it } from 'vitest';
import { CustomerModel } from '../examples/customer/customer-model';
import { CustomerDataService } from '../examples/customer/customer-data.service';
import { PolarsExecutionEngine } from '../src/data/polars-engine';

describe('PolarsExecutionEngine', () => {
  it('aggregates invoice metrics and collections for a customer profile', async () => {
    const dataService = new CustomerDataService();
    const dataset = dataService.getDatasetSnapshot();
    const engine = new PolarsExecutionEngine(dataset);
    const view = CustomerModel.view({
      filter: { id: 'cust_100' },
      aggregate: ['totalRevenue', 'averageInvoice', 'count'],
      limit: 1,
    });

    const result = await engine.execute(view);
    expect(result.rows).toHaveLength(1);
    const row = result.rows[0] as Record<string, any>;

    expect(row.id).toBe('cust_100');
    expect(Array.isArray(row.invoices)).toBe(true);
    expect(row.metrics).toMatchObject({
      invoiceCount: 3,
      totalRevenue: 674250,
      averageInvoice: 224750,
    });
    expect(result.meta.engine).toBe('polars');
    expect(result.meta.arrowBytes).toBeGreaterThan(0);
    expect(typeof result.plan).toBe('string');
  });
});
