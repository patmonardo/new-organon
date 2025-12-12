import { Neo4jConnection } from "./neo4j-client";
import { MorphShape } from "../schema/morph";
import { v4 as uuidv4 } from "uuid";

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

  /**
   * Save a morph to Neo4j
   */
  async saveMorph(morphData: Partial<MorphShape>): Promise<MorphShape> {
    const now = Date.now();

    // Build morph with defaults
    const morph: MorphShape = {
      core: {
        id: morphData.core?.id || uuidv4(),
        type: morphData.core?.type || "morph.unknown",
        name: morphData.core?.name,
        description: morphData.core?.description,
        inputType: morphData.core?.inputType || "FormShape",
        outputType: morphData.core?.outputType || "FormShape",
        transformFn: morphData.core?.transformFn,
        createdAt: morphData.core?.createdAt || new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
      state: morphData.state || {},
      signature: morphData.signature,
      facets: morphData.facets || {},
      composition: morphData.composition || { kind: "single", steps: [] },
      config: morphData.config || {},
      meta: morphData.meta || {},
    };

    // Prepare properties for Neo4j
    const props = {
      id: morph.core.id,
      type: morph.core.type,
      name: morph.core.name || null,
      description: morph.core.description || null,
      inputType: morph.core.inputType,
      outputType: morph.core.outputType,
      transformFn: morph.core.transformFn || null,
      state: JSON.stringify(morph.state),
      signature: morph.signature ? JSON.stringify(morph.signature) : null,
      facets: JSON.stringify(morph.facets),
      compositionKind: morph.composition.kind,
      compositionMode: morph.composition.mode || null,
      compositionSteps: JSON.stringify(morph.composition.steps),
      config: JSON.stringify(morph.config),
      meta: JSON.stringify(morph.meta),
    };

    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (m:Morph {id: $props.id})
          ON CREATE SET m += $props
          ON MATCH SET m += $props
          RETURN m.id as id
          `,
          { props }
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
  async getMorphById(id: string): Promise<MorphShape | null> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run(
          `
          MATCH (m:Morph {id: $id})
          RETURN properties(m) as props
          `,
          { id }
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const rawProps = result.records[0].get("props");

      const morph: MorphShape = {
        core: {
          id: rawProps.id,
          type: rawProps.type,
          name: rawProps.name || undefined,
          description: rawProps.description || undefined,
          inputType: rawProps.inputType || "FormShape",
          outputType: rawProps.outputType || "FormShape",
          transformFn: rawProps.transformFn || undefined,
          createdAt: rawProps.createdAt,
          updatedAt: rawProps.updatedAt,
        },
        state: rawProps.state ? JSON.parse(rawProps.state) : {},
        signature: rawProps.signature ? JSON.parse(rawProps.signature) : undefined,
        facets: rawProps.facets ? JSON.parse(rawProps.facets) : {},
        composition: {
          kind: rawProps.compositionKind || "single",
          mode: rawProps.compositionMode || undefined,
          steps: rawProps.compositionSteps ? JSON.parse(rawProps.compositionSteps) : [],
        },
        config: rawProps.config ? JSON.parse(rawProps.config) : {},
        meta: rawProps.meta ? JSON.parse(rawProps.meta) : {},
      };

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
  async findMorphs(criteria: {
    type?: string;
    inputType?: string;
    outputType?: string;
  } = {}): Promise<MorphShape[]> {
    const session = this.connection.getSession({ defaultAccessMode: "READ" });
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
        cypher += `\nWHERE ${whereClauses.join(" AND ")}`;
      }
      cypher += `\nRETURN properties(m) as props`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const morphs: MorphShape[] = [];
      for (const record of result.records) {
        const rawProps = record.get("props");

        const morph: MorphShape = {
          core: {
            id: rawProps.id,
            type: rawProps.type,
            name: rawProps.name || undefined,
            description: rawProps.description || undefined,
            inputType: rawProps.inputType || "FormShape",
            outputType: rawProps.outputType || "FormShape",
            transformFn: rawProps.transformFn || undefined,
            createdAt: rawProps.createdAt,
            updatedAt: rawProps.updatedAt,
          },
          state: rawProps.state ? JSON.parse(rawProps.state) : {},
          signature: rawProps.signature ? JSON.parse(rawProps.signature) : undefined,
          facets: rawProps.facets ? JSON.parse(rawProps.facets) : {},
          composition: {
            kind: rawProps.compositionKind || "single",
            mode: rawProps.compositionMode || undefined,
            steps: rawProps.compositionSteps ? JSON.parse(rawProps.compositionSteps) : [],
          },
          config: rawProps.config ? JSON.parse(rawProps.config) : {},
          meta: rawProps.meta ? JSON.parse(rawProps.meta) : {},
        };

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
    const session = this.connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      const summary = await session.executeWrite(async (txc) => {
        const result = await txc.run(
          `
          MATCH (m:Morph {id: $id})
          DETACH DELETE m
          `,
          { id }
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
