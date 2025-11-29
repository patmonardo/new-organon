import type { ReactNode } from 'react';
import { ReactController } from '../../src/sdsl/react-controller';
import type { DisplayDocument, FormHandler, FormShape, FormMode } from '../../src/sdsl/types';
import type { FormModel } from '../../src/sdsl/form-model';
import { radixAdapter, type RadixAdapter, type RadixRenderContext } from '../../src/sdsl/radix-adapter';
import { Customer, CustomerShape, Invoice } from './customer';
import {
  CustomerDataService,
  defaultCustomerDataService
} from './customer-data.service';
import { CustomerModel } from './customer-model';
import { CustomerView } from './customer-view';
import {
  SemanticHydrator,
  type HydratorSnapshot,
  type HydratorSpec,
} from '../../src/data/semantic-hydrator';

// Mock Database
const DB: Record<string, Customer> = {};

interface RadixDashboardRender {
  element: ReactNode;
  document: DisplayDocument;
  snapshot?: HydratorSnapshot;
}

interface RadixDashboardOptions {
  adapter?: RadixAdapter;
  handler?: FormHandler;
  context?: Partial<RadixRenderContext>;
}

const customerProfileSpec: HydratorSpec = {
  id: 'customer-profile',
  view: ({ params }) => {
    const customerId = params?.customerId as string | undefined;
    if (!customerId) {
      throw new Error('customerId param is required for customer-profile hydration');
    }
    return CustomerModel.view({
      filter: { id: customerId },
      aggregate: ['totalRevenue', 'averageInvoice', 'count'],
      limit: 1,
    });
  },
  fields: [
    { fieldId: 'id', source: 'id' },
    { fieldId: 'name', source: 'name' },
    { fieldId: 'email', source: 'email' },
    { fieldId: 'imageUrl', source: 'imageUrl' },
  ],
  collections: [
    { id: 'invoices', source: 'invoices', fieldId: 'invoices' },
  ],
  metrics: [
    { name: 'invoiceCount', source: 'metrics.invoiceCount', fieldId: 'invoiceCount' },
    { name: 'totalRevenue', source: 'metrics.totalRevenue', fieldId: 'totalRevenue' },
    { name: 'averageInvoice', source: 'metrics.averageInvoice', fieldId: 'averageInvoice' },
  ],
  metaFields: {
    '$plan': 'semanticPlan',
  },
};

export class CustomerController extends ReactController<FormShape> {
  private dataService: CustomerDataService;
  private hydrator: SemanticHydrator;
  private semanticSnapshot?: HydratorSnapshot;
  private get formModel(): FormModel<FormShape> {
    return (this as unknown as { _model: FormModel<FormShape> })._model;
  }

  constructor(
    mode: FormMode = 'create',
    initialData?: Partial<Customer>,
    dataService: CustomerDataService = defaultCustomerDataService
  ) {
    super(CustomerShape as any, mode);
    this.dataService = dataService;
    this.hydrator = new SemanticHydrator(this.dataService);
    if (initialData) {
      // Pre-populate model
      for (const [key, value] of Object.entries(initialData)) {
        this.formModel.setField(key, value);
      }
    }
  }

  /**
   * Override executeAction to handle business logic
   */
  public async executeAction(actionId: string, data?: unknown): Promise<any> {
    console.log(`[Controller] Executing action: ${actionId}`, data);

    switch (actionId) {
      case 'submit':
        return this.handleSubmit(data as Partial<Customer>);
      case 'cancel':
        return this.handleCancel();
      case 'delete':
        return this.handleDelete(data as { id: string });
      default:
        return super.executeAction(actionId, data);
    }
  }

  private async handleSubmit(data: Partial<Customer>): Promise<Customer> {
    // Simulate DB delay
    await new Promise(resolve => setTimeout(resolve, 500));

    // Validate (simple check)
    if (!data.name || !data.email) {
      throw new Error('Name and Email are required');
    }

    const id = data.id || `cust_${Date.now()}`;
    const customer: Customer = {
      ...data,
      id,
      name: data.name,
      email: data.email,
      invoices: data.invoices || [],
    };

    // Save to "DB"
    DB[id] = customer;
    console.log(`[Controller] Saved customer: ${id}`);

    // Update the model with the saved data (including ID)
    for (const [key, value] of Object.entries(customer)) {
      this.formModel.setField(key, value);
    }

    return customer;
  }

  private async handleCancel(): Promise<void> {
    console.log('[Controller] Cancelled operation');
  }

  private async handleDelete(data: { id: string }): Promise<void> {
    if (DB[data.id]) {
      delete DB[data.id];
      console.log(`[Controller] Deleted customer: ${data.id}`);
    } else {
      throw new Error('Customer not found');
    }
  }

  // ===========================================================================
  // Custom Business Logic (Not just Form Actions)
  // ===========================================================================

  async getInvoices(customerId: string) {
    const customer = DB[customerId];
    if (customer?.invoices) {
      return customer.invoices;
    }
    const snapshotInvoices = this.semanticSnapshot?.collections?.invoices;
    if (snapshotInvoices) {
      return snapshotInvoices as Invoice[];
    }
    throw new Error('Customer not found');
  }

  renderRadixDashboard(options: RadixDashboardOptions = {}): RadixDashboardRender {
    const adapter = options.adapter ?? radixAdapter;
    const handler = options.handler ?? this.createReactHandler();
    const view = new CustomerView(this.formModel, this._mode);
    const document = view.render();
    const context: RadixRenderContext = {
      handler,
      snapshot: this.semanticSnapshot,
      mode: this._mode,
      data: this.extractModelValues(),
      ...options.context,
    };

    return {
      element: adapter.render(document, context),
      document,
      snapshot: this.semanticSnapshot,
    };
  }

  // ===========================================================================
  // Semantic Layer Bridge
  // ===========================================================================

  async loadCustomerProfile(customerId: string): Promise<HydratorSnapshot | null> {
    const snapshot = await this.hydrator.hydrate(this.formModel, customerProfileSpec, {
      params: { customerId },
    });

    if (!snapshot.rows.length) {
      return null;
    }

    this.semanticSnapshot = snapshot;
    this.persistSemanticRow(snapshot);
    return snapshot;
  }

  getSemanticSnapshot(): HydratorSnapshot | undefined {
    return this.semanticSnapshot;
  }

  private persistSemanticRow(snapshot: HydratorSnapshot): void {
    const row = snapshot.rows[0];
    if (!row || typeof row.id !== 'string') {
      return;
    }

    DB[row.id] = {
      id: row.id as string,
      name: String(row.name ?? ''),
      email: String(row.email ?? ''),
      imageUrl: row.imageUrl as string | undefined,
      invoices: (row.invoices as Invoice[]) ?? [],
    };
  }

  private extractModelValues(): Record<string, unknown> {
    return this.formModel.shape.fields.reduce<Record<string, unknown>>((acc, field) => {
      acc[field.id] = field.value;
      return acc;
    }, {});
  }
}
