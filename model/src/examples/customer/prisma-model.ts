/**
 * CustomerPrismaModel - Prisma-backed Customer data access
 * 
 * This is the production version that talks to Postgres.
 * Use this instead of CustomerModel when you have a database.
 * 
 * Usage:
 *   import { CustomerPrismaModel as CustomerModel } from "./prisma-model";
 */

import { prisma } from "../../data/prisma";
import type { OperationResult } from "../../sdsl/types";
import { 
  type CustomerData, 
  type CreateCustomer, 
  type UpdateCustomer,
  CreateCustomerSchema,
  UpdateCustomerSchema,
} from "./schema";

// =============================================================================
// CUSTOMER PRISMA MODEL
// =============================================================================

/**
 * CustomerPrismaModel - Prisma/Postgres CRUD operations
 * 
 * Same interface as CustomerModel (mock) but uses real database.
 */
export class CustomerPrismaModel {
  // ---------------------------------------------------------------------------
  // CREATE
  // ---------------------------------------------------------------------------

  /**
   * Create a new customer in database
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

      // Create in database
      // Note: Prisma schema has 'Customer' model, map fields accordingly
      const customer = await prisma.entity.create({
        data: {
          type: "customer",
          name: validated.data.name,
          data: {
            email: validated.data.email,
            imageUrl: validated.data.imageUrl,
          },
        },
      });

      return {
        data: {
          id: customer.id,
          name: customer.name || validated.data.name,
          email: validated.data.email,
          imageUrl: validated.data.imageUrl,
          createdAt: customer.createdAt,
          updatedAt: customer.updatedAt,
        },
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

  // ---------------------------------------------------------------------------
  // READ
  // ---------------------------------------------------------------------------

  /**
   * Find customer by ID
   */
  static async findById(id: string): Promise<OperationResult<CustomerData>> {
    try {
      const entity = await prisma.entity.findUnique({
        where: { id },
      });

      if (!entity || entity.type !== "customer") {
        return {
          data: null,
          status: "error",
          message: "Customer not found",
        };
      }

      const data = entity.data as { email?: string; imageUrl?: string | null } | null;

      return {
        data: {
          id: entity.id,
          name: entity.name || "",
          email: data?.email || "",
          imageUrl: data?.imageUrl,
          createdAt: entity.createdAt,
          updatedAt: entity.updatedAt,
        },
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
   * Find all customers with pagination and search
   */
  static async findAll(options: {
    query?: string;
    page?: number;
    pageSize?: number;
  } = {}): Promise<OperationResult<CustomerData[]>> {
    try {
      const { query = "", page = 1, pageSize = 10 } = options;
      const skip = (page - 1) * pageSize;

      // Build where clause
      const where = {
        type: "customer",
        ...(query ? {
          OR: [
            { name: { contains: query, mode: "insensitive" as const } },
          ],
        } : {}),
      };

      const entities = await prisma.entity.findMany({
        where,
        skip,
        take: pageSize,
        orderBy: { name: "asc" },
      });

      const customers: CustomerData[] = entities.map(entity => {
        const data = entity.data as { email?: string; imageUrl?: string | null } | null;
        return {
          id: entity.id,
          name: entity.name || "",
          email: data?.email || "",
          imageUrl: data?.imageUrl,
          createdAt: entity.createdAt,
          updatedAt: entity.updatedAt,
        };
      });

      return {
        data: customers,
        status: "success",
        message: `Found ${customers.length} customers`,
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
   * Count customers
   */
  static async count(query?: string): Promise<number> {
    try {
      const where = {
        type: "customer",
        ...(query ? {
          OR: [
            { name: { contains: query, mode: "insensitive" as const } },
          ],
        } : {}),
      };

      return await prisma.entity.count({ where });
    } catch {
      return 0;
    }
  }

  // ---------------------------------------------------------------------------
  // UPDATE
  // ---------------------------------------------------------------------------

  /**
   * Update an existing customer
   */
  static async update(id: string, data: UpdateCustomer): Promise<OperationResult<CustomerData>> {
    try {
      // Check if exists
      const existing = await prisma.entity.findUnique({
        where: { id },
      });

      if (!existing || existing.type !== "customer") {
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

      // Merge with existing data
      const existingData = existing.data as { email?: string; imageUrl?: string | null } | null;
      const newData = {
        email: validated.data.email ?? existingData?.email,
        imageUrl: validated.data.imageUrl ?? existingData?.imageUrl,
      };

      // Update in database
      const updated = await prisma.entity.update({
        where: { id },
        data: {
          name: validated.data.name ?? existing.name,
          data: newData,
        },
      });

      return {
        data: {
          id: updated.id,
          name: updated.name || "",
          email: newData.email || "",
          imageUrl: newData.imageUrl,
          createdAt: updated.createdAt,
          updatedAt: updated.updatedAt,
        },
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

  // ---------------------------------------------------------------------------
  // DELETE
  // ---------------------------------------------------------------------------

  /**
   * Delete a customer
   */
  static async delete(id: string): Promise<OperationResult<CustomerData>> {
    try {
      const existing = await prisma.entity.findUnique({
        where: { id },
      });

      if (!existing || existing.type !== "customer") {
        return {
          data: null,
          status: "error",
          message: "Customer not found",
        };
      }

      await prisma.entity.delete({
        where: { id },
      });

      const data = existing.data as { email?: string; imageUrl?: string | null } | null;

      return {
        data: {
          id: existing.id,
          name: existing.name || "",
          email: data?.email || "",
          imageUrl: data?.imageUrl,
          createdAt: existing.createdAt,
          updatedAt: existing.updatedAt,
        },
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

  // ---------------------------------------------------------------------------
  // UTILITIES
  // ---------------------------------------------------------------------------

  /**
   * Seed sample customers
   */
  static async seed(): Promise<void> {
    const samples = [
      { name: "Alice Smith", email: "alice@example.com" },
      { name: "Bob Jones", email: "bob@example.com" },
      { name: "Carol White", email: "carol@example.com" },
    ];

    for (const sample of samples) {
      await this.create(sample);
    }
  }
}

