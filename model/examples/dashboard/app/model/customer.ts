//@model/customer.ts
import { prisma } from "@data/client";
import type { OperationResult } from "@/data/schema/base";
import type {
  Customer,
  CustomerShape,
  CreateCustomer,
  UpdateCustomer,
} from "@/data/schema/customer";
import {
  CustomerShapeSchema,
  CreateCustomerSchema,
  UpdateCustomerSchema,
} from "@/data/schema/customer";
import { BaseModel } from "./base";

export class CustomerModel extends BaseModel<CustomerShape> {
  constructor(customer?: Customer) {
    // Initialize with a shape that has the customer data and default state
    const shape: CustomerShape = {
      base: customer || ({} as Customer),
      state: {
        status: "active",
        validation: {},
        message: undefined,
      },
    };

    super(CustomerShapeSchema, shape);
  }

  // Add customer-specific methods
  isActive(): boolean {
    return this.state.status === "active";
  }

  static async create(
    data: CreateCustomer
  ): Promise<OperationResult<Customer>> {
    try {
      const validated = CreateCustomerSchema.safeParse(data);
      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Missing Fields. Failed to Create Customer.",
        };
      }
      const customer = await prisma.customer.create({
        data: {
          name: validated.data.name,
          email: validated.data.email,
          imageUrl: validated.data.imageUrl,
        },
      });

      return {
        data: customer,
        status: "success",
        message: "Customer created",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Failed to create Customer",
      };
    }
  }

  // Example of using map() to transform the shape
  markAsArchived(): CustomerModel {
    const archivedShape = this.map((shape) => ({
      ...shape,
      state: {
        ...shape.state,
        status: "archived",
      },
    }));

    return new CustomerModel(archivedShape.base);
  }

  static async update(
    id: string,
    data: UpdateCustomer
  ): Promise<OperationResult<Customer>> {
    try {
      const validated = UpdateCustomerSchema.safeParse(data);
      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Missing Fields. Failed to Update Customer.",
        };
      }
      const customer = await prisma.customer.update({
        where: { id },
        data: {
          ...validated.data,
        },
      });

      return {
        data: customer,
        status: "success",
        message: "Customer updated successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Failed to update Customer",
      };
    }
  }

  static async findById(id: string): Promise<OperationResult<Customer>> {
    try {
      const customer = await prisma.customer.findUnique({
        where: { id },
      });

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
        message: "Failed to find Customer",
      };
    }
  }

  static async findAll(options: {
    query?: string;
    page?: number;
    pageSize?: number;
  } = {}): Promise<OperationResult<Customer[]>> {
    try {
      let customers: Customer[] = [];

      const { page = 1, pageSize = 10, query = '' } = options;
      const offset = (page - 1) * pageSize;

      const where = query
        ? {
            OR: [
              { name: { contains: query } },
              { email: { contains: query } },
            ],
          }
        : {};

      customers = await prisma.customer.findMany({
        where,
        skip: offset,
        take: pageSize,
        orderBy: { name: 'asc' },
      });

      return {
        status: "success",
        data: customers,
        message: "Customers retrieved successfully",
      };
    } catch (error) {
      console.error("Error fetching customers:", error);
      return {
        status: "error",
        data: null,
        message: "Failed to retrieve customers",
      };
    }
  }

  static async count() {
    return await prisma.customer.count();
  }

  static async delete(id: string): Promise<OperationResult<Customer>> {
    try {
      const customer = await prisma.customer.delete({
        where: { id },
      });

      return {
        data: customer as Customer,
        status: "success",
        message: "Customer deleted successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Failed to delete Customer",
      };
    }
  }
}
