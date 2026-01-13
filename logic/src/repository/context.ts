import { v4 as uuidv4 } from 'uuid';
import { Neo4jConnection } from './neo4j-client';
import { ContextShapeSchema, type ContextShapeRepo } from '../schema/context';

/**
 * ContextRepository
 *
 * Persists ContextShapeRepo records into Neo4j (FormDB) using a lean, record-only schema.
 */
export class ContextRepository {
  constructor(private readonly connection: Neo4jConnection) {}

  async saveContext(
    contextData: Partial<ContextShapeRepo>,
  ): Promise<ContextShapeRepo> {
    const now = Date.now();

    const context: ContextShapeRepo = ContextShapeSchema.parse({
      id: contextData.id ?? uuidv4(),
      type: contextData.type ?? 'context.unknown',
      name: contextData.name,
      description: contextData.description,
      state: contextData.state ?? {},
      entities: contextData.entities ?? [],
      relations: contextData.relations ?? [],
      signature: contextData.signature,
      facets: contextData.facets ?? {},
      createdAt: contextData.createdAt ?? now,
      updatedAt: now,
    });

    const props = {
      id: context.id,
      type: context.type,
      name: context.name ?? null,
      description: context.description ?? null,
      state: JSON.stringify(context.state ?? {}),
      entities: JSON.stringify(context.entities ?? []),
      relations: JSON.stringify(context.relations ?? []),
      signature: context.signature ? JSON.stringify(context.signature) : null,
      facets: JSON.stringify(context.facets ?? {}),
      createdAt: context.createdAt,
      updatedAt: context.updatedAt,
    };

    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (c:Context {id: $props.id})
          ON CREATE SET c += $props
          ON MATCH SET c += $props
          RETURN c.id as id
          `,
          { props },
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

  async getContextById(id: string): Promise<ContextShapeRepo | null> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run(
          `
          MATCH (c:Context {id: $id})
          RETURN properties(c) as props
          `,
          { id },
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get('props');
      const context: ContextShapeRepo = ContextShapeSchema.parse({
        id: rawProps.id,
        type: rawProps.type,
        name: rawProps.name ?? undefined,
        description: rawProps.description ?? undefined,
        state: rawProps.state ? JSON.parse(rawProps.state) : {},
        entities: rawProps.entities ? JSON.parse(rawProps.entities) : [],
        relations: rawProps.relations ? JSON.parse(rawProps.relations) : [],
        signature: rawProps.signature
          ? JSON.parse(rawProps.signature)
          : undefined,
        facets: rawProps.facets ? JSON.parse(rawProps.facets) : {},
        createdAt: rawProps.createdAt,
        updatedAt: rawProps.updatedAt,
      });

      return context;
    } catch (error) {
      console.error(`Error getting context by ID (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  async findContexts(
    criteria: { type?: string } = {},
  ): Promise<ContextShapeRepo[]> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const params: Record<string, any> = {};
      let matchClause = `MATCH (c:Context)`;
      const whereClauses: string[] = [];

      if (criteria.type) {
        whereClauses.push(`c.type = $type`);
        params.type = criteria.type;
      }

      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(' AND ')}`;
      }
      cypher += `\nRETURN properties(c) as props`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const contexts: ContextShapeRepo[] = [];
      for (const record of result.records) {
        const rawProps = record.get('props');
        const context = ContextShapeSchema.parse({
          id: rawProps.id,
          type: rawProps.type,
          name: rawProps.name ?? undefined,
          description: rawProps.description ?? undefined,
          state: rawProps.state ? JSON.parse(rawProps.state) : {},
          entities: rawProps.entities ? JSON.parse(rawProps.entities) : [],
          relations: rawProps.relations ? JSON.parse(rawProps.relations) : [],
          signature: rawProps.signature
            ? JSON.parse(rawProps.signature)
            : undefined,
          facets: rawProps.facets ? JSON.parse(rawProps.facets) : {},
          createdAt: rawProps.createdAt,
          updatedAt: rawProps.updatedAt,
        });

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

  async deleteContext(id: string): Promise<boolean> {
    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      const summary = await session.executeWrite(async (txc) => {
        const result = await txc.run(
          `
          MATCH (c:Context {id: $id})
          DETACH DELETE c
          `,
          { id },
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
