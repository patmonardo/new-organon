import { EmpowermentProvider } from '../empowerment-core';
import type { EmpowermentLike } from '../empowerment-core';
import type { EmpowermentCombineResult } from '../empowerment-processor';

/**
 * CompositeProvider - wraps many EmpowermentProviders and presents a single provider
 * - fetchEmpowerments: concat + dedupe by id
 * - validateSignatures: accept if any engine validates
 * - combine: delegate to first engine that implements combine, else fallback
 */
export class CompositeProvider extends EmpowermentProvider {
  name = 'composite-provider';

  private readonly logger?: (msg: string, ...args: any[]) => void;
  // optional initiation / root empowerment (ground from on high)
  readonly rootEmpowerment?: EmpowermentLike;

  constructor(
    private readonly engines: EmpowermentProvider[],
    opts?: {
      logger?: (msg: string, ...args: any[]) => void;
      initiation?: EmpowermentLike;
    },
  ) {
    super();
    this.logger = opts?.logger;
    // map incoming 'initiation' to provider rootEmpowerment
    this.rootEmpowerment = opts?.initiation;
    if (!Array.isArray(engines) || engines.length === 0) {
      throw new Error(
        'CompositeProvider requires at least one EmpowermentProvider',
      );
    }
  }

  async fetchEmpowerments(
    subject: string,
    opts?: any,
  ): Promise<EmpowermentLike[]> {
    this.logger?.(
      `CompositeProvider.fetchEmpowerments start subject=${subject}`,
    );
    const lists = await Promise.all(
      this.engines.map((e) => e.fetchEmpowerments(subject, opts)),
    );
    const merged: EmpowermentLike[] = [];
    const seen = new Set<string>();
    for (const l of lists.flat()) {
      if (!l || !l.id) continue;
      if (seen.has(l.id)) continue;
      seen.add(l.id);
      merged.push(l);
    }
    // if initiation/rootEmpowerment exists and not duplijcated, include it as provider-level root
    if (
      this.rootEmpowerment &&
      !merged.some((x) => x.id === this.rootEmpowerment!.id)
    ) {
      merged.push({ ...this.rootEmpowerment });
      this.logger?.(
        `CompositeProvider.fetchEmpowerments -> injected initiation id=${this.rootEmpowerment.id}`,
      );
    }
    this.logger?.(
      `CompositeProvider.fetchEmpowerments -> merged ${merged.length} items for subject=${subject}`,
    );
    return merged;
  }

  // accept if any engine validates the signatures
  async validateSignatures(emp: EmpowermentLike): Promise<boolean> {
    this.logger?.(
      `CompositeProvider.validateSignatures -> checking emp.id=${emp?.id}`,
    );
    // If empowerment carries a provider tag, prefer validating with that engine
    const origin = (emp as any)?.provider as string | undefined;
    if (origin) {
      const originEngine = this.engines.find((e) => (e as any).name === origin);
      if (
        originEngine &&
        typeof (originEngine as any).validateSignatures === 'function'
      ) {
        try {
          const ok = await (originEngine as any).validateSignatures(emp);
          if (ok) {
            this.logger?.(
              `CompositeProvider.validateSignatures -> accepted by origin engine=${origin}`,
            );
            return true;
          } else {
            this.logger?.(
              `CompositeProvider.validateSignatures -> rejected by origin engine=${origin}`,
            );
            // conservative: proceed to other engines as fallback
          }
        } catch (err) {
          this.logger?.(
            `CompositeProvider.validateSignatures -> origin engine ${origin} errored: ${String(
              err,
            )}`,
          );
        }
      } else {
        this.logger?.(
          `CompositeProvider.validateSignatures -> origin engine ${origin} not present or lacks validateSignatures`,
        );
      }
    }
    // fallback: accept if any engine validates (legacy behavior)
    for (const e of this.engines) {
      if (typeof (e as any).validateSignatures === 'function') {
        try {
          const ok = await (e as any).validateSignatures(emp);
          if (ok) {
            this.logger?.(
              `CompositeProvider.validateSignatures -> accepted by engine=${
                (e as any).name ?? 'unknown'
              }`,
            );
            return true;
          } else {
            this.logger?.(
              `CompositeProvider.validateSignatures -> rejected by engine=${
                (e as any).name ?? 'unknown'
              }`,
            );
          }
        } catch (err) {
          this.logger?.(
            `CompositeProvider.validateSignatures -> engine ${
              (e as any).name ?? 'unknown'
            } errored: ${String(err)}`,
          );
        }
      }
    }
    this.logger?.(
      'CompositeProvider.validateSignatures -> no engine validated signatures',
    );
    // conservative: if no engine validated, reject
    return false;
  }

  // try to delegate combine to first engine that implements it, else fallback
  combine(
    empowerments: EmpowermentLike[],
    root?: EmpowermentLike,
    opts?: { privilegeBoost?: number },
  ): EmpowermentCombineResult {
    this.logger?.(
      `CompositeProvider.combine -> called with ${
        empowerments?.length ?? 0
      } empowerments, root=${root?.id ?? 'none'}`,
    );
    // prefer explicit root param, otherwise use provider initiation/rootEmpowerment
    const effectiveRoot = root ?? this.rootEmpowerment;
    for (const e of this.engines) {
      if (typeof (e as any).combine === 'function') {
        try {
          const res = (e as any).combine(empowerments, effectiveRoot, opts);
          this.logger?.(
            `CompositeProvider.combine -> delegated to engine=${
              (e as any).name ?? 'unknown'
            }`,
          );
          return res;
        } catch (err) {
          this.logger?.(
            `CompositeProvider.combine -> engine ${
              (e as any).name ?? 'unknown'
            } failed combine: ${String(err)}`,
          );
        }
      }
    }
    // fallback simple combine
    const scores = (empowerments || []).map((e) => {
      const w = typeof e.weight === 'number' ? e.weight : 0;
      const c = typeof e.certainty === 'number' ? e.certainty : 1;
      return {
        id: e.id ?? 'unknown',
        subject: e.subject ?? 'unknown',
        score: w * c,
      };
    });
    if (effectiveRoot && !scores.some((s) => s.id === effectiveRoot.id)) {
      const rw =
        typeof effectiveRoot.weight === 'number' ? effectiveRoot.weight : 0;
      const rc =
        typeof effectiveRoot.certainty === 'number'
          ? effectiveRoot.certainty
          : 1;
      scores.push({
        id: effectiveRoot.id ?? 'root',
        subject: effectiveRoot.subject ?? 'root',
        score: rw * rc,
      });
    }
    const total = scores.reduce((acc, it) => acc + it.score, 0);
    this.logger?.(`CompositeProvider.combine -> fallback total=${total}`);
    return { total, scores };
  }
}
