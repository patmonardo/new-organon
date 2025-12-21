import { describe, it, expect } from 'vitest';

import {
  seedReflectionFormProgram,
  seedContainedFromProgram,
  seedFormEvalCallFromProgram,
  runReflectionContainerOnKernel,
} from '../../src/relative/form/dialectic/index.js';

describe('container seed bridge', () => {
  it('defaults to essence→shine→reflection', () => {
    const program = seedReflectionFormProgram();
    expect(program.morph.patterns).toEqual(['essence', 'shine', 'reflection']);
  });

  it('can embed phenomenology into program.context (passthrough)', () => {
    const program = seedReflectionFormProgram({
      phenomenology: {
        contradictions: [{ id: 'c1', claim: 'A and not-A both asserted' }],
        foundation: {
          moment: 'infinite',
          thesis: 'Contradiction-free foundation is presented',
          resolves: ['c1'],
        },
      },
    });

    const phen = (program.context as any)?.phenomenology;
    expect(phen?.foundation?.moment).toBe('infinite');
    expect(phen?.contradictions?.[0]?.id).toBe('c1');
  });

  it('derives contained seeds from container program', () => {
    const program = seedReflectionFormProgram({
      shape: {
        required_fields: ['id'],
        optional_fields: ['name'],
        type_constraints: { id: 'string' },
        validation_rules: { hegel: 'Essence→Shine' },
      },
      context: {
        runtime_strategy: 'strategy',
        conditions: ['id must exist'],
      },
    });

    const contained = seedContainedFromProgram(program);
    expect(contained.entity.fields).toEqual(['id', 'name']);
    expect(contained.property.conditions).toEqual(['id must exist']);
    expect(contained.property.validationRules.hegel).toBe('Essence→Shine');
  });

  it('builds a form_eval call for the kernel boundary', () => {
    const program = seedReflectionFormProgram();
    const call = seedFormEvalCallFromProgram({
      user: { username: 'alice', isAdmin: true },
      databaseId: 'db1',
      graphName: 'g',
      program,
    });

    expect(call.facade).toBe('form_eval');
    expect(call.op).toBe('evaluate');
    expect(call.program.morph.patterns).toEqual(['essence', 'shine', 'reflection']);
  });

  it('links container seed into a living kernel call', async () => {
    const fakeKernel = {
      name: 'fake-kernel',
      async run(request: any) {
        expect(request?.model?.id).toBe('gds.form_eval.evaluate');
        expect(request?.input?.facade).toBe('form_eval');
        expect(request?.input?.op).toBe('evaluate');
        expect(request?.input?.program?.morph?.patterns).toEqual([
          'essence',
          'shine',
          'reflection',
        ]);
        return { ok: true, output: { echoed: true } };
      },
    };

    const out = await runReflectionContainerOnKernel({
      kernel: fakeKernel as any,
      user: { username: 'alice', isAdmin: true },
      databaseId: 'db1',
      graphName: 'g',
      seed: {
        shape: {
          required_fields: ['id'],
          type_constraints: { id: 'string' },
        },
        context: { conditions: ['id must exist'] },
      },
    });

    expect(out.kernelResult.ok).toBe(true);
    expect(out.contained.entity.fields).toEqual(['id']);
    expect(out.contained.property.conditions).toEqual(['id must exist']);
  });
});
