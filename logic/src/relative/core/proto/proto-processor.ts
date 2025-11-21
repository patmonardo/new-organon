import { ProcessorControl } from '../empowerment-protocol';
import { CompositeProvider } from './composite-provider';
import type { EmpowermentProvider, EmpowermentLike } from '../empowerment-core';

export class PrototypeProcessor extends ProcessorControl {
  constructor(
    engines: EmpowermentProvider[],
    opts?: {
      logger?: (msg: string, ...args: any[]) => void;
      // use 'initiation' as the public option for assigning a root empowerment
      initiation?: EmpowermentLike;
    },
  ) {
    const composite = new CompositeProvider(engines, {
      logger: opts?.logger,
      initiation: opts?.initiation,
    });
    super(composite);
  }

  // convenience alias
  async run(
    subject: string,
    action: string,
    resource?: string,
    opts?: { privilegeBoost?: number },
  ) {
    return this.authorize(subject, action, resource, opts);
  }
}
