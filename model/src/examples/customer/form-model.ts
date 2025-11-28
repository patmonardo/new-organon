/**
 * CustomerFormModel - Form state management for Customer
 *
 * Uses composition to wrap FormModel functionality.
 * Handles form state, validation, and persistence via CustomerModel.
 */

import { SimpleFormModel } from "../../sdsl/form-model";
import type { OperationResult, FormField } from "../../sdsl/types";
import { CustomerModel } from "./model";
import {
  CustomerFormShape,
  CustomerDataSchema,
  type CustomerData
} from "./schema";

// =============================================================================
// CUSTOMER FORM MODEL
// =============================================================================

/**
 * CustomerFormModel - Form state for Customer entity
 *
 * Wraps SimpleFormModel to provide:
 * - Form field state management
 * - Validation using CustomerDataSchema
 * - Persistence via CustomerModel
 */
export class CustomerFormModel {
  private _model: SimpleFormModel<typeof CustomerFormShape>;
  private _customerId?: string;

  constructor(customer?: CustomerData) {
    // Create shape with customer values
    const shape = {
      ...CustomerFormShape,
      fields: CustomerFormShape.fields.map(field => ({
        ...field,
        value: customer ? customer[field.id as keyof CustomerData] : undefined,
      })),
    };

    this._model = new SimpleFormModel(shape);
    this._customerId = customer?.id;
  }

  // ---------------------------------------------------------------------------
  // Accessors
  // ---------------------------------------------------------------------------

  get id(): string | undefined {
    return this._customerId;
  }

  get isNew(): boolean {
    return !this._customerId;
  }

  get isDirty(): boolean {
    return this._model.isDirty;
  }

  get shape() {
    return this._model.shape;
  }

  // ---------------------------------------------------------------------------
  // Field operations (delegate to model)
  // ---------------------------------------------------------------------------

  getField(fieldId: string): unknown {
    return this._model.getField(fieldId);
  }

  setField(fieldId: string, value: unknown): void {
    this._model.setField(fieldId, value);
  }

  getFields(): FormField[] {
    return this._model.getFields();
  }

  // ---------------------------------------------------------------------------
  // Value extraction
  // ---------------------------------------------------------------------------

  /**
   * Get all field values as a record
   */
  getValues(): Record<string, unknown> {
    const values: Record<string, unknown> = {};
    for (const field of this.getFields()) {
      values[field.id] = field.value;
    }
    return values;
  }

  /**
   * Get values as CustomerData
   */
  toCustomerData(): Partial<CustomerData> {
    const values = this.getValues();
    return {
      id: this._customerId,
      name: values.name as string,
      email: values.email as string,
      imageUrl: values.imageUrl as string | null | undefined,
    };
  }

  // ---------------------------------------------------------------------------
  // Validation
  // ---------------------------------------------------------------------------

  /**
   * Validate using CustomerDataSchema
   */
  validate(): OperationResult<boolean> {
    const data = this.toCustomerData();
    const result = CustomerDataSchema.safeParse({
      ...data,
      id: data.id || "temp",
    });

    if (!result.success) {
      const errors: Record<string, string[]> = {};
      for (const issue of result.error.issues) {
        const path = issue.path.join(".");
        if (!errors[path]) errors[path] = [];
        errors[path].push(issue.message);
      }

      return {
        status: "error",
        data: false,
        message: "Validation failed",
        errors,
      };
    }

    return {
      status: "success",
      data: true,
    };
  }

  // ---------------------------------------------------------------------------
  // Persistence
  // ---------------------------------------------------------------------------

  /**
   * Load customer by ID
   */
  async load(id?: string): Promise<OperationResult<CustomerData>> {
    const loadId = id || this._customerId;
    if (!loadId) {
      return {
        status: "error",
        data: null,
        message: "No ID to load",
      };
    }

    const result = await CustomerModel.findById(loadId);
    if (result.status !== "success" || !result.data) {
      return {
        status: "error",
        data: null,
        message: result.message || "Customer not found",
      };
    }

    // Update fields with loaded data
    this._customerId = result.data.id;
    for (const field of this.getFields()) {
      const value = result.data[field.id as keyof CustomerData];
      if (value !== undefined) {
        this.setField(field.id, value);
      }
    }

    return {
      status: "success",
      data: result.data,
    };
  }

  /**
   * Save (create or update)
   */
  async save(): Promise<OperationResult<CustomerData>> {
    // Validate first
    const validation = this.validate();
    if (validation.status !== "success") {
      return {
        status: "error",
        data: null,
        message: validation.message,
        errors: validation.errors,
      };
    }

    const data = this.toCustomerData();

    if (this._customerId) {
      // Update existing
      return CustomerModel.update(this._customerId, {
        name: data.name,
        email: data.email,
        imageUrl: data.imageUrl,
      });
    } else {
      // Create new
      const result = await CustomerModel.create({
        name: data.name!,
        email: data.email!,
        imageUrl: data.imageUrl,
      });

      if (result.status === "success" && result.data) {
        this._customerId = result.data.id;
      }
      return result;
    }
  }

  /**
   * Delete the customer
   */
  async delete(): Promise<OperationResult<CustomerData>> {
    if (!this._customerId) {
      return {
        status: "error",
        data: null,
        message: "No customer to delete",
      };
    }

    return CustomerModel.delete(this._customerId);
  }

  // ---------------------------------------------------------------------------
  // Factory methods
  // ---------------------------------------------------------------------------

  /**
   * Create empty form model for new customer
   */
  static create(): CustomerFormModel {
    return new CustomerFormModel();
  }

  /**
   * Create form model from existing customer
   */
  static from(customer: CustomerData): CustomerFormModel {
    return new CustomerFormModel(customer);
  }

  /**
   * Load form model by ID
   */
  static async load(id: string): Promise<CustomerFormModel | null> {
    const result = await CustomerModel.findById(id);
    if (result.status !== "success" || !result.data) {
      return null;
    }
    return new CustomerFormModel(result.data);
  }
}
