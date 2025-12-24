/**
 * Example: Cross-domain inference with GraphStore
 *
 * This demonstrates how an Agent uses the GDSL GraphStore facade
 * to perform inference across SDSL domain boundaries.
 *
 * See gds/doc/GNN-GRAPHSTORE-INFERENCE-LAYER.md for architecture.
 */

import {
  type NodeId,
  type TraversalConstraint,
  MockGraphStoreBuilder,
  NodeIdSchema,
  InferenceResultSchema,
} from '@organon/gdsl';

async function crossDomainInferenceExample() {
  // 1. Build a GraphStore from Postgres (using mock for now)
  const builder = new MockGraphStoreBuilder();
  const graphStore = await builder.buildFromPostgres({
    domains: ['finance', 'operations'],
    includeEdges: [{ name: 'allocates_to' }, { name: 'reports_to' }],
  });

  // 2. Define query: can we reach from finance concept to operations concept?
  const startNode: NodeId = NodeIdSchema.parse({
    domain: 'finance',
    localId: 'budget_item_2024_q1',
  });

  const targetNode: NodeId = NodeIdSchema.parse({
    domain: 'operations',
    localId: 'project_alpha',
  });

  const constraints: TraversalConstraint[] = [
    { kind: 'reversibility' },
    { kind: 'type-safety' },
  ];

  // 3. Check reachability (fast boolean query)
  const isReachable = await graphStore.reachable(startNode, targetNode, constraints);
  console.log(`Can reach ${targetNode.localId} from ${startNode.localId}:`, isReachable);

  if (!isReachable) {
    console.log('No valid path found. Stopping.');
    return;
  }

  // 4. Find all valid paths (more expensive, includes provenance)
  const inferenceResult = await graphStore.traverse(startNode, targetNode, constraints, 3);

  // 5. Validate the result
  const validatedResult = InferenceResultSchema.parse(inferenceResult);

  if (!validatedResult.valid) {
    console.error('Inference failed:', validatedResult.error);
    return;
  }

  // 6. Inspect paths
  console.log(`Found ${validatedResult.paths.length} valid path(s):`);
  for (const path of validatedResult.paths) {
    console.log('  Path:', path.nodes.map((n) => `${n.domain}:${n.localId}`).join(' -> '));
    console.log('  Edges:', path.edges.map((e) => e.name).join(', '));
    console.log('  Proofs:', path.constraintProofs);
  }

  // 7. Validate a specific path (e.g., suggested by LLM)
  const proposedPath = validatedResult.paths[0];
  const isValidPath = await graphStore.validatePath(proposedPath, constraints);
  console.log('Proposed path is valid:', isValidPath);

  // 8. Propagate constraints from startNode
  const affectedNodes = await graphStore.propagateConstraints(
    startNode,
    'budget_cap',
    2, // max depth
  );
  console.log(`Constraint propagation affected ${affectedNodes.size} nodes`);
}

// Run the example
if (import.meta.url === `file://${process.argv[1]}`) {
  crossDomainInferenceExample().catch(console.error);
}

export { crossDomainInferenceExample };
