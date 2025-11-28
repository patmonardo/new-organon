/**
 * CustomerFormView - View rendering for Customer forms
 * 
 * Produces DisplayDocument that adapters render.
 */

import type { DisplayDocument, DisplayElement, FormMode } from "../../sdsl/types";
import { CustomerFormModel } from "./form-model";
import { CustomerFormShape, type CustomerData } from "./schema";

// =============================================================================
// CUSTOMER FORM VIEW
// =============================================================================

/**
 * CustomerFormView - Renders Customer forms as DisplayDocument
 * 
 * Produces:
 * - DisplayDocument for adapter rendering
 * - Mode-specific output (create, edit, view)
 */
export class CustomerFormView {
  private _model: CustomerFormModel;
  private _mode: FormMode;

  constructor(model: CustomerFormModel, mode: FormMode = "view") {
    this._model = model;
    this._mode = mode;
  }

  // ---------------------------------------------------------------------------
  // Accessors
  // ---------------------------------------------------------------------------

  get mode(): FormMode {
    return this._mode;
  }

  setMode(mode: FormMode): void {
    this._mode = mode;
  }

  // ---------------------------------------------------------------------------
  // Rendering
  // ---------------------------------------------------------------------------

  /**
   * Render form as DisplayDocument
   */
  render(): DisplayDocument {
    const isReadOnly = this._mode === "view";
    const title = this._mode === "create" 
      ? "Create Customer" 
      : this._mode === "edit" 
        ? "Edit Customer" 
        : "Customer Details";

    const children: DisplayElement[] = this._model.getFields().map(field => ({
      type: isReadOnly ? "text" : "input",
      text: isReadOnly ? String(field.value ?? "") : undefined,
      props: {
        id: field.id,
        name: field.id,
        type: field.type,
        label: field.label,
        placeholder: field.placeholder,
        required: field.required,
        disabled: field.disabled || isReadOnly,
        defaultValue: field.value,
      },
    }));

    return {
      title,
      layout: {
        type: "stack",
        children,
        gap: 16,
        padding: 24,
      },
      meta: {
        mode: this._mode,
        entityType: "customer",
        customerId: this._model.id,
      },
    };
  }

  /**
   * Render with customer-specific sections
   */
  renderWithSections(): DisplayDocument {
    const base = this.render();
    
    return {
      ...base,
      meta: {
        ...base.meta,
        sections: CustomerFormShape.meta?.sections,
        actions: CustomerFormShape.meta?.actions,
      },
    };
  }

  /**
   * Serialize to JSON
   */
  toJSON(): object {
    return {
      mode: this._mode,
      shape: CustomerFormShape,
      values: this._model.getValues(),
      display: this.render(),
    };
  }

  // ---------------------------------------------------------------------------
  // Static rendering methods
  // ---------------------------------------------------------------------------

  /**
   * Render for list/table display
   */
  static renderListItem(customer: CustomerData): DisplayDocument {
    return {
      title: customer.name,
      layout: {
        type: "row",
        children: [
          {
            type: "text",
            text: customer.name,
            props: { className: "font-medium" },
          },
          {
            type: "text",
            text: customer.email,
            props: { className: "text-gray-600" },
          },
        ],
      },
      meta: {
        id: customer.id,
        entityType: "customer",
      },
    };
  }

  /**
   * Render list of customers
   */
  static renderList(customers: CustomerData[]): DisplayDocument {
    return {
      title: "Customers",
      layout: {
        type: "stack",
        children: customers.map(customer => ({
          type: "card",
          children: [
            {
              type: "text",
              text: customer.name,
              props: { className: "font-medium" },
            },
            {
              type: "text",
              text: customer.email,
              props: { className: "text-gray-600" },
            },
          ],
        })),
      },
      meta: {
        entityType: "customer",
        count: customers.length,
      },
    };
  }

  // ---------------------------------------------------------------------------
  // Factory methods
  // ---------------------------------------------------------------------------

  /**
   * Create view for new customer form
   */
  static forCreate(): CustomerFormView {
    const model = CustomerFormModel.create();
    return new CustomerFormView(model, "create");
  }

  /**
   * Create view for editing customer
   */
  static forEdit(model: CustomerFormModel): CustomerFormView {
    return new CustomerFormView(model, "edit");
  }

  /**
   * Create view for viewing customer (read-only)
   */
  static forView(model: CustomerFormModel): CustomerFormView {
    return new CustomerFormView(model, "view");
  }

  /**
   * Load and create view by ID
   */
  static async load(id: string, mode: FormMode = "view"): Promise<CustomerFormView | null> {
    const model = await CustomerFormModel.load(id);
    if (!model) return null;
    return new CustomerFormView(model, mode);
  }
}
