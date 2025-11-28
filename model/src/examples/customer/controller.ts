/**
 * CustomerController - MVC SDSL Design Template
 *
 * Transport-agnostic business logic for Customer entity.
 *
 * The Controller handles:
 * - Orchestrating Model and View
 * - CRUD operations (transport-agnostic)
 * - Action execution with rules
 *
 * This controller can be called by:
 * - tRPC
 * - HTTP/REST
 * - Next.js Server Actions
 * - GraphQL resolvers
 * - CLI commands
 */

import type {
  FormMode,
  ControllerResult,
  FormDefinition,
  ListResult,
  OperationResult,
  DisplayDocument,
} from "../../sdsl/types";
import { CustomerFormModel } from "./form-model";
import { CustomerFormView } from "./form-view";
import { CustomerModel } from "./model";
import {
  CustomerFormShape,
  type CustomerData,
  type CreateCustomer,
  type UpdateCustomer
} from "./schema";

// =============================================================================
// ACTION TYPES
// =============================================================================

export interface ControllerAction {
  id: string;
  name: string;
  handler: (data?: unknown) => Promise<OperationResult<unknown>>;
}

// =============================================================================
// CUSTOMER CONTROLLER CLASS
// =============================================================================

/**
 * CustomerController - Transport-agnostic business logic
 *
 * Uses CustomerFormModel and CustomerFormView.
 * Returns pure data - transport layer handles rendering.
 */
export class CustomerController {
  private _model: CustomerFormModel;
  private _view: CustomerFormView;
  private _mode: FormMode;
  private _actions: Map<string, ControllerAction> = new Map();

  constructor(mode: FormMode = "view", customer?: CustomerData) {
    this._model = customer
      ? CustomerFormModel.from(customer)
      : CustomerFormModel.create();
    this._view = new CustomerFormView(this._model, mode);
    this._mode = mode;

    this.registerActions();
  }

  // ---------------------------------------------------------------------------
  // Accessors
  // ---------------------------------------------------------------------------

  get mode(): FormMode {
    return this._mode;
  }

  setMode(mode: FormMode): void {
    this._mode = mode;
    this._view.setMode(mode);
  }

  get model(): CustomerFormModel {
    return this._model;
  }

  get view(): CustomerFormView {
    return this._view;
  }

  // ---------------------------------------------------------------------------
  // Actions Registration
  // ---------------------------------------------------------------------------

  private registerActions(): void {
    this.registerAction({
      id: "submit",
      name: "Submit",
      handler: async () => this._model.save(),
    });

    this.registerAction({
      id: "delete",
      name: "Delete",
      handler: async () => this._model.delete(),
    });

    this.registerAction({
      id: "cancel",
      name: "Cancel",
      handler: async () => ({
        status: "success" as const,
        data: null,
        message: "Cancelled",
      }),
    });
  }

  registerAction(action: ControllerAction): void {
    this._actions.set(action.id, action);
  }

  async executeAction(actionId: string, data?: unknown): Promise<OperationResult<unknown>> {
    const action = this._actions.get(actionId);
    if (!action) {
      return {
        status: "error",
        data: null,
        message: `Unknown action: ${actionId}`,
      };
    }
    return action.handler(data);
  }

  // ---------------------------------------------------------------------------
  // INSTANCE METHODS
  // ---------------------------------------------------------------------------

  /**
   * Get form definition for rendering
   */
  getFormDefinition(): FormDefinition {
    return {
      shape: CustomerFormShape,
      values: this._model.getValues(),
      mode: this._mode,
      actions: Array.from(this._actions.keys()),
    };
  }

  /**
   * Get DisplayDocument for adapter rendering
   */
  getDisplayDocument(): DisplayDocument {
    return this._view.renderWithSections();
  }

  /**
   * Validate current form state
   */
  validate(): OperationResult<boolean> {
    return this._model.validate();
  }

  // ---------------------------------------------------------------------------
  // STATIC QUERY METHODS (Read operations - transport-agnostic)
  // ---------------------------------------------------------------------------

  /**
   * Get a single customer by ID
   *
   * Called by: tRPC query, HTTP GET /customers/:id, GraphQL query
   */
  static async getById(id: string): Promise<ControllerResult<CustomerData>> {
    const result = await CustomerModel.findById(id);

    if (result.status !== "success" || !result.data) {
      return {
        success: false,
        error: result.message || "Customer not found",
      };
    }

    return {
      success: true,
      data: result.data,
    };
  }

  /**
   * List customers with pagination
   *
   * Called by: tRPC query, HTTP GET /customers, GraphQL query
   */
  static async list(options: {
    query?: string;
    page?: number;
    pageSize?: number;
  } = {}): Promise<ControllerResult<ListResult<CustomerData>>> {
    const { query = "", page = 1, pageSize = 10 } = options;

    const result = await CustomerModel.findAll({ query, page, pageSize });

    if (result.status !== "success" || !result.data) {
      return {
        success: false,
        error: result.message || "Failed to list customers",
      };
    }

    const total = await CustomerModel.count(query);
    const totalPages = Math.ceil(total / pageSize);

    return {
      success: true,
      data: {
        items: result.data,
        total,
        page,
        pageSize,
        totalPages,
      },
    };
  }

  // ---------------------------------------------------------------------------
  // STATIC MUTATION METHODS (Write operations - transport-agnostic)
  // ---------------------------------------------------------------------------

  /**
   * Create a new customer
   *
   * Called by: tRPC mutation, HTTP POST /customers, GraphQL mutation
   */
  static async create(data: CreateCustomer): Promise<ControllerResult<CustomerData>> {
    if (!data.name || !data.email) {
      return {
        success: false,
        error: "Name and email are required",
      };
    }

    const result = await CustomerModel.create(data);

    if (result.status !== "success" || !result.data) {
      return {
        success: false,
        error: result.message || "Failed to create customer",
      };
    }

    return {
      success: true,
      data: result.data,
      redirect: "/customers",
    };
  }

  /**
   * Update an existing customer
   *
   * Called by: tRPC mutation, HTTP PUT /customers/:id, GraphQL mutation
   */
  static async update(id: string, data: UpdateCustomer): Promise<ControllerResult<CustomerData>> {
    const result = await CustomerModel.update(id, data);

    if (result.status !== "success" || !result.data) {
      return {
        success: false,
        error: result.message || "Failed to update customer",
      };
    }

    return {
      success: true,
      data: result.data,
      redirect: "/customers",
    };
  }

  /**
   * Delete a customer
   *
   * Called by: tRPC mutation, HTTP DELETE /customers/:id, GraphQL mutation
   */
  static async delete(id: string): Promise<ControllerResult<void>> {
    const result = await CustomerModel.delete(id);

    if (result.status !== "success") {
      return {
        success: false,
        error: result.message || "Failed to delete customer",
      };
    }

    return {
      success: true,
      redirect: "/customers",
    };
  }

  // ---------------------------------------------------------------------------
  // FORM DATA HANDLING (For HTML form submissions)
  // ---------------------------------------------------------------------------

  /**
   * Handle form submission (from FormData)
   *
   * Called by: Next.js Server Actions, HTML form POST
   */
  static async handleFormSubmit(
    formData: FormData,
    customerId?: string
  ): Promise<ControllerResult<CustomerData>> {
    const data = {
      name: formData.get("name") as string,
      email: formData.get("email") as string,
      imageUrl: formData.get("imageUrl") as string || null,
    };

    if (customerId) {
      return this.update(customerId, data);
    } else {
      return this.create(data);
    }
  }

  // ---------------------------------------------------------------------------
  // FACTORY METHODS
  // ---------------------------------------------------------------------------

  /**
   * Create controller for new customer
   */
  static forCreate(): CustomerController {
    return new CustomerController("create");
  }

  /**
   * Create controller for editing existing customer
   */
  static async forEdit(id: string): Promise<CustomerController | null> {
    const result = await CustomerModel.findById(id);
    if (result.status !== "success" || !result.data) {
      return null;
    }
    return new CustomerController("edit", result.data);
  }

  /**
   * Create controller for viewing customer
   */
  static async forView(id: string): Promise<CustomerController | null> {
    const result = await CustomerModel.findById(id);
    if (result.status !== "success" || !result.data) {
      return null;
    }
    return new CustomerController("view", result.data);
  }
}

// =============================================================================
// TRANSPORT ADAPTERS
// =============================================================================

// For Next.js Server Actions
export async function createCustomerAction(formData: FormData): Promise<ControllerResult<CustomerData>> {
  "use server";
  return CustomerController.handleFormSubmit(formData);
}

export async function updateCustomerAction(id: string, formData: FormData): Promise<ControllerResult<CustomerData>> {
  "use server";
  return CustomerController.handleFormSubmit(formData, id);
}

export async function deleteCustomerAction(id: string): Promise<ControllerResult<void>> {
  "use server";
  return CustomerController.delete(id);
}

// For tRPC (example router shape)
export const customerTrpcRouter = {
  getById: CustomerController.getById,
  list: CustomerController.list,
  create: CustomerController.create,
  update: CustomerController.update,
  delete: CustomerController.delete,
};

// For REST API (example handlers)
export const customerRestHandlers = {
  list: async (req: { query: { q?: string; page?: string } }) => {
    return CustomerController.list({
      query: req.query.q,
      page: req.query.page ? parseInt(req.query.page) : 1,
    });
  },
  getById: async (req: { params: { id: string } }) => {
    return CustomerController.getById(req.params.id);
  },
  create: async (req: { body: CreateCustomer }) => {
    return CustomerController.create(req.body);
  },
  update: async (req: { params: { id: string }; body: UpdateCustomer }) => {
    return CustomerController.update(req.params.id, req.body);
  },
  delete: async (req: { params: { id: string } }) => {
    return CustomerController.delete(req.params.id);
  },
};
