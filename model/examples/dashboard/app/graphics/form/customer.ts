//@graphics/form/customer.tsx
import type { Customer } from "@/data/schema/customer";
import type { CustomerFormShape } from "@graphics/schema/customer";
import { CustomerFormShapeSchema } from "@graphics/schema/customer";
import { Form } from "@graphics/form/form";

export class CustomerForm extends Form<CustomerFormShape> {
  constructor(private readonly customer?: Customer) {
    super(customer);
  }

  private async getFormShape(
    mode: "create" | "edit"
  ): Promise<CustomerFormShape> {
    const isCreate = mode === "create";

    return CustomerFormShapeSchema.parse({
      layout: {
        title: isCreate ? "New Customer" : "Edit Customer",
        columns: "single",
        sections: [
          {
            title: "Customer Information",
            fieldIds: ["name", "email", "imageUrl"],
          },
        ],
        actions: [
          {
            id: "submit",
            type: "submit",
            label: isCreate ? "Create Customer" : "Save Changes",
            variant: "primary",
          },
          {
            id: "cancel",
            type: "button",
            label: "Cancel",
            variant: "secondary",
          },
        ],
      },
      fields: [
        {
          id: "name",
          type: "text",
          label: "Name",
          required: true,
          defaultValue: isCreate ? "" : this.customer?.name || "",
        },
        {
          id: "email",
          type: "email",
          label: "Email",
          required: true,
          defaultValue: isCreate ? "" : this.customer?.email || "",
        },
        {
          id: "imageUrl",
          type: "text",
          label: "Image URL",
          required: false,
          defaultValue: isCreate ? "" : this.customer?.imageUrl || "",
        },
      ],
      state: {
        status: "idle",
      },
    });
  }

  async createForm(): Promise<CustomerFormShape> {
    return await this.getFormShape("create");
  }

  async editForm(): Promise<CustomerFormShape> {
    return await this.getFormShape("edit");
  }
}
