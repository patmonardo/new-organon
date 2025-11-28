/**
 * Customer Model - MVC SDSL Design Template
 *
 * The Model handles:
 * - Data access (CRUD operations)
 * - Validation
 * - State management
 *
 * Property.values() → Eval (this file)
 * Uses FactStore for persistence (mocked or backed by semantic services)
 */

import {
  type CustomerData,
  type CreateCustomer,
  type UpdateCustomer,
  type OperationResult,
  CustomerDataSchema,
  CreateCustomerSchema,
  UpdateCustomerSchema,
} from "./schema";
import { getFactStore, type FactStoreInterface } from "../../data/fact-store";

// =============================================================================
// CUSTOMER MODEL STATE
// =============================================================================

export interface CustomerModelState {
  customer: CustomerData | null;
  status: "idle" | "loading" | "success" | "error";
  errors: Record<string, string[]>;
  message?: string;
}

// =============================================================================
// CUSTOMER MODEL CLASS
// =============================================================================

/**
 * CustomerModel - Handles Customer data operations
 *
 * Pattern: Static methods for CRUD operations
 *          Uses FactStore for persistence tracking
 */
export class CustomerModel {
  private static factStore: FactStoreInterface = getFactStore();

  // In-memory store for mock (replace with real data service in production)
  private static customers: Map<string, CustomerData> = new Map();
  private static nextId: number = 1;

  // ---------------------------------------------------------------------------
  // CRUD Operations (Static methods)
  // ---------------------------------------------------------------------------

  /**
   * Create a new customer
   */
  static async create(data: CreateCustomer): Promise<OperationResult<CustomerData>> {
    try {
      // Validate input
      const validated = CreateCustomerSchema.safeParse(data);
      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Validation failed: " + validated.error.message,
        };
      }

      // Generate ID and timestamps
      const id = `customer-${this.nextId++}`;
      const now = new Date();

      const customer: CustomerData = {
        id,
        name: validated.data.name,
        email: validated.data.email,
        imageUrl: validated.data.imageUrl ?? null,
        createdAt: now,
        updatedAt: now,
      };

      // Store in mock database
      this.customers.set(id, customer);

      // Also store as Fact in FactStore
      await this.factStore.createFact(id, "customer.created", customer);

      return {
        data: customer,
        status: "success",
        message: "Customer created successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: `Failed to create customer: ${error}`,
      };
    }
  }

  /**
   * Update an existing customer
   */
  static async update(id: string, data: UpdateCustomer): Promise<OperationResult<CustomerData>> {
    try {
      // Check if customer exists
      const existing = this.customers.get(id);
      if (!existing) {
        return {
          data: null,
          status: "error",
          message: "Customer not found",
        };
      }

      // Validate input
      const validated = UpdateCustomerSchema.safeParse(data);
      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Validation failed: " + validated.error.message,
        };
      }

      // Merge updates
      const customer: CustomerData = {
        ...existing,
        ...validated.data,
        updatedAt: new Date(),
      };

      // Store update
      this.customers.set(id, customer);

      // Store as Fact
      await this.factStore.createFact(id, "customer.updated", customer);

      return {
        data: customer,
        status: "success",
        message: "Customer updated successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: `Failed to update customer: ${error}`,
      };
    }
  }

  /**
   * Find customer by ID
   */
  static async findById(id: string): Promise<OperationResult<CustomerData>> {
    try {
      const customer = this.customers.get(id);

      if (!customer) {
        return {
          data: null,
          status: "error",
          message: "Customer not found",
        };
      }

      return {
        data: customer,
        status: "success",
        message: "Customer found",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: `Failed to find customer: ${error}`,
      };
    }
  }

  /**
   * Find all customers with optional filtering
   */
  static async findAll(options: {
    query?: string;
    page?: number;
    pageSize?: number;
  } = {}): Promise<OperationResult<CustomerData[]>> {
    try {
      const { query = "", page = 1, pageSize = 10 } = options;

      let customers = Array.from(this.customers.values());

      // Filter by query
      if (query) {
        const lowerQuery = query.toLowerCase();
        customers = customers.filter(c =>
          c.name.toLowerCase().includes(lowerQuery) ||
          c.email.toLowerCase().includes(lowerQuery)
        );
      }

      // Sort by name
      customers.sort((a, b) => a.name.localeCompare(b.name));

      // Paginate
      const start = (page - 1) * pageSize;
      const paginated = customers.slice(start, start + pageSize);

      return {
        data: paginated,
        status: "success",
        message: `Found ${paginated.length} customers`,
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: `Failed to find customers: ${error}`,
      };
    }
  }

  /**
   * Delete a customer
   */
  static async delete(id: string): Promise<OperationResult<CustomerData>> {
    try {
      const customer = this.customers.get(id);

      if (!customer) {
        return {
          data: null,
          status: "error",
          message: "Customer not found",
        };
      }

      this.customers.delete(id);

      // Store as Fact
      await this.factStore.createFact(id, "customer.deleted", { id });

      return {
        data: customer,
        status: "success",
        message: "Customer deleted successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: `Failed to delete customer: ${error}`,
      };
    }
  }

  /**
   * Count total customers
   */
  static async count(query?: string): Promise<number> {
    if (!query) {
      return this.customers.size;
    }

    const lowerQuery = query.toLowerCase();
    return Array.from(this.customers.values()).filter(c =>
      c.name.toLowerCase().includes(lowerQuery) ||
      c.email.toLowerCase().includes(lowerQuery)
    ).length;
  }

  // ---------------------------------------------------------------------------
  // Utility Methods
  // ---------------------------------------------------------------------------

  /**
   * Clear all customers (for testing)
   */
  static clear(): void {
    this.customers.clear();
    this.nextId = 1;
  }

  /**
   * Seed with test data
   */
  static async seed(): Promise<void> {
    await this.create({ name: "Alice Smith", email: "alice@example.com" });
    await this.create({ name: "Bob Jones", email: "bob@example.com" });
    await this.create({ name: "Carol White", email: "carol@example.com" });
  }
}

