import { describe, it, expect, beforeEach } from "vitest";
import type { Customer } from "@/data/schema/customer";
import type { FormHandler } from "@graphics/schema/form";
import { CustomerForm } from "./customer";

describe("CustomerForm", () => {
  let mockCustomer: Customer;
  let form: CustomerForm;

  beforeEach(() => {
    mockCustomer = {
      id: "123",
      name: "John Doe",
      email: "john@example.com",
      imageUrl: "https://example.com/image.jpg",
      createdAt: new Date(),
      updatedAt: new Date(),
    };
  });

  describe("Create Mode", () => {
    beforeEach(() => {
      form = new CustomerForm();
    });

    it("should generate empty create form", () => {
      const result = form.create();

      // Check operation success
      expect(result.status).toBe("success");
      expect(result.data).toBeDefined();

      if (result.status === "success") {
        const shape = result.data;
        expect(shape.layout.title).toBe("New Customer");
        expect(shape.fields).toHaveLength(3);
        expect(shape.layout.actions).toHaveLength(2);

        // Check default values
        shape.fields.forEach((field) => {
          expect(field.defaultValue).toBe("");
        });
      }
    });

    it("should have correct action buttons", () => {
      const result = form.create();
      expect(result.status).toBe("success");

      if (result.status === "success") {
        const shape = result.data;

        // Check default values
        shape.layout.actions.forEach((action) => {
          switch (action.type) {
            case "submit":
              expect(action.id).toBe("submit");
              expect(action.label).toBe("Create Customer");
              break;
            case "button":
              expect(action.id).toBe("cancel");
              expect(action.label).toBe("Cancel");
              break;
            default:
              console.error("Unknown action type");
              break;
            }
          });
        }
    });
  });

  describe("Edit Mode", () => {
    beforeEach(() => {
      form = new CustomerForm(mockCustomer);
    });

    it("should populate form with customer data", () => {
      const result = form.edit();
      expect(result.status).toBe("success");

      if (result.status === "success") {
        expect(result.data).toBeDefined();
        const shape = result.data;
        expect(shape.layout.title).toBe("Edit Customer");
        expect(shape.fields).toHaveLength(3);
        // Check populated fields
        const nameField = shape.fields.find((f) => f.id === "name");
        expect(nameField?.defaultValue).toBe(mockCustomer.name);

        const emailField = shape.fields.find((f) => f.id === "email");
        expect(emailField?.defaultValue).toBe(mockCustomer.email);

        const imageField = shape.fields.find((f) => f.id === "imageUrl");
        expect(imageField?.defaultValue).toBe(mockCustomer.imageUrl);
      }
    });

    it("should have correct action buttons", () => {
      const result = form.edit();
      expect(result.status).toBe("success");
      if (result.status === "success") {
        const shape = result.data;
        // Check default values
        shape.layout.actions.forEach((action) => {
          switch (action.type) {
            case "submit":
              expect(action.id).toBe("submit");
              expect(action.label).toBe("Save Changes");
              break;
            case "button":
              expect(action.id).toBe("cancel");
              expect(action.label).toBe("Cancel");
              break;
            default:
              console.error("Unknown action type");
              break;
            };
          });
        }
      });
    });

  describe("Form Rendering", () => {
    it("should render as JSX", () => {
      form = new CustomerForm(mockCustomer);
      const result = form.render("edit", "jsx", {} as FormHandler);

      expect(result.status).toBe("success");
      expect(result.data).toBeDefined();
    });

    it("should render as HTML", () => {
      form = new CustomerForm(mockCustomer);
      const result = form.render("edit", "html", {} as FormHandler);

      expect(result.status).toBe("success");
      expect(result.data).toBeDefined();
      expect(typeof result.data).toBe("string");
    });
  });
});
