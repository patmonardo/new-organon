import neo4j from 'neo4j-driver';
import { TaskDefinition } from '../schema/task';

export class TaskRepository {
  constructor(private driver: neo4j.Driver) {}

  async saveTask(task: TaskDefinition): Promise<TaskDefinition> {
    const session = this.driver.session();
    const result = await session.run(
      'MERGE (t:Task {id: $id}) SET t += $props RETURN t',
      { id: task.id, props: task }
    );
    session.close();
    return result.records[0].get('t').properties;
  }

  async getTaskById(id: string): Promise<TaskDefinition | undefined> {
    const session = this.driver.session();
    const result = await session.run(
      'MATCH (t:Task {id: $id}) RETURN t',
      { id }
    );
    session.close();
    return result.records[0]?.get('t').properties;
  }

  async findTasks(query: Partial<TaskDefinition> = {}): Promise<TaskDefinition[]> {
    const session = this.driver.session();
    // Build dynamic Cypher WHERE clause based on query
    // For simplicity, match all if query is empty
    let cypher = 'MATCH (t:Task)';
    if (Object.keys(query).length) {
      cypher += ' WHERE ' + Object.entries(query)
        .map(([k], i) => `t.${k} = $${k}`)
        .join(' AND ');
    }
    cypher += ' RETURN t';
    const result = await session.run(cypher, query);
    session.close();
    return result.records.map(r => r.get('t').properties);
  }

  async deleteTask(id: string): Promise<boolean> {
    const session = this.driver.session();
    await session.run(
      'MATCH (t:Task {id: $id}) DETACH DELETE t',
      { id }
    );
    session.close();
    return true;
  }
}
