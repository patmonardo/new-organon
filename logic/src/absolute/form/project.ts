import type { ProcessorInputs } from '../core/contracts';
import { createContent } from '../../schema/content';

/**
 * Deterministic Content projection from Contexts.
 * This is a small shim copied from compiled dist to satisfy build expectations.
 */
export function projectContentFromContexts(input: ProcessorInputs, opts?: any) {
  const classify =
    opts?.classify ?? ((ctx: any) => {
      const t = ctx.shape.core.type?.toLowerCase?.() ?? '';
      const n = (ctx.shape.core.name ?? '').toLowerCase();
      return t.includes('world') || n.includes('world') ? 'subtle' : 'gross';
    });
  const contentType = opts?.contentType ?? 'system.Content';
  const dedupeEntities = opts?.dedupeEntities ?? true;

  const contexts = input.contexts
    .map((ctx) => ({ ctx, kind: classify(ctx) }))
    .sort((a, b) => {
      const rank = (k: string) => (k === 'subtle' ? 0 : 1);
      const r = rank(a.kind) - rank(b.kind);
      return r !== 0
        ? r
        : a.ctx.shape.core.id.localeCompare(b.ctx.shape.core.id);
    });

  const out: any[] = [];
  for (const { ctx, kind } of contexts) {
    const entities = [...ctx.shape.entities].sort((a: any, b: any) => a.id.localeCompare(b.id));
    const seen = new Set<string>();
    for (const ref of entities) {
      const key = `${ref.type}:${ref.id}`;
      if (dedupeEntities && seen.has(key)) continue;
      if (dedupeEntities) seen.add(key);
      const id = deterministicContentId(ctx.shape.core.id, ref.id, kind);
      out.push(
        createContent({
          id,
          type: contentType,
          name: `${kind}:${ctx.shape.core.name ?? ctx.shape.core.id}#${ref.id}`,
          kind,
          of: { id: ref.id, type: ref.type },
        }),
      );
    }
  }
  return out;
}

function deterministicContentId(ctxId: string, entityId: string, kind: string) {
  return `content:${kind}:${ctxId}:${entityId}`;
}

export default { projectContentFromContexts };
