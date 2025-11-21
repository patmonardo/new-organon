// src/agent/RootAgent.ts
import { Injectable } from '@nestjs/common';

export interface AgentInfo {
  id: string;
  name: string;
  status: string;
  // Add more metadata as needed
}

@Injectable()
export class RootAgentService {
  private agents: Map<string, AgentInfo> = new Map();

  // Register a new agent
  registerAgent(agent: AgentInfo) {
    this.agents.set(agent.id, agent);
  }

  // Advertise all agents
  listAgents(): AgentInfo[] {
    return Array.from(this.agents.values());
  }

  // Discover agent by ID
  getAgent(id: string): AgentInfo | undefined {
    return this.agents.get(id);
  }

  // Remove agent
  removeAgent(id: string) {
    this.agents.delete(id);
  }
}
