import { Workflow } from '../schema/workflow';

/**
 * WorkflowRepository - In-memory and pluggable base for Workflow persistence.
 * Replace with a Neo4j/Cypher adapter as needed.
 */
export class WorkflowRepository {
  private workflows: Map<string, Workflow> = new Map();

  async saveWorkflow(
    workflow: Workflow,
  ): Promise<Workflow> {
    this.workflows.set(workflow.id, workflow);
    return workflow;
  }

  async getWorkflowById(id: string): Promise<Workflow | undefined> {
    return this.workflows.get(id);
  }

  async findWorkflows(
    query?: Partial<Workflow>,
  ): Promise<Workflow[]> {
    return Array.from(this.workflows.values()).filter((workflow) => {
      if (!query) return true;
      return Object.entries(query).every(
        ([key, value]) => (workflow as any)[key] === value,
      );
    });
  }

  async deleteWorkflow(id: string): Promise<boolean> {
    return this.workflows.delete(id);
  }
}
