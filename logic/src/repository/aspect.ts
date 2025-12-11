import { Neo4jConnection } from "../connection";
import { AspectShape } from "../schema/aspect";
import { v4 as uuidv4 } from "uuid";

/**
 * AspectShapeRepository
 *
 * Manages the persistence of Aspect Shapes in Neo4j.
 * Aspects represent spectral/relational perspectives - how essences appear in existence.
 */
export class AspectShapeRepository {
  private connection: Neo4jConnection;

  constructor(connection: Neo4jConnection) {
    this.connection = connection;
  }

  /**
   * Save an aspect to Neo4j
   */
  async saveAspect(aspectData: Partial<AspectShape>): Promise<AspectShape> {
    const now = Date.now();
    const aspect: AspectShape = {
      id: aspectData.id || uuidv4(),
      type: aspectData.type || "aspect.unknown",
      name: aspectData.name,
      signature: aspectData.signature,
      facets: aspectData.facets,
      status: aspectData.status,
      tags: aspectData.tags || [],
      meta: aspectData.meta || {},
      createdAt: aspectData.createdAt || now,
      updatedAt: now,
    };

    // Prepare properties for Neo4j
    const props = {
      id: aspect.id,
      type: aspect.type,
      name: aspect.name || null,
      signature: aspect.signature ? JSON.stringify(aspect.signature) : null,
      facets: aspect.facets ? JSON.stringify(aspect.facets) : null,
      status: aspect.status || null,
      meta: aspect.meta ? JSON.stringify(aspect.meta) : null,
      createdAt: aspect.createdAt,
      updatedAt: aspect.updatedAt,
    };

    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (a:Aspect {id: $props.id})
          ON CREATE SET a += $props
          ON MATCH SET a += $props
          RETURN a.id as id
          `,
          { props }
        );

        // Sync tags
        await txc.run(
          `MATCH (a:Aspect {id: $id})-[r:HAS_TAG]->() DELETE r`,
          { id: aspect.id }
        );

        if (aspect.tags && aspect.tags.length > 0) {
          await txc.run(
            `
            UNWIND $tags as tagName
            MATCH (a:Aspect {id: $id})
            MERGE (t:Tag {name: tagName})
            MERGE (a)-[:HAS_TAG]->(t)
            `,
            { id: aspect.id, tags: aspect.tags }
          );
        }
      });

      return aspect;
    } catch (error) {
      console.error(`Error saving aspect: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Get an aspect by ID
   */
  async getAspectById(id: string): Promise<AspectShape | null> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run<{ props: Record<string, any>; tags: string[] }>(
          `
          MATCH (a:Aspect {id: $id})
          OPTIONAL MATCH (a)-[:HAS_TAG]->(t:Tag)
          RETURN properties(a) as props, collect(t.name) as tags
          `,
          { id }
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get("props");
      const tags = result.records[0].get("tags") || [];

      const aspect: AspectShape = {
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

      return aspect;
    } catch (error) {
      console.error(`Error getting aspect by ID (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Find aspects by criteria
   */
  async findAspects(criteria: {
    type?: string;
    tags?: string[];
  } = {}): Promise<AspectShape[]> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
    try {
      const params: Record<string, any> = {};
      let matchClause = `MATCH (a:Aspect)`;
      let whereClauses: string[] = [];

      if (criteria.type) {
        whereClauses.push(`a.type = $type`);
        params.type = criteria.type;
      }

      if (criteria.tags && criteria.tags.length > 0) {
        criteria.tags.forEach((tag, index) => {
          const paramName = `tag${index}`;
          params[paramName] = tag;
          whereClauses.push(
            `EXISTS { MATCH (a)-[:HAS_TAG]->(:Tag {name: $${paramName}}) }`
          );
        });
      }

      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(" AND ")}`;
      }
      cypher += `
        OPTIONAL MATCH (a)-[:HAS_TAG]->(t:Tag)
        RETURN properties(a) as props, collect(t.name) as tags`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const aspects: AspectShape[] = [];
      for (const record of result.records) {
        const rawProps = record.get("props");
        const tags = record.get("tags") || [];

        const aspect: AspectShape = {
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

        aspects.push(aspect);
      }

      return aspects;
    } catch (error) {
      console.error(`Error finding aspects: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Delete an aspect by ID
   */
  async deleteAspect(id: string): Promise<boolean> {
    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      const summary = await session.executeWrite(async (txc) => {
        const result = await txc.run(
          `
          MATCH (a:Aspect {id: $id})
          DETACH DELETE a
          `,
          { id }
        );
        return result.summary;
      });

      const nodesDeleted = summary.counters.updates().nodesDeleted;
      return nodesDeleted > 0;
    } catch (error) {
      console.error(`Error deleting aspect (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }
}
