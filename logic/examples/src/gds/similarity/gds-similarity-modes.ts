/**
 * Similarity examples via a single mode-aware runner.
 *
 * Usage:
 *   node gds-similarity-modes.ts [mode] [op]
 *
 * mode: stream | stats | estimate | mutate | write (default: mutate)
 * op: optional filter (e.g. node_similarity, knn)
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
      nodes: [0, 1, 2, 3, 4, 5],
      nodeProperties: {
        score: [0.1, 0.2, 0.25, 0.4, 0.5, 0.9],
        vector: [
          [1.0, 0.0],
          [0.9, 0.1],
          [0.8, 0.2],
          [0.0, 1.0],
          [0.1, 0.9],
          [0.2, 0.8],
        ],
      },
      relationships: [
        { type: 'REL', source: 0, target: 1, properties: { weight: 1.0 } },
        { type: 'REL', source: 1, target: 2, properties: { weight: 1.0 } },
        { type: 'REL', source: 2, target: 3, properties: { weight: 1.0 } },
        { type: 'REL', source: 3, target: 4, properties: { weight: 1.0 } },
        { type: 'REL', source: 4, target: 5, properties: { weight: 1.0 } },
        { type: 'REL', source: 5, target: 0, properties: { weight: 1.0 } },
        { type: 'REL', source: 1, target: 4, properties: { weight: 1.0 } },
      ],
    },
  };
}

function buildSimilarityRequests(graphName: string): AlgoRequest[] {
  return [
    {
      op: 'node_similarity',
      graphName,
      request: {
        similarityMetric: 'jaccard',
        similarityCutoff: 0.0,
        topK: 5,
        topN: 0,
      },
    },
    {
      op: 'filtered_node_similarity',
      graphName,
      request: {
        similarityMetric: 'jaccard',
        similarityCutoff: 0.0,
        topK: 5,
        topN: 0,
      },
    },
    {
      op: 'knn',
      graphName,
      request: {
        nodeProperties: [
          { name: 'vector', metric: 'COSINE' },
          { name: 'score', metric: 'DEFAULT' },
        ],
        topK: 3,
        similarityCutoff: 0.0,
      },
    },
    {
      op: 'filtered_knn',
      graphName,
      request: {
        nodeProperties: [{ name: 'vector', metric: 'COSINE' }],
        topK: 3,
        similarityCutoff: 0.0,
      },
    },
  ];
}

function addModeProperty(op: string, mode: Mode, request: Request): Request {
  if (mode === 'mutate') {
    return { ...request, mutateProperty: `${op}_similarity` };
  }
  if (mode === 'write') {
    return { ...request, writeProperty: `${op}_similarity` };
  }
  return request;
}

export function similarityModesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const mode = (process.argv[2] as Mode) ?? 'mutate';
  const opFilter = process.argv[3];

  const graphName = `similarity-${Date.now()}`;

  const requests: Request[] = [
    {
      ...buildGraphPut(graphName),
      user,
      databaseId,
      graphName,
    },
  ];

  const algos = buildSimilarityRequests(graphName)
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
  console.log('Similarity batch request:', JSON.stringify(batch, null, 2));

  const resp = tsjsonInvoke(batch) as any[];
  const results = resp.slice(1);

  // eslint-disable-next-line no-console
  console.log(`\nSimilarity ${mode} results:`);
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
  similarityModesDemo();
}
