import { Neo4jConnection } from "./neo4j-client";
import { PropertyShape } from "../schema/property";
import { v4 as uuidv4 } from "uuid";

/**
 * PropertyShapeRepository
 *
 * Manages the persistence of Property Shapes in Neo4j.
 * Properties represent Law/Invariant as Middle Terms mediating Entity ↔ Aspect.
 * Based on Ground, Condition, Facticity, Entity - the conditional genesis of ground:conditions.
 */
export class PropertyShapeRepository {
  private connection: Neo4jConnection;

  constructor(connection: Neo4jConnection) {
    this.connection = connection;
  }

  /**
   * Save a property to Neo4j
   */
  async saveProperty(propertyData: Partial<PropertyShape>): Promise<PropertyShape> {
    const now = Date.now();
    const property: PropertyShape = {
      id: propertyData.id || uuidv4(),
      type: propertyData.type || "property.unknown",
      name: propertyData.name,
      signature: propertyData.signature,
      facets: propertyData.facets,
      status: propertyData.status,
      tags: propertyData.tags || [],
      meta: propertyData.meta || {},
      createdAt: propertyData.createdAt || now,
      updatedAt: now,
    };

    // Prepare properties for Neo4j
    const props = {
      id: property.id,
      type: property.type,
      name: property.name || null,
      signature: property.signature ? JSON.stringify(property.signature) : null,
      facets: property.facets ? JSON.stringify(property.facets) : null,
      status: property.status || null,
      meta: property.meta ? JSON.stringify(property.meta) : null,
      createdAt: property.createdAt,
      updatedAt: property.updatedAt,
    };

    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (p:Property {id: $props.id})
          ON CREATE SET p += $props
          ON MATCH SET p += $props
          RETURN p.id as id
          `,
          { props }
        );

        // Sync tags
        await txc.run(
          `MATCH (p:Property {id: $id})-[r:HAS_TAG]->() DELETE r`,
          { id: property.id }
        );

        if (property.tags && property.tags.length > 0) {
          await txc.run(
            `
            UNWIND $tags as tagName
            MATCH (p:Property {id: $id})
            MERGE (t:Tag {name: tagName})
            MERGE (p)-[:HAS_TAG]->(t)
            `,
            { id: property.id, tags: property.tags }
          );
        }
      });

      return property;
    } catch (error) {
      console.error(`Error saving property: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Get a property by ID
   */
  async getPropertyById(id: string): Promise<PropertyShape | null> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run<{ props: Record<string, any>; tags: string[] }>(
          `
          MATCH (p:Property {id: $id})
          OPTIONAL MATCH (p)-[:HAS_TAG]->(t:Tag)
          RETURN properties(p) as props, collect(t.name) as tags
          `,
          { id }
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get("props");
      const tags = result.records[0].get("tags") || [];

      const property: PropertyShape = {
        id: rawProps.id,
        type: rawProps.type,
        name: rawProps.name || undefined,
        signature: rawProps.signature ? JSON.parse(rawProps.signature) : undefined,
        facets: rawProps.facets ? JSON.parse(rawProps.facets) : undefined,
        status: rawProps.status || undefined,
        tags: tags,
        meta: rawProps.meta ? JSON.parse(rawProps.meta) : undefined,
        createdAt: rawProps.createdAt,
        updatedAt: rawProps.updatedAt,
      };

      return property;
    } catch (error) {
      console.error(`Error getting property by ID (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Find properties by criteria
   */
  async findProperties(criteria: {
    type?: string;
    tags?: string[];
  } = {}): Promise<PropertyShape[]> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
    try {
      const params: Record<string, any> = {};
      let matchClause = `MATCH (p:Property)`;
      let whereClauses: string[] = [];

      if (criteria.type) {
        whereClauses.push(`p.type = $type`);
        params.type = criteria.type;
      }

      if (criteria.tags && criteria.tags.length > 0) {
        criteria.tags.forEach((tag, index) => {
          const paramName = `tag${index}`;
          params[paramName] = tag;
          whereClauses.push(
            `EXISTS { MATCH (p)-[:HAS_TAG]->(:Tag {name: $${paramName}}) }`
          );
        });
      }

      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(" AND ")}`;
      }
      cypher += `
        OPTIONAL MATCH (p)-[:HAS_TAG]->(t:Tag)
        RETURN properties(p) as props, collect(t.name) as tags`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const properties: PropertyShape[] = [];
      for (const record of result.records) {
        const rawProps = record.get("props");
        const tags = record.get("tags") || [];

        const property: PropertyShape = {
          id: rawProps.id,
          type: rawProps.type,
          name: rawProps.name || undefined,
          signature: rawProps.signature ? JSON.parse(rawProps.signature) : undefined,
          facets: rawProps.facets ? JSON.parse(rawProps.facets) : undefined,
          status: rawProps.status || undefined,
          tags: tags,
          meta: rawProps.meta ? JSON.parse(rawProps.meta) : undefined,
          createdAt: rawProps.createdAt,
          updatedAt: rawProps.updatedAt,
        };

        properties.push(property);
      }

      return properties;
    } catch (error) {
      console.error(`Error finding properties: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Delete a property by ID
   */
  async deleteProperty(id: string): Promise<boolean> {
    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      const summary = await session.executeWrite(async (txc) => {
        const result = await txc.run(
          `
          MATCH (p:Property {id: $id})
          DETACH DELETE p
          `,
          { id }
        );
        return result.summary;
      });

      const nodesDeleted = summary.counters.updates().nodesDeleted;
      return nodesDeleted > 0;
    } catch (error) {
      console.error(`Error deleting property (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }
}
