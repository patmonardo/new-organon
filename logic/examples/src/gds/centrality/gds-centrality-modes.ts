/**
 * Centrality examples via a single mode-aware runner.
 *
 * Usage:
 *   node gds-centrality-modes.ts [mode] [op]
 *
 * mode: stream | stats | mutate | estimate (default: mutate)
 * op: optional filter (e.g. pagerank, hits, celf)
 */

/// <reference types="node" />

import { tsjsonInvoke } from '../_tsjson';

type Mode = 'stream' | 'stats' | 'mutate' | 'estimate';

type Request = Record<string, unknown>;

type AlgoRequest = {
  op: string;
  graphName: string;
  request: Request;
};

function buildGraphPut(graphName: string, directed: boolean): Request {
  const edges = directed
    ? [
        { type: 'REL', source: 0, target: 1 },
        { type: 'REL', source: 1, target: 2 },
        { type: 'REL', source: 2, target: 3 },
        { type: 'REL', source: 3, target: 4 },
        { type: 'REL', source: 4, target: 0 },
      ]
    : [
        { type: 'REL', source: 0, target: 1 },
        { type: 'REL', source: 1, target: 0 },
        { type: 'REL', source: 1, target: 2 },
        { type: 'REL', source: 2, target: 1 },
        { type: 'REL', source: 2, target: 3 },
        { type: 'REL', source: 3, target: 2 },
        { type: 'REL', source: 3, target: 4 },
        { type: 'REL', source: 4, target: 3 },
      ];

  return {
    kind: 'ApplicationForm',
    facade: 'graph_store',
    op: 'put',
    snapshot: {
      nodes: [0, 1, 2, 3, 4],
      relationships: edges,
    },
  };
}

function buildCentralityRequests(
  graphNameDirected: string,
  graphNameUndirected: string,
): AlgoRequest[] {
  return [
    {
      op: 'degree_centrality',
      graphName: graphNameUndirected,
      request: {
        orientation: 'undirected',
        mutateProperty: 'degree_score',
      },
    },
    {
      op: 'closeness',
      graphName: graphNameUndirected,
      request: {
        direction: 'both',
        useWasserman: false,
        mutateProperty: 'closeness_score',
      },
    },
    {
      op: 'harmonic',
      graphName: graphNameUndirected,
      request: {
        direction: 'both',
        mutateProperty: 'harmonic_score',
      },
    },
    {
      op: 'betweenness',
      graphName: graphNameUndirected,
      request: {
        direction: 'both',
        samplingStrategy: 'all',
        mutateProperty: 'betweenness_score',
      },
    },
    {
      op: 'pagerank',
      graphName: graphNameDirected,
      request: {
        direction: 'outgoing',
        iterations: 20,
        dampingFactor: 0.85,
        tolerance: 1e-4,
        mutateProperty: 'pagerank_score',
      },
    },
    {
      op: 'celf',
      graphName: graphNameDirected,
      request: {
        seedSetSize: 2,
        monteCarloSimulations: 10,
        propagationProbability: 0.5,
        batchSize: 2,
        randomSeed: 42,
        mutateProperty: 'celf_spread',
      },
    },
    {
      op: 'hits',
      graphName: graphNameDirected,
      request: {
        maxIterations: 20,
        tolerance: 1e-4,
        mutateProperty: 'hits_hub',
      },
    },
    {
      op: 'articulation_points',
      graphName: graphNameUndirected,
      request: {
        propertyName: 'is_articulation_point',
      },
    },
    {
      op: 'bridges',
      graphName: graphNameUndirected,
      request: {
        mutateProperty: 'is_bridge',
      },
    },
  ];
}

export function centralityModesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const mode = (process.argv[2] as Mode) ?? 'mutate';
  const opFilter = process.argv[3];

  const graphNameDirected = `centrality-directed-${Date.now()}`;
  const graphNameUndirected = `centrality-undirected-${Date.now()}`;

  const requests: Request[] = [
    {
      ...buildGraphPut(graphNameDirected, true),
      user,
      databaseId,
      graphName: graphNameDirected,
    },
    {
      ...buildGraphPut(graphNameUndirected, false),
      user,
      databaseId,
      graphName: graphNameUndirected,
    },
  ];

  const algos = buildCentralityRequests(graphNameDirected, graphNameUndirected)
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
      ...item.request,
    }));

  const batch = [...requests, ...algos];

  // eslint-disable-next-line no-console
  console.log('Centrality batch request:', JSON.stringify(batch, null, 2));

  const resp = tsjsonInvoke(batch) as any[];
  const results = resp.slice(2);

  // eslint-disable-next-line no-console
  console.log(`\nCentrality ${mode} results:`);
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
  centralityModesDemo();
}
