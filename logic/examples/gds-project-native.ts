/**
 * Example: GraphStore Catalog facade (subGraphProject)
 *
 * Pass-1 projection semantics:
 * - Creates a source graph via `graph_store.put`
 * - Projects it into a new catalog entry via `graph_store_catalog.subGraphProject`
 *
 * This is the “Projection/Factory” seam: today it clones from the catalog; later it
 * can swap to a real native factory/loader without changing the TS-JSON shape.
 */
/// <reference types="node" />

import { tsjsonInvoke } from './_tsjson';

export function projectNativeDemo(): void {
  const user = { username: 'alice', isAdmin: true };
  const databaseId = 'db1';
  const sourceGraphName = `src-${Date.now()}`;
  const projectedGraphName = `proj-${Date.now()}`;

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName: sourceGraphName,
      snapshot: {
        nodes: [1, 2, 3],
        relationships: [
          { type: 'KNOWS', source: 1, target: 2 },
          { type: 'LIKES', source: 2, target: 3 },
        ],
      },
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'subGraphProject',
      user,
      databaseId,
      graphName: projectedGraphName,
      originGraphName: sourceGraphName,
      nodeFilter: '*',
      relationshipFilter: 'KNOWS',
      configuration: {},
    },
    {
      kind: 'ApplicationForm',
      facade: 'graph_store_catalog',
      op: 'list_graphs',
      user,
      databaseId,
      includeDegreeDistribution: false,
    },
  ];

  const resp = tsjsonInvoke(batch) as any[];
  // eslint-disable-next-line no-console
  console.log('batch.response:', resp);

  const project = resp?.[1]?.data;
  const entries = resp?.[2]?.data?.entries ?? [];
  // eslint-disable-next-line no-console
  console.dir(
    { sourceGraphName, projectedGraphName, project, entries },
    { depth: null },
  );
}

if (import.meta.url === `file://${process.argv[1]}`) {
  projectNativeDemo();
}
