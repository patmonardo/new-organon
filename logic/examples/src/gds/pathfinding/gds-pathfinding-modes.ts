/**
 * Pathfinding examples via a single mode-aware runner.
 *
 * Usage:
 *   node gds-pathfinding-modes.ts [mode] [op]
 *
 * mode: stream | stats | estimate | mutate | write (default: stream)
 * op: optional filter (e.g. bfs, dijkstra, astar)
 */

/// <reference types="node" />

import { tsjsonInvoke } from '../_tsjson';

type Mode = 'stream' | 'stats' | 'estimate' | 'mutate' | 'write';

type Request = Record<string, unknown>;

type AlgoRequest = {
  op: string;
  graphName: string;
  request: Request;
};

function buildGraphPut(graphName: string): Request {
  return {
    kind: 'ApplicationForm',
    facade: 'graph_store',
    op: 'put',
    snapshot: {
      nodes: [0, 1, 2, 3, 4],
      relationships: [
        { type: 'ROAD', source: 0, target: 1, properties: { weight: 1.0 } },
        { type: 'ROAD', source: 1, target: 2, properties: { weight: 1.0 } },
        { type: 'ROAD', source: 2, target: 3, properties: { weight: 1.0 } },
        { type: 'ROAD', source: 1, target: 3, properties: { weight: 2.5 } },
        { type: 'ROAD', source: 0, target: 4, properties: { weight: 2.0 } },
        { type: 'ROAD', source: 4, target: 3, properties: { weight: 2.0 } },
      ],
    },
  };
}

function buildPathfindingRequests(graphName: string): AlgoRequest[] {
  return [
    {
      op: 'bfs',
      graphName,
      request: {
        source: 0,
        targets: [3],
        maxDepth: 10,
        trackPaths: true,
        delta: 64,
      },
    },
    {
      op: 'dfs',
      graphName,
      request: {
        source: 0,
        targets: [3],
        maxDepth: 10,
        trackPaths: true,
      },
    },
    {
      op: 'dijkstra',
      graphName,
      request: {
        source: 0,
        target: 3,
        weightProperty: 'weight',
        direction: 'outgoing',
        trackRelationships: true,
      },
    },
    {
      op: 'astar',
      graphName,
      request: {
        source: 0,
        target: 3,
        weightProperty: 'weight',
        direction: 'outgoing',
        heuristic: 'manhattan',
      },
    },
    {
      op: 'bellman_ford',
      graphName,
      request: {
        source: 0,
        weightProperty: 'weight',
        direction: 'outgoing',
        trackNegativeCycles: true,
        trackPaths: true,
      },
    },
    {
      op: 'delta_stepping',
      graphName,
      request: {
        source: 0,
        delta: 1.0,
        weightProperty: 'weight',
        direction: 'outgoing',
        storePredecessors: true,
      },
    },
    {
      op: 'yens',
      graphName,
      request: {
        source: 0,
        target: 3,
        k: 3,
        weightProperty: 'weight',
        direction: 'outgoing',
      },
    },
    {
      op: 'all_shortest_paths',
      graphName,
      request: {
        weighted: true,
        direction: 'outgoing',
        weightProperty: 'weight',
        maxResults: 25,
      },
    },
    {
      op: 'topological_sort',
      graphName,
      request: {
        computeMaxDistance: false,
      },
    },
    {
      op: 'spanning_tree',
      graphName,
      request: {
        startNode: 0,
        computeMinimum: true,
        weightProperty: 'weight',
        direction: 'undirected',
      },
    },
    {
      op: 'steiner_tree',
      graphName,
      request: {
        sourceNode: 0,
        targetNodes: [3],
        relationshipWeightProperty: 'weight',
        delta: 1.0,
        applyRerouting: true,
      },
    },
    {
      op: 'random_walk',
      graphName,
      request: {
        walksPerNode: 2,
        walkLength: 5,
        returnFactor: 1.0,
        inOutFactor: 1.0,
        sourceNodes: [0],
      },
    },
    {
      op: 'kspanningtree',
      graphName,
      request: {
        sourceNode: 0,
        k: 4,
        objective: 'min',
        weightProperty: 'weight',
      },
    },
    {
      op: 'dag_longest_path',
      graphName,
      request: {},
    },
  ];
}

function addModeProperty(op: string, mode: Mode, request: Request): Request {
  if (mode === 'mutate') {
    return { ...request, mutateProperty: `${op}_rel` };
  }
  if (mode === 'write') {
    return { ...request, writeProperty: `${op}_rel` };
  }
  return request;
}

export function pathfindingModesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const mode = (process.argv[2] as Mode) ?? 'stream';
  const opFilter = process.argv[3];

  const graphName = `pathfinding-${Date.now()}`;

  const requests: Request[] = [
    {
      ...buildGraphPut(graphName),
      user,
      databaseId,
      graphName,
    },
  ];

  const algos = buildPathfindingRequests(graphName)
    .filter((item) => (opFilter ? item.op === opFilter : true))
    .map((item) => ({
      kind: 'ApplicationForm',
      facade: 'algorithms',
      op: item.op,
      mode,
      user,
      databaseId,
      graphName: item.graphName,
      concurrency: 1,
      ...addModeProperty(item.op, mode, item.request),
    }));

  const batch = [...requests, ...algos];

  // eslint-disable-next-line no-console
  console.log('Pathfinding batch request:', JSON.stringify(batch, null, 2));

  const resp = tsjsonInvoke(batch) as any[];
  const results = resp.slice(1);

  // eslint-disable-next-line no-console
  console.log(`\nPathfinding ${mode} results:`);
  results.forEach((result, index) => {
    const op = algos[index]?.op ?? 'unknown';

    if (result?.ok) {
      // eslint-disable-next-line no-console
      console.log(op, JSON.stringify(result.data, null, 2));
    } else {
      // eslint-disable-next-line no-console
      console.error(op, result?.error ?? result);
    }
  });
}

if (import.meta.url === `file://${process.argv[1]}`) {
  pathfindingModesDemo();
}
