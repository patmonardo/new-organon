import { z } from 'zod';

import { KernelModelRefSchema } from './kernel-api';

/**
 * Syscalls / capabilities (GDSL)
 *
 * RootAgent plans and acts against a declared set of available actions.
 * This stays purely structural: it describes *what can be called*, not how.
 */

export const SyscallIdSchema = z.string().min(1);
export type SyscallId = z.infer<typeof SyscallIdSchema>;

export const SyscallTagSchema = z.string().min(1);
export type SyscallTag = z.infer<typeof SyscallTagSchema>;

export const SyscallBaseSchema = z
  .object({
    id: SyscallIdSchema,
    title: z.string().min(1),
    description: z.string().optional(),
    tags: z.array(SyscallTagSchema).optional(),
  })
  .strict();

/**
 * Tool-like action (external function call).
 *
 * `name` is a stable identifier the agent can emit into `taw.act.payload.action`.
 * `inputSchema` is intentionally loose (JSON Schema or any schema blob).
 */
export const ToolSyscallSchema = SyscallBaseSchema.extend({
  kind: z.literal('tool'),
  name: z.string().min(1),
  inputSchema: z.unknown().optional(),
}).strict();
export type ToolSyscall = z.infer<typeof ToolSyscallSchema>;

/**
 * Kernel run action.
 *
 * This is the declared capability to call `kernel.run` for a given model.
 */
export const KernelRunSyscallSchema = SyscallBaseSchema.extend({
  kind: z.literal('kernel.run'),
  model: KernelModelRefSchema,
}).strict();
export type KernelRunSyscall = z.infer<typeof KernelRunSyscallSchema>;

export const SyscallSchema = z.discriminatedUnion('kind', [ToolSyscallSchema, KernelRunSyscallSchema]);
export type Syscall = z.infer<typeof SyscallSchema>;

export const SyscallTableSchema = z
  .object({
    syscalls: z.array(SyscallSchema),
  })
  .strict();
export type SyscallTable = z.infer<typeof SyscallTableSchema>;

export function parseSyscall(input: unknown): Syscall {
  return SyscallSchema.parse(input);
}

export function parseSyscallTable(input: unknown): SyscallTable {
  return SyscallTableSchema.parse(input);
}
