import { Neo4jConnection } from './neo4j-client';
import { EntityShapeSchema, type EntityShapeRepo } from '../schema/entity';
import {
  ManagedTransaction,
  QueryResult,
  RecordShape,
  ResultSummary,
} from 'neo4j-driver';
import { v4 as uuidv4 } from 'uuid'; // Use uuid library

/**
 * EntityShapeRepository
 *
 * Manages the persistence of Entity Shapes (Instances) in Neo4j.
 */
export class EntityShapeRepository {
  private connection: Neo4jConnection;

  // No FormShapeRepository dependency
  constructor(connection: Neo4jConnection) {
    this.connection = connection;
  }

  /**
   * Saves an entity instance (EntityShape) to Neo4j.
   * Entity is the reciprocation of Form (Rational) and Data (Empirical).
   * Stores formId reference and actual field values.
   *
   * @param entityData The entity instance data with formId and values
   */
  async saveEntity(
    entityData: Partial<EntityShapeRepo>,
  ): Promise<EntityShapeRepo> {
    // 1. Prepare entity object
    const now = Date.now();

    // FormDB rule: do not persist empirical data values in Neo4j.
    // The Model layer owns data (e.g., Postgres). Neo4j stores only transcendental linkage.
    if (entityData.values && Object.keys(entityData.values).length > 0) {
      throw new Error(
        'EntityShape.values must not be persisted in FormDB (Neo4j). Store data in Model/Postgres and keep values empty here.',
      );
    }

    // Validate formId is provided (Entity must reference its Form Principle)
    if (!entityData.formId) {
      throw new Error('Entity must have formId reference to Form Principle');
    }

    const entity: EntityShapeRepo = {
      id: entityData.id || uuidv4(),
      type: entityData.type || 'entity.unknown',
      formId: entityData.formId, // Reference to Form Principle
      name: entityData.name,
      description: entityData.description,
      values: {}, // Data values are not persisted in FormDB
      signature: entityData.signature,
      facets: entityData.facets,
      status: entityData.status,
      tags: entityData.tags || [],
      meta: entityData.meta || {},
      createdAt: entityData.createdAt || now,
      updatedAt: now,
    };

    // 2. Prepare properties for Neo4j
    const props = {
      id: entity.id,
      type: entity.type,
      formId: entity.formId, // Store Form reference
      name: entity.name || null,
      description: entity.description || null,
      values: '{}', // Do not store empirical values in FormDB
      signature: entity.signature ? JSON.stringify(entity.signature) : null,
      facets: entity.facets ? JSON.stringify(entity.facets) : null,
      status: entity.status || null,
      meta: entity.meta ? JSON.stringify(entity.meta) : null,
      createdAt: entity.createdAt,
      updatedAt: entity.updatedAt,
    };

    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      const entityTypeLabel = this.getSafeLabel(entity.type);

      const savedId = await session.executeWrite(
        async (txc: ManagedTransaction) => {
          // Use standard MERGE - no dynamic labels, just :Entity with type property
          const mergeResult = await txc.run(
            `
          MERGE (es:Entity {id: $props.id})
          ON CREATE SET es += $props
          ON MATCH SET es += $props
          RETURN es.id as id
          `,
            { props },
          );
          const nodeId = mergeResult.records[0]?.get('id');

          if (!nodeId) {
            throw new Error('Failed to merge entity node.');
          }

          await this.syncTags(txc, nodeId, entity.tags);

          return nodeId;
        },
      );

      return EntityShapeSchema.parse(entity);
    } catch (error) {
      console.error(`Error saving entity shape to Neo4j: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  async getEntityById(id: string): Promise<EntityShapeRepo | null> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const result = await session.executeRead(
        async (txc: ManagedTransaction) => {
          return await txc.run<{ props: Record<string, any>; tags: string[] }>(
            `
          MATCH (n:Entity {id: $id})
          OPTIONAL MATCH (n)-[:HAS_TAG]->(t:Tag)
          RETURN properties(n) as props, collect(t.name) as tags
          `,
            { id },
          );
        },
      );

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get('props');
      const tags = result.records[0].get('tags') || [];

      // Parse JSON fields
      const entity: EntityShapeRepo = {
        id: rawProps.id,
        type: rawProps.type,
        formId: rawProps.formId, // Form Principle reference
        name: rawProps.name || undefined,
        description: rawProps.description || undefined,
        values: {}, // Do not rehydrate empirical values from FormDB
        signature: rawProps.signature
          ? JSON.parse(rawProps.signature)
          : undefined,
        facets: rawProps.facets ? JSON.parse(rawProps.facets) : undefined,
        status: rawProps.status || undefined,
        tags: tags,
        meta: rawProps.meta ? JSON.parse(rawProps.meta) : undefined,
        createdAt: rawProps.createdAt,
        updatedAt: rawProps.updatedAt,
      };

      return EntityShapeSchema.parse(entity);
    } catch (error) {
      console.error(`Error getting entity shape by ID (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Finds entity instances based on criteria.
   * Supports filtering by type and tags.
   */
  async findEntities(
    criteria: {
      type?: string;
      tags?: string[];
    } = {},
  ): Promise<EntityShapeRepo[]> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const params: Record<string, any> = {};
      let matchClause = `MATCH (n:Entity)`;
      let whereClauses: string[] = [];

      // Filter by type if provided
      if (criteria.type) {
        whereClauses.push(`n.type = $type`);
        params.type = criteria.type;
      }

      // Filter by tags if provided
      if (criteria.tags && criteria.tags.length > 0) {
        criteria.tags.forEach((tag, index) => {
          const paramName = `tag${index}`;
          params[paramName] = tag;
          whereClauses.push(
            `EXISTS { MATCH (n)-[:HAS_TAG]->(:Tag {name: $${paramName}}) }`,
          );
        });
      }

      // Construct query
      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(' AND ')}`;
      }
      cypher += `
        OPTIONAL MATCH (n)-[:HAS_TAG]->(t:Tag)
        RETURN properties(n) as props, collect(t.name) as tags`;

      const result = await session.executeRead(
        async (txc: ManagedTransaction) => {
          return await txc.run(cypher, params);
        },
      );

      // Reconstruct entities
      const entities: EntityShapeRepo[] = [];
      for (const record of result.records) {
        const rawProps = record.get('props');
        const tags = record.get('tags') || [];

        const entity: EntityShapeRepo = {
          id: rawProps.id,
          type: rawProps.type,
          formId: rawProps.formId, // Form Principle reference
          name: rawProps.name || undefined,
          description: rawProps.description || undefined,
          values: {}, // Do not rehydrate empirical values from FormDB
          signature: rawProps.signature
            ? JSON.parse(rawProps.signature)
            : undefined,
          facets: rawProps.facets ? JSON.parse(rawProps.facets) : undefined,
          status: rawProps.status || undefined,
          tags: tags,
          meta: rawProps.meta ? JSON.parse(rawProps.meta) : undefined,
          createdAt: rawProps.createdAt,
          updatedAt: rawProps.updatedAt,
        };

        entities.push(EntityShapeSchema.parse(entity));
      }

      return entities;
    } catch (error) {
      console.error(`Error finding entity shapes: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Deletes an entity instance by its ID.
   * Detaches the node from all relationships before deleting.
   *
   * @param id The unique ID of the entity instance to delete.
   * @returns True if the entity was deleted, false if it was not found.
   */
  async deleteEntity(id: string): Promise<boolean> {
    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      const summary: ResultSummary = await session.executeWrite(
        async (txc: ManagedTransaction) => {
          // Match the node by ID using the base :Entity label
          // DETACH DELETE removes the node and all its relationships
          const result = await txc.run(
            `
          MATCH (n:Entity {id: $id})
          DETACH DELETE n
          `,
            { id },
          );
          // Return the summary which contains counters
          return result.summary;
        },
      );

      // Check if any nodes were actually deleted
      const nodesDeleted = summary.counters.updates().nodesDeleted;
      return nodesDeleted > 0;
    } catch (error) {
      console.error(
        `Error deleting entity shape with ID (${id}) from Neo4j: ${error}`,
      );
      throw error; // Re-throw error after logging
    } finally {
      await session.close();
    }
  }

  // --- syncTags method ---
  private async syncTags(
    txc: ManagedTransaction,
    entityId: string,
    tags: string[] | undefined,
  ) {
    await txc.run(`MATCH (e {id: $id})-[r:HAS_TAG]->() DELETE r`, {
      id: entityId,
    });
    if (tags && tags.length > 0) {
      await txc.run(
        `
        UNWIND $tags as tagName
        MATCH (e {id: $id})
        MERGE (t:Tag {name: tagName})
        MERGE (e)-[:HAS_TAG]->(t)
      `,
        { id: entityId, tags: tags },
      );
    }
  }

  // --- getSafeLabel method ---
  private getSafeLabel(name: string | undefined): string {
    if (!name) return 'Unknown';
    let safeName = name.replace(/[^a-zA-Z0-9_]/g, '_');
    if (!safeName || !/^[a-zA-Z_]/.test(safeName)) {
      safeName = '_' + safeName;
    }
    return safeName || 'Unknown';
  }
}
