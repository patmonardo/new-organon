import type { KernelPort } from '@absolute/form/kernel-api';
import {
  gdsApplicationOperationId,
  type GdsApplicationCall,
} from '../schema/application';

/**
 * GDS Link invocation helper
 *
 * This is the canonical way for app layers to invoke a GDS Application Facade
 * through an abstract `KernelPort`.
 *
 * - Input: a typed `GdsApplicationCall` (facade/op + payload)
 * - Output: the TS-JSON `data` payload (as `unknown`, caller can refine/parse)
 */
export async function invokeGdsApplicationCall(
  port: KernelPort,
  call: GdsApplicationCall,
): Promise<unknown> {
  const modelId = gdsApplicationOperationId(call);
  const result = await port.run({
    model: { id: modelId },
    input: call,
  });

  if (result.ok) return result.output;
  throw new Error(
    typeof (result as any)?.error?.message === 'string'
      ? (result as any).error.message
      : 'GDS call failed',
  );
}
