import { prisma } from "@data/client";
import type { OperationResult } from "@/data/schema/base";
import {
  InvoiceShapeSchema,
  CreateInvoiceSchema,
  UpdateInvoiceSchema,
} from "@/data/schema/invoice";
import type {
  Invoice,
  InvoiceShape,
  CreateInvoice,
  UpdateInvoice,
  InvoiceWithCustomer,
  InvoiceStatus,
} from "@/data/schema/invoice";
import { BaseModel } from "./base";

export class InvoiceModel extends BaseModel<InvoiceShape> {
  constructor(invoice?: Invoice) {
    const shape: InvoiceShape = {
      base: invoice || ({} as Invoice),
      state: {
        status: "active",
        validation: {},
        message: undefined,
      },
    };

    super(InvoiceShapeSchema, shape);
  }

  // ===== CORE CRUD OPERATIONS =====

  static async create(
    data: CreateInvoice
  ): Promise<OperationResult<InvoiceWithCustomer>> {
    try {
      const validated = CreateInvoiceSchema.safeParse(data);
      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Missing Fields. Failed to Create Invoice.",
        };
      }
      const invoice = await prisma.invoice.create({
        data: {
          id: crypto.randomUUID(),
          ...validated.data,
          date: data.date ?? new Date(),
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        include: { customer: true },
      });

      return {
        data: invoice as InvoiceWithCustomer,
        status: "success",
        message: "Invoice created",
      };
    } catch (error) {
      console.log(error);
      return {
        data: null,
        status: "error",
        message: "Failed to create invoice",
      };
    }
  }

  static async update(
    id: string,
    data: UpdateInvoice
  ): Promise<OperationResult<InvoiceWithCustomer>> {
    try {
      const validated = UpdateInvoiceSchema.safeParse(data);
      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Missing Fields. Failed to Update Invoice.",
        };
      }

      const invoice = await prisma.invoice.update({
        where: { id },
        data: {
          ...validated.data,
          updatedAt: new Date(),
        },
        include: { customer: true },
      });

      return {
        data: invoice as InvoiceWithCustomer,
        status: "success",
        message: "Invoice updated successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Failed to update invoice",
      };
    }
  }

  static async delete(
    id: string
  ): Promise<OperationResult<InvoiceWithCustomer>> {
    try {
      const invoice = await prisma.invoice.delete({
        where: { id },
        include: { customer: true },
      });

      return {
        data: invoice as InvoiceWithCustomer,
        status: "success",
        message: "Invoice deleted successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Failed to delete invoice",
      };
    }
  }

  // ===== QUERY OPERATIONS =====

  static async findById(
    id: string
  ): Promise<OperationResult<InvoiceWithCustomer>> {
    try {
      const invoice = await prisma.invoice.findUnique({
        where: { id },
        include: { customer: true },
      });

      if (!invoice) {
        return {
          data: null,
          status: "error",
          message: "Invoice not found",
        };
      }

      return {
        data: invoice as InvoiceWithCustomer,
        status: "success",
        message: "Invoice found",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Failed to find invoice",
      };
    }
  }

  static async findAll(
    options: {
      query?: string;
      page?: number;
      pageSize?: number;
    } = {}
  ): Promise<OperationResult<InvoiceWithCustomer[]>> {
    try {
      const { page = 1, pageSize = 10, query = "" } = options;
      const offset = (page - 1) * pageSize;

      // Build where clause based on field types
      let where = {};

      if (query) {
        // Array to build our OR conditions
        const orConditions = [];

        // Customer name and email - these support 'contains'
        orConditions.push(
          { customer: { name: { contains: query } } },
          { customer: { email: { contains: query } } }
        );

        // Status - enum field requires exact match
        // Only add if query matches a valid status
        const uppercaseQuery = query.toUpperCase();
        const validStatuses = ["PAID", "PENDING", "OVERDUE", "DRAFT"];
        if (validStatuses.includes(uppercaseQuery)) {
          orConditions.push({ status: uppercaseQuery });
        }

        // Amount - numeric field requires numeric comparison
        // Only add if query is a valid number
        const numericQuery = parseFloat(query);
        if (!isNaN(numericQuery)) {
          orConditions.push({ amount: { equals: numericQuery } });
        }

        where = { OR: orConditions };
      }

      const invoices = await prisma.invoice.findMany({
        where,
        skip: offset,
        take: pageSize,
        orderBy: { date: "desc" },
        include: { customer: true },
      });

      return {
        status: "success",
        data: invoices as InvoiceWithCustomer[],
        message: "Invoices retrieved successfully",
      };
    } catch (error) {
      console.error("Error fetching invoices:", error);

      return {
        status: "error",
        data: null,
        message: `Failed to retrieve invoices: ${error}`,
      };
    }
  }

  static async count(): Promise<OperationResult<number>> {
    try {
      const count = await prisma.invoice.count();
      return {
        data: count,
        status: "success",
        message: "Count retrieved successfully"
      };
    } catch (error) {
      console.error("Error counting invoices:", error);
      return {
        data: null,
        status: "error",
        message: "Failed to retrieve count"
      };
    }
  }

  // ===== DASHBOARD & REPORTING =====
  static async getLatestWithCustomers(
    limit = 5
  ): Promise<OperationResult<InvoiceWithCustomer[]>> {
    try {
      const invoices = await prisma.invoice.findMany({
        take: limit,
        orderBy: { date: "desc" },
        include: { customer: true },
      });

      return {
        data: invoices as InvoiceWithCustomer[],
        status: "success",
        message: "Latest invoices retrieved successfully",
      };
    } catch (error) {
      console.error("Error finding latest invoices:", error);
      // Still provide placeholder data, but with status information
      return {
        data: null,
        status: "error",
        message: "Using placeholder data - database error occurred",
      };
    }
  }

  static async getInvoiceCountByStatus(): Promise<
    OperationResult<Record<InvoiceStatus, number>>
  > {
    try {
      // Define statuses array based on InvoiceStatus enum
      const statuses = [
        "PAID",
        "PENDING",
        "OVERDUE",
        "DRAFT",
      ] as InvoiceStatus[];

      const counts: Record<InvoiceStatus, number> = {
        PAID: 0,
        PENDING: 0,
        OVERDUE: 0,
        DRAFT: 0,
      };

      // Get counts for each status
      await Promise.all(
        statuses.map(async (status) => {
          counts[status] = await prisma.invoice.count({
            where: { status },
          });
        })
      );

      return {
        data: counts,
        status: "success",
        message: "Invoice counts by status retrieved successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Error counting invoices by status:",
      };
    }
  }

  static async getTotalByStatus(
    status: InvoiceStatus
  ): Promise<OperationResult<number>> {
    try {
      const result = await prisma.invoice.aggregate({
        where: { status },
        _sum: { amount: true },
      });

      // Better handling of the Decimal value
      let total = 0;
      if (result._sum.amount) {
        // Convert Prisma Decimal to JavaScript number
        total = parseFloat(result._sum.amount.toString());
      }

      return {
        data: total,
        status: "success",
        message: `Total for ${status} invoices retrieved successfully`,
      };
    } catch (error) {
      console.error(`Error getting total for ${status} invoices:`, error);
      return {
        data: null, // Return 0 instead of null for consistency
        status: "error",
        message: `Error getting total for ${status} invoices`
      };
    }
  }
}
