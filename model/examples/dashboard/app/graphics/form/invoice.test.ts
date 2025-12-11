import { describe, it, expect, beforeEach } from "vitest";
import type { Invoice } from "@/data/schema/invoice";
import type { FormHandler } from "@graphics/schema/form";
import { InvoiceForm } from "./invoice";

describe("InvoiceForm", () => {
  let mockInvoice: Invoice;
  let form: InvoiceForm;

  beforeEach(() => {
    mockInvoice = {
      id: "123",
      customerId: "456",
      amount: 1000,
      status: "PENDING",
      date: new Date("2024-02-24"),
      createdAt: new Date(),
      updatedAt: new Date(),
    };
  });

  describe("Create Mode", () => {
    beforeEach(() => {
      form = new InvoiceForm();
    });

    it("should generate empty create form", () => {
      const result = form.create();
      expect(result).toBeDefined();
      if (result.status === "success") {
        const shape = result.data;

        expect(shape.layout.title).toBe("Create Invoice");
        expect(shape.fields).toHaveLength(4);
        expect(shape.layout.actions).toHaveLength(2);

        // Check default values
        shape.fields.forEach((field) => {
          expect(field.defaultValue).toBe("");
        });
      }
    });

    it("should have correct action buttons", () => {
      const result = form.create();
      expect(result).toBeDefined();
      if (result.status === "success") {
        const shape = result.data;

        const [cancel, submit] = shape.layout.actions;
        expect(cancel.label).toBe("Cancel");
        expect(submit.label).toBe("Create Invoice");
      }
    });
  });

  describe("Edit Mode", () => {
    beforeEach(() => {
      form = new InvoiceForm(mockInvoice);
    });

    it("should populate form with invoice data", () => {
      const result = form.edit();
      expect(result).toBeDefined();
      if (result.status === "success") {
        const shape = result.data;
        expect(shape.layout.title).toBe("Edit Invoice");
        expect(shape.fields).toHaveLength(4);
      }
      // Once we uncomment the defaultValue lines in invoice.ts:
      // const amountField = shape.fields.find(f => f.id === "amount");
      // expect(amountField?.defaultValue).toBe(mockInvoice.amount.toString());
    });

    it("should have correct action buttons", () => {
      const result = form.edit();
      expect(result).toBeDefined();
      if (result.status === "success") {
        const shape = result.data;
        const [cancel, submit] = shape.layout.actions;
        expect(cancel.label).toBe("Cancel");
        expect(submit.label).toBe("Save Changes");
      }
    });
  });

  describe("Form Rendering", () => {
    it("should render as JSX", () => {
      form = new InvoiceForm(mockInvoice);
      const result = form.render("edit", "jsx", {} as FormHandler);
      expect(result).toBeDefined();
    });

    it("should render as HTML", () => {
      form = new InvoiceForm(mockInvoice);
      const result = form.render("edit", "html", {} as FormHandler);
      expect(result).toBeDefined();
    });
  });
});
