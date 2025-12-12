import { defaultConnection } from "../repository/neo4j-client";
import neo4j from "neo4j-driver";

/**
 * Helper to convert Neo4j Integer to number
 */
function toNumber(value: any): number {
  if (value === null || value === undefined) return 0;
  if (neo4j.isInt(value)) {
    return value.toNumber();
  }
  return Number(value) || 0;
}

/**
 * Database Query Explorer
 * 
 * Run useful queries to explore your data
 */
export async function exploreDatabase() {
  console.log("🔍 Database Query Explorer\n");

  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error("❌ Failed to connect to Neo4j. Exiting.");
    process.exit(1);
  }

  const session = defaultConnection.getSession();
  try {
    // === 1. Forms Overview ===
    console.log("📋 Forms Overview:");
    const formsQuery = await session.run(`
      MATCH (f:FormShape)
      OPTIONAL MATCH (f)-[:HAS_FIELD]->(field:FormField)
      RETURN f.id as id, f.name as name, f.title as title, count(field) as fieldCount
      ORDER BY f.name
    `);
    
    if (formsQuery.records.length === 0) {
      console.log("   No forms found\n");
    } else {
      for (const record of formsQuery.records) {
        const name = record.get("name") || "Unnamed";
        const title = record.get("title") || "";
        const fieldCount = toNumber(record.get("fieldCount"));
        console.log(`   • ${name}${title ? ` (${title})` : ""} - ${fieldCount} fields [${record.get("id")}]`);
      }
      console.log();
    }

    // === 2. Entities by Form ===
    console.log("👥 Entities by Form:");
    const entitiesByForm = await session.run(`
      MATCH (e:Entity)
      RETURN e.formId as formId, count(e) as count, collect(e.name)[0..3] as sampleNames
      ORDER BY count DESC
    `);
    
    if (entitiesByForm.records.length === 0) {
      console.log("   No entities found\n");
    } else {
      for (const record of entitiesByForm.records) {
        const formId = record.get("formId");
        const count = toNumber(record.get("count"));
        const samples = record.get("sampleNames").filter((n: any) => n).join(", ");
        console.log(`   • ${formId}: ${count} entities`);
        if (samples) {
          console.log(`     Examples: ${samples}${count > 3 ? "..." : ""}`);
        }
      }
      console.log();
    }

    // === 3. Relationships ===
    console.log("🔗 Relationships:");
    const relationships = await session.run(`
      MATCH (a)-[r]->(b)
      RETURN type(r) as type, labels(a)[0] as fromLabel, labels(b)[0] as toLabel, count(r) as count
      ORDER BY count DESC
    `);
    
    if (relationships.records.length === 0) {
      console.log("   No relationships found\n");
    } else {
      for (const record of relationships.records) {
        const type = record.get("type");
        const from = record.get("fromLabel");
        const to = record.get("toLabel");
        const count = toNumber(record.get("count"));
        console.log(`   • ${from} -[${type}]-> ${to}: ${count}`);
      }
      console.log();
    }

    // === 4. Entity Details (with relationships) ===
    console.log("📊 Entity Details:");
    const entityDetails = await session.run(`
      MATCH (e:Entity)
      OPTIONAL MATCH (e)-[r]->(related)
      WITH e, count(r) as relCount, collect(DISTINCT type(r))[0..3] as relTypes
      RETURN e.id as id, e.name as name, e.formId as formId, e.status as status, relCount, relTypes
      ORDER BY relCount DESC
      LIMIT 5
    `);
    
    if (entityDetails.records.length === 0) {
      console.log("   No entities found\n");
    } else {
      for (const record of entityDetails.records) {
        const name = record.get("name") || record.get("id");
        const formId = record.get("formId");
        const status = record.get("status");
        const relCount = toNumber(record.get("relCount"));
        const relTypes = record.get("relTypes").filter((t: any) => t);
        console.log(`   • ${name} [${formId}]${status ? ` (${status})` : ""}`);
        if (relCount > 0) {
          console.log(`     → ${relCount} relationships: ${relTypes.join(", ")}`);
        }
      }
      console.log();
    }

    // === 5. Form Structure Sample ===
    console.log("🏗️  Form Structure Sample (first form):");
    const formStructure = await session.run(`
      MATCH (f:FormShape)
      OPTIONAL MATCH (f)-[:HAS_FIELD]->(field:FormField)
      OPTIONAL MATCH (f)-[:HAS_LAYOUT]->(layout:FormLayout)
      OPTIONAL MATCH (layout)-[:HAS_SECTION]->(section:FormSection)
      WITH f, collect(DISTINCT field.id)[0..5] as fields, count(DISTINCT section) as sectionCount
      RETURN f.id as id, f.name as name, fields, sectionCount
      LIMIT 1
    `);
    
    if (formStructure.records.length > 0) {
      const record = formStructure.records[0];
      const name = record.get("name") || "Unnamed";
      const fields = record.get("fields").filter((f: any) => f);
      const sections = toNumber(record.get("sectionCount"));
      console.log(`   Form: ${name} [${record.get("id")}]`);
      console.log(`   Fields: ${fields.length > 0 ? fields.join(", ") : "none"}`);
      console.log(`   Sections: ${sections}`);
    } else {
      console.log("   No forms found");
    }
    console.log();

  } catch (error) {
    console.error("❌ Error exploring database:", error);
    throw error;
  } finally {
    await session.close();
    await defaultConnection.close();
  }
}

// Run if called directly
exploreDatabase()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error("Failed to explore database:", err);
    process.exit(1);
  });

