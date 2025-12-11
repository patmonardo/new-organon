//@model/revenue.ts
import { prisma } from "@data/client";
import {
  CreateRevenueSchema,
  UpdateRevenueSchema,
  RevenueShapeSchema,
} from "@/data/schema/revenue";
import type {
  Revenue,
  RevenueShape,
  CreateRevenue,
  UpdateRevenue,
  RevenueMetrics,
} from "@/data/schema/revenue";
import type { OperationResult } from "@/data/schema/base";
import { BaseModel } from "./base";

export class RevenueModel extends BaseModel<RevenueShape> {
  constructor(revenue?: Revenue) {
    const shape: RevenueShape = {
      base: revenue || ({} as Revenue),
      state: {
        status: "active",
        validation: {},
        message: undefined,
      },
    };

    super(RevenueShapeSchema, shape);
  }

  // ===== CORE CRUD OPERATIONS =====

  static async create(data: CreateRevenue): Promise<OperationResult<Revenue>> {
    try {
      const validated = CreateRevenueSchema.safeParse({
        ...data,
        expenses: data.expenses || 0, // Ensure expenses has a default value
      });

      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Missing Fields. Failed to Create Revenue.",
        };
      }

      // Create revenue entry with UUID and timestamps
      const revenue = await prisma.revenue.create({
        data: {
          id: crypto.randomUUID(), // Manual UUID is fine
          ...validated.data,
          // Don't set createdAt/updatedAt here
        },
      });

      return {
        data: revenue as Revenue,
        status: "success",
        message: "Revenue created successfully",
      };
    } catch (error) {
      console.error("Error creating revenue:", error);
      return {
        data: null,
        status: "error",
        message: "Failed to create revenue entry",
      };
    }
  }

  static async update(
    id: string,
    data: UpdateRevenue
  ): Promise<OperationResult<Revenue>> {
    try {
      const validated = UpdateRevenueSchema.safeParse(data);

      if (!validated.success) {
        return {
          data: null,
          status: "error",
          message: "Validation failed",
        };
      }

      const revenue = await prisma.revenue.update({
        where: { id },
        data: {
          ...validated.data,
        },
      });

      return {
        data: revenue as Revenue,
        status: "success",
        message: "Revenue updated successfully",
      };
    } catch (error) {
      console.error("Error updating revenue:", error);
      return {
        data: null,
        status: "error",
        message: "Failed to update revenue entry",
      };
    }
  }

  static async delete(id: string): Promise<OperationResult<Revenue>> {
    try {
      const revenue = await prisma.revenue.delete({
        where: { id },
      });

      return {
        data: revenue as Revenue,
        status: "success",
        message: "Revenue entry deleted successfully",
      };
    } catch (error) {
      console.error("Error deleting revenue:", error);
      return {
        data: null,
        status: "error",
        message: "Failed to delete revenue entry",
      };
    }
  }

  // ===== QUERY OPERATIONS =====

  static async findById(id: string): Promise<OperationResult<Revenue>> {
    try {
      const revenue = await prisma.revenue.findUnique({
        where: { id },
      });

      if (!revenue) {
        return {
          data: null,
          status: "error",
          message: "Revenue entry not found",
        };
      }

      return {
        data: revenue as Revenue,
        status: "success",
        message: "Revenue entry found",
      };
    } catch (error) {
      console.error("Error finding revenue:", error);
      return {
        data: null,
        status: "error",
        message: "Failed to find revenue entry",
      };
    }
  }

  static async findAll(): Promise<OperationResult<Revenue[]>> {
    try {
      const revenues = await prisma.revenue.findMany({
        orderBy: { month: "asc" },
      });

      return {
        data: revenues as Revenue[],
        status: "success",
        message: "Revenue entries retrieved",
      };
    } catch (error) {
      console.error("Error fetching revenues:", error);
      return {
        data: null,
        status: "error",
        message: "Failed to fetch revenue entries",
      };
    }
  }

  static async count(): Promise<OperationResult<number>> {
    try {
      const count = await prisma.revenue.count();
      return {
        data: count,
        status: "success",
        message: "Count retrieved successfully"
      };
    } catch (error) {
      console.error("Error counting revenues:", error);
      return {
        data: null,
        status: "error",
        message: "Failed to retrieve count"
      };
    }
  }

  // ===== DASHBOARD & REPORTING =====

  static async getMonthlyMetrics(
    limit = 12
  ): Promise<OperationResult<RevenueMetrics[]>> {
    try {
      const results = await prisma.revenue.findMany({
        orderBy: { month: "asc" },
        take: limit,
        select: {
          month: true,
          revenue: true,
          expenses: true,
        },
      });

      const metrics = results.map((entry) => ({
        month: entry.month,
        revenue: entry.revenue,
        expenses: entry.expenses,
        profit: entry.revenue - entry.expenses,
      }));
      return {
        data: metrics,
        status: "success",
        message: "Monthly metrics retrieved successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Error getting monthly metrics:",
      };
    }
  }

  static async getTotalRevenue(): Promise<number> {
    try {
      const result = await prisma.revenue.aggregate({
        _sum: { revenue: true },
      });

      return Number(result._sum.revenue || 0);
    } catch (error) {
      console.error("Error calculating total revenue:", error);
      return 0;
    }
  }

  static async getTotalProfit(): Promise<OperationResult<number>> {
    try {
      const result = await prisma.revenue.aggregate({
        _sum: {
          revenue: true,
          expenses: true,
        },
      });

      const totalRevenue = Number(result._sum.revenue || 0);
      const totalExpenses = Number(result._sum.expenses || 0);

      return {
        data: totalRevenue - totalExpenses,
        status: "success",
        message: "Total profit calculated successfully",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: "Error calculating total profit:",
      };
    }
  }
}
