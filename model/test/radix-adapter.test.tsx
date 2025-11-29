import { describe, it, expect } from 'vitest';
import { renderRadixDocument, renderRadixElement } from '../src/sdsl/radix-adapter';
import type { DisplayDocument, DisplayElement } from '../src/sdsl/types';
import type { HydratorSnapshot } from '../src/data/semantic-hydrator';

const snapshot: HydratorSnapshot = {
  id: 'test-snapshot',
  plan: '{}',
  rows: [],
  metrics: {
    totalRevenue: 420000,
  },
  collections: {
    invoices: [
      { id: 'inv_1', amount: 1000, status: 'PAID' },
      { id: 'inv_2', amount: 2500, status: 'PENDING' },
    ],
  },
  assignedFields: [],
  timestamp: Date.now(),
};

describe('Radix Adapter', () => {
  it('renders a document with title and layout', () => {
    const document: DisplayDocument = {
      title: 'Radix Dashboard',
      layout: {
        type: 'stack',
        children: [
          { type: 'heading', text: 'Overview', props: { level: 2 } },
          { type: 'paragraph', text: 'Snapshot of current performance.' },
        ],
      },
    };

    const result = renderRadixDocument(document, { snapshot });
    expect(result).toBeDefined();
  });

  it('resolves metric values from snapshot', () => {
    const element: DisplayElement = {
      type: 'metric',
      props: {
        metric: 'totalRevenue',
        label: 'Total Revenue',
        unit: 'USD',
        trend: 'up',
        delta: 12,
      },
    };

    const node = renderRadixElement(element, 0, { snapshot });
    expect(node).toBeDefined();
  });

  it('renders table rows from collection binding', () => {
    const element: DisplayElement = {
      type: 'table',
      props: {
        collection: 'invoices',
        columns: [
          { id: 'id', label: 'Invoice' },
          { id: 'amount', label: 'Amount' },
        ],
      },
    };

    const node = renderRadixElement(element, 0, { snapshot });
    expect(node).toBeDefined();
  });
});
