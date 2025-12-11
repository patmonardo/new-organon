//@model/dashboard.ts
import type { InvoiceWithCustomer } from "@/data/schema/invoice";
import type { RevenueMetrics } from "@/data/schema/revenue";
import { CustomerModel } from "@model/customer";
import { InvoiceModel } from "@model/invoice";
import { RevenueModel } from "@model/revenue";

// Type definitions for the data we'll be returning
type CardData = {
  numberOfCustomers: number;
  numberOfInvoices: number;
  totalPaidInvoices: string;
  totalPendingInvoices: string;
};

export class DashboardModel {
  // Method to fetch card data
  static async getCardData(): Promise<CardData> {
    try {
      const numberOfCustomers = await CustomerModel.count();
      const numberOfInvoices = 1 // await InvoiceModel.count();
      const totalPaidInvoices = await InvoiceModel.getTotalByStatus("PAID");
      const totalPendingInvoices = await InvoiceModel.getTotalByStatus("PENDING");

      if (totalPaidInvoices.status !== "success" || !totalPaidInvoices.data) {
        throw new Error(totalPaidInvoices.message || "Failed to get paid invoices");
      }
      if (totalPendingInvoices.status !== "success" || !totalPendingInvoices.data) {
        throw new Error(totalPendingInvoices.message || "Failed to get pending invoices");
      }
      const totalPaidInvoicesString = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
      }).format(totalPaidInvoices.data);
      const totalPendingInvoicesString = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
      }).format(totalPendingInvoices.data);
      return {
        numberOfCustomers,
        numberOfInvoices,
        totalPaidInvoices: totalPaidInvoicesString,
        totalPendingInvoices: totalPendingInvoicesString,
      };
    } catch (error) {
      console.error("Error fetching card data:", error);
      return {
        numberOfCustomers: 0,
        numberOfInvoices: 0,
        totalPaidInvoices: "0",
        totalPendingInvoices: "0",
      };
    }
  }

  static async getLatestInvoices(): Promise<InvoiceWithCustomer[]> {
    try {
      const result = await InvoiceModel.getLatestWithCustomers(5);

      if (result.status !== "success" || !result.data) {
        console.error("Error fetching latest invoices:", result.message);
        return [];
      }

      return result.data;
    } catch (error) {
      console.error("Error fetching latest invoices:", error);
      return [];
    }
  }

  static async getRevenue(): Promise<RevenueMetrics[]> {
    try {
      const result = await RevenueModel.getMonthlyMetrics(12);

      if (result.status !== "success" || !result.data) {
        throw new Error(result.message || "Failed to get revenue data");
      }

      return result.data;
    } catch (error) {
      console.error("Error fetching revenue data:", error);
      // Return placeholder data if the query fails
      const months = [
        new Date("Jan"),
        new Date("Feb"),
        new Date("Mar"),
        new Date("Apr"),
        new Date("May"),
        new Date("Jun"),
        new Date("Jul"),
        new Date("Aug"),
        new Date("Sep"),
        new Date("Oct"),
        new Date("Nov"),
        new Date("Dec"),
      ];
      return months.map((month) => ({
        month,
        revenue: Math.floor(Math.random() * 5000) + 1000,
        expenses: 0,
        profit: 0,
      }));
    }
  }
}
