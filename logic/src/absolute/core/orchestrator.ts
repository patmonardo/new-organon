// import type { ProcessorInputs } from './contracts';
// import { assembleWorld } from '../essence/world';
// import { projectContentFromContexts } from '../form/project';
// import { deriveSyllogisticEdges } from '../concept/syllogism';
// import { indexContent } from '../essence/world';
// import type { Content } from '@schema';
// import type { World } from '@schema';
// import { groundStage, commitGroundResults, type GroundResult } from '../essence/ground';
// import type { KriyaOptions } from './kriya';
// import { classifyTruthOfRelation } from './qualquant';
// import { computeKnowledgeDelta, type KnowledgeDelta } from './knowledge';

// // Use canonical KriyaOptions from core/kriya.ts

// export type KriyaResult = {
//   world: World;
//   projectedContent: Content[];
//   // Derived syllogistic edges are returned but not merged into world (non-destructive)
//   derivedEdges: ReturnType<typeof deriveSyllogisticEdges>;
//   indexes: {
//     content: ReturnType<typeof indexContent>;
//   };
//   // Ground derivation results (relations + properties) produced by groundStage
//   ground?: GroundResult;
//   knowledge?: KnowledgeDelta;
// };

// export async function runKriya(
//   input: ProcessorInputs,
//   opts: KriyaOptions = {},
// ): Promise<KriyaResult> {
//   const options: Required<KriyaOptions> = {
//     projectContent: opts.projectContent ?? true,
//     contentIndexSource: opts.contentIndexSource ?? 'inputs',
//     deriveSyllogistic: opts.deriveSyllogistic ?? false,
//     triad: opts.triad ?? undefined,
//     commitGround: opts.commitGround ?? false,
//   } as Required<KriyaOptions>;

//   // 1) Assemble world (deterministic)
//   const world = assembleWorld(input);

//   // 2) Project content from contexts (deterministic ids)
//   const projectedContent = options.projectContent
//     ? projectContentFromContexts(input)
//     : [];

//   // 3) Derive syllogistic edges (layer only, not merged)
//   const derivedEdges = options.deriveSyllogistic
//     ? deriveSyllogisticEdges(input)
//     : [];

//   // 4) Index content with stable policy
//   const contentForIndex =
//     options.contentIndexSource === 'projected'
//       ? projectedContent
//       : options.contentIndexSource === 'both'
//       ? // Combine deterministically by id (inputs first, then projected uniques)
//         dedupeById([...input.content, ...projectedContent])
//       : input.content;

//   const indexes = {
//     content: indexContent({ ...input, content: contentForIndex }),
//   };

//   // 5) Ground stage: derive relations/properties from morphs + input entities/properties
//   // Use input.entities/properties (source-of-truth for derivation)
//   let ground: GroundResult | undefined = undefined;
//   try {
//     ground = await groundStage(
//       { morphs: input.morphs },
//       { entities: input.entities, properties: input.properties },
//       opts as any,
//     );
//   } catch (err) {
//     // swallow; ground is best-effort for now
//   }

//   // Optional commit to persistence via triad
//   if (options.commitGround && options.triad && ground) {
//     try {
//       // commitGroundResults is defensive; it will create/update idempotently
//       await commitGroundResults(options.triad, ground);
//     } catch (err) {
//       // swallow commit errors for now
//     }
//   }

//   // 6) Truth actualization: mark essential relations as actual when their truth warrants it
//   //    and emit knowledge events with justification.
//   if (ground) {
//     const bus = (options.triad as any)?.bus;
//     const props = ground.properties ?? [];
//     const before = {
//       relations: [...ground.relations.map((x) => ({ ...x }))],
//       properties: [...props.map((x) => ({ ...x }))],
//     };
//     for (const r of ground.relations) {
//       if (!(r as any)) continue;
//       const truth = classifyTruthOfRelation(
//         r as any,
//         ground.relations as any[],
//         {},
//         { properties: props as any[] },
//       );
//       // Actualize on Mechanism or Chemism; Teleology remains possible by default
//       if (truth === 'Mechanism' || truth === 'Chemism') {
//         const prev = (r as any).provenance?.modality;
//         const confidence = Math.max(
//           prev?.confidence ?? 0,
//           truth === 'Mechanism' ? 0.75 : 0.7,
//         );
//         (r as any).provenance = {
//           ...((r as any).provenance ?? {}),
//           modality: { kind: 'actual', confidence },
//         };
//         bus?.publish?.({
//           kind: 'knowledge.relation.actualized',
//           payload: { id: (r as any).id, truth, confidence },
//         });
//       }
//     }
//     // Compute knowledge delta (before → after) and emit aggregate signal
//     const after = { relations: ground.relations, properties: props };
//     const delta = computeKnowledgeDelta(before, after);
//     if (delta.score > 0)
//       bus?.publish?.({ kind: 'knowledge.delta', payload: delta });
//     // Attach to result
//     (ground as any).__knowledge = delta;
//   }

//   // If ground was computed and has attached knowledge, surface it; otherwise undefined
//   const knowledge = (ground as any)?.__knowledge as KnowledgeDelta | undefined;
//   return { world, projectedContent, derivedEdges, indexes, ground, knowledge };
// }

// // Helper: deterministic dedupe by shape.core.id (or shape.of/id fallback)
// function dedupeById(list: Content[]): Content[] {
//   const key = (c: Content) =>
//     c.shape.core.id ??
//     `content:${c.shape.kind}:${c.shape.of.id}:${c.shape.core.name ?? ''}`;
//   const seen = new Set<string>();
//   const out: Content[] = [];
//   for (const c of list) {
//     const k = key(c);
//     if (seen.has(k)) continue;
//     seen.add(k);
//     out.push(c);
//   }
//   // stable order by id
//   out.sort((a, b) =>
//     (a.shape.core.id ?? '').localeCompare(b.shape.core.id ?? ''),
//   );
//   return out;
// }
