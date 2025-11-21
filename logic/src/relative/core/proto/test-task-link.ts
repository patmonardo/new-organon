import { ProtoEngine } from './proto-engine';
import { AgentEngine } from './agent-engine';
import { AgentRunner } from './agent-runner';
import type { EmpowermentLike } from '../empowerment-core';

// Replace heavy workspace imports with tiny local factories so the proto flow runs
// even when @organon/task imports/path mappings are not available.
const TaskFactory = {
  createBeingTask: (goal: string, methodName: string) => ({
    id: `task:${Math.random().toString(36).slice(2, 9)}`,
    goal,
    method: {
      name: methodName,
      steps: [{ action: 'run' }],
      entityType: 'being-entity',
    },
  }),
};

const AgentFactory = {
  createBeingAgent: (id: string, capabilities: string[] = []) => ({
    id,
    capabilities,
    name: id,
  }),
};

const WorkflowFactory = {
  createBeingWorkflow: (id: string, steps: string[] = []) => ({
    id,
    steps,
  }),
};

async function run() {
  // create domain objects via the lightweight local factories
  const task = TaskFactory.createBeingTask('demo-goal', 'demo-method');
  const actor = 'alice';
  const agent = AgentFactory.createBeingAgent('demo-agent', ['exec']);
  const workflow = WorkflowFactory.createBeingWorkflow('demo-flow', [
    'step1',
    'step2',
  ]);

  console.log(
    'created task.id=',
    task.id,
    'agent.id=',
    agent.id,
    'workflow.id=',
    workflow.id,
  );

  // policy engine grants actor permission for the task action/resource
  const policyEngine = new ProtoEngine('policy-engine', [
    {
      id: `emp:${actor}:task`,
      subject: actor,
      actions: [task.method?.steps?.[0]?.action ?? 'run', 'run'],
      weight: 1,
      certainty: 1,
      facets: [task.method.entityType ?? 'being-entity', 'resource:task'],
      signatures: [{ id: 's-policy', issuer: 'policy', signature: 'ok' }],
    } as EmpowermentLike,
  ]);

  // agent engine: the actual executor
  const agentEngine = new AgentEngine('agent1', [
    {
      id: `emp:agent1:exec`,
      subject: 'agent1',
      actions: ['run'],
      weight: 5,
      certainty: 1,
      facets: ['run', 'resource:task'],
      signatures: [{ id: 's-agent', issuer: 'agent1', signature: 'agent-sig' }],
    } as EmpowermentLike,
  ]);

  // runner composes the pipeline and attempts to run the task
  const runner = new AgentRunner([policyEngine, agentEngine], {
    logger: console.log,
  });

  const protoTask = {
    id: task.id,
    actor,
    action: 'run',
    resource: 'resource:task',
    payload: { task, workflow },
  };

  const outcome = await runner.runTask(protoTask, { privilegeBoost: 1 });
  console.log('agent-run outcome:', JSON.stringify(outcome, null, 2));
}

if (require.main === module) {
  run().catch((e) => {
    console.error('test-task-link failed:', e);
    process.exit(1);
  });
}
