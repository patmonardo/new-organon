// src/agent/AgentModule.ts
import { Module } from '@nestjs/common';
import { RootAgentService } from './RootAgent';
import { RootAgentController } from './RootAgentController';

@Module({
  providers: [RootAgentService],
  controllers: [RootAgentController],
})
export class AgentModule {}
