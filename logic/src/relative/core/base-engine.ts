import { EmpowermentProvider } from './empowerment-core';
import fs from 'fs/promises';
import FormEmpowerment from './empowerment-form';
import type { EmpowermentLike } from './empowerment-core';
import { allSignaturesValid, anyFacetApplicable } from './proto/helpers';

/**
 * BaseEngine
 * - Concrete, lightweight implementation of EmpowermentProvider that engines
 *   can extend. Provides small, sensible defaults (no-op crypto, simple facet
 *   matcher, optional rootEmpowerment, trivial cache).
 * - Engines should override fetchEmpowerments, verifySignatures, and/or
 *   checkFacets as needed.
 */
export class BaseEngine extends EmpowermentProvider {
  name = 'base-engine';
  protected readonly scope: string;
  protected readonly rootEmpowerment?: EmpowermentLike;
  // single in-memory empowerment graph owned by each engine instance
  protected readonly graph: Map<string, EmpowermentLike>;
  protected readonly cache: Map<string, EmpowermentLike[]>;

  constructor(opts?: { scope?: string; rootEmpowerment?: EmpowermentLike }) {
    super();
    this.scope = opts?.scope ?? 'engine';
    this.rootEmpowerment = opts?.rootEmpowerment;
    this.cache = new Map();
    this.graph = new Map();
  }

  // Graph management API
  protected addEmpowerment(emp: EmpowermentLike) {
    const e = { ...emp, provider: emp.provider ?? this.name };
    if (!e.id) throw new Error('empowerment must have id');
    this.graph.set(e.id, e);
    return e;
  }

  protected bulkAdd(empowerments: EmpowermentLike[] = []) {
    for (const e of empowerments) this.addEmpowerment(e);
    // clear cache because graph changed
    this.cache.clear();
  }

  protected removeEmpowerment(id: string) {
    const removed = this.graph.delete(id);
    if (removed) this.cache.clear();
    return removed;
  }

  protected clearGraph() {
    this.graph.clear();
    this.cache.clear();
  }

  // query by subject/action/resource/facet; default provider scope = this.name
  protected queryEmpowerments(
    subject?: string,
    opts: { action?: string; resource?: string; provider?: string } = {},
  ): EmpowermentLike[] {
    const provider = opts.provider ?? this.name;
    const out: EmpowermentLike[] = [];
    for (const e of this.graph.values()) {
      if (provider && e.provider !== provider) continue;
      if (subject && e.subject && e.subject !== subject) continue;
      if (
        opts.action &&
        Array.isArray(e.actions) &&
        !e.actions.includes(opts.action)
      )
        continue;
      if (opts.resource && Array.isArray(e.facets)) {
        const found = e.facets.some(
          (f: any) =>
            f === opts.resource ||
            (typeof f === 'string' &&
              f.endsWith('*') &&
              opts.resource!.startsWith(f.slice(0, -1))),
        );
        if (!found) continue;
      }
      out.push({ ...e });
    }
    return out;
  }

  // Default fetch returns entries from this engine's graph
  async fetchEmpowerments(
    subject: string,
    _opts?: any,
  ): Promise<EmpowermentLike[]> {
    if (this.cache.has(subject)) return this.cache.get(subject)!;
    const list = this.queryEmpowerments(subject, {
      provider: this.name,
      action: _opts?.action,
      resource: _opts?.resource,
    });
    this.cache.set(subject, list);
    return list;
  }

  // Default: accept if no signatures. Engines should override for real crypto.
  protected async verifySignatures(sigs?: any[]): Promise<boolean> {
    try {
      return allSignaturesValid(sigs as any[]);
    } catch {
      // conservative fallback if helper fails
      if (!sigs || sigs.length === 0) return true;
      return sigs.every(
        (s) => typeof s.signature === 'string' && s.signature.length > 0,
      );
    }
  }

  // Exposed for ProcessorControl.validateSignatures path
  async validateSignatures(emp: EmpowermentLike): Promise<boolean> {
    try {
      return await Promise.resolve(
        this.verifySignatures(emp.signatures as any[]),
      );
    } catch {
      return false;
    }
  }

  // Basic facet checker:
  // - string facets: match if facet === '*' || facet === action || facet === resource
  // - object facets: if facet.scope present, accept when any scope entry is '*' or matches resource
  checkFacets(
    facets: Array<any>,
    ctx: { action?: string; resource?: string; context?: any } = {},
  ) {
    try {
      return anyFacetApplicable(facets as any[], {
        action: ctx.action,
        resource: ctx.resource,
      });
    } catch {
      // conservative fallback: permissive when helper fails
      if (!facets || facets.length === 0) return true;
      for (const f of facets) {
        if (
          typeof f === 'string' &&
          (f === '*' || f === ctx.action || f === ctx.resource)
        )
          return true;
        if (
          f &&
          Array.isArray((f as any).scope) &&
          (f as any).scope.length === 0
        )
          return true;
      }
      return false;
    }
  }

  // Combine helper: prefer canonical FormEmpowerment.combine if present
  combine(
    empowerments: EmpowermentLike[],
    root?: EmpowermentLike,
    opts?: { privilegeBoost?: number },
  ) {
    try {
      if (
        FormEmpowerment &&
        typeof (FormEmpowerment as any).combine === 'function'
      ) {
        return (FormEmpowerment as any).combine(empowerments, root, opts);
      }
    } catch {
      // fall through to simple combine
    }

    const boost =
      typeof opts?.privilegeBoost === 'number' ? opts!.privilegeBoost! : 1;
    const scores = (empowerments || []).map((e) => {
      const w = typeof e.weight === 'number' ? e.weight : 0;
      const c = typeof e.certainty === 'number' ? e.certainty : 1;
      return {
        id: e.id ?? 'unknown',
        subject: e.subject ?? 'unknown',
        score: w * c * boost,
      };
    });
    if (root && !scores.some((s) => s.id === root.id)) {
      const rw = typeof root.weight === 'number' ? root.weight : 0;
      const rc = typeof root.certainty === 'number' ? root.certainty : 1;
      scores.push({
        id: root.id ?? 'root',
        subject: root.subject ?? 'root',
        score: rw * rc * boost,
      });
    }
    const total = scores.reduce((acc, it) => acc + it.score, 0);
    return { total, scores };
  }

  // Persistence API: save/load the engine's empowerment graph as JSON
  async saveGraph(filePath: string) {
    const arr = Array.from(this.graph.values());
    try {
      await fs.mkdir(require('path').dirname(filePath), { recursive: true });
      await fs.writeFile(filePath, JSON.stringify(arr, null, 2), 'utf8');
      return { ok: true, path: filePath, count: arr.length };
    } catch (err) {
      return { ok: false, error: String(err) };
    }
  }

  async loadGraph(filePath: string, opts?: { overwrite?: boolean }) {
    try {
      const raw = await fs.readFile(filePath, 'utf8');
      const arr = JSON.parse(raw) as EmpowermentLike[];
      if (opts?.overwrite) this.clearGraph();
      for (const e of arr) {
        // normalize provider and id
        const emp = { ...e, provider: e.provider ?? this.name };
        if (!emp.id) continue;
        this.graph.set(emp.id, emp);
      }
      this.cache.clear();
      return { ok: true, path: filePath, count: arr.length };
    } catch (err) {
      return { ok: false, error: String(err) };
    }
  }

  // Export/import helpers (in-memory, useful for tests)
  exportGraph(): EmpowermentLike[] {
    return Array.from(this.graph.values()).map((e) => ({ ...e }));
  }

  importGraph(items: EmpowermentLike[], opts?: { overwrite?: boolean }) {
    if (opts?.overwrite) this.clearGraph();
    for (const e of items) {
      if (!e.id) continue;
      this.graph.set(e.id, { ...e, provider: e.provider ?? this.name });
    }
    this.cache.clear();
  }
}
