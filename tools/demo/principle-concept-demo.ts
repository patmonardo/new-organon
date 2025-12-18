import { InMemoryRealityPipe } from '../../model/src/sdsl/reality-pipe.js';
import { translatePrincipleToConcept } from '../../logic/src/translator/principle-to-concept.js';

async function main() {
  const pipe = new InMemoryRealityPipe<string, any, any>();

  // translator subscribes to the pipe and republishes Concept prints
  pipe.subscribe((env) => {
    if (env.kind === 'knowing') {
      const concept = translatePrincipleToConcept(env as any) as any;
      pipe.publish(concept);
    }
  });

  // publish a Principle (kernel prajna equivalent)
  const principle = {
    id: 'p-principle-demo-1',
    kind: 'knowing',
    role: 'kernel',
    ts: Date.now(),
    timestamp: new Date().toISOString(),
    ontology: 'monadic',
    provenance: { id: 'prov-demo-1', origin: 'empirical', createdAt: new Date().toISOString() },
    payload: { modality: 'signal', summary: 'demo spike', trace: { node: 'node:demo', metric: 'pagerank', value: 0.7 } },
  } as any;

  console.log('Publishing principle ->', principle.id);
  pipe.publish(principle);

  const view = pipe.read({ kind: 'conceiving', aggregate: { groupBy: (e) => (e as any).payload?.subject ?? 'node:demo', reducer: 'conclusive-latest' } });

  console.log('Aggregated Essence snapshot:', JSON.stringify(view.aggregated, null, 2));
}

main().catch((e) => {
  // eslint-disable-next-line no-console
  console.error('Demo error', e);
});
