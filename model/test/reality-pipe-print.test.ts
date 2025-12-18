import { describe, expect, it } from 'vitest';
import { InMemoryRealityPipe } from '../src/sdsl/reality-pipe';
import { readFileSync } from 'fs';
import { join } from 'path';
import { PrintEnvelopeSchema } from '@organon/task';

const fixturesDir = join(__dirname, '..', '..', 'task', 'test', 'fixtures', 'prints');

describe('InMemoryRealityPipe interoperability with PrintEnvelope', () => {
  it('publishes a knowing print and subscriber receives typed envelope', () => {
    const pipe = new InMemoryRealityPipe<string, unknown, unknown>();
    const raw = JSON.parse(readFileSync(join(fixturesDir, 'knowing.json'), 'utf8'));

    let received: any = null;
    pipe.subscribe((env) => {
      received = env;
    });

    const published = pipe.publish(raw as any);

    // Basic delivery
    expect(received).not.toBeNull();
    expect(published).toBeDefined();

    // Validate using PrintEnvelopeSchema
    const parsed = PrintEnvelopeSchema.parse(received);
    expect(parsed.kind).toBe('knowing');
  });
});
