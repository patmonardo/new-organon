import BaseDriver from './driver';

/**
 * Dialectical driver primitives live in core alongside BaseDriver.
 * Drivers extend DialecticalBase to get a clear contract for phenomenal <> noumenal surfaces.
 */

export type PhenomenalEvent = {
  kind: string;
  payload?: unknown;
};

export type NoumenalEvent = {
  kind: string;
  concept?: string;
  intent?: unknown;
};

export type ActiveCarrier<T = unknown> = {
  id: string;
  kind: string;
  payload: T;
  meta?: {
    noumenal?: { concept?: string; score?: number; [k: string]: unknown };
    [k: string]: unknown;
  };
};

export interface DialecticalDriver<TActive = unknown, TCore = unknown> {
  toActive(core: TCore): ActiveCarrier<TActive>;
  toActiveBatch(cores: TCore[]): ActiveCarrier<TActive>[];
  fromActive(active: ActiveCarrier<TActive>): TCore;

  interpretNoumenal?(event: NoumenalEvent): Promise<ActiveCarrier<TActive>[]>;
  expressNoumenalFromActive?(
    active: ActiveCarrier<TActive>,
  ): NoumenalEvent | null;
}

export abstract class DialecticalBase<TActive = unknown, TCore = unknown>
  extends BaseDriver
  implements DialecticalDriver<TActive, TCore>
{
  constructor(name = 'DialecticalBase') {
    super(name);
  }

  abstract toActive(core: TCore): ActiveCarrier<TActive>;
  toActiveBatch(cores: TCore[]) {
    return cores.map((c) => this.toActive(c));
  }
  abstract fromActive(active: ActiveCarrier<TActive>): TCore;

  async interpretNoumenal(_event: NoumenalEvent) {
    return [];
  }
  expressNoumenalFromActive(_active: ActiveCarrier<TActive>) {
    return null;
  }
}
