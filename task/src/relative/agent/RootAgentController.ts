// src/agent/MasterAgentController.ts
import { Controller, Get, Post, Body, Param, Delete } from '@nestjs/common';
import { MasterAgentService, AgentInfo } from './RootAgent';

@Controller('agents')
export class MasterAgentController {
  constructor(private readonly masterAgent: MasterAgentService) {}

  @Post()
  register(@Body() agent: AgentInfo) {
    this.masterAgent.registerAgent(agent);
    return { status: 'registered', agent };
  }

  @Get()
  list() {
    return this.masterAgent.listAgents();
  }

  @Get(':id')
  get(@Param('id') id: string) {
    return this.masterAgent.getAgent(id);
  }

  @Delete(':id')
  remove(@Param('id') id: string) {
    this.masterAgent.removeAgent(id);
    return { status: 'removed', id };
  }
}
