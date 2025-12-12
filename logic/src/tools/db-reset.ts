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
 * Database Reset Utility
 *
 * Safely wipes the Neo4j database and provides statistics
 */
export async function resetDatabase() {
  console.log("🔄 Resetting Neo4j database...");

  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error("❌ Failed to connect to Neo4j. Exiting.");
    process.exit(1);
  }

  const session = defaultConnection.getSession();
  try {
    // Get count before deletion
    const beforeResult = await session.run("MATCH (n) RETURN count(n) as count");
    const beforeCount = toNumber(beforeResult.records[0]?.get("count"));

    // Get relationship count
    const relResult = await session.run("MATCH ()-[r]->() RETURN count(r) as count");
    const relCount = toNumber(relResult.records[0]?.get("count"));

    console.log(`   Found ${beforeCount} nodes and ${relCount} relationships`);

    // Wipe everything
    console.log("   Deleting all nodes and relationships...");
    const deleteResult = await session.run("MATCH (n) DETACH DELETE n RETURN count(n) as deleted");
    const deletedCount = toNumber(deleteResult.records[0]?.get("deleted"));

    console.log(`   ✅ Deleted ${deletedCount} nodes`);
    console.log("✨ Database reset complete!");
  } catch (error) {
    console.error("❌ Error resetting database:", error);
    throw error;
  } finally {
    await session.close();
    await defaultConnection.close();
  }
}

// Run if called directly
resetDatabase()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error("Database reset failed:", err);
    process.exit(1);
  });

