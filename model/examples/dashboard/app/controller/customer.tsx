import type { ReactNode } from "react";
import { notFound, redirect } from "next/navigation";
import { revalidatePath } from "next/cache";
import type { FormHandler } from "@graphics/schema/form";
import { CreateCustomer, UpdateCustomer } from "@/data/schema/customer";
import { CustomerModel } from "@model/customer";
import { CustomerView } from "@view/customer";
import createCustomer from "@/(controller)/customers/actions/create";
import updateCustomer from "@/(controller)/customers/actions/update";
import cancelCustomer from "@/(controller)/customers/actions/cancel";

export class CustomerController {
  /**
   * Displays the customer creation form with breadcrumbs
   */
  static async createForm(): Promise<ReactNode> {
    const view = new CustomerView();
    const result = await view.display("create", "jsx", {
      submit: createCustomer,
      cancel: cancelCustomer,
    } as FormHandler);

    if (result.status === "error") {
      notFound();
    }

    return <>{result.data}</>;
  }

  /**
   * Renders the customer edit form with breadcrumbs
   * @param id Customer ID
   */
  static async editForm(id: string): Promise<ReactNode> {
    // Fetch the customer data
    const customerResult = await CustomerModel.findById(id);

    if (customerResult.status !== "success" || !customerResult.data) {
      notFound();
    }

    const customer = customerResult.data;
    const view = new CustomerView(customer);

    const formResult = await view.display("edit", "jsx", {
      submit: updateCustomer.bind(null, id),
      cancel: cancelCustomer,
    } as FormHandler);

    if (formResult.status === "error") {
      notFound();
    }

    return <>{formResult.data}</>;
  }

  /**
   * Cancels the customer creation or editing
   */
  static async cancelCustomer() {
    revalidatePath("/customers");
    redirect("/customers");
  }

  static async createCustomer(formData: FormData) {
    try {
      // Extract data from form
      const name = formData.get("name") as string;
      const email = formData.get("email") as string;
      const imageUrl = (formData.get("imageUrl") as string) || null;

      // Validate data
      if (!name || !email) {
        return {
          error: "Name and email are required",
        };
      }

      // Create customer in database
      const result = await CustomerModel.create({
        name,
        email,
        imageUrl,
      } as CreateCustomer);

      if (result.status !== "success") {
        return {
          error: result.message || "Failed to create customer",
        };
      }

      // Clear page cache
      revalidatePath("/customers");
    } catch (error) {
      console.error("Error creating customer:", error);
      return {
        error: "An unexpected error occurred",
      };
    }

    // Redirect to customer list
    redirect("/customers");
  }

  /**
   * Updates a customer
   * @param id Customer ID
   * @param formData Form data
   */
  static async updateCustomer(id: string, formData: FormData) {
    try {
      // Extract data from the form
      const name = formData.get("name") as string;
      const email = formData.get("email") as string;
      const imageUrl = (formData.get("imageUrl") as string) || null;

      // Update in database
      const result = await CustomerModel.update(id, {
        name,
        email,
        imageUrl,
      } as UpdateCustomer);

      if (result.status !== "success") {
        return {
          error: result.message || "Failed to update customer",
        };
      }

      // Clear page cache
      revalidatePath("/customers");
    } catch (error) {
      console.error("Error updating customer:", error);
      return {
        error: "An unexpected error occurred",
      };
    }

    // Redirect back to list
    redirect("/customers");
  }

  /**
   * Deletes a customer
   * @param id Customer ID
   */
  static async deleteCustomer(id: string): Promise<void> {
    // Fetch the customer data
    const result = await CustomerModel.delete(id);

    if (result.status === "error") {
      notFound();
    }
    if (result.status === "success") {
      redirect("/customers");
    }
  }

  /**
   * Lists customers with search and pagination
   * @param query Search query
   * @param page Page number
   * @param pageSize Items per page
   */
  static async listCustomers(query = "", page = 1): Promise<ReactNode> {
    // Fetch data with pagination and search
    const result = await CustomerModel.findAll({ query, page, pageSize: 10 });

    if (result.status !== "success") {
      return (
        <div className="text-center p-4">
          <h2 className="text-xl font-semibold">Error</h2>
          <p className="text-gray-500">{result.message}</p>
        </div>
      );
    }

    // Get total pages
    const totalPages = await this.totalPages();

    // Use the view to display the table
    const view = new CustomerView();
    const displayResult = await view.displayTable(result.data, totalPages);
    if (displayResult.status === "error") {
      return (
        <div className="text-center p-4">
          <h2 className="text-xl font-semibold">Error</h2>
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
    const totalCustomers = await CustomerModel.count();
    return Math.ceil(totalCustomers / pageSize);
  }
}
