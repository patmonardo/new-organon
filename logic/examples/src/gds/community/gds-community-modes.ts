/**
 * Community examples via a single mode-aware runner.
 *
 * Usage:
 *   node gds-community-modes.ts [mode] [op]
 *
 * mode: stream | stats | mutate | estimate (default: mutate)
 * op: optional filter (e.g. louvain, leiden, wcc)
 */

/// <reference types="node" />

import { tsjsonInvoke } from '../_tsjson';

type Mode = 'stream' | 'stats' | 'mutate' | 'estimate';

type Request = Record<string, unknown>;

type AlgoRequest = {
  op: string;
  graphName: string;
  request: Request;
  expectedPropertyName?: string;
};

function buildGraphPut(graphName: string): Request {
  return {
    kind: 'ApplicationForm',
    facade: 'graph_store',
    op: 'put',
    snapshot: {
      nodes: [0, 1, 2, 3, 4, 5],
      nodeProperties: {
        features: [
          [0.0, 0.0],
          [1.0, 0.0],
          [2.0, 0.0],
          [3.0, 1.0],
          [4.0, 1.0],
          [5.0, 1.0],
        ],
      },
      relationships: [
        { type: 'REL', source: 0, target: 1 },
        { type: 'REL', source: 1, target: 2 },
        { type: 'REL', source: 2, target: 3 },
        { type: 'REL', source: 3, target: 4 },
        { type: 'REL', source: 4, target: 5 },
        { type: 'REL', source: 5, target: 0 },
        { type: 'REL', source: 1, target: 4 },
      ],
    },
  };
}

function buildCommunityRequests(graphName: string): AlgoRequest[] {
  const includeLeiden =
    process.env.INCLUDE_LEIDEN === '1' || process.env.INCLUDE_LEIDEN === 'true';

  return [
    {
      op: 'louvain',
      graphName,
      request: {
        mutateProperty: 'community',
      },
      expectedPropertyName: 'community',
    },
    {
      op: 'wcc',
      graphName,
      request: {
        mutateProperty: 'componentId',
      },
      expectedPropertyName: 'componentId',
    },
    ...(includeLeiden
      ? [
          {
            op: 'leiden',
            graphName,
            request: {
              gamma: 1.0,
              theta: 0.01,
              tolerance: 0.0001,
              maxIterations: 10,
              randomSeed: 42,
            },
          },
        ]
      : []),
    {
      op: 'label_propagation',
      graphName,
      request: {
        mutateProperty: 'label',
        maxIterations: 10,
      },
      expectedPropertyName: 'label',
    },
    {
      op: 'kcore',
      graphName,
      request: {
        mutateProperty: 'core',
      },
      expectedPropertyName: 'core',
    },
    {
      op: 'triangle',
      graphName,
      request: {
        mutateProperty: 'triangles',
        maxDegree: 100,
      },
      expectedPropertyName: 'triangles',
    },
    {
      op: 'scc',
      graphName,
      request: {
        mutateProperty: 'componentId',
      },
      expectedPropertyName: 'componentId',
    },
    {
      op: 'approx_max_kcut',
      graphName,
      request: {
        mutateProperty: 'community',
        k: 2,
        iterations: 5,
        randomSeed: 42,
      },
      expectedPropertyName: 'community',
    },
    {
      op: 'kmeans',
      graphName,
      request: {
        mutateProperty: 'community',
        nodeProperty: 'features',
        k: 2,
        maxIterations: 10,
        randomSeed: 42,
      },
      expectedPropertyName: 'community',
    },
  ];
}

function validateMutateResult(
  op: string,
  expectedPropertyName: string | undefined,
  result: any,
): void {
  if (!result?.ok) {
    // eslint-disable-next-line no-console
    console.error(op, result?.error ?? result);
    return;
  }

  const data = result.data ?? {};
  if (expectedPropertyName && data.property_name !== expectedPropertyName) {
    // eslint-disable-next-line no-console
    console.error(
      op,
      `unexpected property_name: ${String(data.property_name)} (expected ${expectedPropertyName})`,
    );
    return;
  }

  if (
    typeof data.nodes_updated !== 'number' &&
    typeof data.nodesWritten !== 'number' &&
    typeof data.nodes_written !== 'number'
  ) {
    // eslint-disable-next-line no-console
    console.warn(op, 'mutate response missing node counts', data);
    return;
  }

  // eslint-disable-next-line no-console
  console.log(op, JSON.stringify(data, null, 2));
}

export function communityModesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const mode = (process.argv[2] as Mode) ?? 'mutate';
  const opFilter = process.argv[3];

  const includeLeiden = opFilter === 'leiden';
  if (includeLeiden) {
    process.env.INCLUDE_LEIDEN = '1';
  }

  const graphName = `community-${Date.now()}`;

  const requests: Request[] = [
    {
      ...buildGraphPut(graphName),
      user,
      databaseId,
      graphName,
    },
  ];

  const algos = buildCommunityRequests(graphName)
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
      expectedPropertyName: item.expectedPropertyName,
      ...item.request,
    }));

  const batch = [...requests, ...algos];

  // eslint-disable-next-line no-console
  console.log('Community batch request:', JSON.stringify(batch, null, 2));

  const resp = tsjsonInvoke(batch) as any[];
  const results = resp.slice(1);

  // eslint-disable-next-line no-console
  console.log(`\nCommunity ${mode} results:`);
  results.forEach((result, index) => {
    const op = algos[index]?.op ?? 'unknown';
    const expectedPropertyName = algos[index]?.expectedPropertyName;

    if (mode === 'mutate') {
      validateMutateResult(op, expectedPropertyName, result);
      return;
    }

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
  communityModesDemo();
}
