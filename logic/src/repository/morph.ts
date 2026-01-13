import { Neo4jConnection } from './neo4j-client';
import { MorphSchema, type MorphShapeRepo } from '@schema/morph';
import { v4 as uuidv4 } from 'uuid';
import neo4j from 'neo4j-driver';

/**
 * MorphRepository
 *
 * Manages the persistence of Morph Shapes in Neo4j.
 * Morphs represent the Ground (Grund) - the unity of Shape and Context.
 */
export class MorphRepository {
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

  private sanitize(input: Partial<MorphShapeRepo>): MorphShapeRepo {
    const now = Date.now();
    const withDefaults = {
      id: input.id ?? uuidv4(),
      createdAt: input.createdAt ?? now,
      updatedAt: now,
      composition: input.composition ?? { kind: 'single', steps: [] },
      inputType: input.inputType ?? 'FormShape',
      outputType: input.outputType ?? 'FormShape',
      ...input,
    } as Partial<MorphShapeRepo>;
    return MorphSchema.parse(withDefaults);
  }

  /**
   * Save a morph to Neo4j
   */
  async saveMorph(morphData: Partial<MorphShapeRepo>): Promise<MorphShapeRepo> {
    const morph = this.sanitize(morphData);

    const props = {
      id: morph.id,
      type: morph.type,
      name: morph.name ?? null,
      description: morph.description ?? null,
      inputType: morph.inputType,
      outputType: morph.outputType,
      transformFn: morph.transformFn ?? null,
      state: morph.state ? JSON.stringify(morph.state) : null,
      signature: morph.signature ? JSON.stringify(morph.signature) : null,
      facets: morph.facets ? JSON.stringify(morph.facets) : null,
      compositionKind: morph.composition.kind,
      compositionMode: morph.composition.mode ?? null,
      compositionSteps: JSON.stringify(morph.composition.steps ?? []),
      config: morph.config ? JSON.stringify(morph.config) : null,
      status: morph.status ?? null,
      tags: morph.tags ? JSON.stringify(morph.tags) : null,
      meta: morph.meta ? JSON.stringify(morph.meta) : null,
      createdAt: morph.createdAt,
      updatedAt: morph.updatedAt,
    };

    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (m:Morph {id: $props.id})
          ON CREATE SET m += $props
          ON MATCH SET m += $props
          RETURN m.id as id
          `,
          { props },
        );
      });

      return morph;
    } catch (error) {
      console.error(`Error saving morph: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Get a morph by ID
   */
  async getMorphById(id: string): Promise<MorphShapeRepo | null> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run(
          `
          MATCH (m:Morph {id: $id})
          RETURN properties(m) as props
          `,
          { id },
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get('props');

      const morph = MorphSchema.parse({
        id: rawProps.id,
        type: rawProps.type,
        name: rawProps.name ?? undefined,
        description: rawProps.description ?? undefined,
        inputType: rawProps.inputType ?? 'FormShape',
        outputType: rawProps.outputType ?? 'FormShape',
        transformFn: rawProps.transformFn ?? undefined,
        state: this.parseJson(rawProps.state) ?? {},
        signature: this.parseJson(rawProps.signature),
        facets: this.parseJson(rawProps.facets) ?? {},
        composition: {
          kind: rawProps.compositionKind ?? 'single',
          mode: rawProps.compositionMode ?? undefined,
          steps: this.parseJson(rawProps.compositionSteps) ?? [],
        },
        config: this.parseJson(rawProps.config) ?? {},
        status: rawProps.status ?? undefined,
        tags: this.parseJson<string[]>(rawProps.tags) ?? [],
        meta: this.parseJson(rawProps.meta),
        createdAt: neo4j.isInt(rawProps.createdAt)
          ? rawProps.createdAt.toNumber()
          : rawProps.createdAt,
        updatedAt: neo4j.isInt(rawProps.updatedAt)
          ? rawProps.updatedAt.toNumber()
          : rawProps.updatedAt,
      });

      return morph;
    } catch (error) {
      console.error(`Error getting morph by ID (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Find morphs by criteria
   */
  async findMorphs(
    criteria: {
      type?: string;
      inputType?: string;
      outputType?: string;
    } = {},
  ): Promise<MorphShapeRepo[]> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const params: Record<string, any> = {};
      let matchClause = `MATCH (m:Morph)`;
      let whereClauses: string[] = [];

      if (criteria.type) {
        whereClauses.push(`m.type = $type`);
        params.type = criteria.type;
      }

      if (criteria.inputType) {
        whereClauses.push(`m.inputType = $inputType`);
        params.inputType = criteria.inputType;
      }

      if (criteria.outputType) {
        whereClauses.push(`m.outputType = $outputType`);
        params.outputType = criteria.outputType;
      }

      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(' AND ')}`;
      }
      cypher += `\nRETURN properties(m) as props`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const morphs: MorphShapeRepo[] = [];
      for (const record of result.records) {
        const rawProps = record.get('props');

        const morph = MorphSchema.parse({
          id: rawProps.id,
          type: rawProps.type,
          name: rawProps.name ?? undefined,
          description: rawProps.description ?? undefined,
          inputType: rawProps.inputType ?? 'FormShape',
          outputType: rawProps.outputType ?? 'FormShape',
          transformFn: rawProps.transformFn ?? undefined,
          state: this.parseJson(rawProps.state) ?? {},
          signature: this.parseJson(rawProps.signature),
          facets: this.parseJson(rawProps.facets) ?? {},
          composition: {
            kind: rawProps.compositionKind ?? 'single',
            mode: rawProps.compositionMode ?? undefined,
            steps: this.parseJson(rawProps.compositionSteps) ?? [],
          },
          config: this.parseJson(rawProps.config) ?? {},
          status: rawProps.status ?? undefined,
          tags: this.parseJson<string[]>(rawProps.tags) ?? [],
          meta: this.parseJson(rawProps.meta),
          createdAt: neo4j.isInt(rawProps.createdAt)
            ? rawProps.createdAt.toNumber()
            : rawProps.createdAt,
          updatedAt: neo4j.isInt(rawProps.updatedAt)
            ? rawProps.updatedAt.toNumber()
            : rawProps.updatedAt,
        });

        morphs.push(morph);
      }

      return morphs;
    } catch (error) {
      console.error(`Error finding morphs: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Delete a morph by ID
   */
  async deleteMorph(id: string): Promise<boolean> {
    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      const summary = await session.executeWrite(async (txc) => {
        const result = await txc.run(
          `
          MATCH (m:Morph {id: $id})
          DETACH DELETE m
          `,
          { id },
        );
        return result.summary;
      });

      const nodesDeleted = summary.counters.updates().nodesDeleted;
      return nodesDeleted > 0;
    } catch (error) {
      console.error(`Error deleting morph (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }
}
