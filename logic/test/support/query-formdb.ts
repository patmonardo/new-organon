/**
 * Query FormDB - inspect persisted Form/Entity data
 *
 * Simple CLI to query Neo4j and verify Form:Entity reciprocation
 *
 * Usage: pnpm tsx test/support/query-formdb.ts
 */

import { FormShapeRepository } from "../../src/repository/form";
import { EntityShapeRepository } from "../../src/repository/entity";
import { defaultConnection } from "../../src/connection";

async function query() {
  console.log("🔍 Querying FormDB...\n");

  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error("❌ Neo4j not available");
    process.exit(1);
  }

  const formRepo = new FormShapeRepository(defaultConnection);
  const entityRepo = new EntityShapeRepository(defaultConnection);

  // === Query Forms ===
  console.log("📝 Forms (Rational structures):");
  console.log("─".repeat(60));

  const orderForm = await formRepo.getFormById("form-customer-order");
  if (orderForm) {
    console.log(`\n${orderForm.id}`);
    console.log(`  Name: ${orderForm.name}`);
    console.log(`  Title: ${orderForm.title}`);
    console.log(`  Fields: ${orderForm.fields.length}`);
    orderForm.fields.forEach((f) => {
      console.log(`    - ${f.name} (${f.label})`);
    });
    console.log(`  Tags: ${orderForm.tags?.map((t) => t.label || t).join(", ")}`);
  }

  const productForm = await formRepo.getFormById("form-product-registration");
  if (productForm) {
    console.log(`\n${productForm.id}`);
    console.log(`  Name: ${productForm.name}`);
    console.log(`  Title: ${productForm.title}`);
    console.log(`  Fields: ${productForm.fields.length}`);
    productForm.fields.forEach((f) => {
      console.log(`    - ${f.name} (${f.label})`);
    });
  }

  // === Query Entities ===
  console.log("\n\n📦 Entities (Empirical instances with values):");
  console.log("─".repeat(60));

  const orderEntity = await entityRepo.getEntityById("entity-order-001");
  if (orderEntity) {
    console.log(`\n${orderEntity.id}`);
    console.log(`  Type: ${orderEntity.type}`);
    console.log(`  FormId: ${orderEntity.formId} ← references Form Principle`);
    console.log(`  Name: ${orderEntity.name}`);
    console.log(`  Values:`, JSON.stringify(orderEntity.values, null, 4));
    console.log(`  Status: ${orderEntity.status}`);
    console.log(`  Tags: ${orderEntity.tags?.join(", ")}`);
    console.log(`  Has meta: ${!!orderEntity.meta ? "✅ YES (Empirical runtime data)" : "NO"}`);
  }

  const productEntity = await entityRepo.getEntityById("entity-product-001");
  if (productEntity) {
    console.log(`\n${productEntity.id}`);
    console.log(`  Type: ${productEntity.type}`);
    console.log(`  FormId: ${productEntity.formId} ← references Form Principle`);
    console.log(`  Name: ${productEntity.name}`);
    console.log(`  Values:`, JSON.stringify(productEntity.values, null, 4));
    console.log(`  Status: ${productEntity.status}`);
    console.log(`  Has meta: ${!!productEntity.meta ? "✅ YES (Empirical runtime data)" : "NO"}`);
  }

  // === Form:Entity Reciprocation ===
  console.log("\n\n🔄 Form:Entity Reciprocation Test:");
  console.log("─".repeat(60));

  if (orderEntity && orderForm) {
    console.log(`\nEntity "${orderEntity.id}" → Form "${orderEntity.formId}"`);
    console.log("\nForm defines fields (keys to the kingdom):");
    const formFieldNames = orderForm.fields.map((f) => f.name);
    formFieldNames.forEach((name) => console.log(`  - ${name}`));

    console.log("\nEntity carries values (data bound to keys):");
    const entityValueKeys = Object.keys(orderEntity.values);
    entityValueKeys.forEach((key) => {
      const value = orderEntity.values[key];
      console.log(`  - ${key}: ${JSON.stringify(value)}`);
    });

    console.log("\nValidation:");
    const allKeysValid = entityValueKeys.every((key) => formFieldNames.includes(key));
    console.log(`  ${allKeysValid ? "✅" : "❌"} All entity value keys match form field names`);
  }

  console.log("\n✨ Query complete\n");

  await defaultConnection.close();
}

query().catch((error) => {
  console.error("❌ Query failed:", error);
  process.exit(1);
});
