import { describe, it, expect } from 'vitest';

import { contextFromFactTrace, defineModel, sum, count, dimension, modelAndViewToFactTraceEvents } from '../src/sdsl';

describe('logical-model-trace', () => {
  it('adds model/view facts into context', () => {
    const Sales = defineModel({
      name: 'Sales',
      source: 'demo.sales',
      dimensions: { day: dimension('created_at', 'day'), customer: 'customer' },
      measures: { revenue: sum('amount'), orders: count() },
    });

    const SalesByDay = Sales.view({ group_by: ['day'], aggregate: ['revenue', 'orders'], limit: 10 });

    const trace = modelAndViewToFactTraceEvents(Sales, SalesByDay, { modelId: 'm-sales', viewId: 'v-sales-by-day' });
    const ctx = contextFromFactTrace(trace, { schema: { id: 'trace:demo' } });

    const modelFact = ctx.facts.find((f) => f.type === 'model.define');
    const viewPlanFact = ctx.facts.find((f) => f.type === 'view.plan');

    expect(modelFact?.provenance).toBe('asserted');
    expect(viewPlanFact?.provenance).toBe('inferred');

    // Make sure the plan includes the source + query scaffold
    expect((viewPlanFact?.value as any)?.plan).toContain('demo.sales');
    expect((viewPlanFact?.value as any)?.plan).toContain('group_by');
  });
});
