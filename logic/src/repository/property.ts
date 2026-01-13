import { Neo4jConnection } from './neo4j-client';
import { PropertyShapeSchema, type PropertyShapeRepo } from '@schema/property';
import { v4 as uuidv4 } from 'uuid';
import neo4j from 'neo4j-driver';

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

  private parseJson<T>(value: unknown): T | undefined {
    if (value === null || value === undefined) return undefined;
    if (typeof value === 'string') {
      try {
        return JSON.parse(value) as T;
      } catch {
        return undefined;
      }
    }
    return value as T;
  }

  private sanitize(input: Partial<PropertyShapeRepo>): PropertyShapeRepo {
    const now = Date.now();
    const withDefaults = {
      id: input.id ?? uuidv4(),
      createdAt: input.createdAt ?? now,
      updatedAt: now,
      ...input,
    } as Partial<PropertyShapeRepo>;
    return PropertyShapeSchema.parse(withDefaults);
  }

  /**
   * Save a property to Neo4j
   */
  async saveProperty(
    propertyData: Partial<PropertyShapeRepo>,
  ): Promise<PropertyShapeRepo> {
    const property = this.sanitize(propertyData);

    const props = {
      id: property.id,
      type: property.type,
      name: property.name ?? null,
      state: property.state ? JSON.stringify(property.state) : null,
      signature: property.signature ? JSON.stringify(property.signature) : null,
      facets: property.facets ? JSON.stringify(property.facets) : null,
      status: property.status ?? null,
      meta: property.meta ? JSON.stringify(property.meta) : null,
      createdAt: property.createdAt,
      updatedAt: property.updatedAt,
    };

    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (p:Property {id: $props.id})
          ON CREATE SET p += $props
          ON MATCH SET p += $props
          RETURN p.id as id
          `,
          { props },
        );

        // Sync tags
        await txc.run(`MATCH (p:Property {id: $id})-[r:HAS_TAG]->() DELETE r`, {
          id: property.id,
        });

        if (property.tags && property.tags.length > 0) {
          await txc.run(
            `
            UNWIND $tags as tagName
            MATCH (p:Property {id: $id})
            MERGE (t:Tag {name: tagName})
            MERGE (p)-[:HAS_TAG]->(t)
            `,
            { id: property.id, tags: property.tags },
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
  async getPropertyById(id: string): Promise<PropertyShapeRepo | null> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run<{ props: Record<string, any>; tags: string[] }>(
          `
          MATCH (p:Property {id: $id})
          OPTIONAL MATCH (p)-[:HAS_TAG]->(t:Tag)
          RETURN properties(p) as props, collect(t.name) as tags
          `,
          { id },
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get('props');
      const tags = result.records[0].get('tags') || [];

      const property = PropertyShapeSchema.parse({
        id: rawProps.id,
        type: rawProps.type,
        name: rawProps.name ?? undefined,
        state: this.parseJson(rawProps.state) ?? {},
        signature: this.parseJson(rawProps.signature),
        facets: this.parseJson(rawProps.facets) ?? {},
        status: rawProps.status ?? undefined,
        tags,
        meta: this.parseJson(rawProps.meta),
        createdAt: neo4j.isInt(rawProps.createdAt)
          ? rawProps.createdAt.toNumber()
          : rawProps.createdAt,
        updatedAt: neo4j.isInt(rawProps.updatedAt)
          ? rawProps.updatedAt.toNumber()
          : rawProps.updatedAt,
      });

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
  async findProperties(
    criteria: {
      type?: string;
      tags?: string[];
    } = {},
  ): Promise<PropertyShapeRepo[]> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
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
            `EXISTS { MATCH (p)-[:HAS_TAG]->(:Tag {name: $${paramName}}) }`,
          );
        });
      }

      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(' AND ')}`;
      }
      cypher += `
        OPTIONAL MATCH (p)-[:HAS_TAG]->(t:Tag)
        RETURN properties(p) as props, collect(t.name) as tags`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const properties: PropertyShapeRepo[] = [];
      for (const record of result.records) {
        const rawProps = record.get('props');
        const tags = record.get('tags') || [];

        const property = PropertyShapeSchema.parse({
          id: rawProps.id,
          type: rawProps.type,
          name: rawProps.name ?? undefined,
          state: this.parseJson(rawProps.state) ?? {},
          signature: this.parseJson(rawProps.signature),
          facets: this.parseJson(rawProps.facets) ?? {},
          status: rawProps.status ?? undefined,
          tags,
          meta: this.parseJson(rawProps.meta),
          createdAt: neo4j.isInt(rawProps.createdAt)
            ? rawProps.createdAt.toNumber()
            : rawProps.createdAt,
          updatedAt: neo4j.isInt(rawProps.updatedAt)
            ? rawProps.updatedAt.toNumber()
            : rawProps.updatedAt,
        });

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
    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      const summary = await session.executeWrite(async (txc) => {
        const result = await txc.run(
          `
          MATCH (p:Property {id: $id})
          DETACH DELETE p
          `,
          { id },
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
