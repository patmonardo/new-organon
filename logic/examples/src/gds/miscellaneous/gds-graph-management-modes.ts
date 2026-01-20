/**
 * Graph management (miscellaneous) reference via a mode-aware runner.
 *
 * Usage:
 *   node gds-graph-management-modes.ts [mode] [op]
 *
 * mode: stream | stats | estimate | mutate | write (default: mutate)
 * op: optional filter (e.g. scale_properties, index_inverse)
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
      nodeProperties: {
        score: [1.0, 2.0, 3.0, 4.0, 5.0],
        vector: [
          [1.0, 0.0],
          [0.9, 0.1],
          [0.8, 0.2],
          [0.1, 0.9],
          [0.0, 1.0],
        ],
      },
      relationships: [
        { type: 'REL', source: 0, target: 1, properties: { weight: 1.0 } },
        { type: 'REL', source: 1, target: 2, properties: { weight: 1.0 } },
        { type: 'REL', source: 2, target: 3, properties: { weight: 1.0 } },
        { type: 'REL', source: 3, target: 4, properties: { weight: 1.0 } },
        { type: 'REL', source: 4, target: 0, properties: { weight: 1.0 } },
      ],
    },
  };
}

function buildMiscRequests(graphName: string): AlgoRequest[] {
  return [
    {
      op: 'scale_properties',
      graphName,
      request: {
        nodeProperties: ['score', 'vector'],
        scaler: 'minMax',
        mutateProperty: 'scaled_features',
        writeProperty: 'scaled_features',
      },
    },
    {
      op: 'index_inverse',
      graphName,
      request: {
        relationshipTypes: ['REL'],
        mutateGraphName: `${graphName}-index-inverse`,
        writeGraphName: `${graphName}-index-inverse`,
      },
    },
    {
      op: 'collapse_path',
      graphName,
      request: {
        pathTemplates: [['REL', 'REL']],
        mutateRelationshipType: 'collapsed',
        mutateGraphName: `${graphName}-collapse-path`,
        writeGraphName: `${graphName}-collapse-path`,
      },
    },
    {
      op: 'to_undirected',
      graphName,
      request: {
        relationshipType: 'REL',
        mutateRelationshipType: 'undirected',
        mutateGraphName: `${graphName}-to-undirected`,
        writeGraphName: `${graphName}-to-undirected`,
      },
    },
  ];
}

function addModeProperty(mode: Mode, request: Request): Request {
  if (mode === 'mutate' || mode === 'write') {
    return request;
  }
  const { mutateProperty, writeProperty, ...rest } = request;
  return rest;
}

export function graphManagementModesDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const mode = (process.argv[2] as Mode) ?? 'mutate';
  const opFilter = process.argv[3];

  const graphName = `graph-mgmt-${Date.now()}`;

  const requests: Request[] = [
    {
      ...buildGraphPut(graphName),
      user,
      databaseId,
      graphName,
    },
  ];

  const algos = buildMiscRequests(graphName)
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
      ...addModeProperty(mode, item.request),
    }));

  const batch = [...requests, ...algos];

  // eslint-disable-next-line no-console
  console.log(
    'Graph management batch request:',
    JSON.stringify(batch, null, 2),
  );

  const resp = tsjsonInvoke(batch) as any[];
  const results = resp.slice(1);

  // eslint-disable-next-line no-console
  console.log(`\nGraph management ${mode} results:`);
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
  graphManagementModesDemo();
}
