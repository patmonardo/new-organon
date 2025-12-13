import { Agent } from '../schema/agent';

/**
 * AgentRepository - In-memory and pluggable base for Agent persistence.
 * Replace with a Neo4j/Cypher adapter as needed.
 */
export class AgentRepository {
  private agents: Map<string, Agent> = new Map();

  async saveAgent(agent: Agent): Promise<Agent> {
    this.agents.set(agent.id, agent);
    return agent;
  }

  async getAgentById(id: string): Promise<Agent | undefined> {
    return this.agents.get(id);
  }

  async findAgents(
    query?: Partial<Agent>,
  ): Promise<Agent[]> {
    return Array.from(this.agents.values()).filter((agent) => {
      if (!query) return true;
      return Object.entries(query).every(
        ([key, value]) => (agent as any)[key] === value,
      );
    });
  }

  async deleteAgent(id: string): Promise<boolean> {
    return this.agents.delete(id);
  }
}
