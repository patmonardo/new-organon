//@graphics/form/invoice.tsx
import type { Invoice } from "@/data/schema/invoice";
import { CustomerModel } from "@model/customer";
import { InvoiceFormShapeSchema } from "@graphics/schema/invoice";
import type { InvoiceFormShape } from "@graphics/schema/invoice";
import { Form } from "./form";

export class InvoiceForm extends Form<InvoiceFormShape> {
  constructor(private readonly invoice?: Invoice) {
    super(invoice);
  }

  private async getFormShape(
    mode: "create" | "edit"
  ): Promise<InvoiceFormShape> {
    const isCreate = mode === "create";

    // Get customer data for the dropdown
    const result = await CustomerModel.findAll();

    if (result.status !== "success") {
      // Handle error case
      throw new Error("Failed to load customers");
    }

    const customers = result.data;

    // Format date properly if it exists
    const formattedDate = this.invoice?.date
      ? new Date(this.invoice.date).toISOString().split("T")[0]
      : "";

    return InvoiceFormShapeSchema.parse({
      layout: {
        title: isCreate ? "Create Invoice" : "Edit Invoice",
        columns: "single",
        sections: [
          {
            title: "Invoice Details",
            fieldIds: ["customerId", "amount", "status", "date"],
          },
        ],
        actions: [
          {
            id: "submit",
            type: "submit",
            label: mode === "create" ? "Create" : "Save Changes",
            variant: "primary",
          },
          {
            id: "cancel",
            label: "Cancel",
            type: "button",
            variant: "secondary",
          },
        ],
      },
      fields: [
        {
          id: "customerId",
          type: "select",
          label: "Customer",
          required: true,
          options: customers.map((c) => ({
            value: c.id,
            label: c.name,
          })),
        },
        {
          id: "amount",
          type: "number",
          label: "Amount ($)",
          required: true,
        },
        {
          id: "status",
          type: "select",
          label: "Status",
          required: true,
          options: [
            { value: "PAID", label: "Paid" },
            { value: "PENDING", label: "Pending" },
            { value: "OVERDUE", label: "Overdue" },
          ],
        },
        {
          id: "date",
          type: "date",
          label: "Date",
          required: true,
          defaultValue: formattedDate, // Set default for edit mode
        },
      ],
      state: {
        status: "idle",
      },
    });
  }

  async createForm(): Promise<InvoiceFormShape> {
    return await this.getFormShape("create");
  }

  async editForm(): Promise<InvoiceFormShape> {
    return await this.getFormShape("edit");
  }
}
