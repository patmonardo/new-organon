import type { ReactNode } from "react";
import { notFound, redirect } from "next/navigation";
import { revalidatePath } from "next/cache";
import type { FormHandler } from "@graphics/schema/form";
import { InvoiceModel } from "@model/invoice";
import { InvoiceView } from "@view/invoice";
import { CreateInvoice, UpdateInvoice } from "@/data/schema/invoice";
import createInvoice from "@/(controller)/invoices/actions/create";
import updateInvoice from "@/(controller)/invoices/actions/update";
import cancelInvoice from "@/(controller)/invoices/actions/cancel";

export class InvoiceController {
  /**
   * Displays the invoice creation form with breadcrumbs
   */
  static async createForm(): Promise<ReactNode> {
    const view = new InvoiceView();
    const result = await view.display("create", "jsx", {
      submit: createInvoice,
      cancel: cancelInvoice,
    } as FormHandler);

    if (result.status === "error") {
      notFound();
    }

    return <>{result.data}</>;
  }

  /**
   * Renders the invoice edit form with breadcrumbs
   * @param id Invoice ID
   */
  static async editForm(id: string): Promise<ReactNode> {
    // Fetch the invoice data
    const invoiceResult = await InvoiceModel.findById(id);

    if (invoiceResult.status !== "success" || !invoiceResult.data) {
      notFound();
    }

    const invoice = invoiceResult.data;
    const view = new InvoiceView(invoice);

    const formResult = await view.display("edit", "jsx", {
      submit: updateInvoice.bind(null, id),
      cancel: cancelInvoice,
    } as FormHandler);

    if (formResult.status === "error") {
      notFound();
    }

    return <>{formResult.data}</>;
  }

  /**
   * Cancels the invoice creation or editing
   */
  static async cancelInvoice() {
    revalidatePath("/invoices");
    redirect("/invoices");
  }

  static async createInvoice(formData: FormData) {
    try {
      // Extract data from form
      const customerId = formData.get("customerId") as string;
      const amountStr = formData.get("amount") as string;
      const dateStr = formData.get("date") as string;
      const status = formData.get("status") as string;

      // Validate data
      if (!customerId || !amountStr || !dateStr || !status) {
        return {
          error: "All fields are required",
        };
      }

      // Convert and validate numeric/date values
      const amount = Number(amountStr);
      if (isNaN(amount) || amount <= 0) {
        return {
          error: "Amount must be a positive number",
        };
      }

      const date = new Date(dateStr);
      if (isNaN(date.getTime())) {
        return {
          error: "Invalid date format",
        };
      }

      // Create invoice in database
      const result = await InvoiceModel.create({
        customerId,
        amount,
        date,
        status,
      } as CreateInvoice);

      if (result.status !== "success") {
        return {
          error: result.message || "Failed to create invoice",
        };
      }

      // Clear page cache
      revalidatePath("/invoices");
    } catch (error) {
      console.error("Error creating invoice:", error);
      return {
        error: "An unexpected error occurred",
      };
    }
    redirect("/invoices");
  }

  /**
   * Updates a invoice
   * @param id InvoiceCreateInvoice ID
   * @param formData Form data
   */
  static async updateInvoice(id: string, formData: FormData) {
    try {
      // Extract data from form
      const customerId = formData.get("customerId") as string;
      const amountStr = formData.get("amount") as string;
      const dateStr = formData.get("date") as string;
      const status = formData.get("status") as string;

      // Validate required fields
      if (!customerId || !amountStr || !dateStr || !status) {
        return {
          status: "error",
          message: "All fields are required",
        };
      }

      // Convert and validate numbers and dates
      const amount = Number(amountStr);
      const date = new Date(dateStr);

      if (isNaN(amount) || amount <= 0) {
        return {
          status: "error",
          message: "Amount must be a positive number",
        };
      }

      if (isNaN(date.getTime())) {
        return {
          status: "error",
          message: "Invalid date format",
        };
      }

      // Update invoice in database
      const result = await InvoiceModel.update(id, {
        customerId,
        amount,
        date,
        status,
      } as UpdateInvoice);

      if (result.status === "error") {
        return {
          status: "error",
          message: result.message || "Failed to update invoice",
        };
      }

      // Clear page cache
      revalidatePath("/invoices");
    } catch (error) {
      console.error("Error updating invoice:", error);
      return {
        status: "error",
        message: "An unexpected error occurred",
      };
    }
    redirect("/invoices");
  }

  /**
   * Deletes an invoice
   * @param id Invoice ID
   */
  static async deleteInvoice(id: string): Promise<void> {
    // Fetch the customer data
    const result = await InvoiceModel.delete(id);

    if (result.status === "error") {
      notFound();
    }
    if (result.status === "success") {
      redirect("/invoices");
    }
  }

  /**
   * Lists invoices with search and pagination
   * @param query Search query
   * @param page Page number
   * @param pageSize Items per page
   */
  static async listInvoices(
    query = "",
    page = 1,
    pageSize = 10
  ): Promise<ReactNode> {
    const result = await InvoiceModel.findAll({ query, page, pageSize });
    if (result.status !== "success" || !result.data) {
      return (
        <div className="text-center p-4">
          <h2 className="text-xl font-semibold">Error fetching invoice list</h2>
          <p className="text-gray-500">{result.message}</p>
        </div>
      );
    }

    // Create a view to transform domain objects to UI schema
    const view = new InvoiceView();

    const displayResult = await view.displayTable(result.data);

    if (displayResult.status !== "success" || !displayResult.data) {
      return (
        <div className="text-center p-4">
          <h2 className="text-xl font-semibold">Error creating invoice list</h2>
          <p className="text-gray-500">{displayResult.message}</p>
        </div>
      );
    }

    return <>{displayResult.data}</>;
  }

  static async latestInvoices() {
    // Fetch latest invoices with customer data
    const result = await InvoiceModel.getLatestWithCustomers(5);
    if (result.status !== "success") {
      console.error("Error fetching latest invoices:", result.message);
      return null;
    }

    // Create a view to transform domain objects to UI schema
    const view = new InvoiceView();

    // Transform invoices to list schema
    const displayResult = await view.displayLatest(result.data);

    if (displayResult.status !== "success" || !displayResult.data) {
      return (
        <div className="text-center p-4">
          <h2 className="text-xl font-semibold">Error creating invoice list</h2>
          <p className="text-gray-500">{displayResult.message}</p>
        </div>
      );
    }

    return <>{displayResult.data}</>;
  }

  /**
   * Get total pages for pagination
   */
  static async totalPages(pageSize = 10): Promise<number> {
    const result = await InvoiceModel.count();
    if (result.status !== "success" || !result.data) {
      console.error("Error counting invoices:", result.message);
      return 0;
    }
    const totalInvoices = result.data;
    if (totalInvoices === 0) {
      return 0;
    }
    // Calculate total pages
    return Math.ceil(totalInvoices / pageSize);
  }
}
