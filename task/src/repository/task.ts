import { Task } from '../schema/task';

/**
 * TaskRepository - In-memory and pluggable base for Task persistence.
 * Replace with a Neo4j/Cypher adapter as needed.
 */
export class TaskRepository {
  private tasks: Map<string, Task> = new Map();

  async saveTask(task: Task): Promise<Task> {
    this.tasks.set(task.id, task);
    return task;
  }

  async getTaskById(id: string): Promise<Task | undefined> {
    return this.tasks.get(id);
  }

  async findTasks(
    query?: Partial<Task>,
  ): Promise<Task[]> {
    return Array.from(this.tasks.values()).filter((task) => {
      if (!query) return true;
      return Object.entries(query).every(
        ([key, value]) => (task as any)[key] === value,
      );
    });
  }

  async deleteTask(id: string): Promise<boolean> {
    return this.tasks.delete(id);
  }
}
