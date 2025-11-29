import { Customer, Invoice } from './customer';
import { CustomerModel } from './customer-model';
import type { DataView } from '../../src/data/sdsl';
import { PolarsExecutionEngine, type PolarsDataset } from '../../src/data/polars-engine';
import type {
  RowLike,
  SemanticDataService,
  SemanticResult,
} from '../../src/data/semantic-hydrator';

export interface CustomerMetricSummary {
  totalRevenue: number;
  averageInvoice: number;
  invoiceCount: number;
}

export interface CustomerSemanticProfile extends Customer {
  invoices: Invoice[];
  metrics: CustomerMetricSummary;
  plan: string;
}

const MOCK_CUSTOMERS: Customer[] = [
  {
    id: 'cust_100',
    name: 'Acme Industries',
    email: 'ops@acme.com',
    imageUrl: 'https://images.ctfassets.net/example/acme-avatar.png',
    region: 'west',
    createdAt: '2023-05-12',
    invoices: [
      { id: 'inv_1001', customerId: 'cust_100', amount: 420_000, status: 'PAID', date: '2024-01-15' },
      { id: 'inv_1002', customerId: 'cust_100', amount: 155_500, status: 'PENDING', date: '2024-02-07' },
      { id: 'inv_1003', customerId: 'cust_100', amount: 98_750, status: 'OVERDUE', date: '2024-03-02' },
    ],
  },
  {
    id: 'cust_200',
    name: 'Blueberry Health',
    email: 'finance@blueberry.health',
    imageUrl: 'https://images.ctfassets.net/example/blueberry-avatar.png',
    region: 'northeast',
    createdAt: '2023-07-01',
    invoices: [
      { id: 'inv_2001', customerId: 'cust_200', amount: 310_250, status: 'PAID', date: '2024-01-30' },
      { id: 'inv_2002', customerId: 'cust_200', amount: 502_000, status: 'PAID', date: '2024-02-18' },
    ],
  },
  {
    id: 'cust_300',
    name: 'Cedar Analytics',
    email: 'ap@cedar.io',
    imageUrl: 'https://images.ctfassets.net/example/cedar-avatar.png',
    region: 'south',
    createdAt: '2022-11-19',
    invoices: [
      { id: 'inv_3001', customerId: 'cust_300', amount: 210_000, status: 'PAID', date: '2024-01-10' },
      { id: 'inv_3002', customerId: 'cust_300', amount: 180_500, status: 'DRAFT', date: '2024-03-28' },
    ],
  },
];

export class CustomerDataService implements SemanticDataService {
  private readonly polarsEngine: PolarsExecutionEngine;

  constructor(
    private readonly customers: Customer[] = MOCK_CUSTOMERS,
    polarsEngine: PolarsExecutionEngine = new PolarsExecutionEngine(),
  ) {
    this.polarsEngine = polarsEngine;
  }

  /**
   * Build the semantic view for a single customer profile.
   */
  private buildProfileView(customerId: string) {
    return CustomerModel.view({
      filter: { id: customerId },
      aggregate: ['totalRevenue', 'averageInvoice', 'count'],
      limit: 1,
    });
  }

  async execute(view: DataView): Promise<SemanticResult> {
    const dataset = this.buildDataset();
    const result = await this.polarsEngine.execute(view, {
      limit: view.query.limit,
      dataset,
    });

    return {
      plan: result.plan,
      rows: result.rows as RowLike[],
      meta: {
        ...result.meta,
        source: 'polars-engine',
      },
    };
  }

  async getCustomerProfile(customerId: string): Promise<CustomerSemanticProfile | null> {
    const view = this.buildProfileView(customerId);
    const result = await this.execute(view);
    const row = result.rows[0];
    if (!row) {
      return null;
    }

    const invoices = (row.invoices as Invoice[]) ?? [];
    const metrics = (row.metrics as CustomerMetricSummary) ?? this.calculateMetrics(invoices);

    return {
      id: String(row.id),
      name: String(row.name),
      email: String(row.email),
      imageUrl: row.imageUrl as string | undefined,
      region: row.region as string | undefined,
      createdAt: row.createdAt as string | undefined,
      invoices,
      metrics,
      plan: result.plan,
    };
  }

  getDatasetSnapshot(): PolarsDataset {
    return this.buildDataset();
  }

  private buildDataset(): PolarsDataset {
    const customers = this.customers.map(customer => ({
      id: customer.id,
      name: customer.name,
      email: customer.email,
      imageUrl: customer.imageUrl,
      region: customer.region ?? 'unassigned',
      createdAt: customer.createdAt ?? '2024-01-01',
    }));

    const invoices = this.customers.flatMap(customer =>
      (customer.invoices ?? []).map(invoice => ({
        id: invoice.id,
        customerId: invoice.customerId ?? customer.id,
        amount: invoice.amount,
        status: invoice.status,
        date: invoice.date,
      })),
    );

    return { customers, invoices };
  }

  private calculateMetrics(invoices: Invoice[]): CustomerMetricSummary {
    const totalRevenue = invoices.reduce((sum, invoice) => sum + invoice.amount, 0);
    const invoiceCount = invoices.length;
    const averageInvoice = invoiceCount === 0 ? 0 : Math.round(totalRevenue / invoiceCount);
    return { totalRevenue, invoiceCount, averageInvoice };
  }
}

export const defaultCustomerDataService = new CustomerDataService();
