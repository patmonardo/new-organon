import { describe, it, expect } from "vitest";
import { v4 as uuidv4 } from "uuid";
import { RevenueSchema } from "./revenue";

describe("Revenue Schema", () => {
  it("should validate a valid revenue", () => {
    const validRevenue = {
      id: uuidv4(),
      month: new Date(),
      revenue: 100,
      expenses: 50,
      createdAt: new Date(),
      updatedAt: new Date()
    };
    const result = RevenueSchema.safeParse(validRevenue);
    expect(result.success).toBe(true);
  });

  it("should invalidate a revenue with invalid id", () => {
    const invalidRevenue = {
      id: "invalid",
      month: new Date(),
      revenue: 100,
      expenses: 50,
    };
    const result = RevenueSchema.safeParse(invalidRevenue);
    expect(result.success).toBe(false);
  });

  it("should invalidate a revenue with negative revenue", () => {
    const invalidRevenue = {
      id: uuidv4(),
      month: new Date(),
      revenue: -100,
      expenses: 50,
    };
    const result = RevenueSchema.safeParse(invalidRevenue);
    expect(result.success).toBe(false);
  });

  it("should invalidate a revenue with negative expenses", () => {
    const invalidRevenue = {
      id: "uuid",
      month: new Date(),
      revenue: 100,
      expenses: -50,
    };
    const result = RevenueSchema.safeParse(invalidRevenue);
    expect(result.success).toBe(false);
  });
});
