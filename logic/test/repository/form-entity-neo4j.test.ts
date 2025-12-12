import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { FormShapeRepository } from "../../src/repository/form";
import { EntityShapeRepository } from "../../src/repository/entity";
import { defaultConnection } from "../../src/connection";
import { FormShape, EntityShape } from "../../src/schema";

/**
 * Integration tests for Form/Entity persistence layer
 *
 * Tests the Rational (Form) vs Empirical (Entity) separation:
 * - FormShape holds pure structure (keys to the kingdom)
 * - EntityShape holds formId reference + actual values
 * - Form Engine strips state/meta (Rational concerns only)
 * - Entity Engine preserves formId/values/state/meta (Empirical data)
 *
 * Requires: Neo4j running at localhost:7687 (or NEO4J_URI env var)
 */
describe("Form/Entity Neo4j Persistence (Rational vs Empirical)", () => {
  let formRepo: FormShapeRepository;
  let entityRepo: EntityShapeRepository;

  beforeAll(async () => {
    // Verify Neo4j connectivity before running tests
    const connected = await defaultConnection.verifyConnectivity();
    if (!connected) {
      throw new Error(
        "Neo4j not available. Start Neo4j or set NEO4J_URI/NEO4J_USER/NEO4J_PASSWORD env vars."
      );
    }

    formRepo = new FormShapeRepository();
    entityRepo = new EntityShapeRepository();
  });

  afterAll(async () => {
    await defaultConnection.close();
  });

  it("saves and retrieves FormShape (Rational structure only, no state/meta)", async () => {
    // FormShape: Rational form structure (keys to the kingdom)
    const form: FormShape = {
      id: "test-form-1",
      type: "form.CustomerOrder",
      name: "Customer Order",
      title: "New Customer Order",
      description: "Order entry form for customers",
      data: {
        source: "api",
        endpoint: "/api/orders",
        method: "POST",
      },
      fields: [
        {
          id: "customer-name",
          type: "text",
          name: "customerName",
          label: "Customer Name",
          required: true,
          validation: { minLength: 2, maxLength: 100 },
        },
        {
          id: "order-total",
          type: "number",
          name: "orderTotal",
          label: "Order Total",
          required: true,
          validation: { min: 0 },
        },
      ],
      layout: {
        type: "vertical",
        sections: [
          {
            id: "main-section",
            title: "Order Details",
            fields: ["customer-name", "order-total"],
          },
        ],
      },
      tags: ["order", "customer"],
    };

    // Save Form (should NOT persist any state/meta)
    const savedForm = await formRepo.saveForm(form);
    expect(savedForm.id).toBe(form.id);
    expect(savedForm.name).toBe("Customer Order");
    expect(savedForm.fields).toHaveLength(2);

    // Retrieve Form
    const retrievedForm = await formRepo.getFormById(form.id);
    expect(retrievedForm).not.toBeNull();
    expect(retrievedForm?.id).toBe(form.id);
    expect(retrievedForm?.type).toBe("form.CustomerOrder");
    expect(retrievedForm?.name).toBe("Customer Order");
    expect(retrievedForm?.fields).toHaveLength(2);
    expect(retrievedForm?.fields[0].name).toBe("customerName");
    expect(retrievedForm?.tags).toContain("order");

    // Verify Form has NO state/meta (Rational concerns)
    expect(retrievedForm).not.toHaveProperty("state");
    expect(retrievedForm).not.toHaveProperty("meta");
  });

  it("saves and retrieves EntityShape (Empirical data with formId + values)", async () => {
    // First create the Form (Principle)
    const form: FormShape = {
      id: "test-form-2",
      type: "form.ProductRegistration",
      name: "Product Registration",
      title: "Register New Product",
      fields: [
        {
          id: "product-name",
          type: "text",
          name: "productName",
          label: "Product Name",
          required: true,
        },
        {
          id: "price",
          type: "number",
          name: "price",
          label: "Price",
          required: true,
        },
      ],
      layout: {
        type: "vertical",
        sections: [],
      },
    };

    await formRepo.saveForm(form);

    // EntityShape: Empirical instance referencing Form + carrying values
    const entity: EntityShape = {
      id: "test-entity-1",
      type: "entity.ProductInstance",
      formId: "test-form-2", // References Form Principle
      name: "Product Instance #1",
      description: "First product registration",
      values: {
        // Actual field values (Empirical data)
        productName: "Widget Pro",
        price: 29.99,
      },
      signature: {
        createdBy: "user-123",
        version: 1,
      },
      facets: {
        metadata: {
          warehouse: "WH-A",
          sku: "WDG-PRO-001",
        },
      },
      status: "active",
      tags: ["product", "widget"],
      meta: {
        // Runtime metadata (Empirical concerns)
        processedAt: new Date().toISOString(),
        validationStatus: "passed",
      },
    };

    // Save Entity (should persist formId + values + state/meta)
    const savedEntity = await entityRepo.saveEntity(entity);
    expect(savedEntity.id).toBe(entity.id);
    expect(savedEntity.formId).toBe("test-form-2");
    expect(savedEntity.values).toHaveProperty("productName");

    // Retrieve Entity
    const retrievedEntity = await entityRepo.getEntityById(entity.id);
    expect(retrievedEntity).not.toBeNull();
    expect(retrievedEntity?.id).toBe(entity.id);
    expect(retrievedEntity?.type).toBe("entity.ProductInstance");
    expect(retrievedEntity?.formId).toBe("test-form-2"); // Form reference preserved
    expect(retrievedEntity?.values).toEqual({
      productName: "Widget Pro",
      price: 29.99,
    });
    expect(retrievedEntity?.signature?.createdBy).toBe("user-123");
    expect(retrievedEntity?.facets?.metadata?.sku).toBe("WDG-PRO-001");
    expect(retrievedEntity?.status).toBe("active");
    expect(retrievedEntity?.tags).toContain("product");
    expect(retrievedEntity?.meta?.validationStatus).toBe("passed");
  });

  it("retrieves Entity and dereferences Form (Form:Entity reciprocation)", async () => {
    // Create Form
    const form: FormShape = {
      id: "test-form-3",
      type: "form.UserProfile",
      name: "User Profile",
      title: "Edit User Profile",
      fields: [
        {
          id: "username",
          type: "text",
          name: "username",
          label: "Username",
          required: true,
        },
        {
          id: "email",
          type: "email",
          name: "email",
          label: "Email",
          required: true,
        },
      ],
      layout: { type: "vertical", sections: [] },
    };

    await formRepo.saveForm(form);

    // Create Entity referencing Form
    const entity: EntityShape = {
      id: "test-entity-2",
      type: "entity.UserProfileInstance",
      formId: "test-form-3",
      name: "User #42",
      values: {
        username: "johndoe",
        email: "john@example.com",
      },
    };

    await entityRepo.saveEntity(entity);

    // Retrieve Entity
    const retrievedEntity = await entityRepo.getEntityById(entity.id);
    expect(retrievedEntity).not.toBeNull();
    expect(retrievedEntity?.formId).toBe("test-form-3");

    // Dereference Form (Entity Engine → Form Engine message passing)
    const referencedForm = await formRepo.getFormById(retrievedEntity!.formId);
    expect(referencedForm).not.toBeNull();
    expect(referencedForm?.id).toBe("test-form-3");
    expect(referencedForm?.name).toBe("User Profile");
    expect(referencedForm?.fields).toHaveLength(2);

    // Verify Form:Entity reciprocation
    // Form holds structure (keys), Entity holds values (data bound to keys)
    const formFieldNames = referencedForm!.fields.map((f) => f.name);
    const entityValueKeys = Object.keys(retrievedEntity!.values);

    expect(formFieldNames).toContain("username");
    expect(formFieldNames).toContain("email");
    expect(entityValueKeys).toContain("username");
    expect(entityValueKeys).toContain("email");
    expect(retrievedEntity!.values.username).toBe("johndoe");
    expect(retrievedEntity!.values.email).toBe("john@example.com");
  });

  it("finds entities by type filter", async () => {
    // Create multiple entities of same type
    const form: FormShape = {
      id: "test-form-4",
      type: "form.TaskTemplate",
      name: "Task Template",
      fields: [
        {
          id: "task-title",
          type: "text",
          name: "title",
          label: "Task Title",
        },
      ],
      layout: { type: "vertical", sections: [] },
    };

    await formRepo.saveForm(form);

    const entity1: EntityShape = {
      id: "test-entity-3",
      type: "entity.Task",
      formId: "test-form-4",
      name: "Task A",
      values: { title: "Complete documentation" },
    };

    const entity2: EntityShape = {
      id: "test-entity-4",
      type: "entity.Task",
      formId: "test-form-4",
      name: "Task B",
      values: { title: "Write tests" },
    };

    await entityRepo.saveEntity(entity1);
    await entityRepo.saveEntity(entity2);

    // Find by type
    const tasks = await entityRepo.findEntities({ type: "entity.Task" });
    expect(tasks.length).toBeGreaterThanOrEqual(2);

    const taskIds = tasks.map((t) => t.id);
    expect(taskIds).toContain("test-entity-3");
    expect(taskIds).toContain("test-entity-4");

    // Verify each entity has formId + values
    const taskA = tasks.find((t) => t.id === "test-entity-3");
    expect(taskA?.formId).toBe("test-form-4");
    expect(taskA?.values?.title).toBe("Complete documentation");
  });
});
