import { ReactController, ServerActionResult } from '../../src/sdsl/react-controller';
import { FormShape, FormMode } from '../../src/sdsl/types';
import { Customer, CustomerShape } from './customer';

// Mock Database
const DB: Record<string, Customer> = {};

export class CustomerController extends ReactController<FormShape> {

  constructor(mode: FormMode = 'create', initialData?: Partial<Customer>) {
    super(CustomerShape as any, mode);
    if (initialData) {
      // Pre-populate model
      for (const [key, value] of Object.entries(initialData)) {
        this._model.setField(key, value);
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
      this._model.setField(key, value);
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
    if (!customer) throw new Error('Customer not found');
    return customer.invoices || [];
  }
}
