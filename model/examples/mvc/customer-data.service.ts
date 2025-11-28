import { Customer, Invoice } from './customer';
import { CustomerModel } from './customer-model';

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
    invoices: [
      { id: 'inv_2001', customerId: 'cust_200', amount: 310_250, status: 'PAID', date: '2024-01-30' },
      { id: 'inv_2002', customerId: 'cust_200', amount: 502_000, status: 'PAID', date: '2024-02-18' },
    ],
  },
];

export class CustomerDataService {
  constructor(private readonly customers: Customer[] = MOCK_CUSTOMERS) {}

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

  private calculateMetrics(invoices: Invoice[]): CustomerMetricSummary {
    const totalRevenue = invoices.reduce((sum, invoice) => sum + invoice.amount, 0);
    const invoiceCount = invoices.length;
    const averageInvoice = invoiceCount === 0 ? 0 : Math.round(totalRevenue / invoiceCount);
    return { totalRevenue, invoiceCount, averageInvoice };
  }

  async getCustomerProfile(customerId: string): Promise<CustomerSemanticProfile | null> {
    const view = this.buildProfileView(customerId);
    const plan = view.toPlan();

    const record = this.customers.find(customer => customer.id === customerId);
    if (!record) {
      return null;
    }

    const invoices = [...(record.invoices ?? [])];
    const metrics = this.calculateMetrics(invoices);

    return {
      ...record,
      invoices,
      metrics,
      plan,
    };
  }
}

export const defaultCustomerDataService = new CustomerDataService();
