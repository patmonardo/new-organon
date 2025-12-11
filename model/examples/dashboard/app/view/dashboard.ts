//@view/dashboard.ts
import { InvoiceWithCustomer } from '@/data/schema/invoice';

export type DashboardCardData = {
  numberOfCustomers: number;
  numberOfInvoices: number;
  totalPaidInvoices: string;
  totalPendingInvoices: string;
};

export type RevenueData = {
  month: string;
  revenue: number;
};

export class DashboardView {
  // Format card data for display
  static formatCardData(data: DashboardCardData) {
    return {
      collected: {
        title: "Collected",
        value: this.formatCurrency(Number(data.totalPaidInvoices)),
        type: "collected"
      },
      pending: {
        title: "Pending",
        value: this.formatCurrency(Number(data.totalPendingInvoices)),
        type: "pending"
      },
      invoices: {
        title: "Total Invoices",
        value: data.numberOfInvoices.toString(),
        type: "invoices"
      },
      customers: {
        title: "Total Customers",
        value: data.numberOfCustomers.toString(),
        type: "customers"
      }
    };
  }

  // Format revenue data for the chart
  static formatRevenueData(data: RevenueData[]) {
    // Ensure we have data for all months, add missing months with zero revenue
    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
                   'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
    const monthMap = new Map<string, number>();

    // Initialize with zeros
    months.forEach(month => monthMap.set(month, 0));

    // Add actual data
    data.forEach(item => {
      monthMap.set(item.month, item.revenue);
    });

    // Convert back to array format
    return months.map(month => ({
      month,
      revenue: monthMap.get(month) || 0
    }));
  }

  // Format latest invoices for display
  static formatLatestInvoices(invoices: InvoiceWithCustomer[]) {
    return invoices.map(invoice => ({
      ...invoice,
      formattedAmount: this.formatCurrency(Number(invoice.amount)),
      formattedDate: this.formatDate(invoice.date),
      statusColor: invoice.status === 'PAID' ? 'bg-green-500' : 'bg-yellow-500'
    }));
  }

  // Helper methods for formatting
  private static formatCurrency(amount: number): string {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 0,
    }).format(amount);
  }

  private static formatDate(date: Date): string {
    return new Date(date).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric'
    });
  }

}
