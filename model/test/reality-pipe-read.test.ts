import { describe, expect, it } from 'vitest';
import { InMemoryRealityPipe } from '../src/sdsl/reality-pipe';
import { readFileSync } from 'fs';
import { join } from 'path';

const fixturesDir = join(__dirname, '..', '..', 'task', 'test', 'fixtures', 'prints');

describe('InMemoryRealityPipe read primitive', () => {
  it('returns all prints when no filter provided', () => {
    const pipe = new InMemoryRealityPipe<string, unknown, unknown>();
    const k = JSON.parse(readFileSync(join(fixturesDir, 'knowing.json'), 'utf8'));
    const c = JSON.parse(readFileSync(join(fixturesDir, 'conceiving.json'), 'utf8'));

    pipe.publish(k as any);
    pipe.publish(c as any);

    const view = pipe.read();
    expect(view.prints.length).toBeGreaterThanOrEqual(2);
    const ids = new Set(view.prints.map((p) => p.id));
    expect(ids.has('p-0001')).toBeTruthy();
    expect(ids.has('p-0002')).toBeTruthy();
  });

  it('filters by kind', () => {
    const pipe = new InMemoryRealityPipe<string, unknown, unknown>();
    const k = JSON.parse(readFileSync(join(fixturesDir, 'knowing.json'), 'utf8'));
    const c = JSON.parse(readFileSync(join(fixturesDir, 'conceiving.json'), 'utf8'));

    pipe.publish(k as any);
    pipe.publish(c as any);

    const view = pipe.read({ kind: 'knowing' });
    expect(view.prints.every((p) => p.kind === 'knowing')).toBeTruthy();
  });

  it('filters by correlationId and source', () => {
    const pipe = new InMemoryRealityPipe<string, any, any>();
    const k = JSON.parse(readFileSync(join(fixturesDir, 'knowing.json'), 'utf8')) as any;
    k.correlationId = 'c-1';
    k.source = 'gds::pagerank';

    pipe.publish(k);

    const view = pipe.read({ correlationId: 'c-1', source: 'gds::pagerank' });
    expect(view.prints.length).toBe(1);
    expect(view.prints[0].id).toBe('p-0001');
  });
});
