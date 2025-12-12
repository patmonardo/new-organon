/**
 * Seed FormDB with test data
 *
 * Simple script to populate Neo4j with Form and Entity samples
 * for testing the Rational/Empirical distinction.
 *
 * Usage: pnpm tsx test/support/seed-formdb.ts
 */

import { FormShapeRepository } from "../../src/repository/form";
import { EntityShapeRepository } from "../../src/repository/entity";
import { defaultConnection } from "../../src/connection";
import { FormShape, EntityShape } from "../../src/schema";

async function seed() {
  console.log("🌱 Seeding FormDB...\n");

  // Check Neo4j connectivity
  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error("❌ Neo4j not available. Check connection settings.");
    process.exit(1);
  }

  console.log("✅ Neo4j connected\n");

  const formRepo = new FormShapeRepository(defaultConnection);
  const entityRepo = new EntityShapeRepository(defaultConnection);

  // === FORM 1: Customer Order Form ===
  console.log("📝 Creating Customer Order Form...");

  const orderForm: FormShape = {
    id: "form-customer-order",
    name: "Customer Order",
    title: "New Customer Order",
    description: "Order entry form for customers",
    fields: [
      {
        id: "customer-name",
        name: "customerName",
        label: "Customer Name",
        required: true,
      },
      {
        id: "order-total",
        name: "orderTotal",
        label: "Order Total ($)",
        required: true,
      },
      {
        id: "order-notes",
        name: "notes",
        label: "Order Notes",
        required: false,
      },
    ],
    layout: {
      id: "main-layout",
      sections: [
        {
          id: "customer-section",
          title: "Customer Information",
          fields: ["customer-name"],
        },
        {
          id: "order-section",
          title: "Order Details",
          fields: ["order-total", "order-notes"],
        },
      ],
    },
    tags: [
      { value: "order", label: "Order" },
      { value: "customer", label: "Customer" },
    ],
  };

  const savedOrderForm = await formRepo.saveForm(orderForm);
  console.log(`   ✓ Saved Form: ${savedOrderForm.id}`);

  // === ENTITY 1: Order Instance ===
  console.log("📦 Creating Order Entity (Entity references Form)...");

  const orderEntity: EntityShape = {
    id: "entity-order-001",
    type: "entity.Order",
    formId: "form-customer-order", // References Form Principle
    name: "Order #001",
    description: "First customer order",
    values: {
      // Empirical field values
      customerName: "Alice Johnson",
      orderTotal: 249.99,
      notes: "Priority shipping requested",
    },
    signature: {
      createdBy: "user-alice",
      version: 1,
    },
    facets: {
      shipping: {
        priority: true,
        address: "123 Main St",
      },
    },
    status: "pending",
    tags: ["urgent"],
    meta: {
      processedAt: new Date().toISOString(),
      source: "web-app",
    },
  };

  const savedOrderEntity = await entityRepo.saveEntity(orderEntity);
  console.log(`   ✓ Saved Entity: ${savedOrderEntity.id} (formId: ${savedOrderEntity.formId})`);

  // === FORM 2: Product Registration ===
  console.log("\n📝 Creating Product Registration Form...");

  const productForm: FormShape = {
    id: "form-product-registration",
    name: "Product Registration",
    title: "Register New Product",
    description: "Product catalog entry form",
    fields: [
      {
        id: "product-name",
        name: "productName",
        label: "Product Name",
        required: true,
      },
      {
        id: "sku",
        name: "sku",
        label: "SKU",
        required: true,
      },
      {
        id: "price",
        name: "price",
        label: "Price ($)",
        required: true,
      },
      {
        id: "category",
        name: "category",
        label: "Category",
        required: false,
      },
    ],
    layout: {
      id: "product-layout",
      sections: [
        {
          id: "basic-info",
          title: "Basic Information",
          fields: ["product-name", "sku"],
        },
        {
          id: "pricing",
          title: "Pricing & Category",
          fields: ["price", "category"],
        },
      ],
    },
    tags: [
      { value: "product", label: "Product" },
      { value: "catalog", label: "Catalog" },
    ],
  };

  const savedProductForm = await formRepo.saveForm(productForm);
  console.log(`   ✓ Saved Form: ${savedProductForm.id}`);

  // === ENTITY 2: Product Instance ===
  console.log("📦 Creating Product Entity...");

  const productEntity: EntityShape = {
    id: "entity-product-001",
    type: "entity.Product",
    formId: "form-product-registration",
    name: "Product: Widget Pro",
    values: {
      productName: "Widget Pro",
      sku: "WDG-PRO-001",
      price: 29.99,
      category: "Electronics",
    },
    signature: {
      createdBy: "user-bob",
      version: 1,
    },
    facets: {
      inventory: {
        warehouse: "WH-A",
        quantity: 150,
      },
    },
    status: "active",
    tags: ["electronics", "featured"],
  };

  const savedProductEntity = await entityRepo.saveEntity(productEntity);
  console.log(`   ✓ Saved Entity: ${savedProductEntity.id} (formId: ${savedProductEntity.formId})`);

  // === Summary ===
  console.log("\n✨ Seeding complete!\n");
  console.log("Summary:");
  console.log("  - Forms created: 2 (Rational structures)");
  console.log("  - Entities created: 2 (Empirical instances with formId references)");
  console.log("\nForm:Entity pairs:");
  console.log(`  1. ${orderForm.id} → ${orderEntity.id}`);
  console.log(`  2. ${productForm.id} → ${productEntity.id}`);
  console.log("\nTest the persistence:");
  console.log("  pnpm test test/repository/form-entity-neo4j.test.ts");

  await defaultConnection.close();
}

// Run seed
seed().catch((error) => {
  console.error("❌ Seed failed:", error);
  process.exit(1);
});
