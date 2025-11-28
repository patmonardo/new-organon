/**
 * Customer MVC Example Tests
 * 
 * Tests the full MVC flow: Model → View → Controller
 */

import { describe, it, expect, beforeEach } from "vitest";
import {
  CustomerModel,
  CustomerController,
  CustomerFormShape,
  type CustomerData,
} from "../src/examples/customer";

describe("Customer MVC Example", () => {
  beforeEach(() => {
    // Clear the mock database before each test
    CustomerModel.clear();
  });

  // ===========================================================================
  // SCHEMA TESTS
  // ===========================================================================

  describe("Schema", () => {
    it("should have correct form shape", () => {
      expect(CustomerFormShape.id).toBe("customer-form");
      expect(CustomerFormShape.name).toBe("customer");
      expect(CustomerFormShape.fields).toHaveLength(3);
    });

    it("should have required fields", () => {
      const nameField = CustomerFormShape.fields.find(f => f.id === "name");
      const emailField = CustomerFormShape.fields.find(f => f.id === "email");
      
      expect(nameField?.required).toBe(true);
      expect(emailField?.required).toBe(true);
    });

    it("should have layout sections in meta", () => {
      const sections = CustomerFormShape.meta?.sections as unknown[];
      expect(sections).toHaveLength(2);
    });

    it("should have actions in meta", () => {
      const actions = CustomerFormShape.meta?.actions as { type: string }[];
      expect(actions).toHaveLength(2);
      expect(actions?.[0].type).toBe("submit");
    });
  });

  // ===========================================================================
  // MODEL TESTS
  // ===========================================================================

  describe("Model", () => {
    it("should create a customer", async () => {
      const result = await CustomerModel.create({
        name: "Alice Smith",
        email: "alice@example.com",
      });

      expect(result.status).toBe("success");
      expect(result.data?.name).toBe("Alice Smith");
      expect(result.data?.email).toBe("alice@example.com");
      expect(result.data?.id).toBeDefined();
    });

    it("should validate required fields on create", async () => {
      const result = await CustomerModel.create({
        name: "",
        email: "alice@example.com",
      });

      expect(result.status).toBe("error");
    });

    it("should update a customer", async () => {
      // Create first
      const createResult = await CustomerModel.create({
        name: "Alice Smith",
        email: "alice@example.com",
      });
      const id = createResult.data!.id!;

      // Update
      const updateResult = await CustomerModel.update(id, {
        name: "Alice Jones",
      });

      expect(updateResult.status).toBe("success");
      expect(updateResult.data?.name).toBe("Alice Jones");
      expect(updateResult.data?.email).toBe("alice@example.com");
    });

    it("should find customer by id", async () => {
      const createResult = await CustomerModel.create({
        name: "Bob Jones",
        email: "bob@example.com",
      });
      const id = createResult.data!.id!;

      const findResult = await CustomerModel.findById(id);

      expect(findResult.status).toBe("success");
      expect(findResult.data?.name).toBe("Bob Jones");
    });

    it("should find all customers", async () => {
      await CustomerModel.create({ name: "Alice", email: "alice@example.com" });
      await CustomerModel.create({ name: "Bob", email: "bob@example.com" });
      await CustomerModel.create({ name: "Carol", email: "carol@example.com" });

      const result = await CustomerModel.findAll();

      expect(result.status).toBe("success");
      expect(result.data).toHaveLength(3);
    });

    it("should filter customers by query", async () => {
      await CustomerModel.create({ name: "Alice Smith", email: "alice@example.com" });
      await CustomerModel.create({ name: "Bob Smith", email: "bob@example.com" });
      await CustomerModel.create({ name: "Carol White", email: "carol@example.com" });

      const result = await CustomerModel.findAll({ query: "Smith" });

      expect(result.status).toBe("success");
      expect(result.data).toHaveLength(2);
    });

    it("should delete a customer", async () => {
      const createResult = await CustomerModel.create({
        name: "Alice",
        email: "alice@example.com",
      });
      const id = createResult.data!.id!;

      const deleteResult = await CustomerModel.delete(id);
      expect(deleteResult.status).toBe("success");

      const findResult = await CustomerModel.findById(id);
      expect(findResult.status).toBe("error");
    });

    it("should count customers", async () => {
      await CustomerModel.create({ name: "Alice", email: "alice@example.com" });
      await CustomerModel.create({ name: "Bob", email: "bob@example.com" });

      const count = await CustomerModel.count();
      expect(count).toBe(2);
    });
  });

  // ===========================================================================
  // CONTROLLER TESTS (Transport-agnostic)
  // ===========================================================================

  describe("Controller", () => {
    it("should create customer via controller", async () => {
      const result = await CustomerController.create({
        name: "Alice Smith",
        email: "alice@example.com",
      });

      expect(result.success).toBe(true);
      expect(result.data?.name).toBe("Alice Smith");
      expect(result.redirect).toBe("/customers");
    });

    it("should validate required fields", async () => {
      const result = await CustomerController.create({
        name: "",
        email: "",
      });

      expect(result.success).toBe(false);
      expect(result.error).toBe("Name and email are required");
    });

    it("should update customer via controller", async () => {
      // Create first
      const createResult = await CustomerController.create({
        name: "Alice Smith",
        email: "alice@example.com",
      });
      const id = createResult.data!.id!;

      // Update
      const updateResult = await CustomerController.update(id, {
        name: "Alice Jones",
      });

      expect(updateResult.success).toBe(true);
      expect(updateResult.data?.name).toBe("Alice Jones");
    });

    it("should delete customer via controller", async () => {
      // Create first
      const createResult = await CustomerController.create({
        name: "Alice",
        email: "alice@example.com",
      });
      const id = createResult.data!.id!;

      // Delete
      const deleteResult = await CustomerController.delete(id);

      expect(deleteResult.success).toBe(true);
      expect(deleteResult.redirect).toBe("/customers");
    });

    it("should list customers via controller", async () => {
      // Seed data
      await CustomerModel.seed();

      const result = await CustomerController.list();

      expect(result.success).toBe(true);
      expect(result.data?.items).toBeDefined();
      expect(result.data?.items.length).toBeGreaterThan(0);
    });

    it("should get customer by id", async () => {
      const createResult = await CustomerController.create({
        name: "Bob",
        email: "bob@example.com",
      });
      const id = createResult.data!.id!;

      const result = await CustomerController.getById(id);

      expect(result.success).toBe(true);
      expect(result.data?.name).toBe("Bob");
    });

    it("should handle form data submission", async () => {
      const formData = new FormData();
      formData.set("name", "Form Test");
      formData.set("email", "form@example.com");

      const result = await CustomerController.handleFormSubmit(formData);

      expect(result.success).toBe(true);
      expect(result.data?.name).toBe("Form Test");
    });
  });

  // ===========================================================================
  // INTEGRATION TESTS
  // ===========================================================================

  describe("Full MVC Flow", () => {
    it("should complete create → list → edit → delete flow", async () => {
      // 1. Create
      const createResult = await CustomerController.create({
        name: "Integration Test",
        email: "test@example.com",
      });
      expect(createResult.success).toBe(true);
      const id = createResult.data!.id!;

      // 2. List
      const listResult = await CustomerController.list();
      expect(listResult.success).toBe(true);
      expect(listResult.data?.items.some(c => c.id === id)).toBe(true);

      // 3. Edit
      const editResult = await CustomerController.update(id, {
        name: "Updated Test",
      });
      expect(editResult.success).toBe(true);
      expect(editResult.data?.name).toBe("Updated Test");

      // 4. Verify edit via getById
      const getResult = await CustomerController.getById(id);
      expect(getResult.data?.name).toBe("Updated Test");

      // 5. Delete
      const deleteResult = await CustomerController.delete(id);
      expect(deleteResult.success).toBe(true);
      expect(deleteResult.redirect).toBe("/customers");

      // 6. Verify delete
      const verifyResult = await CustomerController.getById(id);
      expect(verifyResult.success).toBe(false);
    });
  });
});

