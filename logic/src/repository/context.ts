import { Neo4jConnection } from "../connection";
import { ContextShape } from "../schema/context";
import { v4 as uuidv4 } from "uuid";

/**
 * ContextShapeRepository
 *
 * Manages the persistence of Context Shapes in Neo4j.
 * Contexts define the scope and conditions under which dialectical logic operates.
 */
export class ContextRepository {
  private connection: Neo4jConnection;

  constructor(connection: Neo4jConnection) {
    this.connection = connection;
  }

  /**
   * Save a context to Neo4j
   */
  async saveContext(contextData: Partial<ContextShape>): Promise<ContextShape> {
    const now = Date.now();

    // Build context with defaults
    const context: ContextShape = {
      core: {
        id: contextData.core?.id || uuidv4(),
        type: contextData.core?.type || "context.unknown",
        name: contextData.core?.name,
        description: contextData.core?.description,
        createdAt: contextData.core?.createdAt || new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
      state: contextData.state || {},
      entities: contextData.entities || [],
      relations: contextData.relations || [],
      signature: contextData.signature,
      facets: contextData.facets || {},
    };

    // Prepare properties for Neo4j
    const props = {
      id: context.core.id,
      type: context.core.type,
      name: context.core.name || null,
      description: context.core.description || null,
      state: JSON.stringify(context.state),
      entities: JSON.stringify(context.entities),
      relations: JSON.stringify(context.relations),
      signature: context.signature ? JSON.stringify(context.signature) : null,
      facets: JSON.stringify(context.facets),
    };

    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (c:Context {id: $props.id})
          ON CREATE SET c += $props
          ON MATCH SET c += $props
          RETURN c.id as id
          `,
          { props }
        );
      });

      return context;
    } catch (error) {
      console.error(`Error saving context: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Get a context by ID
   */
  async getContextById(id: string): Promise<ContextShape | null> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run(
          `
          MATCH (c:Context {id: $id})
          RETURN properties(c) as props
          `,
          { id }
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get("props");

      const context: ContextShape = {
        core: {
          id: rawProps.id,
          type: rawProps.type,
          name: rawProps.name || undefined,
          description: rawProps.description || undefined,
          createdAt: rawProps.createdAt,
          updatedAt: rawProps.updatedAt,
        },
        state: rawProps.state ? JSON.parse(rawProps.state) : {},
        entities: rawProps.entities ? JSON.parse(rawProps.entities) : [],
        relations: rawProps.relations ? JSON.parse(rawProps.relations) : [],
        signature: rawProps.signature ? JSON.parse(rawProps.signature) : undefined,
        facets: rawProps.facets ? JSON.parse(rawProps.facets) : {},
      };

      return context;
    } catch (error) {
      console.error(`Error getting context by ID (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Find contexts by criteria
   */
  async findContexts(criteria: {
    type?: string;
  } = {}): Promise<ContextShape[]> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
    try {
      const params: Record<string, any> = {};
      let matchClause = `MATCH (c:Context)`;
      let whereClauses: string[] = [];

      if (criteria.type) {
        whereClauses.push(`c.type = $type`);
        params.type = criteria.type;
      }

      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(" AND ")}`;
      }
      cypher += `\nRETURN properties(c) as props`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const contexts: ContextShape[] = [];
      for (const record of result.records) {
        const rawProps = record.get("props");

        const context: ContextShape = {
          core: {
            id: rawProps.id,
            type: rawProps.type,
            name: rawProps.name || undefined,
            description: rawProps.description || undefined,
            createdAt: rawProps.createdAt,
            updatedAt: rawProps.updatedAt,
          },
          state: rawProps.state ? JSON.parse(rawProps.state) : {},
          entities: rawProps.entities ? JSON.parse(rawProps.entities) : [],
          relations: rawProps.relations ? JSON.parse(rawProps.relations) : [],
          signature: rawProps.signature ? JSON.parse(rawProps.signature) : undefined,
          facets: rawProps.facets ? JSON.parse(rawProps.facets) : {},
        };

        contexts.push(context);
      }

      return contexts;
    } catch (error) {
      console.error(`Error finding contexts: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Delete a context by ID
   */
  async deleteContext(id: string): Promise<boolean> {
    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      const summary = await session.executeWrite(async (txc) => {
        const result = await txc.run(
          `
          MATCH (c:Context {id: $id})
          DETACH DELETE c
          `,
          { id }
        );
        return result.summary;
      });

      const nodesDeleted = summary.counters.updates().nodesDeleted;
      return nodesDeleted > 0;
    } catch (error) {
      console.error(`Error deleting context (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }
}
