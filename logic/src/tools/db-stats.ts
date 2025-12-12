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
 * Database Statistics Utility
 *
 * Shows current state of the Neo4j database
 */
export async function showDatabaseStats() {
  console.log("📊 Neo4j Database Statistics\n");

  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error("❌ Failed to connect to Neo4j. Exiting.");
    process.exit(1);
  }

  const session = defaultConnection.getSession();
  try {
    // Total nodes
    const nodeCount = await session.run("MATCH (n) RETURN count(n) as count");
    const totalNodes = toNumber(nodeCount.records[0]?.get("count"));
    console.log(`Total Nodes: ${totalNodes}`);

    if (totalNodes === 0) {
      console.log("\n   Database is empty. Run seed script to populate.");
      return;
    }

    // Nodes by label
    console.log("\nNodes by Label:");
    const labelsResult = await session.run(`
      MATCH (n)
      RETURN labels(n)[0] as label, count(n) as count
      ORDER BY count DESC
    `);
    for (const record of labelsResult.records) {
      const label = record.get("label") || "Unknown";
      const count = toNumber(record.get("count"));
      console.log(`   ${label}: ${count}`);
    }

    // Total relationships
    const relCount = await session.run("MATCH ()-[r]->() RETURN count(r) as count");
    const totalRels = toNumber(relCount.records[0]?.get("count"));
    console.log(`\nTotal Relationships: ${totalRels}`);

    if (totalRels > 0) {
      // Relationships by type
      console.log("\nRelationships by Type:");
      const relTypesResult = await session.run(`
        MATCH ()-[r]->()
        RETURN type(r) as type, count(r) as count
        ORDER BY count DESC
      `);
      for (const record of relTypesResult.records) {
        const type = record.get("type");
        const count = toNumber(record.get("count"));
        console.log(`   ${type}: ${count}`);
      }
    }

    // Sample data
    console.log("\nSample Forms:");
    const formsResult = await session.run(`
      MATCH (f:FormShape)
      RETURN f.id as id, f.name as name
      LIMIT 5
    `);
    if (formsResult.records.length === 0) {
      console.log("   No forms found");
    } else {
      for (const record of formsResult.records) {
        console.log(`   - ${record.get("name")} (${record.get("id")})`);
      }
    }

    console.log("\nSample Entities:");
    const entitiesResult = await session.run(`
      MATCH (e:Entity)
      RETURN e.id as id, e.name as name, e.formId as formId
      LIMIT 5
    `);
    if (entitiesResult.records.length === 0) {
      console.log("   No entities found");
    } else {
      for (const record of entitiesResult.records) {
        console.log(`   - ${record.get("name")} (${record.get("id")}) [form: ${record.get("formId")}]`);
      }
    }

  } catch (error) {
    console.error("❌ Error getting database stats:", error);
    throw error;
  } finally {
    await session.close();
    await defaultConnection.close();
  }
}

// Run if called directly
showDatabaseStats()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error("Failed to get database stats:", err);
    process.exit(1);
  });

